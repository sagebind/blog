@extends('layout')
@section('title', 'Stephen Coakley\'s Blog')

@section('page-content')
    <header class="main-heading">
        <h1 class="container">Blog</h1>
    </header>

    <div class="content-section style-default">
        <div class="container">
            <p><em class="big">Welcome to my blog,</em> where I post interesting thoughts on web development and programming, demos and examples of code, and rants about the internet and software.</p>

            @if (count($posts) > 0)
                @foreach ($posts as $post)
                    <article>
                        <h2><a href="/blog/post/{{ $post->slug }}">{{ $post->title }}</a></h2>
                        {{ $post->getAbbreviatedContentHtml() }}
                        <p><a href="/blog/post/{{ $post->slug }}">Read full post</a></p>
                    </article>
                @endforeach
            @else
                <p>No blog posts found.</p>
            @endif
        </div>
    </div>
@stop
