# mut_static
Provides a struct to help create mutable statics with lazy_static.

([crates.io][crate])

[crate]: https://crates.io/crates/lazy_static

# Quickstart
To create a mutable static simply put your lazy_static object inside of a `MutStatic`:
``` rust
use mut_static::MutStatic;
use std::mem;
use std::ops::DerefMut;

struct MyStruct { value: usize }

impl MyStruct {
    pub fn new(value: usize) -> Self {
        MyStruct{ value: value }
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value
    }
}

// Declaring a MutStatic
lazy_static! {
    pub static ref MY_STRUCT: MutStatic<MyStruct> = {
        MutStatic::new()
    };
}

// Declaring a MutStatic which already has data
lazy_static! {
    pub static ref MY_STRUCT_PRESET: MutStatic<MyStruct> = {
        MutStatic::from(MyStruct::new(0))
    };
}

fn main() {
    // Setting a MutStatic
    MY_STRUCT.set(MyStruct::new(0)).unwrap();

    // Using a MutStatic
    {
        let my_struct = MY_STRUCT.read().unwrap();
        assert!(my_struct.get_value() == 0);
    }

    // Using a MutStatic mutably
    {
        let mut my_struct = MY_STRUCT.write().unwrap();
        my_struct.set_value(1);
        assert!(my_struct.get_value() == 1);
    }

    // Resetting a MutStatic
    {
        let mut my_struct = MY_STRUCT.write().unwrap();
        mem::replace(my_struct.deref_mut(), MyStruct::new(2));
        assert!(my_struct.get_value() == 2);
    }
}
```
