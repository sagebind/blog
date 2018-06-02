+++
title = "Guide: Setting Up A Simple C++ Development Environment On Windows"
author = "Stephen Coakley"
date = "2015-01-21"
category = "cplusplus"
+++

This is a guide for people just starting out with C++ and need an easy way to write and run C++ programs on their Windows machine. If that sounds like you, then hopefully this guide will be of some help to you.

## Step 1: Choose an editor
The first thing you will need to make your first C++ program is a text editor. There are a *ton* of great editors out there and there isn't a particular one that is the best, but I will point you in the right direction with ones that I recommend after years of trying different editors.

Some of the editors I would recommend are:

- Sublime Text 3 ([download](http://www.sublimetext.com/3))
- Notepad++ ([download](http://notepad-plus-plus.org/download/))
- Visual Studio Community 2013 ([download](http://www.visualstudio.com/en-us/products/visual-studio-community-vs))

<figure>
    <a class="sb" href="/content/images/2015-01-21-sublime-text.png"><img src="/content/images/2015-01-21-sublime-text.png"></a>
    <figcaption>Sublime Text 3, a lightning-fast, powerful cross-platform editor that hides behind a sleek interface.</figcaption>
</figure>

<figure>
    <a class="sb" href="/content/images/2015-01-21-notepadplusplus.png"><img src="/content/images/2015-01-21-notepadplusplus.png"></a>
    <figcaption>Notepad++, a capable Windows-only editor with an easy-to-understand interface and tons of features.</figcaption>
</figure>

<figure>
    <a class="sb" href="/content/images/2015-01-21-visual-studio-community.png"><img src="/content/images/2015-01-21-visual-studio-community.png"></a>
    <figcaption>Visual Studio Community 2013 Edition, a free version of Microsoft's powerful Visual Studio IDE that supports many languages, compiling, GUI-building, and debugging out of the box.</figcaption>
</figure>

All three of the above editors can be used for free (Sublime Text has an unlimited evaluation period, see [the FAQ](http://www.sublimetext.com/sales_faq)) and are capable editors that, in my opinion, have a smaller learning curve than comparable editors out there.

Of those three, my strongest recommendation (and personal favorite) is Sublime Text. It's probably the most lean and lightweight program of the three, it's cross platform, and has an awesome plugin ecosystem around it where you can easily install packages to add any extra functionality you may need.

## Step 2: Install a compiler

If you are using Visual Studio, then you're done! Visual Studio comes with Microsoft's Visual C++ compiler, so no further steps are necessary. If not, then you'll need a separate C++ compiler. Microsoft used to distribute their compiler with their Windows C++ SDK, but today the only easy way to get it is to install Visual Studio.

There is a catch, though. Microsoft's compiler isn't quite the same as the standard [GNU Compiler Collection](http://gcc.gnu.org) and code that compiles in one may not necessarily compile in another. If you'll need to submit your programs in source code form, for like a school project, you may be better off with using a GCC-like compiler.

If you are using Notepad++ or Sublime Text, then you will also need to install [MinGW-w64](http://MinGW-w64.sourceforge.net). There are a couple other C/C++ compilers and software for running GCC on Windows out there, namely [Cygwin](https://www.cygwin.com/), but I've found anything based on MinGW to be standards-supporting and very lightweight. MinGW-w64 is a fork of MinGW that distributes compilers in simple installers and includes 64-bit support by default.

To install MinGW-w64 for Windows, visit the downloads page [here](http://MinGW-w64.sourceforge.net/download.php#mingw-builds) and click on the "SourceForge" link to start the download. After the download is complete, double-click on the file to run the installer. You will probably want to change the installation settings to something similar to the following:

<a class="sb" href="/content/images/2015-01-21-mingw-w64-install-settings.png"><img src="/content/images/2015-01-21-mingw-w64-install-settings.png"></a>

By default, the installer will install MinGW-w64 into a unique folder for the version downloaded. This is handy if you want to install multiple compiler versions, but isn't necessary for most situations. To make build files simpler, remove the last folder part of the install path. It should look something like this:

<a class="sb" href="/content/images/2015-01-21-mingw-w64-install-path.png"><img src="/content/images/2015-01-21-mingw-w64-install-path.png"></a>

After the installation is complete, you should have a Windows-compatible GCC C++ compiler installed and ready to go. If you want to use the compiler from the command line directly, you can. Check out [this quick guide](http://pages.cs.wisc.edu/~beechung/ref/gcc-intro.html) for typical usage, or check [GNU's extensive documentation](https://gcc.gnu.org/onlinedocs/) for details. Keep in mind that the binaries are not added to your `%PATH%` automatically, so you may want to do so manually if you want to call the compiler by hand.

## Step 3: Set up your editor

It is fairly easy to configure Sublime Text and Notepad++ to use the MinGW-w64 compiler for your C++ files:

### Sublime Text
Sublime Text has a build system already built-in, but the C/C++ builder doesn't work properly with MinGW out of the box. Instead, we will create a new build file that will use the MinGW-w64 compiler we installed. I took the liberty of making such a file:

<script src="https://gist.github.com/sagebind/9039773048a3900fa49a.js"></script>

Just download the above script and put it into Sublime Text's package folder. You can open this folder by clicking "Preferences > Browse Packages...". Depending on your installation settings, you may need to tweak the `path` property to the correct folder of your MinGW-w64 binaries folder.

That's it! Now, to compile a C++ file, just open it in Sublime Text and hit Ctrl+B (or by clicking "Tools > Build"). Out will come an executable `.exe` file with the same name as the main `.cpp` file that you compiled. You can even run the file immediately after compiling by hitting Ctrl+Shift+B. Your program's output will be displayed in the output window at the bottom of Sublime Text.

<a class="sb" href="/content/images/2015-01-21-sublime-text-build.png"><img src="/content/images/2015-01-21-sublime-text-build.png"></a>

Note that this doesn't work for programs that take input (`std::cin` and the like); you will need to run such programs from the command line or by opening them from file explorer.

Also note that this compiles other `.cpp` files in the same folder as the main file as well, so organizing your programs into separate folders is a great plan. This script does not work with files in subfolders, a result of Windows' poor support of wildcards in the shell. If you need to organize files into subfolders, you will need to use the compiler by hand and manually specify the files to compile.

### Notepad++
Notepad++ doesn't have a way of running build commands built-in, but you can use the standard NppExec plugin to run arbitrary commands. We can use this to run the compiler for us. First, open the plugin manager (Plugins > Plugin Manager > Show Plugin Manager). Find "NppExec" in the "Available" tab, check the checkbox, and click "Install".

<a class="sb" href="/content/images/2015-01-21-notepadplusplus-plugin-manager.png"><img src="/content/images/2015-01-21-notepadplusplus-plugin-manager.png"></a>

Now we can open the plugin and tell it how to run the compiler. Open NppExec by going to "Plugins > NppExec > Execute" (or the F6 key) and paste in the following code:

```sh
"C:\Program Files\MinGW-w64\mingw64\bin\g++.exe" -o "$(CURRENT_DIRECTORY)\$(NAME_PART).exe" -static-libgcc -static-libstdc++ "$(CURRENT_DIRECTORY)\*.cpp"
```

Depending on your installation settings, you may need to tweak the first part of the command to point to the correct path of your MinGW-w64 compiler.

You can save this script to use later by clicking the "Save" button and giving it a name. Pressing the "OK" button will try to compile the currently open file. Now when you are writing your C++ programs, to compile, simply open up your main file, open NppExec, and run the script. Out will come an executable `.exe` file with the same name as the `.cpp` file, which you can run from the command line.

<a class="sb" href="/content/images/2015-01-21-notepadplusplus-build.png"><img src="/content/images/2015-01-21-notepadplusplus-build.png"></a>

Just like the Sublime Text script, this compiles other `.cpp` files in the same folder as the main file but not files in subfolders. Again, if you need to organize files into subfolders, you will need to use the compiler by hand.

## Step 4: Write code!

If you reached this step, then you should be ready to code! If you're using Visual Studio, then most of this guide was useless except for my recommendation. If you want to avoid these steps and you really don't care what editor you use, you can't go wrong with Visual Studio. Sublime Text and Notepad++ are considerably more lightweight than Visual Studio, so if you want something simpler or lighter on system resources, then either of the two should satisfy.

Ultimately it's up to you how you want to write your code. Just keep learning programming and C++; programming is such a valuable skill in our software-centric world today.

---

## Why I wrote this guide
One of my regular jobs since last fall is peer tutoring at my university, [University of Wisconsin-Whitewater](http://uww.edu). I provide walk-in tutoring for Computer Science students each week, with my primary subject of expertise as C++. I first started tutoring near the beginning of the fall semester of 2014 and by the last month of the semester I had so many students asking for help I couldn't get to all of them.

Something I noticed whilst providing assistance was how bad the development workflow was for these students. The school provides a Linux server that students can SSH into to write and compile their C++ code, which isn't accessible offline or off-campus without a VPN key. Which is awesome, but most budding programmers don't find [Vim](http://www.vim.org) or the command line too intuitive. Don't get me wrong, Vim is pretty powerful, but it takes a lot of work to learn and they are already trying to wrangle C++ for the first time. Hopefully after completing this guide, coders just starting out with C++ will find the learning process easier and faster.
