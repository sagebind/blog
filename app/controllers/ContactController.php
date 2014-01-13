<?php
use Michelf\MarkdownExtra;

class ContactController extends Controller
{
    public function getIndex()
    {
        return View::make('contact');
    }

    public function postIndex()
    {
        // get the input valyooz
        $data = array(
            "name" => Input::get("name"),
            "company" => Input::get("company"),
            "email" => Input::get("email")
        );

        // format the message content
        $formattedContent = strip_tags(Input::get("content"), "<a>");
        $formattedContent = MarkdownExtra::defaultTransform($formattedContent);
        $data["content"] = (string)$formattedContent;

        // datetime
        $date = new DateTime("now");
        $data["date"] = $date->format("l, F j, Y g:i A");

        try
        {
            // email the message
            Mail::send('emails.contact', $data, function($message) use ($data)
            {
                $message->to("me@stephencoakley.com", "Stephen Coakley");
                $message->from($data["email"], $data["name"]);

                if (isset($data["company"]))
                    $message->subject(sprintf("Message from %s at %s", $data["name"], $data["company"]));
                else
                    $message->subject("Message from " . $data["name"]);
            });

            // email the contact reply
            Mail::send('emails.contactReply', $data, function($message) use ($data)
            {
                $message->to($data["email"], $data["name"]);
                $message->from("me@stephencoakley.com", "Stephen Coakley");
                $message->subject("Thanks for contacting me!");
            });
        }

        catch (\Exception $e)
        {
            return View::make('contact', ["errorMessage" => "Your message could not be sent."]);
        }

        // render the view
        return View::make('contact', ["successMessage" => "Your message has been sent."]);
    }
}
