<?php

Route::get('/', function()
{
    return View::make('index');
});

Route::get('/about', function()
{
    return View::make('about');
});

Route::get('/projects', function()
{
    return View::make('projects');
});

Route::controller('/portfolio', "PortfolioController");

Route::controller('/contact', "ContactController");

Route::controller('/blog', "BlogController");
