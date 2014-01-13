@extends('layout')
@section('title', $post->title . ' - Stephen Coakley\'s Blog')

@section('page-content')
    <div class="content-section style-default">
        <article class="container">
            <h1>{{ $post->title }}</h1>
            {{ $post->getContentHtml() }}
        </article>
    </div>
@stop
