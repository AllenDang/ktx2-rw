//! 2D array texture example.
//!
//! Creates a 128x128 RGBA8 texture array with 4 layers, fills each layer with
//! a different gradient, writes the KTX2 file out, and reads it back.
//!
//! 2D arrays are the common stand-in when a renderer does not support 3D
//! textures: depth=1, faces=1, layers>1.

use ktx2_rw::{Ktx2Texture, Result, VkFormat};

fn main() -> Result<()> {
    let width: u32 = 128;
    let height: u32 = 128;
    let layers: u32 = 4;

    println!(
        "Creating {}x{} array texture with {} layers (RGBA8)...",
        width, height, layers
    );
    let mut texture = Ktx2Texture::create(
        width,
        height,
        1,      // depth (must be 1 for arrays)
        layers, // layers
        1,      // faces
        1,      // mip levels
        VkFormat::R8G8B8A8_UNORM,
    )?;
    assert!(texture.is_array());

    let layer_size = (width * height * 4) as usize;
    for layer in 0..layers {
        let mut data = Vec::with_capacity(layer_size);
        for y in 0..height {
            for x in 0..width {
                data.push((x * 2) as u8);
                data.push((y * 2) as u8);
                data.push((layer * 64) as u8);
                data.push(255);
            }
        }
        // face_slice is 0 for non-cubemap, non-3D textures.
        texture.set_image_data(0, layer, 0, &data)?;
    }

    texture.set_metadata("Author", b"ktx2-rw texture_array example")?;

    let ktx_bytes = texture.write_to_memory()?;
    println!("Wrote KTX2 in memory: {} bytes", ktx_bytes.len());

    let loaded = Ktx2Texture::from_memory(&ktx_bytes)?;
    println!("{:?}", loaded);
    assert!(loaded.is_array());
    assert_eq!(loaded.layers(), layers);

    for layer in 0..layers {
        let data = loaded.get_image_data(0, layer, 0)?;
        println!(
            "layer {}: {} bytes (first RGBA=[{}, {}, {}, {}])",
            layer,
            data.len(),
            data[0],
            data[1],
            data[2],
            data[3],
        );
    }

    Ok(())
}
