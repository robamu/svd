use core::ops::{Deref, DerefMut};

use super::{DimElement, RegisterCluster, RegisterInfo};

/// A single register or array of registers. A register is a named, programmable resource that belongs to a [peripheral](crate::Peripheral).
#[derive(Clone, Debug, PartialEq)]
pub enum Register {
    /// A single register.
    Single(RegisterInfo),
    /// An array of registers.
    Array(RegisterInfo, DimElement),
}

impl Deref for Register {
    type Target = RegisterInfo;

    fn deref(&self) -> &RegisterInfo {
        match self {
            Self::Single(info) => info,
            Self::Array(info, _) => info,
        }
    }
}

impl DerefMut for Register {
    fn deref_mut(&mut self) -> &mut RegisterInfo {
        match self {
            Self::Single(info) => info,
            Self::Array(info, _) => info,
        }
    }
}

impl Register {
    /// Return `true` if register instance is single
    pub const fn is_single(&self) -> bool {
        matches!(self, Self::Single(_))
    }
    /// Return `true` if it is register array
    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_, _))
    }
}

/// Register iterator
pub struct RegIter<'a> {
    pub(crate) rem: Vec<&'a RegisterCluster>,
}

impl<'a> std::iter::Iterator for RegIter<'a> {
    type Item = &'a Register;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(b) = self.rem.pop() {
            match b {
                RegisterCluster::Register(reg) => {
                    return Some(reg);
                }
                RegisterCluster::Cluster(cluster) => {
                    for c in cluster.children.iter().rev() {
                        self.rem.push(c);
                    }
                }
            }
        }
        None
    }
}

/// Mutable register iterator
pub struct RegIterMut<'a> {
    pub(crate) rem: Vec<&'a mut RegisterCluster>,
}

impl<'a> std::iter::Iterator for RegIterMut<'a> {
    type Item = &'a mut Register;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(b) = self.rem.pop() {
            match b {
                RegisterCluster::Register(reg) => {
                    return Some(reg);
                }
                RegisterCluster::Cluster(cluster) => {
                    for c in cluster.children.iter_mut().rev() {
                        self.rem.push(c);
                    }
                }
            }
        }
        None
    }
}

#[cfg(feature = "serde")]
mod ser_de {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(serde::Deserialize, serde::Serialize)]
    struct RegisterArray {
        #[cfg_attr(
            feature = "serde",
            serde(flatten, default, skip_serializing_if = "Option::is_none")
        )]
        dim: Option<DimElement>,
        #[cfg_attr(feature = "serde", serde(flatten))]
        info: RegisterInfo,
    }

    impl Serialize for Register {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Single(info) => info.serialize(serializer),
                Self::Array(info, dim) => RegisterArray {
                    dim: Some(dim.clone()),
                    info: info.clone(),
                }
                .serialize(serializer),
            }
        }
    }

    impl<'de> Deserialize<'de> for Register {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let RegisterArray { dim, info } = RegisterArray::deserialize(deserializer)?;
            if let Some(dim) = dim {
                Ok(info.array(dim))
            } else {
                Ok(info.single())
            }
        }
    }
}
