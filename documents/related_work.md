# Related work

There are already many libraries aiming to provide similar abstraction over Vulkan. Some of the most prominent and closest to this work are mentioned below.

## V-EZ

"V-EZ is an open source, cross-platform (Windows and Linux) wrapper intended to alleviate the inherent complexity and application responsibility of using the Vulkan API. V-EZ attempts to bridge the gap between traditional graphics APIs and Vulkan by providing similar semantics to Vulkan while lowering the barrier to entry and providing an easier to use API."[@V-EZ]

This ease of use does come at a price, however. The design of V-EZ leaves no room for the user to properly express their intent at critical points of execution. This leads to unnecessary slowdows and hashmap lookups which outweight most of the benefits gained by simplified API.

Last commit to V-EZ was on 2018-10-05[@V-EZ].

## gfx-hal

gfx-hal or graphics hardware abstraction layer[@gfx-hal] is a project aimed at abstracting graphics computations not only from hardware, but also from low-level APIs like Vulkan or OpenGL. It is, in a sense, lower level than Vulkayes aims to be. The abstraction over multiple APIs, while very useful for most common usages, can hurt usability in niche cases where a specific extension or feature is only available in one API.

In contrast, Vulkayes aims to provide a _transparent_ abstraction over Vulkan API. This allows users to use any features available to them by the API even if the abstraction doesn't implement it directly.

## Vulkano

Vulkano[@Vulkano] aims to provide complete validation and synchronization guarantees for the user. This proved to be too limiting and the original developer eventually left the project. Since then, not much work has been done.

Vulkayes originally started as a fork of Vulkano, however, over time, it grew into a rewrite because of many questionale design choices taken in Vulkano. Vulkano makes havy use of dynamic dispatch, which impacts performance. Its API also promises thorough validation checks, however at the expense of API flexibility, which makes it less likely to be widely adopted. For example, it is still impossible to upload mipmaps to Vulkano's `ImmutableImage` (which is intended as one-time write image abstraction, e.g. for textures in games).

## Tephra

TODO: How to correctly cite this?

Tephra is a very recent work with very similar aims to Vulkayes. It can the though of as a C++ version of Vulkayes. It takes a fresh look at the existing solutions and comes up with a transparent and flexible API for handling Vulkan.

However, many of the design considerations taken in Tephra revolve around safety and sanity of C++ language itself. This is of questionable importance and puts unnecessary strain on the library designer. Overall, most of the well designed concepts in Tephra have to be weighted againts the unfriendliness of the language.