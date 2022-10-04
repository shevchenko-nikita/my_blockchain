use crate::types::Hash;

pub trait Hashable {
    fn hash() -> Hash;
}