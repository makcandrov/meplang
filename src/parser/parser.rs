use std::ops::Deref;
use core::fmt::Debug;

use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use crate::parser::error::new_error_from_pair;

#[derive(Parser)]
#[grammar = "./src/parser/meplang.pest"]
pub struct MeplangParser;

pub trait FromPair where Self: Sized + Debug + Clone{
    fn from_pair(pair: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>>;
}

pub fn map_unique_child<T>(
    pair: Pair<Rule>,
    f: fn(Pair<Rule>) -> T,
) -> T {
    let mut inner = pair.into_inner();
    let child = inner.next().unwrap();
    let res = f(child);
    assert!(inner.next() == None);
    res
}

pub fn get_next<'a, 'rule>(pairs: &'a mut Pairs<'rule, Rule>, expected: Rule) -> Pair<'rule, Rule> {
    let pair = pairs.next().unwrap();
    assert!(pair.as_rule() == expected);
    pair
}

#[derive(Debug, Clone)]
pub struct Located<T> {
    pub start: usize,
    pub end: usize,
    pub inner: T,
}

impl<T: FromPair> FromPair for Located<T> {

    fn from_pair(pair: Pair<'_, Rule>) -> Result<Located<T>, pest::error::Error<Rule>> {
        Ok(Self {
            start: pair.as_span().start(),
            end: pair.as_span().end(),
            inner: T::from_pair(pair)?,
        })
    }
}

impl<T> Deref for Located<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl FromPair for bytes::Bytes {
    fn from_pair(hex_litteral: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(hex_litteral.as_rule() == Rule::hex_litteral);

        if hex_litteral.as_str().len() % 2 != 0 {
            return Err(new_error_from_pair(&hex_litteral, "Hex litterals must be odd size".to_owned()));
        }

        match hex::decode(hex_litteral.as_str().strip_prefix("0x").unwrap()) {
            Ok(decoded) => Ok(decoded.into()),
            Err(err) => Err(new_error_from_pair(&hex_litteral, err.to_string())),
        }
    }
}

impl FromPair for String {
    fn from_pair(string_litteral: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(string_litteral.as_rule() == Rule::string_litteral);
        
        let mut string_inner = string_litteral.into_inner();
        let res = string_inner.next().unwrap();
        assert!(res.as_rule() == Rule::string_inner);
        assert!(string_inner.next() == None);

        Ok(res.as_str().to_owned())
    }
}

