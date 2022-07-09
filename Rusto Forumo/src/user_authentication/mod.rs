use bcrypt;
use bcrypt::{DEFAULT_COST, hash};
use diesel;
use diesel::prelude::*;
use rocket;
use rocket_contrib::Template;
use rocket::http::{Cookie, Cookies};
use rocket::response::{Flash, Redirect};
use rocket::request::{FlashMessage, Form};
use serde_json;
use tera::Context;

use super::DbConn;
use self::user_model::*;
use self::user_forms::*;
mod user_forms;
pub mod user_model;


#[derive(Serialize)]
struct InvalidFormMessage<'a> {
    name: &'a str,
    msg: &'a str
}
////////////USER MATCHING AND HASHING DURING REGISTRATION HERE, BASED OF SOURCE AS WELL /////////////////////////////
#[post("/registration", data = "<form>")]
fn registration(
    user: AnonymousUser,
    form: Form<RegisterForm>,
    conn: DbConn
) -> Result<Redirect, Flash<Redirect>> {
    use super::schema::users::dsl::*;
    use crate::schema::users;

    let mut tera_cont = Context::new();
    tera_cont.insert("user", &user);
    
    let form = match form.get().validate_fields_presence() {
        Ok(val) => val,
        Err(e) => return Err(Flash::error(Redirect::to("/user_authentication/registration"), e.msg))
    };

    let found = users.filter(email_address.eq(&form.email_address.as_ref().unwrap().0))
        .get_results::<User>(&*conn)
        .expect("Error loading users");

    if found.len() > 0 {
        return Err(Flash::error(Redirect::to("/user_authentication/registration"), "Email already taken"))
    }
	
	let found = users.filter(username.eq(&form.username.as_ref().unwrap().0))
        .get_results::<User>(&*conn)
        .expect("Error loading users");

    if found.len() > 0 {
        return Err(Flash::error(Redirect::to("/user_authentication/registration"), "Username already taken"))
    }
	
    if &form.password.as_ref().unwrap().0 != &form.password_confirm.as_ref().unwrap().0 {
        return Err(Flash::error(Redirect::to("/user_authentication/registration"), "Passwords must match!"))
    }

    let secured_password = match hash (&form.password.as_ref().unwrap().0, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => panic!("Error hashing")
    };

    let new_user = NewUser {
		username: &form.username.as_ref().unwrap().0,
        firstname: &form.firstname.as_ref().unwrap().0,
        lastname: &form.lastname.as_ref().unwrap().0,
        email_address: &form.email_address.as_ref().unwrap().0,
        password: &secured_password
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&*conn)
        .expect("Error inserting user");

    Ok(Redirect::to("/user_authentication/login"))
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[get("/registration")]
fn sign_up(user: AnonymousUser, flash: Option<FlashMessage>) -> Template {
    let mut tera_cont = Context::new();
    tera_cont.insert("user", &user);

    if flash.is_some() {
        let flash_values = flash.unwrap();
        let info = InvalidFormMessage {
            name: &flash_values.name(),
            msg: &flash_values.msg()
        };

        tera_cont.insert("flash", &info);
    }

    Template::render("user_authentication/registration", &tera_cont)
}

#[post("/user_settings", data="<form>")]
fn update_settings(
    user: AuthenticatedUser,
    form: Form<RegisterForm>,
    conn: DbConn
) -> Result<Redirect, Flash<Redirect>> {
    use super::schema::users::dsl::*;

    let mut tera_cont = Context::new();
    tera_cont.insert("user", &user);
    
    let form = form.get();

    if &form.password.as_ref().unwrap().0 != &form.password_confirm.as_ref().unwrap().0 {
        return Err(Flash::error(Redirect::to("/user_authentication/registration"), "Passwords must match!"))
    }

    let secured_password = match hash (&form.password.as_ref().unwrap().0, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => panic!("Error hashing")
    };

    let updated_user = ModifyUser {
		username: &form.username.as_ref().unwrap().0,
        firstname: &form.firstname.as_ref().unwrap().0,
        lastname: &form.lastname.as_ref().unwrap().0,
        email_address: &form.email_address.as_ref().unwrap().0,
        password: &secured_password
    };

    println!("UserID: {:?}", &form.id.unwrap());

    diesel::update(users.find(form.id.unwrap()))
        .set(&updated_user)
        .get_result::<User>(&*conn)
        .expect("Error updating user");

    Ok(Redirect::to("/"))
}

#[get("/user_settings")]
fn user_settings(user: AuthenticatedUser, flash: Option<FlashMessage>) -> Template {
    let mut tera_cont = Context::new();
    tera_cont.insert("user", &user);
    
    if flash.is_some() {
        let flash_values = flash.unwrap();
        let info = InvalidFormMessage {
            name: &flash_values.name(),
            msg: &flash_values.msg()
        };

        tera_cont.insert("flash", &info);
    }
    
    Template::render("user_authenication/user_settings", &tera_cont)
}

#[get("/logout")]
fn logout(_user: AuthenticatedUser, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("Session Authentication"));
    Redirect::to("/")
}


#[post("/login", data = "<form>")]
fn authentication(
    _user: AnonymousUser,
    form: Form<LoginForm>,
    mut cookies: Cookies,
    conn: DbConn,
) -> Result<Redirect, Flash<Redirect>> {
    use super::schema::users::dsl::*;

    if cookies.get("Session Authentication").is_none() {
        let form = form.get();
        let found_users = users
            .filter(email_address.eq(&form.email_address))
            .limit(1)
            .load::<User>(&*conn)
            .expect("No Users Found!");

        if found_users.len() == 0 {
            return Err(Flash::error(Redirect::to("/user_authentication/login"), "Invalid credentials"))
        }

        let found_user = &found_users[0];

        if bcrypt::verify(&form.password, &found_user.password).unwrap() {
            let sessions = serde_json::to_string(&found_user);

            if sessions.is_ok() {
                let cookie = Cookie::build("Session Authentication".to_owned(), sessions.unwrap())
                    .path("/")
                    .finish();

                cookies.add_private(cookie);
            }
        } else {
            return Err(Flash::error(Redirect::to("/user_authentication/login"), "Invalid credentials"))
        }


    }

    Ok(Redirect::to("/"))
}

#[get("/login")]
fn login(user: AnonymousUser, flash: Option<FlashMessage>) -> Template {
    let mut tera_cont = Context::new();
    tera_cont.add("user", &user);

    if flash.is_some() {
        let flash_values = flash.unwrap();
        let info = InvalidFormMessage {
            name: &flash_values.name(),
            msg: &flash_values.msg()
        };

        tera_cont.add("flash", &info);
    }

    Template::render("user_authentication/login", &tera_cont)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![authentication, login, logout, sign_up, registration, user_settings, update_settings]
}
