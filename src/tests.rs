use crate::bindings::*;
use crate::compression::BasisCompressionParams;
use crate::error::Error;
use crate::format::TranscodeFormat;
use crate::texture::Ktx2Texture;
use crate::vk_format::VkFormat;

// ============================================================================
// Constants and Bindings Tests
// ============================================================================

#[test]
fn test_error_conversion() {
    let error: Error = ktx_error_code_e_KTX_FILE_DATA_ERROR.into();
    assert_eq!(error, Error::FileDataError);

    let error: Error = ktx_error_code_e_KTX_OUT_OF_MEMORY.into();
    assert_eq!(error, Error::OutOfMemory);
}

#[test]
fn test_all_error_conversions() {
    // Test all error code conversions
    let test_cases = [
        (ktx_error_code_e_KTX_FILE_DATA_ERROR, Error::FileDataError),
        (ktx_error_code_e_KTX_FILE_ISPIPE, Error::FilePipe),
        (ktx_error_code_e_KTX_FILE_OPEN_FAILED, Error::FileOpenFailed),
        (ktx_error_code_e_KTX_FILE_OVERFLOW, Error::FileOverflow),
        (ktx_error_code_e_KTX_FILE_READ_ERROR, Error::FileReadError),
        (ktx_error_code_e_KTX_FILE_SEEK_ERROR, Error::FileSeekError),
        (
            ktx_error_code_e_KTX_FILE_UNEXPECTED_EOF,
            Error::FileUnexpectedEof,
        ),
        (ktx_error_code_e_KTX_FILE_WRITE_ERROR, Error::FileWriteError),
        (ktx_error_code_e_KTX_GL_ERROR, Error::GlError),
        (
            ktx_error_code_e_KTX_INVALID_OPERATION,
            Error::InvalidOperation,
        ),
        (ktx_error_code_e_KTX_INVALID_VALUE, Error::InvalidValue),
        (ktx_error_code_e_KTX_NOT_FOUND, Error::NotFound),
        (ktx_error_code_e_KTX_OUT_OF_MEMORY, Error::OutOfMemory),
        (
            ktx_error_code_e_KTX_TRANSCODE_FAILED,
            Error::TranscodeFailed,
        ),
        (
            ktx_error_code_e_KTX_UNKNOWN_FILE_FORMAT,
            Error::UnknownFileFormat,
        ),
        (
            ktx_error_code_e_KTX_UNSUPPORTED_TEXTURE_TYPE,
            Error::UnsupportedTextureType,
        ),
        (
            ktx_error_code_e_KTX_UNSUPPORTED_FEATURE,
            Error::UnsupportedFeature,
        ),
        (
            ktx_error_code_e_KTX_LIBRARY_NOT_LINKED,
            Error::LibraryNotLinked,
        ),
        (
            ktx_error_code_e_KTX_DECOMPRESS_LENGTH_ERROR,
            Error::DecompressLengthError,
        ),
        (
            ktx_error_code_e_KTX_DECOMPRESS_CHECKSUM_ERROR,
            Error::DecompressChecksumError,
        ),
    ];

    for (code, expected_error) in test_cases {
        let error: Error = code.into();
        assert_eq!(error, expected_error);
    }
}

#[test]
fn test_error_display() {
    let error = Error::FileDataError;
    assert!(error.to_string().contains("inconsistent with the KTX spec"));

    let error = Error::OutOfMemory;
    assert!(error.to_string().contains("Not enough memory"));

    let error = Error::Other(12345);
    assert!(error.to_string().contains("12345"));
}

#[test]
fn test_error_debug() {
    let error = Error::InvalidValue;
    let debug_str = format!("{error:?}");
    assert_eq!(debug_str, "InvalidValue");
}

// ============================================================================
// TranscodeFormat Tests
// ============================================================================

#[test]
fn test_transcode_format_conversion() {
    let format = TranscodeFormat::Bc7Rgba;
    let ktx_format: ktx_transcode_fmt_e = format.into();
    assert_eq!(ktx_format, ktx_transcode_fmt_e_KTX_TTF_BC7_RGBA);
}

#[test]
fn test_all_transcode_format_conversions() {
    let test_cases = [
        (
            TranscodeFormat::Etc1Rgb,
            ktx_transcode_fmt_e_KTX_TTF_ETC1_RGB,
        ),
        (
            TranscodeFormat::Etc2Rgba,
            ktx_transcode_fmt_e_KTX_TTF_ETC2_RGBA,
        ),
        (TranscodeFormat::Bc1Rgb, ktx_transcode_fmt_e_KTX_TTF_BC1_RGB),
        (
            TranscodeFormat::Bc3Rgba,
            ktx_transcode_fmt_e_KTX_TTF_BC3_RGBA,
        ),
        (TranscodeFormat::Bc4R, ktx_transcode_fmt_e_KTX_TTF_BC4_R),
        (TranscodeFormat::Bc5Rg, ktx_transcode_fmt_e_KTX_TTF_BC5_RG),
        (
            TranscodeFormat::Bc7Rgba,
            ktx_transcode_fmt_e_KTX_TTF_BC7_RGBA,
        ),
        (
            TranscodeFormat::Pvrtc1_4_Rgb,
            ktx_transcode_fmt_e_KTX_TTF_PVRTC1_4_RGB,
        ),
        (
            TranscodeFormat::Pvrtc1_4_Rgba,
            ktx_transcode_fmt_e_KTX_TTF_PVRTC1_4_RGBA,
        ),
        (
            TranscodeFormat::Astc_4x4_Rgba,
            ktx_transcode_fmt_e_KTX_TTF_ASTC_4x4_RGBA,
        ),
        (TranscodeFormat::Rgba32, ktx_transcode_fmt_e_KTX_TTF_RGBA32),
        (TranscodeFormat::Rgb565, ktx_transcode_fmt_e_KTX_TTF_RGB565),
        (TranscodeFormat::Bgr565, ktx_transcode_fmt_e_KTX_TTF_BGR565),
        (
            TranscodeFormat::Rgba4444,
            ktx_transcode_fmt_e_KTX_TTF_RGBA4444,
        ),
    ];

    for (format, expected_ktx_format) in test_cases {
        let ktx_format: ktx_transcode_fmt_e = format.into();
        assert_eq!(ktx_format, expected_ktx_format);
    }
}

#[test]
fn test_transcode_format_debug() {
    let format = TranscodeFormat::Bc7Rgba;
    let debug_str = format!("{format:?}");
    assert_eq!(debug_str, "Bc7Rgba");
}

#[test]
fn test_transcode_format_clone() {
    let format = TranscodeFormat::Etc2Rgba;
    let cloned = format;
    assert_eq!(format, cloned);
}

// ============================================================================
// BasisCompressionParams Tests
// ============================================================================

#[test]
fn test_basis_compression_params_default() {
    let params = BasisCompressionParams::builder().build();
    assert!(!params.uastc);
    assert_eq!(params.thread_count, 1);
    assert!(!params.no_endpoint_rdo);
    assert!(!params.no_selector_rdo);
    assert_eq!(params.quality_level, 128);
    assert_eq!(params.input_swizzle, [0, 1, 2, 3]);
}

#[test]
fn test_basis_compression_params_uastc_mode() {
    let params = BasisCompressionParams::builder()
        .uastc(true)
        .quality_level(255)
        .uastc_rdo_quality_scalar(1.0)
        .build();

    assert!(params.uastc);
    assert_eq!(params.quality_level, 255);
    assert_eq!(params.uastc_rdo_quality_scalar, 1.0);
}

#[test]
fn test_basis_compression_params_etc1s_mode() {
    let params = BasisCompressionParams::builder()
        .uastc(false)
        .quality_level(64)
        .thread_count(4)
        .max_endpoints(8000)
        .max_selectors(8000)
        .build();

    assert!(!params.uastc);
    assert_eq!(params.quality_level, 64);
    assert_eq!(params.thread_count, 4);
    assert_eq!(params.max_endpoints, 8000);
    assert_eq!(params.max_selectors, 8000);
}

#[test]
fn test_basis_compression_params_clone() {
    let params = BasisCompressionParams::builder().build();
    let cloned = params.clone();
    assert_eq!(params.thread_count, cloned.thread_count);
    assert_eq!(params.uastc, cloned.uastc);
}

#[test]
fn test_basis_compression_params_debug() {
    let params = BasisCompressionParams::builder().build();
    let debug_str = format!("{params:?}");
    assert!(debug_str.contains("BasisCompressionParams"));
    assert!(debug_str.contains("uastc: false"));
}

#[test]
fn test_basis_compression_params_swizzle() {
    let params = BasisCompressionParams::builder()
        .input_swizzle([2, 1, 0, 3]) // BGR swap
        .build();

    assert_eq!(params.input_swizzle, [2, 1, 0, 3]);
}

#[test]
fn test_basis_compression_params_builder_basic() {
    let params = BasisCompressionParams::builder()
        .uastc(true)
        .quality_level(200)
        .thread_count(4)
        .build();

    assert!(params.uastc);
    assert_eq!(params.quality_level, 200);
    assert_eq!(params.thread_count, 4);
    // Other params should have default values
    assert_eq!(params.compression_level, unsafe {
        KTX_ETC1S_DEFAULT_COMPRESSION_LEVEL
    });
    assert_eq!(params.input_swizzle, [0, 1, 2, 3]);
}

#[test]
fn test_basis_compression_params_builder_all_methods() {
    let params = BasisCompressionParams::builder()
        .uastc(true)
        .thread_count(8)
        .compression_level(6)
        .quality_level(255)
        .max_endpoints(16000)
        .endpoint_rdo_threshold(1.5)
        .max_selectors(16000)
        .selector_rdo_threshold(1.25)
        .normal_map(true)
        .separate_rg_to_color_alpha(true)
        .pre_swizzle(true)
        .no_endpoint_rdo(true)
        .no_selector_rdo(true)
        .uastc_flags(4)
        .uastc_rdo(true)
        .uastc_rdo_quality_scalar(0.75)
        .uastc_rdo_dict_size(8192)
        .input_swizzle([2, 1, 0, 3])
        .build();

    assert!(params.uastc);
    assert_eq!(params.thread_count, 8);
    assert_eq!(params.compression_level, 6);
    assert_eq!(params.quality_level, 255);
    assert_eq!(params.max_endpoints, 16000);
    assert_eq!(params.endpoint_rdo_threshold, 1.5);
    assert_eq!(params.max_selectors, 16000);
    assert_eq!(params.selector_rdo_threshold, 1.25);
    assert!(params.normal_map);
    assert!(params.separate_rg_to_color_alpha);
    assert!(params.pre_swizzle);
    assert!(params.no_endpoint_rdo);
    assert!(params.no_selector_rdo);
    assert_eq!(params.uastc_flags, 4);
    assert!(params.uastc_rdo);
    assert_eq!(params.uastc_rdo_quality_scalar, 0.75);
    assert_eq!(params.uastc_rdo_dict_size, 8192);
    assert_eq!(params.input_swizzle, [2, 1, 0, 3]);
}

#[test]
fn test_basis_compression_params_builder_etc1s_config() {
    let params = BasisCompressionParams::builder()
        .uastc(false)
        .quality_level(64)
        .thread_count(6)
        .compression_level(5)
        .max_endpoints(4000)
        .max_selectors(4000)
        .endpoint_rdo_threshold(0.5)
        .selector_rdo_threshold(0.75)
        .no_endpoint_rdo(false)
        .no_selector_rdo(false)
        .build();

    assert!(!params.uastc);
    assert_eq!(params.quality_level, 64);
    assert_eq!(params.thread_count, 6);
    assert_eq!(params.compression_level, 5);
    assert_eq!(params.max_endpoints, 4000);
    assert_eq!(params.max_selectors, 4000);
    assert_eq!(params.endpoint_rdo_threshold, 0.5);
    assert_eq!(params.selector_rdo_threshold, 0.75);
    assert!(!params.no_endpoint_rdo);
    assert!(!params.no_selector_rdo);
}

#[test]
fn test_basis_compression_params_builder_uastc_config() {
    let params = BasisCompressionParams::builder()
        .uastc(true)
        .quality_level(4)
        .thread_count(12)
        .uastc_flags(2)
        .uastc_rdo(true)
        .uastc_rdo_quality_scalar(1.25)
        .uastc_rdo_dict_size(16384)
        .build();

    assert!(params.uastc);
    assert_eq!(params.quality_level, 4);
    assert_eq!(params.thread_count, 12);
    assert_eq!(params.uastc_flags, 2);
    assert!(params.uastc_rdo);
    assert_eq!(params.uastc_rdo_quality_scalar, 1.25);
    assert_eq!(params.uastc_rdo_dict_size, 16384);
}

#[test]
fn test_basis_compression_params_builder_normal_map_config() {
    let params = BasisCompressionParams::builder()
        .normal_map(true)
        .separate_rg_to_color_alpha(true)
        .input_swizzle([0, 1, 2, 3])
        .quality_level(180)
        .build();

    assert!(params.normal_map);
    assert!(params.separate_rg_to_color_alpha);
    assert_eq!(params.input_swizzle, [0, 1, 2, 3]);
    assert_eq!(params.quality_level, 180);
}

#[test]
fn test_basis_compression_params_builder_chaining() {
    // Test that builder methods can be chained in any order
    let params1 = BasisCompressionParams::builder()
        .thread_count(4)
        .uastc(true)
        .quality_level(200)
        .build();

    let params2 = BasisCompressionParams::builder()
        .quality_level(200)
        .uastc(true)
        .thread_count(4)
        .build();

    assert_eq!(params1.thread_count, params2.thread_count);
    assert_eq!(params1.uastc, params2.uastc);
    assert_eq!(params1.quality_level, params2.quality_level);
}

#[test]
fn test_basis_compression_params_builder_partial() {
    // Test that partially configured builder still has correct defaults
    let params = BasisCompressionParams::builder().thread_count(2).build();

    assert_eq!(params.thread_count, 2);
    // Should have default values for everything else
    assert!(!params.uastc);
    assert_eq!(params.quality_level, 128);
    assert_eq!(params.max_endpoints, 0);
    assert_eq!(params.max_selectors, 0);
    assert!(!params.normal_map);
    assert_eq!(params.input_swizzle, [0, 1, 2, 3]);
}

#[test]
fn test_basis_compression_params_builder_equivalence() {
    // Test that builder produces equivalent results to manual construction
    let builder_default = BasisCompressionParams::builder().build();
    let manual_params = BasisCompressionParams {
        uastc: true,
        thread_count: 8,
        quality_level: 255,
        max_endpoints: 8000,
        normal_map: true,
        input_swizzle: [2, 1, 0, 3],
        ..builder_default
    };

    let builder_params = BasisCompressionParams::builder()
        .uastc(true)
        .thread_count(8)
        .quality_level(255)
        .max_endpoints(8000)
        .normal_map(true)
        .input_swizzle([2, 1, 0, 3])
        .build();

    assert_eq!(manual_params.uastc, builder_params.uastc);
    assert_eq!(manual_params.thread_count, builder_params.thread_count);
    assert_eq!(manual_params.quality_level, builder_params.quality_level);
    assert_eq!(manual_params.max_endpoints, builder_params.max_endpoints);
    assert_eq!(manual_params.normal_map, builder_params.normal_map);
    assert_eq!(manual_params.input_swizzle, builder_params.input_swizzle);
}

// ============================================================================
// Ktx2Texture Creation Tests
// ============================================================================

#[test]
fn test_texture_create_valid() {
    let result = Ktx2Texture::create(256, 256, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm); // VK_FORMAT_R8G8B8A8_UNORM
    assert!(result.is_ok());

    let texture = result.unwrap();
    assert_eq!(texture.width(), 256);
    assert_eq!(texture.height(), 256);
    assert_eq!(texture.depth(), 1);
    assert_eq!(texture.layers(), 1);
    assert_eq!(texture.faces(), 1);
    assert_eq!(texture.levels(), 1);
    assert_eq!(texture.vk_format(), VkFormat::R8G8B8A8Unorm);
}

#[test]
fn test_texture_create_invalid_dimensions() {
    // Zero width should fail
    let result = Ktx2Texture::create(0, 256, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidValue);

    // Zero height should fail
    let result = Ktx2Texture::create(256, 0, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidValue);

    // Zero depth should fail
    let result = Ktx2Texture::create(256, 256, 0, 1, 1, 1, VkFormat::R8G8B8A8Unorm);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidValue);
}

#[test]
fn test_texture_create_excessive_dimensions() {
    // Dimensions too large should fail
    let result = Ktx2Texture::create(100000, 100000, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidValue);
}

#[test]
fn test_texture_create_cubemap() {
    let result = Ktx2Texture::create(256, 256, 1, 1, 6, 1, VkFormat::R8G8B8A8Unorm); // Cubemap has 6 faces
    assert!(result.is_ok());

    let texture = result.unwrap();
    assert_eq!(texture.faces(), 6);
    assert!(texture.is_cubemap());
}

#[test]
fn test_texture_create_array() {
    let result = Ktx2Texture::create(256, 256, 1, 8, 1, 1, VkFormat::R8G8B8A8Unorm); // Array with 8 layers
    assert!(result.is_ok());

    let texture = result.unwrap();
    assert_eq!(texture.layers(), 8);
    assert!(texture.is_array());
}

#[test]
fn test_texture_create_mipmapped() {
    let result = Ktx2Texture::create(256, 256, 1, 1, 1, 9, VkFormat::R8G8B8A8Unorm); // 9 mip levels for 256x256
    assert!(result.is_ok());

    let texture = result.unwrap();
    assert_eq!(texture.levels(), 9);
}

#[test]
fn test_texture_create_3d() {
    let result = Ktx2Texture::create(128, 128, 64, 1, 1, 1, VkFormat::R8G8B8A8Unorm); // 3D texture
    assert!(result.is_ok());

    let texture = result.unwrap();
    assert_eq!(texture.width(), 128);
    assert_eq!(texture.height(), 128);
    assert_eq!(texture.depth(), 64);
}

// ============================================================================
// Texture Property Tests
// ============================================================================

#[test]
fn test_texture_properties() {
    let texture = Ktx2Texture::create(512, 256, 1, 4, 1, 8, VkFormat::R8G8B8A8Unorm).unwrap();

    assert_eq!(texture.width(), 512);
    assert_eq!(texture.height(), 256);
    assert_eq!(texture.depth(), 1);
    assert_eq!(texture.layers(), 4);
    assert_eq!(texture.faces(), 1);
    assert_eq!(texture.levels(), 8);
    assert_eq!(texture.vk_format(), VkFormat::R8G8B8A8Unorm);
    assert!(texture.is_array()); // 4 layers
    assert!(!texture.is_cubemap()); // 1 face
}

#[test]
fn test_texture_compression_status() {
    let texture = Ktx2Texture::create(256, 256, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm).unwrap();

    // Newly created texture should not be compressed
    assert!(!texture.is_compressed());
    assert!(!texture.needs_transcoding());
}

// ============================================================================
// Memory-based Texture Tests
// ============================================================================

#[test]
fn test_texture_from_empty_memory() {
    let empty_data: &[u8] = &[];
    let result = Ktx2Texture::from_memory(empty_data);
    assert!(result.is_err());
}

#[test]
fn test_texture_from_invalid_memory() {
    let invalid_data = vec![0u8; 100]; // Random data, not a valid KTX2 file
    let result = Ktx2Texture::from_memory(&invalid_data);
    assert!(result.is_err());
}

// ============================================================================
// Image Data Tests
// ============================================================================

#[test]
fn test_texture_set_image_data_empty() {
    let mut texture = Ktx2Texture::create(256, 256, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm).unwrap();
    let empty_data: &[u8] = &[];

    let result = texture.set_image_data(0, 0, 0, empty_data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidValue);
}

#[test]
fn test_texture_set_image_data_invalid_indices() {
    let mut texture = Ktx2Texture::create(256, 256, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm).unwrap();
    let data = vec![255u8; 1024];

    // Invalid level
    let result = texture.set_image_data(10, 0, 0, &data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidValue);

    // Invalid layer
    let result = texture.set_image_data(0, 10, 0, &data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidValue);

    // Invalid face
    let result = texture.set_image_data(0, 0, 10, &data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidValue);
}

#[test]
fn test_texture_get_image_data_invalid_indices() {
    let texture = Ktx2Texture::create(256, 256, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm).unwrap();

    // Invalid level
    let result = texture.get_image_data(10, 0, 0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidValue);

    // Invalid layer
    let result = texture.get_image_data(0, 10, 0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidValue);

    // Invalid face
    let result = texture.get_image_data(0, 0, 10);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidValue);
}

// ============================================================================
// Metadata Tests
// ============================================================================

#[test]
fn test_texture_metadata_roundtrip() {
    let mut texture = Ktx2Texture::create(256, 256, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm).unwrap();

    let key = "test_key";
    let value = b"test_value_data";

    // Set metadata
    let result = texture.set_metadata(key, value);
    assert!(result.is_ok());

    // Get metadata
    let result = texture.get_metadata(key);
    assert!(result.is_ok());
    let retrieved_value = result.unwrap();
    assert_eq!(retrieved_value, value);
}

#[test]
fn test_texture_metadata_not_found() {
    let texture = Ktx2Texture::create(256, 256, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm).unwrap();

    let result = texture.get_metadata("nonexistent_key");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::NotFound);
}

#[test]
fn test_texture_metadata_invalid_key() {
    let mut texture = Ktx2Texture::create(256, 256, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm).unwrap();

    // Key with null byte should fail
    let invalid_key = "test\0key";
    let value = b"test_value";

    let result = texture.set_metadata(invalid_key, value);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidValue);
}

// ============================================================================
// Compression Tests
// ============================================================================

#[test]
fn test_compress_basis_simple() {
    let mut texture = Ktx2Texture::create(256, 256, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm).unwrap();

    // Add some dummy image data first
    let image_data = vec![128u8; 256 * 256 * 4]; // RGBA data
    texture.set_image_data(0, 0, 0, &image_data).unwrap();

    let result = texture.compress_basis_simple(128);
    // Note: This might fail due to missing image data or other reasons
    // but it should not panic
    let _result = result;
}

#[test]
fn test_compress_basis_with_params() {
    let mut texture = Ktx2Texture::create(256, 256, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm).unwrap();

    // Add some dummy image data first
    let image_data = vec![128u8; 256 * 256 * 4]; // RGBA data
    texture.set_image_data(0, 0, 0, &image_data).unwrap();

    let params = BasisCompressionParams::builder().build();
    let result = texture.compress_basis(&params);
    // Note: This might fail due to missing image data or other reasons
    // but it should not panic
    let _result = result;
}

// ============================================================================
// Write to Memory Tests
// ============================================================================

#[test]
fn test_write_to_memory() {
    let texture = Ktx2Texture::create(256, 256, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm).unwrap();

    let result = texture.write_to_memory();
    assert!(result.is_ok());

    let data = result.unwrap();
    assert!(!data.is_empty());

    // Should start with KTX2 identifier
    assert!(data.len() >= 12);
    // KTX2 files start with specific bytes
    let expected_header = [
        0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
    ];
    assert_eq!(&data[0..12], &expected_header);
}

// ============================================================================
// Thread Safety Tests
// ============================================================================

#[test]
fn test_texture_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<Ktx2Texture>();
    assert_sync::<Ktx2Texture>();
}

// ============================================================================
// Comprehensive Edge Case Tests
// ============================================================================

#[test]
fn test_various_texture_formats() {
    let formats = [
        VkFormat::R8G8B8A8Unorm, // VK_FORMAT_R8G8B8A8_UNORM
        VkFormat::R8G8B8Unorm,   // VK_FORMAT_R8G8B8_UNORM
        VkFormat::R8Unorm,       // VK_FORMAT_R8_UNORM
        VkFormat::R8G8Unorm,     // VK_FORMAT_R8G8_UNORM
    ];

    for format in formats {
        let result = Ktx2Texture::create(64, 64, 1, 1, 1, 1, format);
        assert!(
            result.is_ok(),
            "Failed to create texture with format {format:?}",
        );
    }
}

#[test]
fn test_texture_extreme_small_size() {
    let result = Ktx2Texture::create(1, 1, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm);
    assert!(result.is_ok());

    let texture = result.unwrap();
    assert_eq!(texture.width(), 1);
    assert_eq!(texture.height(), 1);
}

#[test]
fn test_texture_power_of_two_sizes() {
    let sizes = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];

    for size in sizes {
        let result = Ktx2Texture::create(size, size, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm);
        assert!(result.is_ok(), "Failed to create {size}x{size} texture");
    }
}

#[test]
fn test_texture_non_power_of_two_sizes() {
    let sizes = [(3, 5), (7, 11), (13, 17), (100, 200), (333, 777)];

    for (width, height) in sizes {
        let result = Ktx2Texture::create(width, height, 1, 1, 1, 1, VkFormat::R8G8B8A8Unorm);
        assert!(result.is_ok(), "Failed to create {width}x{height} texture");
    }
}
