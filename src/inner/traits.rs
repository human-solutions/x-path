use std::{
    fmt::{Debug, Display},
    path::Path,
};

use serde::Serialize;

use crate::os::OsGroup;

use super::PathInner;

impl<OS: OsGroup> AsRef<Path> for PathInner<OS> {
    fn as_ref(&self) -> &Path {
        Path::new(&self.path)
    }
}

impl<OS: OsGroup> Display for PathInner<OS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (chr, path) = self.as_contracted(!f.alternate());

        if let Some(chr) = chr {
            write!(f, "{chr}{}", OS::SEP)?;
        }
        write!(f, "{path}")
    }
}

impl<OS: OsGroup> Debug for PathInner<OS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        OS::debug_fmt(&self.path, f)
    }
}

pub trait TryExist<T>: Sized {
    /// Performs the conversion.
    fn try_exist(value: T) -> anyhow::Result<Self>;
}

impl<OS: OsGroup> Serialize for PathInner<OS> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ser.serialize_str(&format!("{self:?}"))
    }
}
