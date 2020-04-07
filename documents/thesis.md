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
mainfont: Fira Sans
mathfont: Fira Code
geometry: margin=2cm
urlcolor: blue

output:
	pdf_document:
		toc: true
		number_sections: true
---

# Introduction

Since its release in 2016, Vulkan API[@VulkanAPI] has been gaining traction as a go-to API for high-performance realtime 3D applications across all platforms. The main reason for this, apart from being cross-platform, is that Vulkan is designed as to be low-level, close to metal and with minimal overhead. This, in contrast to Khornos' older API OpenGL, leaves most of the overhead, but also complexity, to the user of the API. The user can then make decisions on where to sacrifice performance for added usability or vice versa.

Talk about Vulkan API and why it is great

Talk about what the project aims to achieve in short and long term, mention Rust

![](related_work.md)

![](design/design.md)

![](implementation/implementation.md)

![](evaluation/evaluation.md)

# Conclusion

Conclude

