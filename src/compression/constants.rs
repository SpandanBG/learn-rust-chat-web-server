pub const MIN_SIZE_TO_COMPRESS: usize = 150; // We will compress anything that is above 150 Bytes

pub const COMPRESSION_TYPE_NONE: &'static str = "none";
pub const COMPRESSION_TYPE_GZIP: &'static str = "gzip";