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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Host<'a> {
	IP(&'a [u8; 4]),
	Hostname(&'a str),
}

impl Host<'static> {
	const UNSPECIFIED: [u8; 4] = [0; 4];
}

impl Default for Host<'static> {
	fn default() -> Self {
		Host::IP(&Host::UNSPECIFIED)
	}
}

/// Connection to a specific robot.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Connection<'a> {
	port: u16,
	host: Host<'a>,
	meta: Meta,
}

impl<'a> Connection<'a> {
	/// Creates and initialises a connection.
	/// It is assumed you will create a connection with a working ip address.
	#[must_use]
	pub fn new_ip(interface: Interface, mode: Mode, ip: &'a [u8; 4]) -> Self {
		let meta = Meta { interface, mode };
		Self {
			meta,
			port: meta.port(),
			host: Host::IP(ip),
		}
	}

	/// Creates and initialises a connection.
	/// It is assumed you will create a connection with a correct hostname connected to a DNS.
	/// The hostname of the robot by default is (ur-'serial number').
	/// The serial number is an 11 or 10 digit base-10 number representing manufacturing information.
	#[must_use]
	pub fn new_hostname(interface: Interface, mode: Mode, hostname: &'a str) -> Self {
		let meta = Meta { interface, mode };
		Self {
			meta,
			port: meta.port(),
			host: Host::Hostname(hostname),
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
	pub fn ip_octet(&self) -> Option<&[u8; 4]> {
		if let Host::IP(octet) = self.host {
			Some(octet)
		} else {
			None
		}
	}

	/// The hostname of the robot (ur-'serial number').
	/// The serial number is an 11 or 10 digit base-10 number representing manufacturing information.
	#[must_use]
	pub fn host_name(&self) -> Option<&str> {
		if let Host::Hostname(host) = self.host {
			Some(host)
		} else {
			None
		}
	}
}

impl Default for Connection<'static> {
	fn default() -> Self {
		let meta = Meta::default();
		Self {
			meta,
			port: meta.port(),
			host: Host::default(),
		}
	}
}
