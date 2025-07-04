use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::matrix::Matrix;
use crate::scalar::Scalar;

pub struct Vector<K, const N: usize> {
	pub data: [K; N],
	pub size: usize,
}

impl<K: Scalar, const N: usize> Vector<K, N> {
	pub fn new(data: [K; N]) -> Self {
		let size = N;
		Self { data, size }
	}

	pub fn clone(&self) -> Self {
		return Self::new(self.data);
	}

	#[allow(dead_code)]
	pub fn vtom(&self) -> Matrix<K, N, 1> {
		let mut matrix = [[K::default(); 1]; N];

		for i in 0..N {
			matrix[i][0] = self.data[i];
		}

		return Matrix::new(matrix);
	}

	#[allow(dead_code)]
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
			result += self.data[i].to_f32().unwrap().abs()
		}

		return result;
	}

	pub fn norm_2(&self) -> f32 {
		let mut result = f32::default();

		for i in 0..N {
			let val: f32 = self.data[i].to_f32().unwrap();
			result = val.mul_add(val, result);
		}

		result.sqrt()
	}

	pub fn norm_inf(&self) -> f32 {
		if N == 0 {
			return f32::NAN;
		}
		let mut max: f32 = self.data[0].to_f32().unwrap().abs();

		for i in 1..N {
			if max < self.data[i].to_f32().unwrap().abs() {
				max = self.data[i].to_f32().unwrap().abs();
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
		return (u.dot(v.clone())).to_f32().unwrap() / norm_product;
	}

	pub fn cross_product(u: &Vector<K, 3>, v: &Vector<K, 3>) -> Vector<K, 3> {
		Vector::new([
			u.data[1].mul_add(v.data[2], -(u.data[2] * v.data[1])),
			u.data[2].mul_add(v.data[0], -(u.data[0] * v.data[2])),
			u.data[0].mul_add(v.data[1], -(u.data[1] * v.data[0])),
		])
	}
}

impl<K: Scalar, const N: usize> Add for Vector<K, N> {
	type Output = Self;

	fn add(self, v: Self) -> Self::Output {
		let mut result = Self::new([K::default(); N]);

		for i in 0..N {
			result.data[i] = self.data[i] + v.data[i];
		}

		return result;
	}
}

impl<K: Scalar, const N: usize> AddAssign for Vector<K, N> {
	fn add_assign(&mut self, v: Self) {
		for i in 0..N {
			self.data[i] += v.data[i];
		}
	}
}

impl<K: Scalar, const N: usize> Sub for Vector<K, N> {
	type Output = Self;

	fn sub(self, v: Self) -> Self::Output {
		let mut result = Self::new([K::default(); N]);

		for i in 0..N {
			result.data[i] = self.data[i] - v.data[i];
		}

		return result;
	}
}

impl<K: Scalar, const N: usize> SubAssign for Vector<K, N> {
	fn sub_assign(&mut self, v: Self) {
		for i in 0..N {
			self.data[i] -= v.data[i];
		}
	}
}

impl<K: Scalar, const N: usize> Mul<K> for Vector<K, N> {
	type Output = Self;

	fn mul(self, a: K) -> Self::Output {
		let mut result = Self::new([K::default(); N]);

		for i in 0..N {
			result.data[i] = self.data[i] * a;
		}

		return result;
	}
}

impl<K: Scalar, const N: usize> MulAssign<K> for Vector<K, N> {
	fn mul_assign(&mut self, a: K) {
		for i in 0..N {
			self.data[i] *= a;
		}
	}
}
