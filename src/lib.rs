//! Proc macro for generating unique static indexes in a range `0..N`. Ordering
//! of indexes is not guaranteed.
//!
//! [![Build Status](https://api.travis-ci.org/maksimsco/static_index.svg?branch=master.svg?branch=master)](https://travis-ci.org/maksimsco/static_index)
//!
//! ## Example
//!
//! ```rust
//! use static_index::{codegen::static_index, codegen::StaticIndex, Indexable};
//!
//! struct A;
//! struct B;
//! struct C;
//!
//! #[derive(StaticIndex)]
//! struct D;
//!
//! struct E;
//!
//! static_index!(A, B, C);
//!
//! static_index!(E);
//!
//! fn main() {
//!     assert_eq!(1, <A as Indexable>::INDEX);
//!     assert_eq!(2, <B as Indexable>::INDEX);
//!     assert_eq!(3, <C as Indexable>::INDEX);
//!     assert_eq!(0, <D as Indexable>::INDEX);
//!     assert_eq!(4, <E as Indexable>::INDEX);
//! }
//! ```
pub use codegen;

#[doc(hidden)]
pub use codegen_const::GLOBAL_INDEX;

/// Trait that exposes `INDEX` constant.
///
/// ## Example
/// ```
/// use static_index::Indexable;
///
/// struct A;
///
/// impl Indexable for A {
///     const INDEX: usize = 3;
/// }
///
/// assert_eq!(3, <A as Indexable>::INDEX);
/// ```
pub trait Indexable {
    const INDEX: usize;
}
