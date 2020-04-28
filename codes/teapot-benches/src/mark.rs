use std::time::{Duration, Instant};

pub const MAX_LOOPS: usize = 100_000;

static mut MARK_BUFFER: Vec<LoopMarkState> = Vec::new();
const MARK_BUFFER_FLUSH_LIMIT: usize = 1000;

pub fn init_static() {
	unsafe { MARK_BUFFER.reserve(MARK_BUFFER_FLUSH_LIMIT) }
}

fn flush() {
	unsafe {
		if !MARK_BUFFER.is_empty() {
			println!("MARK_BUFFER: {:?}", MARK_BUFFER);
			MARK_BUFFER.set_len(0);
		}
	}
}
pub fn cleanup_static() {
	flush()
}

pub struct LoopMarkState {
	pub start: Instant,
	pub acquire: Option<Instant>,
	pub uniform: Option<Instant>,
	pub command: Option<Instant>,
	pub submit: Option<Instant>,
	pub present: Option<Instant>,
	pub wait: Option<Instant>,
	pub update: Option<Instant>,
	pub finish: Option<Instant>
}
impl LoopMarkState {
	pub fn start() -> Self {
		LoopMarkState {
			start: Instant::now(),
			acquire: None,
			uniform: None,
			command: None,
			submit: None,
			present: None,
			wait: None,
			update: None,
			finish: None
		}
	}

	pub fn before_acquire(&mut self) {
		self.acquire = Some(Instant::now());
	}

	pub fn before_uniform(&mut self) {
		self.uniform = Some(Instant::now());
	}

	pub fn before_command(&mut self) {
		self.command = Some(Instant::now());
	}

	pub fn before_submit(&mut self) {
		self.submit = Some(Instant::now());
	}

	pub fn before_present(&mut self) {
		self.present = Some(Instant::now());
	}

	pub fn before_wait(&mut self) {
		self.wait = Some(Instant::now());
	}

	pub fn before_update(&mut self) {
		self.update = Some(Instant::now());
	}

	pub fn finish(mut self) {
		self.finish = Some(Instant::now());

		unsafe {
			MARK_BUFFER.push(self);
			if MARK_BUFFER.len() >= MARK_BUFFER_FLUSH_LIMIT {
				flush()
			}
		}
	}
}
impl std::fmt::Debug for LoopMarkState {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let start = self.start;
		let acquire = self.acquire.unwrap();
		let uniform = self.uniform.unwrap();
		let command = self.command.unwrap();
		let submit = self.submit.unwrap();
		let present = self.present.unwrap();
		let wait = self.wait.unwrap();
		let update = self.update.unwrap();
		let finish = self.finish.unwrap();

		write!(
			f,
			"{{ preinit = {}, acquire = {}, uniform = {}, command = {}, submit = {}, present = {}, wait = {}, update = {}, total = {} }}",
			acquire.duration_since(start).as_nanos(),
			uniform.duration_since(acquire).as_nanos(),
			command.duration_since(uniform).as_nanos(),
			submit.duration_since(command).as_nanos(),
			present.duration_since(submit).as_nanos(),
			wait.duration_since(present).as_nanos(),
			update.duration_since(wait).as_nanos(),
			finish.duration_since(update).as_nanos(),

			finish.duration_since(start).as_nanos()
		)
	}
}

pub fn output_summary(
	before_preinit: Instant,
	before_init: Instant,
	before_upload: Instant,
	before_loop: Instant,
	loop_iterations: usize,
	before_cleanup: Instant,
	before_summary: Instant
) {
	struct PrettyHeader(&'static str);
	impl std::fmt::Display for PrettyHeader {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			write!(f, "{: >8}\t", self.0)
		}
	}

	struct PrettyDuration(Duration);
	impl std::fmt::Display for PrettyDuration {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			write!(
				f,
				"{:0>2}.{:0>3}_{:0>3}_{:0>3} sec",
				self.0.as_secs(),
				self.0.subsec_millis(),
				self.0.subsec_micros() % 1000,
				self.0.subsec_nanos() % 1000
			)
		}
	}

	println!();

	println!(
		"{}{}",
		PrettyHeader("preinit"),
		PrettyDuration(before_init.duration_since(before_preinit))
	);
	println!(
		"{}{}",
		PrettyHeader("init"),
		PrettyDuration(before_upload.duration_since(before_init))
	);
	println!(
		"{}{}",
		PrettyHeader("upload"),
		PrettyDuration(before_loop.duration_since(before_upload))
	);
	println!(
		"{}{}\t({:_>6} iterations/{} per iteration)",
		PrettyHeader("loop"),
		PrettyDuration(before_cleanup.duration_since(before_loop)),
		loop_iterations,
		PrettyDuration(before_cleanup.duration_since(before_loop) / loop_iterations as u32)
	);
	println!(
		"{}{}",
		PrettyHeader("cleanup"),
		PrettyDuration(before_summary.duration_since(before_cleanup))
	);

	println!();
	println!(
		"{}{}",
		PrettyHeader("total"),
		PrettyDuration(before_summary.duration_since(before_preinit))
	);

	println!();
}
