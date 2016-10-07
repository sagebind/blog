<?php
namespace sagebind\blog;

/**
 * Literally a cache that stores stuff in variables. That's it.
 *
 * Uses compression to hopefully save some memory.
 */
class MemoryCache
{
    private $cache = [];

    /**
     * Checks if a key is in the cache.
     */
    public function has(string $key): bool
    {
        return isset($this->cache[$key]);
    }

    /**
     * Gets the cached value for a key.
     */
    public function get(string $key)
    {
        if ($this->has($key)) {
            return unserialize(gzuncompress($this->cache[$key]));
        }
    }

    /**
     * Sets the value for a key.
     */
    public function set(string $key, $data)
    {
        $this->cache[$key] = gzcompress(serialize($data));
    }

    /**
     * Clears a key from the cache.
     */
    public function clear(string $key)
    {
        unset($this->cache[$key]);
    }

    /**
     * Empties the entire cache.
     */
    public function empty()
    {
        $this->cache = [];
    }
}
