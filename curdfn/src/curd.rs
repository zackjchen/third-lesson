#![allow(warnings)]


use core::panic;
use std::{iter::Map};
use proc_macro2::{Ident, TokenStream, Literal};
use quote::quote;
use syn::{ Data, DeriveInput,punctuated::{Punctuated, Iter}, Field, token::Comma, DataStruct, Fields, FieldsNamed, Type, TypePath, PathSegment, Path, PathArguments, AngleBracketedGenericArguments, GenericArgument, spanned::Spanned};

// #[derive(Debug,Default)]
// pub struct Table{
//     students: Vec<String>,
//     courses: Vec<String>,
//     teachers: Vec<String>,
//     scores: Vec<String>,
// }

#[derive(Debug)]
pub struct Context{
    name: Ident,
    fields: Punctuated<Field,Comma>,
}

impl Context {
    pub fn new(input: DeriveInput) -> Self{
        let name = input.ident;
        let fields = if let Data::Struct(DataStruct{fields: Fields::Named(FieldsNamed{named,..}),..}) = input.data{
            named
        }else{
            panic!("出错啦，只支持有名的结构体")
        };
        
        Context{name, fields}
    }


    pub fn generate(&self) ->TokenStream{
        let struct_name = &self.name;
        println!("get_method 之前");
        let methods = self.get_method();
        println!("get_method 之后");

        quote!(
            impl #struct_name {
                #(#methods)*
            }
        )
    }

    fn get_method<'a>(&'a self) -> Map<Iter<'a, Field>,fn(&'a Field) -> TokenStream >{
        self.fields.iter().map(|f|{
            let field_name = &f.ident.as_ref().unwrap();
            let add_fn_name = Ident::new(&format!("add_{}",field_name),f.ident.span());
            let del_fn_name = Ident::new(&format!("del_{}",field_name),f.ident.span());
            let select_fn_name = Ident::new(&format!("select_{}_by_name",field_name),f.ident.span());

            let (_,ty) = get_inner_ty(&f.ty, "Vec");
            let (is_score,_) = get_inner_ty(&ty, "Score");

            // println!("{:?}",is_score);
            let (id,name) = if is_score {
                (Ident::new("student_id",f.ident.span()),Ident::new("student_name",f.ident.span()))
            }else {
                (Ident::new("id",f.ident.span()),Ident::new("name",f.ident.span()))
            };
            
            

            let column : Ident;
            let val: Literal;

            quote!(
                pub fn #add_fn_name(&mut self,value: #ty){
                    self.#field_name.push(value)
                }
        
                pub fn #del_fn_name(&mut self,id: u32){
                    let mut index:i32 = -1;
                    for (i,st) in self.#field_name.iter().enumerate() {
                        if st.#id == id {
                            index = i as i32;
                        }
                    }
                    if index > 0 {
                        self.#field_name.remove(index as usize);            
                    }else {
                        panic!("connot find the student")
                    }
                }
        
                pub fn #select_fn_name<'a>(&'a self ,name: &str) -> Result<&'a #ty,&str> {
                    for (i,row) in self.#field_name.iter().enumerate(){
                        if row.#name == name{
                            return Ok(&self.#field_name[i])
                        }
                    }
                    return Err("没找到")

                }
            )
        })
    }

    

}
fn f(f:&Field) -> TokenStream{
    let field_name = &f.ident.as_ref().unwrap();
    let add_fn_name = Ident::new(&format!("add_{}",field_name),f.ident.span());
    let del_fn_name = Ident::new(&format!("del_{}",field_name),f.ident.span());


    let (_,ty) = get_inner_ty(&f.ty, "Vec");
    let (is_score,_) = get_inner_ty(&ty, "Score");
    println!("{:?}",is_score);
    let id = if is_score {
        Ident::new("student_id",f.ident.span())
    }else {
        Ident::new("id",f.ident.span())
    };

    quote!(
        pub fn #add_fn_name(&mut self,value: #ty){
            self.#field_name.push(value)
        }

        pub fn #del_fn_name(&mut self,id: u32){
            let mut index:i32 = -1;
            for (i,st) in self.#field_name.iter().enumerate() {
                if st.#id == id {
                    index = i as i32;
                }
            }
            if index > 0 {
                self.#field_name.remove(index as usize);            
            }else {
                panic!("connot find the student")
            }
        }

    )
}



fn get_inner_ty<'a>(ty: &'a Type, name: &str) -> (bool,&'a Type){
    // use syn::Path::AngleBracketedGenericArguments;
    let mut flag = false;
    if let Type::Path(TypePath { path: Path { segments ,..},..}) =  ty{
        if let Some(segments) = segments.iter().next() {
            if segments.ident == name {
                flag = true;
            }
            // println!("{:?}",segments.ident);

            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments{args,..}) = &segments.arguments{
                if let GenericArgument::Type(ty1) = args.iter().next().unwrap(){
                    return (flag, ty1);
                }
            }else {
                return (flag,ty);
            }
        }
    };
    panic!("error in get_vec_inner_ty")
}