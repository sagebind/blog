using System;

namespace Blog
{
    public class CommentAuthor
    {
        /// <summary>
        /// The name the comment author supplied.
        /// </summary>
        public string Name { get; set; }

        /// <summary>
        /// The email address the comment author supplied.
        /// </summary>
        public string Email { get; set; }

        /// <summary>
        /// The website the comment author supplied.
        /// </summary>
        public string Website { get; set; }

        public Uri Avatar => Gravatar.ImageForEmail(Email);
    }
}
