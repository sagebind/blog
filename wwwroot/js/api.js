import m from "https://cdn.skypack.dev/mithril@2";

export async function getComments(articleSlug) {
    await delay(10000);

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
    await delay(500);

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

function delay(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}
