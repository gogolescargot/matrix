/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   traits.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggalon <ggalon@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/24 16:58:30 by ggalon            #+#    #+#             */
/*   Updated: 2024/12/24 17:43:27 by ggalon           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::fmt::Debug;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

impl<T> Traits for T where
    T: Debug +
       Default +
       Copy +
       Add<Output = T> +
       AddAssign +
       Sub<Output = T> +
       SubAssign +
       Mul<Output = T> +
       MulAssign +
       Div<Output = T> +
       DivAssign {}