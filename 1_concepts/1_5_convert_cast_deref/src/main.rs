use std::ops::Deref;

use rand::Rng;

struct EmailString {
    email: String,
}

impl EmailString {

    fn new(email: impl Into<String>) -> Result<Self, &'static str> {
        let email = email.into();
        if Self::is_valid(&email) {
            return Ok(Self { email });
        } else {
            return Err("not valid email addres");
        }
    }

    fn is_valid(email: &str) -> bool {
        let mut parts = email.split("@");
        let part1 = parts.next();
        let part2 = parts.next();
        let part3 = parts.next();

        let part1_res = match part1 {
            Some(s) => s,
            None => "",
        };

        let part2_res = match part2 {
            Some(s) => s,
            None => "",
        };

        if part3.is_some() {
            return false;
        }

        if part1_res.is_empty() {
            return false;
        }

        if !part2_res.contains('.') {
            return false;
        }

        true
    }
}

impl Deref for EmailString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.email
    }
}

impl TryFrom<&str> for EmailString {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match EmailString::new(value) {
            Ok(email) => Ok(email),
            Err(err) => Err(err),
        }
    }
}

impl TryFrom<String> for EmailString {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match EmailString::new(value) {
            Ok(email) => Ok(email),
            Err(err) => Err(err),
        }
    }
}

struct Random<T> {
    val1: Box<T>,
    val2: Box<T>,
    val3: Box<T>,
}

impl<T> Random<T> {
    fn new(val1: T, val2: T, val3: T) -> Self {
        Random { val1: Box::new(val1), val2: Box::new(val2), val3: Box::new(val3) }
    }
}

impl<T> From<[T; 3]> for Random<T>
where
    T: Copy
{
    fn from(value: [T; 3]) -> Self {
        Random::new(value[0], value[1], value[2])
    }
}

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(1..=3);

        match num {
            1 => &self.val1,
            2 => &self.val2,
            3 => &self.val3,
            _ => &self.val1,
        }
    }
}


fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_valid() {
        let valid_emails = vec!["user@example.com", "a.b@domain.co.uk", "name+tag@test.org"];
        for email in valid_emails {
            assert!(EmailString::new(email).is_ok(), "Email должен быть валидным: {}", email);
        }
    }

    #[test]
    fn test_email_invalid() {
        let invalid_emails = vec![
            "",                 // пустая строка
            "invalidemail.com", // нет @
            "user@domain",      // нет точки в домене
            "@domain.com",      // нет имени пользователя
            "user@domain@test.com", // больше одной @
        ];

        for email in invalid_emails {
            assert!(EmailString::new(email).is_err(), "Email должен быть невалидным: {}", email);
        }
    }

    #[test]
    fn test_email_deref() {
        let email = EmailString::new("test@example.com").unwrap();
        // Проверяем, что Deref позволяет использовать методы &str
        assert_eq!(&*email, "test@example.com");
        assert!(email.ends_with(".com"));
    }

    #[test]
    fn test_email_try_from() {
        // Проверка TryFrom<&str>
        let email_ref: Result<EmailString, _> = "hello@world.com".try_into();
        assert!(email_ref.is_ok());

        // Проверка TryFrom<String>
        let email_string: Result<EmailString, _> = String::from("hello@world.com").try_into();
        assert!(email_string.is_ok());
    }
}
