//! Vulkan format definitions for KTX2 textures.
//!
//! This module defines the Vulkan format enum that corresponds to the
//! VkFormat values used in the KTX2 library.

/// Vulkan format enum
///
/// This represents the VkFormat values from the Vulkan specification.
/// Only the most common formats are included here for brevity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum VkFormat {
    /// Undefined format
    Undefined = 0,

    /// 8-bit R component, unsigned normalized
    R8Unorm = 9,

    /// 8-bit R and G components, unsigned normalized
    R8G8Unorm = 16,

    /// 8-bit R, G and B components, unsigned normalized
    R8G8B8Unorm = 23,

    /// 8-bit R, G, B and A components, unsigned normalized
    R8G8B8A8Unorm = 37,

    /// 8-bit R, G, B and A components, sRGB
    R8G8B8A8Srgb = 43,

    /// 8-bit B, G and R components, unsigned normalized
    B8G8R8Unorm = 30,

    /// 8-bit B, G, R and A components, unsigned normalized
    B8G8R8A8Unorm = 44,

    /// 8-bit B, G, R and A components, sRGB
    B8G8R8A8Srgb = 50,

    /// 32-bit R component, signed float
    R32Sfloat = 100,

    /// 32-bit R and G components, signed float
    R32G32Sfloat = 103,

    /// 32-bit R, G, B and A components, signed float
    R32G32B32A32Sfloat = 109,

    /// 16-bit R component, signed float
    R16Sfloat = 70,

    /// 16-bit R and G components, signed float
    R16G16Sfloat = 73,

    /// 16-bit R, G, B and A components, signed float
    R16G16B16A16Sfloat = 97,

    /// BC1 compressed format (DXT1)
    Bc1RgbUnormBlock = 131,

    /// BC1 compressed format with alpha (DXT1)
    Bc1RgbaUnormBlock = 132,

    /// BC1 compressed format with alpha (DXT1), sRGB
    Bc1RgbaSrgbBlock = 134,

    /// BC3 compressed format (DXT5)
    Bc3UnormBlock = 136,

    /// BC3 compressed format (DXT5), sRGB
    Bc3SrgbBlock = 138,

    /// BC4 compressed format (unsigned)
    Bc4UnormBlock = 137,

    /// BC5 compressed format (unsigned)
    Bc5UnormBlock = 140,

    /// BC7 compressed format
    Bc7UnormBlock = 145,

    /// BC7 compressed format (sRGB)
    Bc7SrgbBlock = 146,

    /// ETC2 compressed format (RGB)
    Etc2R8G8B8UnormBlock = 147,

    /// ETC2 compressed format (RGB, sRGB)
    Etc2R8G8B8SrgbBlock = 148,

    /// ETC2 compressed format with alpha
    Etc2R8G8B8A1UnormBlock = 149,

    /// ETC2 compressed format with alpha (sRGB)
    Etc2R8G8B8A1SrgbBlock = 150,

    /// ETC2 compressed format with EAC alpha
    Etc2R8G8B8A8UnormBlock = 151,

    /// ETC2 compressed format with EAC alpha (sRGB)
    Etc2R8G8B8A8SrgbBlock = 152,

    /// ASTC 4x4 compressed format
    Astc4x4UnormBlock = 157,

    /// ASTC 4x4 compressed format (sRGB)
    Astc4x4SrgbBlock = 158,

    /// ASTC 8x8 compressed format
    Astc8x8UnormBlock = 165,

    /// ASTC 8x8 compressed format (sRGB)
    Astc8x8SrgbBlock = 172,
}

impl VkFormat {
    /// Get the raw VkFormat value
    pub fn as_raw(&self) -> u32 {
        *self as u32
    }

    /// Create a VkFormat from a raw value
    ///
    /// Returns None if the value doesn't correspond to a known format
    pub fn from_raw(value: u32) -> Option<Self> {
        match value {
            0 => Some(VkFormat::Undefined),
            9 => Some(VkFormat::R8Unorm),
            16 => Some(VkFormat::R8G8Unorm),
            23 => Some(VkFormat::R8G8B8Unorm),
            37 => Some(VkFormat::R8G8B8A8Unorm),
            43 => Some(VkFormat::R8G8B8A8Srgb),
            30 => Some(VkFormat::B8G8R8Unorm),
            44 => Some(VkFormat::B8G8R8A8Unorm),
            50 => Some(VkFormat::B8G8R8A8Srgb),
            100 => Some(VkFormat::R32Sfloat),
            103 => Some(VkFormat::R32G32Sfloat),
            109 => Some(VkFormat::R32G32B32A32Sfloat),
            70 => Some(VkFormat::R16Sfloat),
            73 => Some(VkFormat::R16G16Sfloat),
            97 => Some(VkFormat::R16G16B16A16Sfloat),
            131 => Some(VkFormat::Bc1RgbUnormBlock),
            132 => Some(VkFormat::Bc1RgbaUnormBlock),
            134 => Some(VkFormat::Bc1RgbaSrgbBlock),
            136 => Some(VkFormat::Bc3UnormBlock),
            138 => Some(VkFormat::Bc3SrgbBlock),
            137 => Some(VkFormat::Bc4UnormBlock),
            140 => Some(VkFormat::Bc5UnormBlock),
            145 => Some(VkFormat::Bc7UnormBlock),
            146 => Some(VkFormat::Bc7SrgbBlock),
            147 => Some(VkFormat::Etc2R8G8B8UnormBlock),
            148 => Some(VkFormat::Etc2R8G8B8SrgbBlock),
            149 => Some(VkFormat::Etc2R8G8B8A1UnormBlock),
            150 => Some(VkFormat::Etc2R8G8B8A1SrgbBlock),
            151 => Some(VkFormat::Etc2R8G8B8A8UnormBlock),
            152 => Some(VkFormat::Etc2R8G8B8A8SrgbBlock),
            157 => Some(VkFormat::Astc4x4UnormBlock),
            158 => Some(VkFormat::Astc4x4SrgbBlock),
            165 => Some(VkFormat::Astc8x8UnormBlock),
            172 => Some(VkFormat::Astc8x8SrgbBlock),
            _ => None,
        }
    }
}

impl From<VkFormat> for u32 {
    fn from(format: VkFormat) -> Self {
        format.as_raw()
    }
}

impl TryFrom<u32> for VkFormat {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        VkFormat::from_raw(value).ok_or(())
    }
}
