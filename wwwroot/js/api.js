import { m } from "./deps.js";

export class Client {
    constructor(token) {
        this.authHeader = "Bearer " + token;
    }

    async getComments(articleSlug) {
        return await m.request({
            method: "GET",
            url: "/api/comments",
            params: {
                slug: articleSlug
            },
            headers: {
                Authorization: this.authHeader
            }
        });
    }

    async getComment(id) {
        return await m.request({
            method: "GET",
            url: `/api/comments/${id}`,
            headers: {
                Authorization: this.authHeader
            }
        });
    }

    async submitComment({
        articleSlug,
        name,
        email,
        website,
        text,
        parentCommentId,
    }) {
        return await m.request({
            method: "POST",
            url: "/api/comments",
            params: {
                slug: articleSlug
            },
            headers: {
                Authorization: this.authHeader
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

    async upvoteComment(id) {
        return await m.request({
            method: "POST",
            url: `/api/comments/${id}/upvote`,
            headers: {
                Authorization: this.authHeader
            },
            extract: xhr => xhr.status === 204
        });
    }

    async downvoteComment(id) {
        return await m.request({
            method: "POST",
            url: `/api/comments/${id}/downvote`,
            headers: {
                Authorization: this.authHeader
            },
            extract: xhr => xhr.status === 204
        });
    }
}
