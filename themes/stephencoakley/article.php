<div class="content style-default">
    <p><a href="/blog"><i class="fa fa-arrow-left"></i> All Articles</a></p>

    <article>
        <p class="postmeta"><small>
            <span class="date"><i class="fa fa-calendar-o"></i> <?=$article->getDate($global['date.format'])?></span>
            <span class="author-by"> by </span>
            <span class="author"><?=$article->getAuthor()?$article->getAuthor():$global['author.name']?></span>
        </small></p>

        <h1><?=$article->getTitle()?></h1>

        <?php echo $article->getContent(); ?>
    </article>

    <p class="tags"><small>
        <i class="fa fa-tags"></i>
        <?php foreach ($article->getTags() as $slug => $tag):?>
            <span class="tag"><a href="/tag/<?=$slug?>"><?=$tag->name?></a></span>
        <?php endforeach;?>
    </small></p>

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
