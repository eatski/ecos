
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct State {
    pub points: Vec<Point>,
}
