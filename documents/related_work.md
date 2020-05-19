# Related work

There are already many libraries aiming to provide similar abstraction over Vulkan. Some of the most prominent and closest to this work are mentioned below.

## V-EZ

"V-EZ is an open source, cross-platform (Windows and Linux) wrapper intended to alleviate the inherent complexity and application responsibility of using the Vulkan API. V-EZ attempts to bridge the gap between traditional graphics APIs and Vulkan by providing similar semantics to Vulkan while lowering the barrier to entry and providing an easier to use API."&nbsp;[@V-EZ] An overview can be seen in in [@fig:vez_api_overview].

This ease of use does come at a price, however. The design of V-EZ leaves no room for the user to properly express their intent at critical points of execution. This leads to unnecessary slowdows and hashmap lookups which outweight most of the benefits gained by simplified API.

The last commit to V-EZ was on 2018-10-05&nbsp;[@V-EZ].

![
    V-EZ greatly reduces the number of objects the user has to care about. Everything else is taken care of behind the scenes&nbsp;[@V-EZ].
](assets/images/V-EZ.png){#fig:vez_api_overview}

## gfx-hal

gfx-hal or graphics hardware abstraction layer&nbsp;[@gfx-hal] is a project aimed at abstracting graphics computations not only from hardware, but also from low-level APIs like Vulkan or OpenGL. It is, in a sense, lower level than Vulkayes aims to be. The abstraction over multiple APIs, while very useful for most common usages, can hurt usability in niche cases where a specific extension or feature is only available in one API.

In contrast, Vulkayes aims to provide a _transparent_ abstraction over Vulkan API. This allows users to use any features available to them by the API even if the abstraction doesn't implement it directly.

![
    gfx-hal creates an abstraction layer over all mainstream graphics APIs&nbsp;[@gfx-hal].
](assets/images/hal.svg)

## Vulkano

Vulkano&nbsp;[@Vulkano] aims to provide complete validation and synchronization guarantees for the user. This proved to be too limiting and the original developer eventually left the project. Since then, not much work has been done.

Vulkayes originally started as a fork of Vulkano, however, over time, it grew into a rewrite because of many questionale design choices taken in Vulkano. Vulkano makes havy use of dynamic dispatch, which impacts performance. Its API also promises thorough validation checks, however at the expense of API flexibility, which makes it less likely to be widely adopted. For example, it is still impossible to upload mipmaps to Vulkanos `ImmutableImage` (which is intended as one-time write image abstraction, e.g. for textures in games).

![
    Vulkano logo&nbsp;[@Vulkano].
](assets/images/vulkano.png)

## Tephra

Tephra&nbsp;[@Tephra] is a very recent work with very similar aims to Vulkayes. It can the though of as a C++ version of Vulkayes. It takes a fresh look at the existing solutions and comes up with a transparent and flexible API for handling Vulkan.

However, many of the design considerations taken in Tephra revolve around safety and sanity of C++ language itself. This is of questionable importance and puts unnecessary strain on the library designer. Overall, most of the well designed concepts in Tephra have to be weighted againts the complexity of the language.

![
    Screenshot of one of the benchmarks for Tephra&nbsp;[@Tephra].
](assets/images/tephra.png)

## Summary

Library |Status   |Language|Goals
--------|---------|--------|----------------------------------------------
V-EZ    |Abandoned| C++    | Usability over performance
gfx-hal |Active   | Rust   | Hardware abstractions
Vulkano |Abandoned| Rust   | Safety over performance
Tephra  |Unknown  | C++    | Performance and usability
        |         |        | 
Vulkayes|Active   | Rust   | Performance, usability and increased safety

Table: _Related work summary_ {#tbl:related_work_summary}

In summary, many projects aiming at simplifying the Vulkan API have been either abandoned or are too broad in scope to consider them production-ready ([@tbl:related_work_summary]). In the end, Tephra comes out as the closest and most practically usable work. However, the C++ language is itself a complex and hard to master system that places many requirements on the user of the library.

In contrast, Rust, and by extension Vulkayes, aims to offload as much off the user as possible without unnecessary and performance-reducing restrictions.