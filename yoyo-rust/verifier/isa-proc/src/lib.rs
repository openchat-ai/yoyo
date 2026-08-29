//! isa-proc — parses the 38-line ISA table and generates TirOp / dispatch helpers.
//! (PROMPT-v3 Part 4.4)

mod isa_parser;

use proc_macro::TokenStream;
use quote::quote;
use syn::LitStr;

use isa_parser::parse_isa_table;

/// Accepts either:
///   isa!(include_str!("isa_table.txt"));
///   isa! { r#" ... "# }
#[proc_macro]
pub fn isa(input: TokenStream) -> TokenStream {
    let src = match extract_source(input) {
        Ok(s) => s,
        Err(e) => {
            let msg = e;
            return quote! { compile_error!(#msg); }.into();
        }
    };
    match expand_isa(&src) {
        Ok(ts) => ts.into(),
        Err(e) => {
            let msg = e;
            quote! { compile_error!(#msg); }.into()
        }
    }
}

fn extract_source(input: TokenStream) -> Result<String, String> {
    let input2 = proc_macro2::TokenStream::from(input.clone());
    // Try plain string literal first
    if let Ok(lit) = syn::parse::<LitStr>(input.clone()) {
        return Ok(lit.value());
    }
    // Try include_str!("path")
    let mut iter = input2.into_iter();
    let first = iter.next().ok_or("empty isa! input")?;
    let ident = match first {
        proc_macro2::TokenTree::Ident(i) => i,
        _ => return Err("isa! expects a string literal or include_str!(\"path\")".into()),
    };
    if ident != "include_str" {
        return Err(format!("unexpected token '{ident}', want include_str"));
    }
    // skip '!'
    match iter.next() {
        Some(proc_macro2::TokenTree::Punct(p)) if p.as_char() == '!' => {}
        _ => return Err("expected ! after include_str".into()),
    }
    let group = match iter.next() {
        Some(proc_macro2::TokenTree::Group(g)) => g,
        _ => return Err("expected (\"path\") after include_str!".into()),
    };
    let path_lit: LitStr = syn::parse2(group.stream()).map_err(|e| e.to_string())?;
    let path = path_lit.value();

    // Resolve relative to CARGO_MANIFEST_DIR/src/
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let full = std::path::Path::new(&manifest).join("src").join(&path);
    std::fs::read_to_string(&full)
        .or_else(|_| std::fs::read_to_string(std::path::Path::new(&manifest).join(&path)))
        .map_err(|e| format!("cannot read ISA table '{}': {e}", full.display()))
}

fn expand_isa(src: &str) -> Result<proc_macro2::TokenStream, String> {
    let rows = parse_isa_table(src)?;
    if rows.is_empty() {
        return Err("ISA table is empty".into());
    }

    let mut seen = std::collections::HashSet::new();
    for r in &rows {
        if !seen.insert(r.opcode) {
            return Err(format!("Duplicate opcode 0x{:02X}", r.opcode));
        }
        // Read emit pattern so CI `-D warnings` accepts the parsed field;
        // also reject empty recipes (parser already checks, belt-and-suspenders).
        if r.pattern.is_empty() {
            return Err(format!(
                "opcode 0x{:02X} ({}) has empty emit pattern",
                r.opcode, r.mnemonic
            ));
        }
    }

    let variants: Vec<_> = rows
        .iter()
        .map(|r| {
            let name = syn::Ident::new(&r.variant_name(), proc_macro2::Span::call_site());
            quote! { #name }
        })
        .collect();

    let name_arms: Vec<_> = rows
        .iter()
        .map(|r| {
            let name = syn::Ident::new(&r.variant_name(), proc_macro2::Span::call_site());
            let mnem = &r.mnemonic;
            quote! { TirOp::#name => #mnem }
        })
        .collect();

    let opcode_arms: Vec<_> = rows
        .iter()
        .map(|r| {
            let name = syn::Ident::new(&r.variant_name(), proc_macro2::Span::call_site());
            let op = r.opcode;
            quote! { #op => Some(TirOp::#name) }
        })
        .collect();

    let op_to_u8: Vec<_> = rows
        .iter()
        .map(|r| {
            let name = syn::Ident::new(&r.variant_name(), proc_macro2::Span::call_site());
            let op = r.opcode;
            quote! { TirOp::#name => #op }
        })
        .collect();

    let arity_arms: Vec<_> = rows
        .iter()
        .map(|r| {
            let name = syn::Ident::new(&r.variant_name(), proc_macro2::Span::call_site());
            let n = r.params.len() as usize;
            quote! { TirOp::#name => #n }
        })
        .collect();

    Ok(quote! {
        /// Generated from the ISA table (isaproc).
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum TirOp {
            #(#variants,)*
        }

        pub fn instr_name(op: TirOp) -> &'static str {
            match op {
                #(#name_arms,)*
            }
        }

        pub fn opcode_from_u8(b: u8) -> Option<TirOp> {
            match b {
                #(#opcode_arms,)*
                _ => None,
            }
        }

        pub fn opcode_to_u8(op: TirOp) -> u8 {
            match op {
                #(#op_to_u8,)*
            }
        }

        pub fn instr_arity(op: TirOp) -> usize {
            match op {
                #(#arity_arms,)*
            }
        }

        /// KY 0x71..=0x7A → x64 JCC second byte (Part 4.4.4).
        pub const JCC_TABLE: [u8; 10] = [
            0x84, 0x85, 0x8C, 0x8D, 0x8E, 0x8F, 0x82, 0x83, 0x86, 0x87,
        ];

        pub const JCC_MNEMONIC: [&'static str; 10] = [
            "je", "jne", "jl", "jge", "jle", "jg", "jb", "jae", "jbe", "ja",
        ];
    })
}
