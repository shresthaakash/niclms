#[macro_use]
extern crate quote;
extern crate syn;
extern crate couch_rs;

extern crate common;

use proc_macro::TokenStream;


#[proc_macro_derive(DocOwnerInfo, attributes(serde))]
pub fn derive_owner_info(input: TokenStream) -> TokenStream {
    impl_derive_owner_info(&syn::parse(input).unwrap())
}

fn impl_derive_owner_info(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl OwnerInfo for #name {
            fn get_owner_id(&self) -> couch_rs::Cow<str> {
                couch_rs::Cow::from(&self.owner_id)
            }

            

            fn set_owner_id(&mut self, owner_id: &str) {
                self.owner_id = owner_id.to_string();
            }

           

           
        }
    };

    gen.into()
}



#[proc_macro_derive(EntityDoc, attributes(serde))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    impl_derive_entity(&syn::parse(input).unwrap())
}

fn impl_derive_entity(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl Entity for #name {
            fn get_entity_type(&self) -> String {
                String::from(&self.entity_type)
            }

          

            fn set_entity_type(&mut self, entity_type: &str) {
                self.entity_type = entity_type.to_string();
            }

           
        }
    };

    gen.into()
}

