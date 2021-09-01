extern crate proc_macro;

use proc_macro::TokenStream;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use quote::quote;
use syn::{parse_macro_input, LitStr};
use walkdir::WalkDir;

#[proc_macro]
pub fn pack_file(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let target = root.join(input.value());

    WalkDir::new(target)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.into_path())
        .filter_map(|p| p.strip_prefix(&root).ok().map(|p| p.to_path_buf()))
        .map(|p| {
            let mut file = File::open(&p).unwrap();

            let dir = match &p.parent() {
                Some(p) => p.to_path_buf().to_str().unwrap().to_owned(),
                None => p.to_str().unwrap().to_owned()
            };
            let p_str = p.to_str().unwrap();

            let mut data = Vec::with_capacity(p.metadata().map(|meta| meta.len()).unwrap_or_default() as usize);
            file.read_to_end(&mut data).unwrap();
            quote! {
                {
                    if !std::path::PathBuf::from(#p_str).exists() {
                        use std::io::Write;
                        std::fs::create_dir_all(#dir).unwrap();
                        let mut file = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(#p_str).unwrap();
                        file.write_all(&[ #(#data),* ]).unwrap();
                        file.sync_data().unwrap();
                    }
                }
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}
