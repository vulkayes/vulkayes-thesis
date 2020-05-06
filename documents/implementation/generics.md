## Generics

Generics are used in key places across Vulkayes. One example are device memory allocators, another would be image views. They are described in detail below.

### Device memory allocator generics {#sec:device_memory_allocator_generics}

Device memory allocators have one of the biggest impact on performance of Vulkan. There is no default memory allocator in Vulkan. Instead, memory has to be allocated manually from the device. That operation, however, can be slow. That is why it is recommended by the Vulkan specification to allocate memory in bigger chunks at once and then distribute and reuse the memory as best as possible in the user code.

For Vulkayes, this means it is required to support user-defined allocators. This is the perfect usecase for generics. An image, which needs some kind of memory backing to operate, has a simplified constructor like this:

```rust
trait DeviceMemoryAllocation {
	// Allocation trait methods
}

trait DeviceMemoryAllocator {
	type Allocation: DeviceMemoryAllocation;

	fn allocate(&self) -> Self::Allocation;
}

struct Image {
	// Image fields
	memory: ??
}
impl Image {
	pub fn new<A: DeviceMemoryAllocator>(
		// Other fields
		memory_allocator: &A
	) -> Self {
		// Initialization code
	}
}
```

The `memory_allocator` parameter can be any user-defined type that implement the `DeviceMemoryAllocator` trait (and thus is capable of distributing memory given some requirements). However, given the requirements of Vulkan specification, we need to ensure that the memory outlives all usages of the image. This implies we need to store some kind of handle to the allocated memory, which can be any type implementing `DeviceMemoryAllocation` (as can be seen in the `DeviceMemoryAllocator` traits associated type `Allocation`).

Storing this memory thus has the same implications as mentioned above. We could make the `Image` struct generic over the memory it stores. This would however mean that the memory generic parameter would have to be present on anything that can possibly store the image, including swapchain images, image views, command buffers and so on. This could prevent us in the future from creating a command buffer and recording into it operations on images with possibly different memory allocations (for example, because one is a sparse image and the other is fully-backed).

Since this is very limiting, the memory inside an image can be stored using dynamic generics. So the `??` in the above code snippet would be replaced with `Box<dyn DeviceMemoryAllocation>`.

This would be ideal for images, where the memory does not need to be accessed until it is to be deallocated (barring linearly tiled images). For buffers, however, this is a common use case. Buffers are often used as staging or uniform. Data is uploaded into a buffer from the host and then copied using device operations into an image backed by fast device-local memory. The upload of data is done by mapping the memory into host memory using Vulkan provided mechanism and then writing to it as if it was normal host memory.

### Mappable memory generics

Some use cases for mapped memory are performance-critical. For example, vertex animating data is done by continuously changing vertex buffer data according to the animation properties. This means the mapped memory has to be accessed every frame. This is where dynamic dispatch cost would be substantial, it is best to avoid it.

One of the ways to avoid this cost is to simply push it back. There are only 3 places where the generics are truly needed:

* The memory map function
* The memory unmap function
* The cleanup function

No other place of the memory handling needs custom user coding. This means it is enough to store 3 generic user-provided functions. In Rust, this can be done using the `Fn` family of traits. For example, instead of `Box<dyn DeviceMemoryAllocation>` for the cleanup function we will use `Box<dyn FnOnce(&Vrc<Device>, vk::DeviceMemory, vk::DeviceSize, NonZeroU64)>` inside a concrete `struct DeviceMemoryAllocation`. The cleanup function can be simply `FnOnce`, which can only ever be called once, while the map and unmap functions might need to be called multiple times and have to be `FnMut`.

The performance of this solution is measured in more detail in [@sec:device_memory_allocator_generics].

### Image view generics

Image views are another object in Vulkano that has to deal with generics. Image view can wrap any type that can "act like" an image and create a view into some kind of subrange. This can be expressed using the `ImageTrait` like so:

```rust
struct ImageView {
	// Image view fields
	image: ??
}
impl ImageView {
	pub fn new<I: ImageTrait>(
		image: I
	) -> Self {
		// Initialization code
	}
}
```

As mentioned above, this is very limiting because of the generic parameter. Unlike the above case, however, the image field needs to be accessed considerably more often.

The following table shows a benchmark of so-called mixed dispatch, where an `enum` is used to provide common possible values for a given generic type and the last variant, which is the only one truly generic, is provided as a `Box<dyn Trait>` to allow using dynamic dispatch where the set of provided types is not extensive enough.

benchmark |avg. black box|avg. no black box
----------|-------------:|----------------:
Enum::Foo | 499.01 ps    | 251.31 ps
Enum::Bar | 499.47 ps    | 252.67 ps
Enum::Dyn | 1.3018 ns    | 1.2512 ns
Foo       | 499.36 ps    | 260.76 ps
Bar       | 499.03 ps    | 252.18 ps
Qux       | 313.34 ps    | 250.41 ps
dyn Qux   | 1.5104 ns    | 1.5028 ns

Table: Benchmark of so-called mixed dispatch enums, where enum variants house common types and the last variant houses a boxed dynamically dispatched one to cover other usecases.

As can be seen from the table, accessing a value through a dynamic dispatch is at least twice as slow as accessing it through static dispatch, and this is with optimizations prevented by using the concept of a black box from the Rust stdlib.

Non-black boxed benchmarks show that the optimizations provided by the compiler for statically dispatched values can further reduce the overhead of static dispatch, while the dynamic dispatch stays mostly the same.

This cones as an alternative to normal generic to avoid generic parameter plague and was chosen as an acceptable way to treat image type dispatch in Vulkayes.