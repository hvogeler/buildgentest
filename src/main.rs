
include!(concat!(env!("OUT_DIR"), "/lines.rs"));

fn main() {
    get_lines("Heiko", 1).iter().for_each(|l| println!("{}", l));
}

