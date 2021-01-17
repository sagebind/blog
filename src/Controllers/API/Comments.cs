using System.Linq;
using System.Threading.Tasks;
using Ganss.XSS;
using Microsoft.AspNetCore.Mvc;

namespace Blog.Controllers
{
    [Route("/api/comments")]
    public class Comments : Controller
    {
        private readonly ArticleStore articleStore;
        private readonly CommentStore commentStore;
        private readonly CommentAuthorService commentAuthorService;
        private readonly HtmlSanitizer htmlSanitizer = new HtmlSanitizer();

        public Comments(
            ArticleStore articleStore,
            CommentStore commentStore,
            CommentAuthorService commentAuthorService
        )
        {
            this.articleStore = articleStore;
            this.commentStore = commentStore;
            this.commentAuthorService = commentAuthorService;

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
        [Produces("application/json")]
        public async Task<IActionResult> GetComments([FromQuery] string slug)
        {
            return Json(await commentStore.TreeForSlug(slug));
        }

        [HttpGet]
        [Route("{commentId}")]
        [Produces("application/json")]
        public async Task<IActionResult> GetComment(string commentId)
        {
            var comment = await commentStore.GetById(commentId, getChildren: true);

            if (comment == null)
            {
                return NotFound();
            }

            return Json(comment);
        }

        [HttpPost]
        [Consumes("application/json")]
        public async Task<IActionResult> SubmitComment(
            [FromQuery] string slug,
            [FromBody] SubmitCommentRequest request
        )
        {
            var article = articleStore.GetBySlug(slug);

            if (article == null)
            {
                return NotFound();
            }

            if (!ModelState.IsValid)
            {
                return BadRequest(new
                {
                    error = string.Join("; ", ModelState.Values
                    .SelectMany(v => v.Errors)
                    .Select(e => e.ErrorMessage))
                });
            }

            request.Text = htmlSanitizer.Sanitize(request.Text);

            await commentStore.Submit(article.Slug, request);

            commentAuthorService.Set(new CommentAuthor
            {
                Name = request.Author,
                Email = request.Email,
                Website = request.Website,
            });

            if (Request.IsHtmx())
            {
                return View("ArticleComments", article);
            }

            return NoContent();
        }

        [HttpPost]
        [Route("{commentId}/upvote")]
        public async Task<IActionResult> UpvoteComment(string commentId)
        {
            var comment = await commentStore.GetById(commentId);

            if (comment == null)
            {
                return NotFound();
            }

            if (await commentStore.Upvote(commentId, Request.HttpContext.Connection.RemoteIpAddress))
            {
                return NoContent();
            }
            else
            {
                return BadRequest();
            }
        }

        [HttpPost]
        [Route("{commentId}/downvote")]
        public async Task<IActionResult> DownvoteComment(string commentId)
        {
            var comment = await commentStore.GetById(commentId);

            if (comment == null)
            {
                return NotFound();
            }

            if (await commentStore.Downvote(commentId, Request.HttpContext.Connection.RemoteIpAddress))
            {
                return NoContent();
            }
            else
            {
                return BadRequest();
            }
        }
    }
}
