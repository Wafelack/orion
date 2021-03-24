use crate::{parser::Node};
use std::collections::BTreeMap;

pub enum Value {
    Integer(i32),
    Single(f32),
    Boolean(bool),
    Str(String),
    Function(Node, Vec<String>, Vec<BTreeMap<String, Value>>)
}
