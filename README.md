# Monprof

**This project is WORK IN PROGRESS**.  
Implementation is not yet finished, and this tool will not provide all features yet!

## Description

Monitor profile management and automatic switching for Linux desktops.

`monprof` is a command line utility that allows you to manage different sets of monitor profiles.
Monitor profiles are a set of instructions that should apply to your system depending on which monitors are currently
connected to the system.

It provides this functionality in case your Desktop Environment does not come with such a feature, or you are not using
a full Desktop Environment in favour of a Window Manager like i3, Sway, Hyprland, Awesome,
bspwm, etc..

When monitors are connected or disconnected from the system, a new set of rules might need to be applied to the system.
The `monprofd` daemon can watch your system for changes in connected monitors and automatically apply the correct
monitor profile when necessary.

## Example

Imagine using a laptop computer with its built-in screen. When connecting to a docking station, an external monitor
might be available through the docking station. We expect the system to automatically enable the external monitor and
use it in addition to, or instead of, the built-in screen.

If you are using a window manager, this often comes with additional requirements such as refreshing the desktop
wallpaper, status bars, windows repositioning etc.

When the laptop computer is disconnected from the docking station, we expect the system to revert back to the built-in
screen.

In this example scenario, assume the built-in screen to be available in the system with the port identifier `e-DP1`,
while the external monitor might be available on different port identifiers, depending on when and where the docking
station get's connected.

The following monitor profiles might exist to handle this use case:

`$XDG_CONFIG_HOME/monprof/default.toml` contains the default profile for the built-in screen.

```toml
default_profile = true # This profile should apply if no other profile is applicable
priority = 99 # If multiple profiles are applicable, this profile has a very low priority (0 is the highest priority)

# This profile should be applied, if the following monitors are present
[[match.monitor]]
# This profile should be applied, if a monitor on port "e-DP1" is available.
port = '^e-DP1$' # Match a monitor on port e-DP1
available = true # The monitor on port e-DP1 should be available

[settings]
# to be defined
```

`$XDG_CONFIG_HOME/monprof/docked.toml` contains the profile that should apply once the laptop is docked.

```toml
priority = 10 # If multiple profiles are applicable, this profile has a high priority

# This profile should be applied, if any external monitors are present
[[match.monitor]]
# This profile should be applied, if a monitor on any port other than "e-DP1" is available.
port = '^(?!e-DP1)$' # Match all monitors except for eDP-1
available = true # The monitor should be available

[settings]

# to be defined
```
