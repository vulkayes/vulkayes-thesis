# Design

The API was designed to fullfil three goals:

1. Be transparent - The API must allow falling back to pure Vulkan if a certain feature is not supported or implemented in the API.
2. Be fast - The API must carefully manage abstraction costs and minimize overhead.
3. Be flexible - The API must be easy to use in different contexts. It must not force the user to unreasonably change their code to fit the API.

## Rust

Where performance is critical, programmers often fall back to the "classical" languages such as C and C++. These languages, however, are often burdened by legacy, backwards compatiblity and outdated design concepts.

C is a very simple and fast language. However, programming industry has changed quite a lot since its first appearance 48 years ago[@clang]. Concepts common at the time in programming, such as easy low-level memory access and easy mapping to machine instruction, are hardly transferrable to todays high-level requirements of programming.

C++ attempted to extend C with a useful standard library of data types, algorithms and other features. This made C++ a much better candidate at creating complex performance-critical applications. However, stemming from C, it still caries the burden of past decisions. Writing sound code often requires the programmer to be _more_ expressive and pay more attention to intricacies of the language. This comes at an expense in code quality, readability and sometimes programmer sanity.

The Rust programming language became a natural choice for this project because goals 2. and 3. are already core concepts of the language itself. Unlike C, it has extensive standard library and was designed for high-level programming. Unlike C++, higher code safety requires _less_ work from the programmer. That is, safety is enforced by the language features in form of statical analysis.

### Ownership 

Rust implements a very simple but powerful ownership model. You cannot prevent the compiler from moving your value. However, the language is smart about this. Moving a value does not just create a bitwise copy, it also moves the ownership which has serious consequences: the owner has to clean up. Values that have non-trivial destructors should run those destructors at some point. Indeed, the memory move semantics in Rust are simply an implementation detail of its higher-level abstraction of movement of ownership.

In C++ the only difference between a copy and a move is that the new value has a chance to take apart the old value. For example, for heap allocated types, this means the new value will take the heap memory (pointer) from the old value. The destructor, however, is still run for both the values, as it if was simply copied. Ownership in C++ is only conceptual, the language itself does not understand it nor enforce it. This moves the burden of reasoning on the the user.

In contrast, Rust statically prevents use of moved-out variables. Once you move a value out of a variable (moving the ownership somewhere else), that variable now acts as if it was uninitialized, it cannot be used anymore and its destructor is not called. The destructor is only called for the "new" value once it goes out of scope, it is the responsibility of the new owner to clean up. Moreover, this move is often optimizable by the compiler and thus is almost or entirely free.

#### Borrow checker

The Rust borrow checker tracks borrowed values. A value is borrowed when a reference to it is created. A reference can either be immutable or mutable. There can only ever be one mutable reference and it cannot coexist with any immutable references. This completely prevents all read-write race conditions _statically_.

Borrow checking also prevents problems such as use-after-free or iterator invalidation. These problems can be considered single-thread race conditions. A reference is created, then the original referred value is destroyed or moved and then the reference is used (to read or write). Such a reference is called dangling. Rust statically prevents the existence of dangling references. When a value is borrowed, it must outlive any references taken from it. That is, an owner can lend the value to someone, but it must then keep the value in its place for as long as the borrow is valid, it cannot be moved to a new location nor dropped. This is done using lifetimes.

#### Lifetimes

Lifetimes are how Rust tracks borrows. Each borrow (a reference) has a lifetime associated with it. The borrow cannot be used for longer than that. For example, if a value is created in a certain scope then a reference to it cannot escape that scope since it could lead to use-after-free. Additionally, programmers can use these lifetimes too, as generic arguments, to express concepts like borrowing subfields or narrowing array views.

There is one lifetime that is always available, the `'static` lifetime. This lifetime is special in that it expresses the concept of _always valid_. References with this lifetime can be used anywhere in the program, at any time, because they are known to always outlive the program itself. For example, taking a reference to static data (data compiled into the binary executable) creates a static reference that can be then freely used inside the application.

### Safety and speed

Of course, some of the lowest-level code cannot be created in this somewhat restricted environment. The abstraction has to be built somehow. This is where **unsafe** Rust comes in. Instead of specifying additional safety features, Rust programmers have to explicitly ask to disable existing features. Code blocks marked `unsafe` are free to work with dangling pointers, have data races or cause other unsoundness, just like C++ normally does.

The implementation of the Rust standard library has empirically proven that the system truly only needs unsafe blocks few and far between. Indeed, only the most basic building blocks have to rely on unsafe operations, while all the other parts can just rely on the soundness of these simple code snippets that can easily be checked and verified over and over by quanta of programmers to ensure they truly are sound. This safety system reduces possible failures to a few narrow blocks of code, instead of leaving the programmer with having to find the bug in all of their code.

All of this is done at compile time and thus has no runtime cost. All code is as fast as the same C++ code would be, but safe.

### Cargo {#sec:design_cargo}

Cargo[@Cargo] is Rusts package manager. It takes care of indexing and retrieving dependencies, compiling them and publishing libraries and binaries to the registry. Cargo also takes care of project configuration. In C/C++ codebases it is common to either invoke the compiler directly, or to use build tools such as `make` or `CMake`. Cargo is similar to those build tools, but it is a component of Rust ecosystem and is targeted at Rust only.

Being a part of Rust itself, cargo is able to provide lots of useful abstraction over the rust compiler. The configuration file `Cargo.toml` is filled with useful project information such as the project name, author and short description. The file also contains technical information, like the targeted language edition, compiler and optimization flags, all of the dependencies (and how/where to look for them) and project features. Platform-specific configuration is also possible.

Features defined in `Cargo.toml` are project-unique strings. These strings can then be used from within the codebase to conditionally compile part of the code, similar to C preprocessor `#ifdef` statements. Contrary to the C preprocessor, however, these strings are defined in one central place and can even define dependency chains, so that certain features might require other features or additional dependencies. This is often used when developing on top of platform-dependent code to provide uniform interface to the user. It is also used in Vulkayes, as mentioned in [@sec:impl_cargo].

### Generics

Generics are a very powerful tool in programming. They help avoid a common problem in libraries: "What if my object doesn't cover all usecases". Generics provide a way for the library user to specify their own object with their own implementation and it only has to conform to some predefined bounds. In Rust, this is done by specifying trait bounds:

```rust
trait BoundTrait {
	fn required_method(&self) -> u32;
}

fn generic_function<P: BoundTrait>(generic_parameter: P) -> u32 {
	generic_parameter.required_method()
}
```

In this code snippet, the `P` parameter of the `generic_function` is generic. The user can then do this:

```rust
struct Foo;
impl BoundTrait for Foo {
	fn required_method(&self) -> u32 {
		0
	}
}

struct Bar(u32);
impl BoundTrait for Foo {
	fn required_method(&self) -> u32 {
		self.0
	}
}
```

Now both the `Foo` struct and the `Bar` implement the trait `BoundTrait` and can be used to call `generic_function`:

```rust
let foo = Foo;
generic_function(foo);

let bar = Bar(1);
generic_function(bar);
```

This usage is zero-cost because the functions are monomophised at the compile time for each calling type.

#### Storing generic parameters

Using generic parameters is one thing, but storing them is harder. Generic parameters can have different sizes that are not known at the definition time:

```rust
struct Holder<B: BoundTrait> {
	item: B
}

let a = Holder { item: Foo };
let b = Holder { item: Bar(1) };
```

In this snippet, it is unknown at the definition time how big the `Holder` struct will be in memory. Instead, it is decided at the use time. That is, the variable `a` possibly takes less space on the stack than the variable `b`. The size of a type is a function of its fields, if the field is generic, it can't be known up front.

Generic parameters are a part of the type. Two `Holder`s with different generic parameters cannot be stored together in an uniform collection (like `Vec`). The only way to achieve that is by using dynamic dispatch.

#### Dynamic generics

Dynamically dispatched generics can be used to mix and match different implementations of traits in the same place. It works by taking a pointer to the generic parameter and then "forgetting" the type of that parameter, only remembering the bounds. In Rust, this is handled by trait objects in the form of `dyn BoundTrait`. This is an unsized (size isn't known at compile time) type and it cannot be stored directly on the stack or in uniform collections either. It needs to be behind some kind of pointer, whether it be a reference, `Box`, `Rc/Arc` or a raw pointer. This pointer will be a so-called "fat" pointer.

For example, to store any kind of `BoundTrait` implementor in a `Vec`, it can be written like this:

```rust
let a = Foo;
let b = Bar(1);

let vec: Vec<Box<dyn BoundTrait>> = vec![
	Box::new(a) as Box<dyn BoundTrait>,
	Box::new(b) as Box<dyn BoundTrait>
];
```

The downside of this is both the allocation of heap memory and the access speed. Accessing methods on the object has to go through one more level of indirection than normally and also prevents certain powerful compiler optimizations. Thus is it undesirable to use dynamic dispatch when it is not necessary.

## Object lifetime management

![Object Dependency Graph of Vulkayes](assets/diagrams/object_dependency_graph.svg){#fig:object_dependnency_graph}

Objects in Vulkan have certain lifetime dependencies - some objects must outlive others - displayed in [@fig:object_dependnency_graph]. Some dependencies are simpler and always apply, others are more complex and conditional. Most of these dependencies in Vulkayes are handled using reference counting. Reference counting is a programming concept where data is shared among multiple actors using some kind of reference (pointer). The pointed-to memory, apart from storing the data object itself, also stores a count of existing references to that memory. This provides an easy way to clean up resources when they are no longer used, all automatically at runtime, with overhead only during the creation and destruction of the resource itself, not during usage. The Rust safety system also prevents the pointed-to memory to be freed or otherwise deinitialized, ensuring safety.

Rust also differentiates between normal reference counted value and atomically counted reference counted value. The former is called `Rc` while the latter is called `Arc`. This is used in Vulkayes, as described in more detail in the [@sec:v_aliases].

## Memory management

There are two types of memory in Vulkan. Host memory - the memory accesible only to the CPU, and device memory - the memory accesible to the device. Device memory might be host-mappable, meaning it can be accessed by the CPU if it is explicitely mapped, similar to the C `mmap` function.

Host memory in Vulkan is managed by the implementation, but Vulkan exposes a way to intercept this process by allowing the application to provide its own allocation callbacks. These callbacks are called whenever the Vulkan implementation wishes to allocate, reallocate or free memory and can be used to handle allocation in a custom manner. Vulkan specification recommends using these callbacks only for debugging purposes or in specific cases, not in general, as they would not impact the performance in any meaningful way.

Device memory, however, is a bigger topic in Vulkan. Applications are expected to allocate and manage memory themselves. Vulkan only recommends that allocation should happen in 128 to 256 MB chunks at a time to reduce the overhead. This means Vulkayes needs to provide its own way to integrate user-defined device memory allocation, as described in more detail in the [@sec:device_memory_allocator_generics].

## Synchronization and validations

Vulkan leaves almost all CPU synchronization to the user. Explicit synchronization requirements are described in the specification and Vulkan functions are not reentrant. The user application has to take care of all the synchronization requirements as to not cause a data race. Vulkayes solves this in two ways. When used normally, no synchronization is done and everything is as performant as it can be. Secondly, it provides a multi-thread feature ([@sec:multi_thread_feature]) where mutexes are used and proper synchronization is ensured.

Validations in Vulkan are generalization of synchronization requirements. Validations specify not only how to prevent data races, but also how to prevent other undefined behaviors. Vulkan validation requirements tend to be very long, dense and hard to parse, leading to an increased chance of breaking them. Vulkayes aims to alleviate this somewhat by guaranteeing at least the most common and statically solvable validations to be fulfilled.

Last topic of synchronization is GPU synchronization. This encompasses synchronization of resource usage in command buffers executed on the GPU queues as well as the synchronization between CPU and GPU. This kind of synchronization is very important, but it is a complex topic on its own and is left to be added to Vulkayes as a separate project.
