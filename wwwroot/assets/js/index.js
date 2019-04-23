mermaid.init({}, "pre > code.language-mermaid");

addEventListener('load', function () {
    hljs.initHighlighting();

    document.querySelectorAll("time").forEach(function (e) {
        e.innerText = moment(e.getAttribute("datetime")).fromNow();
    });
}, false);
