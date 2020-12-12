#[derive(Debug)]
pub struct FirstProblem {
	avg_with_where: f32,
	avg_with_group_by: f32,
	time_of_creating_index:f32,
}
impl FirstProblem {
	pub fn new(avg_with_where: f32, avg_with_group_by: f32, time_of_creating_index: f32) -> Self {
		Self {
			avg_with_where,
			avg_with_group_by,
			time_of_creating_index,
		}
	}
	pub fn conclude(&self){
		if &self.avg_with_where < &self.avg_with_group_by {
			println!("Avg time of query using WHERE: {:?}\nAvg time of query using GROUP BY with limit of 1 row: {:?}\nSearching with WHERE is more eficcient", &self.avg_with_where, &self.avg_with_group_by);
		}else{
			println!("Avg time of query using WHERE: {:?}\nAvg time of query using GROUP BY with limit of 1 row: {:?}\nSearching with GROUP BY is more eficcient", &self.avg_with_where, &self.avg_with_group_by);
		}
		println!("Time of adding index: {:?}", &self.time_of_creating_index);
	}
}


#[derive(Debug)]
pub struct SecoundProblem {
	select_asterisk:f32,
	select_names:f32,
}

impl SecoundProblem {
	pub fn new(select_asterisk:f32, select_names:f32) -> Self{
		Self{
			select_asterisk,
			select_names,
		}
	}
	pub fn conclude(&self){
		if &self.select_asterisk < &self.select_names {
			println!("Avg time of query with asterisk: {:?}\nAvg time of query without asterisk: {:?}\nEnngine simplify querys", &self.select_asterisk, &self.select_names);
		}else {
			println!("Avg time of query with asterisk: {:?}\nAvg time of query without asterisk: {:?}\nEnngine do not simplify querys", &self.select_asterisk, &self.select_names);
		}
	}
}