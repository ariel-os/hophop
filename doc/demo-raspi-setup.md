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

It is assumed that 

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

* Install a few things as root so that they are available for all users (who don't need to change anything about that).
  Run in a single session or re-export the environment variables from earlier.

  System setup:

  ```console
  $ sudo su
  # apt install git ninja-build pkg-config libudev-dev clang gcc-arm-none-eabi gcc-riscv64-unknown-elf gcc curl make
  # echo 'export RUSTUP_HOME=/opt/rust' | sudo tee -a /etc/profile.d/rust.sh
  # echo 'export PATH=${PATH}:/opt/rust/bin' | sudo tee -a /etc/profile.d/rust.sh
  ```

  Install Rust:

  ```console
  # source /etc/profile.d/rust.sh
  # curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
  # rustup component add llvm-tools
  ```

  Install tools with Rust and clean up afterwards:

  ```console
  # export CARGO_HOME=/opt/rust
  # cargo install cargo-generate laze probe-rs-tools
  # rustup target add thumbv8m.main-none-eabihf
  # echo "source <(COMPLETE=bash laze)" | sudo tee -a /etc/skel/.bashrc
  # rm -rf /opt/rust/registry
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

* Creating some users:

  ```console
  # for user in guest-a guest-b guest-c guest-d
  > do
  > adduser $user && adduser $user plugdev
  > done
  ```

  (with a password that is funny at the time; as of 2026-10-12, that is "four")
