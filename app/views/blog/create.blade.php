@extends('layout')
@section('title', 'Write a post - Stephen Coakley\'s Blog')

@section('page-content')
    <header class="main-heading">
        <h1 class="container">Blog</h1>
    </header>

    <div class="content-section style-default">
        <div class="container">
            <h1>Write a post</h1>

            <form action="/blog/create" method="post">
                <label for="titleTextBox">Post title</label>
                <input type="text" id="titleTextBox" name="title" placeholder="Post title">

                <label for="contentTextBox">Post content</label>
                <textarea id="contentTextBox" name="content" rows="10" placeholder="Post content"></textarea>

                <div class="form-buttons">
                    <input class="button button-highlight" type="submit" value="Submit Post">
                </div>
            </form>
        </div>
    </div>
@stop
