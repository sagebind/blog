using System;
using System.Security.Cryptography;
using System.Text;

namespace Blog
{
    public static class Gravatar
    {
        public static Uri ImageForEmail(string email)
        {
            using (MD5 md5 = MD5.Create())
            {
                string id = BitConverter.ToString(md5.ComputeHash(Encoding.UTF8.GetBytes(email.ToLower()))).Replace("-", "").ToLower();

                return new Uri($"https://www.gravatar.com/avatar/{id}?d=identicon");
            }
        }
    }
}
