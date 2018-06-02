using Microsoft.AspNetCore.Mvc;
using System;
using System.Linq;
using System.Text;

namespace Blog.Controllers
{
    public class Errors : Controller
    {
        private static Random random = new Random();

        [Route("/error/404")]
        public IActionResult HandleNotFound()
        {
            var s = new StringBuilder();

            foreach (var _ in Enumerable.Range(0, 1000))
            {
                s.Append((char) random.Next(32, 126));
            }

            return View("404", s.ToString());
        }
    }
}
