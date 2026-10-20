use std::collections::HashSet;
use im::HashMap;

pub mod user {
    #[derive(Clone, Debug, PartialEq, Hash, Eq)]
    pub struct UserId(pub usize);

    #[derive(Clone, Debug, PartialEq, Hash, Eq)]
    pub struct UserName(pub &'static str);

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User {
    id: user::UserId,
    user_name: user::UserName,
}


struct UserDatabase(HashMap<user::UserId, User>);

trait UserRepository {
    fn get_user(&self, id: user::UserId) -> Option<&User>;

    fn get_users(&self, ids: HashSet<user::UserId>) -> HashMap<user::UserId, Option<&User>>;

    fn search(&self, string: &str) -> HashSet<&User>;
}

impl UserRepository for UserDatabase {
    fn get_user(&self, id: user::UserId) -> Option<&User> {
        self.0.get(&id)
    }

    fn get_users(&self, ids: HashSet<user::UserId>) -> HashMap<user::UserId, Option<&User>> {
       ids.into_iter()
           .map(|id| {
               let user = self.0.get(&id);
               (id, user)
           }).collect()
    }

    fn search(&self, string: &str) -> HashSet<&User> {
        self.0.iter()
            .filter(|(_, user)|user.user_name.0.contains(string))
            .map(|user| user.1)
            .collect()
    }
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use im::{hashmap, HashMap};

    fn setup_db() -> (UserDatabase, User, User) {
        let user1 = User {
            id: user::UserId(1),
            user_name: user::UserName("Alice"),
        };
        let user2 = User {
            id: user::UserId(2),
            user_name: user::UserName("Bob"),
        };

        let map = hashmap! {
            user1.id.clone() => user1.clone(),
            user2.id.clone() => user2.clone(),
        };
        (UserDatabase(map), user1, user2)
    }

    #[test]
    fn test_get_user() {
        let (db, user1, _) = setup_db();

        let found = db.get_user(user::UserId(1));
        assert_eq!(found, Some(&user1));

        let not_found = db.get_user(user::UserId(999));
        assert_eq!(not_found, None);
    }

    #[test]
    fn test_get_users() {
        let (db, user1, user2) = setup_db();

        let mut ids = HashSet::new();
        ids.insert(user::UserId(1));
        ids.insert(user::UserId(999));
        let result = db.get_users(ids);

        assert_eq!(result.get(&user::UserId(1)), Some(&Some(&user1)));
        assert_eq!(result.get(&user::UserId(999)), Some(&None));
        assert_eq!(result.get(&user::UserId(2)), None);
    }

    #[test]
    fn test_search() {
        let (db, user1, user2) = setup_db();

        let results = db.search("lic"); // "Alice"
        assert_eq!(results.len(), 1);
        assert!(results.contains(&user1));

        let results_all = db.search("");
        assert_eq!(results_all.len(), 2);
        assert!(results_all.contains(&user1));
        assert!(results_all.contains(&user2));

        let results_empty = db.search("NonExistent");
        assert!(results_empty.is_empty());
    }
}
