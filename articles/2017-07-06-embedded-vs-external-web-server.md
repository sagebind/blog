+++
title = "Embedded vs. External Web Servers"
author = "Stephen Coakley"
date = "2017-07-06"
tags = ["webdev"]
+++

*Firstly, Twitter's 140 character limit is too short for sharing thoughts longer than one or two sentences. I regularly question my development practices and sharing my internal debates is a good way for me to analyze the arguments, but I tend to sound incoherent when I use Twitter for that purpose. What better place to do so than on this blog? Expect to see more frequent, albeit shorter, posts like this one from now on. Now on to the post.*

Recently I have been examining the different models of web application architecture at the server level. I'm experimenting with my own web "framework" (API glue really) for Rust, and I notice some very different approaches in Rust web development than in some other languages. Rust frameworks like [Iron] provide the HTTP server as a library, and your web application runs the server in-process. This is quite different than the classic "web server + CGI" model that so many frameworks were designed around in the last 20 years.

There's really two main models that you see in the wild. For lack of existing terms (this stuff is hard to search for), I'm calling them "embedded servers" and "external servers". So we are on the same page, here's what I understand the differences to be:

With an embedded server:

- HTTP server runs inside the same process space as your application.
- Your application is responsible for starting the server.
- Your application is responsible for configuring the server, often programmatically.

With an external server:

- HTTP server runs separately from your application.
- The server might be responsible for loading your application.
- The server forwards requests to your application.
- The server is configured using separate config files.

Certain programming languages and frameworks seem to have a disposition to one or the other of these models. PHP was originally designed to be used with CGI (Common Gateway Interface) behind a web server like Apache, very much the external model. Node.js was designed from the ground up around the embedded model. Java's [servlet API](https://docs.oracle.com/javaee/7/api/javax/servlet/http/package-summary.html) is focused on the external model, although running a Java web server embedded into your app [is becoming pretty common](http://www.eclipse.org/jetty/documentation/current/embedding-jetty.html).

But which model is _better_? Both probably have their uses, so wrestling over which is better is silly. A better question is: what are the pros and cons of each?

Embedded server pros:

- More self-contained applications. This helps a lot during development.
- As a dependency of your application, you can test against server versions just like any other dependency.
- More control over how the web server behaves (custom filters, headers, caching).
- Single object to be deployed.

Embedded server cons:

- Your application has to be designed around the API of whatever server you are using, making it harder to change servers later. (Java doesn't really have this problem, as you can still use the servlet API when embedding.)
- Dependency bloat, as you have to include all the dependencies of the web server.
- More effort to deploy hotfixes to security exploits in the server.
- You can't group multiple applications behind one server without a proxy.
- A single uncaught exception is enough to take down the entire application server.

External server pros:

- Potentially more flexible application architecture.
- Really easy to switch servers later.
- Application errors can't harm the server.
- Easy to deploy app updates without restarting the server.
- Performance and correctness: servers like [nginx] are _highly_ optimized and tested for complete HTTP correctness, which your app then gets for free.

External server cons:

- Extra performance overhead: there could be anything from an extra layer of method abstraction up to CGI-level overhead for your app and the server to communicate.
- Deployment complexity: you have to maintain the web server and the application, deploy them individually, ad hoc version testing, etc.
- Trickier development environment.

I think people tend to exaggerate the overhead of communicating with an external server, especially if you're using something like [FastCGI] or [ISAPI] which perform very well. Even so, the bottleneck is almost always your application code and not the HTTP server. Unless you are writing a performance-critical, timing-sensitive web service (which you probably are not), I don't find the performance argument for an embedded server very convincing.

The embedded model definitely makes sense for programs with a web-based interface (routers, daemons, etc.), as setting up a web server in addition to your software seems like overkill when the web interface isn't the product itself. For web applications though, requiring an external server seems like a normal requirement (maybe even a feature of choice), but maybe it's because we're used to it by now.

This isn't conclusive evidence for one way or another, but I am currently very interested in making the external model easier to do in Rust. I'm also curious to hear what other people think about this. Maybe I don't hang around the right people, but this seems to be a topic that is under-discussed and everyone just "assumes" different things.


[FastCGI]: https://en.wikipedia.org/wiki/FastCGI
[Iron]: http://ironframework.io
[ISAPI]: https://en.wikipedia.org/wiki/Internet_Server_Application_Programming_Interface
[nginx]: https://nginx.org/en/
