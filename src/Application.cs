using System;
using System.Reflection;
using Autofac;
using Autofac.Extensions.DependencyInjection;
using Markdig;
using Microsoft.AspNetCore;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.FileProviders;

namespace Blog
{
    public class Application : Autofac.Module
    {
        public static void Main(string[] args)
        {
            WebHost.CreateDefaultBuilder(args)
                .ConfigureServices(services => services.AddAutofac())
                .UseStartup<Startup>()
                .Build()
                .Run();
        }

        protected override void Load(ContainerBuilder builder)
        {
            builder.RegisterType<ArticleStore>().AsSelf();
            builder.RegisterInstance(new MarkdownPipelineBuilder()
                .UseAutoIdentifiers()
                .UseAutoLinks()
                .UseFootnotes()
                .UsePipeTables()
                .UseSmartyPants()
                .Build())
                .As<MarkdownPipeline>();
        }
    }

    public class Startup
    {
        public void ConfigureServices(IServiceCollection services)
        {
            services.AddMvc()
                .SetCompatibilityVersion(CompatibilityVersion.Version_2_1)
                .AddRazorOptions(options =>
                {
                    options.ViewLocationFormats.Add("/src/Views/{0}.cshtml");
                });
        }

        public void ConfigureContainer(ContainerBuilder builder)
        {
            builder.RegisterModule<Application>();
        }

        public void Configure(IApplicationBuilder app, IHostingEnvironment env)
        {
            if (env.IsDevelopment())
            {
                app.UseDeveloperExceptionPage();
            }
            else
            {
                app.UseHsts();
            }

            app.UseStatusCodePagesWithReExecute("/error/{0}");
            app.UseDefaultFiles();
            app.UseStaticFiles();
            app.UseMvc();
        }
    }
}
