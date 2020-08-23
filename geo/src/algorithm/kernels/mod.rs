use crate::{CoordinateType, Coordinate};
use super::winding_order::WindingOrder;

/// Kernel trait to provide predicates to operate on
/// different scalar types.
pub trait Kernel {
    type Scalar: CoordinateType;

    /// Gives the orientation of 3 2-dimensional points:
    /// ccw, cw or colinear (None)
    fn orient2d(
        p: Coordinate<Self::Scalar>,
        q: Coordinate<Self::Scalar>,
        r: Coordinate<Self::Scalar>,
    ) -> Option<WindingOrder>;
}

/// Marker trait to assign Kernel for scalars
pub trait HasKernel: CoordinateType {
    type Ker: Kernel<Scalar = Self>;
}

#[macro_export]
macro_rules! has_kernel {
	  ($t:ident, $k:ident) => {
        impl $crate::algorithm::kernels::HasKernel for $t {
            type Ker = $k<$t>;
        }
	  };
}

pub mod robust;
pub use self::robust::RobustKernel;
has_kernel!(f64, RobustKernel);
has_kernel!(f32, RobustKernel);


pub mod simple;
pub use self::simple::SimpleKernel;
has_kernel!(i64, SimpleKernel);
has_kernel!(i32, SimpleKernel);

#[cfg(test)]
mod test;
