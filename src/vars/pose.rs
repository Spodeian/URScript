/*!
# [`Pose`](Pose)

An important data structure required for proper manipulation of both frames of reference and position of the robot's tool.
The URScript version of this is constructed as 6 [`f32`](f32)s, but this implementation will eventually also work for [`f64`](f64).
The URScript implementation uses scaled axis angled format, but eventually this will allow other formats to be manipulated as well.
*/

use core::fmt;
use core::ops::{Div, DivAssign, Mul, MulAssign};

use nalgebra::{Isometry3, Vector3};

/// Pose, a kind of Isometry in 3-Dimensions (currently a wrapper of [`Isometry3`](Isometry3) using [`f32`](f32) as the type).
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Pose(Isometry3<f32>);

impl Pose {
	/// Creates and initialises a Pose using scaled axis-angle format.
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
