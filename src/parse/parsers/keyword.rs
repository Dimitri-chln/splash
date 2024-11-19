use nom::{Compare, CompareResult, Err, IResult, InputTake, Parser};

use crate::parse::SplashParseError;

pub enum Keyword {
    Let,
    If,
    Else,
    While,
    Return,
    Fn,
}

impl Keyword {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Let => "let",
            Self::If => "if",
            Self::Else => "else",
            Self::While => "while",
            Self::Return => "return",
            Self::Fn => "fn",
        }
    }
}

pub fn keyword<'a>(keyword: Keyword) -> impl Parser<&'a str, &'a str, SplashParseError<'a>> {
    move |input: &'a str| {
        let keyword = keyword.as_str();
        let res: IResult<_, _, SplashParseError> = match input.compare(keyword) {
            CompareResult::Ok => Ok(input.take_split(keyword.len())),
            _ => Err(Err::Error(SplashParseError::InvalidKeyword)),
        };
        res
    }
}
