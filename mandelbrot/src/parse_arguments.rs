use std::str::FromStr;

use num::Complex;

pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T,T)>{
    match s.find(separator){
        None => None,
        Some(index ) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])){
                (Ok(x), Ok(y)) => Some((x, y)),
                _ => None
            }
        }
    }
}

pub fn parse_complex(s: &str) -> Option<Complex<f64>>{
    match parse_pair(s, ','){
        Some((x,y)) => Some(Complex {re: x, im: y}),
        None => None
    }
}

// fn complex_square_add_loop(c: Complex<f64>){
//     let mut z = Complex { re: 0.5, im: 0.0};

//     loop{
//         z = z * z + c;
//         println!("Value of Z: {}", z);
//     }
// }


fn escape_time(c:Complex<f64>, limit: usize) -> Option<usize>{
    let mut z = Complex { re: 0.0, im: 0.0};

    for i in 0..limit{
        if z.norm_sqr() > 4.0 {
            // println!("Not a Part");
            return Some(i);
        }
        z = z * z + c;
        // println!("Value of Z: {}", z);
    }
    // println!("Part");
    None
}

pub fn render(pixels: &mut[u8], bounds: (usize, usize), upper_left:Complex<f64>, lower_right: Complex<f64>){

    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1{
        for column in 0..bounds.0{
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

            pixels[row * bounds.0 + column] = 
            match escape_time(point, 255){
                None => 0,
                Some(count) => 255 - count as u8,
            }

        }
    }
}

fn pixel_to_point(bounds:(usize, usize), pixel:(usize, usize), upper_left:Complex<f64>, lower_right: Complex<f64>) -> Complex<f64>{

    let (width, height) = ((lower_right.re - upper_left.re), (upper_left.im - lower_right.im));

    let (ratio_x, ratio_y) = (width / bounds.0 as f64, height / bounds.1 as f64);

    
    Complex {
        re: upper_left.re + pixel.0 as f64 * ratio_x,
        im: upper_left.im - pixel.1 as f64 * ratio_y
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175),
        Complex { re: -1.0, im:  1.0 },
        Complex { re:  1.0, im: -1.0 }),
        Complex { re: -0.5, im: -0.75 });
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",        ','), None);
    assert_eq!(parse_pair::<i32>("10,",     ','), None);
    assert_eq!(parse_pair::<i32>(",10",     ','), None);
    assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}
