use crate::matrix::Matrix;
use crate::vector::{lerp, Vector};

#[cfg(test)]

fn assert_approx_eq(a: f64, b: f64) {
	assert!(
		(a - b).abs() < 1e-9,
		"assertion failed: `(left ≈ right)` (left: `{}`, right: `{}`)",
		a,
		b
	);
}

#[test]
fn test_vector_creation() {
	let v = Vector::new([1., 2., 3.]);
	assert_eq!(v.data, [1., 2., 3.]);
}

#[test]
fn test_vector_addition() {
	let mut v1 = Vector::new([1., 2., 3.]);
	let v2 = Vector::new([4., 5., 6.]);
	Vector::add(&mut v1, &v2);
	assert_eq!(v1.data, [5., 7., 9.]);
}

#[test]
fn test_vector_subtraction() {
	let mut v1 = Vector::new([1., 2., 3.]);
	let v2 = Vector::new([4., 5., 6.]);
	Vector::sub(&mut v1, &v2);
	assert_eq!(v1.data, [-3., -3., -3.]);
}

#[test]
fn test_vector_scalar_multiplication() {
	let mut v1 = Vector::new([1., 2., 3.]);
	Vector::scl(&mut v1, 2.);
	assert_eq!(v1.data, [2., 4., 6.]);
}

#[test]
fn test_vector_add_assign() {
	let mut v1 = Vector::new([1., 2., 3.]);
	let v2 = Vector::new([4., 5., 6.]);
	v1 += v2;
	assert_eq!(v1.data, [5., 7., 9.]);
}

#[test]
fn test_vector_to_matrix() {
	let v = Vector::new([1., 2., 3.]);
	let m = v.vtom();
	assert_eq!(m.data, [[1.], [2.], [3.]]);
}

#[test]
fn test_vector_linear_combination() {
	let vectors = [
		Vector::new([1., 0., 0.]),
		Vector::new([0., 1., 0.]),
		Vector::new([0., 0., 1.]),
	];
	let coefs = [2., -1., 0.5];
	let result = Vector::linear_combination(&vectors, &coefs);
	assert_eq!(result.data, [2., -1., 0.5]);
}

#[test]
fn test_vector_dot_product() {
	let v1 = Vector::new([1., 2., 3.]);
	let v2 = Vector::new([4., 5., 6.]);
	assert_eq!(v1.dot(v2), 32.);
}

#[test]
fn test_vector_norms() {
	let v = Vector::new([1., -2., 3.]);
	assert_eq!(v.norm_1(), 6.);
	assert_approx_eq(v.norm_2(), 14.0_f64.sqrt());
	assert_eq!(v.norm_inf(), 3.);
}

#[test]
fn test_vector_angle_cos() {
	let v1 = Vector::new([1., 0.]);
	let v2 = Vector::new([1., 1.]);
	assert_approx_eq(Vector::angle_cos(&v1, &v2), 1. / 2.0_f64.sqrt());
}

#[test]
fn test_vector_cross_product() {
	let i = Vector::new([1., 0., 0.]);
	let j = Vector::new([0., 1., 0.]);
	let k = Vector::<f64, 3>::cross_product(&i, &j);
	assert_eq!(k.data, [0., 0., 1.]);
}

#[test]
fn test_vector_lerp() {
	let v_start = Vector::new([0., 0., 0.]);
	let v_end = Vector::new([1., 1., 1.]);
	let v_lerp = lerp(v_start, v_end, 0.5);
	assert_eq!(v_lerp.data, [0.5, 0.5, 0.5]);
}

#[test]
fn test_matrix_creation() {
	let m = Matrix::new([[1., 2.], [3., 4.]]);
	assert_eq!(m.data, [[1., 2.], [3., 4.]]);
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
fn test_matrix_matrix_multiplication() {
	let m1 = Matrix::new([[1., 2., 3.], [4., 5., 6.]]);
	let m2 = Matrix::new([[1., 2.], [4., 5.], [7., 8.]]);
	let result = Matrix::mul_mat(&m1, m2);
	assert_eq!(result.data, [[30., 36.], [66., 81.]]);
}

#[test]
fn test_matrix_vector_multiplication() {
	let m = Matrix::new([[1., 2., 3.], [4., 5., 6.]]);
	let v = Vector::new([1., 2., 3.]);
	let result = Matrix::mul_vec(&m, v);
	assert_eq!(result.data, [[14.], [32.]]);
}

#[test]
fn test_matrix_trace() {
	let m = Matrix::new([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
	assert_eq!(m.trace(), 15.);
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
	let m = Matrix::new([
		[8., 5., -2., 4., 28.],
		[4., 2.5, 20., 4., -4.],
		[8., 5., 1., 4., 17.],
	]);
	let echelon = m.row_echelon();
	assert_eq!(echelon.data[1][0], 0.);
	assert_eq!(echelon.data[2][0], 0.);
	assert_eq!(echelon.data[2][1], 0.);
}

#[test]
fn test_matrix_determinant() {
	let m = Matrix::new([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
	assert_approx_eq(m.determinant(), -174.);
}

#[test]
fn test_matrix_inverse() {
	let m = Matrix::new([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
	let inv_m = m.inverse();
	let expected = Matrix::new([
		[113. / 174., 17. / 174., -114. / 174.],
		[-136. / 174., -22. / 174., 168. / 174.],
		[25. / 174., 13. / 174., -36. / 174.],
	]);
	for i in 0..3 {
		for j in 0..3 {
			assert_approx_eq(inv_m.data[i][j], expected.data[i][j]);
		}
	}
}

#[test]
fn test_matrix_rank() {
	let m = Matrix::new([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
	assert_eq!(m.rank(), 3);
}
