//! This crate provides `BinaryHeap` which is backward-compatible with `std::collections::BinaryHeap`.
//!
//! Added features include:
//! * Heaps other than max heap.
//! * Optional `serde` feature.
//!
//! # Quick start
//!
//! ## Max/Min Heap
//!
//! For max heap, `BinaryHeap::from_vec()` is the most versatile way to create a heap.
//!
//! ```rust
//!     use binary_heap_plus::*;
//!
//!     // max heap
//!     let mut h: BinaryHeap<i32> = BinaryHeap::from_vec(vec![]);
//!     // max heap with initial capacity
//!     let mut h: BinaryHeap<i32> = BinaryHeap::from_vec(Vec::with_capacity(16));
//!     // max heap from iterator
//!     let mut h: BinaryHeap<i32> = BinaryHeap::from_vec((0..42).collect());
//!     assert_eq!(h.pop(), Some(41));
//! ```
//!
//! Min heap is similar, but requires type annotation.
//!
//! ```rust
//!     use binary_heap_plus::*;
//!
//!     // min heap
//!     let mut h: BinaryHeap<i32, MinComparator> = BinaryHeap::from_vec(vec![]);
//!     // min heap with initial capacity
//!     let mut h: BinaryHeap<i32, MinComparator> = BinaryHeap::from_vec(Vec::with_capacity(16));
//!     // min heap from iterator
//!     let mut h: BinaryHeap<i32, MinComparator> = BinaryHeap::from_vec((0..42).collect());
//!     assert_eq!(h.pop(), Some(0));
//! ```
//!
//! ## Custom Heap
//!
//! For custom heap, `BinaryHeap::from_vec_cmp()` works in a similar way to max/min heap. The only difference is that you add the comparator closure with apropriate signature.
//!
//! ```rust
//!     use binary_heap_plus::*;
//!
//!     // custom heap: ordered by second value (_.1) of the tuples; min first
//!     let mut h = BinaryHeap::from_vec_cmp(
//!         vec![(1, 5), (3, 2), (2, 3)],
//!         |a: &(i32, i32), b: &(i32, i32)| b.1.cmp(&a.1), // comparator closure here
//!     );
//!     assert_eq!(h.pop(), Some((3, 2)));
//! ```
//!
//! # Constructers
//!
//! ## Generic methods to create different kind of heaps from initial `vec` data.
//!
//! * `BinaryHeap::from_vec(vec)`
//! * `BinaryHeap::from_vec_cmp(vec, cmp)`
//!
//! ```
// use binary_heap_plus::*;
//
// // max heap (default)
// let mut heap: BinaryHeap<i32> = BinaryHeap::from_vec(vec![1,5,3]);
// assert_eq!(heap.pop(), Some(5));
//
// // min heap
// let mut heap: BinaryHeap<i32, MinComparator> = BinaryHeap::from_vec(vec![1,5,3]);
// assert_eq!(heap.pop(), Some(1));
//
// // custom-sort heap
// let mut heap = BinaryHeap::from_vec_cmp(vec![1,5,3], |a: &i32, b: &i32| b.cmp(a));
// assert_eq!(heap.pop(), Some(1));
//
// // custom-key heap
// let mut heap = BinaryHeap::from_vec_cmp(vec![6,3,1], KeyComparator(|k: &i32| k % 4));
// assert_eq!(heap.pop(), Some(3));
//
// // TIP: How to reuse a comparator
// let mod4_comparator = KeyComparator(|k: &_| k % 4);
// let mut heap1 = BinaryHeap::from_vec_cmp(vec![6,3,1], mod4_comparator);
// assert_eq!(heap1.pop(), Some(3));
// let mut heap2 = BinaryHeap::from_vec_cmp(vec![2,4,1], mod4_comparator);
// assert_eq!(heap2.pop(), Some(2));
//! ```
//!
//! ## Dedicated methods to create different kind of heaps
//!
//! * `BinaryHeap::new()` creates a max heap.
//! * `BinaryHeap::new_min()` creates a min heap.
//! * `BinaryHeap::new_by()` creates a heap sorted by the given closure.
//! * `BinaryHeap::new_by_key()` creates a heap sorted by the key generated by the given closure.
//!
#![no_std]
mod binary_heap;
pub use crate::binary_heap::*;

/// An intermediate trait for specialization of `Extend`.
// #[doc(hidden)]
// trait SpecExtend<I: IntoIterator> {
//     /// Extends `self` with the contents of the given iterator.
//     fn spec_extend(&mut self, iter: I);
// }


