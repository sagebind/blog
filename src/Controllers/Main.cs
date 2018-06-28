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

        [Route("/category/{category}")]
        public IActionResult ListArticlesInCategory(string category)
        {
            ViewData["Category"] = category;
            return View("Category", articleStore
                .GetByCategory(category)
                .Reverse());
        }

        [Route("/{year}/{month}/{day}/{name}")]
        public IActionResult GetArticle(int year, int month, int day, string name)
        {
            return View("Article", articleStore.GetBySlug(Request.Path.Value.Substring(1)));
        }
    }
}
