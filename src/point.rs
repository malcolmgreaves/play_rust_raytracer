use std::ops::{Add, Sub};
use vector::Vector3;

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}