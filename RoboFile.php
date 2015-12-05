<?php require_once __DIR__ . '/vendor/autoload.php';

class RoboFile extends \Robo\Tasks
{
    public $scss_input = 'styles/main.scss';
    public $scss_output = 'static/assets/css/style.css';
    public $scss_style = 'compressed';
    public $ftp_host = 'stephencoakley.com';
    public $ftp_username = 'phing';
    public $ftp_password = 'EtTHuoEMv4pnM48';

    private $serverIp = '104.236.61.234';

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
            ->arg('-t coderstephen/blog')
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
            ->arg('coderstephen/blog')
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
