use std::marker::PhantomData;

pub mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(pub String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(pub String);
}

pub mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);
}

#[derive(Clone, Debug, PartialEq)]
pub struct Post<State> {
    pub id: post::Id,
    pub user_id: user::Id,
    pub title: post::Title,
    pub body: post::Body,
    state: PhantomData<State>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct New;
#[derive(Clone, Debug, PartialEq)]
pub struct Unmoderated;
#[derive(Clone, Debug, PartialEq)]
pub struct Published;
#[derive(Clone, Debug, PartialEq)]
pub struct Deleted;

impl Post<New> {
    pub fn publish(self)-> Post<Unmoderated> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: PhantomData,
        }
    }
}
impl Post<Unmoderated> {
    pub fn allow(self) -> Post<Published> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: PhantomData,
        }
    }


    pub fn deny(self) -> Post<Deleted> {
         Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: PhantomData,
        }

    }
}
impl Post<Published> {
    pub fn delete(self) -> Post<Deleted> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: PhantomData,
        }

    }
}

pub fn repost(post: &Post<Published>, new_author_id: user::Id) -> Post<Published> {
    let mut new_post = post.clone();
    new_post.user_id = new_author_id;
    new_post
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_post_lifecycle_and_repost() {
        let new_post = Post {
            id: post::Id(1),
            user_id: user::Id(100),
            title: post::Title("Заголовок".to_string()),
            body: post::Body("Текст статьи".to_string()),
            state: PhantomData,
        };

        let unmoderated_post = new_post.publish();
        assert_eq!(unmoderated_post.user_id, user::Id(100));

        let published_post = unmoderated_post.allow();
        assert_eq!(published_post.id, post::Id(1));

        let new_author = user::Id(200);
        let reposted_post = repost(&published_post, new_author.clone());

        assert_eq!(reposted_post.user_id, new_author);
        assert_eq!(reposted_post.title, published_post.title);
        assert_eq!(reposted_post.body, published_post.body);

        let deleted_post = published_post.delete();
        assert_eq!(deleted_post.id, post::Id(1));
    }

    #[test]
    fn test_deny_unmoderated_post() {
        let new_post = Post {
            id: post::Id(2),
            user_id: user::Id(101),
            title: post::Title("Bad Post".to_string()),
            body: post::Body("Spam".to_string()),
            state: PhantomData,
        };

        let unmoderated_post = new_post.publish();
        let deleted_post = unmoderated_post.deny();

        assert_eq!(deleted_post.id, post::Id(2));
    }


    #[test]
    fn test_invalid_transition_fails_to_compile() {
        let new_post = Post {
            id: post::Id(3),
            user_id: user::Id(102),
            title: post::Title("Title".to_string()),
            body: post::Body("Body".to_string()),
            state: PhantomData,
        };

        let published_post = new_post.allow();
        //Error:...
        let deleted_post = new_post.delete();
    }
}
