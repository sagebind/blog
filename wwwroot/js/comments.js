import m from "https://cdn.skypack.dev/mithril@2";
import htm from "https://cdn.skypack.dev/htm@3";
import * as api from "./api.js";

let html = htm.bind(m);

function getAuthorFromLocalStorage() {
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

        localStorage.setItem("commentAuthor", JSON.stringify(commentAuthor));

        return commentAuthor;
    }

    return {};
}

class FeatherIcon {
    view(vnode) {
        return html`
            <svg
                width="20"
                height="20"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <use xlink:href="/assets/images/feather-sprite.svg#${vnode.attrs.name}"/>
            </svg>
        `;
    }
}

class CommentsSection {
    constructor() {
        this.comments = [];
        this.loading = true;
    }

    async refreshComments() {
        this.comments = await api.getComments(this.articleSlug);
    }

    async oninit(vnode) {
        this.articleSlug = vnode.attrs.articleSlug;

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
                    element.scrollIntoView();
                }
            }
        }
    }

    view() {
        let commentsCount = this.comments.reduce(function reducer(count, comment) {
            return count + 1 + comment.children.reduce(reducer, 0);
        }, 0);

        return html`
            <h2>${commentsCount} comments</h2>

            <p>Let me know what you think in the comments below. Remember to keep it civil!</p>

            <${CommentForm} articleSlug="${this.articleSlug}" />

            <div class="${this.loading ? "loading" : ""}">
                ${this.comments.map(comment => m(Comment, {
                    articleSlug: this.articleSlug,
                    comment,
                    onchange: async () => {
                        await this.refreshComments();
                    }
                }))}
            </div>
        `;
    }
}

class Comment {
    async upvote(vnode) {
        if (await api.upvoteComment(vnode.attrs.comment.id)) {
            await vnode.attrs.onchange();
        }
    }

    async downvote(vnode) {
        if (await api.downvoteComment(vnode.attrs.comment.id)) {
            await vnode.attrs.onchange();
        }
    }

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
                        <a title="Upvote" onclick="${() => this.upvote(vnode)}" tabindex="0">▲ upvote</a>

                        <a title="Downvote" onclick="${() => this.downvote(vnode)}" tabindex="0">▼ downvote</a>

                        <a href="#comment-${id}">permalink</a>

                        ${this.showReply ?
                            html`<a onclick="${() => this.showReply = false}" tabindex="0">close</a>` :
                            html`<a onclick="${() => this.showReply = true}" tabindex="0">reply</a>`}
                    </div>

                    ${this.showReply && m(CommentForm, {
                        articleSlug: vnode.attrs.articleSlug,
                        parentCommentId: vnode.attrs.comment.id,
                        autofocus: true,
                        onsubmit: async () => {
                            await vnode.attrs.onchange();
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
        let commentAuthor = getAuthorFromLocalStorage();

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
            await api.submitComment({
                articleSlug: vnode.attrs.articleSlug,
                name: this.name,
                email: this.email,
                website: this.website,
                text: this.text,
                parentCommentId: vnode.attrs.parentCommentId,
            });

            this.text = "";

            if (vnode.attrs.onsubmit) {
                await vnode.attrs.onsubmit();
            }
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
            <form class="comment-form ${this.submitting && "loading"}" onsubmit=${e => this.onsubmit(vnode, e)}>
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
        `;
    }
}

document.querySelectorAll("#comments").forEach(element => {
    m.mount(element, {
        view() {
            return m(CommentsSection, element.dataset);
        }
    });
});
