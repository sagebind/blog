<?php

class PortfolioController extends Controller
{
    public function getIndex()
    {
        return View::make("portfolio");
    }
}