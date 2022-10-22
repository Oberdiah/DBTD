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

	pub fn make_player(&mut self, player_id: i32, player_name: String){
		let items : Vec<i32> = self.conn.exec(
			"INSERT INTO users
          VALUES (5, \"Shotekri\")",
			Params::Empty).unwrap();
	}


}

