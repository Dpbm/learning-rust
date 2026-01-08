- References (&, &mut) are kinda like pointers but just point to some data and there's no overhead
- smart pointers own the data they point to
- they implement the Deref and Drop traits
    - Deref --> permit it to actually like a pointer and reference at the same time (deference operator \*)
    - Drop --> permit you to customize the behavior when a pointer goes out of scope
- deref coersion --> convert a reference type into another ( the first must implement Deref ) that returns that type a function needs (for example &String to &str, since String implements the Deref and returns &str when derefed, so rust understands it rightaway). It will deref iteratively down the elements tree of your pointer till it matches a type that's compatible with that one required by the function.
- for mutable pointers, you have DerefMut. Rust will do the same as deref.

- Coersion patterns:
    - immut to immut
    - mut to mut
    - mut to immut

- rust drops variables in the reverse order
- to drop something manually you need to use `std::mem::drop`.
- rust doesn't allow you to call `.drop()` because rust will still run the drop when the pointer is out of scope, so it would result in a doblue free error


## Box

- store data on heap and the pointer on the stack
- don't have much overhead (but not many capabilities)
- use when:
    - you don't know the size of something in compile time
    - transfer the ownership of a large amount of data without copying it
    - you want to own a value and only care about which trait it implements
- when it goes out of scope it is deallocated (both and its data)
- Box have known size at compile time, so it's usefull when you need to do something that's dynamically (rust only accepets parts that it can know the size in compile time)
- If any borrowing rule is broken, the program won't compile
- both immutable and mutable

## Rc

- multiple ownership.
- own its data.
- RC == Reference Counting.
- cleans when the amount of references is zero.
- for single threaded applications.
- Useful when we want to use some data throughout our application but we don't know which part will use it last.
- We use ::clone to add another owner for some data (increase the reference counting). It doesn't perform a deep copy.
- immutable only.
- You can use `Rc::downgrade` to add a weak reference. Weak reference won't affect when the pointer will be free nor the ownership counting. Weak references also don't cause reference cycles, since it will be broken when strong referecens reach zero.
- When `weak_count` reaches zero, it doesn't free the memory, but when `strong_count` it actually does.
- Since the memory might have been free, so when creating a weak reference, you must call `.upgrade()` on a `Weak<T>` that returns a `Option<Rc<T>>`.
- `Weak<T>` doesn't own its data, so it's only a reference to something.


## interior Mutability

- a rust design pattern.
- allows you to mutate data when there're immutable references to that.
- uses `unsafe`.

## Test Double

- Test something using another type to understand its behaviour. The another type used is called test double.
- Mocks objects --> are types of test doubles that record what happened during the test and helps you to understand the bahaviour.

## RefCell

- single ownership like Box
- own its data
- for single threaded applications as well
- if any borrowing rule is broken. the program will panic
- enforces invariants at runtime
- useful when you're sure your following the rules but the compiler isn't
- immutable and mutable
- can mutate even when the data is immutable
- to get the reference from its internal data we can call `.borrow()`
- we still can't have many mutable references at the same time (will panic at runtime)
- Use `Rc<RefCell<T>>` to have multiple data that's mutable.

## Memory leaks

- it's not impossible to have memory leaks (Rc and RefCell can cause that). 
- if we have a struct that points to itself in a loop, the references will never get down to zero, meaning that it'll never be free. So it will cause a stack overflow.
- rust can't caught reference cycles.


