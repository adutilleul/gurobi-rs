Controls the aggressiveness of the NoRel heuristic during tree search. This heuristic searches for high-quality feasible solutions.

In contrast to the root node NoRel heuristic controlled by parameters NoRelHeurSolutions , NoRelHeurTime , and NoRelHeurWork , the tree search version of the NoRel heuristic is applied repeatedly during the branch-and-bound tree search.

This parameter controls how much effort is spent on NoRel during tree search relative to the effort spent on the tree search itself.

In its automatic setting (-1), the NoRel tree heuristic is enabled if the MIPFocus parameter is set to MIPFocus=1 . Otherwise, it is disabled.

Applying NoRel during the tree search can be useful for models in which solving LP relaxations and processing search tree nodes is not too hard, but still finding good-quality feasible solutions turns out to be difficult. Moreover, it can be useful for situations in which a tight time limit is in play and the goal is to find a solution within this time frame that is as good as possible.

Since NoRel targets the primal bound rather than the dual bound, spending more effort in NoRel means that the final global dual bound will usually be weaker than with default settings.

Only affects mixed integer programming (MIP) models

For examples of how to query or modify parameter values from our different APIs, refer to our Parameter Examples .