Limits the amount of work performed by the PDHG MIP heuristic. This heuristic runs a branch-and-bound search in which node relaxations are solved using PDHG, and searches for high-quality feasible solutions.

The work metric used in this parameter is difficult to define precisely. A single unit corresponds to roughly a second, but this will depend on the machine, the core count, and in some cases the model. You may need to experiment to find a good setting for your model. Refer to WorkLimit for further details on how work is measured.

Only affects mixed integer programming (MIP) models.

For examples of how to query or modify parameter values from our different APIs, refer to our Parameter Examples .