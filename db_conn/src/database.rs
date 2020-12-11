use mysql::*;
use mysql::prelude::*;

#[derive(Debug)]
pub struct DataBase {
	conn_url: String,
}

impl DataBase {
	pub fn new() -> Self {
		Self {
			conn_url: String::from("mysql://admin:1234qwer!@localhost:3306/employees"),
		}
	}
	pub fn get_actors(&self, query: &str){
		#[derive(Debug)]
		struct Actor {
			first_name: String,
			last_name: String,
		}
		let pool_conn = Pool::new(&self.conn_url).expect("Blad polaczenia");
		let mut conn_to_db = pool_conn.get_conn().expect("Blad polaczenia");
		let query = &conn_to_db.query_map(&query, |(first_name, last_name)| { 
			Actor {first_name, last_name} 
		}).expect("Blad kwerendy");
		println!("{:#?}", query);
	}
}	

