import m from "https://cdn.skypack.dev/mithril@2";
import htm from "https://cdn.skypack.dev/htm@3";
import * as api from "./api.js";

let html = htm.bind(m);

let articleSlug = window.location.pathname.substring(1);

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

let comments = [];

async function refreshComments() {
    comments = await api.getComments(articleSlug);
}

class CommentsSection {
    async oninit() {
        this.loading = true;

        try {
            await refreshComments();
        } finally {
            this.loading = false;
        }
    }

    view() {
        return html`
            <${CommentForm} />
            <div class="${this.loading ? "loading" : ""}">
                ${comments.map(comment => m(Comment, { comment }))}
            </div>
        `;
    }
}

class Comment {
    async upvote(vnode) {
        if (await api.upvoteComment(vnode.attrs.comment.id)) {
            await refreshComments();
        }
    }

    async downvote(vnode) {
        if (await api.downvoteComment(vnode.attrs.comment.id)) {
            await refreshComments();
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
                    <span>
                        ${vnode.attrs.comment.author.website ? html`
                            <a class="author" href="${vnode.attrs.comment.author.website}" rel="nofollow">${vnode.attrs.comment.author.name}</a>
                        ` : html`
                            <span class="author">${vnode.attrs.comment.author.name}</span>
                        `}

                        —

                        <a class="comment-date" href="#comment-${id}">
                            <time datetime="${vnode.attrs.comment.published}">${vnode.attrs.comment.publishedLabel}</time>
                        </a>
                    </span>

                    ${m.trust(vnode.attrs.comment.html)}

                    <div class="comment-actions">
                            <a title="Upvote" onclick="${() => this.upvote(vnode)}">+1</a>

                            ${vnode.attrs.comment.score != 0 && html`
                                <span class="score">${vnode.attrs.comment.score}</span>
                            `}

                            <a title="Downvote" onclick="${() => this.downvote(vnode)}">-1</a>

                        ${this.showReply ?
                            html`<a onclick="${() => this.showReply = false}">Close</a>` :
                            html`<a onclick="${() => this.showReply = true}">Reply</a>`}
                    </div>

                    ${this.showReply && m(CommentForm, {
                        parentCommentId: vnode.attrs.comment.id,
                        autofocus: true,
                        onsubmit: async () => {
                            await refreshComments();
                            this.showReply = false;
                        }
                    })}

                    ${vnode.attrs.comment.children.map(comment => m(Comment, { comment }))}
                </div>
            </article>
        `;
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
                articleSlug,
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
                        oninput="${e => this.text = e.target.value}"
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

document.querySelectorAll(".comments-section").forEach(element => {
    m.mount(element, CommentsSection);
});
