using Microsoft.AspNetCore.Http;

namespace Blog
{
    public static class HtmxExtensions
    {
        public static bool IsHtmx(this HttpRequest request)
        {
            if (request.Headers.ContainsKey("HX-Request"))
            {
                return request.Headers["HX-Request"] == "true";
            }

            return false;
        }
    }
}
