+++
title = "The Littlest Development Device: Follow Up"
author = "Stephen Coakley"
date = "2018-08-25"
tags = ["life", "hardware"]
+++

As promised, I have returned to post a follow up to [my article last month](/2018/07/29/the-littlest-development-device). In it, I ask if the GPD Pocket computer can be used as a development machine and, if so, what the experience is like.

Well, first thing on the table: It actually works. I was surprised at how much this little device could accomplish in such a small form factor. The performance definitely won't _wow_ you, but it has enough punch to run everything I use to do development, including:

- Ubuntu
- Firefox
- Visual Studio Code
- Docker
- Rust
- .NET Core
- GCC

Pretty much everything I needed worked smoothly out of the box. Firefox needed a few tweaks in order for it to cooperate with the touch screen (which turned out to be an absolutely essential input device).

Now for things that _didn't_ work:

- GNOME 3: I really enjoy GNOME on my primary laptop, but it proved to be just too graphics intensive for the little Pocket. I switched to XFCE instead, which performed excellently. A shame too, since GNOME worked brilliantly with the touch screen when it wasn't stuttering.
- [Anbox]: I was hoping to be able to run some Android apps while running Linux on the Pocket. It sounded like the ideal mobile computing experience, but it seems that the Atom chip lacks certain CPU instructions for it to work properly or some such thing. Maybe someday it will work, or maybe the Pocket 2 will fare better.

As far as the form factor goes, the screen is quite small, but just big enough to _not_ be painful for coding. I found myself using touch screen quite a bit more than I anticipated. I'm not sure if it was because the size of the device makes reaching for the screen that much more intuitive, or if the other means of input were simply lacking.

Input devices are about as good as you'd expect. The keyboard is actually really good considering its size, but it is quite cramped which makes long typing sessions unlikely. I wouldn't recommend it as your primary computer for that reason alone, but for short typing sessions on the go it works rather well. The pointing stick is actually really fun and dare I say more useful than a small touch pad would have been.

So what would I conclude? Well, as soon as my primary laptop was back from the shop I switched back to it immediately. As a primary device, the Pocket is simply painful. Does it have a use? Well... yes! I found myself reaching for it quite a bit while travelling, and as a developer I found it to be a nearly _perfect_ travelling companion. Its small size makes it easy to take with you anywhere, and its capabilities are vastly superior to a smartphone or average tablet. If you aren't insane like me and would prefer _not_ to hack away at a terminal while on the go, then a large smartphone will certainly treat you better.


[Anbox]: https://anbox.io
