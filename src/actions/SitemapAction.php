<?php
namespace coderstephen\blog\actions;

use Icicle\Http\Message\RequestInterface;
use Icicle\Http\Message\Response;
use Icicle\Stream\MemorySink;

class SitemapAction extends Action
{
    public function handle(RequestInterface $request, array $args): \Generator
    {
        $html = $this->app->getRenderer()->render('sitemap.xml', [
            'lastmod' => date('c', filemtime(__DIR__.'/../../templates/index.mustache')),
            'articles' => $this->app->getArticleStore()->getIterator(),
        ]);

        $sink = new MemorySink();
        yield $sink->end($html);

        yield new Response(200, [
            'Content-Type' => 'application/xml',
            'Content-Length' => $sink->getLength(),
        ], $sink);
    }
}
