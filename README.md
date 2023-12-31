# `asusctl` for ASUS ROG

[Become a Patron!](https://www.patreon.com/bePatron?u=7602281) - [Asus Linux Website](https://asus-linux.org/)

**WARNING:** Many features are developed in tandem with kernel patches. If you see a feature is missing you either need a patched kernel, or v6.1 which has all my work merged upstream.

`asusd` is a utility for Linux to control many aspects of various ASUS laptops
but can also be used with non-asus laptops with reduced features.

Now includes a GUI, `rog-control-center`.

## Kernel support

**The minimum supported kernel version is 5.17**

**For TUF laptops, the minimum supported kernel version is 6.1**

## Goals

1. To provide an interface for rootless control of some system functions most users wish to control such as fan speeds, keyboard LEDs, graphics modes.
2. Enable third-party apps to use the above with dbus methods
3. To make the above as easy as possible for new users
4. Respect the users resources: be small, light, and fast

Point 3 means that the list of supported distros is very narrow - fedora is explicitly
supported. All other distros are *not* supported (while asusd might still run fine on them).
For best support use fedora 36+ Workstation.

Point 4? asusd currently uses a tiny fraction of cpu time, and less than 1Mb of ram, the way
a system-level daemon should.

## Discord

[Discord server link](https://discord.gg/WTHnqabm)

## SUPPORTED LAPTOPS

Most ASUS gaming laptops that have a USB keyboard. If `lsusb` shows something similar
to this:

```
Bus 001 Device 002: ID 0b05:1866 ASUSTek Computer, Inc. N-KEY Device
```

then it may work without tweaks. Technically all other functions except the LED
and AniMe parts should work regardless of your latop make.

## Implemented

- [X] System daemon
- [X] GUI app (includes tray and notifications)
- [X] Setting/modifying built-in LED modes
- [X] Per-key LED setting
- [X] Fancy LED modes (See examples) (currently being reworked)
- [X] AniMatrix display on G14 and M16 models that include it
- [X] Set battery charge limit (with kernel supporting this)
- [X] Fan curve control on supported laptops (G14/G15, some TUF like FA507)
- [X] Toggle bios setting for boot/POST sound
- [X] Toggle GPU MUX (g-sync, or called MUX on 2022+ laptops)

# GUI

A gui is now in the repo - ROG Control Center. At this time it is still a WIP, but it has almost all features in place already.

![](/extra/system.png)
![](/extra/fan-curves.png)
![](/extra/keyboard.png)

# BUILDING

Requirements are rust >= 1.57 installed from rustup.io if the distro provided version is too old, and `make`.

**Ubuntu (unsuported):**

    apt install libgtk-3-dev libpango1.0-dev libgdk-pixbuf-2.0-dev libglib2.0-dev cmake libclang-dev libudev-dev libayatana-appindicator3-1
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
    make
    sudo make install

**popos (unsuported):**

    sudo apt install cmake libclang-dev libudev-dev libgtk-3-dev libclang-dev libglib2.0-dev libatkmm-1.6-dev libpangomm-1.4-dev librust-gdk-pixbuf-dev
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
    make
    sudo make install


**fedora:**

    dnf install cmake clang-devel systemd-devel glib2-devel cairo-devel atkmm-devel pangomm-devel gdk-pixbuf2-devel gtk3-devel libappindicator-gtk3
    make
    sudo make install

**openSUSE:**

Works with KDE Plasma (without GTK packages)

    zypper in -t pattern devel_basis
    zypper in rustup make cmake systemd-devel clang-devel llvm-devel gdk-pixbuf-devel cairo-devel pango-devel freetype-devel gtk3-devel libexpat-devel libayatana-indicator3-7
    make
    sudo make install

## Installing
- Fedora copr = https://copr.fedorainfracloud.org/coprs/lukenukem/asus-linux/
- openSUSE = https://download.opensuse.org/repositories/home:/luke_nukem:/asus/
- Ubuntu = not supported due to packaging woes, but you can build and install on your own.

=======

The default init method is to use the udev rule, this ensures that the service is
started when the device is initialised and ready.

If you are upgrading from a previous installed version, you will need to restart the service or reboot.

```
$ systemctl daemon-reload && systemctl restart asusd
```

You may also need to activate the service for debian install. If running Pop!_OS, I suggest disabling `system76-power` gnome-shell extension and systemd service.

## Uninstalling

Run `sudo make uninstall` in the source repo, and remove `/etc/asusd/`.

# Contributing

See `CONTRIBUTING.md`. Additionally, also do `cargo clean` and `cargo test` on first checkout to ensure the commit hooks are used (via `cargo-husky`).

Generation of the bindings with `make bindings` requires `typeshare` to be installed.

Dbus introsepction XML requires with `make introspection` requires `anime_sim` to be running before starting `asusd`.

# OTHER

## AniMe Matrix simulator

A simulator using SDL2 can be built using `cargo build --package rog_simulators` and run with `./target/debug/anime_sim`. Once started `asusd` will need restarting to pick it up. If running this sim on a laptop *with* the display, the simulated display will be used instead of the physical display.

## Supporting more laptops

Please file a support request.

# License & Trademarks

Mozilla Public License 2 (MPL-2.0)

---

ASUS and ROG Trademark is either a US registered trademark or trademark of ASUSTeK Computer Inc. in the United States and/or other countries.

Reference to any ASUS products, services, processes, or other information and/or use of ASUS Trademarks does not constitute or imply endorsement, sponsorship, or recommendation thereof by ASUS.

The use of ROG and ASUS trademarks within this website and associated tools and libraries is only to provide a recognisable identifier to users to enable them to associate that these tools will work with ASUS ROG laptops.

---
