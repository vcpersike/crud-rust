use crate::application::user::User;
use crate::infra::repository::UserRepository;

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        UserService { repository }
    }

    pub fn create_user(&mut self, name: &str, email: &str) -> Result<(), String> {
        self.repository.create(name, email)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, String> {
        Ok(self.repository.get_all())
    }
}

pub mod user {
    #[derive(Debug, Clone)]
    pub struct User {
        pub id: u32,
        pub name: String,
        pub email: String,
    }
}
