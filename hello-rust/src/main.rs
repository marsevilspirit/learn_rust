fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界, 你好";
    let english = "world, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
