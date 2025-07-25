use crate::bindings::*;

/// Configuration parameters for Basis Universal compression
///
/// This struct provides comprehensive control over the Basis Universal compression
/// process, supporting both ETC1S and UASTC compression modes.
///
/// ## ETC1S Mode (default)
/// - Smaller file sizes
/// - Longer compression times
/// - Good for textures that will be transcoded to multiple formats
///
/// ## UASTC Mode
/// - Higher quality
/// - Larger file sizes
/// - Better for high-quality textures
///
/// # Examples
///
/// ```rust
/// use ktx2_rw::BasisCompressionParams;
///
/// // Using builder pattern for high-quality UASTC compression
/// let params = BasisCompressionParams::builder()
///     .uastc(true)
///     .quality_level(255)
///     .uastc_rdo_quality_scalar(1.0)
///     .thread_count(8)
///     .build();
///
/// // Using builder pattern for fast ETC1S compression
/// let params = BasisCompressionParams::builder()
///     .uastc(false)
///     .quality_level(128)
///     .thread_count(8)
///     .max_endpoints(8000)
///     .max_selectors(8000)
///     .build();
///
/// // Traditional struct initialization still works
/// let mut params = BasisCompressionParams::builder().build();
/// params.uastc = true;
/// params.quality_level = 255;
/// ```
#[derive(Debug, Clone)]
pub struct BasisCompressionParams {
    pub uastc: bool,
    pub thread_count: u32,
    pub compression_level: u32,
    pub quality_level: u32,
    pub max_endpoints: u32,
    pub endpoint_rdo_threshold: f32,
    pub max_selectors: u32,
    pub selector_rdo_threshold: f32,
    pub normal_map: bool,
    pub separate_rg_to_color_alpha: bool,
    pub pre_swizzle: bool,
    pub no_endpoint_rdo: bool,
    pub no_selector_rdo: bool,
    pub uastc_flags: u32,
    pub uastc_rdo: bool,
    pub uastc_rdo_quality_scalar: f32,
    pub uastc_rdo_dict_size: u32,
    pub input_swizzle: [u8; 4],
}

/// Builder for [`BasisCompressionParams`]
///
/// Provides a fluent API for configuring Basis Universal compression parameters.
///
/// # Examples
///
/// ```rust
/// use ktx2_rw::BasisCompressionParams;
///
/// let params = BasisCompressionParams::builder()
///     .uastc(true)
///     .quality_level(255)
///     .thread_count(8)
///     .build();
/// ```
pub struct BasisCompressionParamsBuilder {
    params: BasisCompressionParams,
}

impl BasisCompressionParams {
    /// Creates a new builder for `BasisCompressionParams`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ktx2_rw::BasisCompressionParams;
    ///
    /// let params = BasisCompressionParams::builder()
    ///     .uastc(true)
    ///     .quality_level(200)
    ///     .build();
    /// ```
    pub fn builder() -> BasisCompressionParamsBuilder {
        BasisCompressionParamsBuilder {
            params: Self::default(),
        }
    }
}

impl BasisCompressionParamsBuilder {
    /// Sets the compression mode
    ///
    /// - `true`: UASTC mode (higher quality, larger files)
    /// - `false`: ETC1S mode (smaller files, longer compression times)
    ///
    /// Default: `false` (ETC1S)
    pub fn uastc(mut self, uastc: bool) -> Self {
        self.params.uastc = uastc;
        self
    }

    /// Sets the number of threads to use for compression
    ///
    /// Default: `1`
    pub fn thread_count(mut self, count: u32) -> Self {
        self.params.thread_count = count;
        self
    }

    /// Sets the compression level (ETC1S mode only)
    ///
    /// Higher values = better compression but slower
    ///
    /// Default: varies by platform
    pub fn compression_level(mut self, level: u32) -> Self {
        self.params.compression_level = level;
        self
    }

    /// Sets the quality level
    ///
    /// - ETC1S mode: 1-255 (128 = good balance)
    /// - UASTC mode: 0-4 (higher = better quality)
    ///
    /// Default: `128`
    pub fn quality_level(mut self, quality: u32) -> Self {
        self.params.quality_level = quality;
        self
    }

    /// Sets the maximum number of endpoints (ETC1S mode only)
    ///
    /// 0 = automatic selection
    ///
    /// Default: `0`
    pub fn max_endpoints(mut self, max: u32) -> Self {
        self.params.max_endpoints = max;
        self
    }

    /// Sets the endpoint RDO threshold (ETC1S mode only)
    ///
    /// Higher values = more aggressive optimization
    ///
    /// Default: `0.0`
    pub fn endpoint_rdo_threshold(mut self, threshold: f32) -> Self {
        self.params.endpoint_rdo_threshold = threshold;
        self
    }

    /// Sets the maximum number of selectors (ETC1S mode only)
    ///
    /// 0 = automatic selection
    ///
    /// Default: `0`
    pub fn max_selectors(mut self, max: u32) -> Self {
        self.params.max_selectors = max;
        self
    }

    /// Sets the selector RDO threshold (ETC1S mode only)
    ///
    /// Higher values = more aggressive optimization
    ///
    /// Default: `0.0`
    pub fn selector_rdo_threshold(mut self, threshold: f32) -> Self {
        self.params.selector_rdo_threshold = threshold;
        self
    }

    /// Configures for normal map compression
    ///
    /// Optimizes compression for normal maps
    ///
    /// Default: `false`
    pub fn normal_map(mut self, is_normal_map: bool) -> Self {
        self.params.normal_map = is_normal_map;
        self
    }

    /// Separates RG to color+alpha channels
    ///
    /// Useful for normal maps and other specialized textures
    ///
    /// Default: `false`
    pub fn separate_rg_to_color_alpha(mut self, separate: bool) -> Self {
        self.params.separate_rg_to_color_alpha = separate;
        self
    }

    /// Enables pre-swizzling
    ///
    /// Default: `false`
    pub fn pre_swizzle(mut self, pre_swizzle: bool) -> Self {
        self.params.pre_swizzle = pre_swizzle;
        self
    }

    /// Disables endpoint RDO optimization (ETC1S mode only)
    ///
    /// Default: `false`
    pub fn no_endpoint_rdo(mut self, disable: bool) -> Self {
        self.params.no_endpoint_rdo = disable;
        self
    }

    /// Disables selector RDO optimization (ETC1S mode only)
    ///
    /// Default: `false`  
    pub fn no_selector_rdo(mut self, disable: bool) -> Self {
        self.params.no_selector_rdo = disable;
        self
    }

    /// Sets UASTC compression flags (UASTC mode only)
    ///
    /// Default: `0`
    pub fn uastc_flags(mut self, flags: u32) -> Self {
        self.params.uastc_flags = flags;
        self
    }

    /// Enables UASTC RDO optimization (UASTC mode only)
    ///
    /// Default: `false`
    pub fn uastc_rdo(mut self, enable: bool) -> Self {
        self.params.uastc_rdo = enable;
        self
    }

    /// Sets UASTC RDO quality scalar (UASTC mode only)
    ///
    /// Higher values = better quality but larger files
    ///
    /// Default: `1.0`
    pub fn uastc_rdo_quality_scalar(mut self, scalar: f32) -> Self {
        self.params.uastc_rdo_quality_scalar = scalar;
        self
    }

    /// Sets UASTC RDO dictionary size (UASTC mode only)
    ///
    /// Default: `4096`
    pub fn uastc_rdo_dict_size(mut self, size: u32) -> Self {
        self.params.uastc_rdo_dict_size = size;
        self
    }

    /// Sets the input channel swizzle
    ///
    /// [0, 1, 2, 3] = RGBA (no swizzle)
    /// [2, 1, 0, 3] = BGRA (red-blue swap)
    ///
    /// Default: `[0, 1, 2, 3]`
    pub fn input_swizzle(mut self, swizzle: [u8; 4]) -> Self {
        self.params.input_swizzle = swizzle;
        self
    }

    /// Builds the final `BasisCompressionParams`
    pub fn build(self) -> BasisCompressionParams {
        self.params
    }
}

impl Default for BasisCompressionParams {
    fn default() -> Self {
        Self {
            uastc: false,
            thread_count: 1,
            compression_level: unsafe { KTX_ETC1S_DEFAULT_COMPRESSION_LEVEL },
            quality_level: 128,
            max_endpoints: 0,
            endpoint_rdo_threshold: 0.0,
            max_selectors: 0,
            selector_rdo_threshold: 0.0,
            normal_map: false,
            separate_rg_to_color_alpha: false,
            pre_swizzle: false,
            no_endpoint_rdo: false,
            no_selector_rdo: false,
            uastc_flags: 0,
            uastc_rdo: false,
            uastc_rdo_quality_scalar: 1.0,
            uastc_rdo_dict_size: 4096,
            input_swizzle: [0, 1, 2, 3],
        }
    }
}

impl From<&BasisCompressionParams> for ktxBasisParams {
    fn from(params: &BasisCompressionParams) -> Self {
        let mut ktx_params = ktxBasisParams {
            structSize: std::mem::size_of::<ktxBasisParams>() as u32,
            uastc: params.uastc,
            verbose: false,
            noSSE: false,
            threadCount: params.thread_count,
            compressionLevel: params.compression_level,
            qualityLevel: params.quality_level,
            maxEndpoints: params.max_endpoints,
            endpointRDOThreshold: params.endpoint_rdo_threshold,
            maxSelectors: params.max_selectors,
            selectorRDOThreshold: params.selector_rdo_threshold,
            inputSwizzle: [0; 4],
            normalMap: params.normal_map,
            separateRGToRGB_A: params.separate_rg_to_color_alpha,
            preSwizzle: params.pre_swizzle,
            noEndpointRDO: params.no_endpoint_rdo,
            noSelectorRDO: params.no_selector_rdo,
            uastcFlags: params.uastc_flags,
            uastcRDO: params.uastc_rdo,
            uastcRDOQualityScalar: params.uastc_rdo_quality_scalar,
            uastcRDODictSize: params.uastc_rdo_dict_size,
            uastcRDOMaxSmoothBlockErrorScale: 10.0,
            uastcRDOMaxSmoothBlockStdDev: 18.0,
            uastcRDODontFavorSimplerModes: false,
            uastcRDONoMultithreading: false,
        };

        for i in 0..4 {
            ktx_params.inputSwizzle[i] = params.input_swizzle[i] as i8;
        }

        ktx_params
    }
}
