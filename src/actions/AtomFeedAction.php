<?php
namespace sagebind\blog\actions;

use Cake\Chronos\Chronos;
use Icicle\Http\Message\{BasicResponse, Request};
use Icicle\Stream\MemorySink;

class AtomFeedAction extends Action
{
    public function handle(Request $request, array $args): \Generator
    {
        $id = 'http://stephencoakley.com/feed';

        if (isset($args['category'])) {
            $articles = $this->app->getArticleStore()->getByCategory($args['category']);
            $id .= '/' . $args['category'];
        } else {
            $articles = $this->app->getArticleStore()->getIterator();
        }

        $html = $this->app->getRenderer()->render('feed.atom', [
            'updated' => Chronos::now(),
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
