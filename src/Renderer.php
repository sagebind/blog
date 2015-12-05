<?php
namespace coderstephen\blog;

use Phly\Mustache\Mustache;
use Phly\Mustache\Resolver\ResolverInterface;

class Renderer
{
    private $mustachRenderer;

    public function __construct(string $path)
    {
        $this->mustachRenderer = new Mustache();
        $this->mustachRenderer->getResolver()->attach(new TemplateResolver($path));
    }

    /**
     * Renders a template.
     */
    public function render(string $template, array $view)
    {
        return $this->mustachRenderer->render($template, $view);
    }
}
