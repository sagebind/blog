+++
title = "Dumb Reasons to Hate PHP"
author = "Stephen Coakley"
date = "2020-06-10"
tags = ["php"]
+++

[PHP] just recently celebrated its 25th anniversary since it was first introduced, which is quite the achievement, considering it still powers a large slice of the Internet today. I don't write much PHP anymore myself as I've more or less moved on to new and different things, but I am incredibly grateful to PHP. It was one of the first "real" programming languages I really invested in to learn programming, and learn I did. I built real things, real websites with it, and also was involved in the community for a while. I saw the rise of [Composer](https://getcomposer.org/) and [Packagist](https://packagist.org/) replace the aging PEAR. I saw the release of PHP 7 and all the work that went into it the years prior leading up to it.

Now as expected whenever talking about PHP on the Internet, people are quick to grab their pitchforks and rehash the same classic criticisms of PHP over and over like a mantra. Is it to feel superior? Do they think they're doing a public service? I don't know. What I _do_ know is that they're right to some extent; PHP isn't the best-designed language by any means, largely because it changed organically and incrementally over time. It certainly hasn't stopped PHP from becoming as popular as it has.

There are plenty of good reasons why PHP isn't the best language for many use-cases, and reasons why other languages are superior. I consider myself very experienced with it, so I speak from experience. Here are some examples just from memory:

- The standard library, while fairly complete, doesn't really follow modern PHP's own best practices for API design, as it was largely created before PHP had things like namespaces and classes. This results in an odd disconnect with modern packages, and that weird mix of styles never really goes away.
- The standard library also cares a lot about backwards compatibility, which is a good thing, but its also a double-edged sword. There are a lot of APIs and extensions that are soft-deprecated or generally not used in favor of higher-quality third-party packages.
- The fact that every class file begins with `<?php` reminds you that PHP was originally _just_ an HTML preprocessor and always runs inside the context of another file format. It makes sense, but its unusual and weird, especially since embedding PHP into HTML isn't even done at all in many frameworks which have dedicated templating languages instead.

There are probably others, but these don't keep me from remembering PHP with fondness as something that just works out of the box and has a lot of convenient features for web development.

What's strange to me though is that instead of reasonable complaints like these, people like to present complaints that don't make sense, aren't true, or are just plain silly. Let's take a look at just a couple that I've seen.

## The syntax is strange and archaic!

This complaint doesn't really make much sense to me. PHP's syntax is very heavily inspired by C (which it is written in) and borrows many things from it. In fact, it fits right in with most of the languages in the C family of syntax. Just swap the dot operator for `->` (which by the way is also lifted from C, its equivalent to `(*struct_ptr).field`), prepend all your variables with the `$` sigil, and that's just about it. It's got your boring traditional `class` syntax that even JavaScript adopted, closures, and pretty much every modern convenience.

Granted, sigils probably remind you of Perl, but don't worry, they don't have crazy effects on data types like in Perl. Just think of it as part of the variable name and you'll be fine.

There are a few PHP-specific oddities in its syntax, like the `@` operator and using `\` as a namespace separator, but these seem like really petty nitpicks to me.

## It isn't modular!

This sort of complaint is really nebulous, and could stand to have some clarifying questions asked. Usually people mean one of two things:

1. Everything is in a global namespace with no modular separation.
2. There is no modular way of packaging code.

Now both of these are just blatantly false. The first one is easy: PHP has [namespaces], like Java, C#, or what-have-you. And they were added to the language in version 5.3, which was released in 2009! Now to be fair, there still exists a lot of codebases that were initially designed before then (like WordPress) that don't leverage namespaces everywhere because of this, and this includes the standard library itself. But generally namespaces have been adopted for some time, and any modern PHP codebase uses them well.

The second complaint is also false, but has a seed of truth in it. Today, PHP has the aforementioned [Packagist], with tons of reusable, modular packages. Its pretty easy to publish your own too! But before Packagist and Composer was [PEAR], which admittedly was frustrating to use and probably _did_ steer people clear of making or using third-party components. So this complaint probably would have nailed home in 2012, but today its just really misinformed.

As an aside, I want to mention that Composer not only exists, but is simply a well-designed package manager that is pleasant to use. It makes way more sense than the defacto package managers for most languages I've used; the only one I've used that I think I like better is [Cargo].

## It's really slow!

This is a deep-rooted one that is probably based on a kernel of truth. Originally, PHP operated through something called [Common Gateway Interface], or CGI, which is basically just a standardized way of invoking subprocesses and using their standard output as an HTTP response.

Now CGI _is_ pretty darn slow, because it spawns a whole new process for every single request! It wasn't much of a problem though when it was first introduced, and honestly I kinda miss the simplicity and portability of CGI. I think the modern IPC equivalent might be using JSON-RPC over standard pipes, which is a pretty cool technique. Anyway, back to the subject of performance.

To fix the slowness of CGI, one approach that was taken and became immensely popular was to integrate PHP into the web server in such a way that allowed you to keep one or more PHP interpreter instances initialized at all times for running scripts, which means you don't need to spawn a new process or initialize an interpreter for each request. [Apache HTTP Server] is very popular for the `mod_php` module that does this.

Another approach is to use something called [FastCGI], whose only relation to CGI is its purpose of integrating applications and web servers. Instead of subprocesses, FastCGI is a fast binary protocol that allows a web server to communicate with a daemonized application to serve requests. This approach is almost on-par with the common modern solution of making the application _be_ the web server (which is also possible with PHP by the way) and is also a popular way of deploying PHP via [php-fpm].

Both of these methods improved performance by removing wasteful steps, like spawning new processes and initializing PHP every request. Needless to say, plain CGI has been mostly abandoned for these alternatives. But not everything is fixed by this, because in both these models, PHP _scripts_ are still short-lived. Any variables you create or resources you open during a request are cleaned up at the end of the request, which for non-trivial code means a lot of duplicated work each request. This again is a cause of slowness, especially for large frameworks.

Now surprisingly, this doesn't really matter at all most of the time, because believe it or not, PHP is fast. Much faster than it has any business being, considering the nature of the language being interpreted. One of the big parts of PHP 7 was a huge refactoring of the internal Zend engine that delivered a significant performance increase of the core language. It [usually outperforms](https://benchmarksgame-team.pages.debian.net/benchmarksgame/fastest/php-python3.html) languages like Python and Ruby. When PHP 7 was first released, it even outperformed Node.js in some benchmarks, though since then Node.js passed PHP with all the constant optimizations Google does on the V8 engine.

To further improve overall performance, PHP offers persistent database connections that live across requests, which connecting to a database is 99% of the time the slow part of a script. And to avoid parsing scripts repeatedly, PHP ships with an opcache that caches the "compiled" opcode of each script so it can be run directly on subsequent requests.

There's also interesting projects like [Amp] which take advantage of PHP's core interpreter performance by eliminating the middleman and running a web server directly in your PHP application, similar to the Node.js model, which performs surprisingly well.

Python and Ruby aren't exactly the fastest kids on the block by any means, but they're not usually disdained because of it, so if Python's performance doesn't bother you, then PHP's certainly shouldn't.

---

None of these comments are really arguments _for_ PHP. As I mentioned earlier, I don't really write any PHP any more in favor of other things, primarily Rust and C# personally and Java professionally. This was mostly for the fun of dissecting silly arguments and playing devil's advocate.

If you'd like to shout at me about how wrong I am though, go ahead in the comments below. Just please be civil while exacting your revenge. Thanks.


[Amp]: https://amphp.org/
[Apache HTTP Server]: https://httpd.apache.org/
[Cargo]: https://doc.rust-lang.org/cargo/
[Common Gateway Interface]: https://en.wikipedia.org/wiki/Common_Gateway_Interface
[Composer]: https://getcomposer.org/
[FastCGI]: https://en.wikipedia.org/wiki/FastCGI
[namespaces]: https://www.php.net/manual/en/language.namespaces.php
[Packagist]: https://packagist.org/
[PEAR]: https://pear.php.net/
[PHP]: https://www.php.net/
[php-fpm]: https://www.php.net/manual/en/install.fpm.php
