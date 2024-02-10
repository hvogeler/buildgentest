use quote::quote;
use std::{env, fs, path::Path};

fn main() {
    dotenv::dotenv().unwrap();
    let msg = env::var("MSG").unwrap_or("Line".to_owned());
    let j = [1, 2, 3];
    let k = vec!["a", "b", "c"];

    let code = quote!(
        fn get_lines(name: &'static str, line_count: u32) -> Vec<String> {
            let mut lines = Vec::new();
              let v = [#(#j),*];
              #(for i in v {
                lines.push(format!("* {} {} {} {}", #msg, #j, #k, i).to_string());
            })*
            lines
        }

    );

    let tmp_ast = syn::parse2(code).unwrap();
    let pretty_code = prettyplease::unparse(&tmp_ast);
    let out_dir = env::var_os("OUT_DIR").unwrap();
    fs::write(Path::new(&out_dir).join("lines.rs"), pretty_code).unwrap();
}

// The above produces output file:
// fn get_lines(name: &'static str, line_count: u32) -> Vec<String> {
//     let mut lines = Vec::new();
//     let v = [1i32, 2i32, 3i32];
//     for i in v {
//         lines.push(format!("* {} {} {} {}", "Hallo", 1i32, "a", i).to_string());
//     }
//     for i in v {
//         lines.push(format!("* {} {} {} {}", "Hallo", 2i32, "b", i).to_string());
//     }
//     for i in v {
//         lines.push(format!("* {} {} {} {}", "Hallo", 3i32, "c", i).to_string());
//     }
//     lines
// }

