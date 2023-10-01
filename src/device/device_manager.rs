use super::CurveAffine;
use group::{GroupOpsOwned, ScalarMulOwned};
use halo2curves::ff::Field;

use super::*;
use panda::gpu_manager::*;

use crate::poly::Basis;
use crate::poly::Polynomial;
use lazy_static::lazy_static;
use std::sync::Mutex;
///
pub trait FftGroup<Scalar: Field>:
    Copy + Send + Sync + 'static + GroupOpsOwned + ScalarMulOwned<Scalar>
{
}

///
impl<T, Scalar> FftGroup<Scalar> for T
where
    Scalar: Field,
    T: Copy + Send + Sync + 'static + GroupOpsOwned + ScalarMulOwned<Scalar>,
{
}

lazy_static! {
    ///
    pub static ref GLOBAL_DEVICE_MANAGER: Mutex<DeviceManager> = Mutex::new(DeviceManager::new());
}

///
#[derive(Clone, Debug)]
pub struct DeviceManager {
    ///
    pub handle: Box<DeviceManagerContext>,
}

/// The implement of DeviceManager
impl DeviceManager {
    /// Create
    pub fn new() -> Self {
        let context = DeviceManagerContext {
            gpu_device_num: 0,
            actived_device_num: 0,
            devices: Vec::<DeviceUnit>::new(),
            msm_param_uints: Vec::<MSMParamUnit>::new(),
            ntt_param_uints: Vec::<NTTParamUnit>::new(),
            init_flag: false,
        };
        Self {
            handle: Box::new(context),
        }
    }

    /// Get the handle of DeviceManager
    pub fn get_handle(&self) -> &DeviceManagerContext {
        &*self.handle
    }

    /// Get the mutable reference handle of DeviceManager.
    pub fn get_handle_mut(&mut self) -> &mut DeviceManagerContext {
        &mut self.handle
    }
}

///
#[derive(Clone, Debug)]
pub struct DeviceManagerContext {
    ///
    pub gpu_device_num: usize,
    ///
    pub actived_device_num: usize,
    ///
    pub devices: Vec<DeviceUnit>,
    ///
    pub msm_param_uints: Vec<MSMParamUnit>,
    ///
    pub ntt_param_uints: Vec<NTTParamUnit>,
    ///
    pub init_flag: bool,
}

impl DeviceManagerContext {
    /// Initialize
    pub fn init(
        &mut self,
        init_device_unit_type: DeviceInitUnitType,
        param_id: Option<usize>,
        bases: Option<&[&[u8]]>,
        omega: Option<&[u8]>,
    ) -> Result<(), DeviceManagerError> {
        // Get the number of GPUs
        self.gpu_device_num = self.get_gpu_device_number().unwrap();

        // In case the number of GPUs is 0, return.
        if self.gpu_device_num == 0 {
            return Err(DeviceManagerError::DeviceManagerErrorGetDeviceNum);
        }

        // Mapping initialization of device computation types.
        let init_uint_type = match init_device_unit_type {
            DeviceInitUnitType::DeviceInitUnitTypeNone => {
                PandaGpuManagerInitUnitType::PandaGpuManagerInitUnitTypeNone
            }
            DeviceInitUnitType::DeviceInitUnitTypeMSM => {
                PandaGpuManagerInitUnitType::PandaGpuManagerInitUnitTypeMSM
            }
            DeviceInitUnitType::DeviceInitUnitTypeNTT => {
                PandaGpuManagerInitUnitType::PandaGpuManagerInitUnitTypeNTT
            }
            DeviceInitUnitType::DevicerInitUnitTypeALL => {
                PandaGpuManagerInitUnitType::PandaGpuManagerInitUnitTypeALL
            }
        };

        // init
        for device_id in 0..self.gpu_device_num {
            // GPU init and get the handle of gpu manager. Setup and copy bases data
            let gm = PandaGpuManager::init(0, init_uint_type.clone(), bases, omega).unwrap();

            match init_uint_type {
                PandaGpuManagerInitUnitType::PandaGpuManagerInitUnitTypeNone => todo!(),
                PandaGpuManagerInitUnitType::PandaGpuManagerInitUnitTypeMSM => {
                    if let Some(id) = param_id {
                        let msm_param_uint = MSMParamUnit {
                            param_id: id,
                            in_usze: true,
                            init_flag: true,
                            gm,
                        };
                        self.msm_param_uints.push(msm_param_uint);
                        // Generate new device unit of MSM.
                        let device: DeviceUnit = DeviceUnit {
                            device_id,
                            device_type: DeviceType::DeviceTypeGPU,
                            device_unit_type: DeviceUnitType::DeviceUnitTypeMSM,
                            device_status: DeviceStatusType::DeviceStatusReady,
                        };
                        self.devices.push(device);
                    } else {
                        return Err(DeviceManagerError::DeviceManagerErrorGetDeviceNum);
                    }
                }
                PandaGpuManagerInitUnitType::PandaGpuManagerInitUnitTypeNTT => {
                    let ntt_param_uint = NTTParamUnit {
                        in_usze: true,
                        init_flag: true,
                        gm,
                    };

                    self.ntt_param_uints.push(ntt_param_uint);
                    // Generate new device unit of NTT.
                    let device: DeviceUnit = DeviceUnit {
                        device_id,
                        device_type: DeviceType::DeviceTypeGPU,
                        device_unit_type: DeviceUnitType::DeviceUnitTypeNTT,
                        device_status: DeviceStatusType::DeviceStatusReady,
                    };
                    self.devices.push(device);
                }
                PandaGpuManagerInitUnitType::PandaGpuManagerInitUnitTypeALL => {
                    if let Some(id) = param_id {
                        let msm_param_uint = MSMParamUnit {
                            param_id: id,
                            in_usze: true,
                            init_flag: true,
                            gm: gm.clone(),
                        };
                        self.msm_param_uints.push(msm_param_uint);
                        // Generate new device unit of MSM.
                        let device: DeviceUnit = DeviceUnit {
                            device_id,
                            device_type: DeviceType::DeviceTypeGPU,
                            device_unit_type: DeviceUnitType::DeviceUnitTypeMSM,
                            device_status: DeviceStatusType::DeviceStatusReady,
                        };
                        self.devices.push(device);
                    } else {
                        return Err(DeviceManagerError::DeviceManagerErrorGetDeviceNum);
                    }
                    // Generate new device unit of NTT.
                    let ntt_param_uint = NTTParamUnit {
                        in_usze: true,
                        init_flag: true,
                        gm: gm.clone(),
                    };

                    self.ntt_param_uints.push(ntt_param_uint);

                    let device: DeviceUnit = DeviceUnit {
                        device_id,
                        device_type: DeviceType::DeviceTypeGPU,
                        device_unit_type: DeviceUnitType::DeviceUnitTypeNTT,
                        device_status: DeviceStatusType::DeviceStatusReady,
                    };
                    self.devices.push(device);
                }
            }
        }

        // Set actived device number and may be a need to use when performing calculations.
        self.actived_device_num = self.gpu_device_num;
        self.init_flag = true;

        Ok(())
    }

    /// Deinitialization
    pub fn deinit(&mut self) -> Result<(), DeviceManagerError> {
        // Set the GPU and active device numbers to 0 to indicate deinitialization.
        self.gpu_device_num = 0;
        self.actived_device_num = 0;

        // Clear the device lists and flags.
        for msm_param_uint in self.msm_param_uints.iter() {
            let mut gm = msm_param_uint.gm.clone();
            gm.deinit();
        }
        self.msm_param_uints.clear();
        for ntt_param_uint in self.ntt_param_uints.iter() {
            let mut gm = ntt_param_uint.gm.clone();
            gm.deinit();
        }
        self.ntt_param_uints.clear();
        self.devices.clear();
        self.init_flag = false;

        Ok(())
    }

    ///
    pub fn get_device_id(&mut self) -> Result<usize, DeviceManagerError> {
        self.gpu_device_num = self.get_gpu_device_number().unwrap();
        if self.gpu_device_num == 0 {
            return Err(DeviceManagerError::DeviceManagerErrorGetDeviceNum);
        }
        Ok(self.gpu_device_num)
    }

    /// Get available devices.
    fn get_available_device(&mut self) -> Result<usize, DeviceManagerError> {
        for i in 0..self.actived_device_num {
            match self.devices[i].device_status {
                DeviceStatusType::DeviceStatusNone => todo!(),
                DeviceStatusType::DeviceStatusIdle => todo!(),
                DeviceStatusType::DeviceStatusReady => {
                    return Ok(i);
                }
                DeviceStatusType::DeviceStatusRunning => todo!(),
            }
        }
        Ok(NO_AVAILABE_DEVICE)
    }

    /// Run the MSM calculation process.
    pub fn execute_msm<C: CurveAffine>(
        &mut self,
        msm_param_id: usize,
        bases_index: usize,
        scalars: &[C::Scalar],
    ) -> Result<Vec<u8>, DeviceManagerError> {
        let device_id = self.get_available_device().unwrap();

        let mut msm_result = Vec::<u8>::new();
        if device_id != NO_AVAILABE_DEVICE {
            if let Some(device) = self.devices.get_mut(device_id) {
                device.device_id = device_id;
                device.device_status = DeviceStatusType::DeviceStatusRunning;
                // todo Need new type~
                device.device_unit_type = DeviceUnitType::DeviceUnitTypeMSM;

                let mut found_msm_param_uint: Option<&MSMParamUnit> = None;

                for msm_param_uint in self.msm_param_uints.iter() {
                    if msm_param_uint.param_id == msm_param_id {
                        found_msm_param_uint = Some(msm_param_uint);
                        break;
                    }
                }
                if let Some(param_uint) = found_msm_param_uint {
                    let gm = &param_uint.gm.clone();
                    msm_result = self.session_msm::<C>(gm, scalars, bases_index).unwrap();
                }
            }

            if let Some(device) = self.devices.get_mut(device_id) {
                device.device_status = DeviceStatusType::DeviceStatusReady;
                device.device_unit_type = DeviceUnitType::DeviceUnitTypeNone;
            }
        } else {
            println!("Warning: Execute MSM No available device");
            return Err(DeviceManagerError::DeviceManagerErrorNoAvailableDevice);
        }

        Ok(msm_result)
    }

    /// Run the NTT calculation process.
    pub fn execute_ntt<Scalar: Field, G: FftGroup<Scalar>>(
        &mut self,
        scalars: &mut [G],
        log_n: u32,
    ) -> Result<(), DeviceManagerError> {
        let device_id = self.get_available_device().unwrap();

        if device_id != NO_AVAILABE_DEVICE {
            if let Some(device) = self.devices.get_mut(device_id) {
                device.device_id = device_id;
                device.device_status = DeviceStatusType::DeviceStatusRunning;
                // todo Need new type~
                device.device_unit_type = DeviceUnitType::DeviceUnitTypeNTT;

                let gm = &self.ntt_param_uints[0].gm.clone();
                self.session_ntt::<Scalar, G>(gm, scalars, log_n).unwrap();
            }

            if let Some(device) = self.devices.get_mut(device_id) {
                device.device_status = DeviceStatusType::DeviceStatusReady;
                device.device_unit_type = DeviceUnitType::DeviceUnitTypeNone;
            }
        } else {
            println!("Warning: Execute NTT No available device");
            return Err(DeviceManagerError::DeviceManagerErrorNoAvailableDevice);
        }

        Ok(())
    }

    /// Get the numbere of units of GPU.
    pub fn get_gpu_unit_number(&mut self) -> Result<usize, DeviceManagerError> {
        return Ok(self.devices.len());
    }

    /// Get the numbere of MSM param units of GPU.
    pub fn get_gpu_msm_param_uints_number(&mut self) -> Result<usize, DeviceManagerError> {
        return Ok(self.msm_param_uints.len());
    }

    /// Get device info of GPUs.
    pub fn get_gpu_device_info(
        &mut self,
        device_id: usize,
    ) -> Result<DeviceGPUInfo, DeviceManagerError> {
        if self.gpu_device_num == 0 || device_id >= self.gpu_device_num {
            return Err(DeviceManagerError::DeviceManagerErrorGetDeviceNum);
        }

        let panda_gpu_info = panda::gpu_manager::device_info(device_id.try_into().unwrap());

        match panda_gpu_info {
            Ok(panda_gpu_info) => {
                let device_info = DeviceGPUInfo {
                    gpu_device_id: device_id,
                    panda_gpu_info,
                };
                Ok(device_info)
            }
            Err(_) => Err(DeviceManagerError::DeviceManagerErrorGetDeviceInfo),
        }
    }

    /// Get device infos of GPUs.
    pub fn get_gpu_device_infos(&mut self) -> Result<Vec<DeviceGPUInfo>, DeviceManagerError> {
        // In case the number of GPUs is 0, return.
        if self.gpu_device_num == 0 {
            return Err(DeviceManagerError::DeviceManagerErrorGetDeviceNum);
        }

        let device_infos: Vec<DeviceGPUInfo> = (0..self.gpu_device_num)
            .map(
                |id| match panda::gpu_manager::device_info(id.try_into().unwrap()) {
                    Ok(panda_gpu_info) => Ok(DeviceGPUInfo {
                        gpu_device_id: id,
                        panda_gpu_info,
                    }),
                    Err(_) => Err(DeviceManagerError::DeviceManagerErrorGetDeviceInfo),
                },
            )
            .collect::<Result<_, DeviceManagerError>>()?;

        Ok(device_infos)
    }

    ///Get device number of GPUs.
    pub fn get_gpu_device_number(&mut self) -> Result<usize, DeviceManagerError> {
        Ok(panda::gpu_manager::get_device_number()
            .unwrap()
            .try_into()
            .unwrap())
    }

    /// Set device for GPU.
    pub fn set_gpu_device(&mut self, device_id: usize) -> Result<(), DeviceManagerError> {
        if let Err(_) = panda::gpu_manager::set_device(device_id) {
            return Err(DeviceManagerError::DeviceManagerSetDeviceError);
        }
        Ok(())
    }

    /// Transmute ther formats into a byte stream.
    pub fn transmute_values<'a, U: std::fmt::Debug>(&mut self, values: &'a [U]) -> &'a [u8] {
        transmute_values(values)
    }
}
