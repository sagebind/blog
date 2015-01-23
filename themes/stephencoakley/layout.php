<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title><?=(isset($global['title'])?$global['title'].' - ':'').$global['site.name']?></title>
        <meta name="description" content="<?php echo $global['site.description']; ?>">
        <meta name="keywords" content="Stephen Coakley, programming, web development, apps, php">
        <meta name="author" content="<?=$global['author.name']?>">
        <link href="/feed/atom.xml" type="application/atom+xml" rel="alternate" title="Blog Feed">

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
        <script type="text/javascript" src="<?=$global['assets.prefix']?>/js/jquery.min.js"></script>
        <script type="text/javascript" src="<?=$global['assets.prefix']?>/js/smoothbox.jquery2.min.js"></script>
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
            <?php echo $content; ?>
            <hr>
            <aside class="social-icons">
                <a class="social-icon-twitter no-underline" href="http://bit.ly/codrst-tw"></a>
                <a class="social-icon-gplus no-underline" href="http://bit.ly/codrst-plus"></a>
                <a class="social-icon-facebook no-underline" href="http://bit.ly/codrst-fb"></a>
                <a class="social-icon-linkedin no-underline" href="http://linkedin.com/in/coderstephen"></a>
                <a class="social-icon-github no-underline" href="http://bit.ly/codrst-github"></a>
                <a class="social-icon-stackoverflow no-underline" href="http://bit.ly/codrst-stack"></a>
            </aside>

            <a class="button top-link" href="#top"><i class="fa fa-arrow-up"></i></a>
        </main>

        <footer class="page-footer">
            <p>Get in touch with me: <a href="mailto:me@stephencoakley.com">me@stephencoakley.com</a></p>
            <p class="page-copyright">All content and design copyright &copy; Stephen Coakley.</p>
            <p>Hosted by my lovely provider <a href="http://hostek.com">Hostek.com</a>.</p>
        </footer>

        <script>
            var _paq=_paq||[];_paq.push(["trackPageView"]);_paq.push(["enableLinkTracking"]);(function(){var e=("https:"==document.location.protocol?"https":"http")+"://analytics.stephencoakley.com/";_paq.push(["setTrackerUrl",e+"piwik.php"]);_paq.push(["setSiteId","1"]);var t=document,n=t.createElement("script"),r=t.getElementsByTagName("script")[0];n.type="text/javascript";n.defer=true;n.async=true;n.src=e+"piwik.js";r.parentNode.insertBefore(n,r)})();
        </script>
    </body>
</html>
