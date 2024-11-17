use nom::error::{ErrorKind, ParseError};

#[derive(Debug)]
pub enum SplashParseError<'a> {
    Nom(&'a str, ErrorKind),
    Stack(Vec<Self>),
    InvalidKeyword,
}

impl<'a> ParseError<&'a str> for SplashParseError<'a> {
    fn from_error_kind(input: &'a str, kind: ErrorKind) -> Self {
        SplashParseError::Nom(input, kind)
    }

    fn append(input: &'a str, kind: ErrorKind, other: Self) -> Self {
        match other {
            Self::Nom(_, _) => Self::Stack(vec![other, Self::from_error_kind(input, kind)]),
            Self::Stack(mut stack) => {
                stack.push(Self::from_error_kind(input, kind));
                Self::Stack(stack)
            }
            other => Self::Stack(vec![other, Self::from_error_kind(input, kind)]),
        }
    }
}
