using Nett;
using System;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Text.RegularExpressions;
using System.Collections.Generic;
using Markdig;

namespace Blog
{
    public class ArticleStore
    {
        private static readonly Assembly assembly = Assembly.GetEntryAssembly();
        private static readonly Regex slugRegex = new Regex(@"^(\d{4})-(\d{2})-(\d{2})-");

        private Dictionary<string, Article> cache = new Dictionary<string, Article>();
        private MarkdownPipeline markdownPipeline;

        public ArticleStore(MarkdownPipeline markdownPipeline)
        {
            this.markdownPipeline = markdownPipeline;
        }

        public IEnumerable<Article> GetAll(bool includeUnpublished = false)
        {
            return assembly
                .GetManifestResourceNames()
                .Where(name => name.StartsWith("blog.articles."))
                .Select(LoadArticleFromResource)
                .Where(article => includeUnpublished || article.IsPublished);
        }

        public Article GetBySlug(string slug)
        {
            return GetAll()
                .Where(article => article.Slug == slug)
                .FirstOrDefault(x => true);
        }

        public IEnumerable<Article> GetByTag(string tag)
        {
            return GetAll().Where(article => article.Tags.Contains(tag));
        }

        private Article LoadArticleFromResource(string name)
        {
            if (cache.ContainsKey(name))
            {
                return cache[name];
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

                string slug = slugRegex.Replace(name.Split(".").TakeLast(2).First(), "$1/$2/$3/");

                var article = new Article
                {
                    Slug = slug,
                    Metadata = metadata,
                    Html = Markdown.ToHtml(source, markdownPipeline),
                    Text = Markdown.ToPlainText(source, markdownPipeline),
                };
                cache[name] = article;

                return article;
            }
        }
    }
}
