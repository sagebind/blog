+++
title = "What Are Coroutines? - Promises in PHP Survey Results"
author = "Stephen Coakley"
date = "2015-07-01"
category = "php"
+++

Hello again! As promised, I am back with this post to share with you the results of my survey about Promises in PHP, as described [in my previous post](/2015/06/24/should-we-be-using-promises-in-async-php). If you want to download a copy of the results, scroll to the bottom of this post where I have a report attached.

First off, I'd like to thank everyone who took the survey! (You know who you are.) As of this writing, 62 people took the survey, which was three times the number of people I thought would take the survey. I think it gives a little insight in what knowledge developers out there have on promises and coroutines, and what you all think of them. I am especially grateful for those who added additional thoughts in the additional comments section at the end; I sat down and read every single one.

Secondly, I've decided to not close the survey. I had announced that the survey would be closed after this post is published, but I decided that since the results are public and real-time, more people might wish to take the survey and we would be able to see how opinions change (or don't). So if you haven't taken it already, [you still can](https://stephencoakley.typeform.com/to/FBa4ga).

## Current results
Now let's discuss the results. The first question was pretty straightforward and helped establish context for the rest of the survey:

![Have you ever heard of Promises?](/content/images/2015-07-01-survey-01.png)

It looks like about 85% of you have heard of, and possibly used, promises in some form or another. I would probably attribute this to the success of promises in JavaScript land and its popularity over there. Next question:

![What do you think about the idea of Promises in PHP?](/content/images/2015-07-01-survey-02.png)

This was kind of an open-ended, opinion-based question. Those who took the survey seemed to be split in half over promises, with 53% thinking they are a good idea, and the other half not so sure. There were about 16% who repeated that they hadn't heard of promises, which makes sense. A few of you didn't like the idea in PHP, but more were indifferent. I think that a neutral position is perfectly valid, and I am glad you took the survey anyway to hear your opinion.

A few of you pointed out in this answer that promises should only be used if implemented properly in an asynchronous environment, which is an excellent point. Even if promises (or async in general) become more common in PHP, I would be appalled if everyone started using promises even in synchronous code, just to jump on the bandwagon.

### Coroutines
And now the question I'd like to address the most in this post: coroutines.

![What do you think of coroutines implemented using generators?](/content/images/2015-07-01-survey-03.png)

A significant portion of you had never heard of "coroutines". Now this doesn't surprise me, since they are not even close to mainstream in PHP, and they aren't incredibly mainstream outside PHP to my knowledge either. I'd recommend you read [Nikita Popov's article about coroutines](https://nikic.github.io/2012/12/22/Cooperative-multitasking-using-coroutines-in-PHP.html), though it is by no means a gentle introduction, which I have yet to find a good one.

Coroutines not only make writing asynchronous code simpler to read and write (in my opinion), but it also gives back the power of exception handling to your code. Last post, I presented the following example code using promises to simplify messy callbacks:

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

Firstly, I forgot error handling, so let's address that:

```php
$asyncFS = new CoolAsyncFileSystemImpl();

$asyncFS->fileGetContents('query.sql')
  ->then(function($contents) {
    return AsyncDB::query($contents);
})->then(function($result) {
    return $asyncFS->fileGetContents('http://example.com?x='.$result['x']);
})->then(function($response) {
    echo $response;
}, function(Exception $exception) {
    echo "Failed to do the cool thing I wanted to! Reason: ".$exception->getMessage();
    return $exception;
});
```

Any error that causes a promise to be rejected will be passed along to the last promise, so we add an `onRejected` callback to the end to figure out what went wrong. The downfall of promises is if you forget that last callback, the failed promise will be ignored, and any thrown exceptions will disappear into the void and you will be unaware if a major problem was encountered. Let's rewrite this again to take advantage of the awesomeness of coroutines:

```php
coroutine(function() {
    $asyncFS = new CoolAsyncFileSystemImpl();

    try {
        $contents = (yield $asyncFS->fileGetContents('query.sql'));
        $result = (yield AsyncDB::query($contents));
        $response = (yield $asyncFS->fileGetContents('http://example.com?x='.$result['x']));
    } catch (Exception $exception) {
        echo "Failed to do the cool thing I wanted to! Reason: ".$exception->getMessage();
    }
})();
```

Wrapped in a non-specific and theoretical `coroutine()` function, we can write code that is much easier for our linear brains to wrap our heads around. In addition, If we took the try/catch block out, any error thrown will bubble up and will result in an uncaught exception, which is exactly how error handling usually works.

I demonstrated some similar (though shorter) code in the survey and asked your opinions about them:

![Rate how pleasant the following code looks to you.](/content/images/2015-07-01-survey-04.png)

Ratings weren't bad overall, but it was definitely clear that not everyone was happy with how simple either method looked. To be honest, I think [async and await keywords](http://docs.hhvm.com/manual/en/hack.async.asyncawait.php) may tip the scales toward coroutines a bit more, but there may also be a third option that hasn't been suggested or even invented yet that is a better option. I'd like to find that option, but for now I think generators are the best we have so far.

Now the last question I got a few comments on:

![Does a standard Promise interface sound like a good idea?](/content/images/2015-07-01-survey-05.png)

One person commented that promises should be "implemented properly", and I'd like to determine exactly what that is. Should PHP promises follow the [Promises/A+](https://promisesaplus.com) specification, which is targeted toward JavaScript? Should there be a [PSR](http://www.php-fig.org) package for a `PromiseInterface`? Some did not like that idea, but it would solve incompatibility issues between implementations, like the gap between ReactPHP and Guzzle promises. Issues like these have yet to be decided on.

## Conclusion
While the results of the survey can be concluded, I think the search for the best way to design asynchronous PHP code will not conclude for some time yet. Whether we use promises, coroutines, or some other kind of future API, we should definitely be investing in asynchronous PHP. I think that event-based code that is easily adapted to run on multiple threads, processes, or even computers is part of the future of computing as a whole, and each time we invent ways to enable our code to run more concurrently, we contribute to the effort to move toward that future.

If you'd like to save the results of the survey for your own records, you can download a PDF version [here](/content/2015-07-01-promises-in-php-survey-results.pdf).

As always, feel free to let me know what you think in the comments below, or reach out to me on [Twitter](http://twitter.com/sagebind).
