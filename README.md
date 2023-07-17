
# Platform-Specific Instructions

*(note: these instructions are only required if you want
to host from one of these platforms. running clicktogether
as a client should work with no further setup.)*

## Linux

**clicktogether currently only works on X (no wayland support).**
we are using the [enigo](https://github.com/enigo-rs/enigo)
rust library to simulate input, which does not yet support
wayland.

'xdotool' is a runtime dependency. see
[this page](https://github.com/jordansissel/xdotool#installation)
for information about how to install on your distribution.


## OSX

Open 'System Preferences' and go to 'Privacy & Security'
-> 'Accessibility' (in the sidebar), and check the box
for the terminal emulator you are using.

## Windows

clicktogether should work on windows without needing to do
anything extra.
