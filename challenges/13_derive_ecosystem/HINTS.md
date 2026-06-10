# Challenge 13: Hints

Work the exercises before opening this file. Each hint is staged — read only
as far as you need, then go back to the code.

## Skeleton: Incident Report Pipeline

**Task 1 — Display/FromStr for Severity.** Two mirrored `match`es: one
writing lowercase strings, one parsing them back, with a descriptive `Err`
for anything else.

**Task 2 — Display for IncidentReport.** One `write!` with all five fields.
For logfmt safety, swap the summary's spaces:
`self.summary.replace(' ', "_")`.

**Task 3 — FromStr for IncidentReport.** Split on whitespace, `split_once('=')`
each token into a `HashMap<&str, &str>`, then pull each field out with
`fields.get("...").ok_or("missing field: ...")?`. Reverse the summary
substitution with `.replace('_', " ")`.

**Task 4 — ReportError.** Three variants exactly as specced, a three-arm
`Display`, and `impl std::error::Error for ReportError {}` (empty body).

**Task 5 — From<String>.** Wrap the message: `ReportError::ParseError(msg)`.
That's all `?` needs to promote the `String` errors from your `FromStr`
impls.

**Task 6 — parse_report_args.** Walk the args two at a time (flag, value)
into a map, then extract the five required flags — `ok_or_else` for
`MissingField`, `map_err` into `InvalidArg { flag, reason }` for the
severity and timestamp parses.

## Debug: SRE Log Exporter

Four bugs. Each hint reveals more: symptom → where to look → the fix.

### Bug 1

- **Symptom:** Doesn't compile — "`?` couldn't convert the error to
  `ExportError`" (E0277) on the timestamp parse in
  `DeployEvent::from_str`.
- **Where:** the error-conversion impls near `From<std::io::Error>`.
- **Fix:** The `?` on a `u64` parse produces a `ParseIntError`, and nothing
  converts that into `ExportError`. Add
  `impl From<ParseIntError> for ExportError`, wrapping the error text in
  `ExportError::Parse`.

### Bug 2

- **Symptom:** `test_event_batch_error_type` doesn't compile — it
  annotates the parse result as `Result<EventBatch, ExportError>`, but
  `EventBatch`'s `FromStr` says its error is `String`.
- **Where:** `impl FromStr for EventBatch` (and one caller).
- **Fix:** Declare `type Err = ExportError;`. The body's
  `.map_err(|e: ExportError| e.to_string())` was only there to satisfy the
  wrong type — delete it and let `?` propagate the `ExportError` directly.
  Then `read_events_from_file`'s `.map_err(|e: String| ...)` no longer
  matches either; `content.parse()?` is all it needs.

### Bug 3

- **Symptom:** `test_error_chain_io` fails — "source() should return Some
  for Io variant".
- **Where:** `impl std::error::Error for ExportError`, the `source` method.
- **Fix:** The `Io` arm binds the inner error and then returns `None`,
  severing the error chain. Return it: `ExportError::Io(err) => Some(err)`.

### Bug 4

- **Symptom:** `test_deploy_event_roundtrip` fails — after a
  serialize/parse round trip, `version` comes back equal to the service
  name.
- **Where:** `impl fmt::Display for DeployEvent`.
- **Fix:** A copy-paste slip: the `version=` slot is fed `self.service`
  instead of `self.version`. Round-trip tests exist precisely to catch
  this class of bug.
