//! # Fu
//!
//! Just an [`Error`] with its location and helpful macros.
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

use std::error::Error as StdError;

type Location = (&'static str, u32, u32);

/// A custom error type that contains file location and a message.
///
/// This struct is used to represent errors with additional context like the file name,
/// line, and column where the error occurred, along with a user-defined message.
pub struct Error {
    context: Option<String>,
    source: Option<Box<dyn StdError + Send + Sync>>,
    location: Location,
}

impl Error {
    /// Creates a new `Error` instance.
    ///
    /// # Example
    /// ```
    /// # use fu::Error;
    /// let err = Error::new(Some("oops"), ("main.rs", 10, 15));
    /// println!("{}", err); // oops    main.rs:[10:15]
    /// ```
    pub fn new<S: Into<String>>(context: Option<S>, location: Location) -> Self {
        Error {
            context: context.map(|c| c.into()),
            location,
            source: None,
        }
    }

    pub fn context<C: Into<String>>(mut self, context: C) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Wraps an existing error with additional context.
    pub fn chain<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    pub fn chain_iter(&self) -> ChainIter<'_> {
        ChainIter {
            current: Some(self),
        }
    }
}

pub struct ChainIter<'a> {
    current: Option<&'a dyn StdError>,
}

impl<'a> Iterator for ChainIter<'a> {
    type Item = &'a dyn StdError;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        self.current = current.source();
        Some(current)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, e) in self.chain_iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            if i == 0 {
                if let Some(context) = &self.context {
                    write!(f, "{}    ", context)?;
                }
                write!(
                    f,
                    "\x1b[90m{}:[{}:{}]\x1b[0m",
                    self.location.0, self.location.1, self.location.2
                )?;
            } else {
                write!(f, "Caused by: {}", e)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_ref()
            .map(|s| s.as_ref() as &(dyn StdError + 'static))
    }
}

pub trait Wrap<T, E> {
    fn wrap<C: Into<String>>(self, context: C) -> std::result::Result<T, Error>;
}

impl<T, E> Wrap<T, E> for std::result::Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn wrap<C: Into<String>>(self, ctx: C) -> std::result::Result<T, Error> {
        self.map_err(|e| {
            Error::new(Some(ctx), (std::file!(), std::line!(), std::column!())).chain(e)
        })
    }
}

/// [`Result`]<T, [`Error`]>.
pub type Result<T> = std::result::Result<T, Error>;

/// Construct a Result with the crates [`Error`] type.
///
/// This macro uses the `file!()`, `line!()`, and `column!()` macros to automatically
/// capture the file and location of the error.
///
/// # Example
/// ```
/// # use fu::*;
/// let result: Result<()> = Err(error!("oops"));
/// assert!(result.is_err());
/// ```
#[macro_export]
macro_rules! error {
    () => {
        $crate::Error::new(
            None,
            (file!(), line!(), column!()),
        )
    };
    ($($arg:tt)*) => {
        $crate::Error::new(
            Some(format!($($arg)*)),
            (file!(), line!(), column!()),
        )
    };
}

/// Return early with an error.
///
/// This macro behaves like the [`error!`] macro but immediately returns the error from the
/// function.
///
/// # Example
/// ```
/// # use fu::*;
/// fn example() -> Result<()> {
///     bail!("an early exit error")
/// }
/// assert!(example().is_err());
/// ```
#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::error!($($arg)*))
    };
}

/// Return early with an error if a condition is not satisfied.
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
            bail!("bigger than x");
        }

        Ok(())
    }

    #[test]
    fn test_error_creation() {
        let err: Result<()> = Err(error!("test error"));
        assert!(err.is_err_and(|e| e.to_string().contains("test error")));
    }

    #[test]
    fn test_error_formatted() {
        let err: Result<()> = Err(error!("test error {}, {} {:?}", "formatted", 1, vec![2, 3]));
        assert!(err.is_err_and(|e| e.to_string().contains("[2, 3]")));
    }

    #[test]
    fn test_example_function() {
        assert!(example_function(-1).is_err());
        assert!(example_function(50).is_ok());
        assert!(example_function(101).is_err());
    }

    #[test]
    fn test_file_not_found() -> Result<()> {
        let res = std::fs::File::open("abc").wrap("wrapped");
        let _ = res.inspect_err(|e| println!("\n{e}"));
        Ok(())
    }

    #[test]
    fn test_wrap() -> Result<()> {
        let res: Result<()> = Err(error!("first"));
        let _ = res.inspect_err(|e| println!("\n{e}"));
        Ok(())
    }
}
