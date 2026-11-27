# Run plan

Imported from Jira project TLDR for run `b19a1ff6-b34b-40ea-ada9-2beb27f25d01`.
Recording SHA-256: `9da48c2d1f68996983bc65800e4da47297bad2ce016ffec61b0b51568075a04b`.

The plan comes from Jira (brief §7.14). Edit it in Jira, not here.

## TLDR-1: Foundation

### TLDR-2: Crate scaffold and command-line entry

| Task | Summary | Layer | Risk | Depends on | ACs | BRs | Tests | Files |
|---|---|---|---|---|---|---|---|---|
| TLDR-3 | Crate scaffold, CI workflow, and --version | infra | low | - | AC-042 | BR-31 | @AC-042 | Cargo.toml, Cargo.lock, .gitignore, .github/workflows/ci.yml, src/main.rs, src/lib.rs, src/cli.rs, src/error.rs |
| TLDR-4 | Argument surface and no-argument help | cli | low | TLDR-3 | AC-041, AC-043 | BR-14, BR-30 | @AC-041, @AC-043 | src/cli.rs, src/platform.rs, src/lib.rs, README.md |

Checklist for TLDR-3:

- Create the binary crate tldr: edition 2021, rust-version 1.77, version 1.0.0, dependencies from 06-technical-design
- Add src/lib.rs declaring the library modules, and src/error.rs with the TldrError variants
- Keep src/main.rs thin: call cli::run(), print the error chain and exit 1 on Err
- Make -v/--version print 'tldr 1.0.0 (Client Specification 2.3)' and exit 0
- Add a CI workflow running cargo fmt --check, cargo clippy -- -D warnings, and cargo test

Checklist for TLDR-4:

- Define every flag and the positional command list from 06-technical-design with clap derive
- Add VALID_PLATFORMS and restrict --platform to it, so an invalid value exits 2
- With no arguments, print help to stderr and exit 1
- Accept --print-completion, print a not-supported notice, and exit 0 (D-07)
- Document usage in README.md

## TLDR-5: Page lookup

### TLDR-6: Fetch pages over the network

| Task | Summary | Layer | Risk | Depends on | ACs | BRs | Tests | Files |
|---|---|---|---|---|---|---|---|---|
| TLDR-7 | Settings from the environment and the page fetcher | core | medium | TLDR-3 | AC-035, AC-037 | BR-25, BR-26 | @AC-035:unit, @AC-037:unit | src/config.rs, src/fetch.rs, src/lib.rs |
| TLDR-8 | Look up a page and print it | cli | low | TLDR-7, TLDR-4 | AC-002, AC-005, AC-035, AC-036, AC-037, AC-038, AC-044 | BR-01, BR-02, BR-04, BR-25, BR-26, BR-29 | @AC-002, @AC-005, @AC-035, @AC-036, @AC-037, @AC-038, @AC-044 | src/cli.rs, src/platform.rs, src/main.rs, Cargo.toml, Cargo.lock |

Checklist for TLDR-7:

- Config::from_env reads every TLDR_* variable listed in 06-technical-design, with the defaults from 02-business-rules
- Strip trailing slashes from TLDR_PAGES_SOURCE_LOCATION
- build_page_url per BR-26: no language suffix for en, percent-encode the command with NON_ALPHANUMERIC
- PageFetcher::new sets User-Agent tldr-rust-client/<version> and a 10-second connect and read timeout
- fetch_page returns no page on 404 and an error on a network failure

Checklist for TLDR-8:

- Join the positional words with hyphens and lowercase them
- Add detect_platform and try the detected platform, then common
- Print the found page to stdout and exit 0 (styling comes with the renderer)
- When no page is found, exit 1 with a not-available message on stderr naming the command and the contribute link
- Let --source override the page source for the invocation
- On a network error or timeout, exit 1 with a fetch-error message on stderr
- Install a SIGINT handler in main.rs before run() that prints a newline and 'Exited on keyboard interrupt.' to stdout and exits 0, so an interrupted lookup ends that way
- Use the ctrlc crate for the handler, as 06-technical-design's main.rs section says: the owner approved adding it to [dependencies] under autonomy policy rule 3 (2026-11-27, decision 0098)

### TLDR-9: TLS verification settings

| Task | Summary | Layer | Risk | Depends on | ACs | BRs | Tests | Files |
|---|---|---|---|---|---|---|---|---|
| TLDR-10 | Custom CA bundle and insecure mode | core | high | TLDR-8 | AC-039, AC-040 | BR-28 | @AC-039, @AC-040 | src/config.rs, src/fetch.rs |

Checklist for TLDR-10:

- When TLDR_CERT is set, verify TLS with that CA bundle; otherwise use the system default
- TLDR_ALLOW_INSECURE=1 disables hostname and certificate checks
- Print a yellow warning before the first insecure request

### TLDR-11: Platform selection

| Task | Summary | Layer | Risk | Depends on | ACs | BRs | Tests | Files |
|---|---|---|---|---|---|---|---|---|
| TLDR-12 | Platform resolution | core | low | TLDR-4 | AC-017, AC-018, AC-019 | BR-14, BR-16 | @AC-017:unit, @AC-018:unit, @AC-019:unit | src/platform.rs |
| TLDR-13 | Multi-platform lookup and platform notes | cli | low | TLDR-12, TLDR-23 | AC-003, AC-004, AC-017, AC-018, AC-019, AC-045, AC-046 | BR-02, BR-03, BR-14, BR-15, BR-16, BR-17 | @AC-003, @AC-004, @AC-017, @AC-018, @AC-019, @AC-045, @AC-046 | src/cli.rs, src/platform.rs |

Checklist for TLDR-12:

- Resolve the platform from --platform, then a valid TLDR_PLATFORM, then auto-detection (BR-14)
- Map std::env::consts::OS to the directory names; unknown values fall back to linux
- On an unrecognised TLDR_PLATFORM, print a yellow warning and use auto-detection
- platform_list returns the current platform, the others, then common

Checklist for TLDR-13:

- Look the page up on every platform in platform_list order and show the first result
- Fall back to common before reporting the page as not available
- Warn naming the platform when the page came from a non-current platform and --platform was not given
- Print a note with the count and names of the other platforms that have the page

### TLDR-14: Language selection

| Task | Summary | Layer | Risk | Depends on | ACs | BRs | Tests | Files |
|---|---|---|---|---|---|---|---|---|
| TLDR-15 | Language resolution | core | low | TLDR-3 | AC-020, AC-021, AC-022 | BR-18, BR-19 | @AC-020:unit, @AC-021:unit, @AC-022:unit | src/language.rs, src/lib.rs |
| TLDR-16 | Language-aware lookup | cli | low | TLDR-15, TLDR-13 | AC-020, AC-021, AC-022 | BR-18, BR-19 | @AC-020, @AC-021, @AC-022 | src/cli.rs |

Checklist for TLDR-15:

- resolve_languages applies --language, TLDR_LANGUAGE, LANGUAGE, then LANG, deduplicates, and ends with en
- Treat LANG values C and POSIX as no preference
- normalise_language_code strips the encoding, maps pt to pt_PT, and keeps pt_PT, pt_BR, and zh_TW

Checklist for TLDR-16:

- Try each (platform, language) pair in resolved order and use the first page found
- Pass -L/--language to resolve_languages

## TLDR-17: Rendering

### TLDR-18: Render pages

| Task | Summary | Layer | Risk | Depends on | ACs | BRs | Tests | Files |
|---|---|---|---|---|---|---|---|---|
| TLDR-19 | Page renderer with default styling | core | low | TLDR-3 | AC-016, AC-023, AC-026, AC-031, AC-032 | BR-13, BR-20, BR-22, BR-24 | @AC-016:unit, @AC-023:unit, @AC-026:unit, @AC-031:unit, @AC-032:unit | src/render.rs, src/lib.rs |
| TLDR-20 | Styled output, --render, --markdown, and -c | cli | low | TLDR-19, TLDR-8 | AC-014, AC-015, AC-016, AC-023, AC-026, AC-031, AC-032, AC-047 | BR-12, BR-13, BR-20, BR-22, BR-24 | @AC-014, @AC-015, @AC-016, @AC-023, @AC-026, @AC-031, @AC-032, @AC-047 | src/cli.rs |

Checklist for TLDR-19:

- Add DisplayMode and a ColorScheme with the BR-20 defaults
- render_page handles the #, >, - and backtick line prefixes as the Python output() does
- Render backtick spans in example lines italic yellow
- Style {{parameter}} text with the parameter colour and the rest of the command with the command colour
- Render escaped braces as literal {{ and }}
- With plain set, write the page bytes unchanged

Checklist for TLDR-20:

- Render looked-up pages with render_page, building ColorScheme::from_env and the display mode at startup
- --render <path> renders a local file; a missing file exits 1 with a no-such-file message
- -m/--markdown prints the raw page with no ANSI codes
- -c/--color forces colour even when stdout is not a TTY

### TLDR-21: Colour and option-form settings

| Task | Summary | Layer | Risk | Depends on | ACs | BRs | Tests | Files |
|---|---|---|---|---|---|---|---|---|
| TLDR-22 | Colour settings from TLDR_COLOR_* | core | low | TLDR-20 | AC-024, AC-025 | BR-20, BR-21 | @AC-024, @AC-025 | src/render.rs |
| TLDR-23 | Option-form display modes | core | low | TLDR-22 | AC-027, AC-028, AC-029, AC-030 | BR-23 | @AC-027, @AC-028, @AC-029, @AC-030 | src/render.rs, src/cli.rs |

Checklist for TLDR-22:

- parse_color_env reads space-separated foreground, background, and attribute tokens per BR-21
- Ignore unknown tokens silently
- ColorScheme::from_env applies TLDR_COLOR_NAME, _DESCRIPTION, _EXAMPLE, _COMMAND, and _PARAMETER over the defaults

Checklist for TLDR-23:

- Substitute {{[s\|l]}} as s in short mode, l in long mode, and s/l in both mode (D-03)
- Start from TLDR_OPTIONS (default long); --short-options or --long-options override it
- Both flags together select both mode

## TLDR-24: Cache

### TLDR-25: Page cache

| Task | Summary | Layer | Risk | Depends on | ACs | BRs | Tests | Files |
|---|---|---|---|---|---|---|---|---|
| TLDR-26 | User and system cache | core | medium | TLDR-7 | AC-033, AC-034, AC-048 | BR-05, BR-06, BR-07 | @AC-033:unit, @AC-034:unit, @AC-048:unit | src/cache.rs, src/lib.rs |
| TLDR-27 | Cached lookup | cli | low | TLDR-26, TLDR-16 | AC-001, AC-033, AC-034, AC-048 | BR-02, BR-05, BR-06, BR-07 | @AC-001, @AC-033, @AC-034, @AC-048 | src/cli.rs |

Checklist for TLDR-26:

- Resolve the user cache root from XDG_CACHE_HOME, then HOME/.cache, then the home-directory API (not dirs::cache_dir on macOS)
- page_path uses pages[.<lang>]/<platform>/<command>.md, the layout the Python client shares
- load_page, store_page, and is_fresh against TLDR_CACHE_MAX_AGE hours
- system_cache finds the first tldr directory in XDG_DATA_DIRS, else /usr/share/tldr, and is never freshness-checked

Checklist for TLDR-27:

- Look in the system cache, then a fresh user-cache page, then the network, and store fetched pages
- TLDR_CACHE_ENABLED=0 skips every cache read and write
- TLDR_NETWORK_ENABLED=0 disables fetches; a cache miss exits 1 with a no-network message (D-04)

### TLDR-28: Cache update and clearing

| Task | Summary | Layer | Risk | Depends on | ACs | BRs | Tests | Files |
|---|---|---|---|---|---|---|---|---|
| TLDR-29 | Download and extract page bundles | core | high | TLDR-27 | AC-006, AC-007 | BR-05, BR-06, BR-08, BR-27 | @AC-007:unit | src/fetch.rs, src/cache.rs |
| TLDR-30 | Clear the cache | cli | medium | TLDR-29 | AC-008, AC-009 | BR-05, BR-09 | @AC-008, @AC-009 | src/cache.rs, src/cli.rs |
| TLDR-31 | --update and --update with a command | cli | low | TLDR-30 | AC-006, AC-007, AC-049 | BR-02, BR-05, BR-06, BR-08, BR-27 | @AC-006, @AC-007, @AC-049 | src/cli.rs |

Checklist for TLDR-29:

- Derive each bundle URL from TLDR_DOWNLOAD_CACHE_LOCATION (default per BR-27) by replacing .zip with -pages.<lang>.zip
- download_bundle fetches the zip for one language
- extract_bundle yields (platform, command, bytes) for entries matching (.+)/(.+)\.md and rejects paths that leave the cache
- Store every extracted page in the user cache

Checklist for TLDR-30:

- clear removes pages[.<lang>] in the user cache for every resolved language
- --clear-cache exits 0 after clearing

Checklist for TLDR-31:

- --update downloads the bundle for every resolved language
- With no command, exit 0 after updating; with a command, render the page (D-02)

### TLDR-32: List and search cached commands

| Task | Summary | Layer | Risk | Depends on | ACs | BRs | Tests | Files |
|---|---|---|---|---|---|---|---|---|
| TLDR-33 | List cached commands | cli | low | TLDR-31 | AC-010, AC-011 | BR-10, BR-14 | @AC-010, @AC-011 | src/cache.rs, src/cli.rs |
| TLDR-34 | Search cached commands | cli | low | TLDR-33 | AC-012, AC-013 | BR-11 | @AC-012, @AC-013 | src/cli.rs |

Checklist for TLDR-33:

- list_commands returns the .md stems for the given platforms and languages
- --list prints one name per line for the resolved platform and language and exits 0

Checklist for TLDR-34:

- --search matches cached names case-insensitively by substring and exits 0 on a match
- Exit 1 with a no-match message when nothing matches
- When the cache is empty, prompt to run --update and exit 0
