<?php
use Michelf\MarkdownExtra;

class BlogPost extends Eloquent
{
    protected $table = 'blog_post';

    public function getUpdatedDate()
    {
        $date = DateTime::createFromFormat('Y-m-d H:i:s', $this->updated_at);
        return $date->format(DateTime::ATOM);
    }

    public function getContentHtml()
    {
        return MarkdownExtra::defaultTransform($this->content);
    }

    public function getAbbreviatedContentHtml()
    {
        $content = $this->content;

        if (strlen($this->content) > 512)
            $content = substr($content, 0, 509) . '...';

        return MarkdownExtra::defaultTransform($content);
    }
}
