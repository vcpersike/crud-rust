use super::schema::usuario;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Usuario {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
    pub phone: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "usuario"]
pub struct NovoUsuario {
    pub name: String,
    pub password: String,
    pub email: String,
    pub phone: String,
}
