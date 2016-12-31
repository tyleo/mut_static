# mut_static
Provides a struct to help create mutable statics with lazy_static.

([crates.io][crate])

[crate]: https://crates.io/crates/lazy_static

# Quickstart
To create a mutable static simply put your lazy_static object inside of a `MutStatic`:
``` rust
use mut_static::MutStatic;
use std::mem;

// Declaring a MutStatic
lazy_static! {
    pub static ref MY_OBJECT: MutStatic<MyObject> = {
        MutStatic::new()
    };
}

fn main() {
    // Setting a MutStatic
    MY_OBJECT.set(MyObject::new()).unwrap();

    // Using a MutStatic
    {
        let my_object = MY_OBJECT.read().unwrap();
        let my_object = my_object.deref();
        my_object.use();
    }

    // Using a MutStatic mutably
    {
        let mut my_object = MY_OBJECT.write().unwrap();
        let mut my_object = my_object.deref_mut();
        my_object.use_mut();

        // Resetting a MutStatic
        mem::replace(my_object, MyObject::new());
    }
}
```
