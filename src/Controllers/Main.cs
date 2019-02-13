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
            return View("Index", articleStore
                .GetAll()
                .Reverse()
                .Take(7));
        }

        [Route("/about")]
        public IActionResult GetAbout()
        {
            return View("About");
        }

        [Route("/stuff")]
        public IActionResult GetStuff()
        {
            return View("Stuff");
        }

        [Route("/articles")]
        public IActionResult ListArticles()
        {
            return View("Articles", articleStore
                .GetAll()
                .Reverse());
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
            return View("Tag", articleStore
                .GetByTag(tag)
                .Reverse());
        }

        [Route("/{year}/{month}/{day}/{name}")]
        public IActionResult GetArticle(int year, int month, int day, string name)
        {
            var article = articleStore.GetBySlug(Request.Path.Value.Substring(1));

            if (article != null)
            {
                return View("Article", article);
            }

            return NotFound();
        }
    }
}
