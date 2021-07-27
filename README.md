<div align="center">
    <img src="https://raw.githubusercontent.com/mazurwiktor/albion-online-stats/master/assets/albion-stats-icon.png" width="100"/>
</div>

[![Downloads](https://pepy.tech/badge/aostats)](https://pepy.tech/project/aostats)

   
**Powered by**  <a href="https://github.com/mazurwiktor/aoaddons-python"> **pyaoaddons** </a> & <a href="https://github.com/mazurwiktor/albion-online-addons"> **albion online addons** </a> 

[![Discord](https://discordapp.com/api/guilds/639922572368150552/widget.png?style=banner2)](https://discord.gg/3U2rpyV)


Albion Online Stats
===================

Albion online stats is an extension to MMORPG game - Albion Online. It tracks network traffic and displays various statistics, such as damage and DPS meter calculated from in-game actions.

## App preview

![0.7.0](https://user-images.githubusercontent.com/11301109/71000581-d9124000-20db-11ea-8c19-4d7f69a2c155.png)

## Youtube live preview

[![Video](http://img.youtube.com/vi/Dy3YyherSmw/0.jpg)](http://www.youtube.com/watch?v=Dy3YyherSmw "Albion online stats")


# How to use this app

> **Note** this section does not require any coding knowledge. Those simple two steps are required to use this software :)

## Installation

1. On windows make sure that WinPcap is installed in your system. [Npcap 0.9983 installer for Windows Vista/2008, 7/2008R2, 8/2012, 8.1/2012R2, 10/2016 (x86 and x64)](https://nmap.org/npcap/dist/npcap-0.9983.exe) **Make sure to install with the "Install Npcap in WinPcap API-compatible Mode"**
2. Install python https://docs.python.org/3/using/
3. Download latest launcher from https://github.com/mazurwiktor/albion-online-stats/releases/latest (albion-online-stats.bat for windows albion-online-stats.sh for linux and mac)
3. Enjoy :)

## Configuration

After first execution the app is going to create default configuration file named `albion-online-stats.cfg`. Feel free to edit it according to your needs.

## Is This Allowed
<p align="center">
    <img src="assets/sbistatement.png">
</p>

- [x] Only monitors your own party
- [x] Does not modify our game client
- [x] Does not track players that are not within the player's view
- [x] Does not have an overlay to the game

> **Note** this traits of the application are true from version 0.9.0+.

## Donate

[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/wiktormazur)


# Getting started

## Prerequisites

- Python installed (python3.6+)
- *Windows only prerequisites*  https://github.com/libpnet/libpnet#windows


## All platforms
1. Clone repository:
    ```shell
    git clone https://github.com/mazurwiktor/albion-online-stats.git
    ```
2. [optional] Create and activate python virtualenv

    ```shell
    python -m venv env
    [linux/mac] . env/bin/activate
    [windows powershell] .\env\Scripts\activate.ps1
    [windows cmd.exe] .\env\Scripts\activate.bat
    ```


3. Install package

    ```shell
    python setup.py install
    ```

4. Run the application
    ```shell
    aostats
    ```

# Contribution

Check out [wiki page](https://github.com/mazurwiktor/albion-online-stats/wiki)


# License
Licensed under either of

  * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
