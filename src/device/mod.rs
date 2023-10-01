use ark_std::{end_timer, start_timer};
use bellman::{bn256::G1, CurveProjective};
pub use halo2curves::{CurveAffine, CurveExt};

pub use utils::*;
///
pub mod common;
///
pub mod device_manager;
///
pub mod device_unit;
///
pub mod utils;

pub use common::*;
pub use device_manager::*;
pub use utils::*;

const NO_AVAILABE_DEVICE: usize = 0x1001;
const MSM_EXECUTION_RESULT_NUM: usize = 254;
const BN256_PROJECTIVE_BYTES: usize = 96;
