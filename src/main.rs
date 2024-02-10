
include!(concat!(env!("OUT_DIR"), "/lines.rs"));

fn main() {
    get_lines("Heiko", 10).iter().for_each(|l| println!("{}", l));
}

