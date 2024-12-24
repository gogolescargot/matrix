/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 13:13:04 by ggalon            #+#    #+#             */
/*   Updated: 2024/12/24 17:53:34 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod matrix;
mod vector;
mod traits;

// use matrix::Matrix;
use vector::Vector;

fn main()
{
	let mut vector0 = Vector::new([1, 2]);
	let vector1 = Vector::new([1, 2]);

	// let mut matrix0 = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
	// let matrix1 = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

	vector0.add(&vector1);

	// matrix0.add(&matrix1);

	let mut vector2 = vector0 + vector1;

	vector2.print();

	vector2 = vector2 - vector2;

	vector2.print();

	// matrix0.print();
}