use std::{convert::TryFrom, ffi::CStr, num::NonZeroU32, time::Instant};

use ash::version::DeviceV1_0;
use vulkayes_core::ash::{self, vk};

use vulkayes_core::memory::device::naive::NaiveDeviceMemoryAllocator;
use vulkayes_core::prelude::*;
use vulkayes_core::{render_pass_description, vertex_input_attributes};

use teapot_benches::{config, data, mark, sub_in_release};

fn main() {
    // PREINIT //
    let time_before_preinit = Instant::now();

    teapot_benches::setup_logger();
    mark::init_static();

    let mut window = teapot_benches::setup_minifb("Teapot - Vulkayes", config::DEFAULT_WINDOW_SIZE);

    let time_before_init;
    let time_before_data_upload;
    let time_before_loop;
    let time_before_cleanup;
    let mut loop_iterations = 0;

    // INIT // (scoped so we can observe drop time)
    {
        time_before_init = Instant::now();
        let entry = Entry::new().expect("Could not create entry");

        // Instance
        let instance = {
            #[allow(unused_variables)]
            let layers: [&'static CStr; 1] =
                [CStr::from_bytes_with_nul(b"VK_LAYER_KHRONOS_validation\0").unwrap()];
            let enabled_layer_names = sub_in_release! {
                debug = layers.iter().copied();
                release = std::iter::empty::<&'static CStr>();
            };

            #[allow(unused_variables)]
            let extensions: [&'static CStr; 2] =
                vulkayes_window::minifb::required_extensions(&window);
            let enabled_extension_names = sub_in_release! {
                debug = std::iter::once(ash::extensions::ext::DebugReport::name()).chain(extensions.iter().copied());
                release = extensions.iter().copied();
            };

            let debug_callback = sub_in_release! {
                debug = vulkayes_core::instance::debug::DebugCallback::Default();
                release = vulkayes_core::instance::debug::DebugCallback::None();
            };

            Instance::new(
                entry,
                ApplicationInfo {
                    application_name: "Teapot - vulkayes",
                    application_version: VkVersion::new(0, 1, 0),
                    engine_name: "Teapot - vulkayes",
                    engine_version: VkVersion::new(0, 1, 0),
                    api_version: VkVersion::new(1, 2, 0),
                },
                enabled_layer_names,
                enabled_extension_names,
                Default::default(),
                debug_callback,
            )
            .expect("Could not create instance")
        };

        // Surface
        let surface = {
            vulkayes_window::minifb::create_surface(instance.clone(), &window, Default::default())
                .expect("Could not create surface")
        };

        // Device
        let (device, queue) = {
            let physical_device = instance.physical_devices().unwrap().next().unwrap();

            let queue_index = physical_device
                .queue_family_properties()
                .into_iter()
                .enumerate()
                .find(|(index, info)| {
                    let supports_graphics = info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                    let supports_surface = surface
                        .physical_device_surface_support(&physical_device, *index as u32)
                        .unwrap();

                    supports_graphics && supports_surface
                })
                .expect("Could not find fitting queue family")
                .0 as u32;

            let data = Device::new(
                physical_device,
                [QueueCreateInfo {
                    queue_family_index: queue_index,
                    queue_priorities: [1.0],
                }],
                std::iter::empty(),
                std::iter::once(ash::extensions::khr::Swapchain::name()),
                vk::PhysicalDeviceFeatures {
                    shader_clip_distance: 1,
                    ..Default::default()
                },
                Default::default(),
            )
            .expect("Could not create device");

            (data.device, data.queues.into_iter().next().unwrap())
        };

        // Device memory
        let device_memory_allocator = NaiveDeviceMemoryAllocator::new(device.clone());

        // Swapchain
        let (swapchain, swapchain_images, surface_size) = {
            let surface_format = surface
                .physical_device_surface_formats(device.physical_device())
                .unwrap()
                .into_iter()
                .inspect(|v| vulkayes_core::log::trace!("surface format {:?}", v))
                .next()
                .unwrap();

            let surface_capabilities = surface
                .physical_device_surface_capabilities(device.physical_device())
                .unwrap();
            let desired_image_count = std::num::NonZeroU32::new(
                if surface_capabilities.max_image_count == surface_capabilities.min_image_count {
                    surface_capabilities.max_image_count
                } else {
                    surface_capabilities.min_image_count + 1
                },
            )
            .unwrap();
            let surface_size = ImageSize::new_2d(
                NonZeroU32::new(surface_capabilities.min_image_extent.width).unwrap(),
                NonZeroU32::new(surface_capabilities.min_image_extent.height).unwrap(),
                vulkayes_core::NONZEROU32_ONE,
                MipmapLevels::One(),
            );

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

            let swapchain = Swapchain::new(
                device.clone(),
                surface,
                SwapchainCreateInfo {
                    image_info: SwapchainCreateImageInfo {
                        min_image_count: desired_image_count,
                        image_format: surface_format.format,
                        image_color_space: surface_format.color_space,
                        image_size: surface_size,
                        image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
                    },
                    sharing_mode: SharingMode::from(&*queue),
                    pre_transform,
                    composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
                    present_mode,
                    clipped: true,
                },
                Default::default(),
            )
            .expect("Could not create swapchain");

            (swapchain.swapchain, swapchain.images, surface_size)
        };

        // Present views
        let present_views = {
            swapchain_images
                .into_iter()
                .map(|image| {
                    ImageView::new(
                        MixedDynImage::SwapchainImage(image),
                        ImageViewRange::Type2D(0, vulkayes_core::NONZEROU32_ONE, 0),
                        None,
                        vk::ComponentMapping {
                            r: vk::ComponentSwizzle::R,
                            g: vk::ComponentSwizzle::G,
                            b: vk::ComponentSwizzle::B,
                            a: vk::ComponentSwizzle::A,
                        },
                        vk::ImageAspectFlags::COLOR,
                        Default::default(),
                    )
                    .expect("Could not create present image view")
                })
                .collect::<Vec<_>>()
        };

        // Depth image
        let depth_image_view = {
            let image = Image::new(
                device.clone(),
                vk::Format::D16_UNORM,
                ImageSizeInfo::General(surface_size.into()),
                ImageTilingAndLayout::OptimalUndefined(),
                vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
                SharingMode::from(&*queue),
                ImageAllocatorParams::Some {
                    allocator: &device_memory_allocator,
                    requirements: vk::MemoryPropertyFlags::DEVICE_LOCAL,
                },
                Default::default(),
            )
            .expect("Could not create depth image");

            let view = ImageView::new(
                MixedDynImage::Image(image),
                ImageViewRange::Type2D(0, vulkayes_core::NONZEROU32_ONE, 0),
                None,
                Default::default(),
                vk::ImageAspectFlags::DEPTH,
                Default::default(),
            )
            .expect("Could not create depth image view");

            view
        };

        // Fence and semaphores
        let (submit_fence, acquire_semaphore, render_semaphore) = {
            let fence = Fence::new(device.clone(), false, Default::default())
                .expect("Could not create fence");

            let acquire_semaphore = Semaphore::binary(device.clone(), Default::default())
                .expect("Could not create sempahore");
            let render_semaphore = Semaphore::binary(device.clone(), Default::default())
                .expect("Could not create sempahore");

            (fence, acquire_semaphore, render_semaphore)
        };

        // Command pool and command buffer
        let (command_pool, command_buffer) = {
            let pool = CommandPool::new(
                &queue,
                vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
                Default::default(),
            )
            .expect("Could not create command pool");

            let command_buffer =
                CommandBuffer::new(pool.clone(), true).expect("Could not allocate command buffer");

            (pool, command_buffer)
        };

        // Render pass
        let render_pass = {
            let (attachments, holder) = render_pass_description! {
                Attachments {
                    UNUSED,
                    color {
                        format = present_views[0].format(),
                        ops = AttachmentOps::Color {
                             load: vk::AttachmentLoadOp::CLEAR,
                             store: vk::AttachmentStoreOp::STORE
                        },
                        layouts = vk::ImageLayout::UNDEFINED => ImageLayoutFinal::PRESENT_SRC_KHR
                    }
                    depth {
                        format = config::DEPTH_FORMAT,
                        ops = AttachmentOps::DepthStencil {
                             depth_load: vk::AttachmentLoadOp::CLEAR,
                             depth_store: vk::AttachmentStoreOp::DONT_CARE,
                             stencil_load: vk::AttachmentLoadOp::DONT_CARE,
                             stencil_store: vk::AttachmentStoreOp::DONT_CARE
                        },
                        layouts = vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL => ImageLayoutFinal::DEPTH_STENCIL_ATTACHMENT_OPTIMAL
                    }
                }
                Subpasses {
                    first {
                        color = [@color{ImageLayoutAttachment::COLOR_ATTACHMENT_OPTIMAL}]
                        depth_stencil = @depth
                    }
                }
            };

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

            let subpasses = [SubpassDescription::try_from(&holder).unwrap()];
            RenderPass::new(
                device.clone(),
                &attachments,
                &subpasses,
                &dependencies,
                Default::default(),
            )
            .expect("Could not create render pass")
        };

        // Frame buffers
        let frame_buffers = {
            present_views
                .iter()
                .map(|view| {
                    Framebuffer::new(
                        render_pass.clone(),
                        std::iter::once(view.clone())
                            .chain(std::iter::once(depth_image_view.clone())),
                        [surface_size.width(), surface_size.height()],
                        surface_size.array_layers(),
                        Default::default(),
                    )
                    .expect("Could not create frame buffer")
                })
                .collect::<Vec<_>>()
        };

        // Index buffer
        let index_buffer = {
            Buffer::new(
                device.clone(),
                std::num::NonZeroU64::new(std::mem::size_of_val(&data::INDICES) as u64).unwrap(),
                vk::BufferUsageFlags::INDEX_BUFFER,
                (&*queue).into(),
                BufferAllocatorParams::Some {
                    allocator: &device_memory_allocator,
                    requirements: vk::MemoryPropertyFlags::HOST_VISIBLE
                        | vk::MemoryPropertyFlags::HOST_COHERENT,
                },
                Default::default(),
            )
            .expect("Could not create index buffer")
        };

        // Vertex buffer
        let vertex_buffer = {
            Buffer::new(
                device.clone(),
                std::num::NonZeroU64::new(std::mem::size_of_val(&data::VERTICES) as u64).unwrap(),
                vk::BufferUsageFlags::VERTEX_BUFFER,
                (&*queue).into(),
                BufferAllocatorParams::Some {
                    allocator: &device_memory_allocator,
                    requirements: vk::MemoryPropertyFlags::HOST_VISIBLE
                        | vk::MemoryPropertyFlags::HOST_COHERENT,
                },
                Default::default(),
            )
            .expect("Could not create vertex buffer")
        };

        // Normal buffer
        let normal_buffer = {
            Buffer::new(
                device.clone(),
                std::num::NonZeroU64::new(std::mem::size_of_val(&data::NORMALS) as u64).unwrap(),
                vk::BufferUsageFlags::VERTEX_BUFFER,
                (&*queue).into(),
                BufferAllocatorParams::Some {
                    allocator: &device_memory_allocator,
                    requirements: vk::MemoryPropertyFlags::HOST_VISIBLE
                        | vk::MemoryPropertyFlags::HOST_COHERENT,
                },
                Default::default(),
            )
            .expect("Could not create normal buffer")
        };

        // Uniform buffer
        let uniform_buffer = {
            Buffer::new(
                device.clone(),
                std::num::NonZeroU64::new(std::mem::size_of::<data::UniformData>() as u64).unwrap(),
                vk::BufferUsageFlags::UNIFORM_BUFFER,
                (&*queue).into(),
                BufferAllocatorParams::Some {
                    allocator: &device_memory_allocator,
                    requirements: vk::MemoryPropertyFlags::DEVICE_LOCAL
                        | vk::MemoryPropertyFlags::HOST_VISIBLE,
                },
                Default::default(),
            )
            .expect("Could not create uniform buffer")
        };

        // Descriptors
        let (descriptor_pool, descriptor_layout, descriptor_set) = {
            let pool = DescriptorPool::new(
                device.clone(),
                vk::DescriptorPoolCreateFlags::empty(),
                vulkayes_core::NONZEROU32_ONE,
                std::iter::once(DescriptorPoolSize {
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                    count: vulkayes_core::NONZEROU32_ONE,
                }),
                None,
                Default::default(),
            )
            .expect("Could not create descriptor set pool");

            let layout = DescriptorSetLayout::new(
                device.clone(),
                Default::default(),
                std::iter::once(DescriptorSetLayoutBinding::Generic(
                    DescriptorSetLayoutBindingGenericType::UNIFORM_BUFFER,
                    vulkayes_core::NONZEROU32_ONE,
                    vk::ShaderStageFlags::VERTEX,
                )),
                Default::default(),
            )
            .expect("Could not create descriptor set layout");

            let set = DescriptorSet::new(pool.clone(), layout.clone())
                .expect("Could not allocate descriptor set");

            (pool, layout, set)
        };

        // Shaders
        let (vertex_shader, fragment_shader, shader_info) = {
            let vertex_shader =
                ShaderModule::new(device.clone(), data::vertex_shader(), Default::default())
                    .expect("Could not create vertex shader");

            let fragment_shader =
                ShaderModule::new(device.clone(), data::fragment_shader(), Default::default())
                    .expect("Could not create vertex shader");

            let entry_name: &'static CStr = CStr::from_bytes_with_nul(b"main\0").unwrap();
            let shader_stage_create_infos = [
                vk::PipelineShaderStageCreateInfo::builder()
                    .module(vertex_shader.handle())
                    .name(entry_name)
                    .stage(vk::ShaderStageFlags::VERTEX)
                    .build(),
                vk::PipelineShaderStageCreateInfo::builder()
                    .module(fragment_shader.handle())
                    .name(entry_name)
                    .stage(vk::ShaderStageFlags::FRAGMENT)
                    .build(),
            ];
            let input_bindings = [
                vk::VertexInputBindingDescription {
                    binding: 0,
                    stride: std::mem::size_of::<data::Vertex>() as u32,
                    input_rate: vk::VertexInputRate::VERTEX,
                },
                vk::VertexInputBindingDescription {
                    binding: 1,
                    stride: std::mem::size_of::<data::Normal>() as u32,
                    input_rate: vk::VertexInputRate::VERTEX,
                },
            ];
            let input_attributes = vertex_input_attributes! {
                layout(location = 0) in vec3 position;
                layout(location = 1) in vec3 normal;
            };

            let shader_info = (shader_stage_create_infos, input_bindings, input_attributes);

            (vertex_shader, fragment_shader, shader_info)
        };

        // Pipeline layout
        let pipeline_layout = {
            PipelineLayout::new(
                device.clone(),
                std::iter::once(descriptor_layout),
                std::iter::once(PushConstantRange {
                    stage_flags: vk::ShaderStageFlags::VERTEX,
                    offset_div_four: 0,
                    size_div_four: std::num::NonZeroU32::new(
                        std::mem::size_of::<data::PushData>() as u32 / 4,
                    )
                    .unwrap(),
                }),
                Default::default(),
            )
            .expect("Could not create pipeline layout")
        };

        // Graphics pipeline
        let graphics_pipeline = {
            // TODO
            let viewports = [vk::Viewport {
                x: 0.0,
                y: 0.0,
                width: surface_size.width().get() as f32,
                height: surface_size.height().get() as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            }];
            let scissors = [vk::Rect2D {
                extent: vk::Extent2D {
                    width: surface_size.width().get(),
                    height: surface_size.height().get(),
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

            let depth_stencil_state_create_info =
                vk::PipelineDepthStencilStateCreateInfo::builder()
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
            let vertex_input_assembly_state_info =
                vk::PipelineInputAssemblyStateCreateInfo::builder()
                    .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

            let graphic_pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
                .stages(&shader_info.0)
                .vertex_input_state(&vertex_input_state_info)
                .input_assembly_state(&vertex_input_assembly_state_info)
                //
                .rasterization_state(&rasterization_info)
                .viewport_state(&viewport_state_info)
                .multisample_state(&multisample_state_info)
                //
                .depth_stencil_state(&depth_stencil_state_create_info)
                .color_blend_state(&color_blend_state)
                //
                .layout(pipeline_layout.handle())
                .render_pass(render_pass.handle());

            let pipeline = unsafe {
                GraphicsPipeline::from_create_info(
                    device.clone(),
                    graphic_pipeline_info,
                    Default::default(),
                )
                .expect("Could not create graphics pipeline")
            };

            pipeline
        };

        // Misc variables
        let clear_values = [
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.5, 0.0, 1.0],
                },
            },
            vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            },
        ];

        // DATA UPLOAD //
        time_before_data_upload = Instant::now();

        // Transfer image layouts
        {
            unsafe {
                let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
                    .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

                {
                    let cb = command_buffer.lock_handle();
                    device
                        .begin_command_buffer(*cb, &command_buffer_begin_info)
                        .unwrap();
                    {
                        let barriers: Vec<_> = std::iter::once(
                            vk::ImageMemoryBarrier::builder()
                                .image(depth_image_view.image().handle())
                                .dst_access_mask(
                                    vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
                                        | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
                                )
                                .old_layout(vk::ImageLayout::UNDEFINED)
                                .new_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                                .subresource_range(
                                    vk::ImageSubresourceRange::builder()
                                        .aspect_mask(vk::ImageAspectFlags::DEPTH)
                                        .layer_count(1)
                                        .level_count(1)
                                        .build(),
                                )
                                .build(),
                        )
                        .chain(present_views.iter().map(|view| {
                            vk::ImageMemoryBarrier::builder()
                                .image(view.image().handle())
                                .dst_access_mask(
                                    vk::AccessFlags::COLOR_ATTACHMENT_READ
                                        | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                                )
                                .old_layout(vk::ImageLayout::UNDEFINED)
                                .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                                .subresource_range(
                                    vk::ImageSubresourceRange::builder()
                                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                                        .layer_count(1)
                                        .level_count(1)
                                        .build(),
                                )
                                .build()
                        }))
                        .collect();
                        device.cmd_pipeline_barrier(
                            *cb,
                            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                            vk::PipelineStageFlags::ALL_GRAPHICS,
                            vk::DependencyFlags::empty(),
                            &[],
                            &[],
                            &barriers,
                        );
                    }
                    device.end_command_buffer(*cb).unwrap();
                }

                Queue::submit_one_fence_only(
                    &queue,
                    [],
                    [],
                    [&command_buffer],
                    [],
                    Some(&submit_fence),
                )
                .expect("Could not submit depth image layout transition");
                submit_fence
                    .wait(Default::default())
                    .expect("Could not wait for fence");
                submit_fence.reset().expect("Could not reset fence");
            }
        }

        // Upload indices
        {
            index_buffer
                .memory()
                .unwrap()
                .map_memory_with(|mut mem| {
                    mem.write_slice(&data::INDICES, 0, Default::default());

                    MappingAccessResult::Unmap
                })
                .expect("Could not map index buffer memory")
        }

        // Upload vertices
        {
            vertex_buffer
                .memory()
                .unwrap()
                .map_memory_with(|mut mem| {
                    mem.write_slice(&data::VERTICES, 0, Default::default());

                    MappingAccessResult::Unmap
                })
                .expect("Could not map vertex buffer memory")
        }

        // Upload normals
        {
            normal_buffer
                .memory()
                .unwrap()
                .map_memory_with(|mut mem| {
                    mem.write_slice(&data::NORMALS, 0, Default::default());

                    MappingAccessResult::Unmap
                })
                .expect("Could not map normal buffer memory")
        }

        // Write descriptor set
        {
            let ds = descriptor_set.lock_safe_handle();
            let buffer_infos = [DescriptorBufferInfo::new(
                &uniform_buffer,
                0,
                uniform_buffer.size(),
            )];
            let writes = [DescriptorSetWrite::new(
                ds.borrow_safe(),
                0,
                0,
                DescriptorSetWriteData::Buffer(DescriptorTypeBuffer::UNIFORM_BUFFER, &buffer_infos),
            )
            .unwrap()];
            descriptor_set.update(&writes, &[]);
        }

        // LOOP //
        time_before_loop = Instant::now();
        loop {
            let mut mark_state = mark::LoopMarkState::start();

            // initial loop variables
            let elapsed = Instant::now()
                .duration_since(time_before_loop)
                .as_secs_f32();
            let frame_state = data::UniformData::for_frame(
                elapsed,
                [surface_size.width(), surface_size.height()],
            );

            mark_state.before_acquire();
            // acquire present image
            let present_index = swapchain
                .acquire_next(
                    Default::default(),
                    AcquireSynchronization::Semaphore(&acquire_semaphore),
                )
                .expect("Could not acquire next image")
                .index();

            mark_state.before_uniform();
            // update uniform data
            {
                uniform_buffer
                    .memory()
                    .unwrap()
                    .map_memory_with(|mut mem| {
                        mem.write_slice(&[frame_state], 0, Default::default());

                        mem.flush().expect("Could not flush uniform data memory");

                        MappingAccessResult::Continue
                    })
                    .unwrap();
            }

            mark_state.before_command();
            // fill command buffer
            {
                let cb = command_buffer.lock_handle();
                let ds = descriptor_set.lock_handle();
                unsafe {
                    // Reset the command buffer. This is okay because we ensure the execution is done by the point by waiting on the fence.
                    device
                        .reset_command_buffer(*cb, vk::CommandBufferResetFlags::RELEASE_RESOURCES)
                        .expect("Could not reset command buffer");

                    let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
                        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
                    device
                        .begin_command_buffer(*cb, &command_buffer_begin_info)
                        .expect("Could not begin command buffer");

                    // do work
                    {
                        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
                            .render_pass(render_pass.handle())
                            .framebuffer(frame_buffers[present_index as usize].handle())
                            .render_area(vk::Rect2D {
                                offset: vk::Offset2D { x: 0, y: 0 },
                                extent: vk::Extent2D {
                                    width: surface_size.width().get(),
                                    height: surface_size.height().get(),
                                },
                            })
                            .clear_values(&clear_values);

                        device.cmd_begin_render_pass(
                            *cb,
                            &render_pass_begin_info,
                            vk::SubpassContents::INLINE,
                        );

                        device.cmd_bind_descriptor_sets(
                            *cb,
                            vk::PipelineBindPoint::GRAPHICS,
                            pipeline_layout.handle(),
                            0,
                            &[*ds],
                            &[],
                        );
                        device.cmd_bind_pipeline(
                            *cb,
                            vk::PipelineBindPoint::GRAPHICS,
                            graphics_pipeline.handle(),
                        );
                        device.cmd_bind_vertex_buffers(
                            *cb,
                            0,
                            &[vertex_buffer.handle(), normal_buffer.handle()],
                            &[0, 0],
                        );
                        device.cmd_bind_index_buffer(
                            *cb,
                            index_buffer.handle(),
                            0,
                            vk::IndexType::UINT16,
                        );

                        for index in 0..data::NUMBER_OF_TEAPOTS {
                            let object_data = data::PushData::for_object(elapsed, index);
                            device.cmd_push_constants(
                                *cb,
                                pipeline_layout.handle(),
                                vk::ShaderStageFlags::VERTEX,
                                0,
                                std::slice::from_raw_parts(
                                    &object_data as *const _ as *const u8,
                                    std::mem::size_of::<data::PushData>(),
                                ),
                            );
                            device.cmd_draw_indexed(*cb, data::INDICES.len() as u32, 1, 0, 0, 1);
                        }

                        device.cmd_end_render_pass(*cb);
                    }

                    device
                        .end_command_buffer(*cb)
                        .expect("Could not end command buffer");
                }
            }

            mark_state.before_submit();
            // submit command buffer
            {
                Queue::submit_one(
                    &queue,
                    [&acquire_semaphore],
                    [vk::PipelineStageFlags::BOTTOM_OF_PIPE],
                    [&command_buffer],
                    [&render_semaphore],
                    Some(&submit_fence),
                )
                .expect("Could not submit frame");
            }

            mark_state.before_present();
            // present frame
            {
                Queue::present_one(
                    &queue,
                    [&render_semaphore],
                    [present_views[present_index as usize]
                        .image()
                        .try_swapchain_image()
                        .unwrap()],
                    false,
                )
                .try_into_single()
                .unwrap()
                .expect("Could not present");
            }

            mark_state.before_wait();
            // wait for frame finish since we want to bench the frame times
            {
                submit_fence
                    .wait(Default::default())
                    .expect("Could not wait for fence");
                submit_fence.reset().expect("Could not reset fence");
            }

            mark_state.before_update();
            // update window and check for closing event
            window.update();

            mark_state.finish();
            loop_iterations += 1;
            if !window.is_open() {
                break;
            }
        }

        time_before_cleanup = Instant::now();
        // CLEANUP // (out of scope drops)
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
        time_before_summary,
    );
}
