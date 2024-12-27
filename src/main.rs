/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 13:13:04 by ggalon            #+#    #+#             */
/*   Updated: 2024/12/27 15:05:06 by ggalon           ###   ########.fr       */
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
    // Test Vector operations
    println!("=== Vector Tests ===");
    
    // Test vector creation and printing
    let mut v1 = Vector::new([1.0, 2.0, 3.0]);
    let mut v2 = Vector::new([4.0, 5.0, 6.0]);
    println!("v1: ");
    v1.print();
    println!("v2: ");
    v2.print();

    // Test vector addition
    let v3 = v1.clone() + v2.clone();
    println!("v1 + v2 = ");
    v3.print();

    // Test vector subtraction
    let v4 = v1.clone() - v2.clone();
    println!("v1 - v2 = ");
    v4.print();

    // Test scalar multiplication
    let v5 = v1.clone() * 2.0;
    println!("v1 * 2.0 = ");
    v5.print();

    // Test add_assign
    v1 += v2.clone();
    println!("v1 += v2: ");
    v1.print();

    // Test vector to matrix conversion
    println!("\nVector to Matrix conversion:");
    let m1 = v1.vtom();
    m1.print();

    // Test linear combination
    let vectors = [
        Vector::new([1.0, 0.0, 0.0]),
        Vector::new([0.0, 1.0, 0.0]),
        Vector::new([0.0, 0.0, 1.0])
    ];
    let coefs = [2.0, -1.0, 0.5];
    let linear_combo = Vector::linear_combination(&vectors, &coefs);
    println!("\nLinear combination result:");
    linear_combo.print();

    // Test dot product
    println!("\n=== Dot Product Tests ===");
    
    // Standard vectors dot product
    let v6 = Vector::new([1.0, 2.0, 3.0]);
    let v7 = Vector::new([4.0, 5.0, 6.0]);
    let dot1 = v6.dot(v7);
    println!("Dot product of [1,2,3] • [4,5,6] = {}", dot1);

    // Zero vector dot product
    let zero_vec = Vector::new([0.0, 0.0, 0.0]);
    let dot2 = v6.dot(zero_vec);
    println!("Dot product with zero vector = {}", dot2);

    // Unit vectors dot product
    let unit_x = Vector::new([1.0, 0.0, 0.0]);
    let unit_y = Vector::new([0.0, 1.0, 0.0]);
    let dot3 = unit_x.dot(unit_y);
    println!("Dot product of unit vectors x•y = {}", dot3);

    // Test Matrix operations
    println!("\n=== Matrix Tests ===");
    
    // Test matrix creation and printing
    let mut m1 = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
    let mut m2 = Matrix::new([[5.0, 6.0], [7.0, 8.0]]);
    println!("Matrix 1:");
    m1.print();
    println!("Matrix 2:");
    m2.print();

    // Test matrix addition
    let m3 = m1.clone() + m2.clone();
    println!("m1 + m2 =");
    m3.print();

    // Test matrix subtraction
    let m4 = m1.clone() - m2.clone();
    println!("m1 - m2 =");
    m4.print();

    // Test scalar multiplication
    let m5 = m1.clone() * 2.0;
    println!("m1 * 2.0 =");
    m5.print();

    // Test add_assign
    m1 += m2.clone();
    println!("m1 += m2:");
    m1.print();

    // Test lerp function
    println!("\n=== Lerp Tests ===");
    let v_start = Vector::new([0.0, 0.0, 0.0]);
    let v_end = Vector::new([1.0, 1.0, 1.0]);
    let v_lerp = lerp(v_start, v_end, 0.5);
    println!("Lerp result (t=0.5):");
    v_lerp.print();

    println!("\n=== Vector Norms Tests ===");
    let mut v_norms = Vector::new([1.0, -2.0, 3.0]);
    println!("Vector for norm tests:");
    v_norms.print();
    
    // Test L1 norm (sum of absolute values)
    println!("L1 norm (Manhattan): {}", v_norms.norm_1());  // Should be 6.0
    
    // Test L2 norm (Euclidean)
    println!("L2 norm (Euclidean): {}", v_norms.norm_2());  // Should be √14 ≈ 3.74
    
    // Test inf norm (maximum absolute value)
    println!("Inf norm (Maximum): {}", v_norms.norm_inf());  // Should be 3.0
}