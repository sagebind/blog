use std::env;

use maud::html;
use poem::{
    endpoint::StaticFilesEndpoint,
    get,
    http::StatusCode,
    post,
    web::{Data, Form, Html, Path, Query},
    EndpointExt, IntoResponse, Response, Route,
};
use serde::Deserialize;

use crate::{
    comments::{CommentStore, PostComment},
    feeds::FeedFormat, web::ClientIp,
};

mod articles;
mod comments;
mod components;
mod csrf;
mod database;
mod feeds;
mod highlight;
mod markdown;
mod pages;
mod web;

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

#[derive(Clone, Debug, Deserialize)]
struct FeedParams {
    tag: Option<String>,
}

#[poem::handler]
async fn get_article_feed(
    format: Option<Path<FeedFormat>>,
    Query(params): Query<FeedParams>,
) -> Response {
    let format = format.map(|Path(f)| f).unwrap_or_default();

    let feed = if let Some(tag) = params.tag {
        feeds::articles(
            format!("Stephen Coakley - {tag}"),
            format!("Articles tagged \"{tag}\""),
            format!("https://stephencoakley.com/feed.{format}?tag={tag}"),
            &articles::get_tagged(tag),
        )
    } else {
        feeds::articles(
            "Stephen Coakley",
            "Latest articles from a Disciple of Christ and software engineer. I post infrequently and usually on technical topics.",
            format!("https://stephencoakley.com/feed.{format}"),
            articles::get_all(false),
        )
    };

    feed.into_response(format)
}

#[poem::handler]
async fn get_comments_feed(
    comment_store: Data<&CommentStore>,
    format: Option<Path<FeedFormat>>,
) -> Response {
    let format = format.map(|Path(f)| f).unwrap_or_default();
    let comments = comment_store.get_newest().await;

    feeds::comments(
        "Stephen Coakley - Comments",
        "Comments on all articles",
        format!("https://stephencoakley.com/comments/feed.{format}"),
        &comments,
    )
    .into_response(format)
}

#[poem::handler]
async fn get_article_comments_feed(
    comment_store: Data<&CommentStore>,
    Path((slug, format)): Path<(String, FeedFormat)>,
) -> Response {
    if let Some(article) = articles::get_by_slug(&slug) {
        let comments = comment_store.fetch_all_comments_for_slug(&slug).await;

        feeds::comments(
            format!("{} - Comments", article.title),
            format!("Comments on \"{}\"", article.title),
            format!(
                "https://stephencoakley.com/{}/comments.{format}",
                article.slug
            ),
            &comments,
        )
        .into_response(format)
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

#[derive(Clone, Debug, Deserialize)]
struct ReplyFormParams {
    id: String,
    show: bool,
}

#[poem::handler]
fn get_comment_reply_form(
    Path(slug): Path<String>,
    Query(ReplyFormParams { id, show }): Query<ReplyFormParams>,
) -> Html<String> {
    Html(html! {
        a hx-swap-oob={ "outerHTML:#comment-" (&id) "-reply-link" } id={ "comment-" (&id) "-reply-link" } hx-get={ "/" (&slug) "/comments/reply?id=" (&id) "&show=" (!show) } hx-target={ "#comment-" (&id) "-reply" } {
            @if show {
                "close"
            } @else {
                "reply"
            }
        }

        @if show {
            (components::comments::comment_form(&slug, Some(&id)))
        }
    }.into_string())
}

#[poem::handler]
async fn post_comment(
    comment_store: Data<&CommentStore>,
    Path(slug): Path<String>,
    Form(post): Form<PostComment>,
) -> Html<String> {
    // Post the comment.
    comment_store.post(&slug, post).await;

    // Reload the comment tree.
    let comments = comment_store.tree_for_slug(&slug).await;

    Html(components::comments::comments_section(&slug, &comments).into_string())
}

#[poem::handler]
async fn post_comment_upvote(
    comment_store: Data<&CommentStore>,
    Path((slug, comment_id)): Path<(String, String)>,
    ClientIp(addr): ClientIp,
) -> Html<String> {
    comment_store.upvote(&comment_id, addr).await;

    // Reload the comment tree.
    let comments = comment_store.tree_for_slug(&slug).await;

    Html(components::comments::comments_section(&slug, &comments).into_string())
}

#[poem::handler]
async fn post_comment_downvote(
    comment_store: Data<&CommentStore>,
    Path((slug, comment_id)): Path<(String, String)>,
    ClientIp(addr): ClientIp,
) -> Html<String> {
    comment_store.downvote(&comment_id, addr).await;

    // Reload the comment tree.
    let comments = comment_store.tree_for_slug(&slug).await;

    Html(components::comments::comments_section(&slug, &comments).into_string())
}

#[poem::handler]
fn get_tag(Path(tag): Path<String>) -> Html<String> {
    Html(pages::tag(&tag).into_string())
}

#[poem::handler]
fn style() -> poem::web::WithContentType<&'static str> {
    include_str!(concat!(env!("OUT_DIR"), "/main.css")).with_content_type("text/css")
}

#[poem::handler]
async fn get_article(
    comment_store: Data<&CommentStore>,
    Path(slug): Path<String>,
) -> poem::Result<Html<String>> {
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

    // Parse embedded articles at startup.
    articles::get_all(true);

    log::info!("creating database connection pool...");
    let pool = database::create_connection_pool();
    let comment_store = CommentStore::new(pool);

    let app = Route::new()
        .at("/", get(home))
        .at("/about", get(about))
        .at("/feeds", get(get_feeds))
        .at("/stuff", get(stuff))
        .at("/articles", get(get_articles))
        .at("/tag/:tag", get(get_tag))
        .at("/category/:tag", get(get_tag))
        .at("/feed", get(get_article_feed))
        .at("/feed.:format<rss|atom|json>", get(get_article_feed))
        .at("/comments/feed", get(get_comments_feed))
        .at(
            "/comments/feed.:format<rss|atom|json>",
            get(get_comments_feed),
        )
        .at(r"/:slug<\d{4}/\d{2}/\d{2}/[^/]+>", get(get_article))
        .at(
            r"/:slug<\d{4}/\d{2}/\d{2}/[^/]+>/comments/reply",
            get(get_comment_reply_form),
        )
        .at(
            r"/:slug<\d{4}/\d{2}/\d{2}/[^/]+>/comments",
            post(post_comment),
        )
        .at(
            r"/:slug<\d{4}/\d{2}/\d{2}/[^/]+>/comments/:id/upvotes",
            post(post_comment_upvote),
        )
        .at(
            r"/:slug<\d{4}/\d{2}/\d{2}/[^/]+>/comments/:id/downvotes",
            post(post_comment_downvote),
        )
        .at(
            r"/:slug<\d{4}/\d{2}/\d{2}/[^/]+>/comments.:format<rss|atom|json>",
            get(get_article_comments_feed),
        )
        .at("/css/style.css", get(style))
        .nest("/assets", StaticFilesEndpoint::new("wwwroot/assets"))
        .nest("/content", StaticFilesEndpoint::new("wwwroot/content"))
        .data(comment_store);

    let addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| String::from("127.0.0.1:5000"));

    log::info!("listening on http://{}", addr);
    poem::Server::new(poem::listener::TcpListener::bind(addr))
        .run(app)
        .await
}
