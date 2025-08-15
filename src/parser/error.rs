use pest::error::ErrorVariant;
use pest::iterators::Pair;
use pest::{Position, Span};

use super::parser::{Located, Location};
use crate::parser::parser::Rule;

pub type PestError = Box<pest::error::Error<Rule>>;

pub fn new_error_from_pair(pair: &Pair<Rule>, message: String) -> PestError {
    Box::new(pest::error::Error::<Rule>::new_from_span(
        ErrorVariant::<Rule>::CustomError { message },
        pair.as_span(),
    ))
}

pub fn new_generic_error(message: String) -> PestError {
    Box::new(pest::error::Error::<Rule>::new_from_pos(
        ErrorVariant::<Rule>::CustomError { message },
        Position::new(" ", 0).unwrap(),
    ))
}

pub fn new_error_from_located<T>(input: &str, located: &Located<T>, message: &str) -> PestError {
    new_error_from_location(input, &located.location, message)
}

pub fn new_error_from_location(input: &str, location: &Location, message: &str) -> PestError {
    Box::new(pest::error::Error::<Rule>::new_from_span(
        ErrorVariant::<Rule>::CustomError {
            message: message.to_owned(),
        },
        Span::new(input, location.start, location.end).unwrap(),
    ))
}
