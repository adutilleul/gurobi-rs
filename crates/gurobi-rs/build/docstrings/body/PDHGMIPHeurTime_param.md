Limits the amount of time (in seconds) spent in the PDHG MIP heuristic. This heuristic runs a branch-and-bound search in which node relaxations are solved using PDHG, and searches for high-quality feasible solutions.

Note that this parameter will introduce non-determinism — different runs may take different paths. Use the PDHGMIPHeurWork parameter for deterministic results.

Only affects mixed integer programming (MIP) models.

For examples of how to query or modify parameter values from our different APIs, refer to our Parameter Examples .