use proc_macro::{TokenStream, Span};
use syn::{Ident, Type, parse::Parse, Token, Expr, punctuated::Punctuated, ext::IdentExt};
use quote::{quote, format_ident};

use crate::kw;

pub(crate)struct BytesGetSet {
    pub var_name: Ident,
    pub var_type: Type,
    pub offset: Expr,
    pub endianess: Ident
}

impl Parse for BytesGetSet {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let var_name: Ident = input.parse().map_err(|_| input.error("Expected field name"))?;
        input.parse::<Token![:]>().map_err(|_| input.error("Expected `:`"))?;

        let var_type: Type = input.parse().map_err(|_| input.error("Expected field type"))?;
        input.parse::<Token![@]>().map_err(|_| input.error("Expected `@`"))?;
        let offset: Expr  = input.parse()?;

        if !input.peek(Token![#]) {
            return Ok(BytesGetSet {
                var_name,
                var_type,
                offset,
                endianess: Ident::new("le", Span::call_site().into())
            })
        }

        input.parse::<Token![#]>()?;
        
        let endian_lookahead = input.lookahead1();
        let endianess = if endian_lookahead.peek(kw::be) || endian_lookahead.peek(kw::le) {
            input.call(Ident::parse_any)?
        } else {
            Err(endian_lookahead.error())?
        };

        Ok(BytesGetSet {
            var_name,
            var_type,
            offset,
            endianess
        })
    }
}

pub(crate) struct BytesGetSetCollection {
    pub fields: Punctuated<BytesGetSet, Token![,]>
}

impl Parse for BytesGetSetCollection {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fields = input.parse_terminated(BytesGetSet::parse, Token![,])?;
        Ok(
            BytesGetSetCollection { 
                fields
            }
        )
    }
}

pub(crate) fn expand_byte_get_set(macro_input: BytesGetSetCollection) -> TokenStream {
    let mapped_fields = macro_input.fields.iter().map(|f| {
        let getter_name = format_ident!("get_{}_from_bytes", &f.var_name);
        let setter_name = format_ident!("set_{}_from_bytes", &f.var_name);

        let type_string = match &f.var_type {
            Type::Path(verbatim) => verbatim.path.get_ident(),
            _ => panic!("No valid type was found")
        };

        if type_string.is_none() || (type_string.unwrap() != "u16" && type_string.unwrap() != "u32") {
            panic!("The type must be u16 or u32");
        }

        let get_fn = format_ident!("get_{}_{}_offset", type_string.unwrap(), &f.endianess);
        let set_fn = format_ident!("to_{}_bytes", &f.endianess);
        
        let field_type = &f.var_type;
        let offset = &f.offset;

        quote! {
            pub fn #getter_name (data: &[u8]) -> #field_type {
                SliceUtils:: #get_fn (data, #offset).unwrap()
            }

            pub fn #setter_name (data: &mut [u8], value: #field_type ) {
                let value_bytes = #field_type :: #set_fn (value);
                data.write_into(&value_bytes, #offset);
            }
        }
    });

    quote! {
        #(#mapped_fields)*
    }.into()
}
