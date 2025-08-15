use core::fmt::Debug;
use std::ops::{Deref, DerefMut};

use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

use crate::parser::error::PestError;

#[derive(Parser)]
#[grammar = "./src/parser/meplang.pest"]
pub struct MeplangParser;

pub trait FromPair
where
    Self: Sized + Debug + Clone,
{
    fn from_pair(pair: Pair<Rule>) -> Result<Self, Box<pest::error::Error<Rule>>>;
}

pub fn map_unique_child<T>(pair: Pair<Rule>, f: fn(Pair<Rule>) -> T) -> T {
    let mut inner = pair.into_inner();
    let child = inner.next().unwrap();
    let res = f(child);
    assert!(inner.next().is_none());
    res
}

pub fn get_next<'rule>(pairs: &mut Pairs<'rule, Rule>, expected: Rule) -> Pair<'rule, Rule> {
    let pair = pairs.next().unwrap();
    assert!(pair.as_rule() == expected);
    pair
}

#[derive(Debug, Clone)]
pub struct Located<T> {
    pub location: Location,
    pub inner: T,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

impl<T: FromPair> FromPair for Located<T> {
    fn from_pair(pair: Pair<'_, Rule>) -> Result<Located<T>, PestError> {
        Ok(Self {
            location: Location {
                start: pair.as_span().start(),
                end: pair.as_span().end(),
            },
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

impl<T> DerefMut for Located<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
