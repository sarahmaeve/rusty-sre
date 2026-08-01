# Report: shutdown exceeds termination grace

**Reported:** 2026-07-28 03:14 UTC  
**Service:** telemetry-forwarder  
**Severity:** rolling deployments stall; forced termination loses buffered events

During a regional collector slowdown, old pods failed to exit within the
30-second termination grace period. The platform sent a forced kill. Healthy
regions and idle pods shut down normally.

A recent change added batching and a retry loop. Operators observed “shutdown
requested” but not “worker stopped.” CPU was low. The collector recovered several
minutes later and new pods became healthy.

Known unknowns:

- whether the worker receives the cancellation signal;
- whether a blocked send prevents the signal from being observed;
- whether the retry or backoff policy resets a deadline;
- whether a shared state lock participates.

Form competing hypotheses before opening evidence.
