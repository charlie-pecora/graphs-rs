use std::fmt::Display;
use std::hash::Hash;

pub trait GraphKeyTrait: Eq + Hash + Clone + Display {}
