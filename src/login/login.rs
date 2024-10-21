//use crate::login::encryptor::Encryptor;
//use crate::database::db::Database;

pub struct Login {
    pub username: String,
    pub password: String
}

impl Login {
    pub fn new(username: String, password: String) -> Self{
        Self {
            username,
            password
        }
    }

    pub fn attempt(/* db: Database,*/) {
        
    }

    pub fn create() {

    }
} 