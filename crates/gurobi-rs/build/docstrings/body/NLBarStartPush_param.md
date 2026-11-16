We consider this feature a preview in this release. This means that it is fully tested and supported, but will likely undergo significant changes in subsequent Gurobi technical or major releases, potentially including breaking changes in API, behavior and packaging.

A user-given starting point for the NL barrier algorithm is modified so that the distance between a variable value and its bounds is at least this amount, with the exception the value is not moved beyond the mid-point of the interval between finite lower and upper bounds.  A small value is often more suitable when a starting point is provided that is close to the optimal solution (see also NLBarInitMu ).

Only affects the NL barrier algorithm

For examples of how to query or modify parameter values from our different APIs, refer to our Parameter Examples .