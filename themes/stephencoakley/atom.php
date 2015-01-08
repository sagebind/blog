<?php
header("Content-Type: application/atom+xml");

if ($articles) {
    reset($articles);
    $key = key($articles);
    $lastBuildDate = date('c', strtotime($articles[$key]->getDate()));
   // create simplexml object
    $xml = new SimpleXMLElement('<?xml version="1.0" encoding="UTF-8" ?><feed xmlns="http://www.w3.org/2005/Atom" />', LIBXML_NOERROR|LIBXML_ERR_NONE|LIBXML_ERR_FATAL);

    // add channel information
    $xml->addChild('title', $global['site.name']);

    $link = $xml->addChild('link');
    $link->addAttribute("href", "http://" . $_SERVER['HTTP_HOST']);

    $link = $xml->addChild('link');
    $link->addAttribute("href", "http://" . $_SERVER['HTTP_HOST'] . "/feed/atom.xml");
    $link->addAttribute("rel", "self");

    $xml->addChild('subtitle', $global['site.title']);
    $xml->addChild('updated', $lastBuildDate);
    $xml->addChild('id', "http://" . $_SERVER['HTTP_HOST'] . "/feed/atom");
    $author = $xml->addChild("author");
    $author->addChild("name", "Stephen Coakley");
    $author->addChild("email", "me@stephencoakley.com");

    foreach ($articles as $article) {
        $entry = $xml->addChild('entry');
        $entry->addChild('title', $article->getTitle());
        $link = $entry->addChild('link');
        $link->addAttribute("href", $article->getUrl());
        $entry->addChild('id', $article->getUrl());
        $entry->addChild("summary");
        $entry->summary = "<![CDATA[" . $article->getContent() . "]]>";
        $entry->summary->addAttribute("type", "html");
        $entry->addChild('updated', date('c', strtotime($article->getDate())));
    }

    // output xml
    echo $xml->asXML();
}
