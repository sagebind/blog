
using System;
using System.Text.Json;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.Hosting;

namespace Blog
{
    public class CommentAuthorService
    {
        private const string cookieName = "comment-author";
        private readonly IHttpContextAccessor httpContextAccessor;
        private readonly IWebHostEnvironment environment;

        public CommentAuthorService(
            IHttpContextAccessor httpContextAccessor,
            IWebHostEnvironment environment
        )
        {
            this.httpContextAccessor = httpContextAccessor;
            this.environment = environment;
        }

        public CommentAuthor Get()
        {
            if (Context.Request.Cookies.TryGetValue(cookieName, out string cookie))
            {
                Context.Items[typeof(CommentAuthor)] = JsonSerializer.Deserialize<CommentAuthor>(cookie);
            }

            return (CommentAuthor)Context.Items[typeof(CommentAuthor)];
        }

        public void Set(CommentAuthor commentAuthor)
        {
            if (commentAuthor != null)
            {
                Context.Items[typeof(CommentAuthor)] = commentAuthor;

                Context.Response.Cookies.Append(
                    cookieName,
                    JsonSerializer.Serialize(commentAuthor),
                    new CookieOptions {
                        HttpOnly = true,
                        MaxAge = TimeSpan.FromDays(999),
                        SameSite = SameSiteMode.Strict,
                        Secure = !environment.IsDevelopment(),
                    }
                );
            }
        }

        private HttpContext Context => httpContextAccessor.HttpContext;
    }
}
