/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   vector.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 13:13:02 by ggalon            #+#    #+#             */
/*   Updated: 2024/12/26 11:08:09 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

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

	pub fn vtom(&self) -> Matrix<K, N, 1>
	{
		let mut matrix = [[K::default(); 1]; N];

		for (i, &elem) in self.data.iter().enumerate()
		{
			matrix[i][0] = elem;
		}

		return Matrix::new(matrix);
	}

	pub fn print(&self)
	{
		println!("{:?}", &self.data);
	}

	pub fn add(&mut self, v: &Vector<K, N>)
	{
		for (i, elem) in self.data.iter_mut().enumerate()
		{
			*elem += v.data[i];
		}
	}

	pub fn sub(&mut self, v: &Vector<K, N>)
	{
		for (i, elem) in self.data.iter_mut().enumerate()
		{
			*elem -= v.data[i];
		}
	}

}

impl<K: Traits, const N: usize> Add for Vector<K, N>
{
	type Output = Self;

	fn add(self, v: Self) -> Self::Output
	{
		let mut result = [K::default(); N];

		for (i, elem) in self.data.iter().enumerate()
		{
			result[i] = *elem + v.data[i];
		}

		return Self::new(result);
	}
}

impl<K: Traits, const N: usize> AddAssign for Vector<K, N>
{
	fn add_assign(&mut self, v: Self)
	{
		for (i, elem) in self.data.iter_mut().enumerate()
		{
			*elem += v.data[i];
		}
	}
}

impl<K: Traits, const N: usize> Sub for Vector<K, N>
{
	type Output = Self;

	fn sub(self, v: Self) -> Self::Output
	{
		let mut result = [K::default(); N];

		for (i, elem) in self.data.iter().enumerate()
		{
			result[i] = *elem - v.data[i];
		}

		return Self::new(result);
	}
}

impl<K: Traits, const N: usize> SubAssign for Vector<K, N>
{
	fn sub_assign(&mut self, v: Self)
	{
		for (i, elem) in self.data.iter_mut().enumerate()
		{
			*elem -= v.data[i];
		}
	}
}