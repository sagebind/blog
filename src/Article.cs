using System;
using Nett;

namespace Blog
{
    public class Article
    {
        public string Slug { get; set; }

        public TomlTable Metadata { get; set; } = Toml.Create();

        public string Html { get; set; }

        public string Text { get; set; }

        public string Title => Metadata.Get<string>("title");

        public string Author => Metadata.Get<string>("author");

        public string[] Tags => Metadata.TryGetValue("tags")?.Get<string[]>() ?? new string[] {};

        public Uri CanonicalUri => new Uri("http://stephencoakley.com/" + Slug);

        public DateTime Date
        {
            get
            {
                DateTime local = DateTime.Parse(Metadata.Get<string>("date"));
                return TimeZoneInfo.ConvertTimeToUtc(local, TimeZoneInfo.FindSystemTimeZoneById("America/Chicago"));
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
    }
}
