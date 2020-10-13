using System;
using System.Collections.Generic;
using System.Data.Common;
using System.IO;
using System.Net;
using System.Threading.Tasks;
using HashidsNet;
using Microsoft.Data.Sqlite;
using Microsoft.Extensions.Configuration;

namespace Blog
{
    /// <summary>
    /// Manages storage for article comments.
    /// </summary>
    public class CommentStore : IDisposable
    {
        // To prevent excessive spam we limit the total number of votes any one comment can receive.
        private const int maxVotes = 500;

        private readonly Hashids hashids;
        private readonly SqliteConnection connection;

        public CommentStore(IConfiguration configuration)
        {
            hashids = new Hashids(configuration["IdSalt"]);

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
                SELECT * FROM CommentWithScore
                WHERE slug = @slug
                    AND parentId IS NULL
            "))
            {
                command.AddParameter("@slug", slug);

                using (var reader = await command.ExecuteReaderAsync())
                {
                    while (await reader.ReadAsync())
                    {
                        yield return GetComment(reader);
                    }
                }
            }
        }

        public async Task<Comment> GetById(string id)
        {
            using (var command = connection.CreateCommand(@"
                SELECT * FROM CommentWithScore
                WHERE id = @id
            "))
            {
                command.AddParameter("@id", DecodeId(id));

                using (var reader = await command.ExecuteReaderAsync())
                {
                    if (await reader.ReadAsync())
                    {
                        return GetComment(reader);
                    }
                    else
                    {
                        return null;
                    }
                }
            }
        }

        public async IAsyncEnumerable<Comment> GetChildrenById(string id)
        {
            using (var command = connection.CreateCommand(@"
                SELECT * FROM CommentWithScore
                WHERE parentId = @id
            "))
            {
                command.AddParameter("@id", DecodeId(id));

                using (var reader = await command.ExecuteReaderAsync())
                {
                    while (await reader.ReadAsync())
                    {
                        yield return GetComment(reader);
                    }
                }
            }
        }

        public async Task Submit(string slug, SubmitCommentRequest request)
        {
            using (var command = connection.CreateCommand(@"
                INSERT INTO Comment (
                    parentId,
                    slug,
                    datePublished,
                    authorName,
                    authorEmail,
                    authorWebsite,
                    text
                ) VALUES (
                    @parentId,
                    @slug,
                    @now,
                    @authorName,
                    @authorEmail,
                    @authorWebsite,
                    @text
                )
            "))
            {
                command.AddParameter("@parentId", DecodeId(request.ParentCommentId));
                command.AddParameter("@slug", slug);
                command.AddParameter("@now", (DateTime.UtcNow - DateTime.UnixEpoch).TotalSeconds);
                command.AddParameter("@authorName", request.Author);
                command.AddParameter("@authorEmail", request.Email);
                command.AddParameter("@authorWebsite", request.Website);
                command.AddParameter("@text", request.Text);

                await command.ExecuteNonQueryAsync();
            }
        }

        public async Task<bool> Upvote(string id, IPAddress address)
        {
            return await Vote(id, address, 1);
        }

        public async Task<bool> Downvote(string id, IPAddress address)
        {
            return await Vote(id, address, -1);
        }

        private async Task<bool> Vote(string id, IPAddress address, int vote)
        {
            using (var command = connection.CreateCommand(@"
                INSERT OR REPLACE INTO Vote (commentId, voterIp, vote) VALUES (@id, @address, @vote)
            "))
            {
                command.AddParameter("@id", DecodeId(id));
                command.AddParameter("@address", address.ToString());
                command.AddParameter("@vote", vote);

                await command.ExecuteNonQueryAsync();
            }

            return true;
        }

        public void Dispose()
        {
            connection?.Dispose();
        }

        private Comment GetComment(DbDataReader reader)
        {
            return new Comment
            {
                Id = EncodeId(reader.Get<long>("id")),
                ParentId = reader.Get<long?>("parentId") is long parentId ? EncodeId(parentId) : null,
                ArticleSlug = reader.Get<string>("slug"),
                Published = DateTimeOffset.UnixEpoch.AddSeconds(reader.Get<double>("datePublished")),
                Author = new CommentAuthor
                {
                    Name = reader.Get<string>("authorName"),
                    Email = reader.Get<string>("authorEmail"),
                    Website = reader.Get<string>("authorWebsite"),
                },
                Score = reader.Get<int>("score"),
                Text = reader.Get<string>("text"),
            };
        }

        private string EncodeId(long id)
        {
            return hashids.EncodeLong(id);
        }

        private long DecodeId(string id)
        {
            return hashids.DecodeLong(id)[0];
        }
    }
}
