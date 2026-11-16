Unlike IsMIP , IsQP , IsQCP , or IsMultiObj , this attribute can only be queried after an optimization has been conducted. It returns whether Gurobi found the model after presolving to be convex, with the exception of MIP components (integrality and SOS constraints).

More precisely, the return value of the attribute has the following meaning:

- 1: After presolve, the objective and all quadratic and nonlinear constraints were convex. Thus, no spatial branching is or was needed to solve this model to global optimality.
- 0: After presolve, Gurobi was unable to prove the convexity of the objective, of one or more quadratic or nonlinear constraints, or both. One of the following occurred: the presolved model contained a nonconvex element, Gurobi’s convexity-detection heuristics failed, or Gurobi did not even try to check convexity. Thus, to prove global optimality for this model, spatial branching may have been or might be required.

For examples of how to query or modify attributes, refer to our Attribute Examples .