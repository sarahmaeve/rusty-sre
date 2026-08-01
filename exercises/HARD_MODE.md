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
| 31 | A set construction fails because its element contract is incomplete. |
| 32 | Adding a variant breaks a match in downstream code. |
| 33 | A helper can return either input, but its lifetime contract covers only one. |
| 34 | A generic thread helper cannot prove that captured state remains valid. |
| 35 | One generic method prevents dynamic dispatch through an otherwise usable trait. |
| 36 | A trait object erases an associated type that its caller still needs. |
| 37 | A plain binding ignores one possible event variant. |
| 38 | A macro rejects a qualified enum variant before expansion. |
| 39 | Reading a pinned worker fails because the code tries to extract it. |
| 40 | An associated trait function has no receiver from which to infer its implementor. |
| 41 | Input is rejected, but for an accidental reason that a broad test accepts. |
| 42 | A failed test leaves process-wide fixture state changed. |
| 43 | One sampling operation invokes its input callback twice. |
| 44 | A validated wrapper can still be constructed with its forbidden value. |
| 45 | Enabling an audit feature disables baseline validation. |
| 46 | Boundary arithmetic behaves differently across build profiles. |
| 47 | A future becomes ready internally but is never scheduled again. |
| 48 | Runtime deployment input loses to a value selected while compiling. |

For each report, record:

- the contract you infer;
- two competing explanations;
- the observation that would distinguish them;
- the smallest useful command to run next.

Only then open the exercise README. Open hints one stage at a time.
