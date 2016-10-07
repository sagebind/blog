<?php
namespace sagebind\blog\actions;

use Icicle\Http\Message\Request;
use Icicle\Http\Message\BasicResponse;
use Icicle\Stream\MemorySink;

class ArticleAction extends Action
{
    public function handle(Request $request, array $args): \Generator
    {
        $article = $this->app->getArticleStore()->getBySlug(substr($request->getUri()->getPath(), 1));

        $html = $this->app->getRenderer()->render('article', [
            'article' => $article,
        ]);

        $sink = new MemorySink();
        yield from $sink->end($html);

        return new BasicResponse(200, [
            'Content-Type' => 'text/html',
            'Content-Length' => $sink->getLength(),
        ], $sink);
    }
}
