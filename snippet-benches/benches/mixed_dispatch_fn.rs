use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! bbc {
	($e: expr) => {
		criterion::black_box(
			$e
		)
	}
}

macro_rules! value_op {
	(
		$e: expr
	) => {
		if bbc!($e) == 5 {
			println!("Unexpected: {}", $e);
		}
	}
}

enum DispatchEnum {
	Static(fn(u32) -> u64),
	Dyn(Box<dyn FnMut(u32) -> u64>)
}
impl DispatchEnum {
	fn call(&mut self, arg: u32) -> u64 {
		match self {
			DispatchEnum::Static(s) => s(arg),
			DispatchEnum::Dyn(d) => d(arg)
		}
	}
}

fn test_fn(a: u32) -> u64 {
	match a {
		0 => 0,
		1 => 5,
		x => x as u64 + 256
	}
}
fn mixed_dispatch_fn(c: &mut Criterion) {
    let (fn_static, mut enum_static) = {
		(
			bbc!(test_fn),
			bbc!(
				DispatchEnum::Static(
					bbc!(
						test_fn
					)
				)
			)
		)
	};

	let (mut fn_dyn, mut enum_dyn) = {
		(
			bbc!(
				Box::new(
					test_fn
				) as Box<dyn FnMut(u32) -> u64>
			),
			bbc!(
				DispatchEnum::Dyn(
					bbc!(
						Box::new(
							test_fn
						) as Box<dyn FnMut(u32) -> u64>
					)
				)
			)
		)
	};

	c.bench_function(
		"Enum::Static",
		|b| b.iter(|| {
			value_op!(
				enum_static.call(
					bbc!(0)
				)
			);
		})
	);

	c.bench_function(
		"Enum::Dyn",
		|b| b.iter(|| {
			value_op!(
				enum_dyn.call(
					bbc!(0)
				)
			);
		})
	);

	c.bench_function(
		"Static",
		|b| b.iter(|| {
			value_op!(
				fn_static(
					bbc!(0)
				)
			);
		})
	);

	c.bench_function(
		"Dyn",
		|b| b.iter(|| {
			value_op!(
				fn_dyn(
					bbc!(0)
				)
			);
		})
	);
}

criterion_group!(benches, mixed_dispatch_fn);
criterion_main!(benches);