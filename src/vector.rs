/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   vector.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 13:13:02 by ggalon            #+#    #+#             */
/*   Updated: 2024/12/31 13:22:18 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::matrix::Matrix;
use crate::traits::Traits;

pub struct Vector<K, const N: usize> {
	pub data: [K; N],
}

impl<K: Traits, const N: usize> Vector<K, N> {
	pub fn new(data: [K; N]) -> Self {
		Self { data }
	}

	pub fn clone(&self) -> Self {
		return Self::new(self.data);
	}

	pub fn get_data(&self) -> &[K; N] {
		return &self.data;
	}

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
			let mut temp = u[i].clone();
			temp *= coefs[i];
			result += temp;
		}

		return result;
	}

	pub fn dot(&self, v: Vector<K, N>) -> K {
		let mut result = K::default();

		for i in 0..self.data.len() {
			result += self.data[i] * v.data[i];
		}

		return result;
	}

	pub fn norm_1(&self) -> f64 {
		let mut result = f64::default();

		for i in 0..self.data.len() {
			result += self.data[i].into().abs()
		}

		return result;
	}

	pub fn norm_2(&self) -> f64 {
		let mut result = f64::default();

		for i in 0..self.data.len() {
			result += self.data[i].into().powi(2);
		}

		return result.sqrt();
	}

	pub fn norm_inf(&self) -> f64 {
		let mut max: f64 = self.data[0].into().abs();

		for i in 1..self.data.len() {
			if max < self.data[i].into().abs() {
				max = self.data[i].into().abs();
			}
		}

		return max;
	}

	pub fn angle_cos(u: &Vector<K, N>, v: &Vector<K, N>) -> f64 {
		return (u.dot(v.clone())).into() / (u.norm_2() * v.norm_2());
	}

	pub fn cross_product(u: &Vector<K, 3>, v: &Vector<K, 3>) -> Vector<K, 3> {
		return Vector::new([
			(u.data[1] * v.data[2]) - (u.data[2] * v.data[1]),
			(u.data[2] * v.data[0]) - (u.data[0] * v.data[2]),
			(u.data[0] * v.data[1]) - (u.data[1] * v.data[0]),
		]);
	}
}

impl<K: Traits, const N: usize> Add for Vector<K, N> {
	type Output = Self;

	fn add(self, v: Self) -> Self::Output {
		let mut result = Self::new([K::default(); N]);

		for i in 0..N {
			result.data[i] = self.data[i] + v.data[i];
		}

		return result;
	}
}

impl<K: Traits, const N: usize> AddAssign for Vector<K, N> {
	fn add_assign(&mut self, v: Self) {
		for i in 0..N {
			self.data[i] += v.data[i];
		}
	}
}

impl<K: Traits, const N: usize> Sub for Vector<K, N> {
	type Output = Self;

	fn sub(self, v: Self) -> Self::Output {
		let mut result = Self::new([K::default(); N]);

		for i in 0..N {
			result.data[i] = self.data[i] - v.data[i];
		}

		return result;
	}
}

impl<K: Traits, const N: usize> SubAssign for Vector<K, N> {
	fn sub_assign(&mut self, v: Self) {
		for i in 0..N {
			self.data[i] -= v.data[i];
		}
	}
}

impl<K: Traits, const N: usize> Mul<K> for Vector<K, N> {
	type Output = Self;

	fn mul(self, a: K) -> Self::Output {
		let mut result = Self::new([K::default(); N]);

		for i in 0..N {
			result.data[i] = self.data[i] * a;
		}

		return result;
	}
}

impl<K: Traits, const N: usize> MulAssign<K> for Vector<K, N> {
	fn mul_assign(&mut self, a: K) {
		for i in 0..N {
			self.data[i] *= a;
		}
	}
}

pub fn lerp<V>(u: V, v: V, t: f64) -> V
where
	V: Mul<f64, Output = V> + Add<Output = V>,
{
	u * (1. - t) + v * t
}
