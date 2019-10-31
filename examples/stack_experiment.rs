fn func() {
    let a;
    println!("was {}, setting to {}\n", a, 42);
    a = 42;
}

fn main() {
    func();
    func();
}
