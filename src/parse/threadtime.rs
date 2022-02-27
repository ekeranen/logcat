use crate::message::{Level, Message, MessageBuilder};
use crate::parse::parser::Parser;
use anyhow::{bail, Context, Result};
use chrono::{Datelike, Local, NaiveDate};

/// Parses a line of text into a message.
///
/// # Examples
///
/// ```
/// use logcat::parse;
///
/// let line = "...";
/// let message = parse::threadtime(line);
/// ```
pub fn threadtime(line: &str) -> Result<Message> {
    let mut parser = ThreadTimeParser::new();
    parser.parse(line)
}

#[derive(Debug)]
struct PartialMessage {
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
    pid: i32,
    tid: i32,
    level: Level,
    tag: String,
}

impl Default for PartialMessage {
    fn default() -> PartialMessage {
        PartialMessage {
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
            pid: 0,
            tid: 0,
            level: Level::Verbose,
            tag: String::new(),
        }
    }
}

pub struct ThreadTimeParser {
    msg: PartialMessage,
}

impl ThreadTimeParser {
    fn new() -> ThreadTimeParser {
        ThreadTimeParser {
            msg: PartialMessage::default(),
        }
    }
}

impl Parser for ThreadTimeParser {
    fn parse(&mut self, line: &str) -> Result<Message> {
        if line.starts_with('-') {
            bail!("malformed line");
        }

        // The `line` is expected to look like:
        //   mm-dd hh:mm:ss.mmm pid tid level tag: content
        self.msg = PartialMessage::default();
        self.parse_date(line)
            .and_then(|x| self.parse_time(x))
            .and_then(|x| self.parse_pid(x))
            .and_then(|x| self.parse_tid(x))
            .and_then(|x| self.parse_level(x))
            .and_then(|x| self.parse_tag(x))
            .and_then(|x| self.parse_content(x))
    }
}

impl ThreadTimeParser {
    fn parse_date<'a>(&mut self, mut rest: &'a str) -> Result<&'a str> {
        // mm-dd <...>
        rest = rest.trim_start();

        let (month_day, rest) = rest
            .split_once(char::is_whitespace)
            .context("invalid line: no groups after date")?;

        let mut parse = || -> Result<&'a str> {
            let (month, day) = month_day.split_once('-').context("'-' not found")?;
            self.msg.month = month.parse()?;
            self.msg.day = day.parse()?;
            Ok(rest)
        };
        parse().with_context(|| format!("invalid date (mm-dd): {}", month_day))
    }

    fn parse_time<'a>(&mut self, mut rest: &'a str) -> Result<&'a str> {
        // hh:mm:ss.mmm <...>
        rest = rest.trim_start();

        let (time, rest) = rest
            .split_once(char::is_whitespace)
            .context("invalid line: no groups after time")?;

        let mut parse = || -> Result<&'a str> {
            let mut splitter = time.split(&[':', '.'][..]);
            self.msg.hour = splitter.next().context("not enough groups")?.parse()?;
            self.msg.minute = splitter.next().context("not enough groups")?.parse()?;
            self.msg.second = splitter.next().context("not enough groups")?.parse()?;
            self.msg.millisecond = splitter.next().context("not enough groups")?.parse()?;
            Ok(rest)
        };
        parse().with_context(|| format!("invalid time: {}", time))
    }

    fn parse_pid<'a>(&mut self, mut rest: &'a str) -> Result<&'a str> {
        rest = rest.trim_start();

        let (pid, rest) = rest
            .split_once(char::is_whitespace)
            .context("invalid line: no groups after process id")?;
        self.msg.pid = pid
            .parse()
            .with_context(|| format!("invalid process id: {}", pid))?;
        Ok(rest)
    }

    fn parse_tid<'a>(&mut self, mut rest: &'a str) -> Result<&'a str> {
        rest = rest.trim_start();

        let (tid, rest) = rest
            .split_once(char::is_whitespace)
            .context("invalid line: no groups after thread id")?;
        self.msg.tid = tid
            .parse()
            .with_context(|| format!("invalid thread id: {}", tid))?;
        Ok(rest)
    }

    fn parse_level<'a>(&mut self, mut rest: &'a str) -> Result<&'a str> {
        rest = rest.trim_start();

        let (level, rest) = rest
            .split_once(char::is_whitespace)
            .context("invalid line: no groups after level")?;
        self.msg.level = match level.chars().next() {
            Some(level) => match level {
                'V' => Level::Verbose,
                'D' => Level::Debug,
                'I' => Level::Info,
                'W' => Level::Warning,
                'E' => Level::Error,
                'F' => Level::Fatal,
                _ => bail!("invalid level: {}", level),
            },
            None => bail!("invalid level: {}", level),
        };
        Ok(rest)
    }

    fn parse_tag<'a>(&mut self, mut rest: &'a str) -> Result<&'a str> {
        rest = rest.trim_start();

        let (tag, rest) = rest.split_once(':').context("invalid line: missing tag")?;
        self.msg.tag = tag.trim_end().to_string();

        // Advance past leading space.
        // parse_content() expects `rest` to contain only content.
        let mut chars = rest.chars();
        chars.next();
        Ok(chars.as_str())
    }

    fn parse_content<'a>(&mut self, rest: &'a str) -> Result<Message> {
        let year = Local::today().year();
        let datetime = NaiveDate::from_ymd(year, self.msg.month, self.msg.day).and_hms_milli(
            self.msg.hour,
            self.msg.minute,
            self.msg.second,
            self.msg.millisecond,
        );

        let message = MessageBuilder::new()
            .level(self.msg.level)
            .tag(&self.msg.tag)
            .content(rest)
            .date_time(datetime)
            .process_id(self.msg.pid)
            .thread_id(self.msg.tid)
            .build();
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::Level;
    use chrono::{Datelike, Timelike};

    #[test]
    fn threadtime() {
        let data = "--------- beginning of main
12-31 22:59:41.271     1   197 I init    : Uptime: 00002.612275 LocalTime: 01-01 03:59:41.276
";

        let lines: Vec<_> = data.lines().collect();

        {
            let first_msg = parse::threadtime(lines[0]);
            assert!(first_msg.is_err());
        }

        let second_msg = parse::threadtime(lines[1]);
        assert!(second_msg.is_ok());

        let second_msg = second_msg.unwrap();
        assert_eq!(second_msg.level(), Level::Info);
        assert_eq!(second_msg.tag(), "init");
        assert!(second_msg.content().starts_with("Uptime:"));

        let date = second_msg.date().unwrap();
        assert!(date.year() > 2000);
        assert_eq!(date.month(), 12);
        assert_eq!(date.day(), 31);

        let time = second_msg.time().unwrap();
        assert_eq!(time.hour(), 22);
        assert_eq!(time.minute(), 59);
        assert_eq!(time.second(), 41);

        assert_eq!(second_msg.process_id().unwrap(), 1);
        assert_eq!(second_msg.thread_id().unwrap(), 197);
    }

    #[test]
    fn threadtime_levels() {
        let cases = [
            ("12-31 0:0:0.0 1 1 V tag: content", Level::Verbose),
            ("12-31 0:0:0.0 1 1 D tag: content", Level::Debug),
            ("12-31 0:0:0.0 1 1 I tag: content", Level::Info),
            ("12-31 0:0:0.0 1 1 W tag: content", Level::Warning),
            ("12-31 0:0:0.0 1 1 E tag: content", Level::Error),
            ("12-31 0:0:0.0 1 1 F tag: content", Level::Fatal),
        ];

        for case in &cases {
            let msg = parse::threadtime(case.0).unwrap();
            assert_eq!(msg.level(), case.1)
        }
    }

    #[test]
    fn threadtime_tags() {
        let cases = [
            ("12-31 0:0:0.0 1 1 V tag: content", "tag"),
            ("12-31 0:0:0.0 1 1 D  tag : content", "tag"),
            ("12-31 0:0:0.0 1 1 I      tag     : content", "tag"),
            (
                "12-31 0:0:0.0 1 1 W longer_snake_tag: content",
                "longer_snake_tag",
            ),
        ];

        for case in &cases {
            let msg = parse::threadtime(case.0).unwrap();
            assert_eq!(msg.tag(), case.1)
        }
    }

    #[test]
    fn threadtime_content() {
        let cases = [
            ("12-31 0:0:0.0 1 1 V tag: content", "content"),
            ("12-31 0:0:0.0 1 1 D  tag :  content", " content"),
            ("12-31 0:0:0.0 1 1 I      tag     : content", "content"),
            (
                "12-31 0:0:0.0 1 1 I      tag     :    content   ",
                "   content   ",
            ),
            (
                "12-31 0:0:0.0 1 1 I tag: multi-word content.",
                "multi-word content.",
            ),
        ];

        for case in &cases {
            let msg = parse::threadtime(case.0).unwrap();
            assert_eq!(msg.content(), case.1)
        }
    }

    #[test]
    fn threadtime_ok() {
        let cases = ["12-31 0:0:0.0 1 1 I tag:", "12-31 0:0:0.0 1 1 I :"];

        for case in &cases {
            println!("{}", case);
            assert!(parse::threadtime(case).is_ok());
        }
    }

    #[test]
    fn threadtime_malformed() {
        let cases = [
            "12- 0:0:0.0 1 1 I tag: content",
            "-31 0:0:0.0 1 1 I tag: content",
            "12-31 :0:0.0 1 1 I tag: content",
            "12-31 0::0.0 1 1 I tag: content",
            "12-31 0:0:.0 1 1 I tag: content",
            "12-31 0:0:. 1 1 I tag: content",
            "12-31 0:0:0.0  1 I tag: content",
            "12-31 0:0:0.0   I tag: content",
            "12-31 0:0:0.0 1 1  tag: content",
            "12-31 0:0:0.0 1 1  I: content",
            "12-31 0:0:0.0 1 1  I content",
        ];

        for case in &cases {
            println!("{}", case);
            assert!(parse::threadtime(case).is_err());
        }
    }
}
