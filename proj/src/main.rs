use db_conn::database::DataBase;

fn main() {
	let _db = DataBase::new();
    let _result_of_query = _db.make_request_and_get_time("select salary, count(salary) as duplicates from salaries where salary = 66074", 2);
    println!("{:?}", _result_of_query);
    let _avg_time = DataBase::calc_avg(_result_of_query);
    println!("{:?}", _avg_time);
    let _result_of_query1 = _db.make_request_and_get_time("select salary, count(salary) as duplicates from salaries where salary = 71046", 2);
    println!("{:?}", _result_of_query1);
    let _avg_time1 = DataBase::calc_avg(_result_of_query1);
    println!("{:?}", _avg_time1);
}
