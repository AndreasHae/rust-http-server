use serde::{Deserialize, Serialize};
use std::cmp::Eq;
use std::fmt::Debug;
use std::sync::RwLock;

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct UserCreation {
    name: String,
}

#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
pub struct User {
    id: u32,
    name: String,
}

pub struct AppState {
    users: RwLock<Vec<User>>,
}

impl AppState {
    pub fn create_user(&self, creation: UserCreation) -> User {
        let users = &mut self.users.write().unwrap();
        let new_user = User {
            id: users.len() as u32,
            name: creation.name.clone(),
        };
        users.push(new_user.clone());
        new_user
    }

    pub fn get_user_by_id(&self, id: u32) -> Option<User> {
        let users = self.users.read().unwrap();
        users
            .iter()
            .find(|&user| user.id == id)
            .map(|user| user.clone())
    }
}

impl Default for AppState {
    fn default() -> AppState {
        AppState {
            users: RwLock::new(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_user() {
        // Given
        let app_state = AppState::default();
        let creation = UserCreation {
            name: String::from("Andi"),
        };

        // When
        let result = app_state.create_user(creation);

        // Then
        let expected = User {
            name: String::from("Andi"),
            id: 0,
        };
        assert_eq!(result, expected);
        assert_eq!(app_state.users.into_inner().unwrap(), vec![expected]);
    }

    #[test]
    fn get_user() {
        // Given
        let app_state = AppState::default();
        let stored_user = app_state.create_user(UserCreation {
            name: String::from("Andi"),
        });

        // When
        let result = app_state.get_user_by_id(stored_user.id).unwrap();

        // Then
        assert_eq!(result, stored_user);
    }

    #[test]
    fn get_user_not_existing() {
        // Given
        let app_state = AppState::default();

        // When
        let result = app_state.get_user_by_id(0);

        // Then
        assert_eq!(result, Option::None)
    }
}
