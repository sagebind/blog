using Nett;
using System;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Text.RegularExpressions;
using System.Collections.Generic;
using Markdig;
using Microsoft.Extensions.Caching.Memory;
using Markdig.Renderers;

namespace Blog
{
    public class ArticleStore
    {
        private static readonly Assembly assembly = Assembly.GetEntryAssembly();
        private static readonly Regex slugRegex = new Regex(@"^blog\.articles\.(\d{4})-(\d{2})-(\d{2})-(.+)\.md$");

        private IMemoryCache articleCache;
        private MarkdownPipeline markdownPipeline;

        public ArticleStore(IMemoryCache articleCache, MarkdownPipeline markdownPipeline)
        {
            this.articleCache = articleCache;
            this.markdownPipeline = markdownPipeline;
        }

        public IEnumerable<Article> GetAll(bool includeUnpublished = false)
        {
            return assembly
                .GetManifestResourceNames()
                .Where(name => name.StartsWith("blog.articles."))
                .Select(LoadArticleFromResource)
                .Where(article => includeUnpublished || article.IsPublished)
                .OrderByDescending(article => article.Date);
        }

        public Article GetBySlug(string slug)
        {
            return GetAll(true)
                .Where(article => article.Slug == slug)
                .FirstOrDefault(x => true);
        }

        public IEnumerable<Article> GetByTag(string tag)
        {
            return GetAll().Where(article => article.Tags.Contains(tag));
        }

        private Article LoadArticleFromResource(string name)
        {
            var article = articleCache.Get<Article>(name);

            if (article != null)
            {
                return article;
            }

            using (StreamReader reader = new StreamReader(assembly.GetManifestResourceStream(name), Encoding.UTF8))
            {
                string source = reader.ReadToEnd();

                TomlTable metadata = null;

                // If a TOML front matter block is given, parse the contained metadata.
                if (source.StartsWith("+++"))
                {
                    source = source.Substring(3);
                    int endPos = source.IndexOf("+++");
                    string frontMatter = source.Substring(0, endPos).Trim();
                    source = source.Substring(endPos + 3).Trim();

                    metadata = Toml.ReadString(frontMatter);
                }

                string slug = slugRegex.Replace(name, "$1/$2/$3/$4");

                article = new Article
                {
                    Slug = slug,
                    Metadata = metadata,
                    Html = Markdown.ToHtml(source, markdownPipeline),
                    Text = RenderPlainText(source),
                };

                articleCache.CreateEntry(name).Value = article;

                return article;
            }
        }

        private string RenderPlainText(string markdown)
        {
            var writer = new StringWriter();
            var renderer = new HtmlRenderer(writer)
            {
                EnableHtmlForBlock = false,
                EnableHtmlForInline = false,
                EnableHtmlEscape = false
            };

            Markdown.Convert(markdown, renderer, markdownPipeline);

            return writer.ToString();
        }
    }
}
