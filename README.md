# tldr

A command-line client for tldr pages.

## Usage

```text
tldr [OPTIONS] [COMMAND]...
```

- `tldr` with no arguments prints the help to stderr and exits with status 1.
- `-p, --platform <PLATFORM>` selects the platform. Valid values are `android`, `common`, `freebsd`, `linux`, `netbsd`, `openbsd`, `osx`, `sunos`, `windows`. Any other value is an argument error and exits with status 2.
- `-v, --version` prints `tldr <version> (Client Specification 2.3)` and exits 0.
- `--print-completion` prints a notice that shell completion is not supported and exits 0.
