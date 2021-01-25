use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Team {
    Blue = 100,
    Red = 200,
}
