<?php
namespace sagebind\blog\actions;

use Icicle\Http\Message\{BasicResponse, Request};
use Icicle\Stream\MemorySink;

class AssetAction extends Action
{
    public function handle(Request $request, array $args): \Generator
    {
        $manager = $this->app->getAssetManager();
        $sink = new MemorySink();

        if (!$manager->exists($args['asset'])) {
            yield from $sink->end('404 Resource Not Found');

            return new BasicResponse(404, [
                'Content-Type' => 'text/plain',
                'Content-Length' => $sink->getLength(),
            ], $sink);
        }

        yield from $sink->end($manager->getBytes($args['asset']));

        return new BasicResponse(200, [
            'Content-Type' => $manager->getMimeType($args['asset']),
            'Content-Length' => $sink->getLength(),
        ], $sink);
    }
}
