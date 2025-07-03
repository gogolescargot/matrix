/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 13:13:04 by ggalon            #+#    #+#             */
/*   Updated: 2025/01/03 18:27:50 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod matrix;
mod test;
mod traits;
mod vector;

use std::ops::{Add, Mul};

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
	V: Mul<f32, Output = V> + Add<Output = V>,
{
	if t < 0. || t > 1. {
		panic!("Scalar need to be between 0 and 1")
	}

	u * (1. - t) + v * t
}

fn main() {}
