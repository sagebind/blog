<?php
namespace coderstephen\blog\actions;

use coderstephen\blog\Application;
use Icicle\Http\Message\RequestInterface;

/**
 * Represents a single action that can handle a request and return a response.
 */
abstract class Action
{
    protected $app;

    /**
     * Creates a new action instance.
     *
     * @param Application $app The application initiating the request.
     */
    public function __construct(Application $app)
    {
        $this->app = $app;
    }

    /**
     * Handles a request and returns a response.
     *
     * @param  RequestInterface $request An HTTP request.
     * @param  array            $args    An array of route variables.
     * @return \Generator
     *
     * @resolve \Icicle\Http\Message\Response The HTTP response to respond with.
     */
    public abstract function handle(RequestInterface $request, array $args): \Generator;
}
