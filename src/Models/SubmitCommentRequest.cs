using System.ComponentModel.DataAnnotations;

namespace Blog
{
    public class SubmitCommentRequest
    {
        [Required]
        [MinLength(3, ErrorMessage = "Comment is too short.")]
        [MaxLength(16384, ErrorMessage = "Comment is too long!")]
        public string Text { get; set; }

        [Required]
        [MaxLength(255)]
        public string Author { get; set; }

        [Required]
        [EmailAddress]
        [MaxLength(255)]
        public string Email { get; set; }

        [MaxLength(255)]
        public string Website { get; set; }

        public string ParentCommentId { get; set; }
    }
}
