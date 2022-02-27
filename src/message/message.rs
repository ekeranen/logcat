use crate::Level;
use chrono::naive::{NaiveDate, NaiveDateTime, NaiveTime};
use std::cell::RefCell;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    level: Level,
    tag: String,
    content: String,

    date_time: Option<NaiveDateTime>,
    pid: Option<i32>,
    tid: Option<i32>,
}

impl Message {
    /// Returns the logging level specified with this message.
    pub fn level(&self) -> Level {
        self.level
    }

    /// Returns the log tag specified with this message.
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// Returns the content of this message.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Returns the date and time this message was logged.
    ///
    /// Returns `None` if the date and time  is not available.
    pub fn date_time(&self) -> Option<NaiveDateTime> {
        self.date_time.clone()
    }

    /// Returns the date this message was logged.
    ///
    /// Returns `None` if the date is not available.
    pub fn date(&self) -> Option<NaiveDate> {
        match self.date_time {
            Some(ref dt) => Some(dt.date().clone()),
            None => None,
        }
    }

    /// Returns the time this message was logged.
    ///
    /// Returns `None` if the time is not available.
    pub fn time(&self) -> Option<NaiveTime> {
        match self.date_time {
            Some(ref dt) => Some(dt.time().clone()),
            None => None,
        }
    }

    /// Returns the process ID of the process that logged this message.
    ///
    /// Returns `None` if the process ID is not available.
    pub fn process_id(&self) -> Option<i32> {
        self.pid
    }

    /// Returns the thread ID of the thread that logged this message.
    ///
    /// Returns `None` if the thread ID is not available.
    pub fn thread_id(&self) -> Option<i32> {
        self.tid
    }
}

/// Builds `Message` in parts.
///
/// # Panics
///
/// Panics if `level()`, `tag()`, or `content()` were not set before `build()` was called.
#[derive(Default)]
pub struct MessageBuilder {
    // Mandatory
    level: RefCell<Option<Level>>,
    tag: RefCell<Option<String>>,
    content: RefCell<Option<String>>,

    // Optional
    date_time: RefCell<Option<NaiveDateTime>>,
    pid: RefCell<Option<i32>>,
    tid: RefCell<Option<i32>>,
}

impl MessageBuilder {
    pub fn new() -> MessageBuilder {
        MessageBuilder::default()
    }

    pub fn level(&mut self, value: Level) -> &mut Self {
        *self.level.borrow_mut() = Some(value);
        self
    }

    pub fn tag(&mut self, value: &str) -> &mut Self {
        *self.tag.borrow_mut() = Some(value.to_owned());
        self
    }

    pub fn content(&mut self, value: &str) -> &mut Self {
        *self.content.borrow_mut() = Some(value.to_owned());
        self
    }

    pub fn date_time(&mut self, value: NaiveDateTime) -> &mut Self {
        *self.date_time.borrow_mut() = Some(value);
        self
    }

    pub fn process_id(&mut self, value: i32) -> &mut Self {
        *self.pid.borrow_mut() = Some(value);
        self
    }

    pub fn thread_id(&mut self, value: i32) -> &mut Self {
        *self.tid.borrow_mut() = Some(value);
        self
    }

    #[must_use]
    pub fn build(&self) -> Message {
        // Clone Option<T>s.
        let level = (*self.level.borrow()).clone().expect("level is required");
        let tag = (*self.tag.borrow()).clone().expect("tag is required");
        let content = (*self.content.borrow())
            .clone()
            .expect("content is required");

        Message {
            level: level,
            tag: tag,
            content: content,

            date_time: (*self.date_time.borrow()).clone(),
            pid: (*self.pid.borrow()).clone(),
            tid: (*self.tid.borrow()).clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::message::MessageBuilder;
    use crate::Level;
    use chrono::{Datelike, NaiveDate, Timelike};

    #[test]
    fn message_with_mandatory() {
        let m = MessageBuilder::new()
            .level(Level::Verbose)
            .tag("tag")
            .content("content")
            .build();

        assert_eq!(m.level(), Level::Verbose);
        assert_eq!(m.tag(), "tag");
        assert_eq!(m.content(), "content");
        assert_eq!(m.date(), None);
        assert_eq!(m.time(), None);
        assert_eq!(m.process_id(), None);
        assert_eq!(m.thread_id(), None);
    }

    #[test]
    fn message_with_optional() {
        let m = MessageBuilder::new()
            .level(Level::Verbose)
            .tag("tag")
            .content("content")
            .date_time(NaiveDate::from_ymd(2017, 8, 1).and_hms(7, 30, 0))
            .process_id(1)
            .thread_id(2)
            .build();

        assert_eq!(m.level(), Level::Verbose);
        assert_eq!(m.tag(), "tag");
        assert_eq!(m.content(), "content");

        let date = m.date().unwrap();
        assert_eq!(date.year(), 2017);
        assert_eq!(date.month(), 8);
        assert_eq!(date.day(), 1);

        let time = m.time().unwrap();
        assert_eq!(time.hour(), 7);
        assert_eq!(time.minute(), 30);
        assert_eq!(time.second(), 0);

        assert_eq!(m.process_id().unwrap(), 1);
        assert_eq!(m.thread_id().unwrap(), 2);
    }

    #[test]
    #[should_panic]
    fn message_without_mandatory1() {
        let _m = MessageBuilder::new().build();
    }

    #[test]
    #[should_panic]
    fn message_without_mandatory2() {
        let _m = MessageBuilder::new().level(Level::Debug).build();
    }

    #[test]
    #[should_panic]
    fn message_without_mandatory3() {
        let _m = MessageBuilder::new().level(Level::Debug).tag("tag").build();
    }

    #[test]
    #[should_panic]
    fn message_without_mandatory4() {
        let _m = MessageBuilder::new().tag("tag").content("content").build();
    }
}
