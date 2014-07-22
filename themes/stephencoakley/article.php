<div class="content-section style-default">
    <article class="container">
        <h1><?php echo $article->getTitle(); ?></h1>
    <div class="postmeta">
    <span class="date"><?php  echo $article->getDate($global['date.format']);  ?></span> /
    <span class="author-by"> By </span>
    <span class="author"><?php  echo $article->getAuthor()
                        ? $article->getAuthor()
                        : $global['author.name'] ;  ?></span>
    </div>
        <?php echo $article->getContent(); ?>

        
    <div class="tags">
      Tags : 
      <?php
        foreach ($article->getTags() as $slug => $tag) {
          echo '<span class="tag"><a href="/tag/' . $slug .'">' . $tag->name . "</a></span>";
        }
        ?>
    </div>
    </article>

    <?php if ($global['disqus.username']): ?>
        <div id="disqus_thread" class="container"></div>
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
        <!--<a href="http://disqus.com" class="dsq-brlink">comments powered by <span class="logo-disqus">Disqus</span></a>-->
    <?php endif; ?>
</div>
