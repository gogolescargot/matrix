#![allow(dead_code)]

#[cfg(test)]
mod test;

mod matrix;
mod scalar;
mod vector;

use std::ops::{Add, Mul};

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
	V: Mul<f32, Output = V> + Add<Output = V>,
{
	if t < 0. || t > 1. {
		panic!("Scalar need to be between 0 and 1")
	}

	return u * (1. - t) + v * t;
}

fn main() {}
