use crate::{
    FfxCommandList, FfxDevice, FfxErrorCode, FfxFsr2Context, FfxFsr2Interface, FfxResource,
    FfxResourceState,
};
use std::ffi::{c_char, c_void};
use widestring::WideChar as wchar_t;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct VkPhysicalDevice(u64);

impl VkPhysicalDevice {
    pub fn from_raw(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct VkDevice(u64);

impl VkDevice {
    pub fn from_raw(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct VkCommandBuffer(u64);

impl VkCommandBuffer {
    pub fn from_raw(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct VkImage(u64);

impl VkImage {
    pub fn from_raw(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct VkImageView(u64);

impl VkImageView {
    pub fn from_raw(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct VkBuffer(u64);

impl VkBuffer {
    pub fn from_raw(value: u64) -> Self {
        Self(value)
    }
}

pub type VkFormat = i32;
pub type VkImageLayout = i32;

pub type VoidFunctionPtr = Option<unsafe extern "system" fn()>;
pub type VkGetDeviceProcAddrFunc = unsafe extern "system" fn(VkDevice, *const c_char) -> VoidFunctionPtr;

extern "C" {
    pub fn ffxFsr2GetScratchMemorySizeVK(device: VkPhysicalDevice) -> usize;

    pub fn ffxFsr2GetInterfaceVK(
        interface: *mut FfxFsr2Interface,
        scratch_buffer: *mut c_void,
        scratch_size: usize,
        device: VkPhysicalDevice,
        fp_get_device_proc_addr: VkGetDeviceProcAddrFunc,
    ) -> FfxErrorCode;

    pub fn ffxGetDeviceVK(device: VkDevice) -> FfxDevice;

    pub fn ffxGetCommandListVK(cmd: VkCommandBuffer) -> FfxCommandList;

    /// Name may be null
    pub fn ffxGetTextureResourceVK(
        image: VkImage,
        view: VkImageView,
        width: u32,
        height: u32,
        format: VkFormat,
        name: *const wchar_t,
        state: FfxResourceState,
    ) -> FfxResource;

    /// Name may be null
    pub fn ffxGetBufferResourceVK(
        buffer: VkBuffer,
        size: u32,
        name: *const wchar_t,
        state: FfxResourceState,
    ) -> FfxResource;

    pub fn ffxGetVkImage(context: *mut FfxFsr2Context, id: u32) -> VkImage;

    pub fn ffxGetVkImageView(context: *mut FfxFsr2Context, id: u32) -> VkImageView;

    pub fn ffxGetVkImageLayout(context: *mut FfxFsr2Context, id: u32) -> VkImageLayout;
}
