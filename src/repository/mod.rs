use crate::database::Database;
pub struct UserRepository<'a> {
    db: &'a Database,
}

pub fn find_all(&self) -> Vec<User> {
    let sql = "SELECT id, name, email FROM users";
    let mut stmt = self.db.conn.prepare(sql).unwrap();
    let rows = stmt.query_map(params![], |row: &Row| {
        User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        }
    }).unwrap();
    rows.map(|r| r.unwrap()).collect()
}


impl<'a> UserRepository<'a> {
    pub fn new(db: &'a Database) -> UserRepository<'a> {
        UserRepository { db }
    }

    pub fn find_all(&self) -> Vec<User> {
        let mut stmt = self;
    }
}