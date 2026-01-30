use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::field::Field;
use crate::matrix::Matrix;
use crate::utils::{sqrt_newton, to_f32_or_exit};

pub struct Vector<K, const N: usize> {
	pub data: [K; N],
	pub size: usize,
}

impl<K: Field, const N: usize> From<[K; N]> for Vector<K, N> {
	fn from(data: [K; N]) -> Self {
		Self::new(data)
	}
}

impl<K: Field, const N: usize> Vector<K, N> {
	pub fn new(data: [K; N]) -> Self {
		let size = N;
		Self { data, size }
	}

	pub fn clone(&self) -> Self {
		return Self::new(self.data);
	}

	pub fn vtom(&self) -> Matrix<K, N, 1> {
		let mut matrix = [[K::default(); 1]; N];

		for i in 0..N {
			matrix[i][0] = self.data[i];
		}

		return Matrix::new(matrix);
	}

	pub fn print(&self) {
		println!("{:?}", &self.data);
	}

	pub fn add(&mut self, v: &Vector<K, N>) {
		for i in 0..N {
			self.data[i] += v.data[i];
		}
	}

	pub fn sub(&mut self, v: &Vector<K, N>) {
		for i in 0..N {
			self.data[i] -= v.data[i];
		}
	}

	pub fn scl(&mut self, a: K) {
		for i in 0..N {
			self.data[i] *= a;
		}
	}

	pub fn linear_combination(u: &[Vector<K, N>], coefs: &[K]) -> Vector<K, N> {
		if u.len() != coefs.len() {
			panic!("Error: Arrays sizes are different");
		}

		let mut result = Vector::new([K::default(); N]);

		for i in 0..u.len() {
			for j in 0..N {
				result.data[j] = u[i].data[j].mul_add(coefs[i], result.data[j]);
			}
		}

		return result;
	}

	pub fn dot(&self, v: Vector<K, N>) -> K {
		let mut result = K::default();

		for i in 0..N {
			result = self.data[i].mul_add(v.data[i], result);
		}

		return result;
	}

	pub fn norm_1(&self) -> f32 {
		let mut result = f32::default();

		for i in 0..N {
			let val = to_f32_or_exit(self.data[i], "vector::norm_1");
			result += val.abs()
		}

		return result;
	}

	pub fn norm_2(&self) -> f32 {
		let mut result = f32::default();

		for i in 0..N {
			let val: f32 = to_f32_or_exit(self.data[i], "vector::norm_2");
			result = val.mul_add(val, result);
		}

		return sqrt_newton(result);
	}

	pub fn norm_inf(&self) -> f32 {
		if N == 0 {
			return f32::NAN;
		}
		let mut max: f32 = to_f32_or_exit(self.data[0], "vector::norm_inf").abs();

		for i in 1..N {
			let cur = to_f32_or_exit(self.data[i], "vector::norm_inf").abs();
			if max < cur {
				max = cur;
			}
		}

		return max;
	}

	pub fn angle_cos(u: &Vector<K, N>, v: &Vector<K, N>) -> f32 {
		if N == 0 {
			return f32::NAN;
		}
		let norm_product = u.norm_2() * v.norm_2();
		if norm_product == 0. {
			return f32::NAN;
		}
		let dot = to_f32_or_exit(u.dot(v.clone()), "vector::angle_cos");
		return dot / norm_product;
	}

	pub fn cross_product(u: &Vector<K, 3>, v: &Vector<K, 3>) -> Vector<K, 3> {
		Vector::new([
			u.data[1].mul_add(v.data[2], -(u.data[2] * v.data[1])),
			u.data[2].mul_add(v.data[0], -(u.data[0] * v.data[2])),
			u.data[0].mul_add(v.data[1], -(u.data[1] * v.data[0])),
		])
	}
}

impl<K: Field, const N: usize> Add for Vector<K, N> {
	type Output = Self;

	fn add(self, v: Self) -> Self::Output {
		let mut result = Self::new([K::default(); N]);

		for i in 0..N {
			result.data[i] = self.data[i] + v.data[i];
		}

		return result;
	}
}

impl<K: Field, const N: usize> AddAssign for Vector<K, N> {
	fn add_assign(&mut self, v: Self) {
		for i in 0..N {
			self.data[i] += v.data[i];
		}
	}
}

impl<K: Field, const N: usize> Sub for Vector<K, N> {
	type Output = Self;

	fn sub(self, v: Self) -> Self::Output {
		let mut result = Self::new([K::default(); N]);

		for i in 0..N {
			result.data[i] = self.data[i] - v.data[i];
		}

		return result;
	}
}

impl<K: Field, const N: usize> SubAssign for Vector<K, N> {
	fn sub_assign(&mut self, v: Self) {
		for i in 0..N {
			self.data[i] -= v.data[i];
		}
	}
}

impl<K: Field, const N: usize> Mul<K> for Vector<K, N> {
	type Output = Self;

	fn mul(self, a: K) -> Self::Output {
		let mut result = Self::new([K::default(); N]);

		for i in 0..N {
			result.data[i] = self.data[i] * a;
		}

		return result;
	}
}

impl<K: Field, const N: usize> MulAssign<K> for Vector<K, N> {
	fn mul_assign(&mut self, a: K) {
		for i in 0..N {
			self.data[i] *= a;
		}
	}
}
