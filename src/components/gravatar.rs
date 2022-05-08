use maud::{html, Markup};
use md5::{Digest, Md5};

pub fn gravatar(email: Option<&str>) -> Markup {
    let url = if let Some(email) = email {
        format!("https://www.gravatar.com/avatar/{:x}?d=identicon", Md5::digest(email.trim().to_ascii_lowercase()))
    } else {
        String::from("https://www.gravatar.com/avatar/00000000000000000000000000000000?d=identicon")
    };

    html! {
        img src=(url);
    }
}
