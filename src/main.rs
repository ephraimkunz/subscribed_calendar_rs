#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::hyper::header::Basic;
use rocket::http::{Header, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::NamedFile;
use std::str::FromStr;

// GET request to http://70.228.79.10/basicauth.ics

fn main() {
    rocket::ignite()
        .mount("/", routes![basicauth])
        .register(catchers![unauthorized])
        .launch();
}

#[derive(Debug)]
enum LoginError {
    WrongUsername,
    WrongPassword,
    MissingAuthorization,
}

struct AuthenticatedUser;

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
    type Error = LoginError;
    fn from_request(request: &'a Request<'r>) -> Outcome<AuthenticatedUser, LoginError> {
        const AUTH_KEY: &str = "Authorization";
        const BASIC_SCHEME: &str = "Basic ";
        println!("{:?}", request.uri());
        println!("{:?}", request.headers());

        let auth_header_value = request.headers().get_one(AUTH_KEY);
        match auth_header_value {
            None => Outcome::Failure((Status::Unauthorized, LoginError::MissingAuthorization)),
            Some(value) => {
                let encoded_portion = value.trim_start_matches(BASIC_SCHEME);
                let basic_auth = Basic::from_str(encoded_portion);
                match basic_auth {
                    Err(_) => {
                        Outcome::Failure((Status::Unauthorized, LoginError::MissingAuthorization))
                    }
                    Ok(basic_auth) => {
                        match (
                            basic_auth.password == Some("password".to_string()),
                            basic_auth.username == *"username",
                        ) {
                            (true, true) => Outcome::Success(AuthenticatedUser),
                            (false, true) => {
                                Outcome::Failure((Status::Unauthorized, LoginError::WrongPassword))
                            }
                            _ => {
                                Outcome::Failure((Status::Unauthorized, LoginError::WrongUsername))
                            }
                        }
                    }
                }
            }
        }
    }
}

#[get("/basicauth.ics")]
fn basicauth(_user: AuthenticatedUser) -> std::io::Result<NamedFile> {
    NamedFile::open("data/basicauth.ics")
}

#[derive(Responder)]
#[response(status = 401)]
struct BasicAuthResponder {
    inner: String,
    header: Header<'static>,
}

#[catch(401)]
fn unauthorized() -> BasicAuthResponder {
    BasicAuthResponder {
        inner: "Authorization header missing or incorrect username or password".to_string(),
        header: Header::new(
            "WWW-Authenticate",
            "Basic realm=Protected subscribed calendar data charset=UTF-8",
        ),
    }
}
