+++
title = "Farewell Disqus"
author = "Stephen Coakley"
date = "2016-11-09 America/Chicago"
category = "web development"
+++

You might notice a fresh look down below in the comments. That's because I am no longer using [Disqus] to host comments on.

There's probably been plenty written about the state of Disqus, and frankly I'm not all that pleased with Disqus as of late either. Somehow it changed from a comments system to a weird, forced social network that you need to log into before commenting. What's more, Disqus has gotten some flak for tracking visitors for ad data, and I'd rather not subjugate you, dear reader, to that kind of tracking. And I've heard from more than one person that does not comment on sites using Disqus because of these problems.

I held off on removing Disqus so long because despite performance and tracking issues, at least I _had_ comments. My site is custom code and I didn't feel like implementing my own comments, but it took a while for me to find an open-source alternative that I liked.

The project I found is called [Isso] by [Martin Zimmermann] and is implemented as a standalone, self-hosted Python application. It is embeddable via JavaScript, and stores comments in a SQLite database on the server. It can run standalone using a built-in server, or as a WSGI module. Currently the built-in server satisfies my needs, but it would be easy to upgrade to a bigger setup if the traffic gets heavy.

Isso is also easy to customize since it injects semantic HTML right into the generated comments area. I've added a little CSS to the comments to blend in with my blog theme.

Since my server is an all-[Docker] system (a subject for a future post?), I implemented Isso on my site as a new Docker container. This was super easy to do since user [Wonderfall] already has [an excellent Docker image][Wonderfall/isso] based on Alpine Linux with Isso already installed and ready to go.

I haven't done any benchmarking, but my pages all load much faster and look cleaner. If you're looking for a lightweight, standalone comments system for your own site or application, mabye give Isso a try. What do you think of it?


[Disqus]: https://disqus.com
[Docker]: https://www.docker.com
[Isso]: https://posativ.org/isso/
[Martin Zimmermann]: https://github.com/posativ
[Wonderfall]: https://github.com/Wonderfall
[Wonderfall/isso]: https://hub.docker.com/r/wonderfall/isso/
