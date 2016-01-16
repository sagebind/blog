+++
title = "Using Sass in PHP Projects"
author = "Stephen Coakley"
date = "2014-05-22"
category = "php"
+++

If you have been paying attention to developments in the web development community recently, you probably know what CSS preprocessors are. If not, I would encourage you to check them out and play around with them. I understand they aren’t for everyone (it does require a more complex workflow), but in general it can help you keep your CSS cleaner and more modular. Check out [this awesome post](http://www.vanseodesign.com/css/css-preprocessors) by [Steven Bradley](http://www.vanseodesign.com) for a gentle introduction to CSS preprocessing.

There are several preprocessors in use out there, but I generally prefer [Sass](http://sass-lang.com) because of its extensive mixin and inheritance capabilities. I stick with the newer SCSS syntax as it’s compatible with existing CSS stylesheets and feels more familiar to me to write. I’ve been using Sass for only a few months, but I already can’t imagine writing styles without it.

The problem with using Sass is that it doesn’t integrate nicely with PHP-based projects. Sass is written in Ruby, so everyone on the team must have Ruby installed on their development machines and then the Sass gem installed. It isn’t a huge hassle, but it falls outside the project workflow. [LESS](http://lesscss.org) and [Stylus](http://learnboost.github.io/stylus/) don’t fare much better, as they both run on [Node.js](http://nodejs.org) (even though Node.js is awesome).

In order to improve the Sass workflow, I sought a better solution and I ran across [scssphp](http://leafo.net/scssphp/). scssphp is SCSS compiler written in PHP by [Leaf Forcoran](http://twitter.com/moonscript). This was exactly what I was looking for, so I downloaded it and tested it out. To test it, I used the command-line tool to try and compile my SCSS code for my website. I was pleased to see that the resulting CSS file was almost identical to the CSS file compiled by the official compiler; the only differences were minor whitespace/formatting issues that were slightly less compressed than the latter. After playing with scssphp for a few days, I decided to switch my website over to it. The change was actually painless; let me show you how to do it.

## Installing scssphp with Composer ##

First we need to download the library into our project. You can download scssphp from its website, but we are interested in its package on [Packagist](http://packagist.org). It should be no secret that I’m a big advocate of [Composer](http://getcomposer.org). I believe that it is one of the biggest changes in the community that will serve to PHP's advance into the future as a language. But that's for another blog post.

Anyway, let's get started with scssphp. First add `leafo/scssphp` as a dependency to your project's `composer.json` file:

``` json
{
    "require": {
        "leafo/scssphp": "0.0.*"
    }
}
```

The current version as of this writing is 0.0.10 (still a bit experimental). Check the library's website to determine what version is the latest and what one you should use.

After running `composer install`, scssphp should be downloaded and ready to go.

## Compiling SCSS with scssphp ##

scssphp offers multiple ways to use the compiler; the author suggests one way of using scssphp directly into your website by using the `scssphp_server` class, which serves SCSS files to the web browser as CSS on the fly. What we're going to use is the `pscss` command-line tool as a near drop-in replacement for the official compiler. This command takes SCSS as standard input and writes CSS to standard output.

My typical workflow for PHP-based projects uses [Phing](http://www.phing.info) as my central build tool. I won't go over Phing in this post; if you aren't familiar with it, its based on [Apache Ant](http://ant.apache.org) (if that helps). Here's part of what my `build.xml` file looked like originally:

``` xml
<?xml version="1.0" encoding="UTF-8"?>
<project name="stephencoakley.com" default="build" basedir=".">
    <!-- . . . -->
    <property name="scss.input" value="${dir.app}/sass/style.scss"/>
    <property name="scss.output" value="${dir.public}/css/style.css"/>
    <property name="scss.style" value="compressed"/>
    <!-- . . . -->
    <target name="styles">
        <echo>Compiling SCSS to CSS...</echo>
        <exec executable="sass" passthru="true">
            <arg line="--style ${scss.style}"/>
            <arg value="--no-cache"/>
            <arg value="--trace"/>
            <arg file="${scss.input}"/>
            <arg file="${scss.output}"/>
        </exec>
    </target>
</project>
```

Here is the newly updated build file using scssphp as my SCSS compiler:

``` xml
<?xml version="1.0" encoding="UTF-8"?>
<project name="stephencoakley.com" default="build" basedir=".">
    <!-- . . . -->
    <property name="scss.input" value="${dir.app}/sass/style.scss"/>
    <property name="scss.output" value="${dir.public}/css/style.css"/>
    <property name="scss.style" value="scss_formatter_compressed"/>
    <!-- . . . -->
    <target name="styles">
        <echo>Compiling SCSS to CSS...</echo>
        <exec
            command="${project.basedir}/vendor/bin/pscss &lt; '${scss.input}' &gt; '${scss.output}' -f=${scss.formatter}"
            dir="${project.basedir}/web/styles"
        />
    </target>
</project>
```

I'm using standard streams to pipe the main input SCSS file to the compiler's STDIN and outputting STDOUT to a CSS file. The `-f` flag indicates the output style of the CSS, similar to the official compiler's [`--style`](http://sass-lang.com/documentation/file.SASS_REFERENCE.html#output_style) flag. Note that the current directory for the command is the `styles` folder; this tells the compiler the include directory when importing other SCSS files.

And that's it! Now your projects can use SCSS without requiring Ruby installed. In my case, I can build my website by just running `phing build`. Having all project dependencies installable with Composer allows your project to be portable and easy to collaborate on without hidden dependencies in order to build.
