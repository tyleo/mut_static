use ForceSomeRwLockReadGuard;
use std::mem;
use std::ops::DerefMut;
use std::sync::RwLock;

pub struct SafeStatic<T> {
    data: RwLock<Option<T>>,
}

impl <T> SafeStatic<T> {
    pub fn new() -> Self {
        SafeStatic {
            data: RwLock::new(None),
        }
    }

    pub fn get(&self) -> ForceSomeRwLockReadGuard<T> {
        match self.data.read() {
            Ok(ok) => {
                ForceSomeRwLockReadGuard::new(ok)
            },
            Err(_) => panic!(),
        }
    }

    pub fn set(&self, value: T) {
        let mut data =
            match self.data.write() {
                Ok(some) => some,
                Err(_) => panic!(),
            };
        let data_ref = data.deref_mut();
        if let Some(_) = mem::replace(data_ref, Some(value)) {
            panic!();
        }
    }
}
