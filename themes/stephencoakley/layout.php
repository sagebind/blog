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

        <!--[if lt IE 9]>
        <script src="<?php echo $global['assets.prefix'];?>/components/html5shiv/dist/html5shiv.js"></script>
        <![endif]-->

        <script src="<?php echo $global['assets.prefix'];?>/components/mootools-core-1.4.5-full-nocompat-yc.js"></script>
        <script src="<?php echo $global['assets.prefix'];?>/components/mootools-more-1.4.0.1.js"></script>
        <script src="<?php echo $global['assets.prefix'];?>/components/prism/prism.js"></script>

        <!-- Piwik -->
        <script type="text/javascript">
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
        <!-- End Piwik Code -->
    </head>

    <body class="page">
        <header class="flow-container page-header" role="banner">
            <nav class="container flow-inner">
                <div class="header-item">
                    <a class="site-logo" href="/" rel="home">
                        <span class="site-logo-text">Stephen Coakley</span>
                    </a>
                </div>

                <div class="header-item nav-link"><a href="/about">About</a></div>
                <div class="header-item nav-link"><a href="/portfolio">Portfolio</a></div>
                <div class="header-item nav-link"><a href="/projects">Projects</a></div>
                <div class="header-item nav-link"><a href="/blog">Blog</a></div>
                <div class="header-item nav-link"><a href="/contact">Contact</a></div>
            </nav>
        </header>

        <div class="page-header-spacer"></div>

        <main class="page-content" role="main">
            <?php echo $content; ?>
        </main>

        <footer class="page-footer">
            <div class="container">
                <a class="badge badge-midwest" href="http://ryanvsclark.com/mxmw" target="_blank" title="Proudly made in the Midwest"></a>

                <a class="badge badge-html5" href="http://www.w3.org/html/logo/" target="_blank" title="HTML5 Powered with Semantics, CSS3 / Styling, Graphics, and 3D &amp; Effects">
                    <span class="badge-text">This site uses HTML5 awesomeness! Yay!</span>
                </a>

                <div class="footer-content">
                    <p class="page-copyright">Copyright &copy; Stephen Coakley. All Rights Reserved.</p>
                    <p>Hosted by my lovely provider <a href="http://hostek.com">Hostek.com</a></p>
                </div>
            </div>
        </footer>
    </body>
</html>
