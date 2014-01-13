<?php

class BlogController extends Controller
{
    public function getIndex()
    {
        return View::make('blog.index', [
            'posts' => BlogPost::all()
        ]);
    }

    public function getPost($slug)
    {
        $post = BlogPost::where('slug', '=', $slug)->firstOrFail();

        return View::make('blog.post', [
            'post' => $post
        ]);
    }

    public function getCreate()
    {
        if (!$this->isAuthorized())
        {
            return Response::make('', 401, [
                'WWW-Authenticate' => 'Basic realm="Log in to post"'
            ]);
        }

        return View::make('blog.create');
    }

    public function postCreate()
    {
        if (!$this->isAuthorized())
        {
            return Response::make('', 401);
        }

        $date = new DateTime('now');

        $post = new BlogPost();
        $post->title = Input::get('title');
        $post->slug = $this->createSlug($post->title);
        $post->content = Input::get('content');
        $post->save();

        return Redirect::to('/blog/post/' . $post->slug);
    }

    public function getFeed()
    {
        return Response::view('blog.atom', [
            'posts' => BlogPost::all()
        ])->header('Content-Type', 'application/atom+xml');
    }

    protected function isAuthorized()
    {
        if (isset($_SERVER['HTTP_AUTHORIZATION']))
        {
            list($user, $pw) = explode(':', base64_decode(substr($_SERVER['HTTP_AUTHORIZATION'], 6)));
            return $user === 'coderstephen' && $pw === 'UStpd2Hobts';
        }

        return false;
    }

    protected function createSlug($title)
    {
        // prep string with some basic normalization
        $slug = html_entity_decode(stripslashes(strip_tags(strtolower($title))));

        // remove quotes (can't, etc.)
        $slug = str_replace('\'', '', $slug);

        // replace non-alpha numeric with hyphens
        $slug = preg_replace('/[^a-z0-9]+/', '-', $slug);

        return trim($slug, '-');
    }
}
