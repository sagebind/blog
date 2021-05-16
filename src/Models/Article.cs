using System;
using System.Text.RegularExpressions;
using Nett;

namespace Blog
{
    public class Article
    {
        private static readonly Regex wordRegex = new Regex(@"\b[\w']+\b");

        public string Slug { get; set; }

        public TomlTable Metadata { get; set; } = Toml.Create();

        public string Html { get; set; }

        public string Text { get; set; }

        public string Title => Metadata.Get<string>("title");

        public string Author => Metadata.Get<string>("author");

        public string[] Tags => Metadata.TryGetValue("tags")?.Get<string[]>() ?? new string[] { };

        public bool IsOutdated => Metadata.TryGetValue("outdated")?.Get<bool>() ?? false;

        public Uri CanonicalUri => new Uri("https://stephencoakley.com/" + Slug);

        public DateTimeOffset Date
        {
            get
            {
                DateTime local = DateTime.Parse(Metadata.Get<string>("date"));
                return new DateTimeOffset(local, TimeZoneInfo.FindSystemTimeZoneById("America/Chicago").GetUtcOffset(local));
            }
        }

        public bool IsPublished
        {
            get
            {
                if (Metadata == null || Metadata.ContainsKey("unpublished") && Metadata.Get<bool>("unpublished"))
                {
                    return false;
                }

                return Date.Date <= DateTime.Now.Date;
            }
        }

        public string Summarize(int length = 250)
        {
            return Text.Substring(0, Text.IndexOf(" ", length)) + "...";
        }

        public TimeSpan EstimatedReadingTime => TimeSpan.FromMinutes(WordCount / 200);

        public int WordCount => wordRegex.Matches(Text).Count;
    }
}
