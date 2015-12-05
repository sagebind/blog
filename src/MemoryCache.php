<?php
namespace coderstephen\blog;

/**
 * Literally a cache that stores stuff in variables. That's it.
 */
class MemoryCache
{
    private $cache = [];

    public function has(string $key): bool
    {
        return isset($this->cache[$key]);
    }

    public function get(string $key)
    {
        if ($this->has($key)) {
            return unserialize(gzuncompress($this->cache[$key]));
        }
    }

    public function set(string $key, $data)
    {
        $this->cache[$key] = gzcompress(serialize($data));
    }

    public function clear(string $key)
    {
        unset($this->cache[$key]);
    }

    public function empty()
    {
        $this->cache = [];
    }
}
