extern crate rusto_lib;
extern crate diesel;
extern crate bcrypt;
#[macro_use] extern crate fake;
use bcrypt::{DEFAULT_COST, hash};
use rusto_lib::*;
use crate::rusto_lib::post_submission::post_model::*;
use crate::rusto_lib::user_authentication::user_model::*;
use diesel::prelude::*;


fn main() {
    use rusto_lib::schema::posts::dsl::*;
    use rusto_lib::schema::users::dsl::*;

    let connection = database_pool().get().unwrap();
    let test_pass = "password_test";
    let hash_pw = match hash (test_pass, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => panic!("Error hashing")
    };
    diesel::delete(posts).execute(&*connection).expect("Error deleteing posts");
    diesel::delete(users).execute(&*connection).expect("Error deleteing users");

    fn gen_user(pw: &str) -> Mass_New_User {
         Mass_New_User {
		   username: fake!(Name.name),
           firstname: fake!(Name.name),
           lastname: fake!(Name.name),
           email_address: fake!(Internet.free_email),
           password: pw.to_string(),
        }
    }

    // Randomly generate post info
    fn gen_post(user: User) -> Mass_Post_Creation {
        let _post_title = &fake!(Lorem.sentence(1, 4))[..];
        let _description = &fake!(Lorem.paragraph(5,5))[..];

        Mass_Post_Creation {
           user_id: user.id,
           post_title: _post_title.to_string(),
           description: _description.to_string(),
        }
    }

    // Create personal login
    let me = NewUser {
		username: "el jake",
        firstname: "jake",
        lastname: "el",
        email_address: "eljakegeneric@gmail.com",
        password: &hash_pw[..],
    };
    
    diesel::insert_into(users)
        .values(&me)
        .execute(&*connection)
        .expect("Error inserting users");

    // Create 10 randomly generated users stored as a vec
    let new_user_list: Vec<Mass_New_User> = (0..10)
        .map( |_| gen_user(&hash_pw))
        .collect();

    // Insert that vec of users and get a vec back of the inserts
    let ret_user = diesel::insert_into(users)
        .values(&new_user_list)
        .get_results::<User>(&*connection)
        .expect("Error inserting users");

    // For each of the new users, create some posts
    let new_post_list: Vec<Mass_Post_Creation> = ret_user
        .into_iter()
        .map(|user| gen_post(user))
        .collect();

    // Insert those posts
    diesel::insert_into(posts)
        .values(&new_post_list)
        .execute(&*connection)
        .expect("Error inserting posts");
}
