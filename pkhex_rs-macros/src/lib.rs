mod bytes_macro;
mod data_getset_macro;

use bytes_macro::BytesGetSetCollection;
use data_getset_macro::DataGetSet;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod kw {
    syn::custom_keyword!(get);
    syn::custom_keyword!(be);
    syn::custom_keyword!(le);
}

/// The endianess can be ommited, doing so it will default to little-endian,
/// the valid endianess are `le` and `be`
/// 
/// Multiple fields can be declared at once, separeted by comma `,`
/// 
/// The types must have a `get_<TYPE>_<ENDIANESS>_offset` method in the [`pkhex:utils::SliceUtils`]!
/// 
/// # Examples
/// ```
/// # use pkhex_rs_macros::byte_parser_proc;
/// # use std::array::TryFromSliceError;
/// # pub trait SliceUtils {
/// # fn write_into(&mut self, data: &[u8], offset: usize);
/// # fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8];
/// # fn get_u16_le(&self) -> Result<u16, TryFromSliceError>;
/// # fn get_u32_le(&self) -> Result<u32, TryFromSliceError>;
/// # fn get_u16_le_offset(&self, offset: usize) -> Result<u16, TryFromSliceError>;
/// # fn get_u32_le_offset(&self, offset: usize) -> Result<u32, TryFromSliceError>;
/// # }
/// # 
/// # impl SliceUtils for [u8] {
/// # fn write_into(&mut self, data: &[u8], offset: usize) { self[offset..(data.len())].copy_from_slice(data);}
/// # 
/// # fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8] { &self[offset..offset + byte_quantity] }
/// # fn get_u16_le(&self) -> Result<u16, TryFromSliceError> { Ok(u16::from_le_bytes(self[..2].try_into()?)) }
/// # fn get_u32_le(&self) -> Result<u32, TryFromSliceError> { Ok(u32::from_le_bytes(self[..4].try_into()?)) }
/// # fn get_u16_le_offset(&self, offset: usize) -> Result<u16, TryFromSliceError> { self.get_offset(offset, 2).get_u16_le() }
/// # fn get_u32_le_offset(&self, offset: usize) -> Result<u32, TryFromSliceError> { self.get_offset(offset, 4).get_u32_le() }
/// # }
/// 
/// byte_parser_proc! { field_1: u32@0x0 }
/// byte_parser_proc! { field_2: u16@0x0#le, field_3: u16@0x0 }
/// 
/// # assert_eq!(get_field_1_from_bytes(&u32::to_le_bytes(0x2A)), 42);
/// # assert_eq!(get_field_2_from_bytes(&u32::to_le_bytes(0x2A)), 42);
/// # assert_eq!(get_field_3_from_bytes(&u32::to_le_bytes(0x2A)), 42);
/// ```
#[proc_macro]
pub fn byte_parser_proc(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as BytesGetSetCollection);

    bytes_macro::expand_byte_get_set(input)
}

/// # Examples
/// ```
/// # use pkhex_rs_macros::data_get_set_proc;
/// 
/// data_get_set_proc! { num: u32;
///     get => { u32::from_le_bytes(data.try_into().unwrap()) }
/// }
/// 
/// # assert_eq!(get_num_from_bytes(&u32::to_le_bytes(0x2A)), 42);
/// ```
#[proc_macro]
pub fn data_get_set_proc(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DataGetSet);

    data_getset_macro::expand_data_get_set(input)
}