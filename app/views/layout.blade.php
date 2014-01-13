<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>@yield('title')</title>
        <meta name="description" content="Programming">
        <meta name="keywords" content="Stephen Coakley, programming, web development, apps, php">
        <meta name="author" content="Stephen Coakley">
        <meta name="google-site-verification" content="8zITdJXRKp2p18XWdQZlSKP0-GvSCT2i0Rjzafs1gKg">
        <link href="/blog/feed" type="application/atom+xml" rel="alternate" title="Blog Feed">

        <link rel="icon" type="image/png" href="/favicon.128.png" sizes="128x128">
        <link rel="icon" type="image/png" href="/favicon.64.png" sizes="64x64">
        <link rel="icon" type="image/png" href="/favicon.48.png" sizes="48x48">
        <link rel="icon" type="image/png" href="/favicon.32.png" sizes="32x32">
        <link rel="shortcut icon" href="/favicon.ico">
        <link rel="icon" type="image/png" href="/favicon.16.png" sizes="16x16">
        <link rel="apple-touch-icon" href="/favicon.128.png">
        <meta name="msapplication-TileColor" content="#D83434">
        <meta name="msapplication-TileImage" content="/favicon.128.png">

        <meta name="viewport" content="initial-scale=1">
        <link rel="stylesheet" href="/css/reset.css">
        <link rel="stylesheet" href="/css/style.css">

        <!--[if lt IE 9]>
        <script src="/js/html5shiv.js"></script>
        <![endif]-->

        @section('scripts')
            <script src="/js/mootools-core-1.4.5-full-nocompat-yc.js"></script>
            <script src="/js/mootools-more-1.4.0.1.js"></script>
            <script src="/js/prism.js"></script>
        @show
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
            @yield('page-content')
        </main>

        <aside class="style-photo social">
            <div class="container">
                <h1>Me on the web</h1>
                <a class="social-link icon-twitter" rel="me" href="http://twitter.com/coderstephen"></a>
                <a class="social-link icon-googleplus" rel="me" href="http://gplus.to/coderstephen"></a>
                <a class="social-link icon-facebook" rel="me" href="http://facebook.com/coderstephen"></a>
                <a class="social-link icon-linkedin" rel="me" href="http://linkedin.com/in/coderstephen"></a>
                <a class="social-link icon-github" href="http://github.com/coderstephen"></a>
                <a class="social-link icon-stackoverflow" href="http://stackoverflow.com/users/2044560/coderstephen"></a>
            </div>
        </aside>

        <footer class="page-footer">
            <div class="container">
                <a class="badge badge-midwest" href="http://ryanvsclark.com/mxmw" target="_blank" title="Proudly made in the Midwest"></a>

                <a class="badge badge-html5" href="http://www.w3.org/html/logo/" target="_blank" title="HTML5 Powered with Semantics, CSS3 / Styling, Graphics, and 3D &amp; Effects">
                    <span class="badge-text">This site uses HTML5 awesomeness! Yay!</span>
                </a>

                <div class="footer-content">
                    <p class="page-copyright">Copyright &copy; 2013 Stephen Coakley. All Rights Reserved.</p>
                    <p>Hosted by my lovely provider <a href="http://hostek.com">Hostek.com</a></p>
                </div>
            </div>
        </footer>

        <script>(function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){(i[r].q=i[r].q||[]).push(arguments)},i[r].l=1*new Date();a=s.createElement(o),m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)})(window,document,'script','//www.google-analytics.com/analytics.js','ga');ga('create', 'UA-42213127-1', 'stephencoakley.com');ga('send', 'pageview');</script>
    </body>
</html>
