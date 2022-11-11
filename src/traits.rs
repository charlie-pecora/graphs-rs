use std::hash::Hash;
use std::fmt::Display;

pub trait GraphKeyTrait: Eq + Hash + Clone + Display {}
