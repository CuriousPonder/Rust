use rocket::http::RawStr;
use rocket::request::FromFormValue;

#[derive(Debug, FromForm, Serialize)]
pub struct LoginForm {
    pub email_address: String,
    pub password: String,
}

pub struct UserField(pub String);
pub struct FieldPresenceError<'a> { 
    pub msg: &'a str,
}

impl<'v> FromFormValue<'v> for UserField {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<UserField, &'v RawStr> {
        match form_value.percent_decode() {
            Ok(ref val) if val.is_empty() => Err(form_value),
            Ok(ref val) => Ok(UserField(val.to_string())),
            Err(_val) => Err(form_value)
        }
    }
}

#[derive(FromForm)]
pub struct RegisterForm {
    pub id: Option<i32>,
    pub username: Option<UserField>,
    pub firstname: Option<UserField>,
    pub lastname: Option<UserField>,
    pub email_address: Option<UserField>,
    pub password: Option<UserField>,
    pub password_confirm: Option<UserField>,
}

impl RegisterForm {
    pub fn validate_fields_presence(&self) -> Result<&Self, FieldPresenceError> {
        if self.username.is_some() &&
			self.firstname.is_some() &&
            self.lastname.is_some() &&
            self.email_address.is_some() &&
            self.password.is_some() &&
            self.password_confirm.is_some()
            {
                return Ok(self);
            }

        Err(FieldPresenceError { msg: "Everything must be filled out!" })
    }
}
