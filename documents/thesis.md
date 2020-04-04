---
title: Implementation of rendering system in Rust
---

# Introduction

Talk about Vulkan API and why it is great

Talk about what the project aims to achieve in short and long term, mention Rust

# Related work

Talk about V-EZ, Vulkano, gfx-hal, mention tephra

# Design

## Rust

Talk about why Rust was chosen, include cpp vs Rust examples

## Object lifetime management

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

# Implementation

## Cargo features

Talk about what cargo features are available and why they are beneficial

### Vrc, Vutex etc.

Talk about Vrc and Vutex aliases


## Generics

Talk about how Rust generics are used and optimized for comfort with minimal overhead

### Deref

Talk about Deref trait usage

### User code a.k.a. `dyn FnMut`

Talk about usage of `dyn`


# Benchmarks

Talk about benchmarks

## Scene 1

## Scene 2

## Scene 3


# Conclusion

Conclude

