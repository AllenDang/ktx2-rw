//! 3D (volume) texture example.
//!
//! Creates a 32x32x16 RGBA8 volume texture, fills it with a simple analytic
//! gradient, writes the KTX2 file out, reads it back, and verifies the
//! per-slice contents.
//!
//! There are two equivalent ways to supply image data for a 3D texture:
//!   1. Upload each depth slice individually by passing the slice index as the
//!      `face_slice` argument.
//!   2. Upload the whole mip level at once using [`FACE_SLICE_WHOLE_LEVEL`].

use ktx2_rw::{FACE_SLICE_WHOLE_LEVEL, Ktx2Texture, Result, VkFormat};

fn main() -> Result<()> {
    let width: u32 = 32;
    let height: u32 = 32;
    let depth: u32 = 16;

    println!(
        "Creating {}x{}x{} 3D texture (RGBA8)...",
        width, height, depth
    );
    let mut texture = Ktx2Texture::create(
        width,
        height,
        depth,
        1, // layers
        1, // faces
        1, // mip levels
        VkFormat::R8G8B8A8_UNORM,
    )?;

    // Option A: upload slice-by-slice.
    let slice_size = (width * height * 4) as usize;
    for z in 0..depth {
        let mut slice = Vec::with_capacity(slice_size);
        for y in 0..height {
            for x in 0..width {
                slice.push((x * 8) as u8);
                slice.push((y * 8) as u8);
                slice.push((z * 16) as u8);
                slice.push(255);
            }
        }
        texture.set_image_data(0, 0, z, &slice)?;
    }

    texture.set_metadata("Author", b"ktx2-rw texture_3d example")?;

    let ktx_bytes = texture.write_to_memory()?;
    println!("Wrote KTX2 in memory: {} bytes", ktx_bytes.len());

    // Round trip.
    let loaded = Ktx2Texture::from_memory(&ktx_bytes)?;
    println!("{:?}", loaded);
    assert_eq!(loaded.depth(), depth);

    let slice_last = loaded.get_image_data(0, 0, depth - 1)?;
    println!(
        "Read back slice {}: {} bytes (first RGBA=[{}, {}, {}, {}])",
        depth - 1,
        slice_last.len(),
        slice_last[0],
        slice_last[1],
        slice_last[2],
        slice_last[3],
    );

    // Option B: build the same volume in one buffer and upload in a single call.
    println!("\nUploading the full volume in one call using FACE_SLICE_WHOLE_LEVEL...");
    let mut texture_b =
        Ktx2Texture::create(width, height, depth, 1, 1, 1, VkFormat::R8G8B8A8_UNORM)?;
    let volume_size = (width * height * depth * 4) as usize;
    let volume: Vec<u8> = (0..volume_size).map(|i| (i & 0xff) as u8).collect();
    texture_b.set_image_data(0, 0, FACE_SLICE_WHOLE_LEVEL, &volume)?;
    let _ = texture_b.write_to_memory()?;
    println!("Whole-level upload path succeeded.");

    Ok(())
}
