use std::ops::Deref;
use std::sync::RwLockReadGuard;

pub struct ForceSomeRwLockReadGuard<'a, T>
    where T: 'a {
    data: RwLockReadGuard<'a, Option<T>>,
}

impl <'a, T> ForceSomeRwLockReadGuard<'a, T>
    where T: 'a {
    pub fn new<'b>(data: RwLockReadGuard<'b, Option<T>>) -> Self
        where 'b: 'a {
        ForceSomeRwLockReadGuard {
            data: data,
        }
    }
}

impl <'a, T> AsRef<T> for ForceSomeRwLockReadGuard<'a, T> {
    fn as_ref(&self) -> &T {
        match self.data.deref() {
            &Some(ref some) => some,
            &None => panic!(),
        }
    }
}

impl <'a, T> Deref for ForceSomeRwLockReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self.data.deref() {
            &Some(ref some) => some,
            &None => panic!(),
        }
    }
}
