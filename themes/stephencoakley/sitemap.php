<?php
Slim\Slim::getInstance()->response->headers->set('Content-Type', 'application/xml; charset=utf-8');

?><?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.sitemaps.org/schemas/sitemap/0.9 http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd" xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
	<url>
		<loc><?=$baseUrl?></loc>
		<lastmod><?=date('c', strtotime('-1 days'))?></lastmod>
		<changefreq>weekly</changefreq>
		<priority>1</priority>
	</url>
	<?php if (!empty($sitemapData)): ?>
		<?php foreach ($sitemapData as $data): ?>
		<url>
			<loc><?=$data['loc']?></loc>
			<lastmod><?=$data['lastmod']?></lastmod>
			<changefreq><?=$data['changefreq']?></changefreq>
			<priority><?=$data['priority']?></priority>
		</url>
		<?php endforeach; ?>
	<?php endif; ?>
</urlset>
