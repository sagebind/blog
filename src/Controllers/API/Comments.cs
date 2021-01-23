using System;
using System.Linq;
using System.Net.Http.Headers;
using System.Threading.Tasks;
using Ganss.XSS;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.Filters;
using Microsoft.Extensions.Logging;
using Microsoft.Net.Http.Headers;

namespace Blog.Controllers
{
    [Route("/api/comments")]
    public class Comments : Controller
    {
        private readonly ApiTokenService apiTokenService;
        private readonly ArticleStore articleStore;
        private readonly CommentStore commentStore;
        private readonly ILogger logger;
        private readonly HtmlSanitizer htmlSanitizer = new HtmlSanitizer();

        public Comments(
            ApiTokenService apiTokenService,
            ArticleStore articleStore,
            CommentStore commentStore,
            ILogger<Comments> logger
        )
        {
            this.apiTokenService = apiTokenService;
            this.articleStore = articleStore;
            this.commentStore = commentStore;
            this.logger = logger;

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

            string commentId = await commentStore.Submit(article.Slug, request);

            return Json(new SubmitCommentResponse
            {
                CommentId = commentId
            });
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

        public override async Task OnActionExecutionAsync(ActionExecutingContext context, ActionExecutionDelegate next)
        {
            if (ValidateAuth(context.HttpContext))
            {
                await next();
            }
            else
            {
                context.Result = StatusCode(403, "Forbidden");
            }
        }

        private bool ValidateAuth(HttpContext context)
        {
            try
            {
                if (context.Request.Headers.ContainsKey(HeaderNames.Authorization))
                {
                    var auth = AuthenticationHeaderValue.Parse(context.Request.Headers[HeaderNames.Authorization]);

                    if (auth.Scheme.Equals("Bearer", StringComparison.OrdinalIgnoreCase))
                    {
                        if (apiTokenService.Validate(auth.Parameter))
                        {
                            return true;
                        }
                    }
                }
            }
            catch (Exception e)
            {
                logger.LogWarning(e, "Exception when validating auth.");
            }

            return false;
        }
    }
}
