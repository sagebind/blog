<div class="content">
    <h1>Blog posts tagged &ldquo;<?=$global['tag']?>&rdquo;</h1>
    <p>Found <?=count($articles)?> article(s).</p>

    <?php foreach($articles as $article): ?>
        <article>
            <h2><a href="<?php echo $article->getUrl(); ?>"><?php echo $article->getTitle(); ?></a></h2>
            <p><?php echo $article->getSummary(250); ?>...</p>
            <p><a href="<?php echo $article->getUrl(); ?>">Read full post <i class="fa fa-arrow-right"></i></a></p>
        </article>
    <?php endforeach; ?>
</div>
