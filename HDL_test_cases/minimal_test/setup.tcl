# TCL setup script for simulation
set sim_time "1000ns"
set clk_period 10

proc run_sim {duration} {
    puts "Running simulation for $duration"
    run $duration
}

# Define clock generator
proc gen_clk {period} {
    while {1} {
        force clk 0
        run [expr {$period / 2}]
        force clk 1
        run [expr {$period / 2}]
    }
}

run_sim $sim_time
