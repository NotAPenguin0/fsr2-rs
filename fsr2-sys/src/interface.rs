use std::ffi::c_void;
use crate::{FfxCommandList, FfxCreateResourceDescription, FfxDevice, FfxDeviceCapabilities, FfxErrorCode, FfxGpuJobDescription, FfxPipelineDescription, FfxPipelineState, FfxResource, FfxResourceDescription, FfxResourceInternal};
use widestring::WideChar as wchar_t;

#[repr(u8)]
pub enum FfxFsr2Pass {
    DepthClip = 0,
    ReconstructPreviousDepth = 1,
    Lock = 2,
    Accumulate = 3,
    AccumulateSharpen = 4,
    Rcas = 5,
    ComputeLuminancePyramid = 6,
    GenerateReactive = 7,
    TcrAutogenerate = 8,
}

impl FfxFsr2Pass {
    const COUNT: usize = 9;
}

#[repr(u8)]
pub enum FfxFsr2MsgType {
    Error = 0,
    Warning = 1,
}

impl FfxFsr2MsgType {
    const COUNT: usize = 2;
}

pub type FfxFsr2CreateBackendContextFunc = extern fn(*mut FfxFsr2Interface, FfxDevice) -> FfxErrorCode;
pub type FfxFsr2GetDeviceCapabilitiesFunc = extern fn(*mut FfxFsr2Interface, *mut FfxDeviceCapabilities, FfxDevice) -> FfxErrorCode;
pub type FfxFsr2DestroyBackendContextFunc = extern fn(*mut FfxFsr2Interface) -> FfxErrorCode;
pub type FfxFsr2CreateResourceFunc = extern fn(*mut FfxFsr2Interface, *const FfxCreateResourceDescription, *mut FfxResourceInternal) -> FfxErrorCode;
pub type FFxFsr2RegisterResourceFunc = extern fn(*mut FfxFsr2Interface, *const FfxResource, *mut FfxResourceInternal) -> FfxErrorCode;
pub type FfxFsr2UnregisterResourcesFunc = extern fn(*mut FfxFsr2Interface) -> FfxErrorCode;
pub type FfxFsr2GetResourceDescriptionFunc = extern fn(*mut FfxFsr2Interface, FfxResourceInternal) -> FfxResourceDescription;
pub type FfxFsr2DestroyResourceFunc = extern fn(*mut FfxFsr2Interface, FfxResourceInternal) -> FfxErrorCode;
pub type FfxFsr2CreatePipelineFunc = extern fn(*mut FfxFsr2Interface, FfxFsr2Pass, *const FfxPipelineDescription, *mut FfxPipelineState) -> FfxErrorCode;
pub type FfxFsr2DestroyPipelineFunc = extern fn(*mut FfxFsr2Interface, *mut FfxPipelineState) -> FfxErrorCode;
pub type FfxFsr2ScheduleGpuJobFunc = extern fn(*mut FfxFsr2Interface, *const FfxGpuJobDescription) -> FfxErrorCode;
pub type FfxFsr2ExecuteGpuJobsFunc = extern fn(*mut FfxFsr2Interface, FfxCommandList) -> FfxErrorCode;
pub type FfxFsr2Message = extern "system" fn(FfxFsr2MsgType, *const wchar_t);

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxFsr2Interface {
    pub fp_create_backend_context: FfxFsr2CreateBackendContextFunc,
    pub fp_get_device_capabilities: FfxFsr2GetDeviceCapabilitiesFunc,
    pub fp_destroy_backend_context: FfxFsr2DestroyBackendContextFunc,
    pub fp_create_resource: FfxFsr2CreateResourceFunc,
    pub fp_register_resource: FFxFsr2RegisterResourceFunc,
    pub fp_unregister_resources: FfxFsr2UnregisterResourcesFunc,
    pub fp_get_resource_description: FfxFsr2GetResourceDescriptionFunc,
    pub fp_destroy_resource: FfxFsr2DestroyResourceFunc,
    pub fp_create_pipeline: FfxFsr2CreatePipelineFunc,
    pub fp_destroy_pipeline: FfxFsr2DestroyPipelineFunc,
    pub fp_schedule_gpu_job: FfxFsr2ScheduleGpuJobFunc,
    pub fp_execute_gpu_jobs: FfxFsr2ExecuteGpuJobsFunc,

    pub scratch_buffer: *mut c_void,
    pub scratch_buffer_size: usize,
}