/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   traits.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 16:58:30 by ggalon            #+#    #+#             */
/*   Updated: 2024/12/26 14:12:50 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::fmt::Debug;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

pub trait Traits:
	Debug +
	Default +
	Copy +
	Add<Output = Self> +
	AddAssign +
	Sub<Output = Self> +
	SubAssign +
	Mul<Output = Self> +
	MulAssign +
	Div<Output = Self> +
	DivAssign {}

impl<V> Traits for V where
	V: Debug +
	Default +
	Copy +
	Add<Output = V> +
	AddAssign +
	Sub<Output = V> +
	SubAssign +
	Mul<Output = V> +
	MulAssign +
	Div<Output = V> +
	DivAssign {}