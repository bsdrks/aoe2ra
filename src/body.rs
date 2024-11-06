use crate::{
    body_meta::BodyMeta,
    operation::Operation,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Body {
    pub meta: BodyMeta,
    pub operations: Vec<Operation>,
}
