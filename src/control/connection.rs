//! Contains data structures representing a robot.

/// Whether you are able to send commands to the robot or only read its state.
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum Mode {
	/// Read only access.
	#[default]
	ReadOnly = 0,
	/// Read and write access.
	ReadWrite = 10,
}

/// Which interface you intend to connect to the robot with.
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum Interface {
	/// Primary interface.
	Primary = 1,
	/// Secondary interface.
	#[default]
	Secondary = 2,
	/// Real time interface.
	RealTime = 3,
	/// Real time data exchange interface.
	RTDE = 4,
}

impl Interface {
	/// Minimum period (ms) of connection, except for RTDE, which is guarenteed.
	pub fn period(&self) -> usize {
		match self {
			Self::Primary => 100,
			Self::Secondary => 100,
			Self::RealTime => 2,
			Self::RTDE => 2,
		}
	}

	/// Maximum frequency (Hz) of connection, except for RTDE, which is guarenteed.
	pub fn frequency(&self) -> usize {
		match self {
			Self::Primary => 10,
			Self::Secondary => 10,
			Self::RealTime => 500,
			Self::RTDE => 500,
		}
	}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
struct Meta {
	interface: Interface,
	mode: Mode,
}

impl Meta {
	fn port(&self) -> u16 {
		30000
			+ self.interface as u16
			+ if self.interface == Interface::RTDE {
				0
			} else {
				self.mode as u16
			}
	}
}

/// Connection to a specific robot.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Connection {
	meta: Meta,
	port: u16,
	ip: [u8; 4],
}

impl Connection {
	/// Creates and initialises a Connection.
	/// It is assumed you will create a connection with a working ip address.
	#[must_use]
	pub fn new(interface: Interface, mode: Mode, ip: [u8; 4]) -> Self {
		let meta = Meta { interface, mode };
		Self {
			meta,
			port: meta.port(),
			ip,
		}
	}

	/// Whetherr the connection is read only.
	#[must_use]
	pub fn read_only(&self) -> bool {
		self.meta.mode == Mode::ReadOnly
	}

	/// Which interface to connect to the robot through.
	#[must_use]
	pub fn interface(&self) -> Interface {
		self.meta.interface
	}

	/// The port you are connecting to the robot through.
	#[must_use]
	pub fn port(&self) -> u16 {
		self.port
	}

	/// The IP adress of the robot.
	#[must_use]
	pub fn ip_octet(&self) -> [u8; 4] {
		self.ip
	}
}

impl Default for Connection {
	fn default() -> Self {
		let meta = Meta::default();
		Self {
			meta,
			port: meta.port(),
			ip: [0, 0, 0, 0],
		}
	}
}
