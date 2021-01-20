import { hljs, m } from "./deps.js";
import { CommentsSection } from "./comments.js";

mermaid.initialize({
    startOnLoad: true,
    theme: "dark"
});

var mermaidCounter = 0;

document.querySelectorAll("pre > code.language-mermaid").forEach(function (code) {
    var container = document.createElement("div");
    container.className = "mermaid-chart";
    container.id = "mermaid-chart-" + (mermaidCounter++);

    mermaid.render(container.id, code.textContent, function (svg) {
        container.innerHTML = svg;
        if (code.parentElement.parentElement) {
            code.parentElement.parentElement.replaceChild(container, code.parentElement);
        }
    });
});

// addEventListener("load", function () {
hljs.initHighlighting();
// }, false);

function animateTyping(element) {
    let fastSpeed = 40;
    let slowSpeed = 150;
    let fullText = element.textContent;

    if (fullText) {
        // Create a clone of this element and hide the original element. We do
        // this to preserve the full text of the original element in the DOM to
        // remain screen-reader friendly instead of changing the document
        // content repeatedly.
        let animatedElement = element.cloneNode();
        animatedElement.setAttribute("aria-hidden", "true");

        let staticElement = element.cloneNode();
        staticElement.style.display = "none";

        let containerElement = document.createElement("div");
        containerElement.appendChild(staticElement);
        containerElement.appendChild(animatedElement);

        element.parentElement.replaceChild(containerElement, element);

        animatedElement.textContent = "";
        let i = 0;

        setTimeout(function increment() {
            animatedElement.textContent += fullText[i++];

            if (i < fullText.length) {
                setTimeout(increment, fullText[i] === " " ? slowSpeed : fastSpeed);
            }
        }, fastSpeed);
    }
}

document.querySelectorAll("h1").forEach(animateTyping);

document.querySelectorAll("#comments").forEach(element => {
    m.mount(element, {
        view() {
            return m(CommentsSection, element.dataset);
        }
    });
});
