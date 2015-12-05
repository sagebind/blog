<?php
namespace coderstephen\blog\actions;

use Icicle\Http\Message\RequestInterface;
use Icicle\Http\Message\Response;
use Icicle\Stream\MemorySink;

class AssetAction extends Action
{
    public function handle(RequestInterface $request, array $args): \Generator
    {
        $manager = $this->app->getAssetManager();
        $sink = new MemorySink();

        if (!$manager->exists($args['asset'])) {
            yield $sink->end('404 Resource Not Found');

            yield new Response(404, [
                'Content-Type' => 'text/plain',
                'Content-Length' => $sink->getLength(),
            ], $sink);
            return;
        }

        yield $sink->end($manager->getBytes($args['asset']));

        yield new Response(200, [
            'Content-Type' => $manager->getMimeType($args['asset']),
            'Content-Length' => $sink->getLength(),
        ], $sink);
    }
}
