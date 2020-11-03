use crate::schemas::root::Context;

#[derive(Default, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub phone: String,
    pub user_name: String,
    pub nick_name: String,
    // pub avatar_small: String,
    // pub avatar_medium: String,
    // pub avatar_large: String,
    // company: Option<Company>,
    // developer: Option<Developer>,
}

#[juniper::object(Context = Context)]
impl User {
    fn id(&self) -> &str {
        &self.id
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn phone(&self) -> &str {
        &self.phone
    }
    fn user_name(&self) -> &str {
        &self.user_name
    }

    fn nick_name(&self) -> &str {
        &self.nick_name
    }
}
