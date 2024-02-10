use std::{env, fs, path::Path};

fn main() {

    let code = r#"
fn get_lines(name: &'static str, line_count: u32) -> Vec<String> {
    let mut lines = Vec::new();
    for i in 0..line_count {
        lines.push(format!("Line {}", i).to_string());
    }
    lines
}
    "#;

    let out_dir = env::var_os("OUT_DIR").unwrap();
    fs::write(Path::new(&out_dir).join("lines.rs"), code).unwrap();
}
