using System;
using System.Security.Cryptography;
using System.Text;

namespace Blog
{
    public class Comment
    {
        /// <summary>
        /// Unique ID of this comment.
        /// </summary>
        public long Id { get; set; }

        /// <summary>
        /// If this comment is a child of another comment, the ID of the parent
        /// comment.
        /// </summary>
        public long? ParentId { get; set; }

        /// <summary>
        /// Date and time when the comment was published.
        /// </summary>
        public DateTimeOffset Published { get; set; }

        /// <summary>
        /// The name the comment author supplied.
        /// </summary>
        public string Author { get; set; }

        /// <summary>
        /// The email address the comment author supplied.
        /// </summary>
        public string Email { get; set; }

        /// <summary>
        /// The website the comment author supplied.
        /// </summary>
        public string Website { get; set; }

        /// <summary>
        /// The text of the comment in Markdown format.
        /// </summary>
        public string Text { get; set; }

        public Uri GravatarUri
        {
            get
            {
                using (MD5 md5 = MD5.Create())
                {
                    string id = BitConverter.ToString(md5.ComputeHash(Encoding.UTF8.GetBytes(Email.ToLower()))).Replace("-", "").ToLower();

                    return new Uri($"https://www.gravatar.com/avatar/{id}?d=identicon");
                }
            }
        }
    }
}
