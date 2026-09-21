use core::hash;
use std::{borrow::Cow, collections::HashMap, hash::Hash};

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(PartialEq, Debug)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

struct UserDb<K, V> {
    map: HashMap<K, V>,
}

impl<K: Eq + Hash, V> Storage<K, V> for UserDb<K, V> {
    fn set(&mut self, key: K, val: V) {
        self.map.insert(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

impl<K, V> Storage<K, V> for Box<dyn Storage<K, V>> {
    fn set(&mut self, key: K, val: V) {
        (**self).set(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        (**self).get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        (**self).remove(key)
    }
}

struct UserRepository<S> {
    storage: S,
}

impl<S> UserRepository<S>
where
    S: Storage<u64, User>
    {
    fn add(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn get(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    fn update(&mut self, user: User) {
        if self.storage.get(&user.id).is_some() {
            self.storage.set(user.id, user);
        }
    }

    fn remove(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}


fn main() {

}


#[cfg(test)]
mod test {
    use super::*;

    fn create_user(id: u64) -> User {
        User {
            id,
            email: Cow::Borrowed("alwdka@gmail.com"),
            activated: true,
        }
    }

    #[test]
    fn static_dispatch() {
        let user1 = create_user(1);
        let mut repo = UserRepository {
            storage: UserDb {
                map: HashMap::new(),
            },
        };

        repo.add(user1);
        let a = repo.get(1).unwrap();
        assert_eq!(a, &create_user(1));
        assert_eq!(a.id, 1);
    }

    #[test]
    fn dynamic_dispatch() {
        let db: UserDb<u64, User> = UserDb {
            map: HashMap::new(),
        };
        let storage_box: Box<dyn Storage<u64, User>> = Box::new(db);

        let mut repo = UserRepository {
            storage: storage_box,
        };

        let user1 = create_user(1);
        repo.add(user1);
        let a = repo.get(1).unwrap();
        assert_eq!(a, &create_user(1));
        assert_eq!(a.id, 1);
    }
}
