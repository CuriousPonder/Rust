use rocket::outcome::Outcome::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde_json;
use serde::ser;
use serde::ser::SerializeStruct;
use crate::schema::users;


#[derive(Debug, Identifiable, Queryable, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email_address: String,
    #[serde(skip_serializing, skip_deserializing)] pub password: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
	pub username: &'a str,
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub email_address: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct Mass_New_User {
	pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email_address: String,
    pub password: String,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct ModifyUser<'a> {
	pub username: &'a str,
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub email_address: &'a str,
    pub password: &'a str,
}

impl ser::Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut s = serializer.serialize_struct("User", 7)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("username", &self.username)?;
        s.serialize_field("firstname", &self.firstname)?;
        s.serialize_field("lastname", &self.lastname)?;
        s.serialize_field("email_address", &self.email_address)?;
        s.serialize_field("is_anonymous", &(self.id == -1))?;
        s.end()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<User, ()> {
        let req = request.clone();
        let mut cookies = req.cookies();

        if let Some(cookie) = cookies.get_private("Session_Authentication") {
            let user: Result<User, _> = serde_json::from_str(cookie.value());
            if user.is_ok() {
                return Success(user.unwrap());
            }
        }

        Success(User {
            id: -1,
            username: "".to_owned(),
            firstname: "".to_owned(),
            lastname: "".to_owned(),
            email_address: "".to_owned(),
            password: "".to_owned(),
        })
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<AuthenticatedUser, ()> {
        let req = request.clone();
        let mut cookies = req.cookies();

        if let Some(cookie) = cookies.get_private("Session_Authentication") {
            let user: Result<User, _> = serde_json::from_str(cookie.value());
            if user.is_err() {
                return Failure((Status::raw(600), ()));
            }

            return Success(AuthenticatedUser(user.unwrap()));
        }

        Failure((Status::raw(600), ()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnonymousUser(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for AnonymousUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<AnonymousUser, ()> {
        let req = request.clone();
        let mut cookies = req.cookies();

        if let Some(cookie) = cookies.get_private("Session_Authentication") {
            let user: Result<User, _> = serde_json::from_str(cookie.value());
            if user.is_ok() {
                return Failure((Status::raw(601), ()));
            }
        }

        Success(AnonymousUser(User {
            id: -1,
            username: "".to_owned(),
            firstname: "".to_owned(),
            lastname: "".to_owned(),
            email_address: "".to_owned(),
            password: "".to_owned(),
        }))
    }
}


