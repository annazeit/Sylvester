fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let a: i32 = 5;
    let a = a + 1;
    let b = 3;
    let c = add(a, b);
    
    println!("Hello, world! {c}");

    for i in 0..10 {
        println!("{i}")
    }
 }
