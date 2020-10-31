namespace Blog
{
    public class Tags
    {
        public static string Normalize(string tag)
        {
            return tag.ToLower().Replace(" ", "-");
        }
    }
}
