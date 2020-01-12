use rust_electron::hello_world;
use rust_electron::render_image;

fn main() {
    println!("{}", hello_world());
    let img = render_image();
    img.save("image.png").unwrap();
    println!("Wrote test image to ./image.png");
}
