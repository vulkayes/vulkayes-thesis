# Implementation

## Bindings

Vulkan API is an interface specified in the C programming language. C language is the de-facto standard in cross-language APIs. This means the system of bindings is available in almost any practical language, including Rust. Vulkayes relies on the ash[@ash] crate to provide these binding and some syntactic sugar on top. This library uses the Vulkan API Registry, canonical machine-readable definition of the API, to generate bindings from Rust to C automatically.

## Cargo features {#sec:impl_cargo}

An important part of any flexible project is to give the user as much control as possible, so the library will fit their usecase. One way to achieve this in Rust are cargo features already mentioned in [@sec:design_cargo].

The most important features defined and exposed in Vulkayes are described below.

#### **naive_device_allocator**

This feature conditionally compiles a very naive device memory allocator into the project. Device memory allocation is a complex topic and applications are required to provide their own allocators to fit their own needs. One popular allocator is the Vulkan Memory Allocator[@VMA], but it is a big dependency that might not be easily accessible for certain usecases. Vulkayes supports integration with VMA (and other allocators) seamlessly, but also provides the naive allocator as a simple no-dependecy alterative for quick prototyping and debugging.

#### **multi_thread** {#sec:multi_thread_feature}

One of the biggest selling points of Vulkan are its multi-threading capabilities. Since the user is in charge of synchronizing the resources, they can design their application to fit their needs. Single-threaded applications require no synchronization, while multi-threaded applications should allow for the full power of multi-threading to be leveraged.

Safe Rust statically prevents data races using the built-in `Send` and `Sync` traits. These traits are automatically implemented (or not implemented) by the compiler to mark types as "capable of being sent between threads" and "capable of being borrowed across threads" respectively. The user is free to unsafely implement these traits back if the compiler decides to not implement them, provided that the user takes the burden of preventing data races upon themselves.

By default, object wrappers in Vulkayes are no `Send` nor `Sync`, simply because they use the `Rc` type, which is a shared pointer wrapper type that uses non-atomic loads and writes to count. By turning this feature on, all usages of `Rc` across the crate are switched to `Arc`, which is atomically counted and thus implemented `Send` and `Sync` safely.

Additionaly, single-threaded Vulkayes replaces use of mutexes with simple wrapper types that emulate the mutex API, but do not implement `Send`/`Sync` and do not do any synchronization. This makes the API of both single-threaded and multi-threaded Vulkayes uniform. The main reason for this feature is performance, since atomic operations and synchronization is costly compared to non-atomic counterparts.

#### **runtime_implicit_validations**

Vulkayes aims to increase safety of Vulkan calls as much as possible without any performance impact. The idea is to always guarantee that the implicit validations defined in Vulkan spec are fulfilled and the explicit validations are only fulfilled when they can be easily derived from the existing API design.

This proved to not be always possible, so a small portion ([@tbl:validation-stats]) of implicit validations requires some runtime checking to ensure their fulfillment. These validation, producing runtime overhead, are conditionaly compiled using this feature to ensure that the user can always opt-out to achieve greater performance.

## Abstraction

### Reference counting

Reference counting is used 

### Type aliases {#sec:v_aliases}

Vulkayes makes use of project-wide type aliases to make transitioning some of the cargo features seamless.

#### `Vrc`

One of the most important type aliases which resolves to either `type Vrc<T> = Arc<T>` or `type Vrc<T> = Rc<T>` depending on whether the multi-threaded feature is enabled or not. This type alias is used practically everywhere, since most types (as seen in [@fig:object_dependnency_graph]) are wrapped in reference counter pointer types.

![](generics.md)

### Deref

Talk about Deref trait usage

### User code a.k.a. `dyn FnMut`

Talk about usage of `dyn`

![](swapchain_recreate.md)

## Windowing

Vulkan handles windowing by providing abstraction over native windows on different platforms using extensions. Each supported platform has a specific extension that can be used to construct a Vulkan handle to a surface, which an object abstracting over the native surface. Some platforms, notably macOS and iOS, have additional requirements on the creation process of the window.

Vulkayes provides abstraction over this in a separate crate called `vulkayes-window`. This crate contains three tiers of code. First tier is raw Vulkan creation method for each platform. This code is platform specific and highly unsafe. Second tier are implementation specific creation methods, which abstract over platform differences using the windowing library implementation, but still require unsafe code for the ash objects.

The third and most important tier are the implementation specific Vulkayes creation methods. These methods are _safe_ and provide full abstraction over the platform and ash, returning Vulkayes wrapper types ready to be used safely. These methods are the main point of the `vulkayes-window` crate, but the other tiers are provided for flexibility and transparency reasons.

Currently supported implementations are:

* `winit` - The most popular fully-featured windowing library in Rust ecosystem. Provides almost full abstraction over platform windows.
* `minifb` - One of the simplest and easiest to use windowing libraries. Provides the minimal needed abstraction to quicky and easily create a draw on windows.
* `raw_window_handle` - A library providing common types intended for all Rust windowing libraries. Both `winit` and `minifb` use this library and `vulkayes-window` theoretically supports all libraries that can be glued through this library.