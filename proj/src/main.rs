use db_conn::database::DataBase;

fn main() {
	let db = DataBase::new();
    db.get_actors("SELECT first_name, last_name FROM actor")
}
