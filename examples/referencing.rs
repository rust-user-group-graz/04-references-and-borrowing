fn add(a: &i32, b: &i32) -> i32 {
    return a + b;
}

fn main() {
    println!("5 + 6 = {}\n", add(&5, &6));
}
