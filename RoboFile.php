<?php require_once __DIR__.'/vendor/autoload.php';

class RoboFile extends \Robo\Tasks
{
    use RoboFtp\FtpDeploy;

    public $scss_input = 'styles/main.scss';
    public $scss_output = 'wwwroot/assets/css/style.css';
    public $scss_style = 'compressed';
    public $ftp_host = 'stephencoakley.com';
    public $ftp_username = 'phing';
    public $ftp_password = 'EtTHuoEMv4pnM48';

    function clean()
    {
        $this->say('Cleaning...');
        $this->_cleanDir('cache');
    }

    function fileWatch()
    {
        $this->_mkdir('wwwroot/assets/css');
        $this->taskExec('sass')
            ->arg('--style compressed')
            ->arg('--no-cache')
            ->arg('--trace')
            ->arg('--watch')
            ->arg($this->scss_input . ':' . $this->scss_output)
            ->background()
            ->run();
    }

    function build()
    {
        $this->clean();
        $this->say('Compiling SCSS to CSS...');

        $this->_mkdir('wwwroot/assets/css');
        $this->taskExec('sass')
            ->arg('--style compressed')
            ->arg('--no-cache')
            ->arg('--trace')
            ->arg($this->scss_input)
            ->arg($this->scss_output)
            ->run();
    }

    function deploy()
    {
        $this->build();

        $ftp = $this->taskFtpDeploy($this->ftp_host, $this->ftp_username, $this->ftp_password)
            ->dir('/')
            ->from('.')
            ->exclude('styles')
            ->exclude('tests')
            ->secure(false)
            ->skipSizeEqual()
            ->skipUnmodified()
            ->run();
    }

    function deployCommit()
    {
        $this->build();

        // get the files changed in the last commit
        $this->say('Writing last commit diff...');
        $result = $this->taskExec('git')
            ->arg('diff-tree')
            ->arg('--no-commit-id')
            ->arg('--name-only')
            ->arg('--diff-filter=ACMRT')
            ->arg('-r master')
            ->run();
        $files = preg_split('/[\r\n]+/', trim($result->getMessage()));

        // deploy the changed files
        $this->say('Deploying build diff for last commit...');
        $ftp = $this->taskFtpDeploy($this->ftp_host, $this->ftp_username, $this->ftp_password)
            ->dir('/test')
            ->files($files)
            ->secure(false)
            ->run();
    }

    function serverStart()
    {
        $this->taskServer(8080)
            ->dir('wwwroot')
            ->background()
            ->run();
    }
}
