using System.Threading.Tasks;
using Ganss.XSS;
using Microsoft.AspNetCore.Mvc;

namespace Blog.Controllers
{
    public class ArticleController : Controller
    {
        private readonly ArticleStore articleStore;
        private readonly CommentStore commentStore;
        private readonly HtmlSanitizer htmlSanitizer = new HtmlSanitizer();

        public ArticleController(
            ArticleStore articleStore,
            CommentStore commentStore
        )
        {
            this.articleStore = articleStore;
            this.commentStore = commentStore;

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
        public async Task<IActionResult> SubmitComment(
            int year,
            int month,
            int day,
            string name,
            [FromForm] SubmitCommentRequest request
        )
        {
            var article = articleStore.GetBySlug($"{year:D2}/{month:D2}/{day:D2}/{name}");

            if (article == null)
            {
                return NotFound();
            }

            request.Text = htmlSanitizer.Sanitize(request.Text);

            await commentStore.Submit(article.Slug, request);

            if (Request.IsHtmx())
            {
                return View("ArticleComments", article);
            }

            return Redirect($"/{article.Slug}");
        }

        [HttpGet]
        [Route("/{year}/{month}/{day}/{name}/comments/{commentId}")]
        public async Task<IActionResult> GetComment(
            int year,
            int month,
            int day,
            string name,
            string commentId,
            [FromQuery] bool showReply
        )
        {
            var article = articleStore.GetBySlug($"{year:D2}/{month:D2}/{day:D2}/{name}");

            if (article == null)
            {
                return NotFound();
            }

            var comment = await commentStore.GetById(commentId);

            if (comment == null || comment.ArticleSlug != article.Slug)
            {
                return NotFound();
            }

            return CommentView(new CommentView
            {
                Comment = comment,
                ShowReply = showReply,
            });
        }

        [HttpPost]
        [Route("/{year}/{month}/{day}/{name}/comments/{commentId}/upvote")]
        public async Task<IActionResult> UpvoteComment(
            int year,
            int month,
            int day,
            string name,
            string commentId
        )
        {
            var article = articleStore.GetBySlug($"{year:D2}/{month:D2}/{day:D2}/{name}");

            if (article == null)
            {
                return NotFound();
            }

            var comment = await commentStore.GetById(commentId);

            if (comment == null || comment.ArticleSlug != article.Slug)
            {
                return NotFound();
            }

            await commentStore.Upvote(commentId, Request.HttpContext.Connection.RemoteIpAddress);

            return CommentView(await commentStore.GetById(commentId));
        }

        [HttpPost]
        [Route("/{year}/{month}/{day}/{name}/comments/{commentId}/downvote")]
        public async Task<IActionResult> DownvoteComment(
            int year,
            int month,
            int day,
            string name,
            string commentId
        )
        {
            var article = articleStore.GetBySlug($"{year:D2}/{month:D2}/{day:D2}/{name}");

            if (article == null)
            {
                return NotFound();
            }

            var comment = await commentStore.GetById(commentId);

            if (comment == null || comment.ArticleSlug != article.Slug)
            {
                return NotFound();
            }

            await commentStore.Downvote(commentId, Request.HttpContext.Connection.RemoteIpAddress);

            return CommentView(await commentStore.GetById(commentId));
        }

        private IActionResult CommentView(Comment comment)
        {
            return CommentView(new CommentView
            {
                Comment = comment,
            });
        }

        private IActionResult CommentView(CommentView commentView)
        {
            if (Request.IsHtmx())
            {
                return View("Comment", commentView);
            }

            return Redirect($"/{commentView.Comment.ArticleSlug}#comment-{commentView.Comment.Id}");
        }
    }
}
