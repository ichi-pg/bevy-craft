extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ItemAndAmount)]
pub fn derive_item_and_amount(input: TokenStream) -> TokenStream {
    match syn::parse(input) {
        Ok(ast) => impl_item_and_amount(&ast),
        Err(_) => todo!(),
    }
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
            fn new(item_id: u16, amount: u16) -> Self {
                #name {
                    item_id,
                    amount,
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Pressed)]
pub fn derive_pressed(input: TokenStream) -> TokenStream {
    match syn::parse(input) {
        Ok(ast) => impl_pressed(&ast),
        Err(_) => todo!(),
    }
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

#[proc_macro_derive(Collided)]
pub fn derive_collided(input: TokenStream) -> TokenStream {
    match syn::parse(input) {
        Ok(ast) => impl_collided(&ast),
        Err(_) => todo!(),
    }
}

fn impl_collided(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Collided for #name {
            fn new(repulsion: Vec2) -> Self {
                #name
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(RepulsionCollided)]
pub fn derive_repulsion_collided(input: TokenStream) -> TokenStream {
    match syn::parse(input) {
        Ok(ast) => impl_repulsion_collided(&ast),
        Err(_) => todo!(),
    }
}

fn impl_repulsion_collided(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Collided for #name {
            fn new(repulsion: Vec2) -> Self {
                #name {
                    repulsion,
                }
            }
        }
    };
    gen.into()
}
