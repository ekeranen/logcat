use crate::message::Message;
use anyhow::Result;

/// Parses from an Android logcat source.
pub trait Parser {
    /// Parses one line from an Android logcat source.
    ///
    /// Returns `None` if parsing failed.
    ///
    /// This trait is usually used with a `MessageIterator<T>` and not
    /// used directly.
    fn parse(&mut self, line: &str) -> Result<Message>;
}
