use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! bbc {
	($e: expr) => {
		// criterion::black_box(
			$e
		// )
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

trait DynTrait {
	fn value(&self) -> u64;
}

#[derive(Clone, Copy)]
struct Foo {
	value: u64
}

#[derive(Clone, Copy)]
struct Bar {
	value: u64
}
impl DynTrait for Bar {
	fn value(&self) -> u64 {
		self.value
	}
}

#[derive(Clone, Copy)]
struct Qux {
	value: u64
}
impl DynTrait for Qux {
	fn value(&self) -> u64 {
		self.value
	}
}

enum DispatchEnum {
	Foo(Foo),
	Bar(Bar),
	Other(Box<dyn DynTrait>)
}
impl DispatchEnum {
	fn value(&self) -> u64 {
		match self {
			DispatchEnum::Foo(foo) => foo.value,
			DispatchEnum::Bar(bar) => bar.value(),
			DispatchEnum::Other(val) => val.value()
		}
	}
}

fn mixed_dispatch(c: &mut Criterion) {
    let value = bbc!(1u64);
	
	let (foo, enum_foo) = {
		let foo = Foo { value };
		let enum_foo = DispatchEnum::Foo(foo);
		
		(
			bbc!(foo),
			bbc!(enum_foo)
		)
	};

	let (bar, enum_bar) = {
		let bar = Bar { value };
		let enum_bar = DispatchEnum::Bar(bar);

		(
			bbc!(bar),
			bbc!(enum_bar)
		)
	};

	let (qux, qux_dyn, enum_qux) = {
		let qux = Box::new(
			Qux { value }
		);
		let qux_dyn = qux.clone() as Box<dyn DynTrait>;
		let qux_enum = DispatchEnum::Other(qux.clone());

		(
			bbc!(qux.clone()),
			bbc!(qux_dyn),
			bbc!(qux_enum)
		)
	};

	c.bench_function(
		"Enum::Foo",
		|b| b.iter(|| {
			value_op!(
				enum_foo.value()
			);
		})
	);

	c.bench_function(
		"Enum::Bar",
		|b| b.iter(|| {
			value_op!(
				enum_bar.value()
			);
		})
	);

	c.bench_function(
		"Enum::Other",
		|b| b.iter(|| {
			value_op!(
				enum_qux.value()
			);
		})
	);

	c.bench_function(
		"Foo",
		|b| b.iter(|| {
			value_op!(
				foo.value
			);
		})
	);
	c.bench_function(
		"Bar",
		|b| b.iter(|| {
			value_op!(
				bar.value()
			);
		})
	);
	c.bench_function(
		"Qux",
		|b| b.iter(|| {
			value_op!(
				qux.value()
			);
		})
	);
	c.bench_function(
		"dyn Qux",
		|b| b.iter(|| {
			value_op!(
				qux_dyn.value()
			);
		})
	);
}

criterion_group!(benches, mixed_dispatch);
criterion_main!(benches);