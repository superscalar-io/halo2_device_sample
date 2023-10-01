use super::*;

use crate::poly::Basis;
use crate::poly::Polynomial;
use halo2curves::ff::Field;
use std::ptr;

use super::CurveAffine;
use panda::gpu_manager::unit::*;
use panda::gpu_manager::wrapper::*;

impl DeviceManagerContext {
    /// The core session of the MSM computation execution.
    pub fn session_msm<C: CurveAffine>(
        &mut self,
        gm: &PandaGpuManager,
        scalars: &[C::Scalar],
        bases_index: usize,
    ) -> Result<Vec<u8>, DeviceManagerError> {
        // Convert scalars to bytes using transmute_values
        let scalars_bytes = transmute_values(scalars.as_ref().as_ref());

        // Call panda_msm_bn254_gpu and unwrap the result
        let mut msm_result = panda_msm_bn254_gpu(gm, scalars_bytes, bases_index).unwrap();

        // Create a vector to hold G1 values with the desired capacity
        let mut values = Vec::<G1>::with_capacity(MSM_EXECUTION_RESULT_NUM);

        // Get pointers to the vectors' data
        let values_ptr = values.as_mut_ptr() as *mut u8;
        let msm_result_ptr = msm_result.as_mut_ptr();
        let size = std::mem::size_of::<u8>() * msm_result.len();

        // Copy `msm_result` into `values`
        unsafe {
            std::ptr::copy_nonoverlapping(msm_result_ptr, values_ptr, size);
        }

        // Release the ownership of `msm_result`
        std::mem::forget(msm_result);

        // Set the length of `values` to `count`
        unsafe { values.set_len(MSM_EXECUTION_RESULT_NUM) };

        let mut sum = G1::zero();
        let mut running_sum = G1::zero();

        for bucket in values.iter().rev() {
            running_sum.double();
            running_sum.add_assign(bucket);
        }
        sum.add_assign(&running_sum);

        let mut result_values = vec![0u8; BN256_PROJECTIVE_BYTES];
        let result_values_ptr = result_values.as_mut_ptr();

        // Copy `sum` into `result_values`
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sum as *const G1 as *const u8,
                result_values_ptr,
                BN256_PROJECTIVE_BYTES,
            );
        }

        // Set the length of `result_values` to `BN256_PROJECTIVE_BYTES`
        unsafe { result_values.set_len(BN256_PROJECTIVE_BYTES) };

        Ok(result_values)
    }

    /// The core session of the NTT computation execution.
    pub fn session_ntt<Scalar: Field, G: FftGroup<Scalar>>(
        &mut self,
        gm: &PandaGpuManager,
        scalars: &mut [G],
        log_n: u32,
    ) -> Result<(), DeviceManagerError> {
        //let time = start_timer!(|| "[device manager][ntt session] transmute scalars");
        let scalars_bytes = transmute_values_mut(scalars.as_ref().as_ref());
        //end_timer!(time);

        //let time = start_timer!(|| "[device manager][ntt session] gpu run ntt");
        panda_ntt_bn254_gpu(gm, scalars_bytes, log_n).unwrap();
        //end_timer!(time);

        Ok(())
    }
}
