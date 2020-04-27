//! Teapot benches

use std::num::NonZeroU32;

pub mod data;
pub mod mark;

pub mod config {
    use std::num::NonZeroU32;

    use vulkayes_core::ash::vk;

    pub const DEFAULT_WINDOW_SIZE: [NonZeroU32; 2] = unsafe {
        [
            NonZeroU32::new_unchecked(1200),
            NonZeroU32::new_unchecked(675),
        ]
    };
    pub const DEPTH_FORMAT: vk::Format = vk::Format::D16_UNORM;
}

#[macro_export]
macro_rules! sub_in_release {
    (
		debug = $debug: expr;
		release = $release: expr;
	) => {{
        #[cfg(debug_assertions)]
        let res = $debug;

        #[cfg(not(debug_assertions))]
        let res = $release;

        res
    }};
}

pub fn setup_logger() {
    let logger = edwardium_logger::Logger::new(
        [edwardium_logger::targets::stderr::StderrTarget::new(
            vulkayes_core::log::Level::Trace,
        )],
        std::time::Instant::now(),
    );
    logger.init_boxed().expect("Could not initialize logger");
}

// pub fn setup_winit(
//     title: &str,
//     size: [NonZeroU32; 2],
// ) -> (winit::event_loop::EventLoop<()>, Window) {
//     let event_loop = winit::event_loop::EventLoop::new();
//     let window = winit::window::WindowBuilder::new()
//         .with_title(title)
//         .with_inner_size(winit::dpi::LogicalSize::new(
//             f64::from(size[0].get()),
//             f64::from(size[1].get()),
//         ))
//         .build(&event_loop)
//         .expect("could not create window");
//
//     (event_loop, window)
// }

pub fn setup_minifb(title: &str, size: [NonZeroU32; 2]) -> vulkayes_window::minifb::minifb::Window {
    vulkayes_window::minifb::minifb::Window::new(
        title,
        size[0].get() as usize,
        size[1].get() as usize,
        vulkayes_window::minifb::minifb::WindowOptions::default(),
    )
    .expect("Could not create window")
}
