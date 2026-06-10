* rust allow us to define the fields of a struct without repeating the name if the variable/parameter has the same name as the field (shorthand)
* we can use the update syntax to fill the remaining fields of a new struct by getting the values of another one: for that we use `..struct_var` 
	* it moves the data into the new one, so heap data may not work correctly this way
* Tuple structs are a way to use tuples but giving a name in a struct like definition `Point(i32, i32, i32)`
	* you can destructure them as well like `let Point(x,y,z) = origin`
* structs that don't have fields are called unit-like
	* similar to `()`
	* useful to implement a trait when you don't want to have data in the type
	* done by: `struct name;`
* to store references you need to set lifetimes
* to add methods to it you can create an `impl` block for the struct
	* every method receives a first parameter `&self`
	* `&self` is the short for `self:&Self`
	* self is an alias for the type we are implementing
	* self can be borrowed, take the ownership or even borrow mutably as any other parameter
	* we could use `&mut self` as well
* we don't need to do `s1.method()`, or `(&s1).method()` because rust can figure out by looking at the type of `self` in the method definition
* methods are called associated functions
	* they can also be defined without the self as first parameter
		* these are generators which return self as in:
			```rust
			impl Rectangle { 
				fn square(size: u32) -> Self { 
					Self { width: size, height: size, } 
				} 
			}
			```
			to call we do
			```rust
			Rectangle::square(3);
			```
* can have multiple `impl` blocks
			