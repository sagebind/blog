<?php
namespace coderstephen\blog;

use League\CommonMark\CommonMarkConverter;
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
        $this->commonMarkConverter = new CommonMarkConverter();
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
     */
    public function getIterator(): \Iterator
    {
        $articles = [];

        foreach (new \GlobIterator($this->path . '/*.md') as $file) {
            if ($file->isFile()) {
                $articles[] = $this->getArticleFromFile($file->getFilename());
            }
        }

        return new \ArrayIterator(array_reverse($articles));
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

        // Parse the article file into metatdata header and contents.
        $pos = strpos($data, '---');
        $metatdata = substr($data, 0, $pos);
        $contents = ltrim(substr($data, $pos + 3));

        // Parse header into an array of options.
        $metatdata = Toml::parse($metatdata);

        // Parse contents Markdown into HTML.
        $contents = $this->commonMarkConverter->convertToHtml($contents);

        // Parse the slug.
        $slug = str_replace('-', '/', substr($name, 0, 11)) . substr($name, 11, -3);

        $article = new Article($metatdata, $contents, $slug);
        $this->cache->set($name, $article);
        return $article;
    }
}
