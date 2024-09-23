#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let person = Rectangle {
        width: 10,
        height: 20,
    };

    println!("rectl is {:?}", person);
}
