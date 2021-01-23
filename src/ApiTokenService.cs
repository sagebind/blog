using System;
using System.Text.Json;
using Microsoft.Extensions.Configuration;
using CryptHash.Net.Encryption.AES.AEAD;

namespace Blog
{
    public class ApiTokenService
    {
        private readonly string secretKey;
        private readonly AEAD_AES_256_GCM aes = new AEAD_AES_256_GCM();

        public ApiTokenService(IConfiguration configuration)
        {
            this.secretKey = configuration["ApiTokenSecretKey"];
        }

        public string Generate()
        {
            var tokenStruct = new ApiToken
            {
                expires = DateTimeOffset.UtcNow.AddHours(1)
            };

            var result = aes.EncryptString(JsonSerializer.Serialize(tokenStruct), secretKey);

            if (!result.Success)
            {
                throw new ApplicationException();
            }

            return result.EncryptedDataBase64String;
        }

        public bool Validate(string token)
        {
            try
            {
                var result = aes.DecryptString(token, secretKey);

                if (result.Success)
                {
                    var tokenStruct = JsonSerializer.Deserialize<ApiToken>(result.DecryptedDataString);

                    return tokenStruct.expires <= DateTimeOffset.UtcNow;
                }
            }
            catch (Exception)
            {
                // ignore
            }

            return false;
        }

        internal struct ApiToken
        {
            internal DateTimeOffset expires;
        }
    }
}
