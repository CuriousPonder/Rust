#[derive(FromForm)]
pub struct postUpdate {
    pub id: i32,
    pub user_id: i32,
    pub post_title: String,
    pub description: String,
    
}

#[derive(FromForm)]
pub struct newSubmission {
    pub post_title: String,
    pub description: String,
}

#[derive(FromForm)]
pub struct DeletePostForm {
    pub id: i32,
}

#[derive(Serialize)]
pub struct InvalidFormMessage<'a> {
    pub name: &'a str,
    pub msg: &'a str,
}

