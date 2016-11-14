<?php
namespace sagebind\blog;

use Yosymfony\Toml\Toml;

/**
 * Storage object for fetching articles.
 */
class ArticleStore implements \IteratorAggregate
{
    private $cache;
    private $commonMarkConverter;
    private $path;

    /**
     * Creates a new article store.
     *
     * @param string $path The path to the directory containing the article flat files.
     */
    public function __construct(string $path)
    {
        $this->cache = new MemoryCache();
        $this->path = $path;
    }

    /**
     * Gets an article by its URL slug.
     */
    public function getBySlug(string $slug): Article
    {
        $fileName = str_replace('/', '-', $slug) . '.md';
        return $this->getArticleFromFile($fileName);
    }

    /**
     * Gets all articles in a given category.
     */
    public function getByCategory(string $category): \Iterator
    {
        foreach ($this->getIterator() as $article) {
            if ($article->category() === $category) {
                yield $article;
            }
        }
    }

    /**
     * Gets an iterator that iterates over each article.
     *
     * This list is never cached.
     *
     * @param bool $unpublished Whether articles not yet published should be fetched.
     */
    public function getIterator($unpublished = false): \Iterator
    {
        $files = array_reverse(glob($this->path . '/*.md'));

        foreach ($files as $file) {
            $file = new \SplFileInfo($file);

            if ($file->isFile()) {
                try {
                    $article = $this->getArticleFromFile($file->getFilename());

                    if ($unpublished || $article->isPublished()) {
                        yield $article;
                    }
                } catch (\Throwable $e) {}
            }
        }
    }

    /**
     * Loads an article by its file name.
     *
     * @param  string  $name The file name.
     * @return Article       The parsed article.
     */
    private function getArticleFromFile(string $name): Article
    {
        // Check for a cache hit first.
        if ($this->cache->has($name)) {
            return $this->cache->get($name);
        }

        $path = $this->path . '/' . $name;
        if (!is_file($path)) {
            throw new \Exception('The article "' . $path . '" does not exist.');
        }

        // This is blocking, but hopefully it only happens once.
        $data = file_get_contents($path);

        // If a TOML front matter block is given, parse the contained metadata.
        if (strpos($data, '+++') === 0) {
            $data = substr($data, 3);
            $pos = strpos($data, '+++');
            $metatdata = substr($data, 0, $pos);
            $contents = ltrim(substr($data, $pos + 3));

            // Parse header into an array of options.
            $metatdata = Toml::parse($metatdata);
        } else {
            // Front matter not found, invalid article.
            throw new \Exception("Invalid article file!");
        }

        // Parse the slug.
        $slug = str_replace('-', '/', substr($name, 0, 11)) . substr($name, 11, -3);

        $article = new Article($metatdata, $contents, $slug);
        $this->cache->set($name, $article);
        return $article;
    }
}
