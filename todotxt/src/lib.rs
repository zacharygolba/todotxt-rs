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
#[cfg(feature = "rayon")]
extern crate rayon;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

pub extern crate chrono;

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

    pub use parser::Input;
    pub use priority::Priority;
    pub use tags::Tag;
    pub use task::Task;
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

    pub use rayon::iter::ParallelIterator;

    pub use parser::{Input, ParallelInput};
    pub use priority::Priority;
    pub use tags::Tag;
    pub use task::Task;
}

pub use priority::Priority;
pub use tags::{Tag, Tags};
pub use task::{State, Task};
