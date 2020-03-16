# Validations

There are two types of validations in Vulkan API: Implicit validations, which talk about technical aspects of the API usage, and explicit validations, which talk about semantical aspects. Vulkayes aims to solve all implicit validations in the core crate. External validations are not always trivial to solve, some of them are statically fulfilled using the type system or the API design, others are left to the user.

External validations resolved statically are enclosed in blue boxes below.

## Implicit validations

![](validations/implicit_validations.md)

## Creation validation

Validations of correct usage in create functions as dictated by the Vulkan specification.

![](validations/creation_validations.md)

## Usage validations

Validations of correct unsage in other functions as dictated by the Vulkan specification.

![](validations/usage_validations.md)

## Statistics

Category |Statically solved|Dynamically solved|Left to user|Total
---------|-----------------|------------------|------------|-----
Implicit |                0|                 0|          60|   60
Creation |               22|                 0|         463|  485
Usage    |                0|                 0|          29|   29
**Total**|               22|                 0|         552|  574