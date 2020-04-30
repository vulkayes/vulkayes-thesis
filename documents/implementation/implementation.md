# Implementation

## Bindings

Vulkan API is an interface specified in the C programming language. C language is the de-facto standard in cross-language APIs. This means the system of bindings is available in almost any practical language, including Rust. Vulkayes relies on the ash[@ash] crate to provide these binding and some syntactic sugar on top. This library uses the Vulkan API Registry, canonical machine-readable definition of the API, to generate bindings from Rust to C automatically.

## Cargo features

An important part of any flexible project is to give the user as much control as possible, so the library will fit their usecase. One way to achieve this in Rust are cargo features already mentioned in [@sec:design-cargo].

The most important features defined and exposed in Vulkayes are described below.

#### **naive_device_allocator**

This feature conditionally compiles a very naive device memory allocator into the project. Device memory allocation is a complex topic and applications are required to provide their own allocators to fit their own needs. One popular allocator is the Vulkan Memory Allocator[@VMA], but it is a big dependency that might not be easily accessible for certain usecases. Vulkayes supports integration with VMA (and other allocators) seamlessly, but also provides the naive allocator as a simple no-dependecy alterative for quick prototyping and debugging.

#### **multi_thread**

One of the biggest selling points of Vulkan are its multi-threading capabilities. Since the user is in charge of synchronizing the resources, they can design their application to fit their needs. Single-threaded applications require no synchronization, while multi-threaded applications should allow for the full power of multi-threading to be leveraged.

Safe Rust statically prevents data races using the built-in `Send` and `Sync` traits. These traits are automatically implemented (or not implemented) by the compiler to mark types as "capable of being sent between threads" and "capable of being borrowed across threads" respectively. The user is free to unsafely implement these traits back if the compiler decides to not implement them, provided that the user takes the burden of preventing data races upon themselves.

By default, object wrappers in Vulkayes are no `Send` nor `Sync`, simply because they use the `Rc` type, which is a shared pointer wrapper type that uses non-atomic loads and writes to count. By turning this feature on, all usages of `Rc` across the crate are switched to `Arc`, which is atomically counted and thus implemented `Send` and `Sync` safely.

Additionaly, single-threaded Vulkayes replaces use of mutexes with simple wrapper types that emulate the mutex API, but do not implement `Send`/`Sync` and do not do any synchronization. This makes the API of both single-threaded and multi-threaded Vulkayes uniform. The main reason for this feature is performance, since atomic operations and synchronization is costly compared to non-atomic counterparts.

#### **runtime_implicit_validations**

Vulkayes aims to increase safety of Vulkan calls as much as possible without any performance impact. The idea is to always guarantee that the implicit validations defined in Vulkan spec are fulfilled and the explicit validations are only fulfilled when they can be easily derived from the existing API design.

This proved to not be always possible, so a small portion ([@tbl:validation-stats]) of implicit validations requires some runtime checking to ensure their fulfillment. These validation, producing runtime overhead, are conditionaly compiled using this feature to ensure that the user can always opt-out to achieve greater performance.

## Details

### Vrc, Vutex etc.

Talk about Vrc and Vutex aliases

![](implementation/generics.md)

### Deref

Talk about Deref trait usage

### User code a.k.a. `dyn FnMut`

Talk about usage of `dyn`

![](implementation/swapchain_recreate.md)