extern crate image;

mod preprocess;
mod machine;

fn main() {
    let img = image::open("/home/stephen/Sync/via.ppm").unwrap().into_rgb();

    let program = preprocess::read_program(img);

    let machine = machine::Machine::new(program);
}
