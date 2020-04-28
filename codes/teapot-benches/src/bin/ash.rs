use std::{ffi::CStr, num::NonZeroU32, time::Instant};

use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};

use vulkayes_core::ash::{self, vk};

use teapot_benches::{config, data, mark, sub_in_release};

mod memory {
	use vulkayes_core::ash::vk;

	pub fn find_memory_type_index(
		memory_req: &vk::MemoryRequirements,
		memory_prop: &vk::PhysicalDeviceMemoryProperties,
		flags: vk::MemoryPropertyFlags
	) -> Option<u32> {
		// vulkayes_core::log::debug!(
		// 	"Finding best memory type index for {:?}: {:#?} {:?}",
		// 	memory_req, flags, memory_prop
		// );

		// Try to find an exactly matching memory flag
		let best_suitable_index =
			find_memory_type_index_f(memory_req, memory_prop, flags, |property_flags, flags| {
				property_flags == flags
			});
		if best_suitable_index.is_some() {
			return best_suitable_index
		}
		// Otherwise find a memory flag that works
		find_memory_type_index_f(memory_req, memory_prop, flags, |property_flags, flags| {
			property_flags & flags == flags
		})
	}

	fn find_memory_type_index_f<F: Fn(vk::MemoryPropertyFlags, vk::MemoryPropertyFlags) -> bool>(
		memory_req: &vk::MemoryRequirements,
		memory_prop: &vk::PhysicalDeviceMemoryProperties,
		flags: vk::MemoryPropertyFlags,
		f: F
	) -> Option<u32> {
		let mut memory_type_bits = memory_req.memory_type_bits;
		for (index, ref memory_type) in memory_prop.memory_types.iter().enumerate() {
			if memory_type_bits & 1 == 1 && f(memory_type.property_flags, flags) {
				return Some(index as u32)
			}
			memory_type_bits >>= 1;
		}
		None
	}
}

fn main() {
	// PREINIT //
	let time_before_preinit = Instant::now();

	teapot_benches::setup_logger();
	mark::init_static();

	let mut window = teapot_benches::setup_minifb("Teapot - ash", config::DEFAULT_WINDOW_SIZE);

	// INIT //
	let time_before_init = Instant::now();
	let entry = ash::Entry::new().expect("Could not create entry");

	// Instance
	let (instance, debug_report) = {
		let application_name = CStr::from_bytes_with_nul(b"Teapot - ash\0").unwrap();

		let application_info = vk::ApplicationInfo::builder()
			.application_name(&application_name)
			.application_version(0)
			.engine_name(&application_name)
			.engine_version(0)
			.api_version(vk::make_version(1, 2, 0));

		let enabled_layer_names: Vec<_> = {
			#[allow(unused_variables)]
			let layers: [&'static CStr; 1] =
				[CStr::from_bytes_with_nul(b"VK_LAYER_KHRONOS_validation\0").unwrap()];

			sub_in_release! {
				debug = layers.iter().copied().map(CStr::as_ptr).collect();
				release = Vec::new();
			}
		};
		let enabled_extension_names: Vec<_> = {
			let extensions: [&'static CStr; 2] =
				vulkayes_window::minifb::required_extensions(&window);

			sub_in_release! {
				debug = std::iter::once(ash::extensions::ext::DebugReport::name()).chain(extensions.iter().copied()).map(CStr::as_ptr).collect();
				release = extensions.iter().copied().map(CStr::as_ptr).collect();
			}
		};
		let create_info = vk::InstanceCreateInfo::builder()
			.application_info(&application_info)
			.enabled_layer_names(&enabled_layer_names)
			.enabled_extension_names(&enabled_extension_names);

		let instance = unsafe {
			entry
				.create_instance(&create_info, None)
				.expect("Could not create Instance")
		};

		let debug_report = sub_in_release! {
			debug = {
				let create_info = vk::DebugReportCallbackCreateInfoEXT::builder()
					.flags(vk::DebugReportFlagsEXT::all())
					.pfn_callback(
						Some(vulkayes_core::instance::debug::default_debug_callback)
					);

				let debug_report_loader = ash::extensions::ext::DebugReport::new(&entry, &instance);
				let debug_call_back = unsafe {
					debug_report_loader
						.create_debug_report_callback(&create_info, None)
						.expect("Could not create DebugReport")
				};

				Some((debug_report_loader, debug_call_back))
			};
			release = None::<(ash::extensions::ext::DebugReport, vk::DebugReportCallbackEXT)>;
		};

		(instance, debug_report)
	};

	// Surface
	let (surface, surface_loader) = {
		let loader = ash::extensions::khr::Surface::new(&entry, &instance);

		let surface = unsafe {
			vulkayes_window::minifb::create_surface_raw(&window, &entry, &instance, None)
				.expect("Could not create surface")
		};

		(surface, loader)
	};

	// Device
	let (physical_device, device, queue, queue_family_index) = {
		let physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };
		let physical_device = *physical_devices
			.get(data::physical_device_index())
			.unwrap_or(physical_devices.get(0).expect("No physical devices found"));

		let queue_family_index = unsafe {
			instance
				.get_physical_device_queue_family_properties(physical_device)
				.iter()
				.enumerate()
				.find(|(index, info)| {
					let supports_graphics = info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
					let supports_surface = surface_loader
						.get_physical_device_surface_support(
							physical_device,
							*index as u32,
							surface
						)
						.unwrap();

					vulkayes_core::log::debug!(
						"Queue family #{}: {:#?} (supports surface: {})",
						index,
						info,
						supports_surface
					);

					supports_graphics && supports_surface
				})
				.expect("Could not find fitting queue family")
				.0 as u32
		};

		let device_extension_names_raw = [ash::extensions::khr::Swapchain::name().as_ptr()];
		let features = vk::PhysicalDeviceFeatures {
			shader_clip_distance: 1,
			..Default::default()
		};
		let priorities = [1.0];

		let queue_info = [vk::DeviceQueueCreateInfo::builder()
			.queue_family_index(queue_family_index)
			.queue_priorities(&priorities)
			.build()];

		let device_create_info = vk::DeviceCreateInfo::builder()
			.queue_create_infos(&queue_info)
			.enabled_extension_names(&device_extension_names_raw)
			.enabled_features(&features);

		let device = unsafe {
			instance
				.create_device(physical_device, &device_create_info, None)
				.expect("Could not create device")
		};

		let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

		(physical_device, device, queue, queue_family_index)
	};

	// Device memory
	let device_memory_properties =
		unsafe { instance.get_physical_device_memory_properties(physical_device) };

	// Swapchain
	let (swapchain, swapchain_loader, surface_format, surface_size) = {
		let surface_format = unsafe {
			surface_loader
				.get_physical_device_surface_formats(physical_device, surface)
				.unwrap()
				.into_iter()
				.inspect(|v| vulkayes_core::log::trace!("surface format {:?}", v))
				.next()
				.unwrap()
		};

		let surface_capabilities = unsafe {
			surface_loader
				.get_physical_device_surface_capabilities(physical_device, surface)
				.unwrap()
		};
		let desired_image_count = {
			if surface_capabilities.max_image_count == surface_capabilities.min_image_count {
				surface_capabilities.max_image_count
			} else {
				surface_capabilities.min_image_count + 1
			}
		};

		// Works for correct hardcoded resolutions only
		let surface_size = [
			NonZeroU32::new(surface_capabilities.min_image_extent.width).unwrap(),
			NonZeroU32::new(surface_capabilities.min_image_extent.height).unwrap()
		];
		let surface_resolution = vk::Extent2D {
			width: surface_size[0].get(),
			height: surface_size[1].get()
		};

		let pre_transform = if surface_capabilities
			.supported_transforms
			.contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
		{
			vk::SurfaceTransformFlagsKHR::IDENTITY
		} else {
			surface_capabilities.current_transform
		};
		let present_mode = {
			// let modes = unsafe {
			// 	surface_loader.get_physical_device_surface_present_modes(physical_device, surface).unwrap()
			// };

			vk::PresentModeKHR::IMMEDIATE
		};
		let swapchain_loader = ash::extensions::khr::Swapchain::new(&instance, &device);

		let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
			.surface(surface)
			.min_image_count(desired_image_count)
			.image_color_space(surface_format.color_space)
			.image_format(surface_format.format)
			.image_extent(surface_resolution)
			.image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
			.image_sharing_mode(vk::SharingMode::EXCLUSIVE)
			.pre_transform(pre_transform)
			.composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
			.present_mode(present_mode)
			.clipped(true)
			.image_array_layers(1);

		let swapchain = unsafe {
			swapchain_loader
				.create_swapchain(&swapchain_create_info, None)
				.expect("Could not create swapchain")
		};

		(swapchain, swapchain_loader, surface_format, surface_size)
	};

	// Present views
	let present_images_views = {
		let present_images = unsafe {
			swapchain_loader
				.get_swapchain_images(swapchain)
				.expect("Could not get swapchain images")
		};
		let present_images_views: Vec<_> = present_images
			.into_iter()
			.map(|image| {
				let create_view_info = vk::ImageViewCreateInfo::builder()
					.view_type(vk::ImageViewType::TYPE_2D)
					.format(surface_format.format)
					.components(vk::ComponentMapping {
						r: vk::ComponentSwizzle::R,
						g: vk::ComponentSwizzle::G,
						b: vk::ComponentSwizzle::B,
						a: vk::ComponentSwizzle::A
					})
					.subresource_range(vk::ImageSubresourceRange {
						aspect_mask: vk::ImageAspectFlags::COLOR,
						base_mip_level: 0,
						level_count: 1,
						base_array_layer: 0,
						layer_count: 1
					})
					.image(image);
				let view = unsafe {
					device
						.create_image_view(&create_view_info, None)
						.expect("Could not create present image view")
				};

				(image, view)
			})
			.collect();

		present_images_views
	};

	// Depth image
	let (depth_image, depth_image_memory, depth_image_view) = {
		let depth_image_create_info = vk::ImageCreateInfo::builder()
			.image_type(vk::ImageType::TYPE_2D)
			.format(vk::Format::D16_UNORM)
			.extent(vk::Extent3D {
				width: surface_size[0].get(),
				height: surface_size[1].get(),
				depth: 1
			})
			.mip_levels(1)
			.array_layers(1)
			.samples(vk::SampleCountFlags::TYPE_1)
			.tiling(vk::ImageTiling::OPTIMAL)
			.usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
			.sharing_mode(vk::SharingMode::EXCLUSIVE);

		let depth_image = unsafe {
			device
				.create_image(&depth_image_create_info, None)
				.expect("Could not create depth image")
		};

		let depth_image_memory = {
			let depth_image_memory_req =
				unsafe { device.get_image_memory_requirements(depth_image) };
			let depth_image_memory_index = memory::find_memory_type_index(
				&depth_image_memory_req,
				&device_memory_properties,
				vk::MemoryPropertyFlags::DEVICE_LOCAL
			)
			.expect("Unable to find suitable memory type for depth image.");

			let depth_image_allocate_info = vk::MemoryAllocateInfo::builder()
				.allocation_size(depth_image_memory_req.size)
				.memory_type_index(depth_image_memory_index);

			let depth_image_memory = unsafe {
				device
					.allocate_memory(&depth_image_allocate_info, None)
					.expect("Could not allocate device memory for depth image")
			};

			unsafe {
				device
					.bind_image_memory(depth_image, depth_image_memory, 0)
					.expect("Could not bind memory to depth image");
			}

			depth_image_memory
		};

		let depth_image_view_info = vk::ImageViewCreateInfo::builder()
			.subresource_range(
				vk::ImageSubresourceRange::builder()
					.aspect_mask(vk::ImageAspectFlags::DEPTH)
					.level_count(1)
					.layer_count(1)
					.build()
			)
			.image(depth_image)
			.format(depth_image_create_info.format)
			.view_type(vk::ImageViewType::TYPE_2D);

		let depth_image_view = unsafe {
			device
				.create_image_view(&depth_image_view_info, None)
				.unwrap()
		};

		(depth_image, depth_image_memory, depth_image_view)
	};

	// Fence and semaphores
	let (submit_fence, acquire_semaphore, render_semaphore) = {
		let fence_create_info = vk::FenceCreateInfo::default();
		let fence = unsafe {
			device
				.create_fence(&fence_create_info, None)
				.expect("Could not create fence")
		};

		let semaphore_create_info = vk::SemaphoreCreateInfo::default();
		let acquire_semaphore = unsafe {
			device
				.create_semaphore(&semaphore_create_info, None)
				.expect("Could not create sempahore")
		};
		let render_semaphore = unsafe {
			device
				.create_semaphore(&semaphore_create_info, None)
				.expect("Could not create semaphore")
		};

		(fence, acquire_semaphore, render_semaphore)
	};

	// Command pool and command buffer
	let (command_pool, command_buffer) = {
		let pool_create_info = vk::CommandPoolCreateInfo::builder()
			.flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
			.queue_family_index(queue_family_index);

		let pool = unsafe {
			device
				.create_command_pool(&pool_create_info, None)
				.expect("Could not create command pool")
		};

		let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
			.command_buffer_count(1)
			.command_pool(pool)
			.level(vk::CommandBufferLevel::PRIMARY);

		let command_buffers = unsafe {
			device
				.allocate_command_buffers(&command_buffer_allocate_info)
				.expect("Could not allocate command buffer")
		};

		(pool, command_buffers.into_iter().next().unwrap())
	};

	// Render pass
	let render_pass = {
		let attachments = [
			vk::AttachmentDescription {
				format: surface_format.format,
				samples: vk::SampleCountFlags::TYPE_1,
				load_op: vk::AttachmentLoadOp::CLEAR,
				store_op: vk::AttachmentStoreOp::STORE,
				initial_layout: vk::ImageLayout::UNDEFINED,
				final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
				..Default::default()
			},
			vk::AttachmentDescription {
				format: config::DEPTH_FORMAT,
				samples: vk::SampleCountFlags::TYPE_1,
				load_op: vk::AttachmentLoadOp::CLEAR,
				store_op: vk::AttachmentStoreOp::DONT_CARE,
				stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
				stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
				initial_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
				final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
				..Default::default()
			}
		];

		let color_attachment_refs = [vk::AttachmentReference {
			attachment: 0,
			layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL
		}];
		let depth_attachment_ref = vk::AttachmentReference {
			attachment: 1,
			layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL
		};

		let subpasses = [vk::SubpassDescription::builder()
			.color_attachments(&color_attachment_refs)
			.depth_stencil_attachment(&depth_attachment_ref)
			.pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
			.build()];

		let dependencies = [vk::SubpassDependency {
			src_subpass: vk::SUBPASS_EXTERNAL,
			src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
				| vk::PipelineStageFlags::HOST,
			src_access_mask: vk::AccessFlags::HOST_WRITE,
			dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
				| vk::AccessFlags::COLOR_ATTACHMENT_WRITE
				| vk::AccessFlags::MEMORY_READ,
			dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
			..Default::default()
		}];

		let create_info = vk::RenderPassCreateInfo::builder()
			.attachments(&attachments)
			.subpasses(&subpasses)
			.dependencies(&dependencies);

		unsafe {
			device
				.create_render_pass(&create_info, None)
				.expect("Could not create render pass")
		}
	};

	// Frame buffers
	let frame_buffers = {
		present_images_views
			.iter()
			.map(|(_, present_view)| {
				let attachments = [*present_view, depth_image_view];
				let create_info = vk::FramebufferCreateInfo::builder()
					.render_pass(render_pass)
					.attachments(&attachments)
					.width(surface_size[0].get())
					.height(surface_size[1].get())
					.layers(1);

				unsafe {
					device
						.create_framebuffer(&create_info, None)
						.expect("Could not create framebuffer")
				}
			})
			.collect::<Vec<_>>()
	};

	// Index buffer
	let (index_buffer, index_buffer_memory) = {
		let create_info = vk::BufferCreateInfo {
			size: std::mem::size_of_val(&data::INDICES) as vk::DeviceSize,
			usage: vk::BufferUsageFlags::INDEX_BUFFER,
			sharing_mode: vk::SharingMode::EXCLUSIVE,
			..Default::default()
		};
		let buffer = unsafe {
			device
				.create_buffer(&create_info, None)
				.expect("Could not create index buffer")
		};

		let memory_req = unsafe { device.get_buffer_memory_requirements(buffer) };
		let memory_index = memory::find_memory_type_index(
			&memory_req,
			&device_memory_properties,
			vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
		)
		.expect("Unable to find suitable memory type for the index buffer.");

		let allocate_info = vk::MemoryAllocateInfo {
			allocation_size: memory_req.size,
			memory_type_index: memory_index,
			..Default::default()
		};
		let memory = unsafe {
			device
				.allocate_memory(&allocate_info, None)
				.expect("Could not allocate index buffer memory")
		};
		unsafe {
			device
				.bind_buffer_memory(buffer, memory, 0)
				.expect("Could not bind index buffer memory");
		}

		(buffer, memory)
	};

	// Vertex buffer
	let (vertex_buffer, vertex_buffer_memory) = {
		let create_info = vk::BufferCreateInfo {
			size: std::mem::size_of_val(&data::VERTICES) as vk::DeviceSize,
			usage: vk::BufferUsageFlags::VERTEX_BUFFER,
			sharing_mode: vk::SharingMode::EXCLUSIVE,
			..Default::default()
		};
		let buffer = unsafe {
			device
				.create_buffer(&create_info, None)
				.expect("Could not create vertex buffer")
		};

		let memory_req = unsafe { device.get_buffer_memory_requirements(buffer) };
		let memory_index = memory::find_memory_type_index(
			&memory_req,
			&device_memory_properties,
			vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
		)
		.expect("Unable to find suitable memory type for the index buffer.");

		let allocate_info = vk::MemoryAllocateInfo {
			allocation_size: memory_req.size,
			memory_type_index: memory_index,
			..Default::default()
		};
		let memory = unsafe {
			device
				.allocate_memory(&allocate_info, None)
				.expect("Could not allocate vertex buffer memory")
		};
		unsafe {
			device
				.bind_buffer_memory(buffer, memory, 0)
				.expect("Could not bind vertex buffer memory");
		}

		(buffer, memory)
	};

	// Normal buffer
	let (normal_buffer, normal_buffer_memory) = {
		let create_info = vk::BufferCreateInfo {
			size: std::mem::size_of_val(&data::NORMALS) as vk::DeviceSize,
			usage: vk::BufferUsageFlags::VERTEX_BUFFER,
			sharing_mode: vk::SharingMode::EXCLUSIVE,
			..Default::default()
		};
		let buffer = unsafe {
			device
				.create_buffer(&create_info, None)
				.expect("Could not create normal buffer")
		};

		let memory_req = unsafe { device.get_buffer_memory_requirements(buffer) };
		let memory_index = memory::find_memory_type_index(
			&memory_req,
			&device_memory_properties,
			vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
		)
		.expect("Unable to find suitable memory type for the index buffer.");

		let allocate_info = vk::MemoryAllocateInfo {
			allocation_size: memory_req.size,
			memory_type_index: memory_index,
			..Default::default()
		};
		let memory = unsafe {
			device
				.allocate_memory(&allocate_info, None)
				.expect("Could not allocate normal buffer memory")
		};
		unsafe {
			device
				.bind_buffer_memory(buffer, memory, 0)
				.expect("Could not bind normal buffer memory");
		}

		(buffer, memory)
	};

	// Uniform buffer
	let (uniform_buffer, uniform_buffer_memory, uniform_buffer_memory_ptr) = {
		let create_info = vk::BufferCreateInfo {
			size: std::mem::size_of::<data::UniformData>() as vk::DeviceSize,
			usage: vk::BufferUsageFlags::UNIFORM_BUFFER,
			sharing_mode: vk::SharingMode::EXCLUSIVE,
			..Default::default()
		};
		let buffer = unsafe {
			device
				.create_buffer(&create_info, None)
				.expect("Could not create uniform buffer")
		};

		let memory_req = unsafe { device.get_buffer_memory_requirements(buffer) };
		let memory_index = memory::find_memory_type_index(
			&memory_req,
			&device_memory_properties,
			vk::MemoryPropertyFlags::HOST_VISIBLE
		)
		.expect("Unable to find suitable memory type for uniform buffer.");

		let allocate_info = vk::MemoryAllocateInfo {
			allocation_size: memory_req.size,
			memory_type_index: memory_index,
			..Default::default()
		};
		let memory = unsafe {
			device
				.allocate_memory(&allocate_info, None)
				.expect("Could not allocate uniform buffer memory")
		};
		unsafe {
			device
				.bind_buffer_memory(buffer, memory, 0)
				.expect("Could not bind uniform buffer memory");
		}

		let memory_ptr = unsafe {
			device
				.map_memory(
					memory,
					0,
					std::mem::size_of::<data::UniformData>() as vk::DeviceSize,
					vk::MemoryMapFlags::empty()
				)
				.unwrap()
		} as *mut data::UniformData;

		(buffer, memory, memory_ptr)
	};

	// Descriptors
	let (descriptor_pool, descriptor_layout, descriptor_set) = {
		let sizes = [vk::DescriptorPoolSize {
			ty: vk::DescriptorType::UNIFORM_BUFFER,
			descriptor_count: 1
		}];
		let pool_create_info = vk::DescriptorPoolCreateInfo::builder()
			.pool_sizes(&sizes)
			.max_sets(1);
		let pool = unsafe {
			device
				.create_descriptor_pool(&pool_create_info, None)
				.expect("Could not create descriptor set pool")
		};

		let bindings = [vk::DescriptorSetLayoutBinding {
			descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
			descriptor_count: 1,
			stage_flags: vk::ShaderStageFlags::VERTEX,
			..Default::default()
		}];
		let layout_create_info = vk::DescriptorSetLayoutCreateInfo::builder().bindings(&bindings);
		let layout = unsafe {
			device
				.create_descriptor_set_layout(&layout_create_info, None)
				.expect("Could not create descriptor set layout")
		};

		let layouts = [layout];
		let set_alloc_info = vk::DescriptorSetAllocateInfo::builder()
			.descriptor_pool(pool)
			.set_layouts(&layouts);
		let sets = unsafe {
			device
				.allocate_descriptor_sets(&set_alloc_info)
				.expect("Could not allocate descriptor set")
		};
		let set = sets.into_iter().next().unwrap();

		(pool, layout, set)
	};

	// Shaders
	let (vertex_shader, fragment_shader, shader_info) = {
		let vertex_shader = {
			let code = data::vertex_shader();
			let create_info = vk::ShaderModuleCreateInfo::builder().code(&code);
			unsafe {
				device
					.create_shader_module(&create_info, None)
					.expect("Could not create vertex shader module")
			}
		};

		let fragment_shader = {
			let code = data::fragment_shader();
			let create_info = vk::ShaderModuleCreateInfo::builder().code(&code);
			unsafe {
				device
					.create_shader_module(&create_info, None)
					.expect("Could not create fragment shader module")
			}
		};

		let entry_name: &'static CStr = CStr::from_bytes_with_nul(b"main\0").unwrap();
		let shader_stage_create_infos = [
			vk::PipelineShaderStageCreateInfo::builder()
				.module(vertex_shader)
				.name(entry_name)
				.stage(vk::ShaderStageFlags::VERTEX)
				.build(),
			vk::PipelineShaderStageCreateInfo::builder()
				.module(fragment_shader)
				.name(entry_name)
				.stage(vk::ShaderStageFlags::FRAGMENT)
				.build()
		];
		let vertex_input_binding_descriptions = [
			vk::VertexInputBindingDescription {
				binding: 0,
				stride: std::mem::size_of::<data::Vertex>() as u32,
				input_rate: vk::VertexInputRate::VERTEX
			},
			vk::VertexInputBindingDescription {
				binding: 1,
				stride: std::mem::size_of::<data::Normal>() as u32,
				input_rate: vk::VertexInputRate::VERTEX
			}
		];
		let vertex_input_attribute_descriptions = [
			vk::VertexInputAttributeDescription {
				location: 0,
				binding: 0,
				format: vk::Format::R32G32B32_SFLOAT,
				offset: 0
			},
			vk::VertexInputAttributeDescription {
				location: 1,
				binding: 0,
				format: vk::Format::R32G32B32_SFLOAT,
				offset: 0
			}
		];

		let shader_info = (
			shader_stage_create_infos,
			vertex_input_binding_descriptions,
			vertex_input_attribute_descriptions
		);

		(vertex_shader, fragment_shader, shader_info)
	};

	// Pipeline layout
	let pipeline_layout = {
		let descriptor_layouts = [descriptor_layout];
		let push_ranges = [vk::PushConstantRange::builder()
			.offset(0)
			.size(std::mem::size_of::<data::PushData>() as u32)
			.stage_flags(vk::ShaderStageFlags::VERTEX)
			.build()];

		let layout_create_info = vk::PipelineLayoutCreateInfo::builder()
			.set_layouts(&descriptor_layouts)
			.push_constant_ranges(&push_ranges);
		unsafe {
			device
				.create_pipeline_layout(&layout_create_info, None)
				.expect("Could not create pipeline layout")
		}
	};

	// Graphics pipeline
	let graphics_pipeline = {
		let viewports = [vk::Viewport {
			x: 0.0,
			y: 0.0,
			width: surface_size[0].get() as f32,
			height: surface_size[1].get() as f32,
			min_depth: 0.0,
			max_depth: 1.0
		}];
		let scissors = [vk::Rect2D {
			extent: vk::Extent2D {
				width: surface_size[0].get(),
				height: surface_size[1].get()
			},
			..Default::default()
		}];
		let viewport_state_info = vk::PipelineViewportStateCreateInfo::builder()
			.viewports(&viewports)
			.scissors(&scissors);

		let rasterization_info = vk::PipelineRasterizationStateCreateInfo::builder()
			.front_face(vk::FrontFace::COUNTER_CLOCKWISE)
			.polygon_mode(vk::PolygonMode::FILL)
			.cull_mode(vk::CullModeFlags::NONE)
			.line_width(1.0);
		let multisample_state_info = vk::PipelineMultisampleStateCreateInfo::builder()
			.rasterization_samples(vk::SampleCountFlags::TYPE_1);

		let depth_stencil_state_create_info = vk::PipelineDepthStencilStateCreateInfo::builder()
			.depth_test_enable(true)
			.depth_write_enable(true)
			.depth_compare_op(vk::CompareOp::LESS)
			.depth_bounds_test_enable(false);

		let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState::builder()
			.blend_enable(false)
			.color_write_mask(vk::ColorComponentFlags::all())
			.build()];
		let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
			.logic_op_enable(false)
			.attachments(&color_blend_attachment_states);

		// let dynamic_state = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
		// let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::builder()
		// 	.dynamic_states(&dynamic_state)
		// ;

		let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo::builder()
			.vertex_binding_descriptions(&shader_info.1)
			.vertex_attribute_descriptions(&shader_info.2);
		let vertex_input_assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo::builder()
			.topology(vk::PrimitiveTopology::TRIANGLE_LIST);

		let graphic_pipeline_infos = vk::GraphicsPipelineCreateInfo::builder()
			.stages(&shader_info.0)
			.vertex_input_state(&vertex_input_state_info)
			.input_assembly_state(&vertex_input_assembly_state_info)
			.rasterization_state(&rasterization_info)
			.viewport_state(&viewport_state_info)
			.multisample_state(&multisample_state_info)
			.depth_stencil_state(&depth_stencil_state_create_info)
			.color_blend_state(&color_blend_state)
			.layout(pipeline_layout)
			.render_pass(render_pass);

		let graphics_pipelines = unsafe {
			device
				.create_graphics_pipelines(
					vk::PipelineCache::null(),
					&[graphic_pipeline_infos.build()],
					None
				)
				.expect("Could not create graphics pipeline")
		};

		graphics_pipelines.into_iter().next().unwrap()
	};

	// Misc variables
	let clear_values = [
		vk::ClearValue {
			color: vk::ClearColorValue {
				float32: [0.0, 0.5, 0.0, 1.0]
			}
		},
		vk::ClearValue {
			depth_stencil: vk::ClearDepthStencilValue {
				depth: 1.0,
				stencil: 0
			}
		}
	];

	// DATA UPLOAD //
	let time_before_data_upload = Instant::now();

	// Transfer image layouts
	{
		unsafe {
			let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
				.flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
			device
				.begin_command_buffer(command_buffer, &command_buffer_begin_info)
				.unwrap();
			{
				let barriers: Vec<_> = std::iter::once(
					vk::ImageMemoryBarrier::builder()
						.image(depth_image)
						.dst_access_mask(
							vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
								| vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE
						)
						.old_layout(vk::ImageLayout::UNDEFINED)
						.new_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
						.subresource_range(
							vk::ImageSubresourceRange::builder()
								.aspect_mask(vk::ImageAspectFlags::DEPTH)
								.layer_count(1)
								.level_count(1)
								.build()
						)
						.build()
				)
				.chain(present_images_views.iter().map(|(image, _)| {
					vk::ImageMemoryBarrier::builder()
						.image(*image)
						.dst_access_mask(
							vk::AccessFlags::COLOR_ATTACHMENT_READ
								| vk::AccessFlags::COLOR_ATTACHMENT_WRITE
						)
						.old_layout(vk::ImageLayout::UNDEFINED)
						.new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
						.subresource_range(
							vk::ImageSubresourceRange::builder()
								.aspect_mask(vk::ImageAspectFlags::COLOR)
								.layer_count(1)
								.level_count(1)
								.build()
						)
						.build()
				}))
				.collect();
				device.cmd_pipeline_barrier(
					command_buffer,
					vk::PipelineStageFlags::BOTTOM_OF_PIPE,
					vk::PipelineStageFlags::ALL_GRAPHICS,
					vk::DependencyFlags::empty(),
					&[],
					&[],
					&barriers
				);
			}
			device.end_command_buffer(command_buffer).unwrap();

			let command_buffers = [command_buffer];
			let submit_info = vk::SubmitInfo::builder().command_buffers(&command_buffers);
			device
				.queue_submit(queue, &[submit_info.build()], submit_fence)
				.expect("Could not submit depth image layout transition");

			device
				.wait_for_fences(&[submit_fence], true, std::u64::MAX)
				.expect("Could not wait for fence");
			device
				.reset_fences(&[submit_fence])
				.expect("Could not reset fence");
		}
	}

	// Upload indices
	{
		let memory_ptr = unsafe {
			device
				.map_memory(
					index_buffer_memory,
					0,
					std::mem::size_of_val(&data::INDICES) as vk::DeviceSize,
					vk::MemoryMapFlags::empty()
				)
				.unwrap()
		};
		unsafe {
			std::ptr::copy_nonoverlapping(
				(&data::INDICES) as *const u16,
				memory_ptr as *mut u16,
				data::INDICES.len()
			);
		}

		unsafe {
			device.unmap_memory(index_buffer_memory);
		}
	}

	// Upload vertices
	{
		let memory_ptr = unsafe {
			device
				.map_memory(
					vertex_buffer_memory,
					0,
					std::mem::size_of_val(&data::VERTICES) as vk::DeviceSize,
					vk::MemoryMapFlags::empty()
				)
				.unwrap()
		};
		unsafe {
			std::ptr::copy_nonoverlapping(
				(&data::VERTICES) as *const _ as *const u8,
				memory_ptr as *mut u8,
				std::mem::size_of_val(&data::VERTICES)
			);
		}

		unsafe {
			device.unmap_memory(vertex_buffer_memory);
		}
	}

	// Upload normals
	{
		let memory_ptr = unsafe {
			device
				.map_memory(
					normal_buffer_memory,
					0,
					std::mem::size_of_val(&data::NORMALS) as vk::DeviceSize,
					vk::MemoryMapFlags::empty()
				)
				.unwrap()
		};
		unsafe {
			std::ptr::copy_nonoverlapping(
				(&data::NORMALS) as *const _ as *const u8,
				memory_ptr as *mut u8,
				std::mem::size_of_val(&data::NORMALS)
			);
		}

		unsafe {
			device.unmap_memory(normal_buffer_memory);
		}
	}

	// Write descriptor set
	{
		let buffer_info = vk::DescriptorBufferInfo {
			buffer: uniform_buffer,
			offset: 0,
			range: std::mem::size_of::<data::UniformData>() as vk::DeviceSize
		};
		let write_info = [vk::WriteDescriptorSet {
			dst_set: descriptor_set,
			descriptor_count: 1,
			descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
			p_buffer_info: &buffer_info,
			..Default::default()
		}];
		unsafe {
			device.update_descriptor_sets(&write_info, &[]);
		}
	}

	// LOOP //
	let time_before_loop = Instant::now();
	let mut loop_iterations = 0;
	loop {
		let mut mark_state = mark::LoopMarkState::start();

		// initial loop variables
		let elapsed = Instant::now()
			.duration_since(time_before_loop)
			.as_secs_f32();
		let frame_state = data::UniformData::for_frame(elapsed, surface_size);

		mark_state.before_acquire();
		// acquire present image
		let (present_index, _) = unsafe {
			swapchain_loader
				.acquire_next_image(
					swapchain,
					std::u64::MAX,
					acquire_semaphore,
					vk::Fence::null()
				)
				.expect("Could not acquire next image")
		};

		mark_state.before_uniform();
		// update uniform data
		{
			unsafe {
				*uniform_buffer_memory_ptr = frame_state;
			}
			let flush_ranges = [vk::MappedMemoryRange::builder()
				.memory(uniform_buffer_memory)
				.size(std::mem::size_of::<data::UniformData>() as vk::DeviceSize)
				.build()];
			unsafe {
				device
					.flush_mapped_memory_ranges(&flush_ranges)
					.expect("Could not flush uniform data memory");
			}
		}

		mark_state.before_command();
		// fill command buffer
		{
			unsafe {
				// Reset the command buffer. This is okay because we ensure the execution is done by the point by waiting on the fence.
				device
					.reset_command_buffer(
						command_buffer,
						vk::CommandBufferResetFlags::RELEASE_RESOURCES
					)
					.expect("Could not reset command buffer");

				let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
					.flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
				device
					.begin_command_buffer(command_buffer, &command_buffer_begin_info)
					.expect("Could not begin command buffer");

				// do work
				{
					let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
						.render_pass(render_pass)
						.framebuffer(frame_buffers[present_index as usize])
						.render_area(vk::Rect2D {
							offset: vk::Offset2D { x: 0, y: 0 },
							extent: vk::Extent2D {
								width: surface_size[0].get(),
								height: surface_size[1].get()
							}
						})
						.clear_values(&clear_values);

					device.cmd_begin_render_pass(
						command_buffer,
						&render_pass_begin_info,
						vk::SubpassContents::INLINE
					);

					device.cmd_bind_descriptor_sets(
						command_buffer,
						vk::PipelineBindPoint::GRAPHICS,
						pipeline_layout,
						0,
						&[descriptor_set],
						&[]
					);
					device.cmd_bind_pipeline(
						command_buffer,
						vk::PipelineBindPoint::GRAPHICS,
						graphics_pipeline
					);
					device.cmd_bind_vertex_buffers(
						command_buffer,
						0,
						&[vertex_buffer, normal_buffer],
						&[0, 0]
					);
					device.cmd_bind_index_buffer(
						command_buffer,
						index_buffer,
						0,
						vk::IndexType::UINT16
					);

					for index in 0 .. data::NUMBER_OF_TEAPOTS {
						let object_data = data::PushData::for_object(elapsed, index);
						device.cmd_push_constants(
							command_buffer,
							pipeline_layout,
							vk::ShaderStageFlags::VERTEX,
							0,
							std::slice::from_raw_parts(
								&object_data as *const _ as *const u8,
								std::mem::size_of::<data::PushData>()
							)
						);
						device.cmd_draw_indexed(
							command_buffer,
							data::INDICES.len() as u32,
							1,
							0,
							0,
							1
						);
					}

					device.cmd_end_render_pass(command_buffer);
				}

				device
					.end_command_buffer(command_buffer)
					.expect("Could not end command buffer");
			}
		}

		mark_state.before_submit();
		// submit command buffer
		{
			let command_buffers = [command_buffer];

			let wait_semaphores = [acquire_semaphore];
			let wait_stages = [vk::PipelineStageFlags::BOTTOM_OF_PIPE];
			let signal_semaphores = [render_semaphore];
			let submit_info = vk::SubmitInfo::builder()
				.wait_semaphores(&wait_semaphores)
				.wait_dst_stage_mask(&wait_stages)
				.command_buffers(&command_buffers)
				.signal_semaphores(&signal_semaphores);

			unsafe {
				device
					.queue_submit(queue, &[submit_info.build()], submit_fence)
					.expect("Could not submit frame");
			}
		}

		mark_state.before_present();
		// present frame
		{
			let wait_semaphores = [render_semaphore];
			let swapchains = [swapchain];
			let indices = [present_index];

			let present_info = vk::PresentInfoKHR::builder()
				.wait_semaphores(&wait_semaphores)
				.swapchains(&swapchains)
				.image_indices(&indices);

			unsafe {
				swapchain_loader
					.queue_present(queue, &present_info)
					.expect("Could not present");
			}
		}

		mark_state.before_wait();
		// wait for frame finish since we want to bench the frame times
		{
			unsafe {
				device
					.wait_for_fences(&[submit_fence], true, std::u64::MAX)
					.expect("Could not wait for fence");
				device
					.reset_fences(&[submit_fence])
					.expect("Could not reset fence");
			}
		}

		mark_state.before_update();
		// update window and check for closing event
		window.update();

		mark_state.finish();
		loop_iterations += 1;
		if !window.is_open() || loop_iterations >= mark::MAX_LOOPS {
			break
		}
	}

	// CLEANUP //
	let time_before_cleanup = Instant::now();
	unsafe {
		device
			.device_wait_idle()
			.expect("Could not wait for device");

		// Graphics pipeline
		device.destroy_pipeline(graphics_pipeline, None);

		// Pipeline layout
		device.destroy_pipeline_layout(pipeline_layout, None);

		// Shaders
		device.destroy_shader_module(fragment_shader, None);
		device.destroy_shader_module(vertex_shader, None);

		// Descriptor layout and pool
		device.destroy_descriptor_set_layout(descriptor_layout, None);
		device.destroy_descriptor_pool(descriptor_pool, None);

		// Uniform buffer
		device.unmap_memory(uniform_buffer_memory);
		device.free_memory(uniform_buffer_memory, None);
		device.destroy_buffer(uniform_buffer, None);

		// Normal buffer
		device.free_memory(normal_buffer_memory, None);
		device.destroy_buffer(normal_buffer, None);

		// Vertex buffer
		device.free_memory(vertex_buffer_memory, None);
		device.destroy_buffer(vertex_buffer, None);

		// Index buffer
		device.free_memory(index_buffer_memory, None);
		device.destroy_buffer(index_buffer, None);

		// Frame buffers
		for frame_buffer in frame_buffers {
			device.destroy_framebuffer(frame_buffer, None);
		}

		// Render pass
		device.destroy_render_pass(render_pass, None);

		// Command pool
		device.free_command_buffers(command_pool, &[command_buffer]);
		device.destroy_command_pool(command_pool, None);

		// Fence and semaphores
		device.destroy_fence(submit_fence, None);
		device.destroy_semaphore(acquire_semaphore, None);
		device.destroy_semaphore(render_semaphore, None);

		// Depth image
		device.destroy_image_view(depth_image_view, None);
		device.free_memory(depth_image_memory, None);
		device.destroy_image(depth_image, None);

		// Present views
		for (_image, view) in present_images_views.into_iter() {
			device.destroy_image_view(view, None);
		}

		// Swapchain
		swapchain_loader.destroy_swapchain(swapchain, None);

		// Device
		device.destroy_device(None);

		// surface
		surface_loader.destroy_surface(surface, None);

		// Instance
		if let Some(debug) = debug_report {
			debug.0.destroy_debug_report_callback(debug.1, None);
		}

		instance.destroy_instance(None);
	}

	// SUMMARY //
	let time_before_summary = Instant::now();
	mark::cleanup_static();
	mark::output_summary(
		time_before_preinit,
		time_before_init,
		time_before_data_upload,
		time_before_loop,
		loop_iterations,
		time_before_cleanup,
		time_before_summary
	);
}
