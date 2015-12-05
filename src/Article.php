<?php
namespace coderstephen\blog;

class Article
{
    private $metatdata;
    private $date;
    private $content;
    private $slug;

    public function __construct(array $metatdata, string $content, string $slug)
    {
        $this->metatdata = $metatdata;
        $this->date = new \DateTimeImmutable($this->metatdata['date'] ?? 'now');
        $this->content = $content;
        $this->slug = $slug;
    }

    public function title(): string
    {
        return $this->metatdata['title'];
    }

    public function date(): \DateTimeImmutable
    {
        return $this->date;
    }

    public function dateString(): string
    {
        return $this->date->format("F n, Y");
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
        return preg_replace('/\s+?(\S+)?$/', '', substr(strip_tags($this->content), 0, $length));
    }

    public function content(): string
    {
        return $this->content;
    }

    public function slug(): string
    {
        return $this->slug;
    }
}
