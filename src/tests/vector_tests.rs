
use crate::vector::Vector3;

#[test]
fn create_empty_vector() {
	let v = Vector3::empty();
	assert_eq!(v.x, 0.0);
	assert_eq!(v.y, 0.0);
	assert_eq!(v.z, 0.0);
}

#[test]
fn create_fill_vector() {
	let v = Vector3::fill(1.23);
	assert_eq!(v.x, 1.23);
	assert_eq!(v.y, 1.23);
	assert_eq!(v.z, 1.23);
}

#[test]
fn vector_add() {
	let v = Vector3::fill(3.2);
	let v2 = Vector3 { x: 1.0, y: 32.0, z: 2.0 };
	let res = Vector3 {x: 4.2, y: 35.2, z: 5.2};
	assert_eq!(v + v2, res);
}

#[test]
fn vector_sub() {
	let v = Vector3::fill(3.2);
	let v2 = Vector3 { x: 1.0, y: 32.0, z: 2.0 };
	let res = Vector3 {x: 2.2, y: -28.8, z: 1.2};
	assert_eq!(v - v2, res);
}

#[test]
fn vector_mul() {
	let v = Vector3::fill(2.0);
	let v2 = Vector3 { x: 1.0, y: 3.5, z: -2.0 };
	let res = Vector3 {x: 2.0, y: 7.0, z: -4.0};
	assert_eq!(v * v2, res);
}

#[test]
fn vector_div() {
	let v = Vector3::fill(3.0);
	let v2 = Vector3 { x: 6.0, y: -2.0, z: 8.0 };
	let res = Vector3 {x: 0.5, y: -1.5, z: 0.375 };
	assert_eq!(v / v2, res);
}

#[test]
fn vector_dot() {
	let v = Vector3 { x: 3.0, y: -4.0, z: -3.0 };
	let v2 = Vector3 {x: 2.0, y: 18.0, z: -3.0 };
	let res = -57.0;
	assert_eq!(Vector3::dot(v, v2), res);
}

#[test]
fn vector_cross() {
	let v = Vector3 { x: 3.0, y: -4.0, z: -3.0 };
	let v2 = Vector3 {x: 2.0, y: 18.0, z: -3.0 };
	let res = Vector3 {x: 66.0, y: 3.0, z: 62.0 };
	assert_eq!(Vector3::cross(v, v2), res);
}

#[test]
fn vector_cross_unit() {
	let v = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
	let v2 = Vector3 {x: 0.0, y: 1.0, z: 0.0 };
	let res = Vector3 {x: 0.0, y: 0.0, z: 1.0 };
	assert_eq!(Vector3::cross(v, v2), res);
}

#[test]
fn vector_len() {
	let v = Vector3 { x: 3.0, y: -2.0, z: 6.0 };
	let res = v.len();
	assert_eq!(7.0, res);
}

#[test]
fn vector_unit() {
	let v = Vector3 { x: 2.0, y: -4.0, z: 4.0 };
	let res = Vector3 { x: 1.0/3.0, y: -2.0/3.0, z: 2.0/3.0 };
	assert_eq!(v.unit(), res);
}

#[test]
fn vector_add_f() {
	let v = Vector3::fill(3.2).add_f(2.3);
	let res = Vector3::fill(5.5);
	assert_eq!(v, res);
}

#[test]
fn vector_sub_f() {
	let v = Vector3::fill(3.2).sub_f(2.4);
	let res = Vector3::fill(0.8);
	assert_eq!(v, res);
}

#[test]
fn vector_mul_f() {
	let v = Vector3::fill(2.0).mul_f(1.5);
	let res = Vector3::fill(3.0);
	assert_eq!(v, res);
}

#[test]
fn vector_div_f() {
	let v = Vector3::fill(3.0).div_f(2.0);
	let res = Vector3::fill(1.5);
	assert_eq!(v, res);
}