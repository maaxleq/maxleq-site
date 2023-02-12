use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {

}

#[derive(Template)]
#[template(path = "blog.html")]
pub struct BlogTemplate {

}

#[derive(Template)]
#[template(path = "resume.html")]
pub struct ResumeTemplate {

}

#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactTemplate {
    sent: bool
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {

}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {

}
