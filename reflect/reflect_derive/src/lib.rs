use proc_macro2::Ident;
use syn::{Data, DeriveInput};

#[proc_macro_derive(Reflect)]
pub fn reflect_derive(token: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_reflect(token).into()
}

fn impl_reflect(input: proc_macro::TokenStream) -> proc_macro2::TokenStream {
    let ast: DeriveInput = syn::parse(proc_macro::TokenStream::from(input)).unwrap();
    let ident = ast.ident;

    let filed_name: Vec<Ident> = match ast.data {
        Data::Struct(s) => s.fields.into_iter().filter_map(|f| f.ident).collect(),
        _ => panic!(),
    };

    quote::quote! {
        impl #ident {
            pub fn struct_name(&self) -> &'static str{
                stringify!(#ident)
            }

            pub fn fields_name(&self) -> Vec<&'static str> {
                vec![#(stringify!(#filed_name)),*]
            }
        }
    }
}
