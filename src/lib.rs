/*
Copyright 2024 Benjamin Richcreek

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
//! # Base62 Converter & Deconverter
//! Convert your positive [integers](https://doc.rust-lang.org/std/primitive.u32.html) to [ASCII](https://en.wikipedia.org/wiki/ASCII)-safe [`String`](https://doc.rust-lang.org/std/string/struct.String.html)s for situations where a concise [`String`] representation of an [integer](https://doc.rust-lang.org/std/primitive.u32.html) is desired, such as when automatically generating file names or identifiers.
//!
//! Each possible integer value is assigned a unique [`String`] representation, allowing for lossless conversion between [`String`]s and [`u32`] values. Each additional character in a [`String`] representation can be thought of as another digit in a [Base62] system. The [Base62] system is similar to [hexadecimal](https://en.wikipedia.org/wiki/Hexadecimal) except instead of only 16 digits, 62 digits are used. Here is a list of all 62 digits, in order from least to greatest value (as defined in this crate):
//! ```no_run
//! # /*
//! 0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ
//! # */
//! ```
//! Since the [`String`] representation of integers is based on [Base62], lower values have shorter representations, but thanks to the large amount of values each digit could be, representations (of [32-bit integers](https://doc.rust-lang.org/std/primitive.u32.html)) are never more than 6 characters long.
//!
//! # Example
//! As a quick example, let's encode a value using the [`encode`](encoding::encode) function from the [`encoding`] module, then decode it using the [`decode`](decoding::decode) function from the [`decoding`] module, like so:
//! ```
//! # use ascii_basing::{encoding,decoding};
//! let number: u32 = 52 * 62 * 62 + 9 * 62 + 35; // = 200481
//! let representation = encoding::encode(number,Some(3)).unwrap();
//! assert_eq!(String::from("Q9z"),representation);
//! assert_eq!(number,decoding::decode(representation.chars()).unwrap());
//! ```
//! # Alternatives
//! This library creates output similar to [base-62], but the implementation is distinct. The main advantages of this library (as of now) are encoding `0` to the string `"0"` rather than an empty String, and allowing for users to give [`Option`]al hints about the size of encoded values (which are used to pre-allocate String capacity). [base-62] has the advantage of allowing encoding and decoding of unsigned integers in `&[u8]` format rather than only u32 format. 
//!
//! In general, it is best to use [base-62] when large numbers or byte sequences need to be encoded, and to use this library in most other situations.
//!
//![base-62]: https://crates.io/crates/base-62
//![Base62]: https://en.wikipedia.org/wiki/Base62
use std::error::Error;
use std::fmt;
use std::fmt::{Formatter,Display};
///Changing [`u32`] values to [`String`]s
pub mod encoding {
    use crate::{Error,fmt,Formatter,Display};
    #[derive(Debug)]
    ///The [`Error`] Type for the [`encoding`](crate::encoding) module
    ///
    /// A simple error type that is returned by [`encode`] or [`push_encoded`] if one of them fails unexpectedly. While both functions return
    /// [`Result`]s in order to allow for user-defined error handling, please note that any behavior that causes an `EncodingError` to be returned is considered an unexpected bug
    /// and may be patched out in future releases. As such, if you call [`encode`] or [`push_encoded`] and recieve this error in response, please open a bug issue in Github.
    pub struct EncodingError {
        message: String,
    }
    impl EncodingError {
        fn new(problem: &'static str) -> EncodingError{
            return EncodingError { message: String::from(problem) };
        }
    }
    impl Display for EncodingError {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "Encoding a value failed because: {} ", self.message)
        }
    }
    impl Error for EncodingError {}
    /// Creates a new [`String`] containing the [Base62] representation of the `value` argument
    /// # Arguments
    /// The function takes two arguments. The first, `value`, is a [`u32`] indicating what numerical value the [`String`] representation produced by the function should represent in [Base62].
    /// The second argument is an [`Option<usize>`]. If the argument is [`None`], then the [`String`] is automatically given a [capacity](https://doc.rust-lang.org/std/string/struct.String.html#method.with_capacity) of 6 so it can hold any value.
    /// If you'd prefer to avoid excessive allocation or need more capacity because you will be adding more characters to the [`String`] later in your program, you can pass in a custom [`usize`] value and the [`String`] created by this function will be
    /// [pre-allocated](https://doc.rust-lang.org/std/string/struct.String.html#method.with_capacity) with the capacity specified.
    ///
    /// [Base62]: https://en.wikipedia.org/wiki/Base62
    /// # Output
    /// Returns a [`Result<String,EncodingError>`] that may store either the [`String`] generated by this function and storing a [Base62] representation of the `value` argument, or an [`EncodingError`]. If an [`EncodingError`] is returned, please record
    /// the input that caused the bug to occur and open an issue on Github.
    ///
    /// # Example
    /// Let's imagine that you were working on 
    /// a library that automatically generated identical fields (with different names) in a Rust data structure. 
    /// In order to support large quantities of fields while keeping identifiers a reasonable length so that the data structure could be efficiently [parsed](https://docs.rs/serde/latest/serde/index.html) and sent online, you would need to encode the data to make the identifiers shorter. For this example, we'll look at only the largest value you would hypothetically need to support - [`u32::MAX`] - or, represented in base 10, `4294967295`. 
    /// Using this library, you could easily condense `4294967295` down to just `4GFfc3` in just a few lines using this function, like so:
    /// ```
    /// # use ascii_basing::encoding;
    /// let big_number = u32::MAX; //4294967295
    /// let short_representation = encoding::encode(big_number,Some(6));
    /// assert_eq!(String::from("4GFfc3"),short_representation.unwrap());
    /// ```
    ///
    /// # Panics
    /// Panics if the `size` argument contains a value that would cause [`String::with_capacity`] from the standard library to fail or panic, for example:
    /// ```should_panic
    /// # use ascii_basing::encoding::encode;
    /// let big_capacity = usize::try_from(isize::MAX).unwrap() + 1;
    /// encode(0,Some(big_capacity));
    /// ```
    /// These situations are not considered bugs, unlike situations that cause the function to return an [`EncodingError`].
    ///
    /// 
    pub fn encode (value: u32, size: Option<usize>) -> Result<String,EncodingError> {
        let mut new_value = if let Some(provided_size) = size {
            String::with_capacity(provided_size)
        } else {
            //6 = max value, see https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=59b6e00e406d8be85b4a79ef9f7fa8a5
            String::with_capacity(6)
        };
        if let Err(problem) = push_encoded(value, &mut new_value) {
            return Err(problem);
        } else {
            return Ok(new_value);
        }
    }
    /// Adds the [Base62] representation of the `value` argument to the `prefix` [`String`]
    /// # Arguments
    /// The first argument, `value`, is a [`u32`] indicating what numerical value should be encoded into [`char`]s and attached (via [`String::push`]) to the second argument, `prefix`, a mutable reference to a [`String`]. 
    ///
    /// # Output
    /// This function returns a [`Result<(),EncodingError>`]. If the empty tuple `()` is returned, then a representation of `value` as a [`String`] in [Base62] has successfully been added to the end of the specified `prefix` [`String`]. However, if an [`EncodingError`]
    /// is returned instead (or the function panics), the conversion has failed. If you encounter such a case, please record the input that caused the error and open an issue on Github so the bug can (hopefully) be removed. 
    ///# Example
    /// If you wanted to control how automatically generated [`String`]s started so that they could be used to, for example, create valid Rust identifiers using this library, you could use this function to automatically add encoded values to existing [`String`]s
    /// ```
    /// # use ascii_basing::encoding;
    /// # fn wrapper() -> Result<(),encoding::EncodingError> {
    /// let medium_number = 35 * 62 * 62 + 52 * 62 + 9; // = 137773
    /// let mut prefix = String::with_capacity(4);
    /// prefix.push('_');
    /// encoding::push_encoded(medium_number,&mut prefix)?;
    /// assert_eq!(String::from("_zQ9"),prefix);
    /// # Ok(())
    /// # }
    /// ```
    /// [Base62]: https://en.wikipedia.org/wiki/Base62
    pub fn push_encoded(mut value: u32, prefix: &mut String) -> Result<(),EncodingError> {
        if value / 62 != 0 {
            push_encoded(value / 62,prefix)?;
        }
        value %= 62;
        if let Some(lowercase) = char::from_digit(value,36) {
            prefix.push(lowercase);
        } else {
            value -= 26;
            prefix.push(char::from_digit(value,36).ok_or(EncodingError::new("There was an attempt to directly parse a value unrepresentable in a single digit of base 62, that is, a value > 62"))?.to_ascii_uppercase());
        }
        Ok(())

    }
}
/// Changing [`String`]s to [`u32`] values.
pub mod decoding {
    use crate::{Formatter,Display,Error,fmt};
    use std::str::Chars;
    const DECODING_ERROR: &str = r"characters in the string passed to decode should only contain characters 0-9, a-z, and/or Z-A";
    #[derive(Debug)]
    /// The [`Error`] Type for the [`decoding`](crate::decoding) module
    ///
    /// A simple error type that may be returned by the [`decode`] function. Unlike [`EncodingError`](crate::encoding::EncodingError), this type may be returned for non-bug reasons, 
    /// such as the [`Chars`] iterator passed to [`decode`] containing a value that cannot be decoded in [Base62](https://en.wikipedia.org/wiki/Base62), that is, a value that is not 
    /// one of the following: `0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`.
    pub struct DecodingError {
        message: String,
    }
    impl DecodingError {
        fn new(problem: &'static str) -> DecodingError {
            return DecodingError { message: String::from(problem) };
        }
    }
    impl Display for DecodingError {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "Decoding a value failed because: {}", self.message)
        }
    }
    impl Error for DecodingError {}
    /// Converts [`Chars`] to [`u32`]s using a [Base62] algorithm
    ///
    /// # Arguments
    /// This function only takes one argument, `written`, a [`Chars`]. The most common way to obtain [`Chars`] is to use the [`chars()`](https://doc.rust-lang.org/std/string/struct.String.html#method.chars) method on
    /// a [`String`]. The function will decode each [`char`] in the [`Chars`] in decreasing value or [big-endian](https://en.wikipedia.org/wiki/Endianness), treating each [`char`] as a single digit in [Base62]
    /// and considering the 'number' as completed as soon as the [`next_back()`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html#tymethod.next_back) method on `written` returns [`None`].
    /// # Output
    /// This function returns a [`Result<u32,DecodingError>`]. If a [`u32`] is returned, then the [`Chars`] have been successfully decoded to a [`u32`] representation. If a [`DecodingError`] is returned,
    /// there was an error. The only expected reason that an Error may be returned is if the [`Chars`] iterator returned a [`char`] that is not a [Base62] digit, that is, a [`char`] that is not one of the
    /// following: `0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`. Any other behavior that causes a [`DecodingError`] to be returned is a bug. If you encounter such a situation, please open an issue on Github.
    /// `decode` is the inverse of [`encode`]. If the [`Chars`] passed to `decode` is directly derived from the [`String`] returned by [`encode`], then `decode` will return the [`u32`] value that was passed
    /// to [`encode`] to create the [`String`].
    /// # Example
    /// If you needed to convert one of the [`String`]s created using this library back to a [`u32`], you could use this function, like so:
    /// ```
    /// # use ascii_basing::decoding;
    /// let short_representation = String::from("4GFfc3"); // u32::MAX
    /// let big_number = decoding::decode(short_representation.chars()).unwrap();
    /// assert_eq!(u32::MAX, big_number);
    /// ```
    /// # Panics
    /// This function panics if the `written` [`Chars`] argument would decode to a value that cannot be represented by a [`u32`], that is, a value greater than [`u32::MAX`]. For example:
    /// ```should_panic
    /// # use ascii_basing::decoding;
    /// let big_representation = String::from("4GFfc4"); // u32::MAX + 1
    /// decoding::decode(big_representation.chars());
    /// ```
    /// [Base62]: https://en.wikipedia.org/wiki/Base62
    /// [`encode`]: crate::encoding::encode
    pub fn decode(mut written: Chars) -> Result<u32,DecodingError> {
        let mut power: u64 = 1;
        let mut total: u32 = 0;
        while let Some(digit) = written.next_back() {
            if digit.is_ascii_uppercase() {
                total += (26 + char::to_digit(digit,36).ok_or(DecodingError::new(DECODING_ERROR))?) * (power as u32);
            } else {
                total += char::to_digit(digit,36).ok_or(DecodingError::new(DECODING_ERROR))? * (power as u32);
            }
            power *= 62;
        }
        return Ok(total);
    }
}
#[cfg(test)]
mod tests {
    use crate::encoding::*;
    use crate::decoding::*;
    #[test]
    fn encode_premade() {
        let numbers = [0,u32::MAX];
        let representations = [String::from("0"),String::from("4GFfc3")];
        //minimum and maximum
        let mut looper = 0;
        while looper < numbers.len() {
            if let Ok(encoded) = encode(numbers[looper],None) {
                assert_eq!(encoded,representations[looper]);
                if let Ok(decoded) = decode(encoded.chars()) {
                    assert_eq!(numbers[looper],decoded);
                } else {
                    panic!("encoded value {} didn't decode to the same value it was before encoding",looper);
                }
            } else {
                panic!("the encode function threw an error when it was passed {}, a test case that should've encoded to \"{}\"",numbers[looper],representations[looper]);
            }
            looper += 1;
        }
    }
    #[test]
    fn encode_automatically() {
        let mut numbers: Vec<u32> = Vec::with_capacity(5 * 2 + 1);
        let mut representations: Vec<String> = Vec::with_capacity(5 * 2 + 1);
        let mut under = String::with_capacity(5);
        let mut over = String::with_capacity(6);
        let mut power: u64 = 62;
        over.push('1');
        representations.push(over.clone());
        numbers.push(1);
        loop {
            match under.len() {
                0 => {
                    numbers.push(61);
                    numbers.push(62);
                },
                _ => {
                    let new_value = numbers.get(numbers.len() - 2).expect("There's an issue with the automatic generation of values for this test") + 61 * (power as u32);
                    numbers.push(new_value);
                    numbers.push(new_value + 1);
                    power *=  62;
                },
            }
            under.push('Z');
            over.push('0');
            if over.len() > 5 {
                break;
            } else {
                representations.push(under.clone());
                representations.push(over.clone());
            }
        }
        representations.push(under);
        representations.push(over);
        let mut test_looper: usize = 0;
        while test_looper < numbers.len() {
            let latest_rep = representations.get(test_looper).expect("Fewer String representations were generated than numbers to encode");
            if let Ok(encoded) = encode(*numbers.get(test_looper).unwrap(),Some(test_looper / 2 + 1)) {
                assert_eq!(encoded,*latest_rep);
                if let Ok(decoded) = decode(encoded.chars()) {
                    assert_eq!(*numbers.get(test_looper).unwrap(),decoded);
                } else {
                    panic!("encoded value {} didn't decode to the same value it was before encoding",test_looper);
                }
            } else {
                panic!("the encode function threw an error when it was passed {}, a test case that should've encoded to \"{}\"",numbers.get(test_looper).unwrap(),latest_rep);
            }
            test_looper += 1;
        }
    }
        
}
