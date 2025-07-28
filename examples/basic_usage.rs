use ktx2_rw::{BasisCompressionParams, Ktx2Texture, Result, TranscodeFormat, VkFormat};

fn main() -> Result<()> {
    println!("KTX2 Rust Wrapper Example");

    // Example 1: Create a new KTX2 texture from scratch
    println!("\n1. Creating a new KTX2 texture...");
    let mut texture = Ktx2Texture::create(
        512,                     // width
        512,                     // height
        1,                       // depth
        1,                       // layers
        1,                       // faces
        1,                       // levels
        VkFormat::R8G8B8A8Unorm, // vk_format (VK_FORMAT_R8G8B8A8_UNORM)
    )?;

    println!(
        "Created texture: {}x{} (format: {:?})",
        texture.width(),
        texture.height(),
        texture.vk_format()
    );

    // Example 2: Add some dummy RGBA data
    println!("\n2. Adding image data...");
    let size = (texture.width() * texture.height() * 4) as usize;
    let dummy_data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
    texture.set_image_data(0, 0, 0, &dummy_data)?;

    // Example 3: Add metadata
    println!("\n3. Adding metadata...");
    texture.set_metadata("Author", b"KTX2-RW Example")?;
    texture.set_metadata("Description", b"Example texture created with ktx2-rw")?;

    // Example 4: Compress with Basis Universal
    println!("\n4. Compressing with Basis Universal...");
    let compression_params = BasisCompressionParams::builder()
        .uastc(false) // Use ETC1S for better compression
        .quality_level(128)
        .thread_count(4)
        .build();

    texture.compress_basis(&compression_params)?;
    println!("Compressed texture successfully");

    // Example 5: Write to memory and show size
    println!("\n5. Writing to memory...");
    let ktx_data = texture.write_to_memory()?;
    println!("KTX2 file size: {} bytes", ktx_data.len());

    // Example 6: Load the texture back from memory
    println!("\n6. Loading texture from memory...");
    let loaded_texture = Ktx2Texture::from_memory(&ktx_data)?;

    println!(
        "Loaded texture: {}x{} (format: {:?})",
        loaded_texture.width(),
        loaded_texture.height(),
        loaded_texture.vk_format()
    );
    println!("Is compressed: {}", loaded_texture.is_compressed());
    println!("Needs transcoding: {}", loaded_texture.needs_transcoding());

    // Example 7: Read metadata
    println!("\n7. Reading metadata...");
    if let Ok(author) = loaded_texture.get_metadata("Author") {
        println!("Author: {}", String::from_utf8_lossy(&author));
    }
    if let Ok(desc) = loaded_texture.get_metadata("Description") {
        println!("Description: {}", String::from_utf8_lossy(&desc));
    }

    // Example 8: Demonstrate transcoding (if needed)
    if loaded_texture.needs_transcoding() {
        println!("\n8. Transcoding to BC7...");
        let mut transcoded = loaded_texture;
        transcoded.transcode_basis(TranscodeFormat::Bc7Rgba)?;
        println!("Transcoded successfully");

        // Get the transcoded image data
        let image_data = transcoded.get_image_data(0, 0, 0)?;
        println!("Transcoded image data size: {} bytes", image_data.len());
    }

    println!("\nExample completed successfully!");
    Ok(())
}
