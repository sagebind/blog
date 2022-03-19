use crate::comments::CommentStore;

mod articles;
mod comments;
mod components;
mod pages;

vial::routes! {
    GET "/" => |_| pages::home().into_string();
    GET "/articles" => |_| pages::articles().into_string();
    GET "/tag/:tag" => |request| pages::tag(request.arg("tag").unwrap()).into_string();
    GET "/about" => |_| pages::about().into_string();
    GET "/feeds" => |_| pages::feeds().into_string();
    GET "/stuff" => |_| pages::stuff().into_string();
    GET "/:year/:month/:day/*name" => |request| {
        let slug = format!(
            "{}/{}/{}/{}",
            request.arg("year").unwrap(),
            request.arg("month").unwrap(),
            request.arg("day").unwrap(),
            request.arg("name").unwrap()
        );

        let comments = CommentStore::new().tree_for_slug(&slug);

        if let Some(article) = articles::get_by_slug(&slug) {
            Response::from(pages::article(&article, &comments).into_string())
        } else {
            Response::from(404).with_body("<h1>404 Not Found</h1>")
        }
    };
}

fn main() {
    dotenv::dotenv().unwrap();

    vial::asset_dir!("wwwroot");
    vial::run!().unwrap();
}
