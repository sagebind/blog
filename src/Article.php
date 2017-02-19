<?php
namespace sagebind\blog;

use Cake\Chronos\Date;
use League\CommonMark\DocParser;
use League\CommonMark\Environment;
use League\CommonMark\HtmlRenderer;


/**
 * Value object for an article.
 */
class Article
{
    protected $document;
    protected $metatdata;
    protected $date;
    protected $slug;
    private $renderer;

    /**
     * Creates a new article object from its values.
     */
    public function __construct(array $metatdata, string $text, string $slug)
    {
        $this->metatdata = $metatdata;
        $this->date = Date::parse($this->metatdata['date'] ?? 'now');
        $this->date->setToStringFormat('F j, Y');
        $this->slug = $slug;

        // Parse the Markdown document.
        $environment = Environment::createCommonMarkEnvironment();
        $parser = new DocParser($environment);
        $this->renderer = new HtmlRenderer($environment);
        $this->document = $parser->parse($text);
    }

    /**
     * Checks if the article is published and should be listed publicly.
     */
    public function isPublished(): bool
    {
        if (isset($this->metadata['unpublished'])) {
            return !$this->metadata['unpublished'];
        }

        if (!$this->date->isToday() && $this->date->isFuture()) {
            return false;
        }

        return true;
    }

    public function title(): string
    {
        return $this->metatdata['title'];
    }

    public function date(): Date
    {
        return $this->date;
    }

    public function dateText(): string
    {
        if ($this->date->isToday()) {
            return "Today";
        }

        return $this->date->diffForHumans();
    }

    public function author()
    {
        return $this->metatdata['author'] ?? null;
    }

    public function category()
    {
        return $this->metatdata['category'] ?? null;
    }

    public function summary($length = 250): string
    {
        $html = $this->renderer->renderBlock($this->document->firstChild());
        $stripped = strip_tags($html);

        if (strlen($stripped) <= $length) {
            return $stripped;
        }

        while ($stripped[$length] !== ' ') {
            --$length;
        }

        return substr($stripped, 0, $length) . '...';
    }

    public function text(): string
    {
        return $this->renderer->renderBlock($this->document);
    }

    public function slug(): string
    {
        return $this->slug;
    }
}
