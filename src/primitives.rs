use regex::Regex;
use crate::parser::{Parser, PError};

pub fn char<'a>(ch: char) -> impl Parser<&'a str, &'a str, PError> {
    move |input: &'a str| {
        if input.len() == 0 { return Err(PError::UnsufficientInputLength);}
        match input.strip_prefix(ch) {
            Some(rem) => Ok((rem, &input[..1])),
            None => Err(PError::UnexpectedInput)
        }
    }
}

pub fn any_char<'a>() -> impl Parser<&'a str, &'a str, PError> {
    move |input: &'a str| {
        if !input.is_empty() {
            return Ok((&input[1..], &input[..1]));
        }
        Err(PError::UnexpectedInput)
    }
}

pub fn string<'a>(string: &'a str) -> impl Parser<&'a str, &'a str, PError>{
    move |input: &'a str| {
        if input.len() < string.len() {
            return Err(PError::UnsufficientInputLength) 
        }
        match input.strip_prefix(string) {
            Some(rem) => Ok((rem, &input[..string.len()])),
            None => Err(PError::UnexpectedInput)
        }
    }
}

pub fn digit<'a>(radix: u32) -> impl Parser<&'a str, &'a str, PError>{
    move |input: &'a str| {
        if input.len() == 0 { return Err(PError::UnsufficientInputLength) } 
        if input.chars().next().unwrap().is_digit(radix) {
            return Ok((&input[1..], &input[..1]))
        }
        Err(PError::UnexpectedInput)
    }
}

pub fn digits<'a>(radix: u32) -> impl Parser<&'a str, &'a str, PError>{
    move |input: &'a str| {
        if input.len() == 0 { return Err(PError::UnsufficientInputLength)} 
        let digit_count = input.chars().take_while(|el|{el.is_digit(radix)}).count();
        if digit_count > 0 { return Ok((&input[digit_count..], &input[0..digit_count]));}
        Err(PError::UnexpectedInput)
    }
}

pub fn letter<'a>() -> impl Parser<&'a str, &'a str, PError> {
    move |input: &'a str| {
        if input.len() == 0 { return Err(PError::UnsufficientInputLength) } 
        let first_letter = &input[..1];
        if first_letter.chars().next().unwrap().is_alphabetic() {
            return Ok((&input[1..], first_letter));
        }
        Err(PError::UnexpectedInput)
    }
}

pub fn letters<'a>() -> impl Parser<&'a str, &'a str, PError>{
    move |input: &'a str| {
        if input.len() == 0 { return Err(PError::UnsufficientInputLength)} 
        let letter_count = input.chars().take_while(|el|{el.is_alphabetic()}).count();
        if letter_count > 0 { return Ok((&input[letter_count..], &input[..letter_count])); }
        return Err(PError::UnexpectedInput)
    }
}

pub fn whitespace<'a>() -> impl Parser<&'a str, &'a str, PError>{
    move |input: &'a str| {
        if input.len() == 0 { return Err(PError::UnsufficientInputLength)} 
        let whitespace_count = input.chars().take_while(|c|{c.is_whitespace()}).count();
        if whitespace_count > 0 { return Ok((&input[whitespace_count..], &input[..whitespace_count]));}
        Err(PError::UnexpectedInput)
    }
}

pub fn optional_whitespace<'a>() -> impl Parser<&'a str, &'a str, PError>{
    move |input: &'a str| {
        let whitespace_count = input.chars().take_while(|c|{c.is_whitespace()}).count();
        if whitespace_count > 0 { return Ok((&input[whitespace_count..], &input[..whitespace_count]));}
        Ok((input, ""))
    }
}

pub fn peek<'a>() -> impl Parser<&'a str, &'a str, PError>{
    move |input: &'a str| {
        if input.len() == 0 { return Err(PError::UnsufficientInputLength)} 
        Ok((input, &input[..1]))
    }
}

pub fn any_of_string<'a>(string: &'a str) -> impl Parser<&'a str, &'a str, PError>{
    move |input: &'a str| {
        if input.len() == 0 { return Err(PError::UnsufficientInputLength)}
        for match_ch in string.chars() { 
            if input.starts_with(match_ch) {
                return Ok((&input[1..], &input[..1]));
            }
        }
        Err(PError::UnexpectedInput)
    }
}

pub fn regex<'a>(string: &'a str) -> impl Parser<&'a str, &'a str, PError>{
    let regex = Regex::new(string).unwrap();
    move |input: &'a str| {
        if input.len() == 0 { return Err(PError::UnsufficientInputLength) }
        let match_count = input.char_indices().
            take_while(|(i,_)| {regex.is_match(&input[*i..*i+1])}).count();
        if match_count > 0 { return Ok((&input[match_count..], &input[..match_count])); }
        Err(PError::UnexpectedInput)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::{Debug, Display};
    #[test]
    fn parse_test_failure_letter() {

        test_parser_success(char('a'), "abc", "bc", "a");
        test_parser_failure(char('a'), "");
        test_parser_failure(char('a'), "b");

        test_parser_success(any_char(), "abc", "bc", "a");
        test_parser_failure(any_char(), "");

        test_parser_success(string("aB ch-123.+*"), "aB ch-123.+*", "", "aB ch-123.+*");
        test_parser_failure(string("adf "), "ad f");
        test_parser_failure(string("aB ch-123.+*"), "aB ch-123");

        test_parser_success(digit(10), "123", "23", "1");
        test_parser_success(digit(16), "adf", "df", "a");
        test_parser_failure(digit(10), "adf");
        test_parser_failure(digit(16), "hji");
        
        test_parser_success(digits(10), "1234567890rem", "rem", "1234567890");
        test_parser_success(digits(16), "10F2Cjorn", "jorn", "10F2C");
        test_parser_failure(digits(10), "adf");
        test_parser_failure(digits(16), "hji");

        test_parser_success(letter(), "a123", "123", "a");
        test_parser_failure(letter(), "1");
        test_parser_failure(letter(), "+");
        test_parser_failure(letter(), "");

        test_parser_success(letters(), "ABCdefghijklmnopqrstuvwrem123", "123", "ABCdefghijklmnopqrstuvwrem");
        test_parser_failure(letters(), "1");
        test_parser_failure(letters(), "-");
        test_parser_failure(letters(), "");

        test_parser_success(whitespace(), " ABC", "ABC", " ");
        test_parser_success(whitespace(), "   ABC", "ABC", "   ");
        test_parser_failure(whitespace(), "1 ");
        test_parser_failure(whitespace(), "-");
        test_parser_failure(whitespace(), "");

        test_parser_success(optional_whitespace(), " ABC", "ABC", " ");
        test_parser_success(optional_whitespace(), "   ABC", "ABC", "   ");
        test_parser_success(optional_whitespace(), "ABC", "ABC", "");

        test_parser_success(peek(), "abc", "abc", "a");
        test_parser_failure(peek(), "");

        test_parser_success(any_of_string("abc"), "ckc", "kc", "c");
        test_parser_failure(any_of_string("abcd"), "inpuz");
        test_parser_failure(any_of_string("abcd"), "");
        
        test_parser_success(regex(r"[a-z]"), "abc", "", "abc");
    }

    fn test_parser_success<I,O,E>(mut parser: impl Parser<I,O,E>,input: I, eq_rem: I, eq_parsed: O) where 
        E: Debug,
        I: Debug + PartialEq,
        O: Debug + PartialEq 
    {
        let ret = parser.run(input);
        assert!(ret.is_ok());
        let (remainder, parsed) = ret.unwrap();
        assert_eq!(remainder, eq_rem);
        assert_eq!(parsed, eq_parsed);
    }

    fn test_parser_failure<I,O,E>(mut parser: impl Parser<I,O,E>, input: I) where 
        E: Debug + Display,
        I: Debug,
        O: Debug 
    {
        let res = parser.run(input);
        assert!(res.is_err());
        println!("{}", res.unwrap_err());
    }
}

