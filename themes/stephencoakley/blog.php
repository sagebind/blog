<div class="content">
    <h1>Articles</h1>
    <p><em class="big">This is my blog,</em> where I post interesting thoughts on web development and programming, demos and examples of code, and rants about the Internet and software.</p>

    <?php if (count($articles) < 1): ?>
        <p>No articles found!</p>
    <?php else: foreach ($articles as $article): ?>
        <article>
            <h2><a href="<?php echo $article->getUrl();?>"><?php echo $article->getTitle();?></a></h2>
            <?php if (!empty($article->getCategories())): ?>
                <?php $category = array_keys($article->getCategories())[0]; ?>
                <small class="category">
                    Posted under <a href="/category/<?=$category?>"><?=$category?></a>
                </small><br/><br/>
            <?php endif; ?>
            <p><?php echo $article->getSummary(250);?>...</p>
            <p><a href="<?php echo $article->getUrl();?>">Read full post <i class="fa fa-arrow-right"></i></a></p>
        </article>
    <?php endforeach;endif;?>
</div>
