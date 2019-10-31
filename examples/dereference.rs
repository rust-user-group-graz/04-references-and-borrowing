fn main() {
    let a = 5;
    let b = 6;

    let mut c = &a;
    c = &b;
    println!("{}", c);
}
