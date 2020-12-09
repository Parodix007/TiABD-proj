use db_conn::db_conn::DB;

fn main() {
	let db = DB::new(String::from("db"));
    println!("{:#?}", db);
}
