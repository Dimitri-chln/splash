use crate::parse::Program;

use super::{block, context::Context, SplashRuntimeError};

pub struct Runtime<'a> {
    program: Program<'a>,
}

impl<'a> Runtime<'a> {
    #[must_use]
    pub fn new(program: Program<'a>) -> Self {
        Self { program }
    }

    /// # Errors
    /// This function will return an error if the program cannot be run to completion successfully.
    pub fn start(self) -> Result<(), SplashRuntimeError<'a>> {
        block::run(&self.program, &mut Context::new())?;
        Ok(())
    }
}
