use crate::matrix::Matrix;
use crate::vector::lerp;
use crate::vector::Vector;

macro_rules! assert_approx_eq {
	($a:expr, $b:expr) => {
		let epsilon = 1e-6;
		assert!(
			($a - $b).abs() < epsilon,
			"assertion failed: `(left ≈ right)`\n  left: `{:?}`,\n right: `{:?}`",
			$a,
			$b
		);
	};
}

fn cmp_matrix<T, const M: usize, const N: usize>(m1: &Matrix<T, M, N>, m2: &Matrix<T, M, N>)
where
	T: PartialEq + std::fmt::Debug,
{
	for i in 0..M {
		for j in 0..N {
			assert_eq!(m1.data[i][j], m2.data[i][j]);
		}
	}
}

fn cmp_vector<T, const N: usize>(v1: &Vector<T, N>, v2: &Vector<T, N>)
where
	T: PartialEq + std::fmt::Debug,
{
	for i in 0..N {
		assert_eq!(v1.data[i], v2.data[i]);
	}
}

fn cmp_matrix_approx<const M: usize, const N: usize>(
	m1: &Matrix<f32, M, N>,
	m2: &Matrix<f32, M, N>,
) {
	for i in 0..M {
		for j in 0..N {
			assert_approx_eq!(m1.data[i][j], m2.data[i][j]);
		}
	}
}

fn cmp_vector_approx<const N: usize>(v1: &Vector<f32, N>, v2: &Vector<f32, N>) {
	for i in 0..N {
		assert_approx_eq!(v1.data[i], v2.data[i]);
	}
}

#[test]
fn test_vector_add() {
	let mut u = Vector::from([2., 3.]);
	let v = Vector::from([5., 7.]);
	Vector::add(&mut u, &v);
	cmp_vector(&u, &Vector::from([7., 10.]));

	let mut u = Vector::from([0, 0]);
	let v = Vector::from([0, 0]);
	Vector::add(&mut u, &v);
	cmp_vector(&u, &Vector::from([0, 0]));

	let mut u = Vector::new([1, 0]);
	let v = Vector::new([0, 1]);
	Vector::add(&mut u, &v);
	cmp_vector(&u, &Vector::new([1, 1]));

	let mut u = Vector::new([1, 1]);
	let v = Vector::new([1, 1]);
	Vector::add(&mut u, &v);
	cmp_vector(&u, &Vector::new([2, 2]));

	let mut u = Vector::new([21, 21]);
	let v = Vector::new([21, 21]);
	Vector::add(&mut u, &v);
	cmp_vector(&u, &Vector::new([42, 42]));

	let mut u = Vector::new([-21, 21]);
	let v = Vector::new([21, -21]);
	Vector::add(&mut u, &v);
	cmp_vector(&u, &Vector::new([0, 0]));

	let mut u = Vector::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
	let v = Vector::new([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
	Vector::add(&mut u, &v);
	cmp_vector(&u, &Vector::new([9, 9, 9, 9, 9, 9, 9, 9, 9, 9]));
}

#[test]
fn test_vector_sub() {
	let mut u = Vector::new([2., 3.]);
	let v = Vector::new([5., 7.]);
	Vector::sub(&mut u, &v);
	cmp_vector(&u, &Vector::new([-3., -4.]));

	let mut u = Vector::new([0, 0]);
	let v = Vector::new([0, 0]);
	Vector::sub(&mut u, &v);
	cmp_vector(&u, &Vector::new([0, 0]));

	let mut u = Vector::new([1, 0]);
	let v = Vector::new([0, 1]);
	Vector::sub(&mut u, &v);
	cmp_vector(&u, &Vector::new([1, -1]));

	let mut u = Vector::new([1, 1]);
	let v = Vector::new([1, 1]);
	Vector::sub(&mut u, &v);
	cmp_vector(&u, &Vector::new([0, 0]));

	let mut u = Vector::new([21, 21]);
	let v = Vector::new([21, 21]);
	Vector::sub(&mut u, &v);
	cmp_vector(&u, &Vector::new([0, 0]));

	let mut u = Vector::new([-21, 21]);
	let v = Vector::new([21, -21]);
	Vector::sub(&mut u, &v);
	cmp_vector(&u, &Vector::new([-42, 42]));

	let mut u = Vector::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
	let v = Vector::new([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
	Vector::sub(&mut u, &v);
	cmp_vector(&u, &Vector::new([-9, -7, -5, -3, -1, 1, 3, 5, 7, 9]));
}

#[test]
fn test_vector_scl() {
	let mut u = Vector::new([2., 3.]);
	Vector::scl(&mut u, 2.);
	cmp_vector(&u, &Vector::new([4., 6.]));

	let mut u = Vector::new([0, 0]);
	Vector::scl(&mut u, 1);
	cmp_vector(&u, &Vector::new([0, 0]));

	let mut u = Vector::new([1, 0]);
	Vector::scl(&mut u, 1);
	cmp_vector(&u, &Vector::new([1, 0]));

	let mut u = Vector::new([1, 1]);
	Vector::scl(&mut u, 2);
	cmp_vector(&u, &Vector::new([2, 2]));

	let mut u = Vector::new([21, 21]);
	Vector::scl(&mut u, 2);
	cmp_vector(&u, &Vector::new([42, 42]));

	let mut u = Vector::new([42., 42.]);
	Vector::scl(&mut u, 0.5);
	cmp_vector_approx(&u, &Vector::new([21., 21.]));
}

#[test]
fn test_vector_linear_combination() {
	let vectors1 = [
		Vector::new([1., 0., 0.]),
		Vector::new([0., 1., 0.]),
		Vector::new([0., 0., 1.]),
	];
	let vectors2 = [Vector::new([1., 2., 3.]), Vector::new([0., 10., -100.])];
	let coeffs1 = [10., -2., 0.5];
	let coeffs2 = [10., -2.];
	let result1 = Vector::linear_combination(&vectors1, &coeffs1);
	let result2 = Vector::linear_combination(&vectors2, &coeffs2);
	cmp_vector_approx(&result1, &Vector::new([10., -2., 0.5]));
	cmp_vector_approx(&result2, &Vector::new([10., 0., 230.]));

	let vectors = [Vector::new([-42., 42.])];
	let coeffs = [-1.];
	let result = Vector::linear_combination(&vectors, &coeffs);
	cmp_vector_approx(&result, &Vector::new([42., -42.]));

	let vectors = [
		Vector::new([-42.]),
		Vector::new([-42.]),
		Vector::new([-42.]),
	];
	let coeffs = [-1., 1., 0.];
	let result = Vector::linear_combination(&vectors, &coeffs);
	cmp_vector_approx(&result, &Vector::new([0.]));

	let vectors = [
		Vector::new([-42., 42.]),
		Vector::new([1., 3.]),
		Vector::new([10., 20.]),
	];
	let coeffs = [1., -10., -1.];
	let result = Vector::linear_combination(&vectors, &coeffs);
	cmp_vector_approx(&result, &Vector::new([-62., -8.]));

	let vectors = [Vector::new([-42., 100., -69.5]), Vector::new([1., 3., 5.])];
	let coeffs = [1., -10.];
	let result = Vector::linear_combination(&vectors, &coeffs);
	cmp_vector_approx(&result, &Vector::new([-52., 70., -119.5]));
}

#[test]
fn test_vector_dot_product() {
	let u = Vector::new([0., 0.]);
	let v = Vector::new([1., 1.]);
	assert_eq!(u.dot(v), 0.0);

	let u = Vector::new([1., 1.]);
	let v = Vector::new([1., 1.]);
	assert_eq!(u.dot(v), 2.0);

	let u = Vector::new([-1., 6.]);
	let v = Vector::new([3., 2.]);
	assert_eq!(u.dot(v), 9.0);

	let u = Vector::new([0, 0]);
	let v = Vector::new([0, 0]);
	assert_eq!(u.dot(v), 0);

	let u = Vector::new([1, 0]);
	let v = Vector::new([0, 0]);
	assert_eq!(u.dot(v), 0);

	let u = Vector::new([1, 0]);
	let v = Vector::new([1, 0]);
	assert_eq!(u.dot(v), 1);

	let u = Vector::new([1, 0]);
	let v = Vector::new([0, 1]);
	assert_eq!(u.dot(v), 0);

	let u = Vector::new([1, 1]);
	let v = Vector::new([1, 1]);
	assert_eq!(u.dot(v), 2);

	let u = Vector::new([4, 2]);
	let v = Vector::new([2, 1]);
	assert_eq!(u.dot(v), 10);
}

#[test]
fn test_vector_norms() {
	let u = Vector::new([0., 0., 0.]);
	assert_approx_eq!(u.norm_1(), 0.0);
	assert_approx_eq!(u.norm_2(), 0.0);
	assert_approx_eq!(u.norm_inf(), 0.0);

	let u = Vector::new([1., 2., 3.]);
	assert_approx_eq!(u.norm_1(), 6.0);
	assert_approx_eq!(u.norm_2(), 3.7416575);
	assert_approx_eq!(u.norm_inf(), 3.0);

	let u = Vector::new([-1., -2.]);
	assert_approx_eq!(u.norm_1(), 3.0);
	assert_approx_eq!(u.norm_2(), 2.236068);
	assert_approx_eq!(u.norm_inf(), 2.0);

	let u = Vector::new([0.]);
	assert_approx_eq!(u.norm_1(), 0.0);
	assert_approx_eq!(u.norm_2(), 0.0);

	let u = Vector::new([1.]);
	assert_approx_eq!(u.norm_1(), 1.0);
	assert_approx_eq!(u.norm_2(), 1.0);

	let u = Vector::new([0., 0.]);
	assert_approx_eq!(u.norm_1(), 0.0);
	assert_approx_eq!(u.norm_2(), 0.0);

	let u = Vector::new([1., 0.]);
	assert_approx_eq!(u.norm_1(), 1.0);
	assert_approx_eq!(u.norm_2(), 1.0);

	let u = Vector::new([2., 1.]);
	assert_approx_eq!(u.norm_1(), 3.0);
	assert_approx_eq!(u.norm_2(), 2.236067977);

	let u = Vector::new([4., 2.]);
	assert_approx_eq!(u.norm_1(), 6.0);
	assert_approx_eq!(u.norm_2(), 4.472135955);

	let u = Vector::new([-4., -2.]);
	assert_approx_eq!(u.norm_1(), 6.0);
	assert_approx_eq!(u.norm_2(), 4.472135955);
}

#[test]
fn test_vector_angle_cos() {
	let u = Vector::new([1., 0.]);
	let v = Vector::new([0., 1.]);
	assert_approx_eq!(Vector::angle_cos(&u, &v), 0.0);

	let u = Vector::new([8., 7.]);
	let v = Vector::new([3., 2.]);
	assert_approx_eq!(Vector::angle_cos(&u, &v), 0.9914543);

	let u = Vector::new([1., 1.]);
	let v = Vector::new([1., 1.]);
	assert_approx_eq!(Vector::angle_cos(&u, &v), 1.0);

	let u = Vector::new([4., 2.]);
	let v = Vector::new([1., 1.]);
	assert_approx_eq!(Vector::angle_cos(&u, &v), 0.9486833);

	let u = Vector::new([-7., 3.]);
	let v = Vector::new([6., 4.]);
	assert_approx_eq!(Vector::angle_cos(&u, &v), -0.5462678);
}

#[test]
fn test_vector_cross_product() {
	let u = Vector::new([0, 0, 0]);
	let v = Vector::new([0, 0, 0]);
	let result = Vector::<i32, 3>::cross_product(&u, &v);
	cmp_vector(&result, &Vector::new([0, 0, 0]));

	let u = Vector::new([1, 0, 0]);
	let v = Vector::new([0, 0, 0]);
	let result = Vector::<i32, 3>::cross_product(&u, &v);
	cmp_vector(&result, &Vector::new([0, 0, 0]));

	let u = Vector::new([1, 0, 0]);
	let v = Vector::new([0, 1, 0]);
	let result = Vector::<i32, 3>::cross_product(&u, &v);
	cmp_vector(&result, &Vector::new([0, 0, 1]));

	let u = Vector::new([8, 7, -4]);
	let v = Vector::new([3, 2, 1]);
	let result = Vector::<i32, 3>::cross_product(&u, &v);
	cmp_vector(&result, &Vector::new([15, -20, -5]));

	let u = Vector::new([1, 1, 1]);
	let v = Vector::new([0, 0, 0]);
	let result = Vector::<i32, 3>::cross_product(&u, &v);
	cmp_vector(&result, &Vector::new([0, 0, 0]));

	let u = Vector::new([1, 1, 1]);
	let v = Vector::new([1, 1, 1]);
	let result = Vector::<i32, 3>::cross_product(&u, &v);
	cmp_vector(&result, &Vector::new([0, 0, 0]));
}

#[test]
fn test_vector_lerp() {
	assert_approx_eq!(lerp(0., 1., 0.), 0.);
	assert_approx_eq!(lerp(0., 1., 1.), 1.0);
	assert_approx_eq!(lerp(0., 42., 0.5), 21.0);
	assert_approx_eq!(lerp(-42., 42., 0.5), 0.0);

	let u = Vector::new([-42., 42.]);
	let v = Vector::new([42., -42.]);
	let result = lerp(u, v, 0.5);
	cmp_vector_approx(&result, &Vector::new([0.0, 0.0]));

	let u = Vector::new([2., 1.]);
	let v = Vector::new([4., 2.]);
	let result = lerp(u, v, 0.3);
	cmp_vector_approx(&result, &Vector::new([2.6, 1.3]));

	let m1 = Matrix::new([[2., 1.], [3., 4.]]);
	let m2 = Matrix::new([[20., 10.], [30., 40.]]);
	let result = lerp(m1, m2, 0.5);
	let expected = Matrix::new([[11., 5.5], [16.5, 22.]]);
	cmp_matrix_approx(&result, &expected);
}

#[test]
fn test_matrix_addition() {
	let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
	let m2 = Matrix::from([[5., 6.], [7., 8.]]);
	Matrix::add(&mut m1, &m2);
	cmp_matrix(&m1, &Matrix::from([[6., 8.], [10., 12.]]));

	let mut m1 = Matrix::from([[0, 0], [0, 0]]);
	let m2 = Matrix::from([[0, 0], [0, 0]]);
	Matrix::add(&mut m1, &m2);
	cmp_matrix(&m1, &Matrix::from([[0, 0], [0, 0]]));

	let mut m1 = Matrix::new([[1, 0], [0, 1]]);
	let m2 = Matrix::new([[0, 0], [0, 0]]);
	Matrix::add(&mut m1, &m2);
	cmp_matrix(&m1, &Matrix::new([[1, 0], [0, 1]]));

	let mut m1 = Matrix::new([[1, 1], [1, 1]]);
	let m2 = Matrix::new([[1, 1], [1, 1]]);
	Matrix::add(&mut m1, &m2);
	cmp_matrix(&m1, &Matrix::new([[2, 2], [2, 2]]));

	let mut m1 = Matrix::new([[21, 21], [21, 21]]);
	let m2 = Matrix::new([[21, 21], [21, 21]]);
	Matrix::add(&mut m1, &m2);
	cmp_matrix(&m1, &Matrix::new([[42, 42], [42, 42]]));
}

#[test]
fn test_matrix_subtraction() {
	let mut m1 = Matrix::new([[1., 2.], [3., 4.]]);
	let m2 = Matrix::new([[5., 6.], [7., 8.]]);
	Matrix::sub(&mut m1, &m2);
	cmp_matrix(&m1, &Matrix::new([[-4., -4.], [-4., -4.]]));

	let mut m1 = Matrix::new([[0, 0], [0, 0]]);
	let m2 = Matrix::new([[0, 0], [0, 0]]);
	Matrix::sub(&mut m1, &m2);
	cmp_matrix(&m1, &Matrix::new([[0, 0], [0, 0]]));

	let mut m1 = Matrix::new([[1, 0], [0, 1]]);
	let m2 = Matrix::new([[0, 0], [0, 0]]);
	Matrix::sub(&mut m1, &m2);
	cmp_matrix(&m1, &Matrix::new([[1, 0], [0, 1]]));

	let mut m1 = Matrix::new([[1, 1], [1, 1]]);
	let m2 = Matrix::new([[1, 1], [1, 1]]);
	Matrix::sub(&mut m1, &m2);
	cmp_matrix(&m1, &Matrix::new([[0, 0], [0, 0]]));

	let mut m1 = Matrix::new([[21, 21], [21, 21]]);
	let m2 = Matrix::new([[21, 21], [21, 21]]);
	Matrix::sub(&mut m1, &m2);
	cmp_matrix(&m1, &Matrix::new([[0, 0], [0, 0]]));
}

#[test]
fn test_matrix_field_multiplication() {
	let mut m1 = Matrix::new([[1., 2.], [3., 4.]]);
	Matrix::scl(&mut m1, 2.);
	cmp_matrix(&m1, &Matrix::new([[2., 4.], [6., 8.]]));

	let mut m1 = Matrix::new([[0, 0], [0, 0]]);
	Matrix::scl(&mut m1, 0);
	cmp_matrix(&m1, &Matrix::new([[0, 0], [0, 0]]));

	let mut m1 = Matrix::new([[1, 0], [0, 1]]);
	Matrix::scl(&mut m1, 1);
	cmp_matrix(&m1, &Matrix::new([[1, 0], [0, 1]]));

	let mut m1 = Matrix::new([[21., 21.], [21., 21.]]);
	Matrix::scl(&mut m1, 0.5);
	cmp_matrix_approx(&m1, &Matrix::new([[10.5, 10.5], [10.5, 10.5]]));
}

#[test]
fn test_matrix_mul_mat() {
	let m1 = Matrix::new([[1., 0.], [0., 1.]]);
	let m2 = Matrix::new([[1., 0.], [0., 1.]]);
	let result = m1.mul_mat(m2);
	cmp_matrix(&result, &Matrix::new([[1., 0.], [0., 1.]]));

	let m1 = Matrix::new([[1., 0.], [0., 1.]]);
	let m2 = Matrix::new([[2., 1.], [4., 2.]]);
	let result = m1.mul_mat(m2);
	cmp_matrix(&result, &Matrix::new([[2., 1.], [4., 2.]]));

	let m1 = Matrix::new([[3., -5.], [6., 8.]]);
	let m2 = Matrix::new([[2., 1.], [4., 2.]]);
	let result = m1.mul_mat(m2);
	cmp_matrix(&result, &Matrix::new([[-14., -7.], [44., 22.]]));
}

#[test]
fn test_matrix_mul_vec() {
	let m = Matrix::new([[0, 0], [0, 0]]);
	let v = Vector::new([4, 2]);
	let result = m.mul_vec(v);
	cmp_vector(&result, &Vector::new([0, 0]));

	let m = Matrix::new([[1, 0], [0, 1]]);
	let v = Vector::new([4, 2]);
	let result = m.mul_vec(v);
	cmp_vector(&result, &Vector::new([4, 2]));

	let m = Matrix::new([[1, 1], [1, 1]]);
	let v = Vector::new([4, 2]);
	let result = m.mul_vec(v);
	cmp_vector(&result, &Vector::new([6, 6]));

	let m = Matrix::new([[2, 0], [0, 2]]);
	let v = Vector::new([2, 1]);
	let result = m.mul_vec(v);
	cmp_vector(&result, &Vector::new([4, 2]));

	let m = Matrix::new([[0.5, 0.], [0., 0.5]]);
	let v = Vector::new([4., 2.]);
	let result = m.mul_vec(v);
	cmp_vector_approx(&result, &Vector::new([2., 1.]));
}

#[test]
fn test_matrix_trace() {
	let m: Matrix<i32, 2, 2> = Matrix::new([[0, 0], [0, 0]]);
	assert_eq!(m.trace(), 0);

	let m: Matrix<i32, 2, 2> = Matrix::new([[1, 0], [0, 1]]);
	assert_eq!(m.trace(), 2);

	let m: Matrix<i32, 2, 2> = Matrix::new([[1, 2], [3, 4]]);
	assert_eq!(m.trace(), 5);

	let m: Matrix<i32, 2, 2> = Matrix::new([[8, -7], [4, 2]]);
	assert_eq!(m.trace(), 10);

	let m: Matrix<i32, 3, 3> = Matrix::new([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
	assert_eq!(m.trace(), 3);
}

#[test]
fn test_matrix_transpose() {
	let m: Matrix<i32, 2, 2> = Matrix::new([[0, 0], [0, 0]]);
	let result = m.transpose();
	let expected = Matrix::new([[0, 0], [0, 0]]);
	cmp_matrix(&result, &expected);

	let m: Matrix<i32, 2, 2> = Matrix::new([[1, 0], [0, 1]]);
	let result = m.transpose();
	let expected = Matrix::new([[1, 0], [0, 1]]);
	cmp_matrix(&result, &expected);

	let m: Matrix<i32, 2, 2> = Matrix::new([[1, 2], [3, 4]]);
	let result = m.transpose();
	let expected = Matrix::new([[1, 3], [2, 4]]);
	cmp_matrix(&result, &expected);

	let m: Matrix<i32, 3, 3> = Matrix::new([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
	let result = m.transpose();
	let expected = Matrix::new([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
	cmp_matrix(&result, &expected);

	let m: Matrix<i32, 3, 2> = Matrix::new([[1, 2], [3, 4], [5, 6]]);
	let result = m.transpose();
	let expected: Matrix<i32, 2, 3> = Matrix::new([[1, 3, 5], [2, 4, 6]]);
	cmp_matrix(&result, &expected);
}

#[test]
fn test_matrix_row_echelon() {
	let m = Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
	let result = m.row_echelon();
	let expected = Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
	cmp_matrix_approx(&result, &expected);

	let m = Matrix::new([[1., 2.], [3., 4.]]);
	let result = m.row_echelon();
	let expected = Matrix::new([[1., 0.], [0., 1.]]);
	cmp_matrix_approx(&result, &expected);

	let m = Matrix::new([[1., 2.], [2., 4.]]);
	let result = m.row_echelon();
	let expected = Matrix::new([[1., 2.], [0., 0.]]);
	cmp_matrix_approx(&result, &expected);

	let m = Matrix::new([
		[8., 5., -2., 4., 28.],
		[4., 2.5, 20., 4., -4.],
		[8., 5., 1., 4., 17.],
	]);
	let result = m.row_echelon();
	let expected = Matrix::new([
		[1., 0.625, 0., 0., -12.166667],
		[0., 0., 1., 0., -3.6666667],
		[0., 0., 0., 1., 29.5],
	]);
	cmp_matrix_approx(&result, &expected);

	let m = Matrix::new([[0., 0.], [0., 0.]]);
	let result = m.row_echelon();
	let expected = Matrix::new([[0., 0.], [0., 0.]]);
	cmp_matrix_approx(&result, &expected);

	let m = Matrix::new([[1., 0.], [0., 1.]]);
	let result = m.row_echelon();
	let expected = Matrix::new([[1., 0.], [0., 1.]]);
	cmp_matrix_approx(&result, &expected);

	let m = Matrix::new([[4., 2.], [2., 1.]]);
	let result = m.row_echelon();
	let expected = Matrix::new([[1., 0.5], [0., 0.]]);
	cmp_matrix_approx(&result, &expected);

	let m = Matrix::new([[-7., 2.], [4., 8.]]);
	let result = m.row_echelon();
	let expected = Matrix::new([[1., 0.], [0., 1.]]);
	cmp_matrix_approx(&result, &expected);

	let m = Matrix::new([[1., 2.], [4., 8.]]);
	let result = m.row_echelon();
	let expected = Matrix::new([[1., 2.], [0., 0.]]);
	cmp_matrix_approx(&result, &expected);
}

#[test]
fn test_matrix_determinant() {
	let m: Matrix<f32, 2, 2> = Matrix::new([[0., 0.], [0., 0.]]);
	assert_eq!(m.determinant(), 0.0);

	let m: Matrix<f32, 2, 2> = Matrix::new([[1., 0.], [0., 1.]]);
	assert_eq!(m.determinant(), 1.0);

	let m: Matrix<f32, 2, 2> = Matrix::new([[2., 0.], [0., 2.]]);
	assert_eq!(m.determinant(), 4.0);

	let m: Matrix<f32, 2, 2> = Matrix::new([[1., 1.], [1., 1.]]);
	assert_eq!(m.determinant(), 0.0);

	let m: Matrix<f32, 2, 2> = Matrix::new([[0., 1.], [1., 0.]]);
	assert_eq!(m.determinant(), -1.0);

	let m: Matrix<f32, 2, 2> = Matrix::new([[1., 2.], [3., 4.]]);
	assert_eq!(m.determinant(), -2.0);

	let m: Matrix<f32, 2, 2> = Matrix::new([[-7., 5.], [4., 6.]]);
	assert_eq!(m.determinant(), -62.0);

	let m: Matrix<f32, 3, 3> = Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
	assert_eq!(m.determinant(), 1.0);

	let m = Matrix::new([[1., -1.], [-1., 1.]]);
	assert_eq!(m.determinant(), 0.0);

	let m = Matrix::new([[2, 0, 0], [0, 2, 0], [0, 0, 2]]);
	assert_eq!(m.determinant(), 8);

	let m = Matrix::new([[8, 5, -2], [4, 7, 20], [7, 6, 1]]);
	assert_eq!(m.determinant(), -174);

	let m = Matrix::new([
		[8., 5., -2., 4.],
		[4., 2.5, 20., 4.],
		[8., 5., 1., 4.],
		[28., -4., 17., 1.],
	]);
	assert_eq!(m.determinant(), 1032.0);
}

#[test]
fn test_matrix_inverse() {
	let m = Matrix::new([[1., 0.], [0., 1.]]);
	let inv = m.inverse();
	let expected = Matrix::new([[1., 0.], [0., 1.]]);
	cmp_matrix_approx(&inv, &expected);

	let m = Matrix::new([[2., 0.], [0., 2.]]);
	let inv = m.inverse();
	let expected = Matrix::new([[0.5, 0.], [0., 0.5]]);
	cmp_matrix_approx(&inv, &expected);

	let m = Matrix::new([[0.5, 0.], [0., 0.5]]);
	let inv = m.inverse();
	let expected = Matrix::new([[2., 0.], [0., 2.]]);
	cmp_matrix_approx(&inv, &expected);

	let m = Matrix::new([[0., 1.], [1., 0.]]);
	let inv = m.inverse();
	let expected = Matrix::new([[0., 1.], [1., 0.]]);
	cmp_matrix_approx(&inv, &expected);

	let m = Matrix::new([[1., 2.], [3., 4.]]);
	let inv = m.inverse();
	let expected = Matrix::new([[-2., 1.], [1.5, -0.5]]);
	cmp_matrix_approx(&inv, &expected);

	let m = Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
	let inv = m.inverse();
	let expected = Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
	cmp_matrix_approx(&inv, &expected);

	// let m = Matrix::new([[1., 1.], [1., 1.]]);
	// m.inverse();
	// Should exit

	let m = Matrix::new([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
	let inv = m.inverse();
	let expected = Matrix::new([
		[0.6494253, 0.09770115, -0.6551724],
		[-0.7816092, -0.12643678, 0.9655172],
		[0.14367816, 0.07471264, -0.20689655],
	]);
	cmp_matrix_approx(&inv, &expected);
}

#[test]
fn test_matrix_rank() {
	let m: Matrix<f32, 2, 2> = Matrix::new([[0., 0.], [0., 0.]]);
	assert_eq!(m.rank(), 0);

	let m: Matrix<f32, 2, 2> = Matrix::new([[1., 0.], [0., 1.]]);
	assert_eq!(m.rank(), 2);

	let m: Matrix<f32, 2, 2> = Matrix::new([[2., 0.], [0., 2.]]);
	assert_eq!(m.rank(), 2);

	let m: Matrix<f32, 2, 2> = Matrix::new([[1., 1.], [1., 1.]]);
	assert_eq!(m.rank(), 1);

	let m: Matrix<f32, 2, 2> = Matrix::new([[0., 1.], [1., 0.]]);
	assert_eq!(m.rank(), 2);

	let m: Matrix<f32, 2, 2> = Matrix::new([[1., 2.], [3., 4.]]);
	assert_eq!(m.rank(), 2);

	let m: Matrix<f32, 2, 2> = Matrix::new([[-7., 5.], [4., 6.]]);
	assert_eq!(m.rank(), 2);

	let m = Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
	assert_eq!(m.rank(), 3);

	let m = Matrix::new([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
	assert_eq!(m.rank(), 2);

	let m = Matrix::new([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
	assert_eq!(m.rank(), 3);
}
