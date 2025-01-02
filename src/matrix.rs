/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   matrix.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 14:20:04 by ggalon            #+#    #+#             */
/*   Updated: 2025/01/02 16:02:18 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
use std::cmp::min;

use crate::vector::Vector;
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
		for i in 0..M
		{
			for j in 0..N
			{
				self.data[i][j] *= a;
			}
		}
	}

	pub fn mul_mat<const P: usize>(&self, mat: Matrix<K, N, P>) -> Matrix<K, M, P>
	{
		let mut result: Matrix<K, M, P> = Matrix::new([[K::default(); P]; M]);

		for i in 0..M
		{
			for j in 0..P
			{
				for k in 0..N
				{
					result.data[i][j] += self.data[i][k] * mat.data[k][j];
				}
			}
		}

		return result;
	}

	pub fn mul_vec(&self, vec: Vector<K, N>) -> Matrix<K, M, 1>
	{
		let mut result: Matrix<K, M, 1> = Matrix::new([[K::default(); 1]; M]);

		for i in 0..M
		{
			for k in 0..N
			{
				result.data[i][0] += self.data[i][k] * vec.get_data()[k];
			}
		}
		return result;
	}

	pub fn trace(&self) -> K
	{

		let mut result = K::default();
		
		for i in 0..min(M, N)
		{
			result += self.data[i][i];
		}

		return result;
	}

	pub fn transpose(&self) -> Matrix<K, N, M>
	{
		let mut result: Matrix<K, N, M> = Matrix::new([[K::default(); M]; N]);

		for i in 0..M
		{
			for j in 0..N
			{
				result.data[j][i] = self.data[i][j];
			}
		}

		return result;
	}

	pub fn row_echelon(&self) -> Matrix<K, M, N>
	{
		let mut result: Matrix<K, M, N> = self.clone();

		let mut pivot_row = 0;

		for col in 0..N
		{
			if pivot_row >= M
			{
				break;
			}
			
			// Get the max row and swap

			let mut max_row: usize = pivot_row;

			for i in pivot_row + 1..M
			{
				if result.data[i][col].into().abs() > result.data[max_row][col].into().abs()
				{
					max_row = i;
				}
			}

			if result.data[pivot_row][col] == K::default()
			{
				continue;
			}

			if max_row != pivot_row
			{
				result.data.swap(pivot_row, max_row);
			}

			// Normalize the pivot row

			if result.data[pivot_row][col] != K::default()
			{
				let pivot = result.data[pivot_row][col];
				
				for j in	col..N
				{
					result.data[pivot_row][j] /= pivot;
				}
			}

			// Cancel the elements below

			for i in 0..M
			{
				if i == pivot_row
				{
					continue;
				}
				
				let factor = result.data[i][col];
				
				for j in col..N
				{
					result.data[i][j] -= factor * result.data[pivot_row][j]
				}
			}

			pivot_row += 1;
			
		}

		return result;

	}

}

impl<K: Traits, const M: usize> Matrix<K, M, M>
{
	
	pub fn determinant(&self) -> K
	{
		if M == 0
		{
			return K::from(1.);
		}
		else if M == 1
		{
			return self.data[0][0];
		}
		else if M == 2
		{
			return self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];
		}
		else if M == 3
		{
			return self.data[0][0] * (self.data[1][1] * self.data[2][2] - self.data[1][2] * self.data[2][1])
			- self.data[0][1] * (self.data[1][0] * self.data[2][2] - self.data[1][2] * self.data[2][0])
			+ self.data[0][2] * (self.data[1][0] * self.data[2][1] - self.data[1][1] * self.data[2][0]);
		}
		else if M == 4
		{
			let mut result = K::default();

			for i in 0..4
			{
				let mut submatrix = Matrix::new([[K::default(); 3]; 3]);
		
				for j in 1..4
				{
					let mut col = 0;

					for k in 0..4
					{
						if k != i
						{
							submatrix.data[j - 1][col] = self.data[j][k];
							col += 1;
						}
					}
				}

				if i % 2 == 0
				{
					result += self.data[0][i] * submatrix.determinant();
				}
				else
				{
					result -= self.data[0][i] * submatrix.determinant();
				}
			}

			return result;
		}
		else
		{
			panic!("Error: Matrix max size is 4x4");
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

impl<K: Traits, const M: usize, const N: usize, const P: usize> Mul<Matrix<K, N, P>> for Matrix<K, M, N>
{
	type Output = Matrix<K, M, P>;

	fn mul(self, v: Matrix<K, N, P>) -> Matrix<K, M, P>
	{
		let mut result: Matrix<K, M, P> = Matrix::new([[K::default(); P]; M]);

		for i in 0..M
		{
			for j in 0..P
			{
				for k in 0..N
				{
					result.data[i][j] += self.data[i][k] * v.data[k][j];
				}
			}
		}
		return result;
	}
}

impl<K: Traits, const M: usize, const N: usize> Mul<Vector<K, N>> for Matrix<K, M, N>
{
	type Output = Matrix<K, M, 1>;

	fn mul(self, v: Vector<K, N>) -> Matrix<K, M, 1>
	{
		let mut result: Matrix<K, M, 1> = Matrix::new([[K::default(); 1]; M]);

		for i in 0..M
		{
			for k in 0..N
			{
				result.data[i][0] += self.data[i][k] * v.get_data()[k];
			}
		}
		return result;
	}
}