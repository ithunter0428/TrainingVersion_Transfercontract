#![recursion_limit = "128"]
extern crate pbc_external;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate sha2;
extern crate syn;

use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::ToTokens;
use sha2::{Digest, Sha256};
use syn::__private::TokenStream2;
use syn::{FnArg, Ident, Type, TypePath};

#[proc_macro_attribute]
pub fn state(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let state_ast: syn::ItemStruct = syn::parse(input.clone()).unwrap();
    let original_state_item: proc_macro2::TokenStream = input.into();
    let state_identifier = state_ast.ident;
    let _raw_state_name = state_identifier.to_string();

    let result = quote! {
        // TODO [tth]: Can we do this without lazy_static?
        #[macro_use]
        extern crate lazy_static;

        use reflection::Schema;

        // TODO [tth]: Consider if we should derive PartialEq, Eq and Debug by default.
        //  #[repr(C)] is probably not needed as the struct itself it not passed via FFI.
        #[repr(C)]
        #[derive(PartialEq, Eq, Debug, Clone, Reflection)]
        #original_state_item

        #[doc = "Export the schema for this contract as json"]
        #[no_mangle]
        pub extern "C" fn export_contract_schema_json(dst_ptr: *mut u8, dst_len: usize) -> usize {
            // /*let json = ABI.as_json();
            // let result = json.as_bytes();
            // let len = result.len();
            //
            // assert!(len <= dst_len, "Buffer too small");
            //
            // unsafe { std::ptr::copy(result.as_ptr(), dst_ptr, len) };*/
            0
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn init(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let fn_ast: syn::ItemFn = syn::parse(input.clone()).unwrap();
    // TODO [tth] validate that the first argument is contract context.
    // TODO [tth] validate that the return type is "state".

    let (
        ctx_name,
        ctx_expression,
        state_name,
        state_expression,
        rpc_param_names,
        rpc_param_expressions,
    ) = variables_for_inner_call(&fn_ast, true);

    let fn_identifier = fn_ast.sig.ident.clone();
    let export_symbol = format_ident!("init");
    let raw_fn_name = fn_identifier.to_string();
    let docs = format!("For contract initializer: {}", raw_fn_name.clone());

    let mut result = wrap_function_for_export(
        fn_identifier,
        export_symbol.clone(),
        docs,
        ctx_name,
        ctx_expression,
        state_name,
        state_expression,
        rpc_param_names,
        rpc_param_expressions,
    );

    result.extend(TokenStream2::from(input));
    result.into()
}

fn hash_identifier(raw_name: &String) -> u32 {
    let mut digest = Sha256::new();
    Digest::update(&mut digest, raw_name.as_bytes());
    let output = digest.finalize();
    let last_four_bytes = output.chunks(4).next().unwrap();
    let new_function_identifier = u32::from_le_bytes(last_four_bytes.try_into().unwrap());
    new_function_identifier
}

#[proc_macro_attribute]
pub fn action(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let fn_ast: syn::ItemFn = syn::parse(input.clone()).unwrap();
    // TODO [jm] Validate arguments

    let (
        ctx_name,
        ctx_expression,
        state_name,
        state_expression,
        rpc_param_names,
        rpc_param_expressions,
    ) = variables_for_inner_call(&fn_ast, false);

    let fn_identifier = fn_ast.sig.ident.clone();

    let fn_name_hash = hash_identifier(&fn_identifier.to_string());
    let export_symbol = format_ident!("action_{:x}", fn_name_hash);
    let raw_fn_name = fn_identifier.to_string();
    let docs = format!("For contract action: {}", raw_fn_name.clone());

    let mut result = wrap_function_for_export(
        fn_identifier,
        export_symbol.clone(),
        docs,
        ctx_name,
        ctx_expression,
        state_name,
        state_expression,
        rpc_param_names,
        rpc_param_expressions,
    );

    result.extend(TokenStream2::from(input));
    result.into()
}

fn wrap_function_for_export(
    fn_identifier: Ident,
    export_symbol: Ident,
    docs: String,
    _ctx_name: TokenStream2,
    ctx_expression: TokenStream2,
    state_name: Option<TokenStream2>,
    state_expression: Option<TokenStream2>,
    rpc_parameter_names: Vec<TokenStream2>,
    rpc_parameter_expressions: Vec<TokenStream2>,
) -> TokenStream2 {
    if state_name.is_some() && state_expression.is_some() {
        quote! {
            #[doc = #docs]
            #[no_mangle]
            pub extern "C" fn #export_symbol(
                ctx_ptr: *const u8, ctx_len: usize,
                state_ptr: *const u8, state_len: usize,
                rpc_ptr: *const u8, rpc_len: usize
            ) -> i64 {
                use std::io::Cursor;

                let rpc = unsafe { std::slice::from_raw_parts(rpc_ptr, rpc_len as usize).to_owned() };
                let mut rpc_reader = Cursor::new(rpc);

                // The expressions, which are used to evaluate the arguments for the inner function,
                // deserialize from "cursor" meaning they have side effects.
                // Because of this, we need to ensure that they are evaluated in the correct order,
                // thus we will bind them to variables instead of #fn_identifier(#(#expression),*)
                // (since function arguments are not guaranteed to evaluate left to right).
                #(let #rpc_parameter_names = #rpc_parameter_expressions)*


                let ctx = unsafe { std::slice::from_raw_parts(ctx_ptr, ctx_len as usize).to_owned() };
                let mut ctx_reader = Cursor::new(ctx);

                let state_slice = unsafe { std::slice::from_raw_parts(state_ptr, state_len as usize).to_owned() };
                let mut state_reader = Cursor::new(state_slice);

                let context = #ctx_expression;
                let prev_state = #state_expression;

                let state = #fn_identifier(context, prev_state, #(#rpc_parameter_names),*);

                let mut method_result: Vec<u8> = Vec::new();
                state.write_to(&mut method_result).unwrap();

                let len = method_result.len() as i64;
                let ptr = method_result.as_ptr() as i64;
                std::mem::forget(method_result);

                len << 32 | ptr
            }
        }
    } else {
        quote! {
            #[doc = #docs]
            #[no_mangle]
            pub extern "C" fn #export_symbol(
                ctx_ptr: *const u8, ctx_len: usize,
                rpc_ptr: *const u8, rpc_len: usize
            ) -> i64 {
                use std::io::Cursor;

                let rpc = unsafe { std::slice::from_raw_parts(rpc_ptr, rpc_len as usize).to_owned() };
                let mut rpc_reader = Cursor::new(rpc);

                // The expressions, which are used to evaluate the arguments for the inner function,
                // deserialize from "cursor" meaning they have side effects.
                // Because of this, we need to ensure that they are evaluated in the correct order,
                // thus we will bind them to variables instead of #fn_identifier(#(#expression),*)
                // (since function arguments are not guaranteed to evaluate left to right).
                #(let #rpc_parameter_names = #rpc_parameter_expressions)*


                let ctx = unsafe { std::slice::from_raw_parts(ctx_ptr, ctx_len as usize).to_owned() };
                let mut ctx_reader = Cursor::new(ctx);

                let context = #ctx_expression;

                let state = #fn_identifier(context,  #(#rpc_parameter_names),*);

                let mut method_result: Vec<u8> = Vec::new();
                state.write_to(&mut method_result).unwrap();

                let len = method_result.len() as i64;
                let ptr = method_result.as_ptr() as i64;
                std::mem::forget(method_result);

                len << 32 | ptr
            }
        }
    }
}

fn variables_for_inner_call(
    item: &syn::ItemFn,
    is_init: bool,
) -> (
    TokenStream2,
    TokenStream2,
    Option<TokenStream2>,
    Option<TokenStream2>,
    Vec<TokenStream2>,
    Vec<TokenStream2>,
) {
    let mut var_name: Vec<TokenStream2> = Vec::new();
    let mut expression: Vec<TokenStream2> = Vec::new();

    let mut iter = item.sig.inputs.iter();

    let (ctx_name, ctx_expression) =
        extract_name_and_expression(format_ident!("ctx_reader"), iter.next().unwrap());
    let (state_name, state_expression) = if is_init {
        (None, None)
    } else {
        let (a, b) =
            extract_name_and_expression(format_ident!("state_reader"), iter.next().unwrap());
        (Some(a), Some(b))
    };

    for token in iter {
        let reader_ident = format_ident!("rpc_reader");
        let (name, expr) = extract_name_and_expression(reader_ident, token);
        var_name.push(name);
        expression.push(expr);
    }

    (
        ctx_name,
        ctx_expression,
        state_name,
        state_expression,
        var_name,
        expression,
    )
}

fn extract_name_and_expression(reader_ident: Ident, token: &FnArg) -> (TokenStream2, TokenStream2) {
    match token {
        FnArg::Receiver(_) => {
            // TODO [tth]: Note that self receivers with a specified type,
            //  such as self: Box<Self>, are parsed as a FnArg::Typed.
            panic!("Contract functions cannot be part of an `impl`.")
        }
        FnArg::Typed(pat) => {
            let name = pat.pat.to_token_stream().to_string();
            let variable_identifier = format_ident!("tmp_{}", name);
            let var_name = quote! {#variable_identifier};
            let ty = *(pat.ty.clone());
            match ty {
                Type::Path(path) => {
                    let expr = generate_instantiating_expression(reader_ident, path);
                    (var_name, expr)
                }
                Type::Tuple(_) => {
                    panic!("Unsupported tuple type");
                }
                Type::Array(_) => {
                    panic!("Unsupported array type");
                }
                Type::ImplTrait(_) => {
                    panic!("Unsupported impl trait type");
                }
                Type::Reference(_) => {
                    panic!("Unsupported reference type");
                }
                Type::Slice(_) => {
                    panic!("Unsupported slice type");
                }
                _ => {
                    panic!("Unsupported argument type")
                }
            }
        }
    }
}

fn generate_instantiating_expression(reader_ident: Ident, path: TypePath) -> TokenStream2 {
    match path.path.get_ident() {
        Some(ident) => {
            if ident.eq(&Ident::new("u64", Span::call_site())) {
                quote! {#reader_ident.read_u64_be();}
            } else if ident.eq(&Ident::new("i64", Span::call_site())) {
                quote! {#reader_ident.read_i64_be();}
            } else if ident.eq(&Ident::new("i32", Span::call_site())) {
                quote! {#reader_ident.read_i32_be();}
            } else {
                quote! {<#ident>::read_from(&mut #reader_ident);}
            }
        }
        None => {
            let tokens = path.path.segments.into_token_stream();
            quote! {<#tokens>::read_from(&mut #reader_ident);}
        }
    }
}
