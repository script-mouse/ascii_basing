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
//! Convert your positive [integers](https://doc.rust-lang.org/std/primitive.u32.html) to [ASCII](https://en.wikipedia.org/wiki/ASCII)-safe [`Strings`](https://doc.rust-lang.org/std/string/struct.String.html) for situations where a concise [`String`](https://doc.rust-lang.org/std/string/struct.String.html) representation of an [integer](https://doc.rust-lang.org/std/primitive.u32.html) is desired, such as when automatically generating file names or identifiers.
//! This library creates output similar to [Base-62](), but the implementation is distinct.
//! Here are a few of the key differences that affect output:
//! |Crate Name  |Supports Integers larger than [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)?|
//! |:----------:|:--------------------------------------------------------------------------------------:|
//! |ascii_basing|No                                                                                      |
//! |base-62     |Yes                                                                                     |
//!
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
        let mut power: u32 = 1;
        let mut total: u32 = 0;
        while let Some(digit) = written.next_back() {
            if digit.is_ascii_uppercase() {
                total += (26 + char::to_digit(digit,36).ok_or(DecodingError::new(DECODING_ERROR))?) * power;
            } else {
                total += char::to_digit(digit,36).ok_or(DecodingError::new(DECODING_ERROR))? * power;
            }
            power *= 62;
        }
        return Ok(total);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::*;
    use crate::decoding::*;
    #[test]
    fn encode_three() {
        if let Ok(encoded) = encode(199888+558+35,Some(3)) {
            assert_eq!(encoded,String::from("Q9z"));
            if let Ok(decoded) = decode(encoded.chars()) {
                assert_eq!(199888+558+35,decoded);
            } else {
                panic!("the encoded value didn't decode to the same value it was before encoding");
            }
        } else {
            panic!("the encode function threw an error when it was passed 200481, a test case that should've encoded to \"Q9z\"");
        }
    }
}
