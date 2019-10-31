fn func() {}

fn main() {
	let f = &func;
	let local1 = 5;
	let local2 = 6;

	println!("func: {:p}", f);
	println!("local1: {:p}", &local1);
	println!("local2: {:p}", &local2);
}