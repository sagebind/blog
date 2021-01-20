import { m } from "./deps.js";

export async function getComments(articleSlug) {
    return await m.request({
        method: "GET",
        url: "/api/comments",
        params: {
            slug: articleSlug
        }
    });
}

export async function getComment(id) {
    return await m.request({
        method: "GET",
        url: `/api/comments/${id}`
    });
}

export async function submitComment({
    articleSlug,
    name,
    email,
    website,
    text,
    parentCommentId,
}) {
    await m.request({
        method: "POST",
        url: "/api/comments",
        params: {
            slug: articleSlug
        },
        body: {
            author: name,
            email,
            website,
            text,
            parentCommentId,
        }
    });
}

export async function upvoteComment(id) {
    return await m.request({
        method: "POST",
        url: `/api/comments/${id}/upvote`,
        extract: xhr => xhr.status === 204
    });
}

export async function downvoteComment(id) {
    return await m.request({
        method: "POST",
        url: `/api/comments/${id}/downvote`,
        extract: xhr => xhr.status === 204
    });
}
