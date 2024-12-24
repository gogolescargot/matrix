/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   matrix.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 14:20:04 by ggalon            #+#    #+#             */
/*   Updated: 2024/12/24 16:07:40 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::fmt::Debug;
use std::ops::AddAssign;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<K, const M: usize, const N: usize>
{
	size_x: usize,
	size_y: usize,
	is_square: bool,
	data: [[K; N]; M],
}

impl<K: Debug + Copy + Default + AddAssign, const M: usize, const N: usize> Matrix<K, M, N>
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

	pub fn print(&self)
	{
		for row in &self.data
		{
			println!("{:?}", row);
		}
	}

	pub fn add(&mut self, v: &Matrix<K, M, N>)
	{
		for (i, row) in self.data.iter_mut().enumerate()
		{
			for (j, elem) in row.iter_mut().enumerate()
			{
				*elem += v.data[i][j];
			}
		}
	}
}