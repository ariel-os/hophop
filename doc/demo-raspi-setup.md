<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
# Setup for workshop and demo sessions

This document describes a setup for Raspberry Pi CM5 style devices,

The setup is not generally recommended, but for specific situations:
- Multiple (workshop participants, or separate experiments) should be able to use the host at the same time.
- It's a short-term setup: it gets run once shortly before the event; incremental updates would be tedious and inefficient.

## Target audience

It is assumed that users of the system are familiar with general programming, some installed editor, and working with a Linux system as a user.

**Warning:** This guide includes the use of `sudo` and `dd`, also dubbed "disk destroyer".
You MUST to understand the commands you run, and check the arguments.

## Tested hardware

So far, this was only run on a Raspberry Pi CM5 in IO case,
with 16GiB RAM and 64GiB internal storage.

A USB stick is used during setup.

## Setup steps

* Obtain the latest Raspberry Pi OS image from <https://www.raspberrypi.com/software/operating-systems/>.
* Unpack onto your USB stick (**make sure you pick the out file (`of=`) device**):

  ```console
  $ xzcat 2026-10-06-raspios-trixie-arm64.img.xz | sudo dd of=/dev/sdz
  ```

* Insert into Pi, connect a screen and keyboard, power it up.
* In the running system, run `rp-imager` to install onto the internal storage:
  - Pick a system name (latest demo uses "hophost1" for the wired Pi and "hophost2" for the Wifi one)
  - Pick your user and an arbitrary password (will be deleted in a moment)
  - Pick your location but English and US layout (or adjust to target audience)
  - No WiFi settings
  - EnableSSH login
* Reboot and unplug USB stick
* `ssh-copy-id` into the system
* Remove sudo password requirement (`visudo` / `%sudo   ALL=(ALL:ALL) NOPASSWD: ALL`)
* Remove password authentication for your user:

  ```console
  $ sudo passwd -l myusername
  ```

* Install base packages needed for Ariel:

  ```
  $ sudo apt install git ninja-build pkg-config libudev-dev clang gcc-arm-none-eabi gcc-riscv64-unknown-elf gcc curl make
  ```

* Install a few things as root so that they are available for all users (who don't need to change anything about that).
  Run in a single session or re-export the environment variables from earlier.

  System setup (with apt installations both from Ariel and additional ones starting with wireshark):

  ```console
  $ sudo su
  # echo 'export RUSTUP_HOME=/opt/rust' | sudo tee -a /etc/profile.d/rust.sh
  # echo 'export PATH=${PATH}:/opt/rust/bin' | sudo tee -a /etc/profile.d/rust.sh
  ```

  Install Rust:

  ```console
  # source /etc/profile.d/rust.sh
  # export CARGO_HOME=/opt/rust
  # curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
  # rustup component add llvm-tools rust-analyzer
  ```

  Install tools with Rust and clean up afterwards:

  ```console
  # cargo install cargo-generate laze probe-rs-tools
  # cargo install Jelly --git https://github.com/chrysn-pull-requests/jelly/ --branch core.p-edit
  # rustup target add thumbv8m.main-none-eabihf
  # echo "source <(COMPLETE=bash laze)" | sudo tee -a /etc/skel/.bashrc
  # rm -rf /opt/rust/registry
  ```

* Install Wireshark with a DECT dissector, and a modern editor (konsole is for kate's built-in console view).

  Edit /etc/apt/sources.list.d/debian.sources: Copy the trixie section to say forky, and add `Include: wireshark wireshark-common` (not sure that really works; but it's a one-shot setup, so a Frankendebian is OK).

  ```console
  # apt update
  # apt install wireshark kate konsole
  ```

  … and select that regular users should be allowed to use wireshark (`dpkg-reconfigure -plow wireshark-common` if you missed it).

* Speaking of permissions: Following [probe-rs setup instructions](https://probe.rs/docs/getting-started/probe-setup/), run:

  ```console
  # wget https://probe.rs/files/69-probe-rs.rules -O /etc/udev/rules.d/69-probe-rs.rules
  ```

* As some user (e.g. your own), run a few demo steps to ensure that everything is in place --
  especially as it will be copied into the skeleton (as downloads such as CMSIS\_5 or sdk-nrfxlib are huge).

  ```console
  $ git clone https://github.com/ariel-os/hophop
  $ laze -C hophop/examples  build -b makerdiary-nrf9151-connect-kit run --bin rssi
  $ laze -C hophop/applications/embedded-pt build -b makerdiary-nrf9151-connect-kit
  $ laze -C hophop/applications/bridge-pt build -b makerdiary-nrf9151-connect-kit
  $ sudo cp -a ~/.cargo ~/hophop /etc/skel/
  ```

  (A lot of the build outputs change later, so everything appears to be rebuilt, because paths changed, but it seems that most of the binary data does get reused).

* For WiFi hosts: Use nmtui or nm-applet to add a network profile named "access point", mode "access point", SSID following the hostname, and some funny password (as of 2026-10-12, that is "four"), IPv4 and IPv6 to "shared".

  WiFi may need to be enabled in the Radio top-level group of nmtui.

* Creating some users:

  ```console
  # adduser YOURUSER wireshark
  # for user in guest-a guest-b guest-c
  > do
  > adduser $user && adduser $user plugdev && adduser $user wireshark
  > done
  ```

  (picking any funny password as above)

* Preparations from the GUI:

  - Set up wireshark as per [../applications/sniffer/README.md](sniffer README)
  - Add hophop website (repo) and Ariel OS website as bookmarks in the default browser in the bookmark bar group, and remove the predefined entries
  - Kate: set font to DejaVu Sans mono
