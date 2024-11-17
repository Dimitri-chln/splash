use crate::parse::Program;

use super::{block, context::Context, SplashRuntimeError};

pub struct Runtime<'a> {
    program: Program<'a>,
}

impl<'a> Runtime<'a> {
    pub fn new(program: Program<'a>) -> Self {
        Self { program }
    }

    pub fn start(self) -> Result<(), SplashRuntimeError<'a>> {
        block::run(&self.program, &mut Context::new())?;
        Ok(())
    }
}
