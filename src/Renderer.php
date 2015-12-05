<?php
namespace coderstephen\blog;

use Phly\Mustache\Mustache;
use Phly\Mustache\Resolver\ResolverInterface;

/**
 * Renders template names with a view of values.
 */
class Renderer
{
    private $mustachRenderer;

    /**
     * Creates a new renderer.
     *
     * @param string $path The path to the template directory.
     */
    public function __construct(string $path)
    {
        // Create a Mustache parser using our custom resolver.
        $this->mustachRenderer = new Mustache();
        $this->mustachRenderer->getResolver()->attach(new TemplateResolver($path));
    }

    /**
     * Renders a template with a view array.
     */
    public function render(string $template, array $view)
    {
        return $this->mustachRenderer->render($template, $view);
    }
}
