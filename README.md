<p align="center">
    <img src="assets/albion-stats-icon.png" width="100"> 
</p>

Albion Online Stats
===================

Albion online stats is an extention to MMORPG game - Albion Online. It tracks network traffic and displays various statistics calculated from in-game actions.


## Is This Allowed
 ```Our position is quite simple. As long as you just look and analyze we are ok with it. The moment you modify or manipulate something or somehow interfere with our services we will react (e.g. perma-ban, take legal action, whatever```

~MadDave, Technical Lead at Sandbox Interactive for Albion Online, [source](https://forum.albiononline.com/index.php/Thread/51604-Is-it-allowed-to-scan-your-internet-trafic-and-pick-up-logs/?postID=512670#post512670)

* copied from [Albion Online Data](https://www.albion-online-data.com/)

# Getting started

## Prerequisites

- Rust installed (https://www.rust-lang.org/tools/install)
- Python installed 
- *Windows only prerequisites*  https://github.com/libpnet/libpnet#windows

## All platforms
1. Clone repository:
    ```shell 
    git clone https://github.com/mazurwiktor/albion-online-stats.git
    ```
2. Build rust library

    ```shell
    cargo build --release
    ```


3. Copy library to GUI directory 

    3.1 on Linux

    ```shell
        cp target/release/libmeter.so gui/
    ```

    3.1 on Windows

    ```shell
        cp target/release/meter.dll gui/libmeter.pyd
    ```

4. Run GUI

    3.1 on Linux

    ```shell
        sudo python gui/meter.py
    ```

    3.1 on Windows

    ```shell
        python gui/meter.py
    ```


# License
Licensed under either of

  * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.