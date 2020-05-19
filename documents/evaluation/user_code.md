## User code {#sec:user-code}

One of the main concerns when designing a library is the user code. How the user code will look like, if it will be readable and comfortable to write. Vulkayes user code was compared with code extracted from `ash` usage examples&nbsp;[@ash], which serve as a baseline for Rust Vulkan bindings, in code samples \ref{code_vertex_buffer_upload_example} and \ref{code_memory_map_example}.

\codeColumnsBegin{2}
```{.rust .numberLines}
let (vertex_buffer, vertex_buffer_memory) = {
	let create_info = vk::BufferCreateInfo {
		size: std::mem::size_of_val(
			&data::VERTICES
		) as vk::DeviceSize,
		usage: vk::BufferUsageFlags::VERTEX_BUFFER,
		sharing_mode: vk::SharingMode::EXCLUSIVE,
		..Default::default()
	};
	let buffer = unsafe {
		device
			.create_buffer(&create_info, None)
			.expect("Could not create vertex buffer")
	};

	let memory_req = unsafe {
		device.get_buffer_memory_requirements(buffer)
	};
	let memory_index = memory::find_memory_type_index(
		&memory_req,
		&device_memory_properties,
		vk::MemoryPropertyFlags::HOST_VISIBLE
			| vk::MemoryPropertyFlags::HOST_COHERENT
	)
	.expect("Unable to find suitable memory type");

	let allocate_info = vk::MemoryAllocateInfo {
		allocation_size: memory_req.size,
		memory_type_index: memory_index,
		..Default::default()
	};
	let memory = unsafe {
		device
			.allocate_memory(&allocate_info, None)
			.expect("Could not allocate memory")
	};
	unsafe {
		device
			.bind_buffer_memory(buffer, memory, 0)
			.expect("Could not bind memory");
	}

	(buffer, memory)
};
```

\codeColumnBreak
```rust
let vertex_buffer = {
	Buffer::new(
		device.clone(),
		std::num::NonZeroU64::new(
			std::mem::size_of_val(&data::VERTICES) as u64
		).unwrap(),
		vk::BufferUsageFlags::VERTEX_BUFFER,
		SharingMode::from(queue.as_ref()),
		BufferAllocatorParams::Some {
			allocator: &device_memory_allocator,
			requirements: vk::MemoryPropertyFlags::HOST_VISIBLE
				| vk::MemoryPropertyFlags::HOST_COHERENT
		},
		Default::default()
	)
	.expect("Could not create vertex buffer")
};
```

\codeColumnsEnd{
	An example of the code with same functionality from the original examples in ash (left) and from the current ones (right). The code after is three times shorter than the original code while exposing the same functionality and providing static validation guarantees.
}{code_vertex_buffer_upload_example}

Overall, the code for benchmarking the spinning teapots written in pure `ash` has 1400 lines of Rust code. The code with Vulkayes with same semantics and even improved static validation guarantess has 942 lines. This is a difference of 458 lines of code. These numbers are clear indicators of the improvement in developer experience by using correctly designed wrappers.

Another interesting code example is the uniform buffer usage:

\codeColumnsBegin{2}
```{.rust .numberLines}
unsafe {
	*uniform_buffer_memory_ptr = frame_state;
}
let flush_ranges = [
	vk::MappedMemoryRange::builder()
		.memory(uniform_buffer_memory)
		.size(
			std::mem::size_of::<data::UniformData>() 
				as vk::DeviceSize
		)
	.build()
];
unsafe {
	device
		.flush_mapped_memory_ranges(&flush_ranges)
		.expect("Could not flush uniform data memory");
}
```

\codeColumnBreak
```rust
uniform_buffer
	.memory().unwrap()
	.map_memory_with(|mut mem| {
		mem.write_slice(&[frame_state], 0, Default::default());
		mem.flush().expect("Could not flush uniform data memory");
		MappingAccessResult::Continue
	})
	.unwrap();
```

\codeColumnsEnd{
	The ash code (left) is twice as long and in some cases possibly even unsafe. Vulkayes API guarantees proper locking and borrowing, provides simplified way to flush the memory and prevents unaligned writes which on some platforms might cause hard errors and abort the process. The checking for correctness, however, does have some runtime cost. One of the guarantees of safe Rust is memory safety and Vulkayes is targeting safe Rust. That is why the write slice method call above does more than just write to a pointer. There is logic to check the align of the pointer and make sure all writes are either properly aligned, or an unaligned instruction is used.
}{code_memory_map_example}