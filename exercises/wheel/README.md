# Report-first investigations

Wheel scenarios imitate an incident handoff. The initial report is incomplete,
several code paths look suspicious, and evidence has different reliability. Your
job is to narrow the incident before changing code.

## Order of work

1. Read only the scenario's `REPORT.md`.
2. Fill in `Impact and contract` and `Initial hypotheses`, then create an empty
   evidence ledger.
3. Open evidence packets one at a time, recording how each changes your ranking.
4. Read `CANDIDATE.md` last. It contains selected source excerpts and change
   context, including plausible red herrings.
5. Propose the smallest causal repair and a regression that would have caught it.

Do not fix every smell you encounter during an incident. Record unrelated findings
separately. A good handoff states impact, mechanism, evidence, uncertainty, repair
scope, and verification.

These scenarios draw on operational patterns visible in projects such as
[TiKV](https://github.com/tikv/tikv), [Vector](https://github.com/vectordotdev/vector),
[Linkerd2-proxy](https://github.com/linkerd/linkerd2-proxy), and
[OpenTelemetry Rust](https://github.com/open-telemetry/opentelemetry-rust).
