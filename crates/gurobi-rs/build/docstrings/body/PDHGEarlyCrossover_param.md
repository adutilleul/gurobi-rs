By default, PDHG performs concurrent crossover by forking off multiple sequential crossover threads, starting when the current PDHG iterate first satisfies relaxed feasibility and convergence tolerances.

The first crossover thread is started when the current iterate satisfies the configured feasibility and convergence tolerances relaxed by a factor of \(10^{X}\) , where \(X\) is the value of PDHGEarlyCrossover .

For the default PDHG feasibility and convergence tolerances ( \(10^{-6}\) ) and the default value of PDHGEarlyCrossover ( 2.0 ), the first crossover thread is therefore started when the PDHG iterate first satisfies these tolerances with accuracy \(10^{-4}\) , i.e., the default \(10^{-6}\) relaxed by a factor of \(10^2\) .

Setting PDHGEarlyCrossover to 0.0 turns off PDHG concurrent crossover and instead starts crossover only after PDHG is finished.

When PDHG concurrent crossover is enabled, the PDHG algorithm may continue running past the point where its feasibility and convergence tolerances are first met. This is done in order to fork off more crossover threads for parallel crossover, potentially speeding up the crossover step.

PDHG only.

Refer to SolutionTarget for interactions between that parameter, Crossover , and PDHGEarlyCrossover .