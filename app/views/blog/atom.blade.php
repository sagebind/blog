<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
    <title>Stephen Coakley's Blog</title>
    <link href="http://stephencoakley.com/blog/feed" rel="self" />
    <link href="http://stephencoakley.com/" />

    @foreach ($posts as $post)
    <entry>
        <title>{{ $post->title }}</title>
        <link href="http://stephencoakley.com/blog/post/{{ $post->slug }}" />
        <updated>{{ $post->getUpdatedDate() }}</updated>
        <author>
            <name>Stephen Coakley</name>
            <email>me@stephencoakley.com</email>
        </author>
        <content type="html">
            {{ $post->getContentHtml() }}
        </content>
    </entry>
    @endforeach
</feed>
