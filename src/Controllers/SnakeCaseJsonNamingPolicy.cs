using System;
using System.Text;
using System.Text.Json;

namespace Blog
{
    public class SnakeCaseJsonNamingPolicy : JsonNamingPolicy
    {
        public override string ConvertName(string name)
        {
            if (name == null)
            {
                throw new ArgumentNullException(nameof(name));
            }

            var result = new StringBuilder();

            for (var i = 0; i < name.Length; i++)
            {
                var c = name[i];
                if (i == 0)
                {
                    result.Append(char.ToLower(c));
                }
                else
                {
                    if (char.IsUpper(c))
                    {
                        result.Append('_');
                        result.Append(char.ToLower(c));
                    }
                    else
                    {
                        result.Append(c);
                    }
                }
            }

            return result.ToString();
        }
    }
}
