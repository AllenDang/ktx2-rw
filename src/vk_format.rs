//! Vulkan format definitions for KTX2 textures.
//!
//! This module defines the Vulkan format enum that corresponds to the
//! VkFormat values used in the Vulkan specification, providing comprehensive
//! coverage of all standard Vulkan formats.

/// Vulkan format enum
///
/// This represents the VkFormat values from the Vulkan specification.
/// Includes all standard formats from undefined (0) to ASTC 12x12 sRGB (184).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum VkFormat {
    /// Undefined format
    Undefined = 0,

    /// 4-bit R and G components, unsigned normalized, packed into 8 bits
    R4G4UnormPack8 = 1,

    /// 4-bit R, G, B and A components, unsigned normalized, packed into 16 bits
    R4G4B4A4UnormPack16 = 2,

    /// 4-bit B, G, R and A components, unsigned normalized, packed into 16 bits
    B4G4R4A4UnormPack16 = 3,

    /// 5-bit R and B components, 6-bit G component, unsigned normalized, packed into 16 bits
    R5G6B5UnormPack16 = 4,

    /// 5-bit B and R components, 6-bit G component, unsigned normalized, packed into 16 bits
    B5G6R5UnormPack16 = 5,

    /// 5-bit R, G and B components, 1-bit A component, unsigned normalized, packed into 16 bits
    R5G5B5A1UnormPack16 = 6,

    /// 5-bit B, G and R components, 1-bit A component, unsigned normalized, packed into 16 bits
    B5G5R5A1UnormPack16 = 7,

    /// 1-bit A component, 5-bit R, G and B components, unsigned normalized, packed into 16 bits
    A1R5G5B5UnormPack16 = 8,

    /// 8-bit R component, unsigned normalized
    R8Unorm = 9,

    /// 8-bit R component, signed normalized
    R8Snorm = 10,

    /// 8-bit R component, unsigned scaled
    R8Uscaled = 11,

    /// 8-bit R component, signed scaled
    R8Sscaled = 12,

    /// 8-bit R component, unsigned integer
    R8Uint = 13,

    /// 8-bit R component, signed integer
    R8Sint = 14,

    /// 8-bit R component, sRGB
    R8Srgb = 15,

    /// 8-bit R and G components, unsigned normalized
    R8G8Unorm = 16,

    /// 8-bit R and G components, signed normalized
    R8G8Snorm = 17,

    /// 8-bit R and G components, unsigned scaled
    R8G8Uscaled = 18,

    /// 8-bit R and G components, signed scaled
    R8G8Sscaled = 19,

    /// 8-bit R and G components, unsigned integer
    R8G8Uint = 20,

    /// 8-bit R and G components, signed integer
    R8G8Sint = 21,

    /// 8-bit R and G components, sRGB
    R8G8Srgb = 22,

    /// 8-bit R, G and B components, unsigned normalized
    R8G8B8Unorm = 23,

    /// 8-bit R, G and B components, signed normalized
    R8G8B8Snorm = 24,

    /// 8-bit R, G and B components, unsigned scaled
    R8G8B8Uscaled = 25,

    /// 8-bit R, G and B components, signed scaled
    R8G8B8Sscaled = 26,

    /// 8-bit R, G and B components, unsigned integer
    R8G8B8Uint = 27,

    /// 8-bit R, G and B components, signed integer
    R8G8B8Sint = 28,

    /// 8-bit R, G and B components, sRGB
    R8G8B8Srgb = 29,

    /// 8-bit B, G and R components, unsigned normalized
    B8G8R8Unorm = 30,

    /// 8-bit B, G and R components, signed normalized
    B8G8R8Snorm = 31,

    /// 8-bit B, G and R components, unsigned scaled
    B8G8R8Uscaled = 32,

    /// 8-bit B, G and R components, signed scaled
    B8G8R8Sscaled = 33,

    /// 8-bit B, G and R components, unsigned integer
    B8G8R8Uint = 34,

    /// 8-bit B, G and R components, signed integer
    B8G8R8Sint = 35,

    /// 8-bit B, G and R components, sRGB
    B8G8R8Srgb = 36,

    /// 8-bit R, G, B and A components, unsigned normalized
    R8G8B8A8Unorm = 37,

    /// 8-bit R, G, B and A components, signed normalized
    R8G8B8A8Snorm = 38,

    /// 8-bit R, G, B and A components, unsigned scaled
    R8G8B8A8Uscaled = 39,

    /// 8-bit R, G, B and A components, signed scaled
    R8G8B8A8Sscaled = 40,

    /// 8-bit R, G, B and A components, unsigned integer
    R8G8B8A8Uint = 41,

    /// 8-bit R, G, B and A components, signed integer
    R8G8B8A8Sint = 42,

    /// 8-bit R, G, B and A components, sRGB
    R8G8B8A8Srgb = 43,

    /// 8-bit B, G, R and A components, unsigned normalized
    B8G8R8A8Unorm = 44,

    /// 8-bit B, G, R and A components, signed normalized
    B8G8R8A8Snorm = 45,

    /// 8-bit B, G, R and A components, unsigned scaled
    B8G8R8A8Uscaled = 46,

    /// 8-bit B, G, R and A components, signed scaled
    B8G8R8A8Sscaled = 47,

    /// 8-bit B, G, R and A components, unsigned integer
    B8G8R8A8Uint = 48,

    /// 8-bit B, G, R and A components, signed integer
    B8G8R8A8Sint = 49,

    /// 8-bit B, G, R and A components, sRGB
    B8G8R8A8Srgb = 50,

    /// 8-bit A, B, G and R components, unsigned normalized, packed into 32 bits
    A8B8G8R8UnormPack32 = 51,

    /// 8-bit A, B, G and R components, signed normalized, packed into 32 bits
    A8B8G8R8SnormPack32 = 52,

    /// 8-bit A, B, G and R components, unsigned scaled, packed into 32 bits
    A8B8G8R8UscaledPack32 = 53,

    /// 8-bit A, B, G and R components, signed scaled, packed into 32 bits
    A8B8G8R8SscaledPack32 = 54,

    /// 8-bit A, B, G and R components, unsigned integer, packed into 32 bits
    A8B8G8R8UintPack32 = 55,

    /// 8-bit A, B, G and R components, signed integer, packed into 32 bits
    A8B8G8R8SintPack32 = 56,

    /// 8-bit A, B, G and R components, sRGB, packed into 32 bits
    A8B8G8R8SrgbPack32 = 57,

    /// 2-bit A component, 10-bit R, G and B components, unsigned normalized, packed into 32 bits
    A2R10G10B10UnormPack32 = 58,

    /// 2-bit A component, 10-bit R, G and B components, signed normalized, packed into 32 bits
    A2R10G10B10SnormPack32 = 59,

    /// 2-bit A component, 10-bit R, G and B components, unsigned scaled, packed into 32 bits
    A2R10G10B10UscaledPack32 = 60,

    /// 2-bit A component, 10-bit R, G and B components, signed scaled, packed into 32 bits
    A2R10G10B10SscaledPack32 = 61,

    /// 2-bit A component, 10-bit R, G and B components, unsigned integer, packed into 32 bits
    A2R10G10B10UintPack32 = 62,

    /// 2-bit A component, 10-bit R, G and B components, signed integer, packed into 32 bits
    A2R10G10B10SintPack32 = 63,

    /// 2-bit A component, 10-bit B, G and R components, unsigned normalized, packed into 32 bits
    A2B10G10R10UnormPack32 = 64,

    /// 2-bit A component, 10-bit B, G and R components, signed normalized, packed into 32 bits
    A2B10G10R10SnormPack32 = 65,

    /// 2-bit A component, 10-bit B, G and R components, unsigned scaled, packed into 32 bits
    A2B10G10R10UscaledPack32 = 66,

    /// 2-bit A component, 10-bit B, G and R components, signed scaled, packed into 32 bits
    A2B10G10R10SscaledPack32 = 67,

    /// 2-bit A component, 10-bit B, G and R components, unsigned integer, packed into 32 bits
    A2B10G10R10UintPack32 = 68,

    /// 2-bit A component, 10-bit B, G and R components, signed integer, packed into 32 bits
    A2B10G10R10SintPack32 = 69,

    /// 16-bit R component, unsigned normalized
    R16Unorm = 70,

    /// 16-bit R component, signed normalized
    R16Snorm = 71,

    /// 16-bit R component, unsigned scaled
    R16Uscaled = 72,

    /// 16-bit R component, signed scaled
    R16Sscaled = 73,

    /// 16-bit R component, unsigned integer
    R16Uint = 74,

    /// 16-bit R component, signed integer
    R16Sint = 75,

    /// 16-bit R component, signed float
    R16Sfloat = 76,

    /// 16-bit R and G components, unsigned normalized
    R16G16Unorm = 77,

    /// 16-bit R and G components, signed normalized
    R16G16Snorm = 78,

    /// 16-bit R and G components, unsigned scaled
    R16G16Uscaled = 79,

    /// 16-bit R and G components, signed scaled
    R16G16Sscaled = 80,

    /// 16-bit R and G components, unsigned integer
    R16G16Uint = 81,

    /// 16-bit R and G components, signed integer
    R16G16Sint = 82,

    /// 16-bit R and G components, signed float
    R16G16Sfloat = 83,

    /// 16-bit R, G and B components, unsigned normalized
    R16G16B16Unorm = 84,

    /// 16-bit R, G and B components, signed normalized
    R16G16B16Snorm = 85,

    /// 16-bit R, G and B components, unsigned scaled
    R16G16B16Uscaled = 86,

    /// 16-bit R, G and B components, signed scaled
    R16G16B16Sscaled = 87,

    /// 16-bit R, G and B components, unsigned integer
    R16G16B16Uint = 88,

    /// 16-bit R, G and B components, signed integer
    R16G16B16Sint = 89,

    /// 16-bit R, G and B components, signed float
    R16G16B16Sfloat = 90,

    /// 16-bit R, G, B and A components, unsigned normalized
    R16G16B16A16Unorm = 91,

    /// 16-bit R, G, B and A components, signed normalized
    R16G16B16A16Snorm = 92,

    /// 16-bit R, G, B and A components, unsigned scaled
    R16G16B16A16Uscaled = 93,

    /// 16-bit R, G, B and A components, signed scaled
    R16G16B16A16Sscaled = 94,

    /// 16-bit R, G, B and A components, unsigned integer
    R16G16B16A16Uint = 95,

    /// 16-bit R, G, B and A components, signed integer
    R16G16B16A16Sint = 96,

    /// 16-bit R, G, B and A components, signed float
    R16G16B16A16Sfloat = 97,

    /// 32-bit R component, unsigned integer
    R32Uint = 98,

    /// 32-bit R component, signed integer
    R32Sint = 99,

    /// 32-bit R component, signed float
    R32Sfloat = 100,

    /// 32-bit R and G components, unsigned integer
    R32G32Uint = 101,

    /// 32-bit R and G components, signed integer
    R32G32Sint = 102,

    /// 32-bit R and G components, signed float
    R32G32Sfloat = 103,

    /// 32-bit R, G and B components, unsigned integer
    R32G32B32Uint = 104,

    /// 32-bit R, G and B components, signed integer
    R32G32B32Sint = 105,

    /// 32-bit R, G and B components, signed float
    R32G32B32Sfloat = 106,

    /// 32-bit R, G, B and A components, unsigned integer
    R32G32B32A32Uint = 107,

    /// 32-bit R, G, B and A components, signed integer
    R32G32B32A32Sint = 108,

    /// 32-bit R, G, B and A components, signed float
    R32G32B32A32Sfloat = 109,

    /// 64-bit R component, unsigned integer
    R64Uint = 110,

    /// 64-bit R component, signed integer
    R64Sint = 111,

    /// 64-bit R component, signed float
    R64Sfloat = 112,

    /// 64-bit R and G components, unsigned integer
    R64G64Uint = 113,

    /// 64-bit R and G components, signed integer
    R64G64Sint = 114,

    /// 64-bit R and G components, signed float
    R64G64Sfloat = 115,

    /// 64-bit R, G and B components, unsigned integer
    R64G64B64Uint = 116,

    /// 64-bit R, G and B components, signed integer
    R64G64B64Sint = 117,

    /// 64-bit R, G and B components, signed float
    R64G64B64Sfloat = 118,

    /// 64-bit R, G, B and A components, unsigned integer
    R64G64B64A64Uint = 119,

    /// 64-bit R, G, B and A components, signed integer
    R64G64B64A64Sint = 120,

    /// 64-bit R, G, B and A components, signed float
    R64G64B64A64Sfloat = 121,

    /// 10-bit B and G components, 11-bit R component, unsigned float, packed into 32 bits
    B10G11R11UfloatPack32 = 122,

    /// 9-bit mantissa for each component, 5-bit shared exponent, unsigned float, packed into 32 bits
    E5B9G9R9UfloatPack32 = 123,

    /// 16-bit D component, unsigned normalized
    D16Unorm = 124,

    /// 24-bit X component, 8-bit D component, unsigned normalized, packed into 32 bits
    X8D24UnormPack32 = 125,

    /// 32-bit D component, signed float
    D32Sfloat = 126,

    /// 8-bit S component, unsigned integer
    S8Uint = 127,

    /// 16-bit D component, unsigned normalized, 8-bit S component, unsigned integer
    D16UnormS8Uint = 128,

    /// 24-bit D component, unsigned normalized, 8-bit S component, unsigned integer, packed into 32 bits
    D24UnormS8Uint = 129,

    /// 32-bit D component, signed float, 8-bit S component, unsigned integer
    D32SfloatS8Uint = 130,

    /// BC1 compressed format (RGB, unsigned normalized)
    Bc1RgbUnormBlock = 131,

    /// BC1 compressed format (RGB, sRGB)
    Bc1RgbSrgbBlock = 132,

    /// BC1 compressed format (RGBA, unsigned normalized)
    Bc1RgbaUnormBlock = 133,

    /// BC1 compressed format (RGBA, sRGB)
    Bc1RgbaSrgbBlock = 134,

    /// BC2 compressed format (unsigned normalized)
    Bc2UnormBlock = 135,

    /// BC2 compressed format (sRGB)
    Bc2SrgbBlock = 136,

    /// BC3 compressed format (DXT5, unsigned normalized)
    Bc3UnormBlock = 137,

    /// BC3 compressed format (DXT5, sRGB)
    Bc3SrgbBlock = 138,

    /// BC4 compressed format (unsigned normalized)
    Bc4UnormBlock = 139,

    /// BC4 compressed format (signed normalized)
    Bc4SnormBlock = 140,

    /// BC5 compressed format (unsigned normalized)
    Bc5UnormBlock = 141,

    /// BC5 compressed format (signed normalized)
    Bc5SnormBlock = 142,

    /// BC6H compressed format (unsigned float)
    Bc6hUfloatBlock = 143,

    /// BC6H compressed format (signed float)
    Bc6hSfloatBlock = 144,

    /// BC7 compressed format (unsigned normalized)
    Bc7UnormBlock = 145,

    /// BC7 compressed format (sRGB)
    Bc7SrgbBlock = 146,

    /// ETC2 compressed format (RGB, unsigned normalized)
    Etc2R8G8B8UnormBlock = 147,

    /// ETC2 compressed format (RGB, sRGB)
    Etc2R8G8B8SrgbBlock = 148,

    /// ETC2 compressed format (RGB with 1-bit alpha, unsigned normalized)
    Etc2R8G8B8A1UnormBlock = 149,

    /// ETC2 compressed format (RGB with 1-bit alpha, sRGB)
    Etc2R8G8B8A1SrgbBlock = 150,

    /// ETC2 compressed format (RGBA with EAC alpha, unsigned normalized)
    Etc2R8G8B8A8UnormBlock = 151,

    /// ETC2 compressed format (RGBA with EAC alpha, sRGB)
    Etc2R8G8B8A8SrgbBlock = 152,

    /// EAC compressed format (R11, unsigned normalized)
    EacR11UnormBlock = 153,

    /// EAC compressed format (R11, signed normalized)
    EacR11SnormBlock = 154,

    /// EAC compressed format (R11G11, unsigned normalized)
    EacR11G11UnormBlock = 155,

    /// EAC compressed format (R11G11, signed normalized)
    EacR11G11SnormBlock = 156,

    /// ASTC 4x4 compressed format (unsigned normalized)
    Astc4x4UnormBlock = 157,

    /// ASTC 4x4 compressed format (sRGB)
    Astc4x4SrgbBlock = 158,

    /// ASTC 5x4 compressed format (unsigned normalized)
    Astc5x4UnormBlock = 159,

    /// ASTC 5x4 compressed format (sRGB)
    Astc5x4SrgbBlock = 160,

    /// ASTC 5x5 compressed format (unsigned normalized)
    Astc5x5UnormBlock = 161,

    /// ASTC 5x5 compressed format (sRGB)
    Astc5x5SrgbBlock = 162,

    /// ASTC 6x5 compressed format (unsigned normalized)
    Astc6x5UnormBlock = 163,

    /// ASTC 6x5 compressed format (sRGB)
    Astc6x5SrgbBlock = 164,

    /// ASTC 6x6 compressed format (unsigned normalized)
    Astc6x6UnormBlock = 165,

    /// ASTC 6x6 compressed format (sRGB)
    Astc6x6SrgbBlock = 166,

    /// ASTC 8x5 compressed format (unsigned normalized)
    Astc8x5UnormBlock = 167,

    /// ASTC 8x5 compressed format (sRGB)
    Astc8x5SrgbBlock = 168,

    /// ASTC 8x6 compressed format (unsigned normalized)
    Astc8x6UnormBlock = 169,

    /// ASTC 8x6 compressed format (sRGB)
    Astc8x6SrgbBlock = 170,

    /// ASTC 8x8 compressed format (unsigned normalized)
    Astc8x8UnormBlock = 171,

    /// ASTC 8x8 compressed format (sRGB)
    Astc8x8SrgbBlock = 172,

    /// ASTC 10x5 compressed format (unsigned normalized)
    Astc10x5UnormBlock = 173,

    /// ASTC 10x5 compressed format (sRGB)
    Astc10x5SrgbBlock = 174,

    /// ASTC 10x6 compressed format (unsigned normalized)
    Astc10x6UnormBlock = 175,

    /// ASTC 10x6 compressed format (sRGB)
    Astc10x6SrgbBlock = 176,

    /// ASTC 10x8 compressed format (unsigned normalized)
    Astc10x8UnormBlock = 177,

    /// ASTC 10x8 compressed format (sRGB)
    Astc10x8SrgbBlock = 178,

    /// ASTC 10x10 compressed format (unsigned normalized)
    Astc10x10UnormBlock = 179,

    /// ASTC 10x10 compressed format (sRGB)
    Astc10x10SrgbBlock = 180,

    /// ASTC 12x10 compressed format (unsigned normalized)
    Astc12x10UnormBlock = 181,

    /// ASTC 12x10 compressed format (sRGB)
    Astc12x10SrgbBlock = 182,

    /// ASTC 12x12 compressed format (unsigned normalized)
    Astc12x12UnormBlock = 183,

    /// ASTC 12x12 compressed format (sRGB)
    Astc12x12SrgbBlock = 184,
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
            1 => Some(VkFormat::R4G4UnormPack8),
            2 => Some(VkFormat::R4G4B4A4UnormPack16),
            3 => Some(VkFormat::B4G4R4A4UnormPack16),
            4 => Some(VkFormat::R5G6B5UnormPack16),
            5 => Some(VkFormat::B5G6R5UnormPack16),
            6 => Some(VkFormat::R5G5B5A1UnormPack16),
            7 => Some(VkFormat::B5G5R5A1UnormPack16),
            8 => Some(VkFormat::A1R5G5B5UnormPack16),
            9 => Some(VkFormat::R8Unorm),
            10 => Some(VkFormat::R8Snorm),
            11 => Some(VkFormat::R8Uscaled),
            12 => Some(VkFormat::R8Sscaled),
            13 => Some(VkFormat::R8Uint),
            14 => Some(VkFormat::R8Sint),
            15 => Some(VkFormat::R8Srgb),
            16 => Some(VkFormat::R8G8Unorm),
            17 => Some(VkFormat::R8G8Snorm),
            18 => Some(VkFormat::R8G8Uscaled),
            19 => Some(VkFormat::R8G8Sscaled),
            20 => Some(VkFormat::R8G8Uint),
            21 => Some(VkFormat::R8G8Sint),
            22 => Some(VkFormat::R8G8Srgb),
            23 => Some(VkFormat::R8G8B8Unorm),
            24 => Some(VkFormat::R8G8B8Snorm),
            25 => Some(VkFormat::R8G8B8Uscaled),
            26 => Some(VkFormat::R8G8B8Sscaled),
            27 => Some(VkFormat::R8G8B8Uint),
            28 => Some(VkFormat::R8G8B8Sint),
            29 => Some(VkFormat::R8G8B8Srgb),
            30 => Some(VkFormat::B8G8R8Unorm),
            31 => Some(VkFormat::B8G8R8Snorm),
            32 => Some(VkFormat::B8G8R8Uscaled),
            33 => Some(VkFormat::B8G8R8Sscaled),
            34 => Some(VkFormat::B8G8R8Uint),
            35 => Some(VkFormat::B8G8R8Sint),
            36 => Some(VkFormat::B8G8R8Srgb),
            37 => Some(VkFormat::R8G8B8A8Unorm),
            38 => Some(VkFormat::R8G8B8A8Snorm),
            39 => Some(VkFormat::R8G8B8A8Uscaled),
            40 => Some(VkFormat::R8G8B8A8Sscaled),
            41 => Some(VkFormat::R8G8B8A8Uint),
            42 => Some(VkFormat::R8G8B8A8Sint),
            43 => Some(VkFormat::R8G8B8A8Srgb),
            44 => Some(VkFormat::B8G8R8A8Unorm),
            45 => Some(VkFormat::B8G8R8A8Snorm),
            46 => Some(VkFormat::B8G8R8A8Uscaled),
            47 => Some(VkFormat::B8G8R8A8Sscaled),
            48 => Some(VkFormat::B8G8R8A8Uint),
            49 => Some(VkFormat::B8G8R8A8Sint),
            50 => Some(VkFormat::B8G8R8A8Srgb),
            51 => Some(VkFormat::A8B8G8R8UnormPack32),
            52 => Some(VkFormat::A8B8G8R8SnormPack32),
            53 => Some(VkFormat::A8B8G8R8UscaledPack32),
            54 => Some(VkFormat::A8B8G8R8SscaledPack32),
            55 => Some(VkFormat::A8B8G8R8UintPack32),
            56 => Some(VkFormat::A8B8G8R8SintPack32),
            57 => Some(VkFormat::A8B8G8R8SrgbPack32),
            58 => Some(VkFormat::A2R10G10B10UnormPack32),
            59 => Some(VkFormat::A2R10G10B10SnormPack32),
            60 => Some(VkFormat::A2R10G10B10UscaledPack32),
            61 => Some(VkFormat::A2R10G10B10SscaledPack32),
            62 => Some(VkFormat::A2R10G10B10UintPack32),
            63 => Some(VkFormat::A2R10G10B10SintPack32),
            64 => Some(VkFormat::A2B10G10R10UnormPack32),
            65 => Some(VkFormat::A2B10G10R10SnormPack32),
            66 => Some(VkFormat::A2B10G10R10UscaledPack32),
            67 => Some(VkFormat::A2B10G10R10SscaledPack32),
            68 => Some(VkFormat::A2B10G10R10UintPack32),
            69 => Some(VkFormat::A2B10G10R10SintPack32),
            70 => Some(VkFormat::R16Unorm),
            71 => Some(VkFormat::R16Snorm),
            72 => Some(VkFormat::R16Uscaled),
            73 => Some(VkFormat::R16Sscaled),
            74 => Some(VkFormat::R16Uint),
            75 => Some(VkFormat::R16Sint),
            76 => Some(VkFormat::R16Sfloat),
            77 => Some(VkFormat::R16G16Unorm),
            78 => Some(VkFormat::R16G16Snorm),
            79 => Some(VkFormat::R16G16Uscaled),
            80 => Some(VkFormat::R16G16Sscaled),
            81 => Some(VkFormat::R16G16Uint),
            82 => Some(VkFormat::R16G16Sint),
            83 => Some(VkFormat::R16G16Sfloat),
            84 => Some(VkFormat::R16G16B16Unorm),
            85 => Some(VkFormat::R16G16B16Snorm),
            86 => Some(VkFormat::R16G16B16Uscaled),
            87 => Some(VkFormat::R16G16B16Sscaled),
            88 => Some(VkFormat::R16G16B16Uint),
            89 => Some(VkFormat::R16G16B16Sint),
            90 => Some(VkFormat::R16G16B16Sfloat),
            91 => Some(VkFormat::R16G16B16A16Unorm),
            92 => Some(VkFormat::R16G16B16A16Snorm),
            93 => Some(VkFormat::R16G16B16A16Uscaled),
            94 => Some(VkFormat::R16G16B16A16Sscaled),
            95 => Some(VkFormat::R16G16B16A16Uint),
            96 => Some(VkFormat::R16G16B16A16Sint),
            97 => Some(VkFormat::R16G16B16A16Sfloat),
            98 => Some(VkFormat::R32Uint),
            99 => Some(VkFormat::R32Sint),
            100 => Some(VkFormat::R32Sfloat),
            101 => Some(VkFormat::R32G32Uint),
            102 => Some(VkFormat::R32G32Sint),
            103 => Some(VkFormat::R32G32Sfloat),
            104 => Some(VkFormat::R32G32B32Uint),
            105 => Some(VkFormat::R32G32B32Sint),
            106 => Some(VkFormat::R32G32B32Sfloat),
            107 => Some(VkFormat::R32G32B32A32Uint),
            108 => Some(VkFormat::R32G32B32A32Sint),
            109 => Some(VkFormat::R32G32B32A32Sfloat),
            110 => Some(VkFormat::R64Uint),
            111 => Some(VkFormat::R64Sint),
            112 => Some(VkFormat::R64Sfloat),
            113 => Some(VkFormat::R64G64Uint),
            114 => Some(VkFormat::R64G64Sint),
            115 => Some(VkFormat::R64G64Sfloat),
            116 => Some(VkFormat::R64G64B64Uint),
            117 => Some(VkFormat::R64G64B64Sint),
            118 => Some(VkFormat::R64G64B64Sfloat),
            119 => Some(VkFormat::R64G64B64A64Uint),
            120 => Some(VkFormat::R64G64B64A64Sint),
            121 => Some(VkFormat::R64G64B64A64Sfloat),
            122 => Some(VkFormat::B10G11R11UfloatPack32),
            123 => Some(VkFormat::E5B9G9R9UfloatPack32),
            124 => Some(VkFormat::D16Unorm),
            125 => Some(VkFormat::X8D24UnormPack32),
            126 => Some(VkFormat::D32Sfloat),
            127 => Some(VkFormat::S8Uint),
            128 => Some(VkFormat::D16UnormS8Uint),
            129 => Some(VkFormat::D24UnormS8Uint),
            130 => Some(VkFormat::D32SfloatS8Uint),
            131 => Some(VkFormat::Bc1RgbUnormBlock),
            132 => Some(VkFormat::Bc1RgbSrgbBlock),
            133 => Some(VkFormat::Bc1RgbaUnormBlock),
            134 => Some(VkFormat::Bc1RgbaSrgbBlock),
            135 => Some(VkFormat::Bc2UnormBlock),
            136 => Some(VkFormat::Bc2SrgbBlock),
            137 => Some(VkFormat::Bc3UnormBlock),
            138 => Some(VkFormat::Bc3SrgbBlock),
            139 => Some(VkFormat::Bc4UnormBlock),
            140 => Some(VkFormat::Bc4SnormBlock),
            141 => Some(VkFormat::Bc5UnormBlock),
            142 => Some(VkFormat::Bc5SnormBlock),
            143 => Some(VkFormat::Bc6hUfloatBlock),
            144 => Some(VkFormat::Bc6hSfloatBlock),
            145 => Some(VkFormat::Bc7UnormBlock),
            146 => Some(VkFormat::Bc7SrgbBlock),
            147 => Some(VkFormat::Etc2R8G8B8UnormBlock),
            148 => Some(VkFormat::Etc2R8G8B8SrgbBlock),
            149 => Some(VkFormat::Etc2R8G8B8A1UnormBlock),
            150 => Some(VkFormat::Etc2R8G8B8A1SrgbBlock),
            151 => Some(VkFormat::Etc2R8G8B8A8UnormBlock),
            152 => Some(VkFormat::Etc2R8G8B8A8SrgbBlock),
            153 => Some(VkFormat::EacR11UnormBlock),
            154 => Some(VkFormat::EacR11SnormBlock),
            155 => Some(VkFormat::EacR11G11UnormBlock),
            156 => Some(VkFormat::EacR11G11SnormBlock),
            157 => Some(VkFormat::Astc4x4UnormBlock),
            158 => Some(VkFormat::Astc4x4SrgbBlock),
            159 => Some(VkFormat::Astc5x4UnormBlock),
            160 => Some(VkFormat::Astc5x4SrgbBlock),
            161 => Some(VkFormat::Astc5x5UnormBlock),
            162 => Some(VkFormat::Astc5x5SrgbBlock),
            163 => Some(VkFormat::Astc6x5UnormBlock),
            164 => Some(VkFormat::Astc6x5SrgbBlock),
            165 => Some(VkFormat::Astc6x6UnormBlock),
            166 => Some(VkFormat::Astc6x6SrgbBlock),
            167 => Some(VkFormat::Astc8x5UnormBlock),
            168 => Some(VkFormat::Astc8x5SrgbBlock),
            169 => Some(VkFormat::Astc8x6UnormBlock),
            170 => Some(VkFormat::Astc8x6SrgbBlock),
            171 => Some(VkFormat::Astc8x8UnormBlock),
            172 => Some(VkFormat::Astc8x8SrgbBlock),
            173 => Some(VkFormat::Astc10x5UnormBlock),
            174 => Some(VkFormat::Astc10x5SrgbBlock),
            175 => Some(VkFormat::Astc10x6UnormBlock),
            176 => Some(VkFormat::Astc10x6SrgbBlock),
            177 => Some(VkFormat::Astc10x8UnormBlock),
            178 => Some(VkFormat::Astc10x8SrgbBlock),
            179 => Some(VkFormat::Astc10x10UnormBlock),
            180 => Some(VkFormat::Astc10x10SrgbBlock),
            181 => Some(VkFormat::Astc12x10UnormBlock),
            182 => Some(VkFormat::Astc12x10SrgbBlock),
            183 => Some(VkFormat::Astc12x12UnormBlock),
            184 => Some(VkFormat::Astc12x12SrgbBlock),
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
