use nalgebra::{Isometry3, Vector3};

#[test]
fn rot() {
	let p = Vector3::default();
	let r = Vector3::new(0.1, 0.2, 0.3);
	let h = Isometry3::new(p, r);
	// let a = h.rotation.scaled_axis();

	assert_eq!(p, h.translation.vector);
}
