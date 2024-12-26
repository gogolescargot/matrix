/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 13:13:04 by ggalon            #+#    #+#             */
/*   Updated: 2024/12/26 14:12:33 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod matrix;
mod vector;
mod traits;

use crate::traits::Traits;

use matrix::Matrix;
use vector::Vector;
use std::ops::{Add, Mul};

fn lerp<V>(u: V, v: V, t: f32) -> V
where V: Mul<f32, Output = V> + Add<Output = V>
{
	if (t < 0. || t > 1.)
	{
		panic!("Third paramater need to be between 0 and 1");
	}

	return (u * t) + (v * (1. - t));
}

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

	// Test linear combination
	println!("\n=== Testing Linear Combination ===\n");

	let v1 = Vector::new([5, 2]);
	let v2 = Vector::new([3, 4]);
	let vectors = [v1, v2];
	let coefficients = [2, -1];
	
	println!("Vector 1:");
	vectors[0].print();
	println!("\nVector 2:");
	vectors[1].print();
	println!("\nCoefficients: {:?}", coefficients);
	
	println!("\nLinear Combination (2v1 - v2):");
	let result = Vector::linear_combination(&vectors, &coefficients);
	result.print();

	// Test linear interpolation
	println!("\n=== Testing Lerp Function ===\n");
	
	// Test with vectors
	let v1 = Vector::new([0.0, 0.0]);
	let v2 = Vector::new([10.0, 10.0]);
	println!("\nVector lerp with t=0.5:");
	lerp(v1, v2, 0.5).print();
	
	// Test with matrices
	let m1 = Matrix::new([[0.0, 0.0], [0.0, 0.0]]);
	let m2 = Matrix::new([[10.0, 10.0], [10.0, 10.0]]);
	println!("\nMatrix lerp with t=0.5:");
	lerp(m1, m2, 0.5).print();

}