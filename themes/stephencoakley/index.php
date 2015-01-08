<div class="content">
    <p><em class="big">Hello there.</em> I'm a software developer based in Wisconsin. I design and develop websites and web apps, program applications, and code awesome projects.</p>

    <p>I am experienced in many areas of computer programming, including web programming, desktop programs, databases, command-line apps, administrative scripts, and software APIs.</p>

    <hr/>

    <h1>Portfolio</h1>

    <div class="portfolio">
        <figure class="item">
            <img class="portfolio-item-figure" src="/images/portfolio/nccquizzing.png" alt="Screenshot of nccquizzing.org">
        </figure>

        <figure class="item">
            <img class="portfolio-item-figure" src="/images/portfolio/beloitfmc.png" alt="Screenshot of beloitfmc.org">
        </figure>
    </div>

    <p><a class="button" href="/portfolio">Show Me More <span class="fa fa-arrow-right"></span></a></p>

    <hr/>

    <h1>Recent Blog Posts</h1>
    <?php foreach (new LimitIterator(new ArrayIterator($articles), 0, 2) as $article): ?>
        <article>
            <h2><a href="<?php echo $article->getUrl(); ?>"><?php echo $article->getTitle(); ?></a></h2>
            <p><?php echo $article->getSummary(250); ?>...</p>
        </article>
    <?php endforeach; ?>

    <p><a class="button" href="/blog">View All <i class="fa fa-arrow-right"></i></a></p>

    <hr/>

    <h1>Contact Me</h1>
    <p><em class="big">Like what you see? Shoot me an email.</em> If you have a cool project you want me to work on, just let me know the name of the project and what my role would be in your message. Are you a non-profit organization with an aging website that needs redesigned? Send me an email, and we'll talk.</p>

    <p class="center">
        <a class="button" data-color="grey" href="mailto:me@stephencoakley.com">Contact Me</a>
    </p>

    <p>Are you hiring developers through <a href="http://odesk.com" target="_blank">oDesk</a>? View my <a href="http://odesk.com/users/~01050b02e7dfac9e14">oDesk profile</a> and send me an interview request for your project.</p>
</div>
