//
//  0 1 0  Mnemosyne: a functional systems programming language.
//  0 0 1  (c) 2015 Hawk Weisman
//  1 1 1  hi@hawkweisman.me
//
//  Mnemosyne is released under the MIT License. Please refer to
//  the LICENSE file at the top-level directory of this distribution
//  or at https://github.com/hawkw/mnemosyne/.
//
//! Mnemosyne error handling

use std::fmt::{ Display, Debug };

/// Wraps Option/Result with an `expect_ice()` method.
///
/// The `expect_ice()` method functions similarly to the standard library's
/// `expect()`, but with the custom Mnemosyne internal compiler error message.
pub trait ExpectICE<T> {
    fn expect_ice(self, msg: &str) -> T;
}

impl<T> ExpectICE<T> for Option<T> {
    /// Unwraps an option, yielding the content of a `Some`
    ///
    /// # Panics
    ///
    /// Panics using the Mnemosyne internal compiler error formatter
    /// if the value is a `None`, with a custom panic message
    /// provided by `msg`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use mnemosyne::errors::ExpectICE;
    /// let x = Some("value");
    /// assert_eq!(x.expect_ice("the world is ending"), "value");
    /// ```
    ///
    /// ```ignore
    /// # use mnemosyne::errors::ExpectICE;
    /// let x: Option<&str> = None;
    /// x.expect_ice("the world is ending");
    /// ```
    #[inline]
    fn expect_ice(self, msg: &str) -> T {
        match self { Some(thing) => thing
                   , None        => ice!(msg)
                   }
    }
}

impl<T, E> ExpectICE<T> for Result<T, E>
where E: Debug {

    /// Unwraps a result, yielding the content of an `Ok`.
    ///
    /// Panics using the Mnemosyne internal compiler error formatter
    /// if the value is an `Err`, with a panic message including the
    /// passed message, and the content of the `Err`.
    ///
    /// # Examples
    /// ```ignore
    /// # use mnemosyne::errors::ExpectICE;
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.expect_ice("Testing expect");
    /// ```
    #[inline]
    fn expect_ice(self, msg: &str) -> T {
        match self { Ok(t) => t
                   , Err(e) => ice!("{}: {:?}", msg, e)
                   }
    }
}

/// Wraps Option/Result with an `unwrap_ice()` method.
///
/// The `unwrap_ice()` method functions similarly to the standard library's
/// `unwrap()`, but with the custom Mnemosyne internal compiler error message.
pub trait UnwrapICE<T> {
    fn unwrap_ice(self) -> T;
}

impl<T> UnwrapICE<T> for Option<T> {
    /// Moves the value `v` out of the `Option<T>` if it is `Some(v)`.
    ///
    /// Unlike the standard library's `unwrap()`, this uses the Mnemosyne
    /// internal compiler error panic formatter.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals `None`.
    ///
    /// # Safety note
    ///
    /// In general, because this function may panic, its use is discouraged.
    /// Instead, prefer to use pattern matching and handle the `None`
    /// case explicitly.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use mnemosyne::errors::UnwrapICE;
    /// let x = Some("air");
    /// assert_eq!(x.unwrap_ice(), "air");
    /// ```
    ///
    /// ```ignore
    /// # use mnemosyne::errors::UnwrapICE;
    /// let x: Option<&str> = None;
    /// assert_eq!(x.unwrap_ice(), "air"); // fails
    /// ```
    #[inline]
    fn unwrap_ice(self) -> T {
        match self { Some(thing) => thing
                   , None =>
                        ice!("called `Option::unwrap()` on a `None` value")
                   }
    }
}

impl<T, E> UnwrapICE<T> for Result<T, E>
where E: Display  {
    /// Unwraps a result, yielding the content of an `Ok`.
    ///
    /// Unlike the standard library's `unwrap()`, this uses the Mnemosyne
    /// internal compiler error panic formatter.
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Err`, with a panic message provided by the
    /// `Err`'s value.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use mnemosyne::errors::UnwrapICE;
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(x.unwrap_ice(), 2);
    /// ```
    ///
    /// ```ignore
    /// # use mnemosyne::errors::UnwrapICE;
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.unwrap_ice(); // panics
    /// ```
    #[inline]
    fn unwrap_ice(self) -> T {
        match self { Ok(t) => t
                   , Err(e) => ice!("{}", e)
                   }
    }
}
//
// impl<T, E> UnwrapICE<T> for Result<T, E>
// where E: Debug {
//     /// Unwraps a result, yielding the content of an `Ok`.
//     ///
//     /// Unlike the standard library's `unwrap()`, this uses the Mnemosyne
//     /// internal compiler error panic formatter.
//     ///
//     /// # Panics
//     ///
//     /// Panics if the value is an `Err`, with a panic message provided by the
//     /// `Err`'s value.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// # use mnemosyne::errors::UnwrapICE;
//     /// let x: Result<u32, &str> = Ok(2);
//     /// assert_eq!(x.unwrap_ice(), 2);
//     /// ```
//     ///
//     /// ```{.should_panic}
//     /// # use mnemosyne::errors::UnwrapICE;
//     /// let x: Result<u32, &str> = Err("emergency failure");
//     /// x.unwrap_ice(); // panics with `emergency failure`
//     /// ```
//     #[inline]
//     fn unwrap_ice(self) -> T {
//         match self {
//             Ok(t) => t
//           , Err(e) =>
//                 ice!("called `Result::unwrap()` on an `Err` value: {:?}", e)
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_expect_ok() {
        let x = Some("value");
        assert_eq!(x.expect_ice("the world is ending"), "value");
    }

    #[test]
    #[should_panic]
    fn test_option_expect_panic() {
        let x: Option<&str> = None;
        x.expect_ice("the world is ending");
    }

    #[test]
    #[should_panic]
    fn test_result_expect_panic() {
        let x: Result<u32, &str> = Err("emergency failure");
        x.expect_ice("Testing expect");
    }

    #[test]
    fn test_option_unwrap_ok() {
        let x = Some("air");
        assert_eq!(x.unwrap_ice(), "air");
    }

    #[test]
    #[should_panic]
    fn test_option_unwrap_panic() {
        let x: Option<&str> = None;
        assert_eq!(x.unwrap_ice(), "air"); // fails
    }

    #[test]
    fn test_result_unwrap_ok() {
        let x: Result<u32, &str> = Ok(2);
        assert_eq!(x.unwrap_ice(), 2);
    }

    #[test]
    #[should_panic]
    fn test_result_unwrap_panic() {
        let x: Result<u32, &str> = Err("emergency failure");
        x.unwrap_ice(); // panics
    }
}
