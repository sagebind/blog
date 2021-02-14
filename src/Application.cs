using Markdig;
using Microsoft.AspNetCore;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.Caching.Memory;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Westwind.AspNetCore.Markdown;

namespace Blog
{
    public class Application
    {
        public static void Main(string[] args)
        {
            WebHost.CreateDefaultBuilder(args)
                .UseStartup<Startup>()
                .Build()
                .Run();
        }
    }

    public class Startup
    {
        public static string GitCommit => ThisAssembly.Git.Commit;

        public virtual void ConfigureServices(IServiceCollection services)
        {
            services.AddHttpContextAccessor();
            services.AddMarkdown(config =>
            {
                var folderConfig = config.AddMarkdownProcessingFolder("/", "src/Views/__MarkdownPageTemplate.cshtml");
                folderConfig.ProcessExtensionlessUrls = true;
            });
            services.AddRazorPages();
            services.AddMvc()
                .AddRazorOptions(options =>
                {
                    options.ViewLocationFormats.Add("/src/Views/{0}.cshtml");
                });

            services.AddSingleton<ApiTokenService>();
            services.AddSingleton<ArticleStore>();
            services.AddScoped<CommentStore>();
            services.AddScoped<ConnectionProvider>();
            services.AddSingleton<MarkdownPipeline>(new MarkdownPipelineBuilder()
                .UseAutoIdentifiers()
                .UseAutoLinks()
                .UseFootnotes()
                .UsePipeTables()
                .UseSmartyPants()
                .Build());
        }

        public virtual void Configure(IApplicationBuilder app)
        {
            app.UseStatusCodePagesWithReExecute("/error/{0}");
            app.UseDefaultFiles();
            app.UseMarkdown();
            app.UseStaticFiles();
            app.UseRouting();

            app.UseEndpoints(endpoints => {
                endpoints.MapControllers();
                endpoints.MapRazorPages();
                endpoints.MapDefaultControllerRoute();
            });
        }
    }

    public class ProdStartup : Startup
    {
        public override void ConfigureServices(IServiceCollection services)
        {
            base.ConfigureServices(services);

            services.AddSingleton<IMemoryCache>(new MemoryCache(new MemoryCacheOptions()));
        }

        public override void Configure(IApplicationBuilder app)
        {
            app.UseHsts();

            base.Configure(app);
        }
    }

    public class DevelopmentStartup : Startup
    {
        public override void ConfigureServices(IServiceCollection services)
        {
            base.ConfigureServices(services);

            services.AddSingleton<IMemoryCache, NoOpCache>();
        }

        public override void Configure(IApplicationBuilder app)
        {
            app.UseDeveloperExceptionPage();

            base.Configure(app);
        }
    }
}
