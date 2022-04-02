use std::fmt::Display;

pub type PResult<I,O,E> = Result<(I, O), E>;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PError{
    UnexpectedInput,
    UnsufficientInputLength,
}

impl Display for PError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PError::UnexpectedInput => f.write_str("Unexpected input"),
            PError::UnsufficientInputLength => f.write_str("Unsufficient input length"),
        }
    }
}

pub trait Parser<I,O,E> {
    fn run(&mut self, input: I) -> PResult<I,O,E>;
}

impl<I,O,E,F> Parser<I,O,E> for F where 
 F: FnMut(I) -> PResult<I,O,E> {
    fn run(&mut self, input: I) -> PResult<I,O,E> {
        (*self)(input)
    }
}
