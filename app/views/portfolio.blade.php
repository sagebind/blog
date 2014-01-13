@extends('layout')
@section('title', 'Portfolio - Stephen Coakley')

@section('scripts')
    @parent
    <script src="/js/portfolio.js"></script>

    <script>
        window.addEvent('load', function() {
            var portfolio = new web.Portfolio("portfolio");
            portfolio.showItem(1);
        });
    </script>
@stop

@section('page-content')
    <header class="main-heading">
        <h1 class="container">Portfolio</h1>
    </header>

    <div class="content-section portfolio">
        <div class="container">
            <figure class="portfolio-item">
                <img class="portfolio-item-figure" src="/images/portfolio/nccquizzing.png" alt="Screenshot of nccquizzing.org">

                <figcaption>
                    <p>A website for a conference youth program for the Free Methodist Church - USA.</p>
                    <a class="button button-grey" href="http://nccquizzing.org" target="_blank">View</a>
                </figcaption>
            </figure>

            <figure class="portfolio-item">
                <img class="portfolio-item-figure" src="/images/portfolio/beloitfmc.png" alt="Screenshot of beloitfmc.org">

                <figcaption>
                    <p>A website for the Free Methodist church in Beloit, Wisconsin.</p>
                    <a class="button button-grey" href="http://beloitfmc.org" target="_blank">View</a>
                </figcaption>
            </figure>

            <figure class="portfolio-item">
                <img class="portfolio-item-figure" src="/images/portfolio/emmanuelfmc.png" alt="Screenshot of emmanuelfmc.org">

                <figcaption>
                    <p>A website for Emmanuel Church in Janesville, Wisconsin.</p>
                    <p>This website is no longer live.</p>
                </figcaption>
            </figure>
        </div>
    </div>
@stop
