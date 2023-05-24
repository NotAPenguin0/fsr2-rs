// FSR2 literals to define error constants are overflowing by default
#![allow(overflowing_literals)]

extern crate core;

pub mod error;
pub mod interface;
pub mod types;
pub mod backend;

use bitflags::bitflags;
pub use error::*;
pub use interface::*;
pub use types::*;
pub use backend::*;

pub const FFX_FSR2_VERSION_MAJOR: u32 = 2;
pub const FFX_FSR2_VERSION_MINOR: u32 = 2;
pub const FFX_FSR2_VERSION_PATCH: u32 = 0;

pub const FFX_FSR2_CONTEXT_SIZE: usize = 16536;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum FfxFsr2QualityMode {
    Quality = 1,
    Balanced = 2,
    Performance = 3,
    UltraPerformance = 4,
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    #[repr(transparent)]
    pub struct FfxFsr2InitializationFlagBits: i32 {
        const ENABLE_HIGH_DYNAMIC_RANGE = 1 << 0;
        const ENABLE_DISPLAY_RESOLUTION_MOTION_VECTORS = 1 << 1;
        const ENABLE_MOTION_VECTORS_JITTER_CANCELLATION = 1 << 2;
        const ENABLE_DEPTH_INVERTED = 1 << 3;
        const ENABLE_DEPTH_INFINITE = 1 << 4;
        const ENABLE_AUTO_EXPOSURE = 1 << 5;
        const ENABLE_DYNAMIC_RESOLUTION = 1 << 6;
        const ENABLE_TEXTURE1D_USAGE = 1 << 7;
        const ENABLE_DEBUG_CHECKING = 1 << 8;
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxFsr2ContextDescription {
    pub flags: FfxFsr2InitializationFlagBits,
    pub max_render_size: FfxDimensions2D,
    pub display_size: FfxDimensions2D,
    pub callbacks: FfxFsr2Interface,
    pub device: FfxDevice,
    pub fp_message: FfxFsr2Message,
}

unsafe impl Send for FfxFsr2ContextDescription {}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxFsr2DispatchDescription {
    pub command_list: FfxCommandList,
    pub color: FfxResource,
    pub depth: FfxResource,
    pub motion_vectors: FfxResource,
    pub exposure: FfxResource,
    pub reactive: FfxResource,
    pub transparency_and_composition: FfxResource,
    pub output: FfxResource,
    pub jitter_offset: FfxFloatCoords2D,
    pub motion_vector_scale: FfxFloatCoords2D,
    pub render_size: FfxDimensions2D,
    pub enable_sharpening: bool,
    pub sharpness: f32,
    /// Frametime delta in milliseconds
    pub frametime_delta: f32,
    pub pre_exposure: f32,
    pub reset: bool,
    pub camera_near: f32,
    pub camera_far: f32,
    pub camera_vertical_fov: f32,
    pub viewspace_to_meters_factor: f32,
    pub enable_auto_reactive: bool,
    pub color_opaque_only: FfxResource,
    pub auto_tc_threshold: f32,
    pub auto_tc_scale: f32,
    pub auto_reactive_scale: f32,
    pub auto_reactive_max: f32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxFsr2GenerateReactiveDescription {
    pub command_list: FfxCommandList,
    pub color_opaque_only: FfxResource,
    pub color_pre_upscale: FfxResource,
    pub out_reactive: FfxResource,
    pub render_size: FfxDimensions2D,
    pub scale: f32,
    pub cutoff_threshold: f32,
    pub binary_value: f32,
    pub flags: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FfxFsr2Context {
    pub data: [u32; FFX_FSR2_CONTEXT_SIZE],
}

extern "C" {
    pub fn ffxFsr2ContextCreate(
        context: *mut FfxFsr2Context,
        description: *const FfxFsr2ContextDescription,
    ) -> FfxErrorCode;

    pub fn ffxFsr2ContextDispatch(
        context: *mut FfxFsr2Context,
        description: *const FfxFsr2DispatchDescription,
    ) -> FfxErrorCode;

    pub fn ffxFsr2ContextGenerateReactiveMask(
        context: *mut FfxFsr2Context,
        params: *const FfxFsr2GenerateReactiveDescription,
    ) -> FfxErrorCode;

    pub fn ffxFsr2ContextDestroy(context: *mut FfxFsr2Context) -> FfxErrorCode;

    pub fn ffxFsr2GetUpscaleRatioFromQualityMode(quality_mode: FfxFsr2QualityMode) -> f32;

    pub fn ffxFsr2GetRenderResolutionFromQualityMode(
        render_width: *mut u32,
        render_height: *mut u32,
        display_width: u32,
        display_height: u32,
        quality_mode: FfxFsr2QualityMode,
    ) -> FfxErrorCode;

    pub fn ffxFsr2GetJitterPhaseCount(render_width: u32, render_height: u32) -> i32;

    pub fn ffxFsr2GetJitterOffset(
        out_x: *mut f32,
        out_y: *mut f32,
        index: i32,
        phase_count: u32,
    ) -> FfxErrorCode;

    pub fn ffxFsr2ResourceIsNull(resource: FfxResource) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::{ffxFsr2GetUpscaleRatioFromQualityMode, FfxFsr2QualityMode};

    #[test]
    pub fn test_linkage() {
        let ratio = unsafe { ffxFsr2GetUpscaleRatioFromQualityMode(FfxFsr2QualityMode::Balanced) };
        println!("ratio: {ratio}");
    }
}
