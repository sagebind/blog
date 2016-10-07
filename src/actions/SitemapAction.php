<?php
namespace sagebind\blog\actions;

use Icicle\Http\Message\{BasicResponse, Request};
use Icicle\Stream\MemorySink;

class SitemapAction extends Action
{
    public function handle(Request $request, array $args): \Generator
    {
        $html = $this->app->getRenderer()->render('sitemap.xml', [
            'lastmod' => date('c', filemtime(__DIR__.'/../../templates/index.mustache')),
            'articles' => $this->app->getArticleStore()->getIterator(),
        ]);

        $sink = new MemorySink();
        yield from $sink->end($html);

        return new BasicResponse(200, [
            'Content-Type' => 'application/xml',
            'Content-Length' => $sink->getLength(),
        ], $sink);
    }
}
