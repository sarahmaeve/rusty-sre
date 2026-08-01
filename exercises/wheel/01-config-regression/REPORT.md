# Report: worker-limit rollout regression

**Reported:** 2026-07-21 15:42 UTC  
**Service:** archive-indexer  
**Severity:** elevated latency; no known data loss

Ten minutes after rollout `2026.07.21.3`, two production instances began consuming
far more memory than expected. The deployment manifest set `WORKER_LIMIT=eight`;
the startup log reported `worker_limit=32 source=environment`. Staging used
`WORKER_LIMIT=8` and did not reproduce.

The new release added configuration snapshot export and changed startup logging.
Rollback restored expected resource use. The service never failed readiness.

Questions from incident command:

- Why did malformed configuration pass startup?
- Can the startup log's source field be trusted?
- Did snapshot serialization participate in the runtime setting?

Begin with hypotheses. Do not open `CANDIDATE.md` yet.
