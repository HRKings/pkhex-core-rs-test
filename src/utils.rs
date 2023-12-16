use std::array::TryFromSliceError;

macro_rules! impl_sliceutils {
    () => {
        fn write_into(&mut self, data: &[u8], offset: usize) {
            self[offset..(data.len())].copy_from_slice(data);
        }

        fn get_mutable_slice(&mut self, start: usize, end: usize) -> &mut [u8] {
            &mut self[start..=end]
        }

        /// Returns a refence to a slice of the original array
        /// with the byte_quantity as its size, stating on the offset
        ///
        /// # Examples
        /// ```
        /// # use pkhex_rs::utils::SliceUtils;
        /// # let bytes = [0x2A, 0, 0, 0];
        ///
        /// let offset = bytes.get_offset(0x0, 1);
        ///
        /// # assert_eq!(&[42], offset);
        /// ```
        fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8] {
            &self[offset..offset + byte_quantity]
        }

        fn get_mutable_offset(&mut self, offset: usize, byte_quantity: usize) -> &mut [u8] {
            self.get_mutable_slice(offset, offset + byte_quantity - 1)
        }

        fn get_u16_le(&self) -> Result<u16, TryFromSliceError> {
            Ok(u16::from_le_bytes(self[..2].try_into()?))
        }

        fn get_u32_le(&self) -> Result<u32, TryFromSliceError> {
            Ok(u32::from_le_bytes(self[..4].try_into()?))
        }

        /// Returns an unsigned 16bit little-endian integer
        /// from the provided bytes, starting at the offset
        ///
        /// # Examples
        /// ```
        /// # use std::array::TryFromSliceError;
        /// # use pkhex_rs::utils::SliceUtils;
        /// # let bytes = [ 0x2A, 0, 0, 0 ];
        ///
        /// let number = bytes.get_u16_le_offset(0x0)?;
        ///
        /// # assert_eq!(42, number);
        /// # Ok::<(), TryFromSliceError>(())
        /// ```
        fn get_u16_le_offset(&self, offset: usize) -> Result<u16, TryFromSliceError> {
            self.get_offset(offset, std::mem::size_of::<u16>())
                .get_u16_le()
        }

        /// Returns an unsigned 32bit little-endian integer
        /// from the provided bytes, starting at the offset
        ///
        /// # Examples
        /// ```
        /// # use std::array::TryFromSliceError;
        /// # use pkhex_rs::utils::SliceUtils;
        /// # let bytes = [ 0x2A, 0, 0, 0 ];
        ///
        /// let number = bytes.get_u32_le_offset(0x0)?;
        ///
        /// # assert_eq!(42, number);
        /// # Ok::<(), TryFromSliceError>(())
        /// ```
        fn get_u32_le_offset(&self, offset: usize) -> Result<u32, TryFromSliceError> {
            self.get_offset(offset, std::mem::size_of::<u32>())
                .get_u32_le()
        }
    };
}

macro_rules! impl_sliceutils_for {
    ($($type:ty),+) => {
        $(
            impl SliceUtils for $type {
                impl_sliceutils!();
            }
        )+
    };
}

pub trait SliceUtils {
    fn write_into(&mut self, data: &[u8], offset: usize);

    fn get_mutable_slice(&mut self, start: usize, end: usize) -> &mut [u8];
    fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8];
    fn get_mutable_offset(&mut self, offset: usize, byte_quantity: usize) -> &mut [u8];

    fn get_u16_le(&self) -> Result<u16, TryFromSliceError>;
    fn get_u32_le(&self) -> Result<u32, TryFromSliceError>;

    fn get_u16_le_offset(&self, offset: usize) -> Result<u16, TryFromSliceError>;
    fn get_u32_le_offset(&self, offset: usize) -> Result<u32, TryFromSliceError>;
}

impl_sliceutils_for! { [u8], Vec<u8> }

impl<const K: usize> SliceUtils for [u8; K] {
    impl_sliceutils!();
}

#[cfg(test)]
mod tests {
    use super::SliceUtils;

    #[test]
    fn exploration() {
        let mut bytes = vec![0u8, 0u8, 0u8, 0u8];

        let slice = bytes.get_mutable_offset(1, 2);

        let data = u16::to_le_bytes(0x2);
        slice.write_into(&data, 0);

        assert_eq!(bytes, [0u8, 2u8, 0u8, 0u8]);
        assert_eq!(u16::to_ne_bytes(0x2), u16::to_le_bytes(0x2));
        assert_ne!(u16::to_be_bytes(0x2), u16::to_le_bytes(0x2));
    }
}
