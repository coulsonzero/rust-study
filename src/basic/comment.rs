

// single comment

/*
 * multi-line comment
 */

/**
 * document comment
 */

/// document comment
///
///  Use [`eprint!`] instead to print error and progress messages.
///
///  necessary to use [`io::stdout().flush()`][flush] to ensure the output is emitted
/// immediately.
///
/// using the `Debug` implementation
///
/// # Examples
///
/// ```rust
/// use std::io::{self, Write};
///
/// print!("this string has a newline, why not choose println! instead?\n");
///
/// io::stdout().flush().unwrap();
/// ```


//! document comment
//!
//! ```
//! (1) string:
//!         a) for c in s.chars() {...}
//!         b) for (_, c) in s.chars().enumerate() {...}
//! (2) array:
//!         for v in nums {...}
//! ```


