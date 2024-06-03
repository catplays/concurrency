use std::sync::Arc;



fn main() {
   let arc_str = Arc::new(String::from("hello"));


   let one = arc_str.clone();
   let handler  = std::thread::spawn(move || {
      println!("thread:{:?}", one)
   });
   handler.join().unwrap();
   println!("main:{}", arc_str.as_str())

}
