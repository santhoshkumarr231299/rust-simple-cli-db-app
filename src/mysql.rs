use mysql::*;

pub fn get_connection() -> PooledConn {
    let url = "mysql://root:root@localhost:3306/db";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    return conn;
}