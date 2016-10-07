<?php
namespace sagebind\blog;

use FastRoute;
use Icicle\Http\Message\{BasicResponse, Request, Response};
use Icicle\Http\Server\RequestHandler as Handler;
use Icicle\Socket\Socket;
use Icicle\Stream\MemorySink;


class RequestHandler implements Handler
{
    private $app;

    public function __construct(Application $app)
    {
        $this->app = $app;
    }

    /**
     * Handles an incoming HTTP request and dispatches it to the appropriate action.
     *
     * @param  Request $request The HTTP request message.
     * @param  Socket  $socket  The client socket connection.
     *
     * @return \Generator
     *
     * @resolve \Icicle\Http\Message\Response The appropriate HTTP response.
     */
    public function onRequest(Request $request, Socket $socket): \Generator
    {
        $dispatched = $this->app->getDispatcher()->dispatch(
            $request->getMethod(),
            $request->getUri()->getPath()
        );

        switch ($dispatched[0]) {
            case FastRoute\Dispatcher::NOT_FOUND: // no route found
                $randomStr = '';
                for ($i = 0; $i < 1000; ++$i) {
                    $char = chr(mt_rand(32, 126));
                    if ($char !== '<') {
                        $randomStr .= $char;
                    }
                }

                $html = $this->app->getRenderer()->render('404', [
                    'randomStr' => $randomStr,
                ]);

                $sink = new MemorySink();
                yield from $sink->end($html);

                return new BasicResponse(404, [
                    'Content-Type' => 'text/html',
                    'Content-Length' => $sink->getLength(),
                ], $sink);

            case FastRoute\Dispatcher::METHOD_NOT_ALLOWED: // HTTP request method not allowed
                $sink = new MemorySink();
                yield from $sink->end('405 Method Not Allowed');

                return new BasicResponse(405, [
                    'Content-Type' => 'text/plain',
                    'Content-Length' => $sink->getLength(),
                ], $sink);

            case FastRoute\Dispatcher::FOUND: // route was found
                $action = new $dispatched[1]($this->app);
                $response = yield from $action->handle($request, $dispatched[2]);
                return $response;

            default:
                throw new \RuntimeException('Invalid router state');
        }
    }

    public function onError(int $code, Socket $socket): Response
    {
        return new BasicResponse($code);
    }
}
