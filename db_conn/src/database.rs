use mysql::*;
use mysql::prelude::*;

#[derive(Debug)]
pub struct DataBase {
	conn_url: String,
}

impl DataBase {
	pub fn new() -> Self {
		Self {
			conn_url: String::from("mysql://root:password@localhost:3307/sakila"),
		}
	}
	pub fn get_actors(&self, query: String){
		let pool_conn = Pool::new(&self.conn_url).expect("Blad polaczenia");
		let conn_to_db = pool_conn.get_conn().expect("Blad polaczenia");
		let query = conn_to_db.query_map("SELECT * FROM actor").expect("Blad kwerendy");
		println!("{:?}", query);
	}
}	

