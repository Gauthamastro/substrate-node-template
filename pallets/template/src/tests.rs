use crate::{Error, mock::*, Trait};
use frame_support::{assert_ok, assert_noop};
use sp_arithmetic::{FixedU128, FixedPointNumber};
use sp_arithmetic::traits::{CheckedDiv, UniqueSaturatedFrom};

#[test]
fn balance_FixedU128_integer_conversion_works(){
	let value = FixedU128::from(2);
	println!("Value for Testing: {}",value);
	let balance_converted = TemplateModule::convert_fixed_u128_to_balance(value);
	assert!(balance_converted.is_some());
	println!("Value in Substrate Balance type (u128): {}",balance_converted.unwrap()); // This is 10^12
	let balance: Option<FixedU128> = TemplateModule::convert_balance_to_fixed_u128(balance_converted.unwrap());
	assert!(balance.is_some());
	println!("Value after converting back to FixedU128 type: {}",balance.unwrap());
	assert_eq!(balance,Some(value));
}

#[test]
fn balance_fixed_u128_float_conversion_works(){
	let value = FixedU128::from_fraction(2.3456789);
	println!("Value for Testing: {}",value);
	let balance_converted = TemplateModule::convert_fixed_u128_to_balance(value);
	assert!(balance_converted.is_some());
	println!("Value in Substrate Balance type (u128): {}",balance_converted.unwrap()); // This is 10^12
	let balance: Option<FixedU128> = TemplateModule::convert_balance_to_fixed_u128(balance_converted.unwrap());
	assert!(balance.is_some());
	println!("Value after converting back to FixedU128 type: {}",balance.unwrap());
	assert_eq!(balance,Some(value));
}

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		// assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// // Read pallet storage and assert an expected result.
		// assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// // Ensure the expected error is thrown when no value is present.
		// assert_noop!(
		// 	TemplateModule::cause_error(Origin::signed(1)),
		// 	Error::<Test>::NoneValue
		// );
	});
}

#[cfg(test)]
mod from_liballoc {
	// The following tests copyed from liballoc/tests/binary_heap.rs
	use crate::binary_heap::*;
	use sp_std::vec;
	use sp_std::vec::Vec;
	// use std::panic;
	// use std::collections::BinaryHeap;
	// use std::collections::binary_heap::{Drain, PeekMut};

	#[test]
	fn test_iterator() {
		let data = vec![5, 9, 3];
		let iterout = [9, 5, 3];
		let heap = BinaryHeap::from(data);
		let mut i = 0;
		for el in &heap {
			assert_eq!(*el, iterout[i]);
			i += 1;
		}
	}

	#[test]
	fn test_iterator_reverse() {
		let data = vec![5, 9, 3];
		let iterout = vec![3, 5, 9];
		let pq = BinaryHeap::from(data);

		let v: Vec<_> = pq.iter().rev().cloned().collect();
		assert_eq!(v, iterout);
	}

	#[test]
	fn test_move_iter() {
		let data = vec![5, 9, 3];
		let iterout = vec![9, 5, 3];
		let pq = BinaryHeap::from(data);

		let v: Vec<_> = pq.into_iter().collect();
		assert_eq!(v, iterout);
	}

	#[test]
	fn test_move_iter_size_hint() {
		let data = vec![5, 9];
		let pq = BinaryHeap::from(data);

		let mut it = pq.into_iter();

		assert_eq!(it.size_hint(), (2, Some(2)));
		assert_eq!(it.next(), Some(9));

		assert_eq!(it.size_hint(), (1, Some(1)));
		assert_eq!(it.next(), Some(5));

		assert_eq!(it.size_hint(), (0, Some(0)));
		assert_eq!(it.next(), None);
	}

	#[test]
	fn test_move_iter_reverse() {
		let data = vec![5, 9, 3];
		let iterout = vec![3, 5, 9];
		let pq = BinaryHeap::from(data);

		let v: Vec<_> = pq.into_iter().rev().collect();
		assert_eq!(v, iterout);
	}

	#[test]
	fn test_into_iter_sorted_collect() {
		let heap = BinaryHeap::from(vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1]);
		let it = heap.into_iter_sorted();
		let sorted = it.collect::<Vec<_>>();
		assert_eq!(sorted, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 2, 1, 1, 0]);
	}

	#[test]
	fn test_peek_and_pop() {
		let data = vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1];
		let mut sorted = data.clone();
		sorted.sort();
		let mut heap = BinaryHeap::from(data);
		while !heap.is_empty() {
			assert_eq!(heap.peek().unwrap(), sorted.last().unwrap());
			assert_eq!(heap.pop().unwrap(), sorted.pop().unwrap());
		}
	}

	#[test]
	fn test_peek_mut() {
		let data = vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1];
		let mut heap = BinaryHeap::from(data);
		assert_eq!(heap.peek(), Some(&10));
		{
			let mut top = heap.peek_mut().unwrap();
			*top -= 2;
		}
		assert_eq!(heap.peek(), Some(&9));
	}

	#[test]
	fn test_peek_mut_pop() {
		let data = vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1];
		let mut heap = BinaryHeap::from(data);
		assert_eq!(heap.peek(), Some(&10));
		{
			let mut top = heap.peek_mut().unwrap();
			*top -= 2;
			assert_eq!(PeekMut::pop(top), 8);
		}
		assert_eq!(heap.peek(), Some(&9));
	}

	#[test]
	fn test_push() {
		let mut heap = BinaryHeap::from(vec![2, 4, 9]);
		assert_eq!(heap.len(), 3);
		assert!(*heap.peek().unwrap() == 9);
		heap.push(11);
		assert_eq!(heap.len(), 4);
		assert!(*heap.peek().unwrap() == 11);
		heap.push(5);
		assert_eq!(heap.len(), 5);
		assert!(*heap.peek().unwrap() == 11);
		heap.push(27);
		assert_eq!(heap.len(), 6);
		assert!(*heap.peek().unwrap() == 27);
		heap.push(3);
		assert_eq!(heap.len(), 7);
		assert!(*heap.peek().unwrap() == 27);
		heap.push(103);
		assert_eq!(heap.len(), 8);
		assert!(*heap.peek().unwrap() == 103);
	}

	// #[test]
	// fn test_push_unique() {
	//     let mut heap = BinaryHeap::<Box<_>>::from(vec![box 2, box 4, box 9]);
	//     assert_eq!(heap.len(), 3);
	//     assert!(**heap.peek().unwrap() == 9);
	//     heap.push(box 11);
	//     assert_eq!(heap.len(), 4);
	//     assert!(**heap.peek().unwrap() == 11);
	//     heap.push(box 5);
	//     assert_eq!(heap.len(), 5);
	//     assert!(**heap.peek().unwrap() == 11);
	//     heap.push(box 27);
	//     assert_eq!(heap.len(), 6);
	//     assert!(**heap.peek().unwrap() == 27);
	//     heap.push(box 3);
	//     assert_eq!(heap.len(), 7);
	//     assert!(**heap.peek().unwrap() == 27);
	//     heap.push(box 103);
	//     assert_eq!(heap.len(), 8);
	//     assert!(**heap.peek().unwrap() == 103);
	// }

	fn check_to_vec(mut data: Vec<i32>) {
		let heap = BinaryHeap::from(data.clone());
		let mut v = heap.clone().into_vec();
		v.sort();
		data.sort();

		assert_eq!(v, data);
		assert_eq!(heap.into_sorted_vec(), data);
	}

	#[test]
	fn test_to_vec() {
		check_to_vec(vec![]);
		check_to_vec(vec![5]);
		check_to_vec(vec![3, 2]);
		check_to_vec(vec![2, 3]);
		check_to_vec(vec![5, 1, 2]);
		check_to_vec(vec![1, 100, 2, 3]);
		check_to_vec(vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 0]);
		check_to_vec(vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1]);
		check_to_vec(vec![9, 11, 9, 9, 9, 9, 11, 2, 3, 4, 11, 9, 0, 0, 0, 0]);
		check_to_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
		check_to_vec(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
		check_to_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 1, 2]);
		check_to_vec(vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 1, 5, 4, 3, 2, 1]);
	}

	#[test]
	fn test_empty_pop() {
		let mut heap = BinaryHeap::<i32>::new();
		assert!(heap.pop().is_none());
	}

	#[test]
	fn test_empty_peek() {
		let empty = BinaryHeap::<i32>::new();
		assert!(empty.peek().is_none());
	}

	#[test]
	fn test_empty_peek_mut() {
		let mut empty = BinaryHeap::<i32>::new();
		assert!(empty.peek_mut().is_none());
	}

	#[test]
	fn test_from_iter() {
		let xs = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];

		let mut q: BinaryHeap<_> = xs.iter().rev().cloned().collect();

		for &x in &xs {
			assert_eq!(q.pop().unwrap(), x);
		}
	}

	#[test]
	fn test_drain() {
		let mut q: BinaryHeap<_> = [9, 8, 7, 6, 5, 4, 3, 2, 1].iter().cloned().collect();

		assert_eq!(q.drain().take(5).count(), 5);

		assert!(q.is_empty());
	}

	#[test]
	fn test_extend_ref() {
		let mut a = BinaryHeap::new();
		a.push(1);
		a.push(2);

		a.extend(&[3, 4, 5]);

		assert_eq!(a.len(), 5);
		assert_eq!(a.into_sorted_vec(), [1, 2, 3, 4, 5]);

		let mut a = BinaryHeap::new();
		a.push(1);
		a.push(2);
		let mut b = BinaryHeap::new();
		b.push(3);
		b.push(4);
		b.push(5);

		a.extend(&b);

		assert_eq!(a.len(), 5);
		assert_eq!(a.into_sorted_vec(), [1, 2, 3, 4, 5]);
	}

	#[test]
	fn test_append() {
		let mut a = BinaryHeap::from(vec![-10, 1, 2, 3, 3]);
		let mut b = BinaryHeap::from(vec![-20, 5, 43]);

		a.append(&mut b);

		assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
		assert!(b.is_empty());
	}

	#[test]
	fn test_append_to_empty() {
		let mut a = BinaryHeap::new();
		let mut b = BinaryHeap::from(vec![-20, 5, 43]);

		a.append(&mut b);

		assert_eq!(a.into_sorted_vec(), [-20, 5, 43]);
		assert!(b.is_empty());
	}

	#[test]
	fn test_extend_specialization() {
		let mut a = BinaryHeap::from(vec![-10, 1, 2, 3, 3]);
		let b = BinaryHeap::from(vec![-20, 5, 43]);

		a.extend(b);

		assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
	}

	// #[test]
	// fn test_placement() {
	//     let mut a = BinaryHeap::new();
	//     &mut a <- 2;
	//     &mut a <- 4;
	//     &mut a <- 3;
	//     assert_eq!(a.peek(), Some(&4));
	//     assert_eq!(a.len(), 3);
	//     &mut a <- 1;
	//     assert_eq!(a.into_sorted_vec(), vec![1, 2, 3, 4]);
	// }

	// #[test]
	// fn test_placement_panic() {
	//     let mut heap = BinaryHeap::from(vec![1, 2, 3]);
	//     fn mkpanic() -> usize {
	//         panic!()
	//     }
	//     let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| {
	//         &mut heap <- mkpanic();
	//     }));
	//     assert_eq!(heap.len(), 3);
	// }

	#[allow(dead_code)]
	fn assert_covariance() {
		fn drain<'new>(d: Drain<'static, &'static str>) -> Drain<'new, &'new str> {
			d
		}
	}
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests_serde {
	use crate::binary_heap::*;
	use serde_json;

	#[test]
	fn deserialized_same_small_vec() {
		let heap = BinaryHeap::from(vec![1, 2, 3]);
		let serialized = serde_json::to_string(&heap).unwrap();
		let deserialized: BinaryHeap<i32> = serde_json::from_str(&serialized).unwrap();

		let v0: Vec<_> = heap.into_iter().collect();
		let v1: Vec<_> = deserialized.into_iter().collect();
		assert_eq!(v0, v1);
	}
	#[test]
	fn deserialized_same() {
		let vec: Vec<i32> = (0..1000).collect();
		let heap = BinaryHeap::from(vec);
		let serialized = serde_json::to_string(&heap).unwrap();
		let deserialized: BinaryHeap<i32> = serde_json::from_str(&serialized).unwrap();

		let v0: Vec<_> = heap.into_iter().collect();
		let v1: Vec<_> = deserialized.into_iter().collect();
		assert_eq!(v0, v1);
	}
}