# FidelityFX Super Resolution 2.2 (FSR 2.2)

AMD FidelityFX Super Resolution 2 (FSR 2) is an open source, high-quality solution for producing high resolution frames from lower resolution inputs.

This crate provides unsafe, 1:1 Rust bindings for the FSR2 library. For documentation on how to use the library, please refer to
<https://github.com/GPUOpen-Effects/FidelityFX-FSR2>.

The crate uses a custom fork of FSR2 with minimal changes:
- Removed dependency on `cauldron-media` to avoid downloading unused sample assets
- Does not build sample backends by default
- Vulkan backend does not link to Vulkan at all, instead relies on provided function pointers on initialization.

# Known issues

- Bindings for the DirectX12 backend are currently missing.
- Computer can freeze for a couple seconds during build. This is due to FSR2 generating shader permutations
  using a bunch of threads.