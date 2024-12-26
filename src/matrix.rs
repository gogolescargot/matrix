/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   matrix.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 14:20:04 by ggalon            #+#    #+#             */
/*   Updated: 2024/12/26 12:34:49 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
use crate::traits::Traits;

pub struct Matrix<K, const M: usize, const N: usize>
{
	size_x: usize,
	size_y: usize,
	is_square: bool,
	data: [[K; N]; M],
}

impl<K: Traits, const M: usize, const N: usize> Matrix<K, M, N>
{
	pub fn new(data: [[K; N]; M]) -> Self
	{
		let size_y = data.len();

		let size_x = if size_y > 0 { data[0].len() } else { 0 };

		let is_square = size_x == size_y;

		Self
		{
			size_x,
			size_y,
			is_square,
			data,
		}
	}

	pub fn clone(&self) -> Self
	{
		return Self::new(self.data);
	}

	pub fn print(&self)
	{
		for row in &self.data
		{
			println!("{:?}", row);
		}
	}

	pub fn add(&mut self, v: &Matrix<K, M, N>)
	{
		for i in 0..M
		{
			for j in 0..N
			{
				self.data[i][j] += v.data[i][j];
			}
		}
	}

	pub fn sub(&mut self, v: &Matrix<K, M, N>)
	{
		for i in 0..M
		{
			for j in 0..N
			{
				self.data[i][j] -= v.data[i][j];
			}
		}
	}

	pub fn scl(&mut self, a: K)
	{
		for i in 0..N
		{
			for j in 0..M
			{
				self.data[i][j] *= a;
			}
		}
	}

}

impl<K: Traits, const M: usize, const N: usize> Add for Matrix<K, M, N>
{
	type Output = Self;

	fn add(self, v: Self) -> Self::Output
	{
		let mut result = Self::new([[K::default(); N]; M]);

		for i in 0..M
		{
			for j in 0..N
			{
				result.data[i][j] = self.data[i][j] + v.data[i][j];
			}
		}

		return result;
	}
}

impl<K: Traits, const M: usize, const N: usize> AddAssign for Matrix<K, M, N>
{
	fn add_assign(&mut self, v: Self)
	{
		for i in 0..M
		{
			for j in 0..N
			{
				self.data[i][j] += v.data[i][j];
			}
		}
	}
}

impl<K: Traits, const M: usize, const N: usize> Sub for Matrix<K, M, N>
{
	type Output = Self;

	fn sub(self, v: Self) -> Self::Output
	{
		let mut result = Self::new([[K::default(); N]; M]);

		for i in 0..M
		{
			for j in 0..N
			{
				result.data[i][j] = self.data[i][j] - v.data[i][j];
			}
		}

		return result;
	}
}

impl<K: Traits, const M: usize, const N: usize> SubAssign for Matrix<K, M, N>
{
	fn sub_assign(&mut self, v: Self)
	{
		for i in 0..M
		{
			for j in 0..N
			{
				self.data[i][j] -= v.data[i][j];
			}
		}
	}
}

impl<K: Traits, const M: usize, const N: usize> Mul<K> for Matrix<K, M, N>
{
	type Output = Self;

	fn mul(self, a: K) -> Self::Output
	{
		let mut result = Self::new([[K::default(); N]; M]);

		for i in 0..M
		{
			for j in 0..N
			{
				result.data[i][j] = self.data[i][j] * a;
			}
		}

		return result;
	}
}

impl<K: Traits, const M: usize, const N: usize> MulAssign<K> for Matrix<K, M, N>
{
	fn mul_assign(&mut self, a: K)
	{
		for i in 0..M
		{
			for j in 0..N	
			{
				self.data[i][j] *= a;
			}
		}
	}
}