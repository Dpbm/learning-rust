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






