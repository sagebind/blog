# Blog
This is the source code for my blog. It is a standalone PHP application that is also its own web server. At its core is [`icicleio/http`](http://github.com/icicleio/http), which provides an asynchronous, pure PHP web server.

## Overview
My blog doesn't use any databases, message queues, or any external services. Articles are plain [CommonMark Markdown](http://commonmark.org) files located in the `articles` directory. At the top of the file, each article also has a small [TOML](https://github.com/toml-lang/toml) header for storing metadata. The URL for each article is determined by the file name.

Every page is rendered through one or more [Mustache](https://mustache.github.io) templates, which can be found in the `templates` directory.

The server itself is managed by an instance of [`sagebind\blog\Application`](src/Application.php), which uses the excellent [FastRoute router](https://github.com/nikic/FastRoute) by [Nikita Popov](https://github.com/nikic) to dispatch requests to a designated *action*, which is an object that handles a single request and uses the renderer to render templates and views. Actions are available under the [`sagebind\blog\actions`](src/actions) namespace.

## Running the server
Running the application server couldn't be easier. It is as simple as executing

    $ bin/server

By default the server will listen on port `8080`.

## Building
The site uses [Sass stylesheets](http://sass-lang.org), so before you can view the site properly, you will need to compile the assets. This can be done with

    $ vendor/bin/robo build

You will need to have Sass installed first for this to work. See [their website](http://sass-lang.com) for installation instructions.

This site runs as a [Docker](https://www.docker.com) container in production. To build the appropriate container, you can simply run

    $ vendor/bin/robo build:docker
