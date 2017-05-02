<?php
namespace sagebind\blog\actions;

use Cake\Chronos\Chronos;
use Icicle\Http\Message\{BasicResponse, Request};
use Icicle\Stream\MemorySink;

class RssFeedAction extends Action
{
    public function handle(Request $request, array $args): \Generator
    {
        if (isset($args['category'])) {
            $articles = $this->app->getArticleStore()->getByCategory($args['category']);
        } else {
            $articles = $this->app->getArticleStore()->getIterator();
        }

        $html = $this->app->getRenderer()->render('feed.rss', [
            'pubDate' => Chronos::now(),
            'articles' => $articles,
        ]);

        $sink = new MemorySink();
        yield from $sink->end($html);

        return new BasicResponse(200, [
            'Content-Type' => 'application/xml',
            'Content-Length' => $sink->getLength(),
        ], $sink);
    }
}
