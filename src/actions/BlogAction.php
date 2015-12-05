<?php
namespace coderstephen\blog\actions;

use Icicle\Http\Message\RequestInterface;
use Icicle\Http\Message\Response;
use Icicle\Stream\MemorySink;

class BlogAction extends Action
{
    public function handle(RequestInterface $request, array $args): \Generator
    {
        $html = $this->app->getRenderer()->render('blog', [
            'articles' => $this->app->getArticleStore()->getIterator(),
        ]);

        $sink = new MemorySink();
        yield $sink->end($html);

        yield new Response(200, [
            'Content-Type' => 'text/html',
            'Content-Length' => $sink->getLength(),
        ], $sink);
    }
}
