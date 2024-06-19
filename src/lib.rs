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
//! # 62 Base Converter & Deconverter
//! Convert your positive [integers](https://doc.rust-lang.org/std/primitive.u32.html) to [ASCII](https://en.wikipedia.org/wiki/ASCII)-safe [`Strings`](https://doc.rust-lang.org/std/string/struct.String.html) for situations where a concise [`String`] representation of an [integer](https://doc.rust-lang.org/std/primitive.u32.html) is desired, such as when automatically generating file names or identifiers.
//!
//! Each possible integer value is assigned a unique [`String`] representation, allowing for lossless conversion between [`Strings`] and [`u32`] values. Each additional character in a [`String`] representation can be thought of as another digit in a [Base62](https://en.wikipedia.org/wiki/Base62) system - lower values have shorter representations - but thanks to the large amount of values each digit could be, representations (of [32-bit integers](https://doc.rust-lang.org/std/primitive.u32.html)) are never longer than 6 characters long.
//!
//! # Example
//! Let's imagine that you were working on a library that automatically generated identical fields (with different names) in a Rust data structure. In order to support large quantities of fields, large identifiers might form unless the data is encoded. For this example, we'll look at only the largest value you hypothetically need to support, [`u32::MAX`] - or, represented in base 10, `4294967295`. If you needed to limit the size of the identifiers so the data structure could be efficiently parsed and sent online, you would need to encode the data. 
//! Using this library, you could easily condense `4294967295` down to just
//! # Alternatives
//! This library creates output similar to [base-62], but the implementation is distinct. The main advantages of this library (as of now) are encoding `0` to the string `"0"` rather than an empty String, and allowing for users to give [`Option`]al hints about the size of encoded values (which are used to pre-allocate String capacity). [base-62] has the advantage of allowing encoding and decoding of unsigned integers in `&[u8]` format rather than only u32 format. 
//!
//! In general, it is best to use [base-62]
//! when large numbers or byte sequences need to be encoded, and to use this library in most other situations.
//!
//![base-62]: https://crates.io/crates/base-62
//![`Strings`]: https://doc.rust-lang.org/std/string/struct.String.html
use std::error::Error;
use std::fmt;
use std::fmt::{Formatter,Display};
///anoither documentation experimen5t
pub mod encoding {
    use crate::{Error,fmt,Formatter,Display};
    #[derive(Debug)]
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
    pub fn encode(value: u32, size: Option<usize>) -> Result<String,EncodingError> {
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
pub mod decoding {
    use crate::{Formatter,Display,Error,fmt};
    use std::str::Chars;
    const DECODING_ERROR: &str = r"characters in the string passed to decode should only contain characters 0-9, a-z, and/or Z-A";
    #[derive(Debug)]
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
        let numbers = [199888+558+35,0,u32::MAX];
        let representations = [String::from("Q9z"),String::from("0"),String::from("4GFfc3")];
        //traditional test, minimum and maximum
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
    fn encode_zero() {
        if let Ok(encoded) = encode(0,Some(1)) {
            assert_eq!(encoded,String::from("0"));
            if let Ok(decoded) = decode(encoded.chars()) {
                assert_eq!(0,decoded);
            } else {
                panic!("the encoded value didn't decode to the same value it was before encoding");
            }
        } else {
            panic!("the encode function threw an error when it was passed 0, a test case that should've encoded to \"0\"");
        }
    }
        
}
