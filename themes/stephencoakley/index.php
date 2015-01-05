<div class="content">
    <p>I am experienced in many areas of computer programming, including web programming, desktop programs, databases, command-line apps, administrative scripts, and software APIs.</p>

    <h1>Contact Me</h1>
    <p><em class="big">Like what you see? Shoot me an email.</em> If you have a cool project you want me to work on, just let me know the name of the project and what my role would be in your message. Are you a non-profit organization with an aging website that needs redesigned? Send me an email, and we'll talk.</p>

    <div class="center-stage">
        <a class="button big" data-color="grey" href="/contact">Contact Me</a>
    </div>

    <p>Are you hiring developers through <a href="http://odesk.com" target="_blank">oDesk</a>? View my <a href="http://odesk.com/users/~01050b02e7dfac9e14">oDesk profile</a> and send me an interview request for your project.</p>

    <h1>Recent Blog Posts</h1>
    <?php foreach (new LimitIterator(new ArrayIterator($articles), 0, 2) as $article): ?>
        <article>
            <h2><a href="<?php echo $article->getUrl(); ?>"><?php echo $article->getTitle(); ?></a></h2>
            <p><?php echo $article->getSummary(250); ?>...</p>
            <p><a href="<?php echo $article->getUrl(); ?>">Read full post</a></p>
        </article>
    <?php endforeach; ?>
</div>
