use crate::field::Field;
use std::fmt::Debug;
use std::ops::{Add, Mul};
use std::process;

pub fn to_f32_or_exit<K: Field + Debug>(val: K, ctx: &str) -> f32 {
	match val.to_f32() {
		Some(f) => f,
		None => {
			eprintln!(
				"Erreur: impossible de convertir la valeur {:?} en f32 ({})",
				val, ctx
			);
			process::exit(1);
		}
	}
}

pub fn sqrt_newton(x: f32) -> f32 {
	if x <= 0.0 {
		return 0.0;
	}

	let mut y = x;
	for _ in 0..7 {
		y = 0.5 * (y + x / y);
	}
	return y;
}

pub fn absolute(x: f32) -> f32 {
	if x < 0. {
		return -x;
	}
	return x;
}

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
	V: Mul<f32, Output = V> + Add<Output = V>,
{
	if t < 0. || t > 1. {
		eprintln!("Field need to be between 0 and 1");
		process::exit(1);
	}

	return u * (1. - t) + v * t;
}
