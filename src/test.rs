use crate::lerp;
use crate::matrix::Matrix;
use crate::vector::Vector;

#[cfg(test)]

fn assert_approx_eq(a: f32, b: f32) {
	assert!(
		(a - b).abs() < 1e-6,
		"assertion failed: `(left != right)` (left: `{}`, right: `{}`)",
		a,
		b
	);
}

#[test]
fn test_vector_add() {
	let mut v1 = Vector::new([2., 3.]);
	let v2 = Vector::new([5., 7.]);
	Vector::add(&mut v1, &v2);
	assert_eq!(v1.data, [7., 10.]);
}

#[test]
fn test_vector_sub() {
	let mut v1 = Vector::new([2., 3.]);
	let v2 = Vector::new([5., 7.]);
	Vector::sub(&mut v1, &v2);
	assert_eq!(v1.data, [-3., -4.]);
}

#[test]
fn test_vector_scl() {
	let mut v1 = Vector::new([2., 3.]);
	Vector::scl(&mut v1, 2.);
	assert_eq!(v1.data, [4., 6.]);
}

#[test]
fn test_vector_linear_combination() {
	let vs1 = [
		Vector::new([1., 0., 0.]),
		Vector::new([0., 1., 0.]),
		Vector::new([0., 0., 1.]),
	];
	let vs2 = [Vector::new([1., 2., 3.]), Vector::new([0., 10., -100.])];
	let coefs1 = [10., -2., 0.5];
	let coefs2 = [10., -2.];
	let result1 = Vector::linear_combination(&vs1, &coefs1);
	let result2 = Vector::linear_combination(&vs2, &coefs2);
	assert_eq!(result1.data, [10., -2., 0.5]);
	assert_eq!(result2.data, [10., -0., 230.]);
}

#[test]
fn test_vector_dot_product() {
	let v1 = Vector::new([0., 0.]);
	let v2 = Vector::new([1., 1.]);
	assert_eq!(v1.dot(v2), 0.0);

	let v1: Vector<f32, 2> = Vector::new([1., 1.]);
	let v2 = Vector::new([1., 1.]);
	assert_eq!(v1.dot(v2), 2.0);

	let v1 = Vector::new([-1., 6.]);
	let v2 = Vector::new([3., 2.]);
	assert_eq!(v1.dot(v2), 9.0);
}

#[test]
fn test_vector_norms() {
	let u = Vector::new([0., 0., 0.]);
	assert_eq!(u.norm_1(), 0.0);
	assert_approx_eq(u.norm_2(), 0.0);
	assert_eq!(u.norm_inf(), 0.0);

	let u = Vector::new([1., 2., 3.]);
	assert_eq!(u.norm_1(), 6.0);
	assert_approx_eq(u.norm_2(), 3.7416575);
	assert_eq!(u.norm_inf(), 3.0);

	let u = Vector::new([-1., -2.]);
	assert_eq!(u.norm_1(), 3.0);
	assert_approx_eq(u.norm_2(), 2.236068);
	assert_eq!(u.norm_inf(), 2.0);
}

#[test]
fn test_vector_angle_cos() {
	let u = Vector::new([1., 0.]);
	let v = Vector::new([1., 0.]);
	assert_approx_eq(Vector::angle_cos(&u, &v), 1.0);

	let u = Vector::new([1., 0.]);
	let v = Vector::new([0., 1.]);
	assert_approx_eq(Vector::angle_cos(&u, &v), 0.0);

	let u = Vector::new([-1., 1.]);
	let v = Vector::new([1., -1.]);
	assert_approx_eq(Vector::angle_cos(&u, &v), -1.0);

	let u = Vector::new([2., 1.]);
	let v = Vector::new([4., 2.]);
	assert_approx_eq(Vector::angle_cos(&u, &v), 1.0);

	let u = Vector::new([1., 2., 3.]);
	let v = Vector::new([4., 5., 6.]);
	assert_approx_eq(Vector::angle_cos(&u, &v), 0.97463185);
}

#[test]
fn test_vector_cross_product() {
	let u = Vector::new([0., 0., 1.]);
	let v = Vector::new([1., 0., 0.]);
	let result = Vector::<f32, 3>::cross_product(&u, &v);
	assert_eq!(result.data, [0., 1., 0.]);

	let u = Vector::new([1., 2., 3.]);
	let v = Vector::new([4., 5., 6.]);
	let result = Vector::<f32, 3>::cross_product(&u, &v);
	assert_eq!(result.data, [-3., 6., -3.]);

	let u = Vector::new([4., 2., -3.]);
	let v = Vector::new([-2., -5., 16.]);
	let result = Vector::<f32, 3>::cross_product(&u, &v);
	assert_eq!(result.data, [17., -58., -16.]);
}

#[test]
fn test_vector_lerp() {
	assert_approx_eq(lerp(0., 1., 0.), 0.0);
	assert_approx_eq(lerp(0., 1., 1.), 1.0);
	assert_approx_eq(lerp(0., 1., 0.5), 0.5);
	assert_approx_eq(lerp(21., 42., 0.3), 27.3);

	let v_start = Vector::new([2., 1.]);
	let v_end = Vector::new([4., 2.]);
	let v_lerp = lerp(v_start, v_end, 0.3);
	assert_approx_eq(v_lerp.data[0], 2.6);
	assert_approx_eq(v_lerp.data[1], 1.3);

	let m_start = Matrix::new([[2., 1.], [3., 4.]]);
	let m_end = Matrix::new([[20., 10.], [30., 40.]]);
	let m_lerp = lerp(m_start, m_end, 0.5);
	let expected_matrix = Matrix::new([[11., 5.5], [16.5, 22.]]);
	for i in 0..2 {
		for j in 0..2 {
			assert_approx_eq(m_lerp.data[i][j], expected_matrix.data[i][j]);
		}
	}
}

#[test]
fn test_matrix_addition() {
	let mut m1 = Matrix::new([[1., 2.], [3., 4.]]);
	let m2 = Matrix::new([[5., 6.], [7., 8.]]);
	Matrix::add(&mut m1, &m2);
	assert_eq!(m1.data, [[6., 8.], [10., 12.]]);
}

#[test]
fn test_matrix_subtraction() {
	let mut m1 = Matrix::new([[1., 2.], [3., 4.]]);
	let m2 = Matrix::new([[5., 6.], [7., 8.]]);
	Matrix::sub(&mut m1, &m2);
	assert_eq!(m1.data, [[-4., -4.], [-4., -4.]]);
}

#[test]
fn test_matrix_scalar_multiplication() {
	let mut m1 = Matrix::new([[1., 2.], [3., 4.]]);
	Matrix::scl(&mut m1, 2.);
	assert_eq!(m1.data, [[2., 4.], [6., 8.]]);
}

#[test]
fn test_matrix_mul_mat() {
	let u = Matrix::new([[1., 0.], [0., 1.]]);
	let v = Matrix::new([[1., 0.], [0., 1.]]);
	let result = u.mul_mat(v);
	assert_eq!(result.data, [[1., 0.], [0., 1.]]);

	let u = Matrix::new([[1., 0.], [0., 1.]]);
	let v = Matrix::new([[2., 1.], [4., 2.]]);
	let result = u.mul_mat(v);
	assert_eq!(result.data, [[2., 1.], [4., 2.]]);

	let u = Matrix::new([[3., -5.], [6., 8.]]);
	let v = Matrix::new([[2., 1.], [4., 2.]]);
	let result = u.mul_mat(v);
	assert_eq!(result.data, [[-14., -7.], [44., 22.]]);
}

#[test]
fn test_matrix_mul_vec() {
	let u = Matrix::new([[1., 0.], [0., 1.]]);
	let v = Vector::new([4., 2.]);
	let result = u.mul_vec(v);
	assert_eq!(result.data, [[4.], [2.]]);

	let u = Matrix::new([[2., 0.], [0., 2.]]);
	let v = Vector::new([4., 2.]);
	let result = u.mul_vec(v);
	assert_eq!(result.data, [[8.], [4.]]);

	let u = Matrix::new([[2., -2.], [-2., 2.]]);
	let v = Vector::new([4., 2.]);
	let result = u.mul_vec(v);
	assert_eq!(result.data, [[4.], [-4.]]);
}

#[test]
fn test_matrix_trace() {
	let u = Matrix::new([[1., 0.], [0., 1.]]);
	assert_eq!(u.trace(), 2.0);

	let u = Matrix::new([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
	assert_eq!(u.trace(), 9.0);

	let u = Matrix::new([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
	assert_eq!(u.trace(), -21.0);
}

#[test]
fn test_matrix_transpose() {
	let m = Matrix::new([[1., 2., 3.], [4., 5., 6.]]);
	let t = m.transpose();
	let expected = Matrix::new([[1., 4.], [2., 5.], [3., 6.]]);
	assert_eq!(t.data, expected.data);
}

#[test]
fn test_matrix_row_echelon() {
	let u = Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
	let echelon = u.row_echelon();
	let expected = Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
	for i in 0..3 {
		for j in 0..3 {
			assert_approx_eq(echelon.data[i][j], expected.data[i][j]);
		}
	}

	let u = Matrix::new([[1., 2.], [3., 4.]]);
	let echelon = u.row_echelon();
	let expected = Matrix::new([[1., 0.], [0., 1.]]);
	for i in 0..2 {
		for j in 0..2 {
			assert_approx_eq(echelon.data[i][j], expected.data[i][j]);
		}
	}

	let u = Matrix::new([[1., 2.], [2., 4.]]);
	let echelon = u.row_echelon();
	let expected = Matrix::new([[1., 2.], [0., 0.]]);
	for i in 0..2 {
		for j in 0..2 {
			assert_approx_eq(echelon.data[i][j], expected.data[i][j]);
		}
	}

	let u = Matrix::new([
		[8., 5., -2., 4., 28.],
		[4., 2.5, 20., 4., -4.],
		[8., 5., 1., 4., 17.],
	]);
	let echelon = u.row_echelon();
	let expected = Matrix::new([
		[1., 0.625, 0., 0., -12.166667],
		[0., 0., 1., 0., -3.6666667],
		[0., 0., 0., 1., 29.5],
	]);
	for i in 0..3 {
		for j in 0..5 {
			assert_approx_eq(echelon.data[i][j], expected.data[i][j]);
		}
	}
}

#[test]
fn test_matrix_determinant() {
	let u = Matrix::new([[1., -1.], [-1., 1.]]);
	assert_approx_eq(u.determinant(), 0.0);

	let u = Matrix::new([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
	assert_approx_eq(u.determinant(), 8.0);

	let u = Matrix::new([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
	assert_approx_eq(u.determinant(), -174.0);

	let u = Matrix::new([
		[8., 5., -2., 4.],
		[4., 2.5, 20., 4.],
		[8., 5., 1., 4.],
		[28., -4., 17., 1.],
	]);
	assert_approx_eq(u.determinant(), 1032.0);
}

#[test]
fn test_matrix_inverse() {
	let u = Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
	let inv_u = u.inverse();
	let expected = Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
	for i in 0..3 {
		for j in 0..3 {
			assert_approx_eq(inv_u.data[i][j], expected.data[i][j]);
		}
	}

	let u = Matrix::new([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
	let inv_u = u.inverse();
	let expected = Matrix::new([[0.5, 0., 0.], [0., 0.5, 0.], [0., 0., 0.5]]);
	for i in 0..3 {
		for j in 0..3 {
			assert_approx_eq(inv_u.data[i][j], expected.data[i][j]);
		}
	}

	let u = Matrix::new([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
	let inv_u = u.inverse();
	let expected = Matrix::new([
		[0.6494253, 0.09770115, -0.6551724],
		[-0.7816092, -0.12643678, 0.9655172],
		[0.14367816, 0.07471264, -0.20689655],
	]);
	for i in 0..3 {
		for j in 0..3 {
			assert_approx_eq(inv_u.data[i][j], expected.data[i][j]);
		}
	}
}

#[test]
fn test_matrix_rank() {
	let u = Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
	assert_eq!(u.rank(), 3);

	let u = Matrix::new([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
	assert_eq!(u.rank(), 2);

	let u = Matrix::new([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
	assert_eq!(u.rank(), 3);
}
