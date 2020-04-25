# Validations

There are two types of validations in Vulkan API: Implicit validations, which talk about technical aspects of the API usage, and explicit validations, which talk about semantical aspects. Vulkayes aims to solve all implicit validations in the core crate. External validations are not always trivial to solve, some of them are statically fulfilled using the type system or the API design, others are left to the user.

External validations resolved statically are enclosed in blue boxes below. Validations optionally checked at runtime are in green boxes.

## Implicit validations

![](progress/validations/implicit_validations.md)

## Creation validation

Validations of correct usage in create functions as dictated by the Vulkan specification.

![](progress/validations/creation_validations.md)

## Usage validations

Validations of correct unsage in other functions as dictated by the Vulkan specification.

![](progress/validations/usage_validations.md)

## Statistics

Category |Statically solved|Dynamically solved|Left to user|Total
---------|-----------------|------------------|------------|-----
Implicit |              306|                27|           3|  336
Creation |               90|                 0|         307|  397
Usage    |               29|                 3|         116|  148
**Total**|              425|                30|         426|  881