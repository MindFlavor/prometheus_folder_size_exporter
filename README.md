# Prometheus Folder size Exporter

[![legal](https://img.shields.io/github/license/mindflavor/prometheus_folder_size_exporter.svg)](LICENSE)  [![alpha](https://img.shields.io/badge/stability-alpha-yellow.svg)](https://img.shields.io/badge/stability-alpha-yellow.svg)

[![Crate](https://img.shields.io/crates/v/prometheus_folder_size_exporter.svg)](https://crates.io/crates/prometheus_folder_size_exporter) [![cratedown](https://img.shields.io/crates/d/prometheus_folder_size_exporter.svg)](https://crates.io/crates/prometheus_folder_size_exporter) [![cratelastdown](https://img.shields.io/crates/dv/prometheus_folder_size_exporter.svg)](https://crates.io/crates/prometheus_folder_size_exporter)

[![tag](https://img.shields.io/github/tag/mindflavor/prometheus_folder_size_exporter.svg)](https://github.com/MindFlavor/prometheus_folder_size_exporter/tree/0.1.0)
[![release](https://img.shields.io/github/release/MindFlavor/prometheus_folder_size_exporter.svg)](https://github.com/MindFlavor/prometheus_folder_size_exporter/tree/0.1.0)
[![commitssince](https://img.shields.io/github/commits-since/mindflavor/prometheus_folder_size_exporter/0.1.0.svg)](https://img.shields.io/github/commits-since/mindflavor/prometheus_folder_size_exporter/0.1.0.svg)

## Intro

A Rust Prometheus exporter for folder size. This tool exports the folder size information (optionally including every subdir) in a format that [Prometheus](https://prometheus.io/) can understand. 

## Prerequisites 

* You need [Rust](https://www.rust-lang.org/) to compile this code. Simply follow the instructions on Rust's website to install the toolchain. If you get wierd errors while compiling please try and update your Rust version first (I have developed it on `rustc 1.33.0-nightly (ceb251214 2019-01-16)`). 

## Compilation

To compile the latest master version:

```bash
git clone https://github.com/MindFlavor/prometheus_folder_size_exporter.git
cd prometheus_folder_size_exporter
cargo install --path .
```

If you want the latest release you can simply use:

```bash
cargo install prometheus_folder_size_exporter
```

## Usage

Start the binary with `-h` to get the complete syntax. The parameters are:

| Parameter | Mandatory | Valid values | Default | Description |
| -- | -- | -- | -- | -- | 
| `-v` | no | <switch> | | Enable verbose mode.
| `-p` | no | any valid port number | 9974 | Specify the serivce port. This is the port your Prometheus instance should point to.
| `-i` | yes | a valid config json file | - | The configuration file. This json is detailed below (you can find an example here: [example.json](example.json)).

Once started, the tool will listen on the specified port (or the default one, 9974, if not specified) and return a Prometheus valid response at the url `/metrics`. So to check if the tool is working properly simply browse the `http://localhost:9974` (or whichever port you choose).

### JSON configuration file

The json is very simple, it's just an array of obejcts with two mandatory fields: `path` and `recursive`. `path` can be any valid path while `recursive` can be either `true` or `false`. If you specify `true`, the exporter will scan the folder contents recursively effectively giving you the folder tree size. For an example please refer to [example.json](example.json). 

### Systemd service file

Now add the exporter to the Prometheus exporters as usual. I recommend to start it as a service. My systemd service file is like this one:

```
[Unit]
Description=Prometheus Folder size Exporter
Wants=network-online.target
After=network-online.target

[Service]
User=node_exporter
Group=node_exporter
Type=simple
ExecStart=/usr/local/bin/prometheus_folder_size_exporter -i /etc/prometheus_folder_size_exporter.json -p 9974

[Install]
WantedBy=multi-user.target
```

