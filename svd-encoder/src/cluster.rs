use super::{Element, ElementMerge, Encode, EncodeError};

use crate::svd::Cluster;

impl Encode for Cluster {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        match self {
            Self::Single(i) => i.encode(),
            Self::Array(i, a) => {
                let mut e = Element::new("cluster");
                e.merge(&a.encode()?);
                e.merge(&i.encode()?);
                Ok(e)
            }
        }
    }
}
