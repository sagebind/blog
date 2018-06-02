using Markdig;
using Nett;
using System;

namespace Blog
{
    public class Article
    {
        public string Title => Metadata.Get<string>("title");
        public string Author => Metadata.Get<string>("author");
        public string Category => Metadata.Get<string>("category");
        public DateTime Date {
            get {
                DateTime local = DateTime.Parse(Metadata.Get<string>("date"));
                return TimeZoneInfo.ConvertTimeToUtc(local, TimeZoneInfo.FindSystemTimeZoneById("America/Chicago"));
            }
        }
        public string Slug { get; }
        public Uri CanonicalUri => new Uri("http://stephencoakley.com/" + Slug);
        public string Html { get; }
        public string Text { get; }
        public TomlTable Metadata { get; }

        public Article(string slug, string source, TomlTable metadata = null)
        {
            Slug = slug;
            Html = Markdown.ToHtml(source);
            Text = Markdown.ToPlainText(source);
            Metadata = metadata ?? Toml.Create();
        }

        public string Summarize(int length = 250)
        {
            return Text.Substring(0, Text.IndexOf(" ", length)) + "...";
        }
    }
}
