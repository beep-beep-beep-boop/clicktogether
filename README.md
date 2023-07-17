# clicktogether

a clicktogether server will click a specified keyboard key
once all clients have clicked theirs.

intended for reading books/visual novels/etc with multiple
people.

## usage

### host

`clicktogether host <KEY>` where \<KEY\> is the keyboard key you want
the server to press.  
use `" "` for space and `\n` for return.

### client

`clicktogether join <ADDRESS> <USERNAME>` where \<ADDRESS\> is the
address to the server including the port *without a trailing slash*
and \<USERNAME\> is the username you want to use.


## Platform-Specific Setup

*(note: these instructions are only required if you want
to host from one of these platforms. running clicktogether
as a client should work with no further setup.)*

### Linux

**the clicktogether server currently only works on X (no wayland
support).**  
we are using the [enigo](https://github.com/enigo-rs/enigo)
rust library to simulate input, which does not yet support
wayland.

'xdotool' is a runtime dependency. see
[this page](https://github.com/jordansissel/xdotool#installation)
for information about how to install on your distribution.


### OSX

Open 'System Preferences' and go to 'Privacy & Security'
-> 'Accessibility' (in the sidebar), and check the box
for the terminal emulator you are using.

### Windows

clicktogether should work on windows without needing to do
anything extra.
