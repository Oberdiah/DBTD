use mysql::prelude::Queryable;
use mysql::*;

pub struct MapResult {
	pub clear_time: f32,
	pub completer:  String,
	pub map:        String,
}

pub struct Db {
	conn: PooledConn,
}

impl Db {
	pub fn init() -> Self {
		let url = "mysql://monty:some_pass@192.168.1.103:3306/dbtd";
		let pool = Pool::new(url).unwrap();

		let mut conn = pool.get_conn().unwrap();

		Self { conn }
	}

	pub fn new_player(&mut self, player_name: &str) {
		self.conn
			.exec_batch("INSERT INTO users (username) VALUES (?);", vec![Params::Positional(vec![
				mysql::Value::from(player_name),
			])])
			.unwrap();
	}

	pub fn upload_map(&mut self, player_name: &str, map_name: &str, map_data: Vec<u8>) {
		self.conn
			.exec_batch("INSERT INTO maps (owner, map_name, map_data) VALUES (?, ?, ?);", vec![
				Params::Positional(vec![
					mysql::Value::from(player_name),
					mysql::Value::from(map_name),
					mysql::Value::from(map_data),
				]),
			])
			.unwrap();
	}

	pub fn set_map_finished(&mut self, map_name: &str, finished: bool) {
		self.conn
			.exec_batch("UPDATE maps SET finished = ? WHERE map_name = ?;", vec![Params::Positional(
				vec![mysql::Value::from(finished), mysql::Value::from(map_name)],
			)])
			.unwrap();
	}

	pub fn get_map(&mut self, map_name: &str) -> Option<Vec<u8>> {
		let mut result = self
			.conn
			.exec_iter(
				"SELECT map_data FROM maps WHERE map_name = ?;",
				Params::Positional(vec![mysql::Value::from(map_name)]),
			)
			.unwrap();
		return result.next()?.unwrap().take(0)?;
	}

	pub fn get_finished_maps(&mut self) -> Vec<String> {
		let mut results = self
			.conn
			.exec_iter("SELECT map_name FROM maps WHERE finished = 1;", Params::Empty)
			.unwrap();
		let mut maps = vec![];
		for result in results {
			let map_name = result.unwrap().take(0).unwrap();
			maps.push(map_name);
		}
		return maps;
	}

	pub fn get_map_names_by_player(&mut self, player_name: &str) -> Vec<String> {
		let mut result = self
			.conn
			.exec_iter(
				"SELECT map_name FROM maps WHERE owner = ?;",
				Params::Positional(vec![mysql::Value::from(player_name)]),
			)
			.unwrap();
		let mut map_names = Vec::new();
		while let Some(row) = result.next() {
			let row = row.unwrap();
			map_names.push(row.get(0).unwrap());
		}
		return map_names;
	}

	pub fn upload_result(&mut self, player_name: &str, map_name: &str, time: f32) {
		// Insert player name, map name and time into the "results" database:
		self.conn
			.exec_batch(
				"INSERT INTO results (completer, map_name, clear_time)  VALUES (?, ?, ?);",
				vec![Params::Positional(vec![
					mysql::Value::from(player_name),
					mysql::Value::from(map_name),
					mysql::Value::from(time),
				])],
			)
			.unwrap();
	}

	pub fn get_results_for_map(&mut self, map_name: &str) -> Vec<MapResult> {
		let mut result = self.conn.exec_iter(
			"SELECT clear_time, completer, map_name FROM results WHERE map_name = ? ORDER BY clear_time ASC;",
				Params::Positional(vec![mysql::Value::from(map_name)],),
			).unwrap();
		let mut results = Vec::new();
		while let Some(row) = result.next() {
			let row = row.unwrap();
			results.push(MapResult {
				clear_time: row.get(0).unwrap(),
				completer:  row.get(1).unwrap(),
				map:        row.get(2).unwrap(),
			});
		}
		return results;
	}
}
