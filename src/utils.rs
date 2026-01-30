use crate::field::Field;
use std::fmt::Debug;
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
