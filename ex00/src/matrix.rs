use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<K, const M: usize, const N: usize>
{
	size_x: usize,
	size_y: usize,
	is_square: bool,
	data: [[K; N]; M],
}

impl<K: Debug + Copy + Default, const M: usize, const N: usize> Matrix<K, M, N>
{
	pub fn new(data: [[K; N]; M]) -> Self
	{
		let size_y = data.len();

		let size_x = if size_y > 0 { data[0].len() } else { 0 };

		let is_square = size_x == size_y;

		Self
		{
			size_x,
            size_y,
            is_square,
            data,
		}
	}

	pub fn print(&self)
	{
		for row in &self.data
		{
            println!("{:?}", row);
        }
	}
}