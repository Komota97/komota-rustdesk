# Komota RustDesk

A [KOMOTA](https://www.komota.lt)-branded build of
[RustDesk](https://github.com/rustdesk/rustdesk), the open-source remote
desktop application.

KOMOTA is a computer repair and IT service company in Lithuania, trading since
1997. This client is what our technicians use to provide remote support to
customers, and it connects only to our own self-hosted relay infrastructure
rather than to any public service.

## What this fork changes

Everything below is a configuration/branding change; no functional changes are
made to RustDesk's remote-desktop implementation.

| Change | Why |
|---|---|
| Relay is `rustdesk.komota.lt`, not `rs-ny.rustdesk.com` | Sessions run entirely on infrastructure we operate |
| App is named `Komota RustDesk` | Customers recognise who is connecting |
| Installation is **disabled** | Ships as a single portable `.exe`; it must never install itself or create a service on a customer's machine |
| Incoming-only home screen | The client is for *receiving* support, not initiating it |
| Unattended access with password-or-click approval | Support can begin without a customer needing to navigate settings |
| Tray, service, password, ID and PIN controls locked | Prevents accidental misconfiguration on a customer machine; a technician can still reconfigure from the command line |
| KOMOTA logo and Lithuanian strings | Branding |

The full set lives in `libs/hbb_common/src/config.rs` and
`flutter/windows/runner/Runner.rc`.

## Transparency about what the client does

Because this client is installed on customers' machines, it's worth stating
plainly: it is configured for **unattended access**, meaning a technician
holding the ID and password can connect without someone clicking Accept. That
is deliberate — it is what makes remote support possible when a customer's
machine is unusable — and it is why the client is branded so that it is always
obvious whose software is running.

## Building

Windows builds run on GitHub-hosted runners via
[`.github/workflows/komota-windows.yml`](.github/workflows/komota-windows.yml).
The build is fully automated from this repository's source: the workflow
verifies the branding is present, builds with the same toolchain versions
upstream RustDesk pins, and produces:

- `Komota RustDesk <version> x86_64.exe` — portable, self-extracting
- `Komota RustDesk <version> x86_64.zip` — the same build as a folder

The submodule `libs/hbb_common` points at
[Komota97/hbb_common](https://github.com/Komota97/hbb_common), which carries the
relay and app-name configuration.

## Licence and attribution

RustDesk is licensed under **AGPL-3.0**, and so is this fork. See
[`LICENCE`](LICENCE).

All credit for RustDesk itself belongs to
[the RustDesk project and its contributors](https://github.com/rustdesk/rustdesk).
This repository exists to satisfy the corresponding-source obligation for the
modified binaries we distribute, and to allow those binaries to be built
verifiably from public source.

Upstream release this is based on: **1.4.9**.
