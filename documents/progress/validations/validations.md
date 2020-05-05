# Validations

There are two types of validations in Vulkan API: Implicit validations, which talk about technical aspects of the API usage, and explicit validations, which talk about semantical aspects. Vulkayes aims to solve all implicit validations in the core crate. External validations are not always trivial to solve, some of them are statically fulfilled using the type system or the API design, others are left to the user.

External validations resolved statically are enclosed in blue boxes below. Validations optionally checked at runtime are in green boxes.

## Implicit validations

![](implicit_validations.md)

## Creation validation

Validations of correct usage in create functions as dictated by the Vulkan specification.

![](creation_validations.md)

## Usage validations

Validations of correct unsage in other functions as dictated by the Vulkan specification.

![](usage_validations.md)

## Statistics

Category |Statically solved|Dynamically solved|Left to user|Total
---------|-----------------|------------------|------------|-----
Implicit |              317|                27|           3|  347
Creation |               91|                 0|         314|  405
Usage    |               29|                 3|         122|  154
**Total**|              437|                30|         439|  906