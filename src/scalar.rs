use num_traits::{MulAdd, Num, One, ToPrimitive};
use std::default::Default;
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign, MulAssign, Neg, SubAssign};

pub trait Scalar:
	Num
	+ Default
	+ One
	+ Neg<Output = Self>
	+ Copy
	+ Debug
	+ PartialOrd
	+ AddAssign
	+ SubAssign
	+ MulAssign
	+ DivAssign
	+ MulAdd<Output = Self>
	+ ToPrimitive
{
}

impl<T> Scalar for T where
	T: Num
		+ Default
		+ One
		+ Neg<Output = Self>
		+ Copy
		+ Debug
		+ PartialOrd
		+ AddAssign
		+ SubAssign
		+ MulAssign
		+ DivAssign
		+ MulAdd<Output = Self>
		+ ToPrimitive
{
}
