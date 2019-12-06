//! A zero-copy [todo.txt] parser implementation.
//!
//! TODO: Write something nice about the following features
//!
//! - Zero-copy, zero-runtime allocations by default
//! - Ability to easily own data if necessary
//! - Easy integration with other languages
//! - Dead-simple, optional parallelism
//! - Easy and optional serialization
//! - Infallible at runtime
//! - Very fast
//!
//! ## Usage
//!
//! ```
//! extern crate todotxt;
//!
//! use todotxt::prelude::*;
//!
//! fn main() {
//!     let data = "
//!         (A) Thank Mom for the meatballs @phone
//!         (B) Schedule Goodwill pickup +GarageSale @phone
//!         Post signs around the neighborhood +GarageSale
//!         @GroceryStore Eskimo pies
//!     ";
//!
//!     for task in data.tasks() {
//!         println!("{:#?}", task);
//!     }
//! }
//! ```
//!
//! [todo.txt]: http://todotxt.org/

#![deny(missing_docs)]

#[macro_use]
extern crate nom;

mod priority;
mod tags;
mod task;

pub mod parser;

#[cfg(not(feature = "rayon"))]
pub mod prelude {
    //! A "batteries-included" module that re-exports frequently used types.
    //!
    //! ## Example
    //!
    //! ```
    //! use todotxt::prelude::*;
    //! ```

    pub use crate::{parser::Input, priority::Priority, tags::Tag, task::Task};
}

#[cfg(feature = "rayon")]
pub mod prelude {
    //! A "batteries-included" module that re-exports frequently used types.
    //!
    //! ## Example
    //!
    //! ```
    //! use todotxt::prelude::*;
    //! ```

    pub use crate::{
        parser::{Input, ParallelInput},
        priority::Priority,
        tags::Tag,
        task::Task,
    };
    pub use rayon::iter::ParallelIterator;
}

pub use crate::{
    priority::Priority,
    tags::{Tag, Tags},
    task::{State, Task},
};
pub use chrono;
