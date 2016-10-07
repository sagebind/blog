<?php
namespace sagebind\blog\actions;

use Icicle\Http\Message\{BasicResponse, Request};
use Icicle\Stream\MemorySink;

class CategoryAction extends Action
{
    public function handle(Request $request, array $args): \Generator
    {
        $html = $this->app->getRenderer()->render('category', [
            'category' => $args['category'],
            'articles' => $this->app->getArticleStore()->getByCategory($args['category']),
        ]);

        $sink = new MemorySink();
        yield from $sink->end($html);

        return new BasicResponse(200, [
            'Content-Type' => 'text/html',
            'Content-Length' => $sink->getLength(),
        ], $sink);
    }
}
