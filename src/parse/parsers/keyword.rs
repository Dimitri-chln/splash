use nom::{Compare, CompareResult, Err, IResult, InputTake, Parser};

use crate::parse::SplashParseError;

pub enum Keyword {
    If,
    Else,
    While,
    Return,
    Fn,
}

impl Keyword {
    pub fn as_str(&self) -> &str {
        match self {
            Self::If => "if",
            Self::Else => "else",
            Self::While => "while",
            Self::Return => "return",
            Self::Fn => "fn",
        }
    }
}

pub fn keyword<'a>(keyword: Keyword) -> impl Parser<&'a str, &'a str, SplashParseError<'a>> {
    move |i: &'a str| {
        let k = keyword.as_str();
        let res: IResult<_, _, SplashParseError> = match i.compare(k) {
            CompareResult::Ok => Ok(i.take_split(k.len())),
            _ => Err(Err::Error(SplashParseError::InvalidKeyword)),
        };
        res
    }
}
