pub mod db_conn {
	#[derive(Debug)]
	pub struct DB {
		pub name: String,
	}
	
	impl DB {
	pub fn new(name: String) -> Self {
		Self {
			name
		}
	}
}
}