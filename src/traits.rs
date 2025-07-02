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
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Traits:
	Debug
	+ Default
	+ Copy
	+ From<f64>
	+ Into<f64>
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
{
}

impl<V> Traits for V where
	V: Debug
		+ Default
		+ Copy
		+ From<f64>
		+ Into<f64>
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
{
}
