using System;
using System.Collections.Generic;
using System.Data.Common;
using System.IO;
using System.Threading.Tasks;
using Microsoft.Data.Sqlite;
using Microsoft.Extensions.Configuration;

namespace Blog
{
    /// <summary>
    /// Manages storage for article comments.
    /// </summary>
    public class CommentStore : IDisposable
    {
        private readonly SqliteConnection connection;

        public CommentStore(IConfiguration configuration)
        {
            string path = configuration["CommentsPath"];
            string dir = Path.GetDirectoryName(path);

            if (!String.IsNullOrEmpty(dir))
            {
                Directory.CreateDirectory(dir);
            }

            connection = new SqliteConnection(new SqliteConnectionStringBuilder
            {
                DataSource = path
            }.ToString());

            connection.Open();
        }

        /// <summary>
        /// Gets all top-level comments for the given article slug.
        /// </summary>
        public async IAsyncEnumerable<Comment> ForSlug(string slug)
        {
            using (var command = connection.CreateCommand(@"
                SELECT c.id, c.parent, c.created, c.text, c.author, c.email, c.website
                FROM comments c
                JOIN threads t ON t.id = c.tid
                WHERE t.uri = @uri
                    AND c.parent IS NULL
                ORDER BY c.created ASC
            "))
            {
                command.AddParameter("@uri", $"/{slug}");

                using (var reader = await command.ExecuteReaderAsync())
                {
                    while (await reader.ReadAsync())
                    {
                        yield return GetComment(reader);
                    }
                }
            }
        }

        public async Task<Comment> GetById(long id)
        {
            using (var command = connection.CreateCommand(@"
                SELECT id, parent, created, text, author, email, website
                FROM comments
                WHERE id = @id
            "))
            {
                command.AddParameter("@id", id);

                using (var reader = await command.ExecuteReaderAsync())
                {
                    return GetComment(reader);
                }
            }
        }

        public async IAsyncEnumerable<Comment> GetChildrenById(long id)
        {
            using (var command = connection.CreateCommand(@"
                SELECT id, parent, created, text, author, email, website
                FROM comments
                WHERE parent = @id
                ORDER BY created ASC
            "))
            {
                command.AddParameter("@id", id);

                using (var reader = await command.ExecuteReaderAsync())
                {
                    while (await reader.ReadAsync())
                    {
                        yield return GetComment(reader);
                    }
                }
            }
        }

        public void Dispose()
        {
            connection?.Dispose();
        }

        private Comment GetComment(DbDataReader reader)
        {
            return new Comment
            {
                Id = reader.Get<long>("id"),
                ParentId = reader.Get<long?>("parent"),
                Published = DateTimeOffset.UnixEpoch.AddSeconds(reader.Get<double>("created")),
                Author = reader.Get<string>("author"),
                Email = reader.Get<string>("email"),
                Website = reader.Get<string>("website"),
                Text = reader.Get<string>("text"),
            };
        }
    }
}
