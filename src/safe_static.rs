use error::*;
use ForceSomeRwLockReadGuard;
use std::any::Any;
use std::error::Error as StdError;
use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::RwLock;

pub struct SafeStatic<T> {
    data: RwLock<Option<T>>,
}

impl <T> SafeStatic<T>
    where T: Any {
    pub fn new() -> Self {
        SafeStatic {
            data: RwLock::new(None),
        }
    }

    pub fn get(&self) -> Result<ForceSomeRwLockReadGuard<T>> {
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
            Ok(())
        }
    }
}
