use std::env::{self, Args, args};
use std::borrow::Cow;

fn main() {
    let a = parse_args();
    match a {
        Err(err) => panic!("Error occured: {err}"),
        Ok(path) => println!("path: {path}"),
    }
}

fn parse_args() -> Result<Cow<'static, str>, String> {
    let mut args = env::args();
    args.next();

    let first_arg = args.next();

    match first_arg {
        Some(s) => {
            if s == "--conf" {
                match args.next() {
                    Some(path) => Ok(Cow::Owned(path)),
                    None => Err("--conf flag is used but no path provided".to_string())
                }
            } else {
                return parse_env_var();
            }
        }
        None => {
            return parse_env_var();
        },
    }
}


fn parse_env_var() -> Result<Cow<'static, str>, String> {
    let a = env::var("APP_CONF");
    match a {
        Ok(s) => {
            if s.is_empty() {
                return Ok(Cow::Borrowed("/etc/app/app.conf"))
            } else {
                return Ok(Cow::Owned(s))
            }
        }
        Err(_) => {
            return Ok(Cow::Borrowed("/etc/app/app.conf"))
        }
    }
}
