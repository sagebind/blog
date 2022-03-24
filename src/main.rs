use feeds::Feed;
use poem::{
    endpoint::StaticFilesEndpoint,
    get, post,
    web::{Data, Form, Html, Path, Json, WithContentType},
    EndpointExt, IntoResponse,
};

use crate::comments::{CommentStore, PostComment};

mod articles;
mod comments;
mod components;
mod csrf;
mod database;
mod feeds;
mod markdown;
mod pages;

#[poem::handler]
fn home() -> Html<String> {
    Html(pages::home().into_string())
}

#[poem::handler]
fn about() -> Html<String> {
    Html(pages::about().into_string())
}

#[poem::handler]
fn get_feeds() -> Html<String> {
    Html(pages::feeds().into_string())
}

#[poem::handler]
fn stuff() -> Html<String> {
    Html(pages::stuff().into_string())
}

#[poem::handler]
fn get_articles() -> Html<String> {
    Html(pages::articles().into_string())
}

#[poem::handler]
async fn get_article_comments(
    comment_store: Data<&CommentStore>,
    Path(slug): Path<ArticleSlug>,
) -> Json<Feed> {
    let comments = comment_store.fetch_all_comments_for_slug(&slug.slug()).await;

    Json(feeds::comments(&comments))
}

#[poem::handler]
async fn get_article_feed() -> Json<Feed> {
    Json(feeds::articles(&articles::get_all(false)))
}

#[poem::handler]
async fn get_article_feed_rss() -> WithContentType<String> {
    feeds::rss::to_rss(feeds::articles(&articles::get_all(false))).with_content_type("application/rss+xml; charset=utf-8")
}

#[poem::handler]
async fn get_article_feed_atom() -> WithContentType<String> {
    feeds::atom::to_atom(feeds::articles(&articles::get_all(false))).with_content_type("application/atom+xml; charset=utf-8")
}

#[poem::handler]
async fn post_comment(
    comment_store: Data<&CommentStore>,
    Path(request): Path<ArticleSlug>,
    Form(post): Form<PostComment>,
) -> Html<String> {
    let article_slug = request.slug();

    // Post the comment.
    comment_store.post(&article_slug, post).await;

    // Reload the comment tree.
    let comments = comment_store.tree_for_slug(&article_slug).await;

    Html(components::comments::comments_section(&article_slug, &comments).into_string())
}

#[poem::handler]
fn get_tag(Path(tag): Path<String>) -> Html<String> {
    Html(pages::tag(&tag).into_string())
}

#[poem::handler]
fn style() -> poem::web::WithContentType<&'static str> {
    include_str!(concat!(env!("OUT_DIR"), "/main.css")).with_content_type("text/css")
}

#[derive(serde::Deserialize)]
struct ArticleSlug {
    year: u16,
    month: u8,
    day: u8,
    name: String,
}

impl ArticleSlug {
    fn slug(&self) -> String {
        format!(
            "{:04}/{:02}/{:02}/{}",
            self.year, self.month, self.day, self.name
        )
    }
}

#[poem::handler]
async fn get_article(
    comment_store: Data<&CommentStore>,
    Path(request): Path<ArticleSlug>,
) -> poem::Result<Html<String>> {
    let slug = request.slug();
    let comments = comment_store.tree_for_slug(&slug).await;

    if let Some(article) = articles::get_by_slug(&slug) {
        Ok(Html(pages::article(&article, &comments).into_string()))
    } else {
        Err(poem::Error::from_status(poem::http::StatusCode::NOT_FOUND))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().unwrap();
    env_logger::init();

    let pool = database::create_connection_pool();
    let comment_store = CommentStore::new(pool);

    let app = poem::Route::new()
        .at("/", get(home))
        .at("/about", get(about))
        .at("/feeds", get(get_feeds))
        .at("/stuff", get(stuff))
        .at("/articles", get(get_articles))
        .at("/tag/:tag", get(get_tag))
        .at("/category/:tag", get(get_tag))
        .at("/:year/:month/:day/:name", get(get_article))
        .at("/:year/:month/:day/:name/comments", post(post_comment))
        .at("/feed.json", get(get_article_feed))
        .at("/feed.rss", get(get_article_feed_rss))
        .at("/feed.atom", get(get_article_feed_atom))
        .at("/:year/:month/:day/:name/comments.json", get(get_article_comments))
        .at("/css/style.css", get(style))
        .nest("/assets", StaticFilesEndpoint::new("wwwroot/assets"))
        .nest("/content", StaticFilesEndpoint::new("wwwroot/content"))
        .data(comment_store);

    log::info!("listening on {}", "127.0.0.1:7667");
    poem::Server::new(poem::listener::TcpListener::bind("127.0.0.1:7667"))
        .run(app)
        .await
}
