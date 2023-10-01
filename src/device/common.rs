use libc::c_void;
use panda::gpu_manager::*;
use std::{mem, ptr};

/// GPU model
#[derive(Copy, Clone, Debug)]
pub enum DeviceGPUType {
    ///
    GPUNone,
    ///
    GPUNVIDIA3090,
    ///
    GPUNVIDIA3090TI,
    ///
    GPUNVIDIA4090,
    ///
    GPUNVIDIA4090TI,
}

/// Device type: GPU/FPGA
#[derive(Copy, Clone, Debug)]
pub enum DeviceType {
    ///
    DeviceTypeNone,
    ///
    DeviceTypeGPU,
    ///
    DeviceTypeFPGA,
}

/// The type of computing that is being performed on the device.
#[derive(Copy, Clone, Debug)]
pub enum DeviceUnitType {
    ///
    DeviceUnitTypeNone,
    ///
    DeviceUnitTypeMSM,
    ///
    DeviceUnitTypeNTT,
}

/// Current device status
#[derive(Copy, Clone, Debug)]
pub enum DeviceStatusType {
    ///
    DeviceStatusNone,
    ///
    DeviceStatusIdle,
    ///
    DeviceStatusReady,
    ///
    DeviceStatusRunning,
}

/// The type of computation initialized on the device.
#[derive(Clone, Debug)]
pub enum DeviceInitUnitType {
    ///
    DeviceInitUnitTypeNone,
    ///
    DeviceInitUnitTypeMSM,
    ///
    DeviceInitUnitTypeNTT,
    ///
    DevicerInitUnitTypeALL,
}

/// Device component unit
#[derive(Clone, Debug)]
pub struct DeviceUnit {
    ///
    pub device_id: usize,
    ///
    pub device_type: DeviceType,
    ///
    pub device_unit_type: DeviceUnitType,
    ///
    pub device_status: DeviceStatusType,
}

/// Device info of GPU.
#[derive(Clone, Debug)]
pub struct DeviceGPUInfo {
    ///
    pub gpu_device_id: usize,
    ///
    pub panda_gpu_info: PandaDeviceInfo,
}

/// Device info.
#[derive(Clone, Debug)]
pub struct DeviceInfo {
    ///
    pub gpu_device_num: usize,
    ///
    pub device_gpu_info: Vec<DeviceGPUInfo>,
}

/// MSM param unit as multiple circuits require multiple params.
#[derive(Clone, Debug)]
pub struct MSMParamUnit {
    ///
    pub param_id: usize,
    ///
    pub in_usze: bool,
    ///
    pub init_flag: bool,
    ///
    pub gm: PandaGpuManager,
}

/// NTT param unit as multiple circuits require multiple params.
#[derive(Clone, Debug)]
pub struct NTTParamUnit {
    ///
    pub in_usze: bool,
    ///
    pub init_flag: bool,
    ///
    pub gm: PandaGpuManager,
}

/// The error type of device manager.
#[derive(Clone, Debug)]
pub enum DeviceManagerError {
    /// It means no device.
    DeviceManagerErrorGetDeviceNum,
    /// It means that there may be devices, but there are no idle devices available.
    DeviceManagerErrorNoAvailableDevice,
    ///
    DeviceManagerErrorBasesIndex,
    ///
    DeviceManagerErrorParamIdNone,
    ///
    DeviceManagerSetDeviceError,
    ///
    DeviceManagerErrorGetDeviceInfo,
}