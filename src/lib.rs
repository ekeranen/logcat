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
//! use logcat::parse;
//!
//! let source = "...";
//! for line in source.lines() {
//!     if let Ok(msg) = parse::threadtime(line) {
//!         if msg.level().is_warning_or_higher() {
//!             // ...
//!         }
//!     }
//! }
//! ```

pub mod message;
pub mod parse;
