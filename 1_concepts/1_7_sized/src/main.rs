use std::{borrow::Cow, collections::HashMap, hash::Hash};

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(PartialEq, Debug, Clone)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

trait UserRepository {
    fn add(&mut self, user: User);
    fn get(&self, id: u64) -> Option<&User>;
    fn update(&mut self, user: User);
    fn remove(&mut self, id: u64) -> Option<User>;
}

struct StorageUserRepository<S> {
    storage: S,
}

impl<S> UserRepository for StorageUserRepository<S>
where
    S: Storage<u64, User>,
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

trait CommandHandler<C> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &mut Self::Context) -> Self::Result;
}

struct CreateUser {
    id: u64,
    email: Cow<'static, str>
}

enum UserError {
    AlreadyExists
}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), UserError>;

    fn handle_command(&self, cmd: &CreateUser, user_repo: &mut Self::Context) -> Self::Result {
        if user_repo.get(cmd.id).is_some() {
            return Err(UserError::AlreadyExists);
        };

        let user = User {
            id: cmd.id,
            email: cmd.email.clone(),
            activated: false
        };

        user_repo.add(user);

        Ok(())
    }
}

struct HashMapStorage(HashMap<u64, User>);

impl Storage<u64, User> for HashMapStorage {
    fn set(&mut self, key: u64, val: User) {
        self.0.insert(key, val);
    }

    fn get(&self, key: &u64) -> Option<&User> {
        self.0.get(key)
    }

    fn remove(&mut self, key: &u64) -> Option<User> {
        self.0.remove(key)
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;


    fn create_user_and_storage() -> (User, StorageUserRepository<HashMapStorage>) {
        let user = User {
            id: 0,
            email: Cow::Borrowed("awddw@gmail.com"),
            activated: true,
        };

        let repo = StorageUserRepository { storage: HashMapStorage(HashMap::new()) };

        (user, repo)
    }

    #[test]
    fn test_create_user_success() {
        let (user, mut repo) = create_user_and_storage();
        let cmd = CreateUser { id: 1, email: Cow::Borrowed("test@gmail.com") };

        assert!(user.handle_command(&cmd, &mut repo as &mut dyn UserRepository).is_ok());
        assert!(repo.get(1).is_some());
    }

    #[test]
    fn test_create_user_already_exists() {
        let (user, mut repo) = create_user_and_storage();
        repo.add(User { id: 1, email: Cow::Borrowed(""), activated: true });

        let cmd = CreateUser { id: 1, email: Cow::Borrowed("test@gmail.com") };

        let res = user.handle_command(&cmd, &mut repo as &mut dyn UserRepository);
        assert!(matches!(res, Err(UserError::AlreadyExists)));
    }
}
