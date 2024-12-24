/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   vector.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 13:13:02 by ggalon            #+#    #+#             */
/*   Updated: 2024/12/24 13:13:02 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::matrix::Matrix;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<K, const N: usize>
{
	size: usize,
	data: [K; N]
}

impl<K: Debug + Copy + Default, const N: usize> Vector<K, N>
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
}