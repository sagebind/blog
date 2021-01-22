import { m, html } from "./deps.js";
import { Client } from "./api.js";
import Loadable from "./loadable.js";

const AuthorLocalStorage = {
    get author() {
        let commentAuthor = localStorage.getItem("commentAuthor");

        if (commentAuthor) {
            return JSON.parse(commentAuthor);
        }

        // Migrate Isso storage.
        let author = localStorage.getItem("author");

        if (author) {
            commentAuthor = {
                name: JSON.parse(author),
                email: JSON.parse(localStorage.getItem("email")),
                website: JSON.parse(localStorage.getItem("website")),
            };

            this.author = commentAuthor;

            return commentAuthor;
        }

        return {};
    },

    set author(author) {
        localStorage.setItem("commentAuthor", JSON.stringify(author));
    }
};

export class CommentsSection {
    constructor() {
        this.comments = [];
        this.loading = true;
    }

    async refreshComments() {
        this.comments = await this.apiClient.getComments(this.articleSlug);
    }

    async oninit(vnode) {
        this.articleSlug = vnode.attrs.articleSlug;
        this.apiClient = new Client(vnode.attrs.apiToken);

        try {
            await this.refreshComments();
        } finally {
            this.loading = false;
        }
    }

    onupdate() {
        // Permalinks won't work on initial page load since the element ID isn't
        // in the initial DOM. Fix this by scrolling to the comment dynamically
        // if a permalink hash is found.
        if (!this.loading && !this.checkedPermalink) {
            this.checkedPermalink = true;

            if (/^#comment-\w+$/.test(window.location.hash)) {
                let element = document.querySelector(window.location.hash);

                if (element) {
                    scrollIntoViewIfNeeded(element);
                }
            }
        }
    }

    async comment(form, parentCommentId) {
        let { commentId } = await this.apiClient.submitComment({
            articleSlug: this.articleSlug,
            name: form.name,
            email: form.email,
            website: form.website,
            text: form.text,
            parentCommentId
        });

        await this.refreshComments();

        // Scroll to new comment if found.
        setTimeout(() => {
            let element = document.querySelector("#comment-" + commentId);

            if (element) {
                scrollIntoViewIfNeeded(element);
            }
        }, 1);
    }

    view() {
        let commentsCount = this.comments.reduce(function reducer(count, comment) {
            return count + 1 + comment.children.reduce(reducer, 0);
        }, 0);

        return html`
            <h2>${commentsCount} comments</h2>

            <p>Let me know what you think in the comments below. Remember to keep it civil!</p>

            <${CommentForm} onsubmit="${form => this.comment(form)}" />

            <${Loadable} loading="${this.loading}">
                ${this.comments.map(comment => m(Comment, {
                    comment,
                    onupvote: async comment => {
                        if (await this.apiClient.upvoteComment(comment.id)) {
                            await this.refreshComments();
                        }
                    },
                    ondownvote: async comment => {
                        if (await this.apiClient.downvoteComment(comment.id)) {
                            await this.refreshComments();
                        }
                    },
                    onreply: async (comment, form) => {
                        await this.comment(form, comment.id);
                    }
                }))}
            </>
        `;
    }
}

class Comment {
    view(vnode) {
        let id = vnode.attrs.comment.id;

        return html`
            <article class="comment" id="comment-${id}">
                <div class="avatar">
                    <img src="${vnode.attrs.comment.author.avatar}" />
                </div>

                <div class="text-wrapper">
                    <div class="comment-toolbar">
                        <span class="author">
                            ${vnode.attrs.comment.author.website ? html`
                                <a href="${vnode.attrs.comment.author.website}" rel="nofollow">${vnode.attrs.comment.author.name}</a>
                            ` : html`
                                ${vnode.attrs.comment.author.name}
                            `}
                        </span>

                        <${ScoreLabel} score="${vnode.attrs.comment.score}" />

                        <time datetime="${vnode.attrs.comment.published}" title="${new Date(vnode.attrs.comment.published).toLocaleString()}">${vnode.attrs.comment.publishedLabel}</time>
                    </div>

                    ${m.trust(vnode.attrs.comment.html)}

                    <div class="comment-toolbar">
                        <a title="Upvote" onclick="${() => vnode.attrs.onupvote(vnode.attrs.comment)}" tabindex="0">▲ upvote</a>

                        <a title="Downvote" onclick="${() => vnode.attrs.ondownvote(vnode.attrs.comment)}" tabindex="0">▼ downvote</a>

                        <a href="#comment-${id}">permalink</a>

                        ${this.showReply ?
                            html`<a onclick="${() => this.showReply = false}" tabindex="0">close</a>` :
                            html`<a onclick="${() => this.showReply = true}" tabindex="0">reply</a>`}
                    </div>

                    ${this.showReply && m(CommentForm, {
                        autofocus: true,
                        onsubmit: async form => {
                            await vnode.attrs.onreply(vnode.attrs.comment, form);
                            this.showReply = false;
                        }
                    })}

                    ${vnode.attrs.comment.children.map(comment => m(Comment, {
                        ...vnode.attrs,
                        comment
                    }))}
                </div>
            </article>
        `;
    }
}

class ScoreLabel {
    view({ attrs }) {
        if (attrs.score > 1) {
            return m("span", `${attrs.score} points`);
        }

        if (attrs.score === 1) {
            return m("span", "1 point");
        }
    }
}

class CommentForm {
    oninit() {
        let commentAuthor = AuthorLocalStorage.author;

        if (commentAuthor) {
            this.name = commentAuthor.name;
            this.email = commentAuthor.email;
            this.website = commentAuthor.website;
        }
    }

    oncreate(vnode) {
        if (vnode.attrs.autofocus) {
            vnode.dom.querySelector("textarea").focus();
        }
    }

    async onsubmit(vnode, e) {
        e.preventDefault();

        this.submitting = true;

        try {
            AuthorLocalStorage.author = {
                name: this.name,
                email: this.email,
                website: this.website
            };

            if (vnode.attrs.onsubmit) {
                await vnode.attrs.onsubmit({
                    name: this.name,
                    email: this.email,
                    website: this.website,
                    text: this.text
                });
            }

            this.text = "";
        } catch (e) {
            let textarea = vnode.dom.querySelector("textarea");
            textarea.setCustomValidity(e.response.error);
            textarea.reportValidity();
        } finally {
            this.submitting = false;
        }
    }

    view(vnode) {
        return html`
            <${Loadable} loading="${this.submitting}">
                <form class="comment-form" onsubmit=${e => this.onsubmit(vnode, e)}>
                    <div>
                        <textarea
                            name="text"
                            placeholder="Comment text (supports Markdown)"
                            required
                            oninput="${e => {
                                this.text = e.target.value;
                                e.target.setCustomValidity("");
                            }}"
                        >${this.text}</textarea>
                    </div>
                    <div class="author-details">
                        <input
                            type="text"
                            name="name"
                            placeholder="Name"
                            required
                            maxlength="255"
                            value="${this.name}"
                            oninput="${e => this.name = e.target.value}"
                        />
                        <input
                            type="email"
                            name="email"
                            placeholder="Email"
                            required
                            maxlength="255"
                            value="${this.email}"
                            oninput="${e => this.email = e.target.value}"
                        />
                        <input
                            type="text"
                            name="website"
                            placeholder="Website (optional)"
                            maxlength="255"
                            value="${this.website}"
                            oninput="${e => this.website = e.target.value}"
                        />
                    </div>
                    <div>
                        <input type="submit" value="Submit" />
                    </div>
                </form>
            </>
        `;
    }
}

function scrollIntoViewIfNeeded(element) {
    let { top, bottom } = element.getBoundingClientRect();

    if (top > window.innerHeight || bottom < 0) {
        element.scrollIntoView();
    }
}
