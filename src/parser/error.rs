use pest::{error::ErrorVariant, iterators::Pair, Position};
use crate::parser::parser::Rule;

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
