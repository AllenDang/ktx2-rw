use image::ImageReader;
use ktx2_rw::{BasisCompressionParams, Ktx2Texture, Result, TranscodeFormat};
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    println!("PNG to KTX2 Converter Example");

    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() > 1 {
        &args[1]
    } else {
        // Create a sample PNG if no input provided
        create_sample_png("sample.png")?;
        "sample.png"
    };

    let output_path = if args.len() > 2 {
        &args[2]
    } else {
        "output.ktx2"
    };

    println!("Input: {input_path}");
    println!("Output: {output_path}");

    // Step 1: Load PNG image
    println!("\n1. Loading PNG image...");
    let img = ImageReader::open(input_path)
        .map_err(|_| ktx2_rw::Error::InvalidValue)?
        .decode()
        .map_err(|_| ktx2_rw::Error::InvalidValue)?;

    let (width, height) = (img.width(), img.height());
    println!("Loaded image: {width}x{height} pixels");

    // Convert to RGBA8 format
    let rgba_img = img.to_rgba8();
    let image_data = rgba_img.as_raw();

    println!("Image data size: {} bytes", image_data.len());

    // Step 2: Create KTX2 texture
    println!("\n2. Creating KTX2 texture...");
    let mut texture = Ktx2Texture::create(
        width, height, 1,  // depth
        1,  // layers
        1,  // faces
        1,  // levels
        37, // vk_format (VK_FORMAT_R8G8B8A8_UNORM)
    )?;

    // Step 3: Set image data
    println!("\n3. Setting image data...");
    texture.set_image_data(0, 0, 0, image_data)?;

    // Step 4: Add metadata
    println!("\n4. Adding metadata...");
    texture.set_metadata("OriginalFormat", b"PNG")?;
    texture.set_metadata("Tool", b"ktx2-rw PNG converter")?;
    texture.set_metadata("SourceFile", input_path.as_bytes())?;

    // Add image dimensions as metadata
    let dims = format!("{width}x{height}");
    texture.set_metadata("Dimensions", dims.as_bytes())?;

    // Step 5: Demonstrate different compression modes
    println!("\n5. Compressing with Basis Universal...");

    // Create two versions with different compression settings
    let mut texture_etc1s = texture;
    let mut texture_uastc = Ktx2Texture::create(width, height, 1, 1, 1, 1, 37)?;
    texture_uastc.set_image_data(0, 0, 0, image_data)?;
    texture_uastc.set_metadata("CompressionMode", b"UASTC")?;

    // ETC1S compression (smaller files, faster)
    println!("  - Compressing with ETC1S (smaller size)...");
    let etc1s_params = BasisCompressionParams::builder()
        .uastc(false)
        .quality_level(128)
        .thread_count(4)
        .endpoint_rdo_threshold(1.5)
        .selector_rdo_threshold(1.5)
        .build();

    texture_etc1s.compress_basis(&etc1s_params)?;
    texture_etc1s.set_metadata("CompressionMode", b"ETC1S")?;

    // UASTC compression (higher quality)
    println!("  - Compressing with UASTC (higher quality)...");
    let uastc_params = BasisCompressionParams::builder()
        .uastc(true)
        .quality_level(255)
        .thread_count(4)
        .uastc_rdo(true)
        .uastc_rdo_quality_scalar(1.0)
        .build();

    texture_uastc.compress_basis(&uastc_params)?;

    // Step 6: Save files and compare sizes
    println!("\n6. Saving KTX2 files...");

    let etc1s_data = texture_etc1s.write_to_memory()?;
    let uastc_data = texture_uastc.write_to_memory()?;

    println!(
        "Original PNG size: {} bytes",
        std::fs::metadata(input_path)
            .map_err(|_| ktx2_rw::Error::FileReadError)?
            .len()
    );
    println!(
        "ETC1S KTX2 size: {} bytes ({:.1}% of original)",
        etc1s_data.len(),
        etc1s_data.len() as f64
            / std::fs::metadata(input_path)
                .map_err(|_| ktx2_rw::Error::FileReadError)?
                .len() as f64
            * 100.0
    );
    println!(
        "UASTC KTX2 size: {} bytes ({:.1}% of original)",
        uastc_data.len(),
        uastc_data.len() as f64
            / std::fs::metadata(input_path)
                .map_err(|_| ktx2_rw::Error::FileReadError)?
                .len() as f64
            * 100.0
    );

    // Save the ETC1S version (smaller) as the default output
    std::fs::write(output_path, &etc1s_data).map_err(|_| ktx2_rw::Error::FileWriteError)?;

    // Save both versions for comparison
    let input_stem = Path::new(input_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(ktx2_rw::Error::InvalidValue)?;
    let etc1s_path = format!("{input_stem}.etc1s.ktx2");
    let uastc_path = format!("{input_stem}.uastc.ktx2");

    std::fs::write(&etc1s_path, &etc1s_data).map_err(|_| ktx2_rw::Error::FileWriteError)?;
    std::fs::write(&uastc_path, &uastc_data).map_err(|_| ktx2_rw::Error::FileWriteError)?;

    println!("Saved files:");
    println!("  - {etc1s_path} (ETC1S)");
    println!("  - {uastc_path} (UASTC)");

    // Step 7: Verify by loading back and transcoding
    println!("\n7. Verifying conversion by loading back...");
    let loaded_texture = Ktx2Texture::from_memory(&etc1s_data)?;

    println!(
        "Loaded texture: {}x{} (format: {})",
        loaded_texture.width(),
        loaded_texture.height(),
        loaded_texture.vk_format()
    );
    println!("Is compressed: {}", loaded_texture.is_compressed());
    println!("Needs transcoding: {}", loaded_texture.needs_transcoding());

    // Read back metadata
    if let Ok(original_format) = loaded_texture.get_metadata("OriginalFormat") {
        println!(
            "Original format: {}",
            String::from_utf8_lossy(&original_format)
        );
    }
    if let Ok(compression_mode) = loaded_texture.get_metadata("CompressionMode") {
        println!(
            "Compression mode: {}",
            String::from_utf8_lossy(&compression_mode)
        );
    }

    // Step 8: Demonstrate transcoding to different GPU formats
    println!("\n8. Demonstrating transcoding to GPU formats...");

    let formats = vec![
        (TranscodeFormat::Bc7Rgba, "BC7 (Desktop)"),
        (TranscodeFormat::Etc2Rgba, "ETC2 (Mobile)"),
        (TranscodeFormat::Astc_4x4_Rgba, "ASTC 4x4 (Mobile)"),
        (TranscodeFormat::Rgba32, "RGBA32 (Universal)"),
    ];

    for (format, name) in formats {
        let mut transcoded = Ktx2Texture::from_memory(&etc1s_data)?;
        transcoded.transcode_basis(format)?;
        let transcoded_data = transcoded.get_image_data(0, 0, 0)?;
        println!("  - {}: {} bytes", name, transcoded_data.len());
    }

    println!("\n✅ PNG to KTX2 conversion completed successfully!");
    println!("\nUsage:");
    println!("  cargo run --example png_to_ktx2                    # Use sample image");
    println!("  cargo run --example png_to_ktx2 input.png          # Convert specific PNG");
    println!("  cargo run --example png_to_ktx2 input.png out.ktx2 # Specify output file");

    Ok(())
}

// Helper function to create a sample PNG for testing
fn create_sample_png(path: &str) -> Result<()> {
    use image::{ImageBuffer, Rgba};

    println!("Creating sample PNG image...");

    let width = 256;
    let height = 256;

    // Create a colorful gradient pattern
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let r = ((x as f32 / width as f32) * 255.0) as u8;
        let g = ((y as f32 / height as f32) * 255.0) as u8;
        let b = (((x + y) as f32 / (width + height) as f32) * 255.0) as u8;
        let a = 255u8;

        // Add some pattern variation
        let pattern = if (x / 32) % 2 == (y / 32) % 2 { 50 } else { 0 };

        Rgba([
            r.saturating_add(pattern),
            g.saturating_add(pattern),
            b.saturating_add(pattern),
            a,
        ])
    });

    img.save(path).map_err(|_| ktx2_rw::Error::FileWriteError)?;
    println!("Created sample image: {path} ({width}x{height})");

    Ok(())
}
