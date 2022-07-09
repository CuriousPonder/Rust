#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate tera;
extern crate rusto_lib;
extern crate rocket;
extern crate rocket_contrib;

use rusto_lib::*;
use rusto_lib::{user_authentication, post_submission, /*superuser_authentication, group_submission*/};
use rocket::{Catcher, Error};
use rocket_contrib::Template;
use rocket::request::Request;
use rocket::response;
use rocket::response::{NamedFile, Redirect, Responder};
use std::path::{Path, PathBuf};
use tera::Context;

#[get("/webpage_design/<file..>")]
fn setup(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("webpage_design/").join(file)).ok()
}

#[get("/")]
fn redi() -> Redirect {
    Redirect::to("/post_submission")
}

fn redirect_login<'r>(_: Error, r: &'r Request) -> response::Result<'r> {
    Redirect::to("/user_authentication/login").respond_to(r)
}

fn redirect_core<'r>(_: Error, r: &'r Request) -> response::Result<'r> {
        Redirect::to("/").respond_to(r)
}

fn main() {

	let login = Catcher::new(1000, redirect_login);
	let core = Catcher::new(1001, redirect_core);		
	
	rocket::ignite()
		.manage(database_pool())
		.mount("/", routes![redi, setup])
		.mount("/user_authentication", user_authentication::routes())
		.mount("/post_submission", post_submission::routes())
		//.mount("/superuser_authentication", auth::routes())
		//.mount("/group_submission", posts::routes())
		.attach(Template::fairing())
		.catch(vec![login, core])
		.launch();
}
