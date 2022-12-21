use core::fmt;
use core::ops::{Div, DivAssign, Mul, MulAssign};

use nalgebra::{Isometry3, Vector3};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Pose(Isometry3<f32>);

impl Pose {
	pub fn new_pose(x: f32, y: f32, z: f32, ax: f32, ay: f32, az: f32) -> Self {
		let p: _ = Vector3::new(x, y, z);
		let r: _ = Vector3::new(ax, ay, az);

		Self(Isometry3::new(p, r))
	}
}

impl fmt::Display for Pose {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let r: _ = self.0.rotation.scaled_axis();
		write!(
			f,
			"p[{},{},{},{},{},{}]",
			self.0.translation.x, self.0.translation.y, self.0.translation.z, r.x, r.y, r.z
		)
	}
}

impl Div for Pose {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self(self.0 / rhs.0)
	}
}

impl DivAssign for Pose {
	fn div_assign(&mut self, rhs: Self) {
		self.0 /= rhs.0
	}
}

impl Mul for Pose {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self(self.0 * rhs.0)
	}
}

impl MulAssign for Pose {
	fn mul_assign(&mut self, rhs: Self) {
		self.0 *= rhs.0
	}
}
