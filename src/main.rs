fn main() {
	log4rs::init_file("log/config.yaml", Default::default()).unwrap();

	println!("Hello, world!");
}
