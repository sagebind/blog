<?php
error_reporting(E_ALL);
require_once 'google-api-php-client/src/Google_Client.php';
require_once 'google-api-php-client/src/contrib/Google_PlusService.php';

// google api
$client = new Google_Client();
$client->setApplicationName('Google+ IFTTT Channel');
$client->setDeveloperKey('AIzaSyCbnW4QM5akrYQGmMNmGl5ZnQ6ZvBnORk8');

// google+
$plus = new Google_PlusService($client);
$activities = $plus->activities->listActivities('103202132384011460527', 'public');

// rss document
$document = new DOMDocument('1.0');
$rss = $document->appendChild($document->createElement('rss'));
$rss->setAttribute('version', '2.0');

// channel
$channel = $rss->appendChild($document->createElement('channel'));
$channel->appendChild($document->createElement('title', 'CoderStephen - Public posts'));
$channel->appendChild($document->createElement('link', 'http://gplus.to/CoderStephen'));
$channel->appendChild($document->createElement('description', 'Programmer, tech junkie, artist and photographer, guitarist and musician, lover of literature, Christ-follower.'));

// items
foreach ($activities['items'] as $activity)
{
    if (!isset($_GET['type']) || $activity['object']['objectType'] === $_GET['type'])
    {
        $pubDate = new DateTime($activity['published']);

        $item = $channel->appendChild($document->createElement('item'));
        $item->appendChild($document->createElement('title', $activity['title']));
        $item->appendChild($document->createElement('link', $activity['url']));
        $item->appendChild($document->createElement('guid', $activity['id']));
        $item->appendChild($document->createElement('pubDate', $pubDate->format(DateTime::RSS)));
        $item->appendChild($document->createElement('description', strip_tags(stripcslashes($activity['object']['content']))));
    }
}

// write document
header('Content-Type: application/rss+xml');
print $document->saveXML();