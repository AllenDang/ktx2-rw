//! Cubemap example.
//!
//! Creates a 256x256 RGBA8 cubemap (6 faces), paints each face a distinct
//! solid color, writes the KTX2 file out, and reads it back.
//!
//! Cubemap face ordering follows the KTX2 / Vulkan convention:
//!   0: +X   1: -X   2: +Y   3: -Y   4: +Z   5: -Z

use ktx2_rw::{Ktx2Texture, Result, VkFormat};

fn main() -> Result<()> {
    let size: u32 = 256;

    println!("Creating {0}x{0} cubemap (RGBA8, 6 faces)...", size);
    let mut texture = Ktx2Texture::create(
        size,
        size,
        1, // depth
        1, // layers
        6, // faces (cubemap)
        1, // mip levels
        VkFormat::R8G8B8A8_UNORM,
    )?;
    assert!(texture.is_cubemap());

    let face_colors: [[u8; 4]; 6] = [
        [255, 0, 0, 255],   // +X red
        [0, 255, 255, 255], // -X cyan
        [0, 255, 0, 255],   // +Y green
        [255, 0, 255, 255], // -Y magenta
        [0, 0, 255, 255],   // +Z blue
        [255, 255, 0, 255], // -Z yellow
    ];

    let face_size = (size * size * 4) as usize;
    for (face, color) in face_colors.iter().enumerate() {
        let mut data = Vec::with_capacity(face_size);
        for _ in 0..(size * size) {
            data.extend_from_slice(color);
        }
        texture.set_image_data(0, 0, face as u32, &data)?;
    }

    texture.set_metadata("Author", b"ktx2-rw cubemap example")?;

    let ktx_bytes = texture.write_to_memory()?;
    println!("Wrote KTX2 in memory: {} bytes", ktx_bytes.len());

    let loaded = Ktx2Texture::from_memory(&ktx_bytes)?;
    println!("{:?}", loaded);
    assert!(loaded.is_cubemap());
    assert_eq!(loaded.faces(), 6);

    for (face, expected) in face_colors.iter().enumerate() {
        let data = loaded.get_image_data(0, 0, face as u32)?;
        assert_eq!(&data[..4], expected);
        println!("face {}: first RGBA = {:?} (ok)", face, &data[..4]);
    }

    Ok(())
}
