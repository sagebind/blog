<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title><?php
            $title= (isset($global['title'])) ? $global['title'] : $global['site.title'];
            echo $title .' - '. $global['site.name'];
        ?></title>
        <meta name="description" content="<?php echo $global['site.description']; ?>">
        <meta name="keywords" content="Stephen Coakley, programming, web development, apps, php">
        <meta name="author" content="<?php echo $global['author.name']; ?>">
        <link href="/blog/feed" type="application/atom+xml" rel="alternate" title="Blog Feed">

        <link rel="icon" type="image/png" href="<?php echo $global['assets.prefix'];?>/images/favicon.128.png" sizes="128x128">
        <link rel="icon" type="image/png" href="<?php echo $global['assets.prefix'];?>/images/favicon.64.png" sizes="64x64">
        <link rel="icon" type="image/png" href="<?php echo $global['assets.prefix'];?>/images/favicon.48.png" sizes="48x48">
        <link rel="icon" type="image/png" href="<?php echo $global['assets.prefix'];?>/images/favicon.32.png" sizes="32x32">
        <link rel="shortcut icon" href="<?php echo $global['assets.prefix'];?>/favicon.ico">
        <link rel="icon" type="image/png" href="<?php echo $global['assets.prefix'];?>/images/favicon.16.png" sizes="16x16">
        <link rel="apple-touch-icon" href="<?php echo $global['assets.prefix'];?>/images/favicon.128.png">
        <meta name="msapplication-TileColor" content="#D83434">
        <meta name="msapplication-TileImage" content="<?php echo $global['assets.prefix'];?>/images/favicon.128.png">

        <meta name="viewport" content="initial-scale=1">
        <link rel="stylesheet" href="<?php echo $global['assets.prefix'];?>/css/style.css">
        <link rel="stylesheet" href="//cdnjs.cloudflare.com/ajax/libs/highlight.js/8.4/styles/github.min.css">

        <!--[if lt IE 9]>
        <script src="<?php echo $global['assets.prefix'];?>/js/html5shiv.min.js"></script>
        <![endif]-->
        <script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/8.4/highlight.min.js"></script>
        <script>hljs.initHighlightingOnLoad();</script>
    </head>

    <body id="top">
        <header class="page-header">
            <div class="title">
                <a class="site-logo" href="/" rel="home">
                    <span class="site-logo-text">Stephen Coakley</span>
                </a>

                <p><em class="big">Hello there.</em> I'm a software developer based in Wisconsin. I design and develop websites and web apps, program applications, and code awesome projects.</p>
            </div>

            <nav class="center">
                <a class="button" href="/">Home</a>
                <a class="button" href="/blog">Blog</a>
                <a class="button" href="/portfolio">Portfolio</a>
            </nav>
        </header>

        <main role="main">
            <?php echo $content; ?>

            <a class="button top-link fa fa-arrow-up" href="#top"></a>
        </main>

        <footer class="page-footer">
            <p class="page-copyright">All content and design copyright &copy; Stephen Coakley.</p>
            <p>Hosted by my lovely provider <a href="http://hostek.com">Hostek.com</a></p>
        </footer>

        <script>
          var _paq = _paq || [];
          _paq.push(["trackPageView"]);
          _paq.push(["enableLinkTracking"]);

          (function() {
            var u=(("https:" == document.location.protocol) ? "https" : "http") + "://analytics.stephencoakley.com/";
            _paq.push(["setTrackerUrl", u+"piwik.php"]);
            _paq.push(["setSiteId", "1"]);
            var d=document, g=d.createElement("script"), s=d.getElementsByTagName("script")[0]; g.type="text/javascript";
            g.defer=true; g.async=true; g.src=u+"piwik.js"; s.parentNode.insertBefore(g,s);
          })();
        </script>
    </body>
</html>
