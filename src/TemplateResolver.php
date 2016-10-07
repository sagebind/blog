<?php
namespace sagebind\blog;

use Phly\Mustache\Resolver\ResolverInterface;

/**
 * Custom Mustache template resolver that uses caching.
 */
class TemplateResolver implements ResolverInterface
{
    private $cache;
    private $path;

    /**
     * Creates a new resolver.
     *
     * @param string $path The path to the template directory.
     */
    public function __construct(string $path)
    {
        $this->cache = new MemoryCache();
        $this->path = $path;
    }

    /**
     * Resolves a template and returns its file contents.
     *
     * @param  string $template The template name.
     * @return string           The template Mustache source.
     */
    public function resolve($template)
    {
        if ($this->cache->has($template)) {
            return $this->cache->get($template);
        }

        $path = $this->path . '/' . $template . '.mustache';
        if (!is_file($path)) {
            throw new \Exception('The template "' . $path . '" does not exist.');
        }

        $data = file_get_contents($path);
        $this->cache->set($template, $data);
        return $data;
    }
}
