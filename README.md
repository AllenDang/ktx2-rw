# ktx2-rw

A high-level Rust wrapper for the [KTX2](https://registry.khronos.org/KTX/specs/2.0/ktxspec.v2.html) texture format with full [Basis Universal](https://github.com/BinomialLLC/basis_universal) support.

KTX2 is the next-generation texture format developed by Khronos, designed for efficient GPU texture storage and transmission. This library provides safe, idiomatic Rust bindings with comprehensive support for texture compression, transcoding, and metadata management.

## Features

- ✅ **Full KTX2 Support**: Read, write, and manipulate KTX2 texture files
- ✅ **Basis Universal Integration**: Compress textures with ETC1S and UASTC
- ✅ **Universal Transcoding**: Convert to GPU-specific formats (BC7, ETC2, ASTC, PVRTC, etc.)
- ✅ **Cross-Platform**: Support for Windows, macOS, and Linux (x64, ARM64)
- ✅ **Memory Safety**: Safe Rust API with proper error handling
- ✅ **Metadata Support**: Read/write custom key-value metadata
- ✅ **Zero-Copy Operations**: Efficient memory usage where possible

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
ktx2-rw = { git = "https://github.com/AllenDang/ktx2-rw" }
```

### Basic Usage

```rust
use ktx2_rw::{Ktx2Texture, BasisCompressionParams, TranscodeFormat};

// Create a new texture
let mut texture = Ktx2Texture::create(512, 512, 1, 1, 1, 1, 37)?; // RGBA8

// Add image data (512x512 RGBA)
let rgba_data: Vec<u8> = generate_image_data();
texture.set_image_data(0, 0, 0, &rgba_data)?;

// Compress with Basis Universal using the builder pattern
let compression_params = BasisCompressionParams::builder()
    .quality_level(128)
    .thread_count(4)
    .build();
texture.compress_basis(&compression_params)?;

// For simple compression, you can also use:
texture.compress_basis_simple(128)?; // Quality level only

// Save to file
texture.write_to_file("texture.ktx2")?;

// Load from file
let loaded = Ktx2Texture::from_file("texture.ktx2")?;

// Transcode to GPU format
let mut transcoded = loaded;
transcoded.transcode_basis(TranscodeFormat::Bc7Rgba)?;
```

### Advanced Compression

```rust
use ktx2_rw::BasisCompressionParams;

// Using the builder pattern for configuration
let params = BasisCompressionParams::builder()
    .uastc(true)                        // Use UASTC (higher quality)
    .quality_level(255)                 // Maximum quality
    .thread_count(8)                    // Multi-threaded compression
    .normal_map(true)                   // Optimize for normal maps
    .uastc_rdo_quality_scalar(1.0)      // RDO quality
    .build();

texture.compress_basis(&params)?;
```

## Supported Platforms

| Platform | Architecture | Status |
| -------- | ------------ | ------ |
| Windows  | x64          | ✅     |
| macOS    | x64          | ✅     |
| macOS    | ARM64        | ✅     |
| Linux    | x64          | ✅     |

## Transcoding Formats

The library supports transcoding to all major GPU texture formats:

- **Desktop**: BC1, BC3, BC4, BC5, BC7
- **Mobile**: ETC1, ETC2, ASTC 4x4, PVRTC1
- **Universal**: RGBA32, RGB565, RGBA4444

## API Reference

### Core Types

- `Ktx2Texture` - Main texture handle with safe lifetime management
- `BasisCompressionParams` - Comprehensive compression settings
  - `BasisCompressionParams::builder()` - Fluent builder for creating params
- `TranscodeFormat` - Supported GPU texture formats
- `Error` - Detailed error types with proper error messages

### Key Methods

#### Creation and Loading

```rust
Ktx2Texture::create(width, height, depth, layers, faces, levels, vk_format)
Ktx2Texture::from_file(path)
Ktx2Texture::from_memory(bytes)
```

#### Texture Operations

```rust
texture.compress_basis(params)           // Compress with Basis Universal
texture.compress_basis_simple(quality)   // Simple compression with quality level
texture.transcode_basis(format)          // Transcode to GPU format
texture.get_image_data(level, layer, face) // Get raw image data
texture.set_image_data(level, layer, face, data) // Set image data
```

#### I/O Operations

```rust
texture.write_to_file(path)              // Save to file
texture.write_to_memory()                // Export to bytes
```

#### Metadata

```rust
texture.set_metadata(key, value)         // Set custom metadata
texture.get_metadata(key)                // Read metadata
```

#### Properties

```rust
texture.width(), texture.height(), texture.depth()
texture.layers(), texture.faces(), texture.levels()
texture.is_compressed(), texture.needs_transcoding()
texture.vk_format()
```

## Error Handling

All operations return `Result<T, Error>` with detailed error information:

```rust
match texture.compress_basis(&params) {
    Ok(()) => println!("Compression successful"),
    Err(ktx2_rw::Error::OutOfMemory) => println!("Not enough memory"),
    Err(ktx2_rw::Error::UnsupportedFeature) => println!("Feature not supported"),
    Err(e) => println!("Other error: {}", e),
}
```

## Performance Notes

- Basis Universal compression is CPU-intensive but produces excellent results
- UASTC mode provides higher quality but larger file sizes than ETC1S
- Multi-threaded compression significantly improves performance
- Transcoding is typically very fast (GPU-optimized)

## Thread Safety

`Ktx2Texture` implements `Send + Sync` and can be safely used across threads.

## License

This library is provided under the same license terms as the underlying KTX2 library.

## Contributing

Contributions are welcome! Please ensure all tests pass and follow Rust coding conventions.

