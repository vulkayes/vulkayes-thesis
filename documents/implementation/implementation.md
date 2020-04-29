# Implementation

## Bindings

Vulkan API is an interface specified in the C programming language. C language is the de-facto standard in cross-language APIs. This means the system of bindings is available in almost any practical language, including Rust. Vulkayes relies on the ash[@ash] crate to provide these binding and some syntactic sugar on top. This library uses the Vulkan API Registry, canonical machine-readable definition of the API, to generate bindings from Rust to C automatically.

## Cargo features

Talk about what cargo features are available and why they are beneficial

### Vrc, Vutex etc.

Talk about Vrc and Vutex aliases

![](implementation/generics.md)

### Deref

Talk about Deref trait usage

### User code a.k.a. `dyn FnMut`

Talk about usage of `dyn`

![](implementation/swapchain_recreate.md)