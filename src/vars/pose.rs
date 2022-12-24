/*!
# [`Pose`](Pose)

An important data structure required for proper manipulation of both frames of reference and more specifically the position of the robot's tool.
The `URScript` implementation uses scaled axis angled format, but eventually this will allow other formats to be manipulated as well.
*/

use core::fmt;
use core::ops::{Div, DivAssign, Mul, MulAssign};

use nalgebra::RealField;
use nalgebra::{Isometry3, Vector3};

use num::Float;

/// Pose, a kind of [Isometry](https://en.wikipedia.org/wiki/Isometry) in 3-Dimensions (currently a wrapper of [`Isometry3`] which is itself wrapping a float).
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Pose<T>(Isometry3<T>)
where
	T: Float + RealField;

impl<T> Pose<T>
where
	T: Float + RealField,
{
	/// Creates and initialises a Pose using scaled axis-angle format.
	pub fn new_pose(x: T, y: T, z: T, ax: T, ay: T, az: T) -> Self {
		Self(Isometry3::new(
			Vector3::new(x, y, z),
			Vector3::new(ax, ay, az),
		))
	}

	/// Creates and initialises a Pose with only a position, and no orientation.
	/// This is especially useful for generating new targets relative to a home position.
	pub fn new_pos(x: T, y: T, z: T) -> Self {
		Self(Isometry3::translation(x, y, z))
	}

	/// Creates and initialises a Pose with only an orientation, and position.
	/// This is useful for rotating targets without moving them.
	pub fn new_rot(ax: T, ay: T, az: T) -> Self {
		Self(Isometry3::rotation(Vector3::new(ax, ay, az)))
	}
}

impl<T> fmt::Display for Pose<T>
where
	T: Float + RealField,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let r: _ = self.0.rotation.scaled_axis();
		write!(
			f,
			"p[{},{},{},{},{},{}]",
			self.0.translation.x.to_f32().unwrap(),
			self.0.translation.y.to_f32().unwrap(),
			self.0.translation.z.to_f32().unwrap(),
			r.x.to_f32().unwrap(),
			r.y.to_f32().unwrap(),
			r.z.to_f32().unwrap()
		)
	}
}

impl<T> Div for Pose<T>
where
	T: Float + RealField,
{
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self(self.0 / rhs.0)
	}
}

impl<T> DivAssign for Pose<T>
where
	T: Float + RealField,
{
	fn div_assign(&mut self, rhs: Self) {
		self.0 /= rhs.0;
	}
}

impl<T> Mul for Pose<T>
where
	T: Float + RealField,
{
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self(self.0 * rhs.0)
	}
}

impl<T> MulAssign for Pose<T>
where
	T: Float + RealField,
{
	fn mul_assign(&mut self, rhs: Self) {
		self.0 *= rhs.0;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn mul() {
		use core::f32::consts::PI;

		let h1 = Pose::new_pose(0., 1., 2., 0., PI, 0.);
		let h2 = Pose::new_pos(0., 2., 4.);
		let h3 = Pose::new_pose(0., 3., 6., 0., PI, 0.);

		assert_eq!(h3, h2 * h1);
	}
}
