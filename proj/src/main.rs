use db_conn::database::DataBase;

fn main() {
	let db = DataBase::new();
    println!("{:#?}", db);
}
