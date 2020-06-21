using LibSassHost;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Caching.Memory;
using System.IO;
using System.Threading.Tasks;

namespace Blog.Controllers
{
    [Route("/assets")]
    public class Assets : Controller
    {
        private readonly IWebHostEnvironment webHostEnvironment;
        private readonly IMemoryCache cache;

        public Assets(IWebHostEnvironment webHostEnvironment, IMemoryCache cache)
        {
            this.webHostEnvironment = webHostEnvironment;
            this.cache = cache;
        }

        [Route("css/style.css")]
        public async Task<IActionResult> GetStyles()
        {
            var compiled = await CompileSass();

            return this.Content(compiled.CompiledContent, "text/css");
        }

        private Task<CompilationResult> CompileSass()
        {
            string stylesDir = Path.Combine(webHostEnvironment.WebRootPath, "assets/css");
            string fullPath = Path.Combine(stylesDir, "style.scss");

            return cache.GetOrCreateAsync<CompilationResult>(fullPath, async entry =>
            {
                string src = await System.IO.File.ReadAllTextAsync(fullPath);
                return SassCompiler.Compile(src, new CompilationOptions
                {
                    IncludePaths = { stylesDir },
                    OutputStyle = OutputStyle.Compressed,
                });
            });
        }
    }
}
