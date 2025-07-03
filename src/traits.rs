/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   traits.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 16:58:30 by ggalon            #+#    #+#             */
/*   Updated: 2025/01/02 15:11:33 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait MulAdd<A = Self, B = Self> {
	type Output;

	fn mul_add(self, a: A, b: B) -> Self::Output;
}

impl MulAdd for f32 {
	type Output = f32;

	fn mul_add(self, a: f32, b: f32) -> Self::Output {
		f32::mul_add(self, a, b)
	}
}

pub trait Traits:
	Debug
	+ Default
	+ Copy
	+ From<f32>
	+ Into<f32>
	+ Add<Output = Self>
	+ AddAssign
	+ Sub<Output = Self>
	+ SubAssign
	+ Mul<Output = Self>
	+ MulAssign
	+ Div<Output = Self>
	+ DivAssign
	+ PartialEq
	+ PartialOrd
	+ MulAdd<Output = Self>
	+ Neg<Output = Self>
{
}

impl<V> Traits for V where
	V: Debug
		+ Default
		+ Copy
		+ From<f32>
		+ Into<f32>
		+ Add<Output = V>
		+ AddAssign
		+ Sub<Output = V>
		+ SubAssign
		+ Mul<Output = V>
		+ MulAssign
		+ Div<Output = V>
		+ DivAssign
		+ PartialEq
		+ PartialOrd
		+ MulAdd<Output = Self>
		+ Neg<Output = Self>
{
}
