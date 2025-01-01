/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 13:13:04 by ggalon            #+#    #+#             */
/*   Updated: 2025/01/01 18:42:40 by ggalon           ###   ########.fr       */
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
	return (u * t) + (v * (1. - t));
}

fn main()
{
    // // Test Vector operations
    // println!("=== Vector Tests ===");
    
    // // Test vector creation and printing
    // let mut v1 = Vector::new([1.0, 2.0, 3.0]);
    // let mut v2 = Vector::new([4.0, 5.0, 6.0]);
    // println!("v1: ");
    // v1.print();
    // println!("v2: ");
    // v2.print();

    // // Test vector addition
    // let v3 = v1.clone() + v2.clone();
    // println!("v1 + v2 = ");
    // v3.print();

    // // Test vector subtraction
    // let v4 = v1.clone() - v2.clone();
    // println!("v1 - v2 = ");
    // v4.print();

    // // Test scalar multiplication
    // let v5 = v1.clone() * 2.0;
    // println!("v1 * 2.0 = ");
    // v5.print();

    // // Test add_assign
    // v1 += v2.clone();
    // println!("v1 += v2: ");
    // v1.print();

    // // Test vector to matrix conversion
    // println!("\nVector to Matrix conversion:");
    // let m1 = v1.vtom();
    // m1.print();

    // // Test linear combination
    // let vectors = [
    //     Vector::new([1.0, 0.0, 0.0]),
    //     Vector::new([0.0, 1.0, 0.0]),
    //     Vector::new([0.0, 0.0, 1.0])
    // ];
    // let coefs = [2.0, -1.0, 0.5];
    // let linear_combo = Vector::linear_combination(&vectors, &coefs);
    // println!("\nLinear combination result:");
    // linear_combo.print();

    // // Test dot product
    // println!("\n=== Dot Product Tests ===");
    
    // // Standard vectors dot product
    // let v6 = Vector::new([1.0, 2.0, 3.0]);
    // let v7 = Vector::new([4.0, 5.0, 6.0]);
    // let dot1 = v6.dot(v7);
    // println!("Dot product of [1,2,3] • [4,5,6] = {}", dot1);

    // // Zero vector dot product
    // let zero_vec = Vector::new([0.0, 0.0, 0.0]);
    // let dot2 = v6.dot(zero_vec);
    // println!("Dot product with zero vector = {}", dot2);

    // // Unit vectors dot product
    // let unit_x = Vector::new([1.0, 0.0, 0.0]);
    // let unit_y = Vector::new([0.0, 1.0, 0.0]);
    // let dot3 = unit_x.dot(unit_y);
    // println!("Dot product of unit vectors x•y = {}", dot3);

    // // Test Matrix operations
    // println!("\n=== Matrix Tests ===");
    
    // // Test matrix creation and printing
    // let mut m1 = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
    // let mut m2 = Matrix::new([[5.0, 6.0], [7.0, 8.0]]);
    // println!("Matrix 1:");
    // m1.print();
    // println!("Matrix 2:");
    // m2.print();

    // // Test matrix addition
    // let m3 = m1.clone() + m2.clone();
    // println!("m1 + m2 =");
    // m3.print();

    // // Test matrix subtraction
    // let m4 = m1.clone() - m2.clone();
    // println!("m1 - m2 =");
    // m4.print();

    // // Test scalar multiplication
    // let m5 = m1.clone() * 2.0;
    // println!("m1 * 2.0 =");
    // m5.print();

    // // Test add_assign
    // m1 += m2.clone();
    // println!("m1 += m2:");
    // m1.print();

    // // Test lerp function
    // println!("\n=== Lerp Tests ===");
    // let v_start = Vector::new([0.0, 0.0, 0.0]);
    // let v_end = Vector::new([1.0, 1.0, 1.0]);
    // let v_lerp = lerp(v_start, v_end, 0.5);
    // println!("Lerp result (t=0.5):");
    // v_lerp.print();

    // println!("\n=== Vector Norms Tests ===");
    // let mut v_norms = Vector::new([1.0, -2.0, 3.0]);
    // println!("Vector for norm tests:");
    // v_norms.print();
    
    // // Test L1 norm (sum of absolute values)
    // println!("L1 norm (Manhattan): {}", v_norms.norm_1());  // Should be 6.0
    
    // // Test L2 norm (Euclidean)
    // println!("L2 norm (Euclidean): {}", v_norms.norm_2());  // Should be √14 ≈ 3.74
    
    // // Test inf norm (maximum absolute value)
    // println!("Inf norm (Maximum): {}", v_norms.norm_inf());  // Should be 3.0

    // println!("\n=== Angle Cosine Tests ===");
    
    // // Test parallel vectors (should be 1.0)
    // let v1 = Vector::new([1.0, 0.0, 0.0]);
    // let v2 = Vector::new([2.0, 0.0, 0.0]);
    // println!("Parallel vectors cos angle: {}", Vector::angle_cos(&v1, &v2));
    
    // // Test perpendicular vectors (should be 0.0)
    // let v3 = Vector::new([1.0, 0.0, 0.0]);
    // let v4 = Vector::new([0.0, 1.0, 0.0]);
    // println!("Perpendicular vectors cos angle: {}", Vector::angle_cos(&v3, &v4));
    
    // // Test opposite vectors (should be -1.0)
    // let v5 = Vector::new([1.0, 0.0, 0.0]);
    // let v6 = Vector::new([-1.0, 0.0, 0.0]);
    // println!("Opposite vectors cos angle: {}", Vector::angle_cos(&v5, &v6));
    
    // // Test 45-degree angle vectors (should be ~0.707)
    // let v7 = Vector::new([1.0, 0.0, 0.0]);
    // let v8 = Vector::new([1.0, 1.0, 0.0]);
    // println!("45-degree angle vectors cos angle: {}", Vector::angle_cos(&v7, &v8));

	// println!("\n=== Cross Product Tests ===");
    
    // // Test i × j = k (unit vectors)
    // let i = Vector::new([1.0, 0.0, 0.0]);
    // let j = Vector::new([0.0, 1.0, 0.0]);
    // let k = Vector::<f64, 3>::cross_product(&i, &j);
    // println!("i × j = ");
    // k.print();  // Should be [0, 0, 1]
    
    // // Test anti-commutative property (j × i = -k)
    // let k_opposite = Vector::<f64, 3>::cross_product(&j, &i);
    // println!("j × i = ");
    // k_opposite.print();  // Should be [0, 0, -1]

	// let mut m6 = Matrix::new([[1., 2., 3.], [4., 5., 6.]]);
	// let mut m7 = Matrix::new([[1., 2.], [4., 5.], [7., 8.]]);

    // let m8 = m6.clone() * m7.clone();

	// m8.print();

	// m6.print();

	// v6.print();

	// let m9 = m6.clone() * v6.clone();

	// m9.print();

	let u = Matrix::new([
		[8., 5., -2., 4., 28.],
		[4., 2.5, 20., 4., -4.],
		[8., 5., 1., 4., 17.],
		]);
		u.row_echelon().print();
		// [1.0, 0.625, 0.0, 0.0, -12.1666667]
		// [0.0, 0.0, 1.0, 0.0, -3.6666667]
		// [0.0, 0.0, 0.0, 1.0, 29.5 ]

}