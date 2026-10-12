
## CONTRIBUTING.md

```markdown
# Contributing

Thanks for your interest in improving Time Entry!

## Reporting issues

Open a [GitHub Issue](../../issues) with:

- What you did (steps to reproduce)
- What you expected
- What actually happened, including any error popup text
- Your Windows and Outlook versions

Since submissions go through Outlook COM automation, Outlook-specific environment details (classic vs. new Outlook, elevation level) are often the key to diagnosis.

## Development setup

1. Install the [Rust toolchain](https://rustup.rs/)
2. Clone the repository
3. `cargo check` to verify everything compiles

For testing email submission you need Windows with classic desktop Outlook installed.
`cargo check` works on any platform, but the app itself is Windows-only.

## Guidelines

- Keep the receipient email body format byte-for-byte compatible — the backend parses it.
- Match the style of the existing code; comment anything COM-related generously.
- Test on Windows with real Outlook before submitting email-related changes.
- Update CHANGELOG.md under "Unreleased" for any user-facing change.

## Submitting changes

1. Fork and create a feature branch
2. Make your changes
3. `cargo check` must pass cleanly (CI will verify)
4. Open a pull request describing what changed and why
