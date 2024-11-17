use std::io;

use nom::Err;

use crate::{parse::SplashParseError, run::SplashRuntimeError};

#[derive(Debug)]
pub enum SplashError<'a> {
    Parse(Err<SplashParseError<'a>>),
    Runtime(SplashRuntimeError<'a>),
    Io(io::Error),
}

impl<'a> From<Err<SplashParseError<'a>>> for SplashError<'a> {
    fn from(value: Err<SplashParseError<'a>>) -> Self {
        Self::Parse(value)
    }
}

impl<'a> From<SplashRuntimeError<'a>> for SplashError<'a> {
    fn from(value: SplashRuntimeError<'a>) -> Self {
        Self::Runtime(value)
    }
}

impl From<io::Error> for SplashError<'_> {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
