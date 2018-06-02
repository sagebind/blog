# Blog
This is the source code for my personal website and blog.

My blog doesn't use any databases, message queues, or any external services. Articles are plain [CommonMark] Markdown files located in the `articles` directory. At the top of the file, each article also has a small [TOML] header for storing metadata. The URL for each article is determined by the file name.

While most of the content has been preserved since the beginning, the code has undergone many changes over the years. Below is a high-level list of technical changes:

- _September 2012_: Simple [Laravel] app.
- _Sometime 2013_: Add a [MySQL] database to hold blog posts in HTML.
- _January 2015_: [Slim] app using plain Markdown files.
- _August 2015_: Deployment as a Docker image.
- _December 2015_: From-scratch standalone PHP app using [Icicle] as a web server and Markdown files.
- _June 2018_: [ASP.NET Core] app using Razor and Markdown files.

## License
All source code is released into the public domain. Article contents are copyright © Stephen Coakley.


[ASP.NET Core]: https://docs.microsoft.com/en-us/aspnet/core/
[CommonMark]: http://commonmark.org
[Icicle]: https://github.com/icicleio
[Laravel]: https://laravel.com
[MySQL]: https://www.mysql.com
[Slim]: https://www.slimframework.com
[TOML]: https://github.com/toml-lang/toml
