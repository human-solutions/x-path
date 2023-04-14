use crate::{all_paths, inner::PathInner, try_from};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyDir(pub(crate) PathInner);

all_paths!(AnyDir);
try_from!(AnyDir);

impl AnyDir {
    pub(crate) fn validate(self) -> Result<Self> {
        Ok(self)
    }
}
