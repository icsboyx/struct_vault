use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, ItemStruct, parse_macro_input};

fn struct_vault_crate_path() -> proc_macro2::TokenStream {
    match proc_macro_crate::crate_name("struct_vault") {
        Ok(proc_macro_crate::FoundCrate::Itself) => quote!(crate),
        Ok(proc_macro_crate::FoundCrate::Name(name)) => {
            let ident = format_ident!("{}", name);
            quote!(::#ident)
        }
        Err(_) => quote!(::struct_vault),
    }
}

fn expand_persistent(mut input: ItemStruct) -> TokenStream {
    // Preserve original attributes (derive, repr, cfg, serde, etc.).
    let attrs = input.attrs.clone();

    // Only support named fields. Struct with unnamed fields do not hold data.
    let named_fields = match &mut input.fields {
        Fields::Named(n) => &mut n.named,
        _ => {
            return syn::Error::new_spanned(&input, "persistent supports only structs with named fields")
                .to_compile_error()
                .into();
        }
    };

    // Field name injected into the struct.
    let injected_field_ident = format_ident!("__vault_config");

    // Prevent double-injection.
    let already_present = named_fields
        .iter()
        .any(|f| f.ident.as_ref().map(|id| id == &injected_field_ident).unwrap_or(false));
    if already_present {
        return syn::Error::new_spanned(&input, "field __vault_config already exists (macro applied twice?)")
            .to_compile_error()
            .into();
    }

    // Resolve the runtime crate path and build the injected type.
    let vault_path = struct_vault_crate_path();
    let struct_save_config_ty = quote!(::core::option::Option<#vault_path::StructSaveConfig>);

    // Inject the field (serde skip so it won't be serialized).
    named_fields.push(syn::parse_quote! {
        #[doc(hidden)]
        #[serde(skip)]
        #injected_field_ident: #struct_save_config_ty
    });

    // Now re-emit the struct plus add methods.
    let vis = &input.vis;
    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = &input.generics.where_clause;
    let fields = &input.fields;

    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    TokenStream::from(quote! {
        #(#attrs)*
        #vis struct #ident #generics #fields #where_clause

    impl #impl_generics #vault_path::PersistentStructConfig for #ident #ty_generics #where_clause {
         #[inline]
        fn vault_config(&self) -> ::core::option::Option<&#vault_path::StructSaveConfig> {
            self.#injected_field_ident.as_ref()
        }
         #[inline]
        fn vault_get_config(&mut self) -> &mut ::core::option::Option<#vault_path::StructSaveConfig> {
           &mut self.#injected_field_ident
        }
    }
    })
}

#[proc_macro_attribute]
pub fn vault_config(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    expand_persistent(input)
}
