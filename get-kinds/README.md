# get-kinds
A procedural macro and trait that returns the name of the variant

## How it works
The macro generates a function called `kind` for each enum it is applied on.
The function will inherit the visibility of the enum.
See the documentation for the `kind` function to see its usage.

## Examples
See [docs.rs](https://docs.rs/generate-kinds/latest/generate_kinds/)