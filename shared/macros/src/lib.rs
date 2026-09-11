use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{
        parse::{Parse, ParseStream},
        *,
    },
};

enum CaveBaseArgs {
    Raw(u64),
    ArchSpecific {
        x86: u64,
        x64: u64,
    },
}

impl Parse for CaveBaseArgs {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        if input.peek(syn::LitInt) {
            let value = parse_hex(input)?;

            if !input.is_empty() {
                return Err(input.error("expected a single hexadecimal number"));
            }

            return Ok(CaveBaseArgs::Raw(value));
        }

        let first_name: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let first_value = parse_hex(input)?;

        input.parse::<Token![,]>()?;

        let second_name: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let second_value = parse_hex(input)?;

        if !input.is_empty() {
            return Err(input.error("unexpected tokens"));
        }

        match (first_name.to_string().as_str(), second_name.to_string().as_str()) {
            ("x86", "x64") => {
                Ok(CaveBaseArgs::ArchSpecific {
                    x86: first_value,
                    x64: second_value,
                })
            }

            ("x64", "x86") => {
                Ok(CaveBaseArgs::ArchSpecific {
                    x86: second_value,
                    x64: first_value,
                })
            }

            _ => Err(syn::Error::new(first_name.span(), "expected `x86` and `x64`")),
        }
    }
}

fn parse_hex(input: ParseStream) -> Result<u64> {
    let lit: syn::LitInt = input.parse()?;
    let text = lit.to_string();

    if !text.starts_with("0x") {
        return Err(syn::Error::new(lit.span(), "expected a hexadecimal number"));
    }

    u64::from_str_radix(&text[2..], 16)
        .map_err(|_| syn::Error::new(lit.span(), "invalid hexadecimal number"))
}

#[proc_macro_attribute]
pub fn code_cave(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as CaveBaseArgs);
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let base_offset = match args {
        CaveBaseArgs::Raw(off) => {
            quote! { #off }
        }
        CaveBaseArgs::ArchSpecific {
            x86,
            x64,
        } => {
            quote! {
                match gubtool_core::attached::is_32() {
                    true => #x86,
                    false => #x64,
                }
            }
        }
    };

    let derive = quote! {
        #[repr(u64)]
        #[derive(Clone, Copy, Debug, strum::EnumIter)]
        #input
    };

    let address = quote! {
        impl gubtool_core::address::Address for #struct_name {
            fn u64_address(&self) -> u64 {
                gubtool_core::attached::module_base()
                    .saturating_add(#base_offset)
                    .saturating_add(*self as u64)
            }

            fn game(&self) -> Option<gubtool_core::game_version::Game> {
                Some(crate::mem::crate_game())
            }
        }
    };

    let u64 = quote! {
        impl From<#struct_name> for u64 {
            fn from(value: #struct_name) -> u64 {
                <#struct_name as gubtool_core::address::Address>::u64_address(&value)
            }
        }
    };

    let patch = quote! {
        impl assemble::patch::Patch<gubtool_core::address::POINTER> for #struct_name {
            fn patch(self, fun: &mut assemble::asm_folder::AsmFunction, relocation: &'static str) {
                let addr = <Self as gubtool_core::address::Address>::u64_address(&self);
                let bytes: &[u8] = if gubtool_core::attached::is_32() {
                    &(addr as u32).to_le_bytes()
                } else {
                    &addr.to_le_bytes()
                };
                assemble::patch::patch_asm_fun(fun, relocation, bytes);
            }
        }
    };

    let pointers = quote! {
        pub fn pointers() -> Vec<(String, u64)> {
            <#struct_name as strum::IntoEnumIterator>::iter()
                .map(|addr| (format!("{:?}", addr), <#struct_name as gubtool_core::address::Address>::u64_address(&addr)))
                .collect()
        }
    };

    let output = quote! {
        #derive
        #address
        #u64
        #patch
        #pointers
    };

    output.into()
}

fn pascal_to_snake(s: &str) -> String {
    let mut result = String::new();

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                result.push('_');
            }
            result.extend(c.to_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}

#[proc_macro_attribute]
pub fn base_address(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let variants = match &input.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("expected an enum"),
    };

    let generated = variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let name =
            syn::Ident::new(&pascal_to_snake(&variant_name.to_string()), variant_name.span());

        quote! {
            Self::#variant_name => module_offsets.#name
        }
    });

    let derive = quote! {
        #[repr(u64)]
        #[derive(Clone, Copy, Debug)]
        #input
    };

    let address = quote! {
        impl gubtool_core::address::Address for #struct_name {
            fn u64_address(&self) -> u64 {
                let module_offsets = crate::offsets::module_offsets::r#struct::module_offsets();
                let offset = match self {
                    #(#generated),*
                };
                gubtool_core::attached::module_base() + offset
            }

            fn game(&self) -> Option<gubtool_core::game_version::Game> {
                Some(crate::mem::crate_game())
            }
        }
    };

    let u64 = quote! {
        impl From<#struct_name> for u64 {
            fn from(value: #struct_name) -> u64 {
                <#struct_name as gubtool_core::address::Address>::u64_address(&value)
            }
        }
    };

    let patch = quote! {
        impl assemble::patch::Patch<gubtool_core::address::POINTER> for #struct_name {
            fn patch(self, fun: &mut assemble::asm_folder::AsmFunction, relocation: &'static str) {
                let addr = <Self as gubtool_core::address::Address>::u64_address(&self);
                let bytes: &[u8] = if gubtool_core::attached::is_32() {
                    &(addr as u32).to_le_bytes()
                } else {
                    &addr.to_le_bytes()
                };
                assemble::patch::patch_asm_fun(fun, relocation, bytes);
            }
        }
    };

    let output = quote! {
        #derive
        #address
        #u64
        #patch
    };

    output.into()
}

#[proc_macro_derive(GameObject)]
pub fn derive_game_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let pointers_field = match &input.data {
        syn::Data::Struct(data) => {
            data.fields.iter().find(|field| {
                field
                    .ident
                    .as_ref()
                    .map(|ident| ident == "pointers")
                    .unwrap_or(false)
            })
        }
        _ => panic!("GameObject can only be derived for structs"),
    };

    let pointers_type = &pointers_field
        .expect("GameObject requires a `pointers` field")
        .ty;

    let pointer_key = match pointers_type {
        syn::Type::Path(type_path) => {
            let segment = type_path
                .path
                .segments
                .last()
                .expect("invalid PointerCache type");

            match &segment.arguments {
                syn::PathArguments::AngleBracketed(args) => {
                    args.args
                        .first()
                        .expect("PointerCache requires a type parameter")
                }
                _ => panic!("PointerCache requires a type parameter"),
            }
        }
        _ => panic!("`pointers` must be a PointerCache<T>"),
    };

    quote! {
        impl shared::obj_ref::GameObject for #name {
            type PointerKey = #pointer_key;
            fn pointer_cache(&self) -> &shared::pointer_cache::PointerCache<Self::PointerKey> {
                &self.pointers
            }
            fn pointer_cache_mut(&mut self) -> &mut shared::pointer_cache::PointerCache<Self::PointerKey> {
                &mut self.pointers
            }
        }
    }
    .into()
}
