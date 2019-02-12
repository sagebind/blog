+++
title = "Terminal Setup Tour"
author = "Stephen Coakley"
date = "2017-02-19"
tags = ["workflow"]
+++

There are three types of developers in the world:

1. Those who occasionally use the terminal for compiling, version control, or for handy scripts.
2. Those who swear a GUI is better for everything.
3. Those who prefer to use the terminal for most everyday tasks and always have at least one open.

Category number 1 seems to be the most common in the wild, but if you know me, you know I fall squarely into category 3. Like most terminal addicts, I have a highly customized setup tailored to my workflow with lots of configuration and custom scripts, so here I will give you a short rundown of how it is set up and how it works.

Maybe you are also a terminal addict, or maybe you are just looking into more or better ways you could use the terminal for more tasks. This isn't just for me to geek-brag on how cool it is, but it is a way for me to share some tips and tricks I've spent a while learning and adopting. I'm hoping a little tour of my setup will give you some new ideas on how to improve your own workflow, or you might just find it interesting if you are a big nerd like me.

## [tmux]
This is our first stop, and probably the most important one. Being able to open multiple terminals is a must, but window management with the mouse is slow and tedious. I use tmux to enhance my copy-paste ability, tab management, and of course split windows [Terminator] style. The benefit of using tmux to provide these features instead of the terminal emulator itself (some terminal emulators have some pretty darn good versions of these features) is that I can have consistent key bindings across operating systems, environments, and over SSH. I use Linux, Windows, and OS X every day and every little bit of unification helps.

## [fish]
Fish: the friendly interactive shell. I switched to fish from Bash a few years ago and never looked back. Fish has a similar goal to the ever popular ZSH, which is to make command shells more powerful and useful. Unlike ZSH, Fish sacrifices syntax compatability with Bash for a much better, logical syntax. Even better, Fish has super useful interactive capabilities like command completion, interactive history, and more out of the box without writing a 200-line config file. I strongly recommend checking it out and giving it a spin.

If you do try out Fish, I'd recommend trying out [Oh My Fish] as well. Kind of like [Oh-My-Zsh], Oh My Fish provides a package loading system that makes it easy to organize configuration into groups and to install plugins and themes other users have shared. I'm probably biased since I'm an Oh My Fish maintainer, but it is a really neat project and I'd encourage you to try it out.

## [vim]
Vim needs no introduction. A staple editor with useful features and powerful editing control. I don't actually do much programming in Vim; I mostly use it how you might use Notepad, for editing text files when I don't want to fire up a whole project editor or IDE.

I've also been experimenting with `mcedit` for a while which has a really nice keybindings system, but I'm not sure it has all the editing capabilities I need.

## Directory navigation
This is something everyone should experiment with. There's a lot better ways of navigating a large number of directories than with a simple `cd`. There are quite a few tools out there to solve this problem. Most of them either let you "bookmark" certain directories for fast access, or learn what directories you go to often as you navigate. I've used [autojump], then [z], then [fasd], and all three of them work well. I'm always looking on how to improve my workflow though, so currently I'm using [marlin], something of my own invention.

I don't care what tool you use, but once you pick _a_ directory jump tool you will never look back.

## Version-controlled dotfiles
The last thing I want to note is that I keep all my important dotfiles in [a public GitHub repository](https://github.com/sagebind/dotfiles). Keeping my config files here helps me in a few ways:

- I can set up a new system to my liking very quickly.
- I can remember _what_ programs I have customized.
- I can keep changes I make to configuration synchronized across systems.
- I can roll back bad configuration.

If you do any substantial configuration yourself, I recommend storing your config files in a central location as well. For a programmer, your config files will probably your longest ongoing project, so it's worth keeping them safe and organized. If you don't want to roll your own solution for managing your dotfiles there is a really good list of some tools and frameworks you can use at [dotfiles.github.io](https://dotfiles.github.io).

## What do you use?
That's the brief fly-by of my setup; maybe you will be inspired to improve your own workflow. I'm always interested in other people's workflows and the tricks they've developed themselves for the sake of fun and efficiency. Leave a comment below if you'd like to share some of your own tricks!


[autojump]: https://github.com/wting/autojump
[fasd]: https://github.com/clvv/fasd
[fish]: http://fishshell.com
[marlin]: https://github.com/oh-my-fish/marlin
[oh My Fish]: https://github.com/oh-my-fish/oh-my-fish
[Oh-My-Zsh]: https://github.com/robbyrussell/oh-my-zsh
[Terminator]: https://gnometerminator.blogspot.com/p/introduction.html
[tmux]: https://tmux.github.io
[vim]: http://www.vim.org
[z]: https://github.com/rupa/z
