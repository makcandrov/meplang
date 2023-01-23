use crate::parser::parser::Rule;
use pest::{error::ErrorVariant, iterators::Pair, Position, Span};

use super::parser::{Located, Location};

pub fn new_error_from_pair(pair: &Pair<Rule>, message: String) -> pest::error::Error<Rule> {
    pest::error::Error::<Rule>::new_from_span(
        ErrorVariant::<Rule>::CustomError { message },
        pair.as_span(),
    )
}

pub fn new_generic_error(message: String) -> pest::error::Error<Rule> {
    pest::error::Error::<Rule>::new_from_pos(
        ErrorVariant::<Rule>::CustomError { message },
        Position::new(" ", 0).unwrap(),
    )
}

pub fn new_error_from_located<T>(
    input: &str,
    located: &Located<T>,
    message: &str,
) -> pest::error::Error<Rule> {
    new_error_from_location(input, &located.location, message)
}

pub fn new_error_from_location(
    input: &str,
    location: &Location,
    message: &str,
) -> pest::error::Error<Rule> {
    pest::error::Error::<Rule>::new_from_span(
        ErrorVariant::<Rule>::CustomError { message: message.to_owned() },
        Span::new(input, location.start, location.end).unwrap(),
    )
}
