use db_conn::database::DataBase;
use db_conn::struct_of_examined_problem::FirstProblem;
use db_conn::struct_of_examined_problem::SecoundProblem;

fn main() {
	let _n = 2; // ENTER YOUR NUMBER
	let _db = DataBase::new();


	// First examinded problem
    let _result_of_query_with_where = _db.make_request_and_get_time("select salary, count(salary) as duplicates from salaries where salary = 66074", &_n);
    let _avg_time_of_where = DataBase::calc_time(_result_of_query_with_where);

    let _result_of_query_with_group_by = _db.make_request_and_get_time("select salary, count(salary) as duplicates from salaries group by salary limit 1", &_n);
    let _avg_time_of_group_by = DataBase::calc_time(_result_of_query_with_group_by);

    let _result_of_query_index = _db.make_request_and_get_time("alter table salaries add index (salary)", &1);
    let _time_of_adding_index = DataBase::calc_time(_result_of_query_index);

    let _first_problem = FirstProblem::new(_avg_time_of_where, _avg_time_of_group_by, _time_of_adding_index);


    //Secound examined problem
    let _result_of_query_asterisk = _db.make_request_and_get_time("select * from employees", &_n);
    let _avg_time_of_asterisk = DataBase::calc_time(_result_of_query_asterisk);

    let _result_of_query_without = _db.make_request_and_get_time("select emp_no, birth_date, first_name, last_name, gender, hire_date from employees", &_n);
    let _avg_time_of_without = DataBase::calc_time(_result_of_query_without);

    let _secound_problem = SecoundProblem::new(_avg_time_of_asterisk, _avg_time_of_without);


    //Outcome
    println!("For n = {:?}", _n);
    _first_problem.conclude();
    _secound_problem.conclude();
}
