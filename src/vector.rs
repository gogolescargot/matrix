/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   vector.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 13:13:02 by ggalon            #+#    #+#             */
/*   Updated: 2024/12/26 12:40:57 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

use crate::matrix::Matrix;
use crate::traits::Traits;

pub struct Vector<K, const N: usize>
{
	size: usize,
	data: [K; N]
}

impl<K: Traits, const N: usize> Vector<K, N>
{
	pub fn new(data: [K; N]) -> Self
	{
		let size = data.len();

		Self
		{
			data,
			size,
		}
	}

	pub fn clone(&self) -> Self
	{
		return Self::new(self.data);
	}

	pub fn vtom(&self) -> Matrix<K, N, 1>
	{
		let mut matrix = [[K::default(); 1]; N];

		for i in 0..N
		{
			matrix[i][0] = self.data[i];
		}

		return Matrix::new(matrix);
	}

	pub fn print(&self)
	{
		println!("{:?}", &self.data);
	}

	pub fn add(&mut self, v: &Vector<K, N>)
	{
		for i in 0..N
		{
			self.data[i] += v.data[i];
		}
	}

	pub fn sub(&mut self, v: &Vector<K, N>)
	{
		for i in 0..N
		{
			self.data[i] -= v.data[i];
		}
	}

	pub fn scl(&mut self, a: K)
	{
		for i in 0..N
		{
			self.data[i] *= a;
		}
	}
	
	pub fn linear_combination(u: &[Vector<K, N>], coefs: &[K]) -> Vector<K, N>
	{
		if u.len() != coefs.len()
		{
			panic!("Error: Arrays sizes are different");
		}

		let mut result = Vector::new([K::default(); N]);

		for i in 0..u.len()
		{
			let mut temp = u[i].clone();
			temp *= coefs[i];
			result += temp;
		}
		
		return result;
	}

}

impl<K: Traits, const N: usize> Add for Vector<K, N>
{
	type Output = Self;

	fn add(self, v: Self) -> Self::Output
	{
		let mut result = Self::new([K::default(); N]);

		for i in 0..N
		{
			result.data[i] = self.data[i] + v.data[i];
		}

		return result;
	}
}

impl<K: Traits, const N: usize> AddAssign for Vector<K, N>
{
	fn add_assign(&mut self, v: Self)
	{
		for i in 0..N
		{
			self.data[i] += v.data[i];
		}
	}
}

impl<K: Traits, const N: usize> Sub for Vector<K, N>
{
	type Output = Self;

	fn sub(self, v: Self) -> Self::Output
	{
		let mut result = Self::new([K::default(); N]);

		for i in 0..N
		{
			result.data[i] = self.data[i] - v.data[i];
		}

		return result;
	}
}

impl<K: Traits, const N: usize> SubAssign for Vector<K, N>
{
	fn sub_assign(&mut self, v: Self)
	{
		for i in 0..N
		{
			self.data[i] -= v.data[i];
		}
	}
}

impl<K: Traits, const N: usize> Mul<K> for Vector<K, N>
{
	type Output = Self;

	fn mul(self, a: K) -> Self::Output
	{
		let mut result = Self::new([K::default(); N]);

		for i in 0..N
		{
			result.data[i] = self.data[i] * a;
		}

		return result;
	}
}

impl<K: Traits, const N: usize> MulAssign<K> for Vector<K, N>
{
	fn mul_assign(&mut self, a: K)
	{
		for i in 0..N
		{
			self.data[i] *= a;
		}
	}
}