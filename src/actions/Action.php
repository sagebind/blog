<?php
namespace coderstephen\blog\actions;

use coderstephen\blog\Application;
use Icicle\Http\Message\RequestInterface;

abstract class Action
{
    protected $app;

    public function __construct(Application $app)
    {
        $this->app = $app;
    }

    public abstract function handle(RequestInterface $request, array $args): \Generator;
}
