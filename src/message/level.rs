/// Logging levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Verbose,
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

impl Level {
    /// Returns `true` if `Debug`, `Info`, `Warning`, `Error`, or `Fatal`.
    pub fn is_debug_or_higher(self) -> bool {
        match self {
            Level::Debug | Level::Info | Level::Warning | Level::Error | Level::Fatal => true,
            _ => false,
        }
    }

    /// Returns `true` if `Info`, `Warning`, `Error`, or `Fatal`.
    pub fn is_info_or_higher(self) -> bool {
        match self {
            Level::Info | Level::Warning | Level::Error | Level::Fatal => true,
            _ => false,
        }
    }

    /// Returns `true` if `Warning`, `Error`, or `Fatal`.
    pub fn is_warning_or_higher(self) -> bool {
        match self {
            Level::Warning | Level::Error | Level::Fatal => true,
            _ => false,
        }
    }

    /// Returns `true` if `Error` or `Fatal`.
    pub fn is_error_or_higher(self) -> bool {
        match self {
            Level::Error | Level::Fatal => true,
            _ => false,
        }
    }

    /// Returns the short description for this `Level`.
    ///
    /// # Examples
    ///
    /// ```
    /// use logcat::Level;
    ///
    /// assert_eq!(Level::short(Level::Verbose), "V");
    /// assert_eq!(Level::short(Level::Debug), "D");
    /// assert_eq!(Level::short(Level::Info), "I");
    /// assert_eq!(Level::short(Level::Warning), "W");
    /// assert_eq!(Level::short(Level::Error), "E");
    /// assert_eq!(Level::short(Level::Fatal), "F");
    /// ```
    pub fn short(self) -> &'static str {
        match self {
            Level::Verbose => "V",
            Level::Debug => "D",
            Level::Info => "I",
            Level::Warning => "W",
            Level::Error => "E",
            Level::Fatal => "F",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Level;

    #[test]
    fn level() {
        assert!(!Level::is_debug_or_higher(Level::Verbose));
        assert!(Level::is_debug_or_higher(Level::Debug));
        assert!(Level::is_debug_or_higher(Level::Info));
        assert!(Level::is_debug_or_higher(Level::Warning));
        assert!(Level::is_debug_or_higher(Level::Error));
        assert!(Level::is_debug_or_higher(Level::Fatal));

        assert!(!Level::is_info_or_higher(Level::Verbose));
        assert!(!Level::is_info_or_higher(Level::Debug));
        assert!(Level::is_info_or_higher(Level::Info));
        assert!(Level::is_info_or_higher(Level::Warning));
        assert!(Level::is_info_or_higher(Level::Error));
        assert!(Level::is_info_or_higher(Level::Fatal));

        assert!(!Level::is_warning_or_higher(Level::Verbose));
        assert!(!Level::is_warning_or_higher(Level::Debug));
        assert!(!Level::is_warning_or_higher(Level::Info));
        assert!(Level::is_warning_or_higher(Level::Warning));
        assert!(Level::is_warning_or_higher(Level::Error));
        assert!(Level::is_warning_or_higher(Level::Fatal));

        assert!(!Level::is_error_or_higher(Level::Verbose));
        assert!(!Level::is_error_or_higher(Level::Debug));
        assert!(!Level::is_error_or_higher(Level::Info));
        assert!(!Level::is_error_or_higher(Level::Warning));
        assert!(Level::is_error_or_higher(Level::Error));
        assert!(Level::is_error_or_higher(Level::Fatal));
    }
}
