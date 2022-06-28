use std::array::TryFromSliceError;

pub trait SliceUtils {
    fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8];
}

impl SliceUtils for &[u8] {
    fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8] {
        &self[offset..offset + byte_quantity]
    }
}

impl SliceUtils for Vec<u8> {
    fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8] {
        &self[offset..offset + byte_quantity]
    }
}

pub fn little_endian_u8_to_u16(byte_array: &[u8]) -> Result<u16, TryFromSliceError> {
    Ok(u16::from_le_bytes(<[u8; 2]>::try_from(&byte_array[..2])?))
}

pub fn little_endian_u8_to_u32(byte_array: &[u8]) -> Result<u32, TryFromSliceError> {
    Ok(u32::from_le_bytes(<[u8; 4]>::try_from(&byte_array[..4])?))
}