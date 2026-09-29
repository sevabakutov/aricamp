#[derive(Clone, Debug, PartialEq)]
pub enum State {
    New,
    Unmoderated,
    Published,
    Deleted,
}

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
pub struct Post {
    pub id: post::Id,
    pub user_id: user::Id,
    pub title: post::Title,
    pub body: post::Body,
    pub state: State,
}

impl Post {
    pub fn new(author_id: user::Id, id: post::Id, title: post::Title, body: post::Body) -> Post {
        Post {
            id,
            user_id: author_id,
            title,
            body,
            state: State::New,
        }
    }

    pub fn publish(&mut self) {
        if self.state == State::New {
            println!("waiting for approval {:?}", self.body);
            self.state = State::Unmoderated;
        } else {
            println!("wrong action");
        }
    }

    pub fn allow(&mut self) {
        if self.state == State::Unmoderated {
            println!("post: {:?} is published", self.title);
            self.state = State::Published;
        } else {
            println!("wrong action");
        }
    }

    pub fn delete(&mut self) {
        if self.state == State::Published {
            println!("post: {:?} has been deleted", self.title);
            self.state = State::Deleted;
        } else {
            println!("wrong action");
        }
    }

    pub fn deny(&mut self) {
        if self.state == State::Unmoderated {
            println!("{:?} was denied", self.body);
            self.state = State::Deleted;
        } else {
            println!("wrong action");
        }
    }
}

pub fn repost(post: &Post, new_author_id: user::Id) -> Post {
    let mut new_post = post.clone();
    new_post.user_id = new_author_id;
    new_post
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_post() -> Post {
        Post::new(
            user::Id(1),
            post::Id(100),
            post::Title("Test Title".to_string()),
            post::Body("Test Body".to_string()),
        )
    }

    #[test]
    fn test_new_post_initial_state() {
        let post = create_test_post();
        assert_eq!(post.state, State::New);
        assert_eq!(post.user_id, user::Id(1));
        assert_eq!(post.id, post::Id(100));
    }

    #[test]
    fn test_publish_from_new_state() {
        let mut post = create_test_post();
        post.publish();
        assert_eq!(post.state, State::Unmoderated);
    }

    #[test]
    fn test_publish_invalid_transition() {
        let mut post = create_test_post();
        post.publish();
        post.publish();

        assert_eq!(post.state, State::Unmoderated);
    }

    #[test]
    fn test_allow_from_unmoderated_state() {
        let mut post = create_test_post();
        post.publish();
        post.allow();

        assert_eq!(post.state, State::Published);
    }

    #[test]
    fn test_allow_invalid_transition_from_new() {
        let mut post = create_test_post();
        post.allow();
        assert_eq!(post.state, State::New);
    }

    #[test]
    fn test_deny_from_unmoderated_state() {
        let mut post = create_test_post();
        post.publish();
        post.deny();

        assert_eq!(post.state, State::Deleted);
    }

    #[test]
    fn test_deny_invalid_transition_from_published() {
        let mut post = create_test_post();
        post.publish();
        post.allow();
        post.deny();
        assert_eq!(post.state, State::Published);
    }

    #[test]
    fn test_delete_from_published_state() {
        let mut post = create_test_post();
        post.publish();
        post.allow();
        post.delete();

        assert_eq!(post.state, State::Deleted);
    }

    #[test]
    fn test_delete_invalid_transition_from_new() {
        let mut post = create_test_post();
        post.delete();
        assert_eq!(post.state, State::New);
    }

    #[test]
    fn test_repost() {
        let original_post = create_test_post();
        let new_author = user::Id(2);

        let new_post = repost(&original_post, new_author.clone());

        assert_eq!(new_post.user_id, new_author);
        assert_eq!(new_post.id, original_post.id);
        assert_eq!(new_post.title, original_post.title);
        assert_eq!(new_post.body, original_post.body);
        assert_eq!(new_post.state, original_post.state);

        assert_eq!(original_post.user_id, user::Id(1));
    }
}
