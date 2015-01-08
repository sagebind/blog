<?php
define('APP_ROOT', dirname(__DIR__));
define('APP_MODE', '');
chdir(APP_ROOT);

// require composer autoloader
require APP_ROOT . '/vendor/autoload.php';

// load config file
$config = json_decode(file_get_contents(APP_ROOT . '/config.json'), true);

// create an instance of Slim with custom view
// and set the configurations from config file
$app = new Slim\Slim(array('view' => new Textpress\View(),'mode' => APP_MODE));

// create an object of Textpress and pass the object of Slim to it
$textpress = new Textpress\Textpress($app, $config);

// register extra routes
$app->post('/contact/post', function () use ($textpress)
{
    $controller = new Coderstephen\Blog\ContactController($textpress);
    $controller->post();
});

// finally run Textpress
$textpress->run();
