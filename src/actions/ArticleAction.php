<?php
namespace coderstephen\blog\actions;

use Icicle\Http\Message\RequestInterface;
use Icicle\Http\Message\Response;
use Icicle\Stream\MemorySink;

class ArticleAction extends Action
{
    public function handle(RequestInterface $request, array $args): \Generator
    {
        $article = $this->app->getArticleStore()->getBySlug(substr($request->getRequestTarget(), 1));

        $html = $this->app->getRenderer()->render('article', [
            'article' => $article,
        ]);

        $sink = new MemorySink();
        yield $sink->end($html);

        yield new Response(200, [
            'Content-Type' => 'text/html',
            'Content-Length' => $sink->getLength(),
        ], $sink);
    }
}
