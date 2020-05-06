# Implementation

## Bindings

Vulkan API is an interface specified in the C programming language. C language is the de-facto standard in cross-language APIs. This means the system of bindings is available in almost any practical language, including Rust. Vulkayes relies on the ash[@ash] crate to provide these binding and some syntactic sugar on top. This library uses the Vulkan API Registry, canonical machine-readable definition of the API, to generate bindings from Rust to C automatically.

![](cargo_features.md)

![](generics.md)

![](abstraction.md)

![](swapchain_recreate.md)

## Windowing

Vulkan handles windowing by providing abstraction over native windows on different platforms using extensions. Each supported platform has a specific extension that can be used to construct a Vulkan handle to a surface, which is an object abstracting over the native surface. Some platforms, notably macOS and iOS, have additional requirements on the creation process of the window.

Vulkayes provides abstraction over this in a separate crate called `vulkayes-window`. This crate contains three tiers of code. First tier is raw Vulkan creation method for each platform. This code is platform specific and highly unsafe. Second tier are implementation specific creation methods, which abstract over platform differences using the windowing library implementation, but still require unsafe code for the ash objects.

The third and most important tier are the implementation specific Vulkayes creation methods. These methods are _safe_ and provide full abstraction over the platform and ash, returning Vulkayes wrapper types ready to be used safely. These methods are the main point of the `vulkayes-window` crate, but the other tiers are provided for flexibility and transparency reasons.

Currently supported implementations are:

* `winit` - The most popular fully-featured windowing library in Rust ecosystem. Provides almost full abstraction over platform windows.
* `minifb` - One of the simplest and easiest to use windowing libraries. Provides the minimal needed abstraction to quicky and easily create and draw on window surfaces.
* `raw_window_handle` - A library providing common types intended for all Rust windowing libraries. Both `winit` and `minifb` use this library and `vulkayes-window` supports all libraries that can be glued through this library.