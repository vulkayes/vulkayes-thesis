---
title: Implementation of rendering system in Rust
author: Eduard Lavuš
date: 2020-05-22
lang: en

bibliography:
	- assets/bibliography.json
csl: assets/ieee-with-url.csl
link-citations: true
# reference-section-title: Bibliography

documentclass: book
papersize: a4
geometry: margin=2cm
fontsize: 12pt
mainfont: Charter
# mainfont: Arial
mathfont: Fira Code

urlcolor: blue
secnumdepth: 2
---

# Introduction

Since its release in 2016, Vulkan API[@VulkanAPI] has been gaining traction as a go-to API for high-performance realtime 3D applications across all platforms. The main reason for this, apart from being cross-platform, is that Vulkan is designed as to be low-level, close to metal and with minimal overhead. This, in contrast to Khornos' older API OpenGL, leaves most of the overhead, but also complexity, to the user of the API. The user can then make decisions on where to sacrifice performance for added usability or vice versa.

This project aims to design a flexible, usable and performant wrapper on top of Vulkan API in the Rust language. It aims to provide statically upholdable invariants that are easy to break in C language. It aims to add minimal required overhead to ensure basic memory safety that is the core concept of the Rust language. The name is a play on the Rust library the project is inspired by, the Vulka**no**[@Vulkano] library.

![](vulkan_introduction.md)

![](related_work.md)

![](design/design.md)

![](implementation/implementation.md)

![](evaluation/evaluation.md)

# Conclusion

The core Vulkayes library is successful at reducing the complexity of creating and using Vulkan types, as well as correctly destroying them at appropriate times and checking basic safety requirements. Benchmarks show that this added complexity is mostly compile-time and scales well into the runtime where applicable. Additionaly, safety is guaranteed at a certain level that should provide the user of the API with certain amount of confidence that their application will not segfault. Overall, the Vulkayes project is a good step towards a flexible and transparent Vulkan API in the Rust ecosystem, learning from previous mistakes and designs.

However, there still remains a lot of work to be done to create an API with a application design advantage as well. Designing synchronization in Vulkan by hand is error prone due to high complexity and Vulkayes should be extended with user-friendly API that is capable of lifting the burden off the user onto the implementation, prefferably mostly at compile time. Declarative synchronization definition API and other improvements to Vulkayes are left for future work.

![](backmatter.md)