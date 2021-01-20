using System;
using System.Collections.Generic;
using System.Data.Common;
using System.Net;
using System.Threading.Tasks;
using HashidsNet;
using Markdig;
using Microsoft.Extensions.Configuration;

namespace Blog
{
    /// <summary>
    /// Manages storage for article comments.
    /// </summary>
    public class CommentStore
    {
        // To prevent excessive spam we limit the total number of votes any one comment can receive.
        private const int maxVotes = 500;

        private readonly Hashids hashids;
        private readonly ConnectionProvider connectionProvider;
        private readonly MarkdownPipeline markdownPipeline;

        public CommentStore(
            IConfiguration configuration,
            ConnectionProvider connectionProvider,
            MarkdownPipeline markdownPipeline
        )
        {
            hashids = new Hashids(configuration["IdSalt"], 5);
            this.connectionProvider = connectionProvider;
            this.markdownPipeline = markdownPipeline;
        }

        /// <summary>
        /// Gets all comments for the given article slug as a tree.
        /// </summary>
        public async Task<IEnumerable<Comment>> TreeForSlug(string slug)
        {
            var comments = new Dictionary<string, Comment>();
            var rootComments = new List<Comment>();

            // Load all comments for this article from the database first.
            using (var connection = await connectionProvider.Connect())
            {
                using (var command = connection.CreateCommand(@"
                    SELECT * FROM CommentWithScore
                    WHERE slug = @slug
                        AND dateDeleted IS NULL
                "))
                {
                    command.AddParameter("@slug", slug);

                    using (var reader = await command.ExecuteReaderAsync())
                    {
                        while (await reader.ReadAsync())
                        {
                            var comment = GetComment(reader);
                            comments[comment.Id] = comment;

                            if (comment.ParentId == null)
                            {
                                rootComments.Add(comment);
                            }
                        }
                    }
                }
            }

            // Build the tree.
            foreach (var comment in comments.Values)
            {
                if (comment.ParentId != null)
                {
                    comments[comment.ParentId].Children.Add(comment);
                }
            }

            return rootComments;
        }

        /// <summary>
        /// Gets all top-level comments for the given article slug.
        /// </summary>
        public async IAsyncEnumerable<Comment> ForSlug(string slug)
        {
            using (var connection = await connectionProvider.Connect())
            {
                using (var command = connection.CreateCommand(@"
                    SELECT * FROM CommentWithScore
                    WHERE slug = @slug
                        AND dateDeleted IS NULL
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
        }

        public async Task<Comment> GetById(string id, bool getChildren = false)
        {
            using (var connection = await connectionProvider.Connect())
            {
                Comment comment = null;

                using (var command = connection.CreateCommand(@"
                    SELECT * FROM CommentWithScore
                    WHERE id = @id
                        AND dateDeleted IS NULL
                "))
                {
                    command.AddParameter("@id", DecodeId(id));

                    using (var reader = await command.ExecuteReaderAsync())
                    {
                        if (await reader.ReadAsync())
                        {
                            comment = GetComment(reader);
                        }
                    }
                }

                if (getChildren)
                {
                    await foreach (var child in GetChildrenById(connection, id))
                    {
                        comment.Children.Add(child);
                    }
                }

                return comment;
            }
        }

        public async IAsyncEnumerable<Comment> GetChildrenById(string id)
        {
            using (var connection = await connectionProvider.Connect())
            {
                await foreach (var item in GetChildrenById(connection, id))
                {
                    yield return item;
                }
            }
        }

        private async IAsyncEnumerable<Comment> GetChildrenById(DbConnection connection, string id, bool getChildren = false)
        {
            using (var command = connection.CreateCommand(@"
                SELECT * FROM CommentWithScore
                WHERE parentId = @id
                    AND dateDeleted IS NULL
            "))
            {
                command.AddParameter("@id", DecodeId(id));

                using (var reader = await command.ExecuteReaderAsync())
                {
                    while (await reader.ReadAsync())
                    {
                        var comment = GetComment(reader);

                        if (getChildren)
                        {
                            await foreach (var child in GetChildrenById(connection, comment.Id))
                            {
                                comment.Children.Add(child);
                            }
                        }

                        yield return comment;
                    }
                }
            }
        }

        public async Task Submit(string slug, SubmitCommentRequest request)
        {
            using (var connection = await connectionProvider.Connect())
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
                    command.AddParameter("@parentId", request.ParentCommentId != null ? DecodeId(request.ParentCommentId) : null);
                    command.AddParameter("@slug", slug);
                    command.AddParameter("@now", (DateTime.UtcNow - DateTime.UnixEpoch).TotalSeconds);
                    command.AddParameter("@authorName", request.Author);
                    command.AddParameter("@authorEmail", request.Email);
                    command.AddParameter("@authorWebsite", request.Website);
                    command.AddParameter("@text", request.Text);

                    await command.ExecuteNonQueryAsync();
                }
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
            using (var connection = await connectionProvider.Connect())
            {
                using (var command = connection.CreateCommand(@"
                    REPLACE INTO Vote (commentId, voterIp, vote) VALUES (@id, @address, @vote)
                "))
                {
                    command.AddParameter("@id", DecodeId(id));
                    command.AddParameter("@address", address.ToString());
                    command.AddParameter("@vote", vote);

                    await command.ExecuteNonQueryAsync();
                }
            }

            return true;
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
                Score = (int)reader.Get<Decimal>("score"),
                Text = reader.Get<string>("text"),
                Html = Markdown.ToHtml(reader.Get<string>("text"), markdownPipeline),
            };
        }

        private string EncodeId(long id)
        {
            return hashids.EncodeLong(id);
        }

        private long? DecodeId(string id)
        {
            return hashids.DecodeLong(id)[0];
        }
    }
}
