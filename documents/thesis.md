---
title: Implementation of rendering system in Rust
author: Eduard Lavuš
date: 2020-05-22
lang: en

bibliography:
	- assets/bibliography.json
csl: assets/ieee-with-url.csl
link-citations: true
reference-section-title: Bibliography

documentclass: book
fontsize: 12pt
mainfont: Charter
mathfont: Fira Code
papersize: a4
geometry: margin=2cm
urlcolor: blue
secnumdepth: 2
---

# Introduction

Since its release in 2016, Vulkan API[@VulkanAPI] has been gaining traction as a go-to API for high-performance realtime 3D applications across all platforms. The main reason for this, apart from being cross-platform, is that Vulkan is designed as to be low-level, close to metal and with minimal overhead. This, in contrast to Khornos' older API OpenGL, leaves most of the overhead, but also complexity, to the user of the API. The user can then make decisions on where to sacrifice performance for added usability or vice versa.

This project aims to design a flexible, usable and performant wrapper on top of Vulkan API in the Rust language. It aims to provide statically upholdable invariants that are easy to break in C language. It aims to add minimal required overhead to ensure basic memory safety that is the core concept of the Rust language. The name is a play on the Rust library the project is inspired by, the Vulka**no**[@Vulkano] library.

## Vulkan API

Vulkan API, originally released in 2016[@VulkanAPIRelease], is a specification of a an open API for high-efficiency, cross-platform access to graphics and compute on modern GPUs.

It is designed to minimize the overhead between the user application and the hardware device. Vulkan achieves this by staying low level a explicitly requiring all releavant state to be reference by the user application, minimizing required lookups and orchestration on the driver side. This allows the user application to optimize for their specific usecase instead of relying on the driver to guess the correct strategy.

One of the reasons for Vulkans popularity is that it was designed in an intense collaboration between leading hardware, game engine and platform vendors[@VulkanAPIRelease]. This resulted in a lot of vendors having zero-day support for the specification in their drivers and software and it being immediatelly adopted as a native rendering platform on many platforms.

The openness of Vulkan also goes hand-in-hand its cross-platform capabilities. Vulkan is available on all three major desktop platforms (Linux, macOS, Windows) and both major smartphone platforms (Android, iOS), but also on many smaller and embedded platforms. This allows applications to easily target multiple platforms with minimal variance in the rendering code. It also prevents vendor locks as seen with DirectX or Metal APIs. Lastly, it allows the community of both professionals and hobbyists to participate in the standard itself and improve it.

Khronos Group, the industry consortium responsible for Vulkan API, has been continuously improving the API and releasing updates. The API is currently on version 1.2[@VulkanAPIRelease12], which brough important updates that have been requested by the community. This proves that Vulkan aims to improve alongside the industry and provide support and improvements into the forseeable future.

![](related_work.md)

![](design/design.md)

![](implementation/implementation.md)

![](evaluation/evaluation.md)

# Conclusion

Conclude


Category |Statically solved|Dynamically solved|Left to user|Total
---------|-----------------|------------------|------------|-----
Implicit |              317|                28|           2|  347
Creation |               91|                 0|         314|  405
Usage    |               29|                 3|         122|  154
**Total**|              437|                30|         439|  906

Table: Vulkan API validations status in the project {#tbl:validation-stats}