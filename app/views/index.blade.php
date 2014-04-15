@extends('layout')
@section('title', 'Stephen Coakley')

@section('page-content')
    <div class="content-section banner-section">
        <div class="container">
            <div class="banner-text">
                <span class="banner-line banner-line1">Hi, I'm Stephen Coakley</span>
                <span class="banner-line banner-line2">Software developer &amp; web designer</span>
                <span class="banner-line banner-line3">I make<br>things</span>
            </div>
        </div>
    </div>

    <div class="content-section style-default style-intro">
        <div class="container">
            <p><em class="big">Hello there.</em> I'm a software developer based in Wisconsin. I design and develop websites and web apps, program applications, and code awesome projects.</p>

            <p>Keep scrolling to learn more about me and what I do.</p>
        </div>
    </div>

    <div class="content-section style-dark">
        <div class="container">
            <h1>Coder</h1>
            <p><em class="big">I love to write code.</em> I do it for work. I do it for school. I most certainly do it for fun. Do you have a new project for me? I'll commit, because I'll enjoy every minute of it.</p>

            <p><em class="big">New technology is my passion.</em></p>

            <p>My code is always carefully crafted. Standards are important to me, so you can always count on <em>everything</em> I make to be <em>stable, extensible, and compatible</em> across a wide range of devices and platforms. All my websites use valid HTML5 and CSS3.</p>

            <p>I am experienced in many areas of computer programming, including web programming, desktop programs, databases, command-line apps, administrative scripts, and software APIs.</p>
        </div>
    </div>

    <div id="designer" class="content-section style-photo style-designer">
        <div class="container">
            <h1>Designer</h1>
            <p><em class="big">I love to create beautiful things.</em> Design is key to how you appear to your users and how they interact with your interface. Even more, it is an expression; a personal flair and a statement of your personality and creativity.</p>

            <p>Because design is so important to me, it is a top priority when I create apps and websites.</p>

            <div class="center-stage">
                <a class="button button-highlight" href="/portfolio">View my portfolio</a>
            </div>
        </div>
    </div>

    <div class="content-section style-light">
        <div class="container">
            <h1>Want to hire me?</h1>
            <p><em class="big">Like what you see? Shoot me an email.</em> If you have a cool project you want me to work on, just let me know the name of the project and what my role would be in your message. Are you a non-profit organization with an aging website that needs redesigned? Send me an email, and we'll talk.</p>

            <div class="center-stage">
                <a class="button button-grey" href="/contact">Contact me</a>
            </div>

            <p>Are you hiring developers through <a href="http://odesk.com" target="_blank">oDesk</a>? View my <a href="http://odesk.com/users/~01050b02e7dfac9e14">oDesk profile</a> and send me an interview request for your project.</p>
        </div>
    </div>

    <aside class="style-photo social">
        <div class="container">
            <h1>Me on the web</h1>
            <a class="social-link icon-twitter" rel="me" href="http://bit.ly/codrst-tw"></a>
            <a class="social-link icon-googleplus" rel="me" href="http://bit.ly/codrst-plus"></a>
            <a class="social-link icon-facebook" rel="me" href="http://bit.ly/codrst-fb"></a>
            <a class="social-link icon-linkedin" rel="me" href="http://linkedin.com/in/coderstephen"></a>
            <a class="social-link icon-github" href="http://bit.ly/codrst-github"></a>
            <a class="social-link icon-stackoverflow" href="http://bit.ly/codrst-stack"></a>
        </div>
    </aside>
@stop
