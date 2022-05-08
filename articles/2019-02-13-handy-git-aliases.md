+++
title = "Handy Git Aliases"
author = "Stephen Coakley"
date = "2019-02-13"
tags = ["programming", "workflow"]
+++

I am a heavy [Git] user. I use it every day at work, I use it almost every other evening for personal projects, notes, and anything else I can find a use case for Git. I've also been using it for a little while, since at least 2012 (approximately), and one of my favorite Git features is [aliases](https://git-scm.com/book/en/v2/Git-Basics-Git-Aliases). Aliases let you add your own short little subcommands to the Git command by adding one-liners to your user configuration. It's great for renaming commands to make them easier to remember and for composing a common sequence of commands into one.

Over the years I have collected quite a list of aliases that I have found useful. Some of them I came up with myself, others I've "borrowed" from other blog posts of people sharing their aliases, just like this one. Since I found such posts helpful to my own productivity, I figured it is about time I return the favor to the community and share my own aliases.

I won't be explaining all the aliases I have since most of them are boring, but if you want to see the full list you can look in my [dotfiles] project.

## `git aliases`

Even though aliases are useful, I'll ~~sometimes~~ often forget what aliases I have or what they do. This alias helps me remember, because `git aliases` is really easy to remember.

```sh
aliases = config --get-regexp "^alias\\."
```

## `git staged`

This one is pretty simple; it just shows a diff between the current branch and whatever changes you have staged. It's not much shorter than `git diff --staged`, but it is easier to remember.

```sh
staged = diff --staged
```

## `git amend`

Like the last one, this alias just gives a shorter name for a common action. I amend commits more than I'd care to admit, so this alias is useful for me.

```sh
amend = commit --amend
```

## `git contains`

Does what you might expect; takes some sort of ref, like a commit hash, and returns 1 or 0 if the current branch contains that commit. This is something I discovered that you could do one time when I needed it, but there's no way anyone could remember a command like `merge-base --is-ancestor`. So this alias just gives it a friendly name.

```sh
contains = merge-base --is-ancestor
```

I don't need this one often, but when I do, it saves me from having to look something up.

## `git pub`

I open GitHub pull requests a lot, and my usual workflow for doing so is to branch off of `master`, make some commits, then push the branch and open a PR from the GitHub website. A slight annoyance in this process in the past for me was remembering to set the remote branch name before I could push the new branch to the remote repository. This alias does it for me.

```sh
pub = "!git push -u origin $(git rev-parse --abbrev-ref HEAD)"
```

Just create a new branch, make some commits, run `git pub` (short for "publish" is how I remember it), and BAM your branch is ready to merge.

I just recently started trying to integrate GitHub's own [hub] command line tool into my workflow, which may make this alias obsolete. I've sure gotten really good use out of it so far though, and I hope you do too.

## `git all`

This last one I want to share I think is pretty interesting. As you might guess, I have a _lot_ of different Git repositories cloned on my development computers. Some of those sit for a long intervals before I touch them, and some of them I work on from multiple machines throughout the week. I then will take the occasional chore of `cd`-ing into each of these projects and doing a `git pull --rebase` in each one.

This alias lets me speed up that process, and also has come in handy for other things as well. All it does is run a given Git command for every repository found in the current directory tree. For example, running `git all status` inside a directory `$HOME/src` (where I keep all my Git clones) will print the current status for every single repo underneath the `src` directory. Pretty neat, eh?

The biggest use I get out of this alias is making sure I have everything committed. Sometimes I'll work on a project and then forget about it for a little while without committing. When I want to double-check that I'm not forgetting anything, I run `git all status --short` to quickly scan which repos have uncommitted changes that I need to take care of.

```sh
all = "!find . -type d -name .git -prune -execdir pwd ';' -execdir git $@ ';' #"
```

I have also been thinking about how to improve this alias with the option of specifying a list of repositories or maybe a regular expression the directory name has to match, but I haven't needed such a thing yet.

_Also, I double-checked that the flags in this alias are compatible with the BSD core commands, so enjoy macOS users! You'd be surprised at how much some commands deviate from each other between Linux and BSD. My first three iterations of this alias were non-portable._


[dotfiles]: https://github.com/sagebind/dotfiles
[Git]: https://git-scm.com/
[hub]: https://hub.github.com/
