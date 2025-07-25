use crate::bindings::*;

/// GPU texture formats supported for transcoding
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranscodeFormat {
    /// ETC1 RGB format (mobile)
    Etc1Rgb,
    /// ETC2 RGBA format (mobile)
    Etc2Rgba,
    /// BC1 RGB format (desktop)
    Bc1Rgb,
    /// BC3 RGBA format (desktop)
    Bc3Rgba,
    /// BC4 R format (desktop)
    Bc4R,
    /// BC5 RG format (desktop)
    Bc5Rg,
    /// BC7 RGBA format (desktop, high quality)
    Bc7Rgba,
    /// PVRTC1 4bpp RGB format (iOS)
    Pvrtc1_4_Rgb,
    /// PVRTC1 4bpp RGBA format (iOS)
    Pvrtc1_4_Rgba,
    /// ASTC 4x4 RGBA format (modern mobile)
    Astc_4x4_Rgba,
    /// Uncompressed RGBA32 format (universal)
    Rgba32,
    /// RGB565 format (mobile, low memory)
    Rgb565,
    /// BGR565 format
    Bgr565,
    /// RGBA4444 format (mobile, low memory)
    Rgba4444,
}

impl From<TranscodeFormat> for ktx_transcode_fmt_e {
    fn from(format: TranscodeFormat) -> Self {
        match format {
            TranscodeFormat::Etc1Rgb => ktx_transcode_fmt_e_KTX_TTF_ETC1_RGB,
            TranscodeFormat::Etc2Rgba => ktx_transcode_fmt_e_KTX_TTF_ETC2_RGBA,
            TranscodeFormat::Bc1Rgb => ktx_transcode_fmt_e_KTX_TTF_BC1_RGB,
            TranscodeFormat::Bc3Rgba => ktx_transcode_fmt_e_KTX_TTF_BC3_RGBA,
            TranscodeFormat::Bc4R => ktx_transcode_fmt_e_KTX_TTF_BC4_R,
            TranscodeFormat::Bc5Rg => ktx_transcode_fmt_e_KTX_TTF_BC5_RG,
            TranscodeFormat::Bc7Rgba => ktx_transcode_fmt_e_KTX_TTF_BC7_RGBA,
            TranscodeFormat::Pvrtc1_4_Rgb => ktx_transcode_fmt_e_KTX_TTF_PVRTC1_4_RGB,
            TranscodeFormat::Pvrtc1_4_Rgba => ktx_transcode_fmt_e_KTX_TTF_PVRTC1_4_RGBA,
            TranscodeFormat::Astc_4x4_Rgba => ktx_transcode_fmt_e_KTX_TTF_ASTC_4x4_RGBA,
            TranscodeFormat::Rgba32 => ktx_transcode_fmt_e_KTX_TTF_RGBA32,
            TranscodeFormat::Rgb565 => ktx_transcode_fmt_e_KTX_TTF_RGB565,
            TranscodeFormat::Bgr565 => ktx_transcode_fmt_e_KTX_TTF_BGR565,
            TranscodeFormat::Rgba4444 => ktx_transcode_fmt_e_KTX_TTF_RGBA4444,
        }
    }
}
