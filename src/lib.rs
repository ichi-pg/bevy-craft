extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ItemAndAmount)]
pub fn derive_item_and_amount(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_item_and_amount(&ast)
}

fn impl_item_and_amount(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ItemAndAmount for #name {
            fn item_id(&self) -> u16 {
                self.item_id
            }
            fn amount(&self) -> u16 {
                self.amount
            }
            fn set_item_id(&mut self, item_id: u16) {
                self.item_id = item_id;
            }
            fn set_amount(&mut self, amount: u16) {
                self.amount = amount;
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Pressed)]
pub fn derive_pressed(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_pressed(&ast)
}

fn impl_pressed(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Pressed for #name {
            fn pressed(&self) -> bool {
                self.pressed
            }
            fn just_pressed(&self) -> bool {
                self.just_pressed
            }
            fn set_pressed(&mut self, pressed: bool) {
                self.pressed = pressed;
            }
            fn set_just_pressed(&mut self, just_pressed: bool) {
                self.just_pressed = just_pressed;
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Stats)]
pub fn derive_stats(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_stats(&ast)
}

fn impl_stats(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Stats for #name {
            fn get(&self) -> u16 {
                self.0
            }
            fn set(&mut self, stats: u16) {
                self.0 = stats;
            }
        }
    };
    gen.into()
}
