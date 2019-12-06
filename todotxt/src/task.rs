use std::{
    borrow::Cow,
    fmt::{self, Debug, Display, Formatter},
};

use chrono::NaiveDate;
use nom::{self, space, IResult};
#[cfg(feature = "serde")]
use serde::ser::{Serialize, SerializeStruct, Serializer};

use parser::Parse;
use priority::Priority;
use tags::{Tag, Tags};

/// The disjoint state of complete and incomplete tasks.
///
/// The purpose of this type is to enforce the invariants defined in the
/// specification. While a complete task and incomplete task can be viewed as
/// distinct types, for convenience in the common cases, this library choses to
/// wrap the disjoint union of a complete and incomplete task in a single type.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Complete(Option<(NaiveDate, NaiveDate)>),
    Incomplete(Option<Priority>, Option<NaiveDate>),
}

/// A single complete or incomplete task.
#[derive(Eq, PartialEq)]
pub struct Task<'a> {
    state: State,
    text: Cow<'a, str>,
}

impl<'a> Debug for Task<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let tags: Vec<Tag> = self.tags().collect();

        f.debug_struct("Task")
            .field("completion_date", &self.completion_date())
            .field("creation_date", &self.creation_date())
            .field("description", &self.description())
            .field("is_complete", &self.is_complete())
            .field("priority", &self.priority())
            .field("tags", &tags)
            .finish()
    }
}

impl<'a> Parse<'a> for NaiveDate {
    type Output = NaiveDate;

    fn parse(input: &str) -> IResult<&str, Self::Output> {
        named!(ymd<&str, (i32, u32, u32)>,
            tuple!(
                terminated!(flat_map!(take!(4), parse_to!(i32)), char!('-')),
                terminated!(flat_map!(take!(2), parse_to!(u32)), char!('-')),
                flat_map!(take!(2), parse_to!(u32))
            )
        );

        map_opt!(input, complete!(ymd), |(y, m, d)| {
            NaiveDate::from_ymd_opt(y, m, d)
        })
    }
}

impl<'a> Parse<'a> for State {
    type Output = State;

    fn parse(input: &str) -> IResult<&str, Self::Output> {
        named!(complete<&str, (NaiveDate, NaiveDate)>,
            terminated!(
                separated_pair!(NaiveDate::parse, space, NaiveDate::parse),
                space
            )
        );

        named!(unknown<&str, (Option<Priority>, Option<NaiveDate>, Option<NaiveDate>)>,
            tuple!(
                opt!(terminated!(Priority::parse, space)),
                opt!(terminated!(NaiveDate::parse, space)),
                opt!(terminated!(NaiveDate::parse, space))
            )
        );

        switch!(input, opt!(terminated!(char!('x'), space)),
            Some(_) => map!(opt!(complete), State::Complete) |
            None => map!(unknown, |result| match result {
                (None, Some(completion_date), Some(creation_date)) => {
                    State::Complete(Some((completion_date, creation_date)))
                }
                (priority, creation_date, _) => {
                    State::Incomplete(priority, creation_date)
                }
            })
        )
    }
}

impl<'a> Task<'a> {
    /// Get the completion date of the task. If the task is incomplete, the
    /// completion date is guaranteed to be `Option::None`.
    pub fn completion_date(&self) -> Option<NaiveDate> {
        match self.state {
            State::Complete(state) => state.map(|(date, _)| date),
            State::Incomplete(_, _) => None,
        }
    }

    /// Get the creation date of the task.
    pub fn creation_date(&self) -> Option<NaiveDate> {
        match self.state {
            State::Complete(state) => state.map(|(date, _)| date),
            State::Incomplete(_, date) => date,
        }
    }

    /// Get a reference to the task's description.
    pub fn description(&self) -> &str {
        &self.text
    }

    /// Returns `true` if the task is complete, otherwise returns `false`.
    pub fn is_complete(&self) -> bool {
        match self.state {
            State::Complete(_) => true,
            State::Incomplete(_, _) => false,
        }
    }

    /// Get the priority of the task. If the task is complete, the priority
    /// is guaranteed to be `Option::None`.
    pub fn priority(&self) -> Option<Priority> {
        match self.state {
            State::Complete(_) => None,
            State::Incomplete(priority, _) => priority,
        }
    }

    /// This method is useful if you want to refine the data of a task to the
    /// distinct data of a complete or incomplete task.
    ///
    /// ## Example
    ///
    /// ```
    /// # extern crate todotxt;
    /// #
    /// # use todotxt::prelude::*;
    /// #
    /// # fn main() {
    /// # let data = "(A) Thank Mom for the meatballs @phone";
    /// # let task = data.tasks().next().unwrap();
    /// #
    /// use todotxt::State;
    ///
    /// match task.state() {
    ///     State::Complete(dates) => {
    ///         // Do something with the creation and/or completion date(s)...
    ///     }
    ///     State::Incomplete(priority, creation_date) => {
    ///         // Do something with the priority and/or creation date...
    ///     }
    /// }
    /// # }
    /// ```
    pub fn state(&self) -> State {
        self.state
    }

    /// Lazily parse and iterate over the tags contained within the description
    /// of the task.
    ///
    /// ## Example
    ///
    /// ```
    /// # extern crate todotxt;
    /// #
    /// # use todotxt::prelude::*;
    /// #
    /// # fn main() {
    /// # let data = "(A) Thank Mom for the meatballs @phone";
    /// # let task = data.tasks().next().unwrap();
    /// #
    /// for tag in task.tags() {
    ///     println!("{:#?}", tag);
    /// }
    /// # }
    /// ```
    pub fn tags(&self) -> Tags {
        let data = self.description();
        let iter = data.char_indices();

        Tags { data, iter }
    }
}

impl<'a> Clone for Task<'a> {
    fn clone(&self) -> Task<'static> {
        Task {
            state: self.state,
            text: Cow::Owned(String::from(&*self.text)),
        }
    }
}

impl<'a> Display for Task<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_complete() {
            f.write_str("x ")?;
        }

        if let Some(priority) = self.priority() {
            write!(f, "{} ", priority)?;
        }

        if let Some(completion_date) = self.completion_date() {
            write!(f, "{} ", completion_date)?;
        }

        if let Some(creation_date) = self.creation_date() {
            write!(f, "{} ", creation_date)?;
        }

        f.write_str(self.description())
    }
}

impl<'a> Parse<'a> for Task<'a> {
    type Output = Task<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Output> {
        map!(
            input,
            pair!(ws!(State::parse), map!(nom::rest, Cow::Borrowed)),
            |(state, text)| Task { state, text }
        )
    }
}

#[cfg(feature = "serde")]
impl<'a> Serialize for Task<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Task", 5)?;
        let tags: Vec<Tag> = self.tags().collect();

        if let Some(completion_date) = self.completion_date() {
            state.serialize_field("completion_date", &completion_date)?;
        }

        if let Some(creation_date) = self.creation_date() {
            state.serialize_field("creation_date", &creation_date)?;
        }

        state.serialize_field("description", self.description())?;

        if let Some(priority) = self.priority() {
            state.serialize_field("priority", &priority)?;
        }

        state.serialize_field("tags", &tags)?;

        if self.is_complete() {
            state.serialize_field("type", "COMPLETE")?;
        } else {
            state.serialize_field("type", "INCOMPLETE")?;
        }

        state.end()
    }
}
