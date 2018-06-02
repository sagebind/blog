using Microsoft.AspNetCore.Mvc;
using System;
using System.Linq;
using System.Text;

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
            return View("Feeds/Atom", articleStore.GetAll());
        }

        [Route("/feed/{category}")]
        [Route("/feed/{category}.atom")]
        public IActionResult GetAtomFeed(string category)
        {
            Response.ContentType = "application/atom+xml";
            return View("Feeds/Atom", articleStore.GetByCategory(category));
        }

        [Route("/feed.rss")]
        public IActionResult GetRssFeed()
        {
            Response.ContentType = "application/rss+xml";
            return View("Feeds/Rss", articleStore.GetAll());
        }

        [Route("/feed/{category}.rss")]
        public IActionResult GetRssFeed(string category)
        {
            Response.ContentType = "application/rss+xml";
            return View("Feeds/Rss", articleStore.GetByCategory(category));
        }
    }
}
