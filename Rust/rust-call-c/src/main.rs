extern "C" {
    fn twice(input: i32) -> i32;
}

impl Counter {
 fn default() -> Self {
   Counter {
     count: 0
   }
 }

 pub fn decrease(&self, amount: i32) -> Self {
    let x = self.count - amount;
    Self { count: x }
 }

 pub fn increase(&self, amount: i32) -> Self {
    let x: i32 = self.count + amount;
    Self { count: x }
 }
}

fn main() {
    println!("Hello, world!");
    println!("Twice value of 3 according to C: {}", unsafe { twice(3) } );
}
