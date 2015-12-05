<?php require_once __DIR__ . '/vendor/autoload.php';

class RoboFile extends \Robo\Tasks
{
    public $scss_input = 'styles/main.scss';
    public $scss_output = 'static/assets/css/style.css';
    public $scss_style = 'compressed';

    public function fileWatch()
    {
        $this->_mkdir('static/assets/css');
        $this->taskExec('sass')
             ->arg('--style compressed')
             ->arg('--no-cache')
             ->arg('--trace')
             ->arg('--watch')
             ->arg($this->scss_input . ':' . $this->scss_output)
             ->run();
    }

    public function build()
    {
        $this->say('Compiling SCSS to CSS...');

        $this->_mkdir('static/assets/css');
        $this->taskExec('sass')
             ->arg('--style compressed')
             ->arg('--no-cache')
             ->arg('--trace')
             ->arg($this->scss_input)
             ->arg($this->scss_output)
             ->run();
    }

    public function buildDocker()
    {
        $this->build();

        $this->taskExec('docker')
            ->arg('build')
            ->arg('-t stephencoakley.com')
            ->arg('.')
            ->run();
    }

    public function deployDocker()
    {
        $this->buildDocker();

        $this->_mkdir('build');
        $this->taskExec('docker')
            ->arg('save')
            ->arg('-o build/blog.tar')
            ->arg('stephencoakley.com')
            ->run();

        $this->taskExec('scp')
            ->arg('build/blog.tar')
            ->arg('root@' . $this->serverIp . ':/root/')
            ->run();
    }

    public function serve()
    {
        $this->taskExec('php')
             ->arg('bin/server')
             ->run();
    }
}
