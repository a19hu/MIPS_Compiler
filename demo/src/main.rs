fn main() {
    println!("Hello, world!");
   let c= add(5,6);
   println!("{c}");
   let mut s= String::from("hello");
   s.push_str(", world");
   println!("{s}");
}
fn add(a: i32,b: i32) ->i32{
    return a+b;
}