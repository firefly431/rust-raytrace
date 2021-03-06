//! Type aliases for convenience.

pub extern crate nalgebra as na_crate;
pub extern crate rand as rand_crate;
/// Nalgebra crate
pub use self::na_crate as na;
/// Rand crate
pub use self::rand_crate as rand;

/// 2D point
pub type Pnt2 = self::na::Pnt2<f64>;
/// 3D point
pub type Pnt3 = self::na::Pnt3<f64>;
/// 4D point
pub type Pnt4 = self::na::Pnt4<f64>;
/// 2D vector
pub type Vec2 = self::na::Vec2<f64>;
/// 3D vector
pub type Vec3 = self::na::Vec3<f64>;
/// 4D vector
pub type Vec4 = self::na::Vec4<f64>;
/// 3x3 matrix
pub type Mat3 = self::na::Mat3<f64>;
/// 4x4 matrix
pub type Mat4 = self::na::Mat4<f64>;
/// Type of RNG used
pub type RngT = rand::XorShiftRng;
/// Rng trait so you don't have to import it yourself
pub use self::rand::Rng;
