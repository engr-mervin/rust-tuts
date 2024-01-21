#[allow(dead_code)]
#[allow(unused_variables)]


//DATA TYPES
/*

use std::mem;
fn main() {
    let a:u8 = 123; // u = unsigned, 8 bits, 0-255
    println!("a = {}", a); //let = immutable
    
    let mut b:i8 = 0; // i = signed, 8 bits, -128, 127
    println!("b = {} before", b);

    b = 127;
    println!("b = {} after", b);

    let mut c = 123456789; // i32
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    c = -1;
    println!("c = {}", c);

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    let size_of_size_of_z = mem::size_of_val(&size_of_z);

    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);
    println!("size_of_z takes up {} bytes", size_of_size_of_z);


    let d: char = 'x';
    println!("{} is a char, size = {} bytes",d,mem::size_of_val(&d));

    //f32, f64 IEEE754 always signed;

    let e = 2.5;
    println!("{}, size = {} bytes",e,mem::size_of_val(&e));


    let g: bool = false; 
    println!("{}, size = {} bytes", g, mem::size_of_val(&g));

}



 //OPERATORS
 fn main(){
    let mut a = 2+3*4;
    println!("{}",a);

    a += 1;
    a -= 2;

    a = a % 7;
    println!("{}", a);


    let a_cubed = i32::pow(a, 3);

    println!("{}", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);

    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed = {}, {}^pi = {}",b,b_cubed,b,b_to_pi);

    let c = 1 | 2; // | OR & AND ^ XOR !NOR
                    //01 OR 10

    println!("{}",c);

    let two_to_10 = 1 << 10;
    println!("{}", two_to_10);


    let  x =5;
    let pi_less_4 = std::f64::consts::PI < 4.0;

    let x_is_5 = x == 5;
    
    println!("{}", x_is_5);

    println!("{}",pi_less_4);
 }

 
 fn scope_and_shadowing(){
    let a = 123;
    {
        let b = 456;
        println!("inside {}",b);
        let a = 321;
        println!("inside {}", a);
        
    }
    println!("{}", a);
 }

 fn main(){
    scope_and_shadowing();
 }


const MEANING_OF_LIFE:u8 = 42;

static mut Z:i32 = 123;



 fn main(){
   unsafe{
      println!("{}", MEANING_OF_LIFE);
      println!("{}", Z);
   }
 }

 mod sh;


 fn if_statement(){
   let temp: i32 = 15;
   
   if temp > 30 {
      println!("It's really hot outside.");
      return;
   }
   
   if temp <10 {
      println!("It's really cold outside.");
      return;
   }

   println!("Wonderful weather we're having.");

   let day: &str = if temp > 20 {"Sunny"} else {"Cloudy"};
   println!("Today is a {} day.", day);

   println!("It is {}", if temp> 20 {"hot"} else if temp<10 {"cold"} else {"okay"});
 }

 fn while_and_loop(){
   let mut x: i32 = 1;

   while x < 1000 {
      x *= 2;

      if x == 64 { continue;}
      println!("x is {}",x);
   }

   let mut y: i32 = 1;
   loop {
      y*=2;
      println!("y = {}", y);

      if y > 1000 {break;}
   }  
 }

 fn for_loop(){

   for x in 1..11 {
      if x == 3 {continue};
      if x == 8 {break;};
      println!("x = {}", x);
   }

   for (pos, y) in (30..41).enumerate(){
      println!("index is {}, value is {}",pos,y);
   }
 }

 
fn match_keyword(){

   let country_code = 0;

   let country  = match country_code {
      44 => "UK",
      46 => "Sweden",
      7 => "Russia",
      1..=1000 => "Unknown",
      _ => "Invalid"
   };

   println!("Code is {} country is {}",country_code,country);

   let x = false;

   let s = match x{
      true => "Male",
      false => "Female"
   };
}


 */

//  use rand::Rng;
//  use std::io::stdin;

 fn combination_lock(){

   let code: String = String::from("1234");

   println!("{}", code);
 }



 fn main(){
   //sh::stack_and_heap();
   //if_statement();
   //while_and_loop();
   //for_loop();
   //match_keyword();
   combination_lock();
 }