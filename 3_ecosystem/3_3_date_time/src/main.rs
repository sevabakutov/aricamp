use time::format_description::well_known::Iso8601;
use time::{Date, Month};

fn main() {}

const NOW: &str = "2019-06-26";

struct User(time::Date);

impl User {
    fn with_birthdate(year: i32, month: u32, day: u32) -> Self {
        let month = Month::try_from(month as u8).unwrap();
        let day: u8 = day.try_into().unwrap();
        Self(time::Date::from_calendar_date(year, month, day).unwrap())
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u16 {
        let now = Date::parse(NOW, &Iso8601::DEFAULT).unwrap();
        let birthdate = self.0;

        if birthdate > now {
            return 0;
        }

        let mut years = now.year() - birthdate.year();

        if (now.month(), now.day()) < (birthdate.month(), birthdate.day()) {
            years -= 1;
        }

        years.try_into().unwrap_or(0)
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 4), 29),
            ((1990, 7, 4), 28),
            ((0, 1, 1), 2019),
            ((1970, 1, 1), 49),
            ((2019, 6, 25), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in vec![
            ((2032, 6, 25), 0),
            ((2019, 6, 27), 0),
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }
}
