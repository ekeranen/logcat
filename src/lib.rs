//! This crate contains Android logcat parsing facilities.
//!
//! # Examples
//!
//! ```
//! use logcat::parse;
//!
//! let source = "...";
//! for line in source.lines() {
//!     if let Ok(msg) = parse::threadtime(line) {
//!         println!("level = {:?}", msg.level());
//!         println!("tag = {}", msg.tag());
//!         println!("content = {}", msg.content());
//!     }
//! }
//! ```
//!
//! Iterate over all warnings and errors.
//!
//! ```
//! use logcat::{message::Level, parse};
//!
//! let source = "...";
//! for line in source.lines() {
//!     if let Ok(msg) = parse::threadtime(line) {
//!         if Level::is_warning_or_higher(msg.level()) {
//!             // ...
//!         }
//!     }
//! }
//! ```

pub mod message;
pub mod parse;
