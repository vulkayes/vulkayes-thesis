use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, BatchSize};

use smallvec::SmallVec;

use reusable_memory::ReusableMemory;

use bumpalo::{Bump, collections::Vec as BumpVec};

macro_rules! bbc {
	($e: expr) => {
		// criterion::black_box(
			$e
		// )
	}
}

type ItemType = usize;
const SHORT_LEN: usize = 3;
const MEDIUM_LEN: usize = 30;
const LONG_LEN: usize = 300;

fn empty_iterator() -> impl Iterator<Item = ItemType> {
	bbc!(
		std::iter::empty()
	)
}
fn short_iterator() -> impl Iterator<Item = ItemType> {
	bbc!(
		(0 .. SHORT_LEN).into_iter()
	)
}
fn medium_iterator() -> impl Iterator<Item = ItemType> {
	bbc!(
		(0 .. MEDIUM_LEN).into_iter()
	)
}
fn long_iterator() -> impl Iterator<Item = ItemType> {
	bbc!(
		(0 .. LONG_LEN).into_iter()
	)
}

fn collecting_iterators(crit: &mut Criterion) {
	macro_rules! bench_strategy_fn {
		(
			$strategy_name: expr, $input: tt, $strategy: expr,
			$group: expr, $name: literal, $iter: expr, $len: expr
		) => {
			$group.bench_function(
				concat!($strategy_name, "_", $name),
				|b| b.iter_batched_ref(
					|| $input,
					|input| {
						$strategy(
							input, $iter, |collected| {
								assert_eq!(collected.len(), $len);
							}
						);
					},
					BatchSize::SmallInput
				)
			);
		}
	}
	
	// let mut empty_group = c.benchmark_group("empty iter");
	// let mut short_group = c.benchmark_group("short iter");
	// let mut medium_group = c.benchmark_group("medium iter");
	// let mut long_group = c.benchmark_group("long iter");
	macro_rules! bench_strategy {
		(
			$strategy_name: expr, $input: tt, $strategy: expr
		) => {
			{
				let mut group = crit.benchmark_group($strategy_name);
				bench_strategy_fn!(
					$strategy_name, $input, $strategy,
					group, "empty", empty_iterator(), 0
				);
				bench_strategy_fn!(
					$strategy_name, $input, $strategy,
					group, "short", short_iterator(), SHORT_LEN
				);
				bench_strategy_fn!(
					$strategy_name, $input, $strategy,
					group, "medium", medium_iterator(), MEDIUM_LEN
				);
				bench_strategy_fn!(
					$strategy_name, $input, $strategy,
					group, "long", long_iterator(), LONG_LEN
				);
			}
		}
	}
	
	// Global alloc into a Vec
	fn global_alloc_vec(_: &mut (), iter: impl Iterator<Item = ItemType>, call_me: impl FnOnce(Vec<ItemType>)) {
		call_me(
			iter.collect::<Vec<_>>()
		)
	}
	bench_strategy!(
		"global_alloc", (), global_alloc_vec
	);

	// Smallvecs of varying sizes
	macro_rules! smallvec_strategy {
		(
			$name: ident, $count: literal
		) => {
			fn $name(_: &mut (), iter: impl Iterator<Item = ItemType>, call_me: impl FnOnce(SmallVec<[ItemType; $count]>)) {
				call_me(
					iter.collect::<SmallVec<[ItemType; $count]>>()
				)
			}
			bench_strategy!(
				stringify!($name), (), $name
			);
		}
	}
	smallvec_strategy!(smallvec_0, 0);
	smallvec_strategy!(smallvec_4, 4);
	smallvec_strategy!(smallvec_32, 32);
	smallvec_strategy!(smallvec_128, 128);

	// Reusable memory of varying sizes
	macro_rules! reusable_memory_strategy {
		(
			$name: ident, $count: literal
		) => {
			fn $name(memory: &mut ReusableMemory<usize>, iter: impl Iterator<Item = ItemType>, call_me: impl FnOnce(&[ItemType])) {
				let mut borrow = memory.borrow_mut_as(
					std::num::NonZeroUsize::new($count).unwrap()
				).unwrap();

				match borrow.push_from_iter(iter) {
					Ok(_) => call_me(borrow.as_slice()),
					Err(iter) => {
						let v = borrow.drain(..).chain(iter).collect::<Vec<_>>();
						call_me(v.as_slice());
					}
				}
			}
			bench_strategy!(
				stringify!($name), { ReusableMemory::<ItemType>::with_capacity($count).unwrap() }, $name
			);
		}
	}
	reusable_memory_strategy!(reusable_memory_1, 1);
	reusable_memory_strategy!(reusable_memory_4, 4);
	reusable_memory_strategy!(reusable_memory_32, 32);
	reusable_memory_strategy!(reusable_memory_128, 128);

	macro_rules! bumpalo_strategy {
		(
			$name: ident, $count: literal
		) => {
			fn $name(bump: &mut Bump, iter: impl Iterator<Item = ItemType>, call_me: impl FnOnce(BumpVec<ItemType>)) {
				call_me(
					BumpVec::from_iter_in(
						iter,
						bump
					)
				)
			}
			bench_strategy!(
				stringify!($name), { Bump::with_capacity($count * std::mem::size_of::<ItemType>()) }, $name
			);
		}
	}
	bumpalo_strategy!(bumpalo_0, 0);
	bumpalo_strategy!(bumpalo_4, 4);
	bumpalo_strategy!(bumpalo_32, 32);
	bumpalo_strategy!(bumpalo_128, 128);
}

criterion_group!(
	name = benches;
	config = Criterion::default().sample_size(200).warm_up_time(Duration::from_secs(10)).measurement_time(Duration::from_secs(10));
	targets = collecting_iterators
);
criterion_main!(benches);