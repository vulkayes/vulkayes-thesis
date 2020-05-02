## Safety

One of the main goals of Vulkayes is increasing safety. This mainly includes memory safety. Vulkayes, being a safe wrapper, provides safe abstraction in the types is wraps in both the Rust way and the Vulkan API way.

Category |Statically solved|Dynamically solved|Left to user|Total
---------|-----------------|------------------|------------|-----
Implicit |              317|                28|           2|  347
Creation |               91|                 0|         314|  405
Usage    |               29|                 3|         122|  154
**Total**|              437|                30|         439|  906

Table: _Vulkan API validations status in the project._ {#tbl:validation-stats}

In [@tbl:validation-stats] it can be seen that the goal was achieved almost perfectly. Only two implicit validations are left to the user. This decision wasn't made lightly, but it was chosen as the most sensible one given the current limitations of the stable version of language. A small number of implicit validations couldn't be solved statically. These validations are instead checked at runtime, but only conditionally under the `runtime_implicit_validations` Cargo feature. All other implicit validations were successfully solved statically. More details about the specific validations can be found in the appendix.

Additionally, a significant amount of explicit validations, categorized under creation and usage, have been solved statically as a consequence of the natural API design and/or the implicit validations. Overall this means increased safety for the user of the API at no runtime cost. 