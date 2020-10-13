using System;
using System.Collections.Generic;
using System.Net;

namespace Blog
{
    public class Comment
    {
        /// <summary>
        /// Unique ID of this comment.
        /// </summary>
        public string Id { get; set; }

        /// <summary>
        /// If this comment is a child of another comment, the ID of the parent
        /// comment.
        /// </summary>
        public string ParentId { get; set; }

        /// <summary>
        /// Slug of the article this comment is for.
        /// </summary>
        public string ArticleSlug { get; set; }

        /// <summary>
        /// Date and time when the comment was published.
        /// </summary>
        public DateTimeOffset Published { get; set; }

        /// <summary>
        /// The author of the comment.
        /// </summary>
        public CommentAuthor Author { get; set; }

        public IPAddress IP { get; set; }

        public int Score { get; set; }

        public IReadOnlyCollection<IPAddress> Voters { get; set; }

        /// <summary>
        /// The text of the comment in Markdown format.
        /// </summary>
        public string Text { get; set; }
    }
}
