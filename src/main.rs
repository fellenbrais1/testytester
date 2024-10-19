pub mod helpers;
pub mod closures;

fn main() {
    println!("Hello, world!");
    // let first = "Ellim";
    // let last = "Liastal";
    // We can feed in variable arguments as in the line below
    // let full_name = get_full_name(first, last);
    // Or we can feed in values directly as arguments as in this line
    let full_name = helpers::name_helpers::get_full_name("Bobby", "Bobson");
    println!("Full name is {}", full_name);
    let data = helpers::data_base::display_data();
    println!("Data is {}", data);
    // testfunc()
    closures::test_closures();
}

// When casting a floating point value to a u8 value like the below, the
// decimal information is cut away completely and is not used in rounding
#[allow(dead_code)]
fn testfunc() {
    let x: f32 = 255.156;
    let y: u8 = x as u8 - 10;
    println!("The value of y is {}", y)
}