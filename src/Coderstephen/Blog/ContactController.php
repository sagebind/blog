<?php
namespace Coderstephen\Blog;

use DateTime;
use Michelf\MarkdownExtra;
use Swift_Mailer;
use Swift_MailTransport;
use Swift_Message;
use Textpress\Textpress;

class ContactController
{
    public $textpress;

    public function __construct(Textpress $textpress)
    {
        $this->textpress = $textpress;
    }

    public function post()
    {
        // get the input valyooz
        $data = array(
            'name' => $this->textpress->slim->request->params('name'),
            'company' => $this->textpress->slim->request->params('company'),
            'email' => $this->textpress->slim->request->params('email')
        );

        // format the message content
        $formattedContent = strip_tags($this->textpress->slim->request->params('content'), '<a>');
        $formattedContent = MarkdownExtra::defaultTransform($formattedContent);
        $data['content'] = (string)$formattedContent;

        // datetime
        $date = new DateTime('now');
        $data['date'] = $date->format('l, F j, Y g:i A');

        // subject
        if (isset($data['company']))
            $data['subject'] = sprintf('Message from %s at %s', $data['name'], $data['company']);
        else
            $data['subject'] = 'Message from ' . $data['name'];

        try
        {
            // create email message for me
            $contactMessage = $this->renderEmail('contactEmail', $data)
                ->setTo(array('me@stephencoakley.com' => 'Stephen Coakley'))
                ->setFrom(array($data['email'] => $data['name']))
                ->setSubject($data['subject']);

            // create reply email
            $contactReplyMessage = $this->renderEmail('contactReplyEmail', $data)
                ->setTo(array($data['email'] => $data['name']))
                ->setFrom(array('me@stephencoakley.com' => 'Stephen Coakley'))
                ->setSubject('Thanks for contacting me!');

            // create email sender
            $mailer = Swift_Mailer::newInstance(Swift_MailTransport::newInstance());

            // send emails
            $mailer->send($contactMessage);
            $mailer->send($contactReplyMessage);

            $this->textpress->viewData['successMessage'] = 'Your message has been sent.';
        }

        catch (\Exception $e)
        {
            $this->textpress->viewData['errorMessage'] = 'Your message could not be sent.';
        }

        $this->textpress->setLayout('layout');
        $this->textpress->render('contact');
    }

    public function renderEmail($template, $data)
    {
        // create the message
        $message = Swift_Message::newInstance();

        // render email template
        ob_start();
        $this->textpress->slim->render($template, $data);
        $message->setBody(ob_get_clean(), 'text/html');

        return $message;
    }
}
