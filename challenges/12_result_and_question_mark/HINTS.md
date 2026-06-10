# Challenge 12: Hints

Work the exercises before opening this file. Each hint is staged — read only
as far as you need, then go back to the code.

## Skeleton: Config Loader

**Task 1 — ConfigError.** Four variants exactly as specced. One trap: the
tests compare results with `assert_eq!`, so the enum needs
`#[derive(Debug, PartialEq)]` even though the task only mentions `Debug`.
The `Error` impl is one empty line: `impl Error for ConfigError {}`.

**Task 2 — From<ParseIntError>.** A two-line impl whose `from` wraps the
parse error's text: `ConfigError::InvalidPort(e.to_string())`.

**Task 3 — parse_port.** `map_err` converts the `ParseIntError` into
`InvalidPort` with the *original input* embedded; then reject zero
explicitly.

**Task 4 — parse_threshold.** Same shape; when the parse itself fails
there's no number to report, so `InvalidThreshold(f64::NAN)` works. Then
check `(0.0..=1.0).contains(&value)`.

**Task 5 — parse_url.** `str::split_once("://")` returns
`Option<(&str, &str)>` — match on it and check the scheme is `http`/`https`
and the host is non-empty.

**Task 6 — load_config.** `Option::ok_or(ConfigError::MissingField("port"))?`
turns a missing field into an early return; chain each parser with `?` so
the first failure short-circuits.

## Debug: Alert Ingestion

Four bugs. Two of them are entangled — the file won't compile until both
are fixed. Each hint reveals more: symptom → where to look → the fix.

### Bug 1

- **Symptom:** Doesn't compile — `parse_alert` returns `Err(...)`/`Ok(...)`
  and uses `?`, but its signature says it returns a plain `Alert` (E0308 /
  E0277).
- **Where:** the `parse_alert` signature.
- **Fix:** The body is already written in Result style; only the signature
  lies. Change it to `-> Result<Alert, IngestError>`.

### Bug 2

- **Symptom:** Even with Bug 1 fixed — "`?` couldn't convert the error to
  `IngestError`" (E0277) on the `parse()` lines.
- **Where:** just below `impl Error for IngestError`.
- **Fix:** `?` calls `From::from` to promote the error type, and there is
  no `From<ParseIntError> for IngestError`. Add it:
  `impl From<ParseIntError> for IngestError { fn from(e: ParseIntError) -> Self { IngestError::Parse(e) } }`.

### Bug 3

- **Symptom:** The tests call `parse_all(...).unwrap()` and expect
  `Err(...)` for bad input — but `parse_all` returns a plain `Vec` and
  panics on the first bad line.
- **Where:** `parse_all`.
- **Fix:** Change the signature to `Result<Vec<Alert>, IngestError>` and
  let `collect` short-circuit on the first error:
  `lines.iter().map(|line| parse_alert(line)).collect()` — collecting an
  iterator of `Result`s into `Result<Vec<_>, _>` stops at the first `Err`.

### Bug 4

- **Symptom:** `process_returns_err_on_bad_input` fails — `process` returns
  `Ok` with empty data for malformed input.
- **Where:** `process`.
- **Fix:** `.ok().unwrap_or_default()` is Python's `except: pass` — it
  converts the error into a fake-empty success. Propagate instead:
  `let alerts = parse_all(lines)?;`.
