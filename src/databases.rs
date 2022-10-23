use mysql::*;
use mysql::prelude::Queryable;

pub struct Db{
	conn: PooledConn,
}

impl Db{
	pub fn init() -> Self{
		let url = "mysql://monty:some_pass@192.168.1.103:3306/dbtd";
		let pool = Pool::new(url).unwrap();

		let mut conn = pool.get_conn().unwrap();

		Self{
			conn
		}
	}

	pub fn make_player(&mut self, player_name: &str){
		self.conn.exec_batch(
			"INSERT INTO users (username) VALUES (?);",
			vec![
				Params::Positional(vec![mysql::Value::from(player_name)],),
			]).unwrap();
	}


}

