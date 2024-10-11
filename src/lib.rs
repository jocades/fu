//! # Fu
//!
//! Just an [`Error`] with its  and helpful macros.
//!
//! - Custom `Error` type with file name, line, and column information.
//! - Short and convenient macros: `error!`, `bail!`, and `ensure!`.
//! - Lightweight.
//!
//! ## Usage
//!
//! ```should_panic
//! use fu::{bail, ensure, Result};
//!
//! const MAX: i32 = 10;
//!
//! fn example(value: i32) -> Result<()> {
//!     ensure!(value >= 0, "value must be non-negative");
//!
//!     if value > MAX {
//!         bail!("value is larger than {}", MAX);
//!     }
//!
//!     Ok(())
//! }
//!
//! fn main() -> Result<()> {
//!     example(-1)
//! }
//!
//! // Error: value must be non-negative    examples/foo.rs:[4:5]
//!```

/// A custom error type that contains file location and a message.
///
/// This struct is used to represent errors with additional context like the file name,
/// line, and column where the error occurred, along with a user-defined message.
pub struct Error {
    file: String,
    location: (u32, u32),
    message: String,
}

impl Error {
    /// Creates a new `Error` instance.
    ///
    /// # Parameters
    /// - `file`: The name of the file where the error occurred (use [`file!()`] macro).
    /// - `location`: A tuple `(line, column)` representing the line and column where the error occurred.
    /// - `message`: A description of the error.
    ///
    /// # Examples
    /// ```
    /// # use fu::Error;
    /// let err = Error::new("main.rs", (10, 15), "oops");
    /// println!("{}", err); // Error: at main.rs:[10:15]: oops
    /// ```
    pub fn new<S: Into<String>>(file: S, location: (u32, u32), message: S) -> Self {
        Error {
            file: file.into(),
            location,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}    \x1b[90m{}:[{}:{}]\x1b[0m",
            self.message, self.file, self.location.0, self.location.1
        )
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

/// A type alias for a `Result` that uses the custom `Error` type.
pub type Result<T> = std::result::Result<T, Error>;

/// Macro to generate an [`Error`] and return an `Err` result.
///
/// This macro uses the `file!()`, `line!()`, and `column!()` macros to automatically
/// capture the file and location of the error.
///
/// # Example
/// ```
/// # use fu::*;
/// let result: Result<()> = error!("oops");
/// assert!(result.is_err());
/// ```
#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        Err($crate::Error::new(
            file!(),
            (line!(), column!()),
            $msg,
        ))
    };

    ($($arg:tt)*) => {
        Err($crate::Error::new(
            file!(),
            (line!(), column!()),
            &format!($($arg)*),
        ))
    };
}

/// Macro to return early with an error.
///
/// This macro behaves like the [`error!`] macro but immediately returns the error from the
/// function, mimicking the behavior of `bail!` in the `anyhow` crate.
///
/// # Example
/// ```
/// # use fu::*;
/// fn example() -> Result<()> {
///     bail!("an early exit error");
/// }
/// assert!(example().is_err());
/// ```
#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return $crate::error!($($arg)*)
    };
}

/// Macro to check a condition and return an error if the condition is false.
///
/// If the provided condition is not met, this macro will trigger a [`bail!`] with the given
/// error message. It is a convenient way to validate inputs.
///
/// # Example
/// ```
/// # use fu::*;
/// fn check(value: i32) -> Result<()> {
///     ensure!(value >= 0, "value must be non-negative");
///     Ok(())
/// }
/// assert!(check(-1).is_err());
/// ```
#[macro_export]
macro_rules! ensure {
    ($condition:expr, $($arg:tt)*) => {
        if !($condition) {
            $crate::bail!($($arg)*);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_function(value: i32) -> Result<()> {
        ensure!(value >= 0, "value must be non-negative");

        if value > 100 {
            bail!("value is too large");
        }

        Ok(())
    }

    #[test]
    fn test_error_creation() {
        let err: Result<()> = error!("test error");
        assert!(err.is_err_and(|e| e.to_string().contains("test error")));
    }

    #[test]
    fn test_error_formatted() {
        let err: Result<()> = error!("test error {}, {} {:?}", "formatted", 1, vec![2, 3]);
        assert!(err.is_err_and(|e| e.to_string().contains("[2, 3]")));
    }

    #[test]
    fn test_example_function() {
        assert!(example_function(-1).is_err());
        assert!(example_function(50).is_ok());
        assert!(example_function(101).is_err());
    }
}
