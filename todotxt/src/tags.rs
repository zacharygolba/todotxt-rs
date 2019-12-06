use std::{iter::FusedIterator, ops::Index, str::CharIndices};

/// The various tags that can appear within the description of a task.
///
/// For the sake of flexible [copy semantics], each tag variant contains a
/// start index as well as an end index rather than the text contents of the
/// tag. The indices are relative to the parsed description of the task in
/// which the tag belongs to.
///
/// As a convenience, [`Index<Tag>`] is implemented for [`str`]. That means
/// that you can get the value of a tag by using the tag to index into the
/// task's description.
///
/// ```
/// # extern crate todotxt;
/// #
/// # use todotxt::prelude::*;
/// #
/// # fn main() {
/// # let data = "(A) Thank Mom for the meatballs @phone";
/// # let task = data.tasks().next().unwrap();
/// let description = task.description();
///
/// for tag in task.tags() {
///     let value = &description[tag];
///
///     println!("tag: {:?}", tag);
///     println!("value: {}", value);
/// }
/// # }
/// ```
///
/// [`Index<Tag>`]: https://doc.rust-lang.org/std/ops/trait.Index.html
/// [`str`]: https://doc.rust-lang.org/std/primitive.str.html
/// [copy semantics]: https://doc.rust-lang.org/std/marker/trait.Copy.html
#[allow(missing_docs)]
#[cfg_attr(
    feature = "serde",
    serde(content = "location", rename_all = "UPPERCASE", tag = "type")
)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tag {
    Context { start: usize, end: usize },
    Project { start: usize, end: usize },
    Special { start: usize, end: usize },
}

/// An iterator over the tags of a given task.
///
/// When you parse a task, you may not actually need to look at it's tags.
/// Rather than making the assumption that you need access to the tags of
/// a task, tags are parsed lazily. This data structure contains an iterator
/// that parses the next tag in a given task's description everytime the `next`
/// method is called.
///
/// ## Example
///
/// This example parses the tags of a task until it finds a matching value:
///
/// ```
/// # extern crate todotxt;
/// #
/// # use todotxt::prelude::*;
/// #
/// #
/// # fn main() {
/// fn has_project(task: &Task, project: &str) -> bool {
///     let description = task.description();
///
///     task.tags().any(|tag| match tag {
///         Tag::Context { .. } | Tag::Special { .. } => false,
///         Tag::Project { .. } => project == &description[tag],
///     })
/// }
///
/// let data = "x write a +todo.txt parser in @rust";
/// let task = data.tasks().next().expect("data does not contain any tasks");
///
/// assert_eq!(has_project(&task, "+todo.txt"), true);
/// # }
/// ```
///
#[derive(Clone, Debug)]
pub struct Tags<'a> {
    pub(super) data: &'a str,
    pub(super) iter: CharIndices<'a>,
}

impl Tag {
    /// Returns the end index of the tag, relative to the task's description.
    ///
    /// ```
    /// # extern crate todotxt;
    /// #
    /// # use todotxt::prelude::*;
    /// #
    /// # fn main() {
    /// let tag = Tag::Context {
    ///     start: 5,
    ///     end: 10,
    /// };
    ///
    /// assert_eq!(tag.end(), 10);
    /// # }
    /// ```
    pub fn end(&self) -> usize {
        match *self {
            Tag::Context { end, .. } | Tag::Project { end, .. } | Tag::Special { end, .. } => end,
        }
    }

    /// Returns the start index of the tag, relative to the task's description.
    ///
    /// ```
    /// # extern crate todotxt;
    /// #
    /// # use todotxt::prelude::*;
    /// #
    /// # fn main() {
    /// let tag = Tag::Context {
    ///     start: 5,
    ///     end: 10,
    /// };
    ///
    /// assert_eq!(tag.start(), 5);
    /// # }
    /// ```
    pub fn start(&self) -> usize {
        match *self {
            Tag::Context { start, .. }
            | Tag::Project { start, .. }
            | Tag::Special { start, .. } => start,
        }
    }
}

impl Index<Tag> for str {
    type Output = str;

    fn index(&self, tag: Tag) -> &Self::Output {
        &self[tag.start()..tag.end()]
    }
}

impl<'a> FusedIterator for Tags<'a> {}

impl<'a> Iterator for Tags<'a> {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, end) = next_word_boundary(&mut self.iter)?;
        let word = &self.data[start..end];

        if word.starts_with('@') {
            Some(Tag::Context { start, end })
        } else if word.starts_with('+') {
            Some(Tag::Project { start, end })
        } else if word.contains(':') {
            Some(Tag::Special { start, end })
        } else {
            self.next()
        }
    }
}

fn is_not_whitespace((_, item): &(usize, char)) -> bool {
    !item.is_whitespace()
}

fn is_whitespace((_, item): &(usize, char)) -> bool {
    item.is_whitespace()
}

fn next_word_boundary(iter: &mut CharIndices) -> Option<(usize, usize)> {
    let mut iter = iter.skip_while(is_whitespace).take_while(is_not_whitespace);
    let start = iter.next().map(|(index, _)| index)?;
    let end = iter.last().map_or(start, |(index, _)| index + 1);

    Some((start, end))
}
