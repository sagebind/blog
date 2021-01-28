using System;
using System.Collections.Generic;
using System.Linq;

namespace Blog
{
    public class Feed
    {
        public string Title { get; set; }
        public string Description { get; set; }
        public string SelfLink { get; set; }
        public IEnumerable<Item> Items { get; set; }
        public DateTimeOffset? LastUpdated => Items.FirstOrDefault()?.DatePublished;

        public class Author
        {
            public string Name { get; set; }
            public string Uri { get; set; }
        }

        public class Item
        {
            public Uri Uri { get; set; }
            public string Title { get; set; }
            public DateTimeOffset DatePublished { get; set; }
            public Author[] Authors { get; set; }
            public string[] Tags { get; set; }
            public string Html { get; set; }

            public static explicit operator Item(Article article)
            {
                return new Item
                {
                    Uri = article.CanonicalUri,
                    Title = article.Title,
                    DatePublished = article.Date,
                    Tags = article.Tags,
                    Html = article.Html
                };
            }

            public static explicit operator Item(Comment comment)
            {
                return new Item
                {
                    Uri = comment.GetCanonicalUri(),
                    Title = $"Comment on {comment.ArticleSlug.Substring(11)} by {comment.Author.Name}",
                    DatePublished = comment.Published,
                    Authors = new Author[]{
                        new Author
                        {
                            Name = comment.Author.Name,
                            Uri = comment.Author.Website
                        }
                    },
                    Html = comment.Html
                };
            }
        }
    }
}
