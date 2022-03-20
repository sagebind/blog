use vial::{Request, Response};

use crate::comments::CommentStore;

mod articles;
mod comments;
mod components;
mod csrf;
mod database;
mod pages;

vial::routes! {
    GET "/" => |_| pages::home().into_string();
    GET "/articles" => |_| pages::articles().into_string();
    GET "/tag/:tag" => |request| pages::tag(request.arg("tag").unwrap()).into_string();
    GET "/about" => |_| pages::about().into_string();
    GET "/feeds" => |_| pages::feeds().into_string();
    GET "/stuff" => |_| pages::stuff().into_string();
    GET "/reply" => |request| {
        if let Some(id) = request.query("id") {
            Some(components::comments::comment_form(Some(id)).into_string())
        } else {
            None
        }
    };
    GET "/:year/:month/:day/*name" => get_article;
    GET "/css/style.css" => |_| Response::from(include_str!(concat!(env!("OUT_DIR"), "/main.css")))
        .with_header("Content-Type", "text/css");
}

fn get_article(request: Request) -> Option<String> {
    let slug = format!(
        "{}/{}/{}/{}",
        request.arg("year").unwrap(),
        request.arg("month").unwrap(),
        request.arg("day").unwrap(),
        request.arg("name").unwrap()
    );

    let comments = request.state::<CommentStore>().tree_for_slug(&slug);

    if let Some(article) = articles::get_by_slug(&slug) {
        Some(pages::article(&article, &comments).into_string())
    } else {
        None
    }
}

fn main() {
    dotenv::dotenv().unwrap();

    let pool = database::create_connection_pool();
    let comment_store = CommentStore::new(pool);
    vial::use_state!(comment_store);

    vial::asset_dir!("wwwroot");
    vial::run!().unwrap();
}
