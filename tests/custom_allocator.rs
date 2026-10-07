//! Regression test: buffers returned by libktx are allocated with C `malloc`, so they
//! must not be freed through Rust's global allocator.

use ktx2_rw::{Ktx2Texture, VkFormat};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[test]
fn custom_global_allocator() {
    let (w, h) = (64u32, 64u32);
    let mut texture = Ktx2Texture::create(w, h, 1, 1, 1, 1, VkFormat::R8G8B8A8_UNORM).unwrap();
    let data = vec![0x7fu8; (w * h * 4) as usize];
    texture.set_image_data(0, 0, 0, &data).unwrap();

    for _ in 0..16 {
        let bytes = texture.write_to_memory().unwrap();
        let reloaded = Ktx2Texture::from_memory(&bytes).unwrap();
        assert_eq!(reloaded.width(), w);
        assert_eq!(reloaded.height(), h);
    }
}
