## Cargo features {#sec:impl_cargo}

An important part of any flexible project is to give the user as much control as possible, so the library will fit their usecase. One way to achieve this in Rust are cargo features already mentioned in [@sec:design_cargo].

The most important features defined and exposed in Vulkayes are described below.

#### **naive_device_allocator**

This feature conditionally compiles a very naive device memory allocator into the project. Device memory allocation is a complex topic and applications are required to provide their own allocators to fit their own needs. One popular allocator is the Vulkan Memory Allocator&nbsp;[@VMA], but it is a big dependency that might not be easily accessible for certain usecases. Vulkayes supports integration with VMA (and other allocators) seamlessly, but also provides the naive allocator as a simple no-dependecy alterative for quick prototyping and debugging.

#### **multi_thread** {#sec:multi_thread_feature}

One of the biggest selling points of Vulkan are its multi-threading capabilities. Since the user is in charge of synchronizing the resources, they can design their application to fit their needs. Single-threaded applications require no synchronization, while multi-threaded applications should allow for the full power of multi-threading to be leveraged.

Safe Rust statically prevents data races using the built-in `Send` and `Sync` traits. These traits are automatically implemented (or not implemented) by the compiler to mark types as "capable of being sent between threads" and "capable of being borrowed across threads" respectively. The user is free to unsafely implement these traits back if the compiler decides to not implement them, provided that the user takes the burden of preventing data races upon themselves.

By default, object wrappers in Vulkayes are not `Send` nor `Sync`, simply because they use the `Rc` type, which is a shared pointer wrapper type that uses non-atomic loads and writes to count. By turning this feature on, all usages of `Rc` across the crate are switched to `Arc`, which is atomically counted and thus implementes `Send` and `Sync` safely.

Additionaly, single-threaded Vulkayes replaces the use of mutexes with simple wrapper types that emulate the mutex API, but do not implement `Send`/`Sync` and do not do any synchronization. This makes the API of both single-threaded and multi-threaded Vulkayes uniform. The main reason for this feature is performance, since atomic operations and synchronization is costly compared to non-atomic counterparts.

#### **runtime_implicit_validations**

Vulkayes aims to increase safety of Vulkan calls as much as possible without any performance impact. The idea is to always guarantee that the implicit validations defined in Vulkan spec are fulfilled and the explicit validations are only fulfilled when they can be easily derived from the existing API design.

This proved to not be always possible, so a small portion of implicit validations requires some runtime checking to ensure their fulfillment. These validation, producing runtime overhead, are conditionaly compiled using this feature to ensure that the user can always opt-out to achieve greater performance.