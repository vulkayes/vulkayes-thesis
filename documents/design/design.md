# Design

Something and more

## Rust

Talk about why Rust was chosen, include cpp vs Rust examples

![](design/drop_order.md)

## Object lifetime management

![Object Dependency Graph](assets/diagrams/object_dependency_graph.svg)

Talk about how object lifetime is managed, maybe compare to tephra, talk about cargo features toggling

## Synchronization

Talk about how some objects are internally synchronized

Talk about how GPU synchronization is left for future work

## Validations

Talk about how only implicit validation are guaranteed, but some explicit validations are implicitly handled by api design and type system

## Memory management

Talk about how device memory management is done through user-supplied memory allocator

## Windows

Talk about how windows are handle, what is a surface and a swapchain and how they are supported
