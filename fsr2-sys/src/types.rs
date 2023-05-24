use bitflags::{bitflags, Flags};
use std::ffi::c_void;
use widestring::WideChar;

#[allow(nonstandard_style)]
type wchar_t = WideChar;

pub const FFX_MAX_NUM_SRVS: usize = 16;
pub const FFX_MAX_NUM_UAVS: usize = 8;
pub const FFX_MAX_NUM_CONST_BUFFERS: usize = 2;
pub const FFX_MAX_CONST_SIZE: usize = 64;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum FfxSurfaceFormat {
    Unknown = 0,
    RGBA32Typeless = 1,
    RGBA32Float = 2,
    RGBA16Float = 3,
    RGBA16Unorm = 4,
    RG32Float = 5,
    R32Uint = 6,
    RGBA8Typeless = 7,
    RGBA8Unorm = 8,
    R11G11B11Float = 9,
    RG16Float = 10,
    RG16Uint = 11,
    R16Float = 12,
    R16Uint = 13,
    R16Unorm = 14,
    R16Snorm = 15,
    R8Unorm = 16,
    R8Uint = 17,
    RG8Unorm = 18,
    R32Float = 19,
}

bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    #[repr(transparent)]
    pub struct FfxResourceUsage: i32 {
        const READ_ONLY = 0;
        const RENDERTARGET = 1 << 0;
        const UAV = 1 << 1;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    #[repr(transparent)]
    pub struct FfxResourceState: i32 {
        const UNORDERED_ACCESS = 1 << 0;
        const COMPUTE_READ = 1 << 1;
        const COPY_SRC = 1 << 2;
        const COPY_DEST = 1 << 3;
        const GENERIC_READ = Self::COPY_SRC.bits() | Self::COMPUTE_READ.bits();
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum FfxResourceDimension {
    Texture1D = 0,
    Texture2D = 1,
}

bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    #[repr(transparent)]
    pub struct FfxResourceFlags: i32 {
        const NONE = 0;
        const ALIASABLE = 1 << 0;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum FfxResourceViewType {
    UnorderedAccess = 0,
    ShaderRead = 1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum FfxFilterType {
    Point = 0,
    Linear = 1,
}

#[allow(nonstandard_style)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum FfxShaderModel {
    ShaderModel_5_1 = 0,
    ShaderModel_6_0 = 1,
    ShaderModel_6_1 = 2,
    ShaderModel_6_2 = 3,
    ShaderModel_6_3 = 4,
    ShaderModel_6_4 = 5,
    ShaderModel_6_5 = 6,
    ShaderModel_6_6 = 7,
    ShaderModel_6_7 = 8,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum FfxResourceType {
    Buffer = 0,
    Texture1D = 1,
    Texture2D = 2,
    Texture3D = 3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum FfxHeapType {
    Default = 0,
    Upload = 1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum FfxJobType {
    ClearFloat = 0,
    Copy = 1,
    Compute = 2,
}

pub type FfxDevice = *mut c_void;
pub type FfxCommandList = *mut c_void;
pub type FfxRootSignature = *mut c_void;
pub type FfxPipeline = *mut c_void;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxDeviceCapabilities {
    pub minimum_supported_shader_model: FfxShaderModel,
    pub wave_lane_count_min: u32,
    pub wave_lane_count_max: u32,
    pub fp16_supported: bool,
    pub raytracing_supported: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct FfxDimensions2D {
    pub width: u32,
    pub height: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct FfxIntCoords2D {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct FfxFloatCoords2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxResourceDescription {
    pub ty: FfxResourceType,
    pub surface_format: FfxSurfaceFormat,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_count: u32,
    pub flags: FfxResourceFlags,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxResource {
    pub resource: *mut c_void,
    pub name: [wchar_t; 64],
    pub description: FfxResourceDescription,
    pub state: FfxResourceState,
    pub is_depth: bool,
    pub descriptor_data: u64,
}

impl FfxResource {
    pub const NULL: Self = FfxResource {
        resource: std::ptr::null_mut(),
        name: [0; 64],
        description: FfxResourceDescription {
            ty: FfxResourceType::Buffer,
            surface_format: FfxSurfaceFormat::Unknown,
            width: 0,
            height: 0,
            depth: 0,
            mip_count: 0,
            flags: FfxResourceFlags::NONE,
        },
        state: FfxResourceState::from_bits_retain(0),
        is_depth: false,
        descriptor_data: 0,
    };
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxResourceInternal {
    pub internal_index: i32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxResourceBinding {
    pub slot_index: u32,
    pub resource_identifier: u32,
    pub name: [wchar_t; 64],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxPipelineState {
    pub root_signature: FfxRootSignature,
    pub pipeline: FfxPipeline,
    pub uav_count: u32,
    pub srv_count: u32,
    pub const_count: u32,

    pub uav_resource_bindings: [FfxResourceBinding; FFX_MAX_NUM_UAVS],
    pub srv_resource_bindings: [FfxResourceBinding; FFX_MAX_NUM_SRVS],
    pub cv_resource_bindings: [FfxResourceBinding; FFX_MAX_NUM_CONST_BUFFERS],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxCreateResourceDescription {
    pub heap_type: FfxHeapType,
    pub resource_description: FfxResourceDescription,
    pub initial_state: FfxResourceState,
    pub init_data_size: u32,
    pub init_data: *mut c_void,
    pub name: *const wchar_t,
    pub usage: FfxResourceUsage,
    pub id: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxPipelineDescription {
    pub context_flags: u32,
    pub samplers: *mut FfxFilterType,
    pub sampler_count: usize,
    pub root_constant_buffer_size: *const u32,
    pub root_constant_buffer_count: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxConstantBuffer {
    pub uint32_size: u32,
    pub data: [u32; FFX_MAX_CONST_SIZE],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxClearFloatJobDescription {
    pub color: [f32; 4],
    pub target: FfxResourceInternal,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxComputeJobDescription {
    pub pipeline: FfxPipeline,
    pub dimensions: [u32; 3],
    pub srvs: [FfxResourceInternal; FFX_MAX_NUM_SRVS],
    pub srv_names: [[wchar_t; 64]; FFX_MAX_NUM_SRVS],
    pub uavs: [FfxResourceInternal; FFX_MAX_NUM_UAVS],
    pub uav_mip: [u32; FFX_MAX_NUM_UAVS],
    pub uav_names: [[wchar_t; 64]; FFX_MAX_NUM_UAVS],
    pub cbs: [FfxConstantBuffer; FFX_MAX_NUM_CONST_BUFFERS],
    pub cb_names: [[wchar_t; 64]; FFX_MAX_NUM_CONST_BUFFERS],
    pub cb_slot_index: [u32; FFX_MAX_NUM_CONST_BUFFERS],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxCopyJobDescription {
    pub src: FfxResourceInternal,
    pub dst: FfxResourceInternal,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union FfxGpuJob {
    pub clear_job: FfxClearFloatJobDescription,
    pub copy_job: FfxCopyJobDescription,
    pub compute_job: FfxComputeJobDescription,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FfxGpuJobDescription {
    pub ty: FfxJobType,
    pub descriptor: FfxGpuJob,
}
