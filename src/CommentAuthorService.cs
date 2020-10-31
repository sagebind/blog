
using System.Text.Json;
using Microsoft.AspNetCore.Http;

namespace Blog
{
    public class CommentAuthorService
    {
        private const string cookieName = "comment-author";
        private readonly IHttpContextAccessor httpContextAccessor;

        public CommentAuthorService(IHttpContextAccessor httpContextAccessor)
        {
            this.httpContextAccessor = httpContextAccessor;
        }

        public CommentAuthor Get()
        {
            if (httpContextAccessor.HttpContext.Request.Cookies.TryGetValue(cookieName, out string cookie))
            {
                return JsonSerializer.Deserialize<CommentAuthor>(cookie);
            }

            return null;
        }
    }
}
