use super::*;
use crate::elementext::ElementExt;
use crate::svd::{DimElement, Register, RegisterInfo};

impl Parse for Register {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("register") {
            return Err(SVDError::NotExpectedTag("register".to_string()).at(tree.id()));
        }

        let info = RegisterInfo::parse(tree, config)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = DimElement::parse(tree, config)?;
            if info.derived_from.is_some() {
                return Err(SVDErrorAt {
                    error: SVDError::DerivedRegisterArray,
                    id: tree.id()
                })
            }
            check_has_placeholder(&info.name, "register").map_err(|e| e.at(tree.id()))?;
            if let Some(indexes) = &array_info.dim_index {
                if array_info.dim as usize != indexes.len() {
                    return Err(SVDError::IncorrectDimIndexesCount(
                        array_info.dim as usize,
                        indexes.len(),
                    )
                    .at(tree.id()));
                }
            }
            Ok(Register::Array(info, array_info))
        } else {
            Ok(Register::Single(info))
        }
    }
}
