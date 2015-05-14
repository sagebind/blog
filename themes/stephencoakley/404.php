<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Page not found - <?=$global['site.name']?></title>
        <meta name="description" content="<?php echo $global['site.description']; ?>">
        <meta name="keywords" content="Stephen Coakley, programming, web development, apps, php">
        <meta name="author" content="<?=$global['author.name']?>">
        <link href="/feed/atom" type="application/atom+xml" rel="alternate" title="Blog Feed">

        <link rel="icon" type="image/png" href="<?=$global['assets.prefix']?>/images/favicon.128.png" sizes="128x128">
        <link rel="icon" type="image/png" href="<?=$global['assets.prefix']?>/images/favicon.64.png" sizes="64x64">
        <link rel="icon" type="image/png" href="<?=$global['assets.prefix']?>/images/favicon.48.png" sizes="48x48">
        <link rel="icon" type="image/png" href="<?=$global['assets.prefix']?>/images/favicon.32.png" sizes="32x32">
        <link rel="shortcut icon" href="/favicon.ico">
        <link rel="icon" type="image/png" href="<?=$global['assets.prefix']?>/images/favicon.16.png" sizes="16x16">
        <link rel="apple-touch-icon" href="<?=$global['assets.prefix']?>/images/favicon.128.png">
        <meta name="msapplication-TileColor" content="#D83434">
        <meta name="msapplication-TileImage" content="<?=$global['assets.prefix']?>/images/favicon.128.png">

        <meta name="viewport" content="initial-scale=1">
        <link rel="stylesheet" href="<?=$global['assets.prefix']?>/css/style.css">

        <!--[if lt IE 9]>
        <script src="<?=$global['assets.prefix']?>/js/html5shiv.min.js"></script>
        <![endif]-->
        <script src="<?=$global['assets.prefix']?>/js/highlight.pack.js"></script>
        <script>hljs.initHighlightingOnLoad();</script>
    </head>

    <body id="top">
        <header class="page-header">
            <p class="center">
                <a class="site-logo no-underline" href="/" rel="home">
                    <img src="<?=$global['assets.prefix']?>/images/logo-white.svg">
                    <span class="site-logo-text">Stephen Coakley</span>
                </a>
            </p>

            <p class="bio-text">Disciple of Christ, software developer, PHP enthusiast, techie, guitarist and musician, amateur photographer, lover of literature.</p>

            <nav>
                <a class="button" href="/">Home</a>
                <a class="button" href="/portfolio">Portfolio</a>
                <a class="button" href="/blog">Blog</a>
            </nav>
        </header>

        <main role="main">
            <div class="content">
                <h1>This is not the page you are looking for.</h1>

                <p><img src="<?=$global['assets.prefix']?>/images/yoda.png" style="border: none;"></p>

                <p class="big">Found, the page you requested, was not. Perhaps, try another link at the top or left you will, hmm? Since came so far, you have, enjoy a random ASCII string, you will.</p>

                <pre><code class="json" style="word-wrap:break-word;">"<?php
                    for ($i = 0; $i < 1000; $i++) {
                        $char = chr(rand(32, 126));
                        if ($char !== '<') echo $char;
                    }
                ?>"</code></pre>

                <p>Those darned Insecticons must be eating my hard drives again...</p>
            </div>

            <a class="button top-link fa fa-arrow-up" href="#top"></a>

            <footer class="page-footer">
                <p class="page-copyright">All content and design copyright &copy; Stephen Coakley.</p>
                <p>Hosted by my lovely provider <a href="http://hostek.com">Hostek.com</a>.</p>
            </footer>
        </main>

        <script>
            var _paq=_paq||[];_paq.push(["trackPageView"]);_paq.push(["enableLinkTracking"]);(function(){var e=("https:"==document.location.protocol?"https":"http")+"://analytics.stephencoakley.com/";_paq.push(["setTrackerUrl",e+"piwik.php"]);_paq.push(["setSiteId","1"]);var t=document,n=t.createElement("script"),r=t.getElementsByTagName("script")[0];n.type="text/javascript";n.defer=true;n.async=true;n.src=e+"piwik.js";r.parentNode.insertBefore(n,r)})();
        </script>
    </body>
</html>

