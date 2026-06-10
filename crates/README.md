
## Crate
- smallest piece that the compiler can compile at a time (can be a single file with rustc)
- can be a lib or binary
- the root is the where the compiler starts looking
	* src/main.rs is for binary
	* src/lib.rs is for lib

## Package
- a bundle of one or more crates
- has a Cargo.toml file
- can have multiple binary crates but only one lib
- must contain at least one crate

## Modules
* a way to group things
* in the crate root you need to declare the modules using `mod name;`
	* the compiler will look for the modules in: the current file, the file `src/name.rs` and in `src/name/mod.rs`
		* it follows the same idea for submodules
		* you don't need to specify the submodules in the root
* by default every module is private, but you can set it to public with `pub`
* to use the functions of a module we can call it by the absolute path as `crate::name::...` or the relative path as `name::...`.
* for libs the tree of calls should be defined at src/lib.rs
* by using `super::` we can reference functions and data that's defined in the parent module
* for structs you need to set itself as `pub` and all its methods and data are still private, so you need to set one by one
	* enums on the other hand you only need to set themselves as public
* to bring modules into the scope you can use the keyword `use` followed by the path
	* is like a shortcut
	* you should usually bring the parent module instead of a single function (is more idiomatic)
	* we can give aliases to avoid collisions using the keyword `as` in the `use` clause
* we can re-export modules by importing it into a file but using `pub use`
	* now instead of calling the whole path, you only need to import from the re-exported file
* when importing you can used nested imports by defining collections like this: `use std::{cmp::Ordering, io};`
	* when you want to bring into scope the part itself you use `self` like in `use std::io::{self, Write};`
	* if you want to bring all you can use `*` as well `use std::collections::*;`