using System;
using System.Data;
using System.Data.Common;
using System.Threading.Tasks;

namespace Blog
{
    /// <summary>
    /// Extensions to the base ADO.NET interface which is pretty bland.
    /// </summary>
    public static class DbExtensions
    {
        public static DbCommand CreateCommand(this DbConnection connection, string query)
        {
            var command = connection.CreateCommand();
            command.CommandText = query;

            return command;
        }

        public static void AddParameter(this IDbCommand command, string name, object value)
        {
            var parameter = command.CreateParameter();
            parameter.ParameterName = name;
            parameter.Value = value;
            command.Parameters.Add(parameter);
        }

        public static async Task<T> ExecuteScalarAsync<T>(this DbCommand command)
        {
            object result = await command.ExecuteScalarAsync();

            if (result == null || result == DBNull.Value)
            {
                return default(T);
            }

            return (T)result;
        }

        public static T Get<T>(this DbDataReader reader, string name)
        {
            int ordinal = reader.GetOrdinal(name);

            if (reader.IsDBNull(ordinal))
            {
                return default(T);
            }

            return reader.GetFieldValue<T>(ordinal);
        }
    }
}
