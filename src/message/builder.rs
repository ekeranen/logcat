use crate::message::{Level, Message};
use chrono::naive::NaiveDateTime;
use std::cell::RefCell;
use thiserror::Error;

/// The error type for [`MessageBuilder`] operations.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum Error {
    /// The specified field was not set.
    #[error("field not set: `{0}`")]
    FieldNotSet(&'static str),
}

/// Builds [`Message`] in parts.
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
    /// Creates a new MessageBuilder.
    pub fn new() -> MessageBuilder {
        MessageBuilder::default()
    }

    /// Sets the required message log level.
    pub fn level(&mut self, value: Level) -> &mut Self {
        *self.level.borrow_mut() = Some(value);
        self
    }

    /// Sets the required message tag.
    pub fn tag(&mut self, value: &str) -> &mut Self {
        *self.tag.borrow_mut() = Some(value.to_owned());
        self
    }

    /// Sets the required message content.
    pub fn content(&mut self, value: &str) -> &mut Self {
        *self.content.borrow_mut() = Some(value.to_owned());
        self
    }

    /// Sets the optional message date and time.
    pub fn date_time(&mut self, value: NaiveDateTime) -> &mut Self {
        *self.date_time.borrow_mut() = Some(value);
        self
    }

    /// Sets the optional message process ID.
    pub fn process_id(&mut self, value: i32) -> &mut Self {
        *self.pid.borrow_mut() = Some(value);
        self
    }

    /// Sets the optional message thread ID.
    pub fn thread_id(&mut self, value: i32) -> &mut Self {
        *self.tid.borrow_mut() = Some(value);
        self
    }

    /// Builds and returns the Message.
    ///
    /// An error may be returned if one or more required fields were not set.
    pub fn build(&self) -> Result<Message, Error> {
        // Clone Option<T>s.
        let level = (*self.level.borrow()).ok_or(Error::FieldNotSet("level"))?;
        let tag = (*self.tag.borrow())
            .clone()
            .ok_or(Error::FieldNotSet("tag"))?;
        let content = (*self.content.borrow())
            .clone()
            .ok_or(Error::FieldNotSet("content"))?;

        Ok(Message {
            level,
            tag,
            content,

            date_time: *self.date_time.borrow(),
            pid: *self.pid.borrow(),
            tid: *self.tid.borrow(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::message::{
        builder::{Error, MessageBuilder},
        Level,
    };
    use chrono::{Datelike, NaiveDate, Timelike};

    #[test]
    fn message_with_mandatory() {
        let m = MessageBuilder::new()
            .level(Level::Verbose)
            .tag("tag")
            .content("content")
            .build()
            .unwrap();

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
            .build()
            .unwrap();

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
    fn message_without_mandatory_fields() {
        assert_eq!(
            MessageBuilder::new().build(),
            Err(Error::FieldNotSet("level"))
        );
        assert_eq!(
            MessageBuilder::new().level(Level::Debug).build(),
            Err(Error::FieldNotSet("tag"))
        );
        assert_eq!(
            MessageBuilder::new().level(Level::Debug).tag("tag").build(),
            Err(Error::FieldNotSet("content"))
        );
        assert_eq!(
            MessageBuilder::new().tag("tag").content("content").build(),
            Err(Error::FieldNotSet("level"))
        );
    }
}
