## Abstraction

### Reference counting {#sec:ref_counting}

Reference counting is used for two purposes. First, the most obvious one, is shared usage. Most of the objects in Vulkan stem from the Device object and operations on these objects require access to the device and device pointers. Second is lifetime dependecy. Objects in Vulkan have a defined partial ordering on their destruction order, that is, the device must pretty much outlive all its children. This is achieved as a consequence of shared pointer usage, since all children of the device keep a link to the device, the last child to be dropped drops the device (unless the user is holding the device pointer as well, in which case it is still alive).

### Type aliases {#sec:v_aliases}

Vulkayes makes use of project-wide type aliases to make transitioning some of the cargo features seamless.

#### `Vrc`

One of the most important type aliases which resolves to either `type Vrc<T> = Arc<T>` or `type Vrc<T> = Rc<T>` depending on whether the multi-threaded feature is enabled or not. This type alias is used practically everywhere, since most types (as seen in [@fig:object_dependnency_graph] and discussed in [@sec:ref_counting]) are wrapped in reference counted pointer types.

### Deref

This trait comes from the Rust standard library and is a specially known trait to the compiler. It is intended to be implemented for smart pointer types as a way to uniquely claim that a type Bar is really just a wrapper around type Foo. This fits nicely with the notion of smart wrappers in Vulkayes. For example the `Image` object:

```rust
pub struct Image {
	image: vk::Image,
	// fields omitted
}
impl Deref for Image {
	type Target = vk::Image;

	fn deref(&self) -> &Self::Target {
		&self.image
	}
}
```

This way, the `Image` object claims that under the hood it is simply the `vk::Image` handle but with some added information and utility on top. The `Deref` trait itself defines an associated type `Target` and a `deref` method. These two things together provide complete information about what type the `Image` object derefs to and an ability to borrow it as that type.

This is used heavily across Vulkayes for all smart wrappers around Vulkan handles. Additionally, Vulkayes makes heavy use of Rust macros-by-example system to implement most important traits (such as `Eq`, `Hash` and `Ord`) on each of the smart wrapper objects. The `impl_common_handle_traits` macro saves over 500 lines of repetitive code across the Vulkayes crate.