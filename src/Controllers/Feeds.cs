using Microsoft.AspNetCore.Mvc;
using System;
using System.Linq;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace Blog.Controllers
{
    public class Feeds : Controller
    {
        private readonly ArticleStore articleStore;
        private readonly CommentStore commentStore;

        public Feeds(ArticleStore articleStore, CommentStore commentStore)
        {
            this.articleStore = articleStore;
            this.commentStore = commentStore;
        }

        [Route("/sitemap.xml")]
        public IActionResult GetSitemap()
        {
            return View("Feeds/Sitemap", articleStore.GetAll());
        }

        [Route("/feed")]
        [Route("/feed.{format}")]
        public IActionResult GetFeed(string format = "atom", [FromQuery] string tag = null)
        {
            if (!string.IsNullOrWhiteSpace(tag))
            {
                return FeedResult(format, new Feed
                {
                    Title = $"Stephen Coakley - {tag}",
                    Description = $"Articles tagged \"{tag}\"",
                    SelfLink = $"https://stephencoakley.com/feed.{format}?tag={tag}",
                    Items = articleStore.GetByTag(Tags.Normalize(tag)).Select(article => (Feed.Item)article)
                });
            }

            return FeedResult(format, new Feed
            {
                Title = "Stephen Coakley",
                Description = "Latest articles from a Disciple of Christ and software engineer. I post infrequently and usually on technical topics.",
                SelfLink = $"https://stephencoakley.com/feed.{format}",
                Items = articleStore.GetAll().Select(article => (Feed.Item)article)
            });
        }

        [Route("/feed/{tag}")]
        [Route("/feed/{tag}.{format}")]
        public IActionResult GetTagFeed(string tag, string format = "atom")
        {
            return RedirectPermanent($"/feed.{format}?tag={tag}");
        }

        [Route("/comments/feed")]
        [Route("/comments/feed.{format}")]
        public async Task<IActionResult> GetCommentsFeed(string format = "atom")
        {
            return FeedResult(format, new Feed
            {
                Title = "Stephen Coakley - Comments",
                Description = "Comments on all articles",
                SelfLink = $"https://stephencoakley.com/comments/feed.{format}",
                Items = await commentStore.GetNewest().Select(comment => (Feed.Item)comment).ToListAsync()
            });
        }

        private IActionResult FeedResult(string format, Feed feed)
        {
            switch (format)
            {
                case "atom":
                    return View("Feeds/Atom", feed);

                case "rss":
                    return View("Feeds/Rss", feed);

                case "json":
                    return JsonFeedResult(feed);

                default:
                    return NotFound();
            }
        }

        private IActionResult JsonFeedResult(Feed feed)
        {
            var result = Json(new
            {
                Version = "https://jsonfeed.org/version/1.1",
                Title = feed.Title,
                Description = feed.Description,
                HomePageUrl = "https://stephencoakley.com",
                FeedUrl = feed.SelfLink,
                Language = "en-US",
                Favicon = "https://stephencoakley.com/assets/images/favicon.ico",
                Authors = new[] {
                    new {
                        Name = "Stephen Coakley",
                        Url = "https://stephencoakley.com"
                    }
                },
                Items = feed.Items.Select(item => new
                {
                    Id = item.Uri,
                    Url = item.Uri,
                    Title = item.Title,
                    DatePublished = item.DatePublished,
                    Tags = item.Tags,
                    Authors = item.Authors?.Select(author => new
                    {
                        Name = author.Name,
                        Url = author.Uri
                    }),
                    ContentHtml = item.Html
                })
            }, new JsonSerializerOptions
            {
                DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
                PropertyNamingPolicy = new SnakeCaseJsonNamingPolicy()
            });

            result.ContentType = "application/feed+json; charset=utf-8";

            return result;
        }
    }
}
