pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_news_article() {
        let article = NewsArticle {
            headline: String::from("Rust reaches 1.0"),
            location: String::from("Internet"),
            author: String::from("The Rust Team"),
            content: String::from("Rust is ready for production."),
        };

        assert_eq!(
            article.summarize(),
            "Rust reaches 1.0, by The Rust Team (Internet)"
        );
    }

    #[test]
    fn summarizes_social_post() {
        let post = SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from("people"),
            reply: false,
            repost: false,
        };

        assert_eq!(post.summarize(), "horse_ebooks: people");
    }
}
