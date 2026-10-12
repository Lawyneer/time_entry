# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Rack rate support: `rack_rate` setting with per-entry Standard/Rack dropdown, replacing `multi_bill_rate`
- Calendar date picker with date validation
- Multiple tabs (max 5) for concurrent matters, individually closable with confirmation when time has been entered
- Per-tab start/stop timers; switching tabs pauses the active timer and resumes it when the tab is revisited
- Live hours-field population from running timer
- Time rounding to nearest 0.1 or 0.25 hour increments

## [1.0.0] - 2026-10-12

### Added
- Initial Rust release, a feature-equivalent port of the Python PySimpleGUI time-entry application (v1.2 Python, last updated 2021-06-17).
- Single self-contained Windows executable; no Python runtime required.
- Time-entry form: Date (defaults to today), Matter No., Hours, Phase, Description, and conditional Rate field for multi-rate users.
- Email submission via Outlook COM automation in the exact recipient format expected by the billing backend, with the matter number as subject.
- Configuration via `config.ini` ([user] section: tk_number, billing_rate, email, multi_bill_rate).

### Changed
- Failed email submissions now display an error dialog instead of failing silently.
- Missing or malformed configuration files produce a specific error message identifying the problem.

### Removed
- `bill_strings.json` standard billing narratives (used only for testing
  in the Python version).