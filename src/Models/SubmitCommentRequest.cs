using System.ComponentModel.DataAnnotations;

namespace Blog
{
    public class SubmitCommentRequest
    {
        [Required]
        [MinLength(3)]
        public string Text { get; set; }

        [Required]
        public string Author { get; set; }

        [Required]
        [EmailAddress]
        public string Email { get; set; }

        public string Website { get; set; }
    }
}
