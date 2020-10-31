using Microsoft.AspNetCore.Mvc;
using System;
using System.Linq;

namespace Blog.Controllers
{
    public class Feeds : Controller
    {
        private readonly ArticleStore articleStore;

        public Feeds(ArticleStore articleStore)
        {
            this.articleStore = articleStore;
        }

        [Route("/sitemap.xml")]
        public IActionResult GetSitemap()
        {
            Response.ContentType = "application/xml";
            return View("Feeds/Sitemap", articleStore.GetAll());
        }

        [Route("/feed")]
        [Route("/feed.atom")]
        public IActionResult GetAtomFeed()
        {
            Response.ContentType = "application/atom+xml";
            return View("Feeds/Atom", articleStore.GetAll().Reverse());
        }

        [Route("/feed/{tag}")]
        [Route("/feed/{tag}.atom")]
        public IActionResult GetAtomFeed(string tag)
        {
            tag = Tags.Normalize(tag);
            Response.ContentType = "application/atom+xml";
            return View("Feeds/Atom", articleStore.GetByTag(tag).Reverse());
        }

        [Route("/feed.rss")]
        public IActionResult GetRssFeed()
        {
            Response.ContentType = "application/rss+xml";
            return View("Feeds/Rss", articleStore.GetAll().Reverse());
        }

        [Route("/feed/{tag}.rss")]
        public IActionResult GetRssFeed(string tag)
        {
            tag = Tags.Normalize(tag);
            Response.ContentType = "application/rss+xml";
            return View("Feeds/Rss", articleStore.GetByTag(tag).Reverse());
        }
    }
}
