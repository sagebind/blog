CREATE TABLE "Comment" (
    "id" BIGINT NOT NULL UNIQUE,
    "parentId" BIGINT,
    "slug" TEXT NOT NULL,
    "datePublished" NUMERIC NOT NULL,
    "authorName" TEXT NOT NULL,
    "authorEmail" TEXT,
    "authorWebsite" TEXT,
    "text" TEXT NOT NULL,
    PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "Vote" (
    "commentId" BIGINT NOT NULL,
    "voterIp" VARCHAR(15) NOT NULL,
    "vote" INTEGER NOT NULL,
    PRIMARY KEY("commentId","voterIp")
);

CREATE VIEW "CommentWithScore" AS
SELECT
    Comment.id,
    Comment.parentId,
    Comment.slug,
    Comment.datePublished,
    Comment.authorName,
    Comment.authorEmail,
    Comment.authorWebsite,
    coalesce(sum(Vote.vote), 0) AS score,
    Comment.text
FROM Comment
LEFT JOIN Vote ON Vote.commentId = Comment.id
GROUP BY Comment.id
ORDER BY Comment.datePublished ASC;
