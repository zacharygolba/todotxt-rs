//! Contains data structures and traits used to parse a list of tasks.

use crate::task::Task;
#[cfg(feature = "rayon")]
use rayon::{
    iter::{plumbing::UnindexedConsumer, ParallelIterator},
    str::{Lines as ParallelLines, ParallelString},
};
use std::{iter::FusedIterator, str::Lines};

/// Provides methods for types that can be used as parser input.
pub trait Input {
    /// Returns an iterator of tasks contained in `self`.
    fn tasks(&self) -> Iter<'_>;
}

/// An iterator over the tasks of a given input.
#[derive(Clone, Debug)]
pub struct Iter<'a> {
    lines: Lines<'a>,
}

#[allow(missing_docs)]
#[cfg(feature = "rayon")]
pub trait ParallelInput {
    fn par_tasks(&self) -> ParallelIter;
}

#[allow(missing_docs)]
#[cfg(feature = "rayon")]
#[derive(Clone, Debug)]
pub struct ParallelIter<'a> {
    lines: ParallelLines<'a>,
}

pub(crate) trait Parse<'a> {
    type Output;
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self::Output>;
}

pub(crate) fn parse<'a, T>(input: &'a str) -> Option<T::Output>
where
    T: Parse<'a>,
{
    match T::parse(input) {
        Err(ref e) if cfg!(debug) => unreachable!("Error: {:#?}", e),
        result => result.ok().map(|(_, output)| output),
    }
}

impl Input for str {
    fn tasks(&self) -> Iter<'_> {
        Iter {
            lines: self.lines(),
        }
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let line = self.lines.next_back()?.trim();

        if line.is_empty() {
            self.next_back()
        } else {
            parse::<Self::Item>(line)
        }
    }
}

impl<'a> FusedIterator for Iter<'a> {}

impl<'a> Iterator for Iter<'a> {
    type Item = Task<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?.trim();

        if line.is_empty() {
            self.next()
        } else {
            parse::<Self::Item>(line)
        }
    }
}

#[cfg(feature = "rayon")]
impl ParallelInput for str {
    fn par_tasks(&self) -> ParallelIter {
        ParallelIter {
            lines: self.par_lines(),
        }
    }
}

#[cfg(feature = "rayon")]
impl<'a> ParallelIterator for ParallelIter<'a> {
    type Item = Task<'a>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.lines
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .filter_map(parse::<Self::Item>)
            .drive_unindexed(consumer)
    }
}
