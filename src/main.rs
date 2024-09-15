fn main() {
    println!("Hello, world!");

    // mutable keyword
    let mut counter = 0;
    
    // for loop
    for i in 1..=5{
        counter += i;
    }

    println!("Counter {}", counter)

}
