# Hard mode

Directory names and normal READMEs identify the concept under study. Hard mode
removes that advantage. Pick an exercise by number, run it, and read only the
corresponding report below until you have a falsifiable hypothesis.

| Exercise | Initial report |
| --- | --- |
| 01 | A malformed deployment value is accepted and the service starts with a plausible value. |
| 02 | Repeated failures from one service are reported as a single failure. |
| 03 | A preview endpoint panics only for some customer names. |
| 04 | A very large metric wraps to a small value downstream. |
| 05 | An incident with no severity is treated as harmless. |
| 06 | A state update returns but the caller still holds the old value. |
| 07 | A set contains two records that comparison says are equal. |
| 08 | Serialized policy output omits an explicitly disabled flag. |
| 09 | A pool is reported healthy while one required member is unavailable. |
| 10 | The last status in the HTTP success class is reported as failure. |
| 11 | Cleanup preserves decommissioned hosts and removes active ones. |
| 12 | A request gets fewer attempts than its retry budget permits. |
| 13 | The top-level error renders correctly, but its causal chain ends early. |
| 14 | An uppercase configuration extension is classified incorrectly. |
| 15 | A single-threaded request panics despite using runtime-checked borrowing. |
| 16 | Dropping the last apparent owner does not release a graph. |
| 17 | After one worker panics, unrelated work also begins to panic. |
| 18 | The active-resource gauge never falls after guards leave scope. |
| 19 | Concurrent requests complete almost serially. |
| 20 | A task waiting for unrelated work prevents peers from making progress. |
| 21 | Background work fails, but the caller reports success. |
| 22 | A large input causes a sharp memory and scheduler spike. |
| 23 | A dependency that never responds prevents the operation from finishing. |
| 24 | Repeated slow failures exceed the operation's total time budget. |
| 25 | Cancelling an idle worker does not make it exit. |
| 26 | Batch ingestion returns only its first envelope. |
| 27 | The compiler reports use of a value after ownership moved. |
| 28 | The compiler rejects overlapping access to the same value. |
| 29 | The compiler rejects a returned reference whose referent is gone. |
| 30 | Task spawning rejects a future that cannot cross threads safely. |
| 31 | A generic map helper fails because its key contract is incomplete. |
| 32 | Adding a variant breaks a match in downstream code. |

For each report, record:

- the contract you infer;
- two competing explanations;
- the observation that would distinguish them;
- the smallest useful command to run next.

Only then open the exercise README. Open hints one stage at a time.
