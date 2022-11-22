mod builder;
mod level;

pub use builder::{Error, MessageBuilder};
use chrono::naive::{NaiveDate, NaiveDateTime, NaiveTime};
pub use level::Level;

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
        self.date_time
    }

    /// Returns the date this message was logged.
    ///
    /// Returns `None` if the date is not available.
    pub fn date(&self) -> Option<NaiveDate> {
        self.date_time.as_ref().map(|dt| dt.date())
    }

    /// Returns the time this message was logged.
    ///
    /// Returns `None` if the time is not available.
    pub fn time(&self) -> Option<NaiveTime> {
        self.date_time.as_ref().map(|dt| dt.time())
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
