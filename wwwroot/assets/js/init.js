window.addEventListener('DOMContentLoaded', function() {
    mermaid.initialize({
        startOnLoad: true,
        theme: 'dark'
    });

    var mermaidCounter = 0;

    document.querySelectorAll('pre > code.language-mermaid').forEach(function (code) {
        var container = document.createElement('div');
        container.className = 'mermaid-chart';
        container.id = 'mermaid-chart-' + (mermaidCounter++);

        mermaid.render(container.id, code.textContent, function (svg) {
            container.innerHTML = svg;
            if (code.parentElement.parentElement) {
                code.parentElement.parentElement.replaceChild(container, code.parentElement);
            }
        });
    });
});
