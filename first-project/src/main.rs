#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;
fn main() {
   let a:u8=123;
   println!("a={}",a);
   let mut b:i8=0;
   println!("b={} before",b);
   b=103;
   println!("b={} after",b);


   let c=123456789;
   println!("c={}, takes up {} bytes",c, mem::size_of_val(&c))
}
