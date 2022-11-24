fn main() {
    println!("Hello, world!");
    println!("hi {:.1}", add(14.3333, 14.222));
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}
