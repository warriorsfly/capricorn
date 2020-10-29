#[derive(Default, Debug)]
pub struct User {
    pub id: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub user_name: String,
    pub real_name: String,
    pub avatar_small: Option<String>,
    pub avatar_medium: Option<String>,
    pub avatar_large: Option<String>,
    // company: Option<Company>,
    // developer: Option<Developer>,
}

#[juniper::object(Context = Context)]
impl User {
    fn id(&self) -> &str {
        &self.id
    }
}
