pub struct HexConfig<D> {
    /// Base offset that this buffer starts at
    pub offset: usize,
    /// Align the buffer to the display width
    pub align: bool,
    /// Width of the dump
    pub width: usize,
    /// Width to split the dump
    pub split: usize,
    /// Separator rules
    pub separators: SeparatorConfig,
    /// Coloring rules
    pub colors: ColorConfig,
    /// Decoder
    pub decoder: D,
}

impl<D: Default> Default for HexConfig<D> {
    fn default() -> Self {
        Self {
            offset: 0,
            align: true,
            width: 16,
            split: 8,
            separators: SeparatorConfig,
            colors: ColorConfig,
            decoder: D::default(),
        }
    }
}

pub struct SeparatorConfig;
pub struct ColorConfig;
