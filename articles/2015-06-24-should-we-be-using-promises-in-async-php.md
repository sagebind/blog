+++
title = "Should We Be Using Promises in Async PHP?"
author = "Stephen Coakley"
date = "2015-06-24 America/Chicago"
category = "php"
+++

I'd like to talk briefly about a new survey that I'm trying to distribute. It's nothing amazing, just a really short poll to try and help me gage people's opinions using Promises for handling asynchronous code. Let me explain why this survey is important.

I am currently working on an architectural redesign for [Evflow](http://github.com/evflow/evflow), an experiment in asynchronous code with PHP. I started the project mid-2014 and a lot has changed since then. PHP 5.6 was released in August 2014, and PHP 7 is in alpha and comes with it a [host of new features and improvements](https://blog.engineyard.com/2015/what-to-expect-php-7). Evflow's current architecture is a bit flimsy and mostly consists of random ideas glued together.

## We need promises!
I'd love to talk your ear off about all the exciting, cool new things I have in the works for Evflow (including a better name!), but that is not the subject of this post. Instead, let's consider promises for a moment. For those of you who don't know what a "promise" is, it is like an alternative to callbacks for delayed functions. I'd recommend reading an article such as [this one](http://www.html5rocks.com/en/tutorials/es6/promises/#toc-async) to get up to speed.

Promises have taken the JavaScript world by storm, but what does this have to do with PHP? Well, JavaScript *needs* something like promises to avoid callback hell, since a large number of things in JavaScript are already asynchronous. If we want to create asynchronous systems in PHP, which me and [others as well](https://medium.com/@assertchris/a-case-for-async-php-f33e5e31ebba) are convinced of doing, it would be prudent to learn from the mistakes of other languages and get it right the first time. Consider the following code:

```php
$asyncFS = new CoolAsyncFileSystemImpl();

$asyncFS->fileGetContents('query.sql', function($contents) {
    AsyncDB::query($contents, function($result) {
        $asyncFS->fileGetContents('http://example.com?x='.$result['x'], function($response) {
            echo $response;
        });
    });
});
```

This contrived example demonstrates this "callback hell" that we are trying to avoid. Promises, on the other hand, avoid this to some extent (not that you can't write bad code with promises). Here is the above code rewritten to use promises:

```php
$asyncFS = new CoolAsyncFileSystemImpl();

$asyncFS->fileGetContents('query.sql')
  ->then(function($contents) {
    return AsyncDB::query($contents);
})->then(function($result) {
    return $asyncFS->fileGetContents('http://example.com?x='.$result['x']);
})->then(function($response) {
    echo $response;
});
```

Much cleaner, don't you think? This is why promises are so popular and projects like [ReactPHP](http://reactphp.org) use them as their callback mechanism of choice.

## We need promises... or do we?
But are promises really the answer? With things like generators and [coroutines](https://nikic.github.io/2012/12/22/Cooperative-multitasking-using-coroutines-in-PHP.html), some have suggested that typical promises [aren't the best solution](https://github.com/amphp/amp/blob/master/guide.md#promises) for PHP. I haven't decided myself, so I decided to create this survey to see what you as the community thinks. You can click the link below to take the survey:

**[Take the survey!](https://stephencoakley.typeform.com/to/FBa4ga)**

I will analyze the survey results and post my response here when the survey ends, which will be in one week from now, on July 1. Feel free to let me know what you think in the comments below.

---

**Update:** I have analyzed and discussed the survey results and posted them [here](/2015/07/01/what-are-coroutines-promises-in-php-survey-results), but the survey is still open if you'd like to take it.
