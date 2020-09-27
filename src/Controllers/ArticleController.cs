using Ganss.XSS;
using Microsoft.AspNetCore.Mvc;

namespace Blog.Controllers
{
    public class ArticleController : Controller
    {
        private readonly ArticleStore articleStore;
        private readonly HtmlSanitizer htmlSanitizer = new HtmlSanitizer();

        public ArticleController(ArticleStore articleStore)
        {
            this.articleStore = articleStore;

            htmlSanitizer.AllowedAttributes.Remove("contenteditable");
            htmlSanitizer.AllowedAttributes.Remove("draggable");
            htmlSanitizer.AllowedAttributes.Remove("dropzone");
            htmlSanitizer.AllowedAttributes.Remove("src");
            htmlSanitizer.AllowedSchemes.Add("mailto");
            htmlSanitizer.AllowedTags.Remove("button");
            htmlSanitizer.AllowedTags.Remove("fieldset");
            htmlSanitizer.AllowedTags.Remove("form");
            htmlSanitizer.AllowedTags.Remove("img");
            htmlSanitizer.AllowedTags.Remove("input");
            htmlSanitizer.AllowedTags.Remove("label");
            htmlSanitizer.AllowedTags.Remove("legend");
            htmlSanitizer.AllowedTags.Remove("main");
            htmlSanitizer.AllowedTags.Remove("menu");
            htmlSanitizer.AllowedTags.Remove("menuitem");
            htmlSanitizer.AllowedTags.Remove("option");
            htmlSanitizer.AllowedTags.Remove("progress");
            htmlSanitizer.AllowedTags.Remove("select");
            htmlSanitizer.AllowedTags.Remove("textarea");
        }

        [HttpGet]
        [Route("/{year}/{month}/{day}/{name}")]
        public IActionResult Get(int year, int month, int day, string name)
        {
            var article = articleStore.GetBySlug($"{year:D2}/{month:D2}/{day:D2}/{name}");

            if (article == null)
            {
                return NotFound();
            }

            return View("Article", article);
        }

        [HttpPost]
        [Route("/{year}/{month}/{day}/{name}/comments")]
        [Consumes("application/x-www-form-urlencoded")]
        [ValidateAntiForgeryToken]
        public IActionResult SubmitComment(
            int year,
            int month,
            int day,
            string name,
            [FromForm] SubmitCommentRequest request
        )
        {
            // if (!ModelState.IsValid)
            // {
            //     return BadRequest();
            // }

            var article = articleStore.GetBySlug($"{year:D2}/{month:D2}/{day:D2}/{name}");

            if (article == null)
            {
                return NotFound();
            }

            request.Text = htmlSanitizer.Sanitize(request.Text);

            if (Request.IsHtmx())
            {
                return View("ArticleComments", article);
            }

            return Redirect($"/{article.Slug}");
        }
    }
}
