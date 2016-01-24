<?php
namespace coderstephen\blog;

use FastRoute;
use Icicle\Http\Message\RequestInterface;
use Icicle\Http\Message\Response;
use Icicle\Http\Server\Server;
use Icicle\Loop;
use Icicle\Socket\SocketInterface;
use Icicle\Stream\MemorySink;

/**
 * Main entry point for the server application. Manages HTTP connections and objects
 * that persist between requests.
 */
class Application
{
    private $path;
    private $dispatcher;
    private $server;
    private $renderer;
    private $assetManager;
    private $articleStore;

    /**
     * Create a new server application instance.
     *
     * @param string $path The path to the application files.
     */
    public function __construct(string $path)
    {
        $this->path = $path;

        // Register some routes.
        $this->dispatcher = FastRoute\simpleDispatcher(function(FastRoute\RouteCollector $r) {
            $r->addRoute('GET', '/', actions\IndexAction::class);
            $r->addRoute('GET', '/blog', actions\BlogAction::class);
            $r->addRoute('GET', '/category/{category}', actions\CategoryAction::class);
            $r->addRoute('GET', '/portfolio', actions\PortfolioAction::class);
            $r->addRoute('GET', '/{year:\\d{4}}/{month:\\d{2}}/{day:\\d{2}}/{name}', actions\ArticleAction::class);
            $r->addRoute('GET', '/{asset:(?:content|assets)/[^?]+}[?{query}]', actions\AssetAction::class);

            // Complicated feed routes :/
            $r->addRoute('GET', '/sitemap.xml', actions\SitemapAction::class);
            $r->addRoute('GET', '/feed[/{category:[a-z]+}]', actions\AtomFeedAction::class);
            $r->addRoute('GET', '/feed.atom', actions\AtomFeedAction::class);
            $r->addRoute('GET', '/feed/{category:[a-z]+}.atom', actions\AtomFeedAction::class);
            $r->addRoute('GET', '/feed.rss', actions\RssFeedAction::class);
            $r->addRoute('GET', '/feed/{category:[a-z]+}.rss', actions\RssFeedAction::class);
        });

        // Create the server object.
        $this->server = new Server(function ($request, $socket) {
            try {
                yield $this->handleRequest($request, $socket);
            } catch (\Throwable $e) {
                echo $e;
                throw $e;
            }
        });

        // Create some core services.
        $this->renderer = new Renderer($path . '/templates');
        $this->assetManager = new AssetManager($path . '/static');
        $this->articleStore = new ArticleStore($path . '/articles');
    }

    public function getRenderer(): Renderer
    {
        return $this->renderer;
    }

    public function getAssetManager(): AssetManager
    {
        return $this->assetManager;
    }

    public function getArticleStore(): ArticleStore
    {
        return $this->articleStore;
    }

    /**
     * Runs the application.
     */
    public function run(int $port)
    {
        $this->server->listen($port);
        Loop\run();
    }

    /**
     * Handles an incoming HTTP request and dispatches it to the appropriate action.
     *
     * @param  RequestInterface $request The HTTP request message.
     * @param  SocketInterface  $socket  The client socket connection.
     *
     * @return \Generator
     *
     * @resolve \Icicle\Http\Message\Response The appropriate HTTP response.
     */
    private function handleRequest(RequestInterface $request, SocketInterface $socket): \Generator {
        $dispatched = $this->dispatcher->dispatch(
            $request->getMethod(),
            $request->getRequestTarget()
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

                $html = $this->renderer->render('404', [
                    'randomStr' => $randomStr,
                ]);

                $sink = new MemorySink();
                yield $sink->end($html);

                $response = new Response(404, [
                    'Content-Type' => 'text/html',
                    'Content-Length' => $sink->getLength(),
                ], $sink);
                break;

            case FastRoute\Dispatcher::METHOD_NOT_ALLOWED: // HTTP request method not allowed
                $sink = new MemorySink();
                yield $sink->end('405 Method Not Allowed');

                $response = new Response(405, [
                    'Content-Type' => 'text/plain',
                    'Content-Length' => $sink->getLength(),
                ], $sink);
                break;

            case FastRoute\Dispatcher::FOUND: // route was found
                $action = new $dispatched[1]($this);
                $response = yield $action->handle($request, $dispatched[2]);
                break;
        }

        yield $response;
    }
}
