<div class="content style-default">
    <p><a href="/blog"><i class="fa fa-arrow-left"></i> All Articles</a></p>

    <article>
        <h1><?=$article->getTitle()?></h1>

        <p class="postmeta"><small>
            <span class="date"><i class="fa fa-calendar-o"></i> <?=$article->getDate($global['date.format'])?></span>
            <span class="author-by"> by </span>
            <span class="author"><?=$article->getAuthor()?$article->getAuthor():$global['author.name']?></span>

            <?php if (!empty($article->getCategories())): ?>
                <?php $category = array_keys($article->getCategories())[0]; ?>
                <p class="category">
                    Posted under <a href="/category/<?=$category?>"><?=$category?></a>
                </p>
            <?php endif; ?>
        </small></p>

        <?php echo $article->getContent(); ?>
    </article>

    <?php if ($global['disqus.username']): ?>
        <div id="disqus_thread" class="comments"></div>
        <script type="text/javascript">
            var disqus_developer = 1;
            var disqus_shortname = '<?=$global['disqus.username']?>';
            (function() {
                var dsq = document.createElement('script'); dsq.type = 'text/javascript'; dsq.async = true;
                dsq.src = '//' + disqus_shortname + '.disqus.com/embed.js';
                (document.getElementsByTagName('head')[0] || document.getElementsByTagName('body')[0]).appendChild(dsq);
            })();
        </script>
        <noscript>Please enable JavaScript to view the <a href="http://disqus.com/?ref_noscript">comments powered by Disqus.</a></noscript>
    <?php endif; ?>
</div>
