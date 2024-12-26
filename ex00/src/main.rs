/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 13:13:04 by ggalon            #+#    #+#             */
/*   Updated: 2024/12/26 11:44:46 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod matrix;
mod vector;
mod traits;

use matrix::Matrix;
use vector::Vector;

fn main()
{
    // Test Matrix operations
    println!("=== Testing Matrix Operations ===\n");
    
    let matrix1 = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let matrix2 = Matrix::new([[9, 8, 7], [6, 5, 4], [3, 2, 1]]);
    
    println!("Matrix 1:");
    matrix1.print();
    println!("\nMatrix 2:");
    matrix2.print();

    println!("\nMatrix Addition (matrix1 + matrix2):");
    let sum = matrix1.clone() + matrix2.clone();
    sum.print();

    println!("\nMatrix Subtraction (matrix1 - matrix2):");
    let diff = matrix1.clone() - matrix2.clone();
    diff.print();

    println!("\nMatrix Scalar Multiplication (matrix1 * 2):");
    let scaled = matrix1.clone() * 2;
    scaled.print();

    // Test Vector operations
    println!("\n=== Testing Vector Operations ===\n");
    
    let vector1 = Vector::new([1, 2, 3]);
    let vector2 = Vector::new([4, 5, 6]);
    
    println!("Vector 1:");
    vector1.print();
    println!("\nVector 2:");
    vector2.print();

    println!("\nVector Addition (vector1 + vector2):");
    let vec_sum = vector1.clone() + vector2.clone();
    vec_sum.print();

    println!("\nVector Subtraction (vector1 - vector2):");
    let vec_diff = vector1.clone() - vector2.clone();
    vec_diff.print();

    println!("\nVector Scalar Multiplication (vector1 * 2):");
    let vec_scaled = vector1.clone() * 2;
    vec_scaled.print();

    println!("\nVector to Matrix Conversion (vector1):");
    let vec_as_matrix = vector1.vtom();
    vec_as_matrix.print();
}