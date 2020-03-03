use std::ops::Drop;

#[derive(Clone, Copy)]
struct Dummy;
impl Dummy {
	pub unsafe fn destroy_surface(&self, _other: Dummy) {
		println!("Destroying dummy");
	}
}
type Loader = Dummy;
type SurfaceKHR = Dummy;

#[derive(Debug)]
struct Window {
	a: i32
}
impl Window {
	pub fn new(a: i32) -> Self {
		Window { a }
	}

	// There is no move constructor, all types are moveable
}
impl Drop for Window {
	fn drop(&mut self) {
		println!("Destroying window {}", self.a);
	}
}

struct Surface<W: std::fmt::Debug> {
	window: Option<W>,
	loader: Loader,
	surface: SurfaceKHR
}
impl<W: std::fmt::Debug> Surface<W> {
	pub fn new(window: W) -> Self {
		Surface {
			window: Some(window),
			loader: Dummy,
			surface: Dummy
		}
	}

	pub fn destroy_without_window(mut self) -> W {
		self.window.take().unwrap()
	}
}
impl<W: std::fmt::Debug> Drop for Surface<W> {
	fn drop(&mut self) {
		println!("Destroying surface with window {:?}", self.window);

		unsafe {
			self.loader.destroy_surface(self.surface);
		}
	}
}

fn main() {
	println!(">> Moving out");
	{
		let original = Window::new(1);
		println!("original {:?}\n", original);

		let surface = Surface::new(original);
		// println!("original {:?}", original); // Compiler error, original was moved
		println!("in surface {:?}\n", surface.window);

		let moved = surface.destroy_without_window();
		// println!("original {:?}", original); // Compiler error, original was moved
		// println!("in surface {:?}", surface.window); // Compiler error, surface was moved
		println!("moved out {:?}\n", moved);
	}

	println!("\n>>Not moving out");
	{
		let original = Window::new(2);
		println!("original {:?}\n", original);

		let surface = Surface::new(original);
		println!("in surface {:?}\n", surface.window);
	}
}