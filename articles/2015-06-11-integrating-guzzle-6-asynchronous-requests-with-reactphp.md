+++
title = "Integrating Guzzle 6 Asynchronous Requests with ReactPHP"
author = "Stephen Coakley"
date = "2015-06-11 America/Chicago"
category = "php"
+++

One of my current "toy" side-projects at the moment is [a better PHP API client for Slack](http://github.com/sagebind/slack-client). There are a handful of incomplete ones already on [Packagist](https://packagist.org/search/?q=slack), but I decided to add another one to the list anyway. It uses [Guzzle](https://github.com/guzzle/guzzle) for making regular API calls, and [PHPWS](https://github.com/Devristo/phpws) (a WebSocket library) for connecting to Slack's real-time messaging API. It's actually a pretty cool project so far, though it still is under construction.

One of the interesting problems I ran into while writing this library was how to make API calls and connect to WebSockets simultaneously. To be as snappy as possible, I wanted the entire library to be asynchronous, so I decided to use [ReactPHP](http://reactphp.org) at the core since it is reasonably mature project for asynchronous processing. This would allow you to both connect to Slack and do other things at once, such as you might need in a chat bot. PHPWS also uses React, so instant profit! Guzzle, however, has its own take on async operations, so I set out to make Guzzle and React become friends.

## Promising a response
One of the core features of Guzzle 6 is an implementation of promises. For those of you that are unfamiliar with the concept of promises, here is [an excellent article](http://www.sitepoint.com/overview-javascript-promises/) that gives you a gentle introduction (though it is targeted toward JavaScript). To summarize, a promise is an object that represents some value that will be determined in the future. Callbacks can be attached to the promise using a `then()` method which will be triggered when the promise's value becomes available. Here is a quick example on how to make an asynchronous Guzzle request:

```php
use GuzzleHttp\Client;
use GuzzleHttp\Exception\RequestException;
use Psr\Http\Message\ResponseInterface;

$client = new Client();
$client->getAsync('http://loripsum.net/api')->then(
    function (ResponseInterface $res) {
        echo $res->getStatusCode() . "\n";
    }, function (RequestException $e) {
        echo $e->getMessage() . "\n";
        echo $e->getRequest()->getMethod();
    }
);
```

In this example, `$client->getAsync()` returns a `GuzzleHttp\Promise\Promise` instance, promising to give us a response object when the request is complete. We call `then()` to register interest in the result, passing two functions: (1) a callback that accepts the promised value if the promise resolves, and (2) a callback that accepts an error value if the promise is rejected.

### Using React's promise interface
React has [its own promises implementation](https://github.com/reactphp/promise), which React-based libraries (such as my Slack client) usually use to return values asynchronously. This can create some problems when using Guzzle's promises *and* React promises, because they are not compatible with each other. While they both have the all-important `then()` method, they don't play nicely together because they don't implement each other's interfaces. The best way to fix this is to convert Guzzle's promises into React promises, which are a bit more library-agnostic and more common. In addition, using React promises everywhere allows you to easily chain other React promises from the growing number of React-powered libraries.

My first attempt at solving this problem looked something like this:

```php
use GuzzleHttp\Promise\Promise as GuzzlePromise;
use GuzzleHttp\Promise\PromiseInterface as GuzzlePromiseInterface;
use React\Promise\PromiseInterface as ReactPromiseInterface;

class Promise extends GuzzlePromise implements ReactPromiseInterface, GuzzlePromiseInterface {
    public function then(callable $onFulfilled = null, callable $onRejected = null, callable $onProgress = null)
    {
        return parent::then($onFulfilled, $onRejected);
    }
}
```

Yep, that's what it looks like -- a Frankenstein's hybrid of Guzzle and React promises. The above example was just the beginning; it turned out that far too much work would be required down this path because the way promises are processed is fundamentally different between Guzzle and React.

Instead of that approach, I found a solution that is much simpler: create a React promise, and resolve it when the Guzzle promise resolves. Let me show you how easy it is:

```php
use GuzzleHttp\Client;
use GuzzleHttp\Exception\RequestException;
use Psr\Http\Message\ResponseInterface;
use React\Promise\Deferred;

function getLoremIpsum()
{
    $deferred = new Deferred();
    $client = new Client();
    $promise = $client->getAsync('http://loripsum.net/api');

    $promise->then(function (ResponseInterface $response) use ($deferred) {
        $deferred->resolve((string)$response->getBody());
    }, function (RequestException $e) use ($deferred) {
        $deferred->reject($e);
    });

    return $deferred->promise();
}
```

Here we defined a function, `getLoremIpsum()`, which returns a React promise that resolves when the asynchronous request completes, using a [*deferred* object](https://github.com/reactphp/promise#deferred-1). It turns out that this pattern of converting Guzzle and React promises is similar in almost all situations. We could then write a convenient function to do this for us. Below is a basic implementation for such a function and its usage:

```php
use GuzzleHttp\Promise\PromiseInterface as GuzzlePromise;
use React\Promise\Deferred;

function guzzleToReactPromise(GuzzlePromise $promise)
{
    $deferred = new Deferred();
    $promise->then(function ($value) {
        $deferred->resolve($value);
    }, function ($error) {
        $deferred->reject($error);
    });
    return $deferred->promise();
}
```

Let's try rewriting `getLoremIpsum()` with this new function in hand:

```php
function getLoremIpsum()
{
    $client = new Client();
    return guzzleToReactPromise($client->getAsync('http://loripsum.net/api'))
    ->then(function (ResponseInterface $response) {
        return (string)$response->getBody();
    });
}
```

It makes our `getLoremIpsum()` function much clearer to write and more concise. With something like this function, you can make sure all your promises are React-compatible, which still being able to use the excellent capabilities of Guzzle to send web requests.

## Waiting on requests
The tricky thing about asynchronous operations is that they have to be executed *sometime* during the program, or you will never get a result. Guzzle uses an internal `TaskQueue` object to keep track of unfulfilled promises and tasks that are yet to be completed. By default, Guzzle deals with creating and running this queue automatically as needed. Consider the following asynchronous example:

```php
use GuzzleHttp\Client;
use GuzzleHttp\Promise;

$client = new Client();

// Fetch some data
$loremPromise = $client->getAsync('http://loripsum.net/api');
$randomPromise = $client->getAsync('https://www.random.org/sequences/?min=1&max=52&col=1&format=plain');

// Wait on all of the requests to complete
$results = Promise\unwrap([$loremPromise, $randomPromise]);
```

In this example, Guzzle waits until you absolutely need a result before blocking execution and waiting for a response. `Promise\unwrap` is just a fancy Guzzle function that waits for a promises in an array to be resolved before continuing. If you only have one promise, you can similarly call `$promise->wait()` to wait for that promise only.

Now, this is a really nifty way of making asynchronous HTTP calls easy, and Guzzle is extremely robust. The issue arises when you have a list of HTTP requests you are waiting for, but suddenly need to address something else -- user keystrokes, a callback timer, incoming nuclear warheads, etc. React can be configured to be interrupted by some of these types of events, but not Guzzle, which only deals with HTTP requests. To be friendly to *all* kinds of notable events, we need to shift the responsibility of waiting for Guzzle requests to a React event loop.

### Using a React event loop -- the naïve approach
A React event loop, like Guzzle's `TaskQueue`, is like a big list of things to do, streams to watch, and things to wait for, which automatically cycles through and handles things in the order they come. Check [this article](http://blog.wyrihaximus.net/2015/02/reactphp-event-loop/) for a brief introduction of the event loop.

Now a functional, but ultimately naïve, way is to simply schedule the `wait`ing on of requests in the event loop. This can be done fairly simply when you make the request:

```php
// Make the request
$promise = $client->getAsync('http://loripsum.net/api');

// Schedule the request to be force-resolved later
$loop->futureTick(function () use ($promise) {
    $promise->wait();
});

// Handle the response
$promise->then(function (ResponseInterface $response) {
    return (string)$response->getBody();
});
```

On the surface, this looks great! In fact, at the time of this writing, this is how my Slack client is [working around the problem](https://github.com/sagebind/slack-client/blob/v0.1.1/src/ApiClient.php#L147). Requests actually get handled, React's event loop isn't overtly locked, and other scheduled tasks still run. All isn't dragons and unicorns, however. The event loop is still being halted synchronously to wait for each request when the time comes for `wait()` to be called. Even worse, requests are waited for in the order they are made and not in the order that the responses are received. So, we need a better solution.

### Using a React event loop -- a better way
The most reliable way is to take advantage of Guzzle's use of cURL multi handles to integrate cURL into React. Now, this requires that you use the `CurlMultiHandler` [handler](http://guzzle.readthedocs.org/en/latest/handlers-and-middleware.html). Since we need direct access to the handler instance, we need to create Guzzle's handler manually:

```php
use GuzzleHttp\Client;
use GuzzleHttp\HandlerStack;
use GuzzleHttp\Handler\CurlMultiHandler;

$handler = new CurlMultiHandler();
$client = new Client([
    'handler' => HandlerStack::create($handler)
]);
```

Of course, since we're using Guzzle's cURL handler, you must have the cURL extension installed, and you can't use a different handler for this method, no matter how shiny.

Now that we have access to Guzzle's handler, we can control the internals of Guzzle a bit more. Fortunately, the light at the end of the tunnel is visible! All that remains is to tell the event loop to regularly give our handler a small patch of processing time to handle any requests made by the Guzzle client as they finish. `CurlMultiHandler` provides a helpful (but undocumented) `tick()` method which checks only once for completed requests, and takes action on them. We can tell the event loop to periodically call `tick()` using a timer:

```php
$loop = \React\EventLoop\Factory::create();

$timer = $loop->addPeriodicTimer(0, \Closure::bind(function () use (&$timer) {
    // Do a smidgen of request processing
    $this->tick();
    // Stop the timer when there are no more requests
    if (empty($this->handles) && Promise\queue()->isEmpty()) {
        $timer->cancel();
    }
}, $handler, $handler));

$loop->run();
```

Here we create a periodic timer that calls `tick()` on the cURL handler. Using some naughty [closure bindings](http://php.net/manual/en/closure.bind.php), we then access the handler's private `$handles` array and the Guzzle task queue mentioned above to check if there are more requests to handle. Since we probably don't want to run the loop indefinitely, we cancel the timer when there are no more pending requests.

Now that the actual request handler and the event loop are connected, any Guzzle client using the connected handler will have its requests managed by the event loop. Now when we start the event loop, Guzzle's internal loop will be periodically polled and requests will be handled in parallel, truly asynchronously.

## Putting it all together
Now let's put it all together into a complete example. Below is a simple program that sends a request asynchronously and displays the response body, using React as the event loop:

```php
use GuzzleHttp\Client;
use GuzzleHttp\HandlerStack;
use GuzzleHttp\Handler\CurlMultiHandler;
use GuzzleHttp\Promise;
use Psr\Http\Message\ResponseInterface;
use React\EventLoop\Factory;

// Create a React event loop
$loop = Factory::create();

// Create a Guzzle handler that integrates with React
$handler = new CurlMultiHandler();
$timer = $loop->addPeriodicTimer(0, \Closure::bind(function () use (&$timer) {
    $this->tick();
    if (empty($this->handles) && Promise\queue()->isEmpty()) {
        $timer->cancel();
    }
}, $handler, $handler));

// Create a Guzzle client that uses our special handler
$client = new Client([
    'handler' => HandlerStack::create($handler),
]);

// Send a request and handle the response asynchronously
$client->getAsync('http://loripsum.net/api')
->then(function (ResponseInterface $response) {
    echo 'Response: '.$response->getBody();
});

// Run everything to completion!
$loop->run();
```

We finally made it to the end! This article was rather long, but hopefully it contains some helpful information. If you have been using Guzzle and React or can think of alternate ways to integrate them, comment below and share your experiences and ideas!
