# Solutions

Each exercise keeps the defective implementation in the working tree. Its solution
is a small patch and a debrief that explains the underlying contract. Diagnose the
failure and write down your proposed repair before opening either.

Check and apply one solution from the repository root:

```console
git apply --check solutions/01-silent-config.patch
git apply solutions/01-silent-config.patch
make ex N=01
```

Reverse it when you want to repeat the exercise:

```console
git apply -R solutions/01-silent-config.patch
```

Do not reverse a patch over other edits to the same lines. Use
`git apply --check -R` first. Exercises 27–32 are intentionally non-compiling
programs excluded from the main workspace; their patches make the selected
program compile.

| Exercise | Patch | Debrief |
| --- | --- | --- |
| 01 | [silent config](01-silent-config.patch) | [debrief](debriefs/01-silent-config.md) |
| 02 | [overwritten count](02-overwritten-count.patch) | [debrief](debriefs/02-overwritten-count.md) |
| 03 | [UTF-8 prefix](03-utf8-prefix.patch) | [debrief](debriefs/03-utf8-prefix.md) |
| 04 | [checked narrowing](04-checked-narrowing.patch) | [debrief](debriefs/04-checked-narrowing.md) |
| 05 | [safe severity default](05-safe-severity-default.patch) | [debrief](debriefs/05-safe-severity-default.md) |
| 06 | [lost mutation](06-lost-mutation.patch) | [debrief](debriefs/06-lost-mutation.md) |
| 07 | [hash contract](07-hash-contract.patch) | [debrief](debriefs/07-hash-contract.md) |
| 08 | [explicit false](08-explicit-false.patch) | [debrief](debriefs/08-explicit-false.md) |
| 09 | [all probes](09-all-probes.patch) | [debrief](debriefs/09-all-probes.md) |
| 10 | [status range](10-status-range.patch) | [debrief](debriefs/10-status-range.md) |
| 11 | [retain active](11-retain-active.patch) | [debrief](debriefs/11-retain-active.md) |
| 12 | [retry count](12-retry-count.patch) | [debrief](debriefs/12-retry-count.md) |
| 13 | [error source](13-error-source.patch) | [debrief](debriefs/13-error-source.md) |
| 14 | [path extension](14-path-extension.patch) | [debrief](debriefs/14-path-extension.md) |
| 15 | [RefCell reborrow](15-refcell-reborrow.patch) | [debrief](debriefs/15-refcell-reborrow.md) |
| 16 | [break Rc cycle](16-break-rc-cycle.patch) | [debrief](debriefs/16-break-rc-cycle.md) |
| 17 | [poison recovery](17-poison-recovery.patch) | [debrief](debriefs/17-poison-recovery.md) |
| 18 | [release lease](18-release-lease.patch) | [debrief](debriefs/18-release-lease.md) |
| 19 | [nonblocking sleep](19-nonblocking-sleep.patch) | [debrief](debriefs/19-nonblocking-sleep.md) |
| 20 | [lock scope](20-lock-scope.patch) | [debrief](debriefs/20-lock-scope.md) |
| 21 | [join export](21-join-export.patch) | [debrief](debriefs/21-join-export.md) |
| 22 | [bounded fanout](22-bounded-fanout.patch) | [debrief](debriefs/22-bounded-fanout.md) |
| 23 | [scrape timeout](23-scrape-timeout.patch) | [debrief](debriefs/23-scrape-timeout.md) |
| 24 | [overall retry budget](24-overall-retry-budget.patch) | [debrief](debriefs/24-overall-retry-budget.md) |
| 25 | [worker cancellation](25-worker-cancellation.patch) | [debrief](debriefs/25-worker-cancellation.md) |
| 26 | [drain channel](26-drain-channel.patch) | [debrief](debriefs/26-drain-channel.md) |
| 27 | [borrow input](27-borrow-input.patch) | [debrief](debriefs/27-borrow-input.md) |
| 28 | [shorten borrow](28-shorten-borrow.patch) | [debrief](debriefs/28-shorten-borrow.md) |
| 29 | [return owned](29-return-owned.patch) | [debrief](debriefs/29-return-owned.md) |
| 30 | [drop guard before await](30-drop-guard-before-await.patch) | [debrief](debriefs/30-drop-guard-before-await.md) |
| 31 | [derive Hash and Eq](31-derive-hash-eq.patch) | [debrief](debriefs/31-derive-hash-eq.md) |
| 32 | [exhaustive match](32-exhaustive-match.patch) | [debrief](debriefs/32-exhaustive-match.md) |

The patch is one sound repair, not proof that every alternative is wrong. Compare
alternatives by contract, allocation, failure behavior, and API clarity.
