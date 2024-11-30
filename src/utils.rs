use crate::parse::SplashParseError;

type Line = usize;
type Column = usize;

/// # Panics
/// This function will panic if the input in not properly enclosed in curly brackets.
#[must_use]
pub fn locate_error(input: &str, error: &SplashParseError) -> Option<(Line, Column)> {
    match error {
        SplashParseError::Nom(remaining, _) => {
            let consumed = input
                .strip_prefix('{')
                .unwrap()
                .strip_suffix(remaining)
                .unwrap();

            let line: Line = consumed.chars().filter(|&c| c == '\n').count();
            let column: Column = consumed.lines().nth(line).unwrap_or_default().len();

            Some((line + 1, column + 1))
        }
        _ => None,
    }
}
