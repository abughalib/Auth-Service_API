use super::models::SessionUser;
use yarte::Template;

#[derive(Template)]
#[template(path="pages/help_page.hbs")]
pub struct HomePage {}

#[derive(Template)]
#[template(path = "pages/register.hbs")]
pub struct Register {
    pub sent: bool,
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "pages/password.hbs")]
pub struct Password {
    pub email: String,
    pub path_id: String,
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "pages/me.hbs")]
pub struct Me {
    pub user: SessionUser,
}

#[derive(Template)]
#[template(path = "pages/sign_in.hbs")]
pub struct SignIn {
    pub error: Option<String>,
}
