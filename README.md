# LibreQoS cnMaestro Helper

This package provides some linkage between LibreQoS and cnMaestro. cnMaestro isn't a CRM - so it can't provide very much 
in the way of mappings between users and plans. But it *is* a management system, that can keep track of things like 
updated IP addresses, what devices are connected where - and which devices aren't listed in your `ShapedDevices.csv` file.

## What Does This Do?

The program:

* Reads your `ShapedDevices.csv` file.
* Connects to cnMaestro and obtains all devices and their statistics (needed for AP mapping).
* Maps MAC addresses between `ShapedDevices` and cnMaestro.
* When a mapped device has a different IP address in `ShapedDevices`, the address from cnMaestro is used instead.
* Writes `ShapedDevices.csv`

## How to Use This

1. Clone the repo
2. Build the Rust program with `cargo build --release`
3. Copy `cnmaestro_meta_integration` from `target/release` into your LibreQoS installation's `bin` directory.

Now that its in place, you need to configure it. You can either wrap it in a script that 
creates environment variables, or you can make a `.env` file. A `.env` file would look
like this:

```bash
CNMAESTRO_USERNAME="<your token username>"
CNMAESTRO_SECRET="<your token secret>"
CNMAESTRO_URL="<https//whereever.i.put.cnmaestro>"
SHAPED_DEVICES_PATH="/opt/libreqos/src/ShapedDevices.csv" # Or wherever you installed it
```

(Place your `.env` file in the same directory as the binary)

You can obtain a username/secret by going into cnMaestro, and going to `Services -> API Client`.

Now do a test run. `/opt/libreqos/src/bin/cnmaestro_meta_integration`

If all is well, it will tell you about your network - and update some IP addresses.

