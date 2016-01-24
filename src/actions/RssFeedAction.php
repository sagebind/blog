<?php
namespace coderstephen\blog\actions;

use Carbon\Carbon;
use Icicle\Http\Message\RequestInterface;
use Icicle\Http\Message\Response;
use Icicle\Stream\MemorySink;

class RssFeedAction extends Action
{
    public function handle(RequestInterface $request, array $args): \Generator
    {
        if (isset($args['category'])) {
            $articles = $this->app->getArticleStore()->getByCategory($args['category']);
        } else {
            $articles = $this->app->getArticleStore()->getIterator();
        }

        $html = $this->app->getRenderer()->render('feed.rss', [
            'pubDate' => Carbon::now(),
            'articles' => $articles,
        ]);

        $sink = new MemorySink();
        yield $sink->end($html);

        yield new Response(200, [
            'Content-Type' => 'application/xml',
            'Content-Length' => $sink->getLength(),
        ], $sink);
    }
}
