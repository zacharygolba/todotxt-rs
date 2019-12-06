use crate::parser::Parse;
#[cfg(feature = "serde")]
use serde::Serialize;
use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
};

/// The priority of an incomplete task.
///
/// ## Equality and Ordering
///
/// The implementation of equality and ordering operators uphold the rules
/// defined in the specification. This enables you compare priorities using
/// the same operators you would use to, for example, compare numbers.
///
/// ```
/// # extern crate todotxt;
/// #
/// # use todotxt::Priority;
/// #
/// # fn main() {
/// assert!(Priority::A > Priority::B);  // (A) is a higher priority than (B)
/// assert!(Priority::B < Priority::A);  // (B) is a lower priority than (A)
/// assert!(Priority::A == Priority::A); // (A) is the same priority as (A)
/// # }
/// ```
#[allow(missing_docs)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq)]
pub enum Priority {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z
}

impl Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self)
    }
}

impl<'a> Parse<'a> for Priority {
    type Output = Priority;

    fn parse(input: &str) -> nom::IResult<&str, Self::Output> {
        named!(value<&str, Priority>,
            alt!(
                value!(Priority::A, char!('A')) |
                value!(Priority::B, char!('B')) |
                value!(Priority::C, char!('C')) |
                value!(Priority::D, char!('D')) |
                value!(Priority::E, char!('E')) |
                value!(Priority::F, char!('F')) |
                value!(Priority::G, char!('G')) |
                value!(Priority::H, char!('H')) |
                value!(Priority::I, char!('I')) |
                value!(Priority::J, char!('J')) |
                value!(Priority::K, char!('K')) |
                value!(Priority::L, char!('L')) |
                value!(Priority::M, char!('M')) |
                value!(Priority::N, char!('N')) |
                value!(Priority::O, char!('O')) |
                value!(Priority::P, char!('P')) |
                value!(Priority::Q, char!('Q')) |
                value!(Priority::R, char!('R')) |
                value!(Priority::S, char!('S')) |
                value!(Priority::T, char!('T')) |
                value!(Priority::U, char!('U')) |
                value!(Priority::V, char!('V')) |
                value!(Priority::W, char!('W')) |
                value!(Priority::X, char!('X')) |
                value!(Priority::Y, char!('Y')) |
                value!(Priority::Z, char!('Z'))
            )
        );

        delimited!(input, char!('('), value, char!(')'))
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Priority) -> Option<Ordering> {
        let lhs = *self as usize;
        let rhs = *other as usize;

        if lhs == rhs {
            Some(Ordering::Equal)
        } else if lhs < rhs {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}
