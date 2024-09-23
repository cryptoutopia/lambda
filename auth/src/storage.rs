use crate::error::Error;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

pub trait AuthStorage {
    fn get_user_by_email(&self, email: &str) -> Result<User, Error>;
    fn create_user(&self, email: &str, password: &str) -> Result<(), Error>;
    fn set_user_password(&self, email: &str, password: &str) -> Result<(), Error>;
}

pub struct PostgrestAuthStorage {
    client: Postgrest,
    table_name: String,
}

impl PostgrestAuthStorage {
    pub fn new(url: String, table_name: String) -> Self {
        Self { client: Postgrest::new(url), table_name }
    }
}

impl AuthStorage for PostgrestAuthStorage {
    fn get_user_by_email(&self, email: &str) -> Result<User, Error> {
        let response = self.client
            .from(&self.table_name)
            .select("*")
            .eq("email", email)
            .execute()
            .map_err(|e| Error::DatabaseError(e.to_string()))?;

        let users: Vec<User> = response.json().map_err(|e| Error::DatabaseError(e.to_string()))?;

        if users.is_empty() {
            return Err(Error::UserNotFound);
        }

        Ok(users[0].clone())
    }

    fn create_user(&self, email: &str, password: &str) -> Result<(), Error> {
        let user = User {
            email: email.to_string(),
            password: password.to_string(),
        };

        self.client
            .from(&self.table_name)
            .insert(&[user])
            .execute()
            .map_err(|e| Error::DatabaseError(e.to_string()))?;
        Ok(())
    }

    fn set_user_password(&self,, email: &str, password: &str) -> Result<(), Error> {
        let user = User {
            email: email.to_string(),
            password: password.to_string(),
        };

        self.client
            .from(&self.table_name)
            .update(&user)
            .eq("email", email)
            .execute()
            .map_err(|e| Error::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    email: String,
    password: String,
}