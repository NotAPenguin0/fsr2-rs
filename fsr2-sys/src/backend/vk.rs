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
pub struct VkInstance(u64);

impl VkInstance {
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
pub type VkFlags = u32;
pub type VkMemoryPropertyFlags = VkFlags;
pub type VkMemoryHeapFlags = VkFlags;
pub type VkDeviceSize = u64;
pub type VkPhysicalDeviceType = i32;

pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const VK_MAX_MEMORY_TYPES: usize = 32;
pub const VK_MAX_MEMORY_HEAPS: usize = 32;
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const VK_UUID_SIZE: usize = 16;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkExtensionProperties {
    pub extension_name: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub spec_version: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkMemoryType {
    pub property_flags: VkMemoryPropertyFlags,
    pub heap_index: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlags,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [VkMemoryType; VK_MAX_MEMORY_TYPES],
    pub memory_heap_count: u32,
    pub memory_heaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}

pub type VoidFunctionPtr = Option<unsafe extern "system" fn()>;

pub type VkEnumerateDeviceExtensionPropertiesFunc = unsafe extern "system" fn(VkPhysicalDevice, *const c_char, *mut u32, *mut VkExtensionProperties);

pub type VkGetDeviceProcAddrFunc =
    unsafe extern "system" fn(VkDevice, *const c_char) -> VoidFunctionPtr;

pub type VkGetPhysicalDeviceMemoryPropertiesFunc = unsafe extern "system" fn(VkPhysicalDevice, *mut VkPhysicalDeviceMemoryProperties);

/// fn(VkPhysicalDevice, *mut VkPhysicalDeviceProperties)
/// This type is not fully spelled out to avoid depending on ash or another vulkan loader for the vulkan types
pub type VkGetPhysicalDevicePropertiesFunc = unsafe extern "system" fn(VkPhysicalDevice, *mut c_void);

/// fn(VkPhysicalDevice, *mut VkPhysicalDeviceProperties2)
/// This type is not fully spelled out to avoid depending on ash or another vulkan loader for the vulkan types
pub type VkGetPhysicalDeviceProperties2Func = unsafe extern "system" fn(VkPhysicalDevice, *mut c_void);

/// fn(VkPhysicalDevice, *mut VkPhysicalDeviceFeatures2)
/// This type is not fully spelled out to avoid depending on ash or another vulkan loader for the vulkan types
pub type VkGetPhysicalDeviceFeatures2Func = unsafe extern "system" fn(VkPhysicalDevice, *mut c_void);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FfxFsr2InstanceFunctionPointerTableVk {
    pub fp_enumerate_device_extension_properties: VkEnumerateDeviceExtensionPropertiesFunc,
    pub fp_get_device_proc_addr: VkGetDeviceProcAddrFunc,
    pub fp_get_physical_device_memory_properties: VkGetPhysicalDeviceMemoryPropertiesFunc,
    pub fp_get_physical_device_properties: VkGetPhysicalDevicePropertiesFunc,
    pub fp_get_physical_device_properties2: VkGetPhysicalDeviceProperties2Func,
    pub fp_get_physical_device_features2: VkGetPhysicalDeviceFeatures2Func,
}

extern "C" {
    pub fn ffxFsr2GetScratchMemorySizeVK(device: VkPhysicalDevice, fp_table: *const FfxFsr2InstanceFunctionPointerTableVk) -> usize;

    pub fn ffxFsr2GetInterfaceVK(
        interface: *mut FfxFsr2Interface,
        scratch_buffer: *mut c_void,
        scratch_size: usize,
        device: VkPhysicalDevice,
        fp_table: *const FfxFsr2InstanceFunctionPointerTableVk,
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
