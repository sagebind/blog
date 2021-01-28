using Microsoft.AspNetCore.Mvc;
using System.Linq;

namespace Blog.Controllers
{
    public class Main : Controller
    {
        private readonly ArticleStore articleStore;

        public Main(ArticleStore articleStore)
        {
            this.articleStore = articleStore;
        }

        [Route("/")]
        public IActionResult GetIndex()
        {
            return View("Index", articleStore.GetAll().Take(7));
        }

        [Route("/about")]
        public IActionResult GetAbout()
        {
            return View("About");
        }

        [Route("/selftest")]
        public IActionResult GetSelftest()
        {
            return View("Selftest");
        }

        [Route("/stuff")]
        public IActionResult GetStuff()
        {
            return View("Stuff");
        }

        [Route("/articles")]
        public IActionResult ListArticles()
        {
            return View("Articles", articleStore.GetAll());
        }

        // I got rid of categories, but redirect to the equivalent tag to be helpful.
        [Route("/category/{category}")]
        public IActionResult ListArticlesInCategory(string category)
        {
            return RedirectPermanent($"/tag/{Tags.Normalize(category)}");
        }

        [Route("/tag/{tag}")]
        public IActionResult ListArticlesTagged(string tag)
        {
            tag = Tags.Normalize(tag);
            ViewData["Tag"] = tag;
            return View("Tag", articleStore.GetByTag(tag));
        }

        [HttpGet]
        [Route("/{year}/{month}/{day}/{name}")]
        public IActionResult GetArticle(int year, int month, int day, string name)
        {
            var article = articleStore.GetBySlug($"{year:D2}/{month:D2}/{day:D2}/{name}");

            if (article == null)
            {
                return NotFound();
            }

            return View("Article", article);
        }
    }
}
