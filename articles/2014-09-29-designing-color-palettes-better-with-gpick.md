+++
title = "Designing Color Palettes Better with Gpick"
author = "Stephen Coakley"
date = "2014-09-29"
tags = ["design"]
+++

When you are designing a website, colors are your bread and butter. Layout, balance, and organization are all important, but if your colors are bland, flashy, or poorly coordinated, then all your work will have been for naught. This applies to pretty much any other kind of design as well as web design. Great color schemes can make or break a website design, and a great color picker should be a tool that every designer should have in their arsenal even if you only use it occasionally.

[Gpick](https://code.google.com/p/gpick/) is an awesome color picker desktop app that works on both Windows and Linux. Tons of color picker extensions are available for popular web browsers, but desktop apps have the advantage of being able to select colors from any program or screen, not just from web pages. Gpick follows your mouse cursor and displays the color under your cursor in its window. It also has an optional magnifying glass that follows your cursor, but that feature didn't work for me on Windows 8.

![Gpick](/content/images/2014-09-29-gpick.png)

Web designer-useful features include its color format support and auto-naming. I had a hard time finding a program that let me get a color value in HSL (Hue-Saturation-Lightness), my preferred format in CSS, but Gpick supports them all: HSV, HSL, RGB, LCH, CMYK, and of course hex RGB. Gpick is also able to automatically give a name to any color, finding the closest named color in its database. This is an incredibly useful and convenient feature if you are using a CSS preprocessor like Sass. Life is easier if you have [Sass color variables that don't suck](http://davidwalsh.name/sass-color-variables-dont-suck), and Gpick makes it even easier to do so than [Name That Color](http://chir.ag/projects/name-that-color), a neat color naming tool that I used previously.

The real useful features of Gpick is in color palette creation. Gpick gives you a simple interface to make a palette of multiple colors and even give them custom names. After you play around with colors and make a good-looking list of hues, you can test them out quickly in a basic table layout or web layout right in Gpick. Saving your swatches for later is easy too. Gpick can read and write GIMP/Inkscape palette files (.gpl) and Adobe Swatch Exchange files (.ase), as well as Gpick's own binary format. Gpick claims that colors are stored more accurately in their own format, but I couldn't find any noticeable difference.

I started incorporating Gpick into my web design workflow a few months ago and am much happier because of it. The HSL support makes writing SCSS much easier and saving to Inkscape palette files is a lifesaver. I use Inkscape for most of my mock-ups and Inkscape's built-in palette editor is awful. Now I simply edit my color palettes in Gpick and use them in Inkscape. If you use color pickers or think you may want to try one, I highly recommend that you give Gpick a try.
