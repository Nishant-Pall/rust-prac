pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub content: String,
    pub username: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        // default implementation
        String::from("(Read more...)")
    }
}

fn main() {
    let tweet = Tweet {
        content: String::from("Hello World"),
        username: String::from("@johndoe"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("The sky is falling"),
        content: String::from("Yes, the sky is falling"),
        author: String::from("John Doe"),
    };

    println!("Tweet Summary: {}", tweet.summarize());
    println!("News Summary: {}", article.summarize());
    println!("{}", article.summarize_author());
    println!("{}", tweet.summarize_author());
}
