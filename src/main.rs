use std::fs;

fn main() {
    let src = fs::read_to_string("./kasm/42.wat").unwrap();

    println!("{:?}", src);
}
