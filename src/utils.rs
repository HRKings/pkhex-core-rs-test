use std::array::TryFromSliceError;

pub trait SliceUtils {
    fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8];

    fn get_u16_le(&self) -> Result<u16, TryFromSliceError>;
    fn get_u32_le(&self) -> Result<u32, TryFromSliceError>;

    fn get_u16_le_offset(&self, offset: usize) -> Result<u16, TryFromSliceError>;
    fn get_u32_le_offset(&self, offset: usize) -> Result<u32, TryFromSliceError>;
}

impl SliceUtils for &[u8] {
    fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8] {
        &self[offset..offset + byte_quantity]
    }

    fn get_u16_le(&self) -> Result<u16, TryFromSliceError> {
        Ok(u16::from_le_bytes(self[..2].try_into()?))
    }

    fn get_u32_le(&self) -> Result<u32, TryFromSliceError> {
        Ok(u32::from_le_bytes(self[..4].try_into()?))
    }

    fn get_u16_le_offset(&self, offset: usize) -> Result<u16, TryFromSliceError> {
        self.get_offset(offset, 2).get_u16_le()
    }

    fn get_u32_le_offset(&self, offset: usize) -> Result<u32, TryFromSliceError> {
        self.get_offset(offset, 4).get_u32_le()
    }
}

impl SliceUtils for Vec<u8> {
    fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8] {
        &self[offset..offset + byte_quantity]
    }

    fn get_u16_le(&self) -> Result<u16, TryFromSliceError> {
        Ok(u16::from_le_bytes(self[..2].try_into()?))
    }

    fn get_u32_le(&self) -> Result<u32, TryFromSliceError> {
        Ok(u32::from_le_bytes(self[..4].try_into()?))
    }

    fn get_u16_le_offset(&self, offset: usize) -> Result<u16, TryFromSliceError> {
        self.get_offset(offset, 2).get_u16_le()
    }

    fn get_u32_le_offset(&self, offset: usize) -> Result<u32, TryFromSliceError> {
        self.get_offset(offset, 4).get_u32_le()
    }
}