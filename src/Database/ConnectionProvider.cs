using System.Data.Common;
using System.Threading.Tasks;
using Microsoft.Extensions.Configuration;
using MySql.Data.MySqlClient;

namespace Blog
{
    public class ConnectionProvider
    {
        private readonly string connectionString;

        public ConnectionProvider(IConfiguration configuration)
        {
            connectionString = configuration["ConnectionString"];
        }

        public async Task<DbConnection> Connect()
        {
            var connection = new MySqlConnection(connectionString);
            await connection.OpenAsync();

            return connection;
        }
    }
}
