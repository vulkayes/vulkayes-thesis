# Design

The API was designed to fullfil three goals:

1. Be transparent - The API must allow falling back to pure Vulkan if a certain feature is not supported or implemented in the API.
2. Be fast - The API must carefully manage abstraction costs and minimize overhead.
3. Be flexible - The API must be easy to use in different contexts. It must not force the user unreasonably to change their code to fit the API.

## Rust

Where performance is critical, programmers often fall back to the "classical" languages such as C and C++. These languages, however, are often burdened by legacy, backwards compatiblity and outdated design concepts.

C is a very simple and fast language. However, programming industry has changed quite a lot since its first appearance 48 years ago[@clang]. Concepts common at the time in programming, such as easy low-level memory access and easy mapping to machine instruction, are hardly transferrable to todays high-level requirements of programming.

C++ attempted to extend C with a useful standard library of data types, algorithms and other features. This made C++ a much better candidate at creating complex performance-critical applications. However, stemming from C, it still caries the burden of past decisions. Writing sound code often requires the programmer to be _more_ expressive and pay more attention to intricacies of the language. This comes at an expense in code quality, readability and sometimes programmer sanity.

The Rust programming language became a natural choice for this project because goals 2. and 3. are already core concepts of the language itself. Unlike C, it has extensive standard library and was designed for high-level programming. Unlike C++, higher code safety requires _less_ work from the programmer. That is, safety is enforced by the language features in form of statical analysis:

#### Ownership 

Rust implements a very simple but powerful ownership model. Values are always moveable. You cannot prevent the compiler from moving your value. However, the language is smart about this. Moving a value does not just create a bitwise copy, it also moves the ownership. Ownership has serious consequences: the owner has to clean up. Values that have non-trivial destructors should run those destructors at some point.

In C++ the only difference between a copy and a move is that the new value has a chance to take apart the old value. For example, for heap allocated types, this means the new value will take the heap memory (pointer) from the old value. The destructor, however, is still run for both the values, as it if was simply copied.

In contrast, Rust statically prevents use of moved-out variables. Once you move a value out of a variable, that variable now acts as if it was uninitialized, it cannot be used anymore and its destructor is not called. The destructor is only called for the "new" value once it goes out of scope. Moreover, this move is often optimizable by the compiler and thus is almost or entirely free.

#### Borrow checker

The Rust borrow checker tracks borrowed values. A value is borrowed when a reference to it is created. A reference can either be immutable or mutable. There can only ever be one mutable reference and it also cannot coexist with any immutable references. This completely prevents all read-write race conditions _statically_.

Borrow checking also prevents problems such as use-after-free or iterator invalidation. These problems can be considered single-thread rade conditions. A reference is created, then the original referred value is destroyed and then the reference is used (to read or write). Such a reference is called dangling. Rust statically prevents the existence of dangling references. When a value is borrowed, it must outlive any references taked from it. This is done using lifetimes.

#### Lifetimes

Lifetimes are how Rust tracks borrows. Each borrow (a reference) has a lifetime associated with it. The borrow cannot be used for longer than that. For example, if a value is created in a certain scope then a reference to it cannot escape that scope since it could lead to use after free. Additionally, programmers can use these lifetimes too, as generic arguments, to express concepts like borrowing subfields or narrowing array views.

### Safety and speed

Of course, some of the lowest-level code cannot be created in this somewhat restricted environment. The abstraction has to be built somehow. This is where **unsafe** Rust comes in. Instead of specifying additional safety features, Rust programmers have to explicitly ask to disable existing features. Code blocks marked `unsafe` are free to work with dangling pointers, have data races or cause other unsoundness, just like C++ normally does.

The implementation of the Rust standard library has empirically proven that the system truly only needs unsafe blocks few and far between. Indeed, only the most basic building blocks have to rely on unsafe operations, while all the other parts can just rely on the soundness of these simple code snippets that can easily be checked and verified over and over by quanta of programmers to ensure they truly are sound. This safety system reduces possible failures to a few narrow blocks of code, instead of leaving the programmer with having to find the bug in all of their code.

All of this is done at compile time and thus has no runtime cost. All code is as fast as the same C++ code would be, but safe.

## Object lifetime management

![Object Dependency Graph](assets/diagrams/object_dependency_graph.svg)

Objects in Vulkan have certain lifetime dependencies - some objects must outlive others - displayed in the diagram[TODO Figure number thing]. Some dependencies are simpler and always apply, others are more complex and conditional. Most of these dependencies in Vulkayes are handled using reference counting. Reference counting is a programming concept where data is shared among multiple actors using some kind of reference (pointer). The pointed-to memory, apart from storing the data object itself, also stores a count of existing references to that memory. This provides an easy way to clean up resources when they are no longer used, all automatically at runtime, with overhead only during the creation and destruction of the resource itself, not during usage. The Rust safety system also prevents the pointed-to memory to be freed or otherwise deinitialized, ensuring safety.

## Memory management

Talk about how device memory management is done through user-supplied memory allocator

## Synchronization

Talk about how some objects are internally synchronized

Talk about how GPU synchronization is left for future work

## Validations

Talk about how only implicit validation are guaranteed, but some explicit validations are implicitly handled by api design and type system

## Windows

Talk about how windows are handle, what is a surface and a swapchain and how they are supported
