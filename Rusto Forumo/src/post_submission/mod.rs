use self::post_model::*;
use self::post_form::*;
use crate::user_authentication::user_model::*;
use crate::DbConn;
use diesel;
use diesel::prelude::*;
use rocket;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;
use tera::Context;
pub mod post_model;
mod post_form;

#[post("/post_update", data = "<form>")]
fn post_update(user: AuthenticatedUser, form: Form<postUpdate>, conn: DbConn) -> Result<Redirect, Flash<Redirect>> {
    use super::schema::posts::dsl::*;

    let submitted_posting = form.get();

    if user.0.id != submitted_posting.user_id {
        let url = &format!("/post_submission/viewer/{}", submitted_posting.id)[..];
        return Err(Flash::error(Redirect::to(url), "Unauthorized Action"))
    }

    let update = Post_Update {
        user_id: None,
        post_title: &submitted_posting.post_title[..].to_string(),
        description: &submitted_posting.description[..].to_string(),
        is_published: false,
    };

    diesel::update(posts.find(submitted_posting.id))
        .set(&update)
        .get_result::<Post>(&*conn)
        .expect("Error updating Post");

    Ok(Redirect::to("/"))
}

#[post("/initiate", data = "<form>")]
fn initiate(user: AuthenticatedUser, form: Form<newSubmission>, conn: DbConn) -> Redirect {
    use crate::schema::posts;

    let submitted_posting = form.get();

    let new_submission = Post_Creation {
		user_id: user.0.id,
        post_title: &submitted_posting.post_title,
        description: &submitted_posting.description,
    };

    diesel::insert_into(posts::table)
        .values(&new_submission)
        .get_result::<Post>(&*conn)
        .expect("Error saving new post");

    Redirect::to("/")
}

#[get("/modify/<post_id>")]
fn modify(user: AuthenticatedUser, post_id: i32, conn: DbConn) -> Result<Template, Flash<Redirect>>{
    use super::schema::posts::dsl::*;

    let mut tera_cont = Context::new();
    tera_cont.insert("user", &user);

    let post = posts
        .find(post_id)
        .get_result::<Post>(&*conn)
        .expect("Exception! Could not load posts");

    if user.0.id != post.user_id {
        let url = &format!("/posts/viewer/{}", post_id)[..];
        return Err(Flash::error(Redirect::to(url), "Unauthorized Action"))
    }

    tera_cont.insert("post", &post);

    Ok(Template::render("post_submission/modify", &tera_cont))
}

#[get("/viewer/<post_id>")]
fn viewer(user: User, pop: Option<FlashMessage>, post_id: i32, conn: DbConn) -> Template {
   
    let mut tera_cont = Context::new();
    
    if pop.is_some() {
        let msg_values = pop.unwrap();
        let info = InvalidFormMessage {
            name: &msg_values.name(),
            msg: &msg_values.msg()
        };

        tera_cont.insert("pop", &info);
    }
   
    let author_postings = Author_Post::search(post_id, conn);

    tera_cont.insert("post", &author_postings);
    tera_cont.insert("user", &user);

    Template::render("post_submission/viewer", &tera_cont)
}

#[get("/starter")]
fn starter(user: AuthenticatedUser) -> Template {
    let mut tera_cont = Context::new();

    tera_cont.insert("user", &user);

    Template::render("post_submission/starter", &tera_cont)
}

#[post("/erase", data = "<form>")]
fn erase(user: AuthenticatedUser, form: Form<DeletePostForm>, conn: DbConn) -> Result<Redirect, Flash<Redirect>> {
    use super::schema::posts::dsl::*;
    
    let submitted_posts = form.get();
    
    let post = posts.find(&submitted_posts.id)
        .get_result::<Post>(&*conn)
        .expect("Error loading post");
    
    if user.0.id != post.user_id {
        let url = &format!("/post_submission/viewer/{}", post.id)[..];
        return Err(Flash::error(Redirect::to(url), "Unauthorized Action"))
    }

    diesel::delete(&post)
        .execute(&*conn)
        .expect("Error deleting post");
    
    Ok(Redirect::to("/"))
}

#[get("/deletion/<post_id>")]
fn deletion(user: AuthenticatedUser, post_id: i32, conn: DbConn) -> Result<Template, Flash<Redirect>> {
    use super::schema::posts::dsl::*;
    
    let mut tera_cont = Context::new();

    let submission = posts
        .find(post_id)
        .get_result::<Post>(&*conn)
        .expect("Error loading post");
    
    if user.0.id != submission.user_id {
        let url = &format!("/post_submission/viewer/{}", post_id)[..];
        return Err(Flash::error(Redirect::to(url), "Unauthorized Action"))
    }
    
    tera_cont.insert("user", &user);
    tera_cont.insert("post", &submission);

    Ok(Template::render("post_submission/deletion", &tera_cont))
}

#[get("/")]
fn home(user: User, conn: DbConn) -> Template {
    let mut tera_cont = Context::new();

    let published_posts = Author_Post::seed_information(conn);

    tera_cont.insert("posts", &published_posts);
    tera_cont.insert("user", &user);

    Template::render("post_submission/home", &tera_cont)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![home, initiate, deletion, erase, modify, starter, viewer, post_update]
}
