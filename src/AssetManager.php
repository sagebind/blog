<?php
namespace sagebind\blog;

use Dflydev\ApacheMimeTypes;

/**
 * An asset manager that reads static asset files and uses memory caching.
 */
class AssetManager
{
    private $cache;
    private $repository;
    private $path;

    /**
     * Cretes a new asset manager.
     *
     * @param string $path The path to the asset directory.
     */
    public function __construct(string $path)
    {
        $this->cache = new MemoryCache();
        $this->repository = new ApacheMimeTypes\FlatRepository();
        $this->path = $path;
    }

    /**
     * Checks if an asset exists.
     *
     * @param string $asset The asset name.
     */
    public function exists(string $asset): bool
    {
        return strpos($asset, '..') !== false
            || $this->cache->has($asset)
            || is_file($this->path . '/' . $asset);
    }

    /**
     * Gets the raw contents of the asset file.
     *
     * @param  string $asset The asset name.
     * @return string        The asset file contents.
     */
    public function getBytes(string $asset): string
    {
        // Check for a cache hit.
        if ($this->cache->has($asset)) {
            return $this->cache->get($asset);
        }

        if (!$this->exists($asset)) {
            throw new \Exception("The asset '$asset' does not exist.");
        }

        // This is blocking, but hopefully it only happens once.
        $path = $this->path . '/' . $asset;
        $bytes = file_get_contents($path);

        $this->cache->set($asset, $bytes);
        return $bytes;
    }

    /**
     * Gets the mime type of a given asset.
     *
     * @param  string $asset The asset name
     * @return string        The asset mime type.
     */
    public function getMimeType($asset)
    {
        if (!$this->exists($asset)) {
            throw new \Exception("The asset '$asset' does not exist.");
        }

        $extension = pathinfo($asset, PATHINFO_EXTENSION);
        switch ($extension) {
            // Special handling for fonts.
            case 'eot':
                return 'application/vnd.ms-fontobject';
            case 'ttf':
                return 'application/font-sfnt';
            case 'woff':
                return 'application/font-woff';

            default:
                return $this->repository->findType($extension) ?: 'application/octet-stream';
        }
    }
}
