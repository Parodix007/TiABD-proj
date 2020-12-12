use mysql::*;
use mysql::prelude::*;

#[derive(Debug)]
pub struct DataBase {
	conn_url: String,
}

impl DataBase {
	pub fn new() -> Self {
		Self {
			conn_url: String::from("mysql://root:haslo@localhost:3306/employees"), // -> ENTER YOUR EMPLOYEES DATABASE URL
		}
	}

	pub fn make_request_and_get_time(&self, query: &str, n: &i32) -> Vec<f32> {
		let pool_conn = Pool::new(&self.conn_url).expect("Connection error");
		let mut conn_to_db = pool_conn.get_conn().expect("Connection error");
		conn_to_db.query_drop("set profiling = 1").unwrap();
		for _ in 0..*n {	
			conn_to_db.query_drop(&query).expect("Query Error");
		}

		let mut vec_of_time = vec![];

		let _result:Vec<Row> = conn_to_db.query("show profiles").unwrap();
		
		for item in _result {
			let value:String = item.get(1).unwrap();
			let value:f32 = value.parse().unwrap();
			vec_of_time.push(value);
		}
		conn_to_db.query_drop("SET @@profiling_history_size = 0").unwrap();
		conn_to_db.query_drop("set profiling = 0").unwrap();

		vec_of_time
	}

	pub fn calc_time(vec_with_result:Vec<f32>) -> f32{
		let mut sum:f32 = 0.0;
		for item in &vec_with_result{
			sum += item;
		}
		let avg_time:f32 = sum / vec_with_result.len() as f32;
		
		avg_time
	}
}	

