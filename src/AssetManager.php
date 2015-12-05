<?php
namespace coderstephen\blog;

use Dflydev\ApacheMimeTypes;

class AssetManager
{
    private $cache;
    private $repository;
    private $path;

    public function __construct(string $path)
    {
        $this->cache = new MemoryCache();
        $this->repository = new ApacheMimeTypes\FlatRepository();
        $this->path = $path;
    }

    public function exists($asset): bool
    {
        return strpos($asset, '..') !== false
            || $this->cache->has($asset)
            || is_file($this->path . '/' . $asset);
    }

    public function getBytes($asset)
    {
        if ($this->cache->has($asset)) {
            return $this->cache->get($asset);
        }

        if (!$this->exists($asset)) {
            throw new \Exception("The asset '$asset' does not exist.");
        }

        $path = $this->path . '/' . $asset;
        $bytes = file_get_contents($path);
        $this->cache->set($asset, $bytes);
        return $bytes;
    }

    public function getMimeType($asset)
    {
        if (!$this->exists($asset)) {
            throw new \Exception("The asset '$asset' does not exist.");
        }

        $extension = pathinfo($asset, PATHINFO_EXTENSION);
        switch ($extension) {
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
