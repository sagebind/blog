<?php
namespace sagebind\blog;

use FastRoute;
use Icicle\Http\Server\Server;
use Icicle\Loop;

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
        $this->server = new Server(new RequestHandler($this));

        // Create some core services.
        $this->renderer = new Renderer($path . '/templates');
        $this->assetManager = new AssetManager($path . '/static');
        $this->articleStore = new ArticleStore($path . '/articles');
    }

    public function getDispatcher(): FastRoute\Dispatcher
    {
        return $this->dispatcher;
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
    public function run(int $port, string $address = '*')
    {
        $this->server->listen($port, $address);
        Loop\run();
    }
}
