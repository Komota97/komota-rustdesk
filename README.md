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

Both platforms build on GitHub-hosted runners from this repository's source.
Each workflow verifies the branding is present before it builds, and uses the
same toolchain versions upstream RustDesk pins.

[`.github/workflows/komota-windows.yml`](.github/workflows/komota-windows.yml)
produces:

- `Komota RustDesk <version> x86_64.exe` — portable, self-extracting
- `Komota RustDesk <version> x86_64.zip` — the same build as a folder

[`.github/workflows/komota-linux.yml`](.github/workflows/komota-linux.yml)
produces, for x86_64:

- `komota-rustdesk-<version>-linux-x64-<stamp>.deb`
- `Komota-RustDesk-<version>-x86_64-<stamp>.AppImage` — portable

It builds on Ubuntu 22.04 so that the artifacts do not require a newer glibc
than the machines they are meant to run on.

[`.github/workflows/komota-upstream-watch.yml`](.github/workflows/komota-upstream-watch.yml)
does not build anything: it checks weekly whether upstream RustDesk has
published a release newer than the one this fork is branded from, and fails
if so, purely as a notice.

The submodule `libs/hbb_common` points at
[Komota97/hbb_common](https://github.com/Komota97/hbb_common), which carries the
relay and app-name configuration.

## Code signing policy

Free code signing provided by [SignPath.io](https://about.signpath.io),
certificate by [SignPath Foundation](https://signpath.org).

- **Committers and reviewers:** KOMOTA staff. Changes to this repository are
  made and reviewed by KOMOTA; the branding configuration originates from
  KOMOTA's internal build repository.
- **Approvers:** KOMOTA staff. Every signing request is approved manually
  before a release is signed.

Windows binaries are built exclusively by
[the GitHub Actions workflow in this repository](.github/workflows/komota-windows.yml)
on GitHub-hosted runners, and are signed from that build's own artifact — no
binary built elsewhere is ever submitted for signing. What gets signed is
defined by [`.signpath/artifact-configuration.xml`](.signpath/artifact-configuration.xml),
kept in this repository so it can be reviewed.

### Privacy

This program will not transfer any information to other networked systems
unless specifically requested by the user or the person installing or
operating it.

The client connects only to KOMOTA's own relay infrastructure
(`rustdesk.komota.lt`) and does so only to establish remote-support sessions.
It does not contact RustDesk's public infrastructure. See also
[KOMOTA's site](https://www.komota.lt).

## Licence and attribution

RustDesk is licensed under **AGPL-3.0**, and so is this fork. See
[`LICENCE`](LICENCE).

All credit for RustDesk itself belongs to
[the RustDesk project and its contributors](https://github.com/rustdesk/rustdesk).
This repository exists to satisfy the corresponding-source obligation for the
modified binaries we distribute, and to allow those binaries to be built
verifiably from public source.

Upstream release this is based on: **1.4.9**.
