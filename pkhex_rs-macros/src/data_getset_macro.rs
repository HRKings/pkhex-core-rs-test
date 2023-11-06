use proc_macro::TokenStream;
use syn::{Ident, Type, parse::Parse, Token, Expr};
use quote::{quote, format_ident};

use crate::kw;

pub(crate) struct DataGetSet {
    pub var_name: Ident,
    pub var_type: Type,
    pub get_block: Expr 
}

impl Parse for DataGetSet {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let var_name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let var_type: Type = input.parse()?;
        input.parse::<Token![;]>()?;
        input.parse::<kw::get>()?;
        input.parse::<Token![=>]>()?;
        let get_block: Expr  = input.parse()?;

        Ok(DataGetSet {
            var_name,
            var_type,
            get_block,
        })
    }
}

pub(crate) fn expand_data_get_set(macro_input: DataGetSet) -> TokenStream{
    let DataGetSet {
        var_name,
        var_type,
        get_block,
    } = macro_input;

    if let Expr::Block(ref _get_block) = get_block {
    } else {
        panic!("The expression is not a block");
    }

    let fn_name = format_ident!("get_{}_from_bytes", var_name);
    let var_type_string = quote!( #var_type ).to_string();
    let get_block_string = quote!( #get_block ).to_string();
    
    let doc_part = if let Type::Array(_) = var_type {
        format!(r"# let data = <[u8; 32]>::default();
let {var_name} = {fn_name}(&data);")
    } else {
        format!(r"# let data = {var_type_string}::to_le_bytes(0x2A);
let number = {fn_name}(&data);
# assert_eq!(number, 42);")
    };

    let docs = format!(r"# Examples
```
# use pkhex_rs_macros::data_get_set_proc;
# use std::array::TryFromSliceError;
# pub trait SliceUtils {{
# fn write_into(&mut self, data: &[u8], offset: usize);
# fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8];
# fn get_u16_le(&self) -> Result<u16, TryFromSliceError>;
# fn get_u32_le(&self) -> Result<u32, TryFromSliceError>;
# fn get_u16_le_offset(&self, offset: usize) -> Result<u16, TryFromSliceError>;
# fn get_u32_le_offset(&self, offset: usize) -> Result<u32, TryFromSliceError>;
# }}
# impl SliceUtils for [u8] {{
# fn write_into(&mut self, data: &[u8], offset: usize) {{ self[offset..(data.len())].copy_from_slice(data);}}
# fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8] {{ &self[offset..offset + byte_quantity] }}
# fn get_u16_le(&self) -> Result<u16, TryFromSliceError> {{ Ok(u16::from_le_bytes(self[..2].try_into()?)) }}
# fn get_u32_le(&self) -> Result<u32, TryFromSliceError> {{ Ok(u32::from_le_bytes(self[..4].try_into()?)) }}
# fn get_u16_le_offset(&self, offset: usize) -> Result<u16, TryFromSliceError> {{ self.get_offset(offset, 2).get_u16_le() }}
# fn get_u32_le_offset(&self, offset: usize) -> Result<u32, TryFromSliceError> {{ self.get_offset(offset, 4).get_u32_le() }}
# }}
# data_get_set_proc! {{ {var_name}: {var_type_string};
#   get => {get_block_string}
# }}
{doc_part}
```");

    quote! {
        #[doc = #docs]
        pub fn #fn_name (data: &[u8]) -> #var_type #get_block
    }.into()
}