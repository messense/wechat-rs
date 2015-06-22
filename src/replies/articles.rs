use time;
use replies::ReplyRenderer;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Article {
    pub title: String,
    pub description: String,
    pub url: String,
    pub image: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ArticlesReply {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub articles: Vec<Article>,
}

impl Article {
    
    #[inline]
    pub fn new(title: &str, url: &str) -> Article {
        Article {
            title: title.to_owned(),
            url: url.to_owned(),
            image: "".to_owned(),
            description: "".to_owned(),
        }
    }

    #[inline]
    pub fn with_image(title: &str, url: &str, image: &str) -> Article {
        Article {
            title: title.to_owned(),
            url: url.to_owned(),
            image: image.to_owned(),
            description: "".to_owned(),
        }
    }

    #[inline]
    pub fn with_description(title: &str, url: &str, description: &str) -> Article {
        Article {
            title: title.to_owned(),
            url: url.to_owned(),
            image: "".to_owned(),
            description: description.to_owned(),
        }
    }

    fn render(&self) -> String {
        format!("<item>\n
            <Title><![CDATA[{title}]]></Title>\n\
            <Description><![CDATA[{description}]]></Description>\n\
            <PicUrl><![CDATA[{picurl}]]></PicUrl>\n\
            <Url><![CDATA[{url}]]></Url>\n\
            </item>",
            title=self.title,
            description=self.description,
            picurl=self.image,
            url=self.url,
        )
    }
}

impl ArticlesReply {
    #[inline]
    pub fn new(source: &str, target: &str) -> ArticlesReply {
        ArticlesReply {
            source: source.to_owned(),
            target: target.to_owned(),
            time: time::get_time().sec,
            articles: vec![],
        }
    }

    #[inline]
    pub fn with_articles(source: &str, target: &str, articles: &[Article]) -> ArticlesReply {
        ArticlesReply {
            source: source.to_owned(),
            target: target.to_owned(),
            time: time::get_time().sec,
            articles: articles.to_vec(),
        }
    }

    pub fn add_article(&mut self, article: Article) -> bool {
        if self.articles.len() >= 10 {
            return false;
        }
        self.articles.push(article);
        true
    }
}

impl ReplyRenderer for ArticlesReply {
    #[inline]
    fn render(&self) -> String {
        let mut articles = vec![];
        for article in self.articles.iter() {
            articles.push(article.render());
        }
        let articles_str = articles.connect("\n");
        format!("<xml>\n\
            <ToUserName><![CDATA[{target}]]></ToUserName>\n\
            <FromUserName><![CDATA[{source}]]></FromUserName>\n\
            <CreateTime>{time}</CreateTime>\n\
            <MsgType><![CDATA[news]]></MsgType>\n\
            <ArticleCount>{count}</ArticleCount>\n\
            <Articles>{articles}</Articles>\n\
            </xml>",
            target=self.target,
            source=self.source,
            time=self.time,
            count=self.articles.len(),
            articles=articles_str,
        )
    }
}

#[cfg(test)]
mod tests {
    use replies::ReplyRenderer;
    use super::{Article, ArticlesReply};

    #[test]
    fn test_render_text_reply() {
        let mut reply = ArticlesReply::new("test1", "test2");
        let article1 = Article::new("test3", "test4");
        let article2 = Article::with_image("test5", "test6", "test7");
        let article3 = Article::with_description("test8", "test9", "test10");
        reply.add_article(article1);
        reply.add_article(article2);
        reply.add_article(article3);
        let rendered = reply.render();

        assert!(rendered.contains("test1"));
        assert!(rendered.contains("test2"));
        assert!(rendered.contains("test3"));
        assert!(rendered.contains("test4"));
        assert!(rendered.contains("test5"));
        assert!(rendered.contains("test6"));
        assert!(rendered.contains("test7"));
    }
}
