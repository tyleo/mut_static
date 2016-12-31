use std::ops::{ Deref, DerefMut };
use std::sync::RwLockWriteGuard;

pub struct ForceSomeRwLockWriteGuard<'a, T>
    where T: 'a {
    data: RwLockWriteGuard<'a, Option<T>>,
}

impl <'a, T> ForceSomeRwLockWriteGuard<'a, T>
    where T: 'a {
    pub fn new<'b>(data: RwLockWriteGuard<'b, Option<T>>) -> Self
        where 'b: 'a {
        ForceSomeRwLockWriteGuard {
            data: data,
        }
    }
}

impl <'a, T> AsRef<T> for ForceSomeRwLockWriteGuard<'a, T> {
    fn as_ref(&self) -> &T {
        match self.data.deref() {
            &Some(ref some) => some,
            &None => panic!("Fatal error getting static. The static was never set. This should be impossible."),
        }
    }
}

impl <'a, T> Deref for ForceSomeRwLockWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self.data.deref() {
            &Some(ref some) => some,
            &None => panic!("Fatal error getting static. The static was never set. This should be impossible."),
        }
    }
}

impl <'a, T> DerefMut for ForceSomeRwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self.data.deref_mut() {
            &mut Some(ref mut some) => some,
            &mut None => panic!("Fatal error getting static. The static was never set. This should be impossible."),
        }
    }
}
