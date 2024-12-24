/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 13:13:04 by ggalon            #+#    #+#             */
/*   Updated: 2024/12/24 13:51:01 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod matrix;
mod vector;

use matrix::Matrix;
use vector::Vector;

fn main()
{
	let matrix = Matrix::new([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ]);

	matrix.print();

	let vector = Vector::new([10, 20, 30]);

	vector.print();

	let matrix2 = vector.vtom();

	matrix2.print();

}