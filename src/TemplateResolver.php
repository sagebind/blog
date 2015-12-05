<?php
namespace coderstephen\blog;

use Phly\Mustache\Resolver\ResolverInterface;

class TemplateResolver implements ResolverInterface
{
    private $cache;
    private $path;

    public function __construct(string $path)
    {
        $this->cache = new MemoryCache();
        $this->path = $path;
    }

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
