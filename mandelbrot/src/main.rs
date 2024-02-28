// use num::Complex;
use std::env;
mod parse_arguments;
mod image;



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5{
        eprintln!("Usage: {} file pixels upper_left lower_right", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_arguments::parse_pair(&args[2], 'x').expect("Error parsing image dimensions");

    let upper_left = parse_arguments::parse_complex(&args[3]).expect("Error parsing upper left point");

    let lower_right = parse_arguments::parse_complex(&args[4]).expect("Error parsing lower right point");
    
    let mut pixels = vec![0; bounds.0 * bounds.1];

    parse_arguments::render(&mut pixels, bounds, upper_left, lower_right);

    image::write_image(&args[1], &pixels, bounds).expect("Error writing PNG file");


}

