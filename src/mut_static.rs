use { ForceSomeRwLockReadGuard, ForceSomeRwLockWriteGuard };
use error::*;
use std::any::Any;
use std::cell::Cell;
use std::error::Error as StdError;
use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::RwLock;

pub struct MutStatic<T> {
    data: RwLock<Option<T>>,
    is_set: Cell<bool>
}

unsafe impl <T> Sync for MutStatic<T>
    where T: Sync { }

impl <T> MutStatic<T>
    where T: Any {
    pub fn new() -> Self {
        MutStatic {
            data: RwLock::new(None),
            is_set: Cell::new(false)
        }
    }

    pub fn is_set(&self) -> bool {
        self.is_set.get()
    }

    pub fn read(&self) -> Result<ForceSomeRwLockReadGuard<T>> {
        match self.data.read() {
            Ok(ok) => {
                if let &None = ok.deref() {
                    return Err(ErrorKind::StaticWasNeverSet.into())
                }
                Ok(ForceSomeRwLockReadGuard::new(ok))
            },
            Err(err) => Err(ErrorKind::PoisonError(err.description().to_string(), format!("{}", err)).into()),
        }
    }

    pub fn set(&self, value: T) -> Result<()> {
        let mut data =
            match self.data.write() {
                Ok(some) => some,
                Err(err) => return Err(ErrorKind::PoisonError(err.description().to_string(), format!("{}", err)).into()),
            };

        let data_ref = data.deref_mut();

        if let Some(_) = mem::replace(data_ref, Some(value)) {
            Err(ErrorKind::StaticIsAlreadySet.into())
        } else {
            self.is_set.set(true);
            Ok(())
        }
    }

    pub fn write(&self) -> Result<ForceSomeRwLockWriteGuard<T>> {
        match self.data.write() {
            Ok(ok) => {
                if let &None = ok.deref() {
                    return Err(ErrorKind::StaticWasNeverSet.into())
                }
                Ok(ForceSomeRwLockWriteGuard::new(ok))
            },
            Err(err) => Err(ErrorKind::PoisonError(err.description().to_string(), format!("{}", err)).into()),
        }
    }
}

impl <T> From<T> for MutStatic<T> {
    fn from(value: T) -> Self {
        MutStatic {
            data: RwLock::new(Some(value)),
            is_set: Cell::new(true)
        }
    }
}
