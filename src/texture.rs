use std::ffi::CString;
use std::fmt;
use std::path::Path;
use std::ptr;

use crate::bindings::*;
use crate::compression::BasisCompressionParams;
use crate::error::{Error, Result};
use crate::format::TranscodeFormat;

/// Main texture handle for KTX2 textures
///
/// This struct provides a safe, high-level interface to KTX2 textures with automatic
/// memory management. All operations are memory-safe and use proper error handling.
///
/// ## Thread Safety
///
/// `Ktx2Texture` implements `Send + Sync` and can be safely used across threads.
///
/// ## Memory Management
///
/// The texture handle automatically manages the underlying C resources via RAII.
/// When dropped, all associated memory is properly freed.
///
/// # Examples
///
/// ```rust,no_run
/// use ktx2_rw::{Ktx2Texture, BasisCompressionParams, TranscodeFormat};
/// # fn main() -> ktx2_rw::Result<()> {
///
/// // Create a new texture
/// let mut texture = Ktx2Texture::create(512, 512, 1, 1, 1, 1, 37)?;
///
/// // Load from file
/// let texture = Ktx2Texture::from_file("texture.ktx2")?;
///
/// // Load from memory
/// let data = std::fs::read("texture.ktx2").map_err(|_| ktx2_rw::Error::FileReadError)?;
/// let texture = Ktx2Texture::from_memory(&data)?;
/// # Ok(())
/// # }
/// ```
pub struct Ktx2Texture {
    texture: *mut ktxTexture2,
}

impl Ktx2Texture {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_str().ok_or(Error::InvalidValue)?;
        let c_path = CString::new(path_str).map_err(|_| Error::InvalidValue)?;

        let mut texture: *mut ktxTexture2 = ptr::null_mut();

        let result = unsafe {
            ktxTexture2_CreateFromNamedFile(
                c_path.as_ptr(),
                ktxTextureCreateFlagBits_KTX_TEXTURE_CREATE_LOAD_IMAGE_DATA_BIT,
                &mut texture,
            )
        };

        if result != ktx_error_code_e_KTX_SUCCESS {
            return Err(result.into());
        }

        Ok(Self { texture })
    }

    pub fn from_memory(data: &[u8]) -> Result<Self> {
        let mut texture: *mut ktxTexture2 = ptr::null_mut();

        let result = unsafe {
            ktxTexture2_CreateFromMemory(
                data.as_ptr(),
                data.len(),
                ktxTextureCreateFlagBits_KTX_TEXTURE_CREATE_LOAD_IMAGE_DATA_BIT,
                &mut texture,
            )
        };

        if result != ktx_error_code_e_KTX_SUCCESS {
            return Err(result.into());
        }

        Ok(Self { texture })
    }

    pub fn create(
        width: u32,
        height: u32,
        depth: u32,
        layers: u32,
        faces: u32,
        levels: u32,
        vk_format: u32,
    ) -> Result<Self> {
        // Validate input parameters
        if width == 0 || height == 0 {
            return Err(Error::InvalidValue);
        }
        if depth == 0 || layers == 0 || faces == 0 || levels == 0 {
            return Err(Error::InvalidValue);
        }

        // Validate reasonable limits to prevent excessive memory allocation
        const MAX_DIMENSION: u32 = 65536; // 64K max dimension
        if width > MAX_DIMENSION || height > MAX_DIMENSION || depth > MAX_DIMENSION {
            return Err(Error::InvalidValue);
        }

        let create_info = ktxTextureCreateInfo {
            glInternalformat: 0,
            vkFormat: vk_format,
            pDfd: ptr::null_mut(),
            baseWidth: width,
            baseHeight: height,
            baseDepth: depth,
            numDimensions: if depth > 1 {
                3
            } else if height > 1 {
                2
            } else {
                1
            },
            numLevels: levels,
            numLayers: layers,
            numFaces: faces,
            isArray: layers > 1,
            generateMipmaps: false,
        };

        let mut texture: *mut ktxTexture2 = ptr::null_mut();

        let result = unsafe {
            ktxTexture2_Create(
                &create_info,
                ktxTextureCreateStorageEnum_KTX_TEXTURE_CREATE_ALLOC_STORAGE,
                &mut texture,
            )
        };

        if result != ktx_error_code_e_KTX_SUCCESS {
            return Err(result.into());
        }

        // Safety: Check that texture was actually created
        if texture.is_null() {
            return Err(Error::OutOfMemory);
        }

        Ok(Self { texture })
    }

    pub fn width(&self) -> u32 {
        if self.texture.is_null() {
            return 0;
        }
        unsafe { (*self.texture).baseWidth }
    }

    pub fn height(&self) -> u32 {
        if self.texture.is_null() {
            return 0;
        }
        unsafe { (*self.texture).baseHeight }
    }

    pub fn depth(&self) -> u32 {
        if self.texture.is_null() {
            return 0;
        }
        unsafe { (*self.texture).baseDepth }
    }

    pub fn layers(&self) -> u32 {
        if self.texture.is_null() {
            return 0;
        }
        unsafe { (*self.texture).numLayers }
    }

    pub fn faces(&self) -> u32 {
        if self.texture.is_null() {
            return 0;
        }
        unsafe { (*self.texture).numFaces }
    }

    pub fn levels(&self) -> u32 {
        if self.texture.is_null() {
            return 0;
        }
        unsafe { (*self.texture).numLevels }
    }

    pub fn vk_format(&self) -> u32 {
        if self.texture.is_null() {
            return 0;
        }
        unsafe { (*self.texture).vkFormat }
    }

    pub fn is_array(&self) -> bool {
        if self.texture.is_null() {
            return false;
        }
        unsafe { (*self.texture).isArray }
    }

    pub fn is_cubemap(&self) -> bool {
        if self.texture.is_null() {
            return false;
        }
        unsafe { (*self.texture).isCubemap }
    }

    pub fn is_compressed(&self) -> bool {
        if self.texture.is_null() {
            return false;
        }
        unsafe { (*self.texture).isCompressed }
    }

    pub fn needs_transcoding(&self) -> bool {
        unsafe {
            // Safety: Check texture is valid first
            if self.texture.is_null() {
                return false;
            }

            let vtbl = (*self.texture).vtbl;
            if vtbl.is_null() {
                return false;
            }

            let needs_transcoding = (*vtbl).NeedsTranscoding;
            match needs_transcoding {
                Some(func) => func(self.texture as *mut ktxTexture),
                None => false,
            }
        }
    }

    pub fn get_image_data(&self, level: u32, layer: u32, face: u32) -> Result<&[u8]> {
        // Safety: Check texture validity first
        if self.texture.is_null() {
            return Err(Error::InvalidOperation);
        }

        // Validate parameters against texture properties
        let texture = unsafe { &*self.texture };
        if level >= texture.numLevels || layer >= texture.numLayers || face >= texture.numFaces {
            return Err(Error::InvalidValue);
        }

        let mut offset = 0usize;

        let result = unsafe {
            let vtbl = texture.vtbl;
            if vtbl.is_null() {
                return Err(Error::InvalidOperation);
            }

            let get_image_offset = (*vtbl).GetImageOffset;
            match get_image_offset {
                Some(func) => func(
                    self.texture as *mut ktxTexture,
                    level,
                    layer,
                    face,
                    &mut offset,
                ),
                None => return Err(Error::UnsupportedFeature),
            }
        };

        if result != ktx_error_code_e_KTX_SUCCESS {
            return Err(result.into());
        }

        let size = unsafe {
            let vtbl = texture.vtbl;
            let get_image_size = (*vtbl).GetImageSize;
            match get_image_size {
                Some(func) => func(self.texture as *mut ktxTexture, level),
                None => return Err(Error::UnsupportedFeature),
            }
        };

        // Safety: Validate the calculated bounds
        if texture.pData.is_null() {
            return Err(Error::InvalidOperation);
        }

        let data_size = texture.dataSize;
        if offset.saturating_add(size) > data_size {
            return Err(Error::InvalidOperation);
        }

        unsafe {
            let data_ptr = texture.pData.add(offset);
            Ok(std::slice::from_raw_parts(data_ptr, size))
        }
    }

    pub fn set_image_data(&mut self, level: u32, layer: u32, face: u32, data: &[u8]) -> Result<()> {
        // Safety: Check texture validity first
        if self.texture.is_null() {
            return Err(Error::InvalidOperation);
        }

        // Validate parameters against texture properties
        let texture = unsafe { &*self.texture };
        if level >= texture.numLevels || layer >= texture.numLayers || face >= texture.numFaces {
            return Err(Error::InvalidValue);
        }

        // Check data is not empty
        if data.is_empty() {
            return Err(Error::InvalidValue);
        }

        let result = unsafe {
            let vtbl = texture.vtbl;
            if vtbl.is_null() {
                return Err(Error::InvalidOperation);
            }

            let set_image_from_memory = (*vtbl).SetImageFromMemory;
            match set_image_from_memory {
                Some(func) => func(
                    self.texture as *mut ktxTexture,
                    level,
                    layer,
                    face,
                    data.as_ptr(),
                    data.len(),
                ),
                None => return Err(Error::UnsupportedFeature),
            }
        };

        if result != ktx_error_code_e_KTX_SUCCESS {
            return Err(result.into());
        }

        Ok(())
    }

    pub fn transcode_basis(&mut self, format: TranscodeFormat) -> Result<()> {
        let result = unsafe { ktxTexture2_TranscodeBasis(self.texture, format.into(), 0) };

        if result != ktx_error_code_e_KTX_SUCCESS {
            return Err(result.into());
        }

        Ok(())
    }

    pub fn compress_basis(&mut self, params: &BasisCompressionParams) -> Result<()> {
        let mut ktx_params: ktxBasisParams = params.into();

        let result = unsafe { ktxTexture2_CompressBasisEx(self.texture, &mut ktx_params) };

        if result != ktx_error_code_e_KTX_SUCCESS {
            return Err(result.into());
        }

        Ok(())
    }

    pub fn compress_basis_simple(&mut self, quality: u32) -> Result<()> {
        let result = unsafe { ktxTexture2_CompressBasis(self.texture, quality) };

        if result != ktx_error_code_e_KTX_SUCCESS {
            return Err(result.into());
        }

        Ok(())
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path_str = path.as_ref().to_str().ok_or(Error::InvalidValue)?;
        let c_path = CString::new(path_str).map_err(|_| Error::InvalidValue)?;

        let result = unsafe { ktxTexture2_WriteToNamedFile(self.texture, c_path.as_ptr()) };

        if result != ktx_error_code_e_KTX_SUCCESS {
            return Err(result.into());
        }

        Ok(())
    }

    pub fn write_to_memory(&self) -> Result<Vec<u8>> {
        let mut data: *mut ktx_uint8_t = ptr::null_mut();
        let mut size: ktx_size_t = 0;

        let result = unsafe { ktxTexture2_WriteToMemory(self.texture, &mut data, &mut size) };

        if result != ktx_error_code_e_KTX_SUCCESS {
            return Err(result.into());
        }

        // Safety: Check that we got valid data before proceeding
        if data.is_null() || size == 0 {
            return Err(Error::InvalidOperation);
        }

        // Create Vec from the allocated memory - this is safe because:
        // 1. data is non-null and size > 0 (checked above)
        // 2. ktxTexture2_WriteToMemory allocates with malloc()
        // 3. Vec::from_raw_parts will take ownership and free with the global allocator
        let vec = unsafe { Vec::from_raw_parts(data, size, size) };

        Ok(vec)
    }

    pub fn get_metadata(&self, key: &str) -> Result<Vec<u8>> {
        let c_key = CString::new(key).map_err(|_| Error::InvalidValue)?;

        let mut value_len = 0u32;
        let mut value: *mut libc::c_void = ptr::null_mut();

        let result = unsafe {
            ktxHashList_FindValue(
                &mut (*self.texture).kvDataHead,
                c_key.as_ptr(),
                &mut value_len,
                &mut value,
            )
        };

        if result != ktx_error_code_e_KTX_SUCCESS {
            return Err(result.into());
        }

        if value.is_null() {
            return Err(Error::NotFound);
        }

        let data =
            unsafe { std::slice::from_raw_parts(value as *const u8, value_len as usize).to_vec() };

        Ok(data)
    }

    pub fn set_metadata(&mut self, key: &str, value: &[u8]) -> Result<()> {
        let c_key = CString::new(key).map_err(|_| Error::InvalidValue)?;

        let result = unsafe {
            ktxHashList_AddKVPair(
                &mut (*self.texture).kvDataHead,
                c_key.as_ptr(),
                value.len() as u32,
                value.as_ptr() as *const libc::c_void,
            )
        };

        if result != ktx_error_code_e_KTX_SUCCESS {
            return Err(result.into());
        }

        Ok(())
    }
}

impl Drop for Ktx2Texture {
    fn drop(&mut self) {
        if !self.texture.is_null() {
            unsafe {
                ktxTexture2_Destroy(self.texture);
            }
        }
    }
}

impl fmt::Debug for Ktx2Texture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ktx2Texture")
            .field("width", &self.width())
            .field("height", &self.height())
            .field("depth", &self.depth())
            .field("layers", &self.layers())
            .field("faces", &self.faces())
            .field("levels", &self.levels())
            .field("vk_format", &self.vk_format())
            .field("is_array", &self.is_array())
            .field("is_cubemap", &self.is_cubemap())
            .field("is_compressed", &self.is_compressed())
            .finish()
    }
}

unsafe impl Send for Ktx2Texture {}
unsafe impl Sync for Ktx2Texture {}
