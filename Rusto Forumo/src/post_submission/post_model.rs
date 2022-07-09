use serde::ser;
use serde::ser::SerializeStruct;

use crate::DbConn;
use crate::user_authentication::user_model::User;
use crate::schema::posts;

#[derive(Debug, Associations, Identifiable, Queryable, Serialize)]
#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub post_title: String,
    pub description: String,
    pub file_id: i32,
    pub files: String,
    pub file_size: String,
    pub file_ext: String,
    pub file_name: String,
    pub is_published: bool,
}

#[derive(Debug, Queryable)]
pub struct Author_Post{
    pub id: i32,
    pub user_id: i32,
    pub post_title: String,
    pub description: String,
    pub file_id: i32,
    pub files: String,
    pub file_size: String,
    pub file_ext: String,
    pub file_name: String,
    pub is_published: bool,
    pub username: String,   
}

impl ser::Serialize for Author_Post {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut cereal = serializer.serialize_struct("Author_Post", 11)?;
        cereal.serialize_field("id", &self.id)?;
        cereal.serialize_field("user_id", &self.user_id)?;
        cereal.serialize_field("post_title", &self.post_title)?;
        cereal.serialize_field("description", &self.description)?;
        cereal.serialize_field("file_id", &self.file_id)?;
        cereal.serialize_field("files", &self.files)?;
        cereal.serialize_field("file_size", &self.file_size)?;
        cereal.serialize_field("file_ext", &self.file_ext)?;
        cereal.serialize_field("file_name", &self.file_name)?;
        cereal.serialize_field("is_published", &self.is_published)?;
        cereal.serialize_field("username", &self.username)?;
        cereal.end()
    }
}

impl Author_Post {

    pub fn search(post_id: i32, conn: DbConn) -> Author_Post {
        use diesel::prelude::*;
        use crate::schema::posts::dsl::*;
        use crate::schema::users;
        use crate::schema::posts;

        let base_query = posts.find(post_id);
        base_query.inner_join(users::table)
            .select(
                (
                    posts::id,
					posts::user_id,
					posts::post_title,
					posts::description,
					posts::file_id,
					posts::files,
					posts::file_size,
					posts::file_ext,
					posts::file_name,
					posts::is_published,
                    users::username,
                    
                )
            )
            .first::<Author_Post>(&*conn)
            .expect("Error loading post")
    }

	pub fn seed_information(conn: DbConn) -> Vec<Author_Post> {
		use diesel::prelude::*;
		use crate::schema::posts::dsl::*;
		use crate::schema::users;
		use crate::schema::posts;

		posts.inner_join(users::table)
		    .select(
		        (
		            posts::id,
					posts::user_id,
					posts::post_title,
					posts::description,
					posts::file_id,
					posts::files,
					posts::file_size,
					posts::file_ext,
					posts::file_name,
					posts::is_published,
		            
		            users::username,
		        )
		    )
		    .order(id.desc())
		    .get_results::<Author_Post>(&*conn)
		    .expect("Error loading post")
	    }
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct Post_Creation<'a> {
    pub user_id: i32,
    pub post_title: &'a str,
    pub description: &'a str,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct Mass_Post_Creation {
    pub user_id: i32,
    pub post_title: String,
    pub description: String,
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct Post_Update<'a> {
    pub user_id: Option<i32>,
    pub post_title: &'a str,
    pub description: &'a str,
    pub is_published: bool,
}
