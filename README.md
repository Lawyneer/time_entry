# Time Entry

[![Build](https://github.com/Lawyneer/time_entry/actions/workflows/ci.yml/badge.svg)](https://github.com/Lawyneer/time_entry/actions/workflows/ci.yml)
[![Version](https://img.shields.io/github/v/tag/Lawyneer/time_entry?label=version&sort=semver)](https://github.com/Lawyneer/time_entry/releases)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

A native Windows time-entry application for professionals such as lawyer and accountants
who bill by the hour. Built in Rust as a modern replacement for the original Python
version, producing a single self-contained executable with no runtime dependencies to install.

Entries are formatted into a billing email and sent through the user's locally installed
Outlook client, exactly matching the email format expected by the billing system on the
receiving end.

## Status

**Version 1.0.0** — a faithful Rust port of the original Python application.
See [CHANGELOG.md](CHANGELOG.md) for release history and the [Roadmap](#roadmap) 
for planned features.

## Features

- Simple time-entry form: Date, Matter Number, Hours, Phase, Description
- Default date auto-populated with the current date
- Standard or multi-rate billing rate configuration
- Emails sent via Outlook COM automation — no password stored, no SMTP setup
- Reads user settings from an editable `config.ini`
- Clear error reporting for missing configuration (no silent failures)

## Requirements

- Windows 10 or later
- Microsoft Outlook (classic desktop client), installed and configured

## Installation

Download the latest `time_entry.exe` from
[Releases](../../releases), place it in a folder, and create a `config.ini`
file next to it (see below).

No installer and no other dependencies are required.

## Configuration

The app reads its settings from a `config.ini` file located in the same
directory as the executable. Edit the file with any text editor.

```ini
[user]
tk_number = 1234
billing_rate = 100
email = recipient@example.com
multi_bill_rate = no
```

| Setting           | Description                                                             |
|-------------------|-------------------------------------------------------------------------|
| `tk_number`       | Your timekeeper number                                                  |
| `billing_rate`    | Your standard billing rate                                              |
| `email`           | Recipient address for billing submissions                               |
| `multi_bill_rate` | `yes` to show an editable Rate field; `no` to use `billing_rate` for every entry |

## Email format

Each submitted entry is emailed with the matter number as the subject and a body in
this exact format:

```
INCUR DATE=MM/DD/YYYY
HOURS=x.x
Rate=xxx
Timekeeper=xxxx
Phase=xxx
DESCRIPTION=...
```

**Note:** Outlook may display a one-time security prompt ("a program is trying to send
email on your behalf") the first time an entry is submitted. This is standard Outlook
behavior and can be acknowledged.

## Building from source

Requires the [Rust toolchain](https://rustup.rs/):

```
cargo build --release
```

The executable is produced at `target/release/time_entry.exe`.

> If your machine enforces Application Control (WDAC/AppLocker/Smart App Control),
> unsigned build scripts may be blocked. On Windows 11 personal machines this is
> commonly Smart App Control; see Windows Security → App & browser control.

## Roadmap

Planned features for future releases — see [CHANGELOG.md](CHANGELOG.md) for details:

- **Rack rate support** — a second, higher `rack_rate` in the config, selected per
  entry via a Standard/Rack dropdown (replacing the `multi_bill_rate` concept)
- **Calendar date picker** with full date validation
- **Multiple tabs** (up to 5) so users can quickly record time for a second matter
  when interrupted, each tab closable individually
- **Per-tab timers** — start/stop timer per tab, counting up; switching tabs pauses
  the active tab's timer and resumes it when returning to that tab
- **Automatic hours population** — timer elapsed time fills the Hours field live
- **Increment rounding** — time rounds to the nearest 0.1 or 0.25 hour increment

## License

[MIT](LICENSE)