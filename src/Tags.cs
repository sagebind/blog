using Nett;
using System;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Text.RegularExpressions;
using System.Collections.Generic;
using Markdig;

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
