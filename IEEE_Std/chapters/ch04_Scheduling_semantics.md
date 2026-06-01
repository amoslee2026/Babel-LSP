# Chapter 4: Scheduling semantics

## 4. Scheduling semantics
### 4.1 General
This clause describes the following:
— Event-based simulation scheduling semantics
— SystemVerilog’s stratified event scheduling algorithm
— Determinism and nondeterminism of event ordering
— Possible sources of race conditions
— PLI callback control points
### 4.2 Execution of a hardware model and its verification environment
The balance of the clauses of this standard describe the behavior of each of the elements of the language.
This clause gives an overview of the interactions between these elements, especially with respect to the
scheduling and execution of events.
The elements that make up the SystemVerilog language can be used to describe the behavior, at varying
levels of abstraction, of electronic hardware. SystemVerilog is a parallel programming language. The
execution of certain language constructs is defined by parallel execution of blocks or processes. It is
important to understand what execution order is guaranteed to the user and what execution order is
indeterminate.
Although SystemVerilog is used for more than simulation, the semantics of the language is defined for
simulation, and everything else is abstracted from this base definition.
### 4.3 Event simulation
The SystemVerilog language is defined in terms of a discrete event execution model. The discrete event
simulation is described in more detail in this clause to provide a context to describe the meaning and valid
interpretation of SystemVerilog constructs. These resulting definitions provide the standard SystemVerilog
reference algorithm for simulation, which all compliant simulators shall implement. Within the following
event execution model definitions, there is a great deal of choice, and differences in some details of
execution are to be expected between different simulators. In addition, SystemVerilog simulators are free to
use different algorithms from those described in this clause, provided the user-visible effect is consistent
with the reference algorithm.
A SystemVerilog description consists of connected threads of execution or processes. Processes are objects
that can be evaluated, that can have state, and that can respond to changes on their inputs to produce outputs.
Processes are concurrently scheduled elements, such as initial procedures. Examples of processes
include, but are not limited to, primitives; initial, always, always_comb, always_latch, and
always_ff procedures; continuous assignments; asynchronous tasks; and procedural assignment
statements.
Every change in state of a net or variable in the system description being simulated is considered an update
event.
Processes are sensitive to update events. When an update event is executed, all the processes that are
sensitive to that event are considered for evaluation in an arbitrary order. The evaluation of a process is also
an event, known as an evaluation event.

<!-- Page 64 -->

IEEE Std Evaluation events also include PLI callbacks, which are points in the execution model where PLI application
routines can be called from the simulation kernel.
In addition to events, another key aspect of a simulator is time. The term simulation time is used to refer to
the time value maintained by the simulator to model the actual time it would take for the system description
being simulated. The term time is used interchangeably with simulation time in this clause.
To fully support clear and predictable interactions, a single time slot is divided into multiple regions where
events can be scheduled that provide for an ordering of particular types of execution. This allows properties
and checkers to sample data when the design under test is in a stable state. Property expressions can be
safely evaluated, and testbenches can react to both properties and checkers with zero delay, all in a
predictable manner. This same mechanism also allows for nonzero delays in the design, clock propagation,
and/or stimulus and response code to be mixed freely and consistently with cycle-accurate descriptions.
### 4.4 Stratified event scheduler
A compliant SystemVerilog simulator shall maintain some form of data structure that allows events to be
dynamically scheduled, executed, and removed as the simulator advances through time. The data structure is
normally implemented as a time-ordered set of linked lists, which are divided and subdivided in a well-
defined manner.
The first division is by time. Every event has one and only one simulation execution time, which at any
given point during simulation can be the current time or some future time. All scheduled events at a specific
time define a time slot. Simulation proceeds by executing and removing all events in the current simulation
time slot before moving on to the next nonempty time slot, in time order. This procedure guarantees that the
simulator never goes backwards in time.
A time slot is divided into a set of ordered regions:
a) Preponed
b) Pre-Active
c) Active
d) Inactive
e) Pre-NBA
f) NBA
g) Post-NBA
h) Pre-Observed
i) Observed
j) Post-Observed
k) Reactive
l) Re-Inactive
m) Pre-Re-NBA
n) Re-NBA
o) Post-Re-NBA
p) Pre-Postponed
q) Postponed
The purpose of dividing a time slot into these ordered regions is to provide predictable interactions between
the design and testbench code.

<!-- Page 65 -->

IEEE Std NOTE—These regions essentially encompass the IEEE Std 1364-2005 reference model for simulation, with exactly the
same level of determinism. In other words, legacy Verilog code should continue to run correctly without modification
within the SystemVerilog mechanism.
#### 4.4.1 Active region sets and reactive region sets
There are two important groupings of event regions that are used to help define the scheduling of
SystemVerilog activity, the active region set and the reactive region set. Events scheduled in the Active,
Inactive, Pre-NBA,NBA, and Post-NBA regions are active region set events. Events scheduled in the
Reactive, Re-Inactive, Pre- Re-NBA, Re-NBA, and Post-Re-NBA regions are reactive region set events.
The Active, Inactive, Pre-NBA,NBA, Post-NBA, Pre-Observed,Observed, Post-Observed, Reactive, Re-
Inactive, Pre-Re-NBA, Re-NBA, Post-Re-NBA, and Pre-Postponed regions are known as the iterative
regions.
In addition to the active region set and reactive region set, all of the event regions of each time slot can be
categorized as simulation regions (see 4.4.2) or PLI regions (see 4.4.3).
#### 4.4.2 Simulation regions
The simulation regions of a time slot are the Preponed, Active, Inactive, NBA, Observed, Reactive, Re-
Inactive, Re-NBA and Postponed regions. The flow of execution of the event regions is specified in
Figure4-1.
4.4.2.1 Preponed events region
The #1step sampling delay provides the ability to sample data immediately before entering the current time
slot. #1step sampling is identical to taking the data samples in the Preponed region of the current time slot.
Sampling in the Preponed region is equivalent to sampling in the previous Postponed region.
Preponed region PLI events are also scheduled in this region (see 4.4.3.1).
4.4.2.2 Active events region
The Active region holds the current active region set events being evaluated and can be processed in any
order.
4.4.2.3 Inactive events region
The Inactive region holds the events to be evaluated after all the Active events are processed.
If events are being executed in the active region set, an explicit #0 delay control requires the process to be
suspended and an event to be scheduled into the Inactive region of the current time slot so that the process
can be resumed in the next Inactive to Active iteration.
4.4.2.4 NBA events region
The NBA (nonblocking assignment update) region holds the events to be evaluated after all the Inactive
events are processed.
If events are being executed in the active region set, a nonblocking assignment creates an event in the NBA
region scheduled for the current or a later simulation time.

<!-- Page 66 -->

IEEE Std 4.4.2.5 Observed events region
The Observed region is for evaluation of property expressions when they are triggered. During property
evaluation, pass/fail code shall be scheduled in the Reactive region of the current time slot. PLI callbacks are
not allowed in the Observed region.
4.4.2.6 Reactive events region
The Reactive region holds the current reactive region set events being evaluated and can be processed in any
order.
The code specified by blocking assignments in checkers, program blocks and the code in action blocks of
concurrent assertions are scheduled in the Reactive region. The Reactive region is the reactive region set
dual of the Active region (see 4.4.2.2).
4.4.2.7 Re-Inactive events region
The Re-Inactive region holds the events to be evaluated after all the Reactive events are processed.
If events are being executed in the reactive region set, an explicit #0 delay control requires the process to be
suspended and an event to be scheduled into the Re-Inactive region of the current time slot so that the
process can be resumed in the next Re-Inactive to Reactive iteration. The Re-Inactive region is the reactive
region set dual of the Inactive region (see 4.4.2.3).
4.4.2.8 Re-NBA events region
The Re-NBA region holds the events to be evaluated after all the Re-Inactive events are processed.
If events are being executed in the reactive region set, a nonblocking assignment creates an event in the Re-
NBA update region scheduled for the current or a later simulation time. The Re-NBA region is the reactive
region set dual of the NBA region (see 4.4.2.4).
4.4.2.9 Postponed events region
$monitor, $strobe, and other similar events are scheduled in the Postponed region.
No new value changes are allowed to happen in the current time slot once the Postponed region is reached.
Within this region, it is illegal to write values to any net or variable or to schedule an event in any previous
region within the current time slot.
Postponed region PLI events are also scheduled in this region (see 4.4.3.10).
#### 4.4.3 PLI regions
In addition to the simulation regions, where PLI callbacks can be scheduled, there are additional PLI-
specific regions. The PLI regions of a time slot are the Preponed, Pre-Active, Pre-NBA, Post-NBA,
Pre-Observed, Post-Observed, Pre-Re-NBA, Post-Re-NBA and Pre-Postponed regions. The flow of
execution of the PLI regions is specified in Figure4-1.
4.4.3.1 Preponed PLI region
The Preponed region provides for a PLI callback control point that allows PLI application routines to access
data at the current time slot before any net or variable has changed state. Within this region, it is illegal to
write values to any net or variable or to schedule an event in any other region within the current time slot.

<!-- Page 67 -->

IEEE Std NOTE—The PLI currently does not schedule callbacks in the Preponed region.
4.4.3.2 Pre-Active PLI region
The Pre-Active region provides for a PLI callback control point that allows PLI application routines to read
and write values and create events before events in the Active region are evaluated (see 4.5).
4.4.3.3 Pre-NBA PLI region
The Pre-NBA region provides for a PLI callback control point that allows PLI application routines to read
and write values and create events before the events in the NBA region are evaluated (see 4.5).
4.4.3.4 Post-NBA PLI region
The Post-NBA region provides for a PLI callback control point that allows PLI application routines to read
and write values and create events after the events in the NBA region are evaluated (see 4.5).
4.4.3.5 Pre-Observed PLI region
The Pre-Observed region provides for a PLI callback control point that allows PLI application routines to
read values after the active region set has stabilized. Within this region, it is illegal to write values to any net
or variable or to schedule an event within the current time slot.
4.4.3.6 Post-Observed PLI region
The Post-Observed region provides for a PLI callback control point that allows PLI application routines to
read values after properties are evaluated (in the Observed or an earlier region).
NOTE—The PLI currently does not schedule callbacks in the Post-Observed region.
4.4.3.7 Pre-Re-NBA PLI region
The Pre-Re-NBA region provides for a PLI callback control point that allows PLI application routines to
read and write values and create events before the events in the Re-NBA region are evaluated (see 4.5).
4.4.3.8 Post-Re-NBA PLI region
The Post-Re-NBA region provides for a PLI callback control point that allows PLI application routines to
read and write values and create events after the events in the Re- NBA region are evaluated (see 4.5).
4.4.3.9 Pre-Postponed PLI region
The Pre-Postponed region provides a PLI callback control point that allows PLI application routines to read
and write values and create events after processing all other regions except the Postponed region.
4.4.3.10 Postponed PLI region
The Postponed region provides a PLI callback control point that allows PLI application routines to create
read-only events after processing all other regions. PLI cbReadOnlySynch and other similar events are
scheduled in the Postponed region.
The SystemVerilog flow of time slots and event regions is shown in Figure4-1.

<!-- Page 68 -->

IEEE Std time slot
from previous
time slot
Preponed
Pre-Active
Active
Inactive
Legend:
region
Pre-NBA
PLI region NBA
Post-NBA
Pre-Observed
Observed
Post-Observed
Reactive
Re-Inactive
Pre-Re-NBA
Re-NBA
Post-Re-NBA
Pre-Postponed to next
time slot
Postponed
Figure4-1—Event scheduling regions

<!-- Page 69 -->

IEEE Std ### 4.5 SystemVerilog simulation reference algorithm
execute_simulation {
T = 0;
initialize the values of all nets and variables;
schedule all initialization events into time zero slot;
while (some time slot is nonempty) {
move to the first nonempty time slot and set T;
execute_time_slot (T);
}
}
execute_time_slot {
execute_region (Preponed);
execute_region (Pre-Active);
while (any region in [Active ... Pre-Postponed] is nonempty) {
while (any region in [Active ... Post-Observed] is nonempty) {
execute_region (Active);
R = first nonempty region in [Active ... Post-Observed];
if (R is nonempty)
move events in R to the Active region;
}
while (any region in [Reactive ... Post-Re-NBA] is nonempty) {
execute_region (Reactive);
R = first nonempty region in [Reactive ... Post-Re-NBA];
if (R is nonempty)
move events in R to the Reactive region;
}
if (all regions in [Active ... Post-Re-NBA] are empty)
execute_region (Pre-Postponed);
}
execute_region (Postponed);
}
execute_region {
while (region is nonempty) {
E = any event from region;
remove E from the region;
if (E is an update event) {
update the modified object;
schedule evaluation event for any process sensitive to the object;
} else { /* E is an evaluation event */
evaluate the process associated with the event and possibly
schedule further events for execution;
}
}
}
The Iterative regions and their order are Active, Inactive, Pre-NBA,NBA, Post-NBA, Pre-Observed,
Observed, Post-Observed, Reactive, Re-Inactive, Pre-Re-NBA, Re-NBA, Post-Re-NBA, and Pre-
Postponed. As shown in the algorithm, once the Reactive, Re-Inactive, Pre-Re-NBA, Re-NBA, or Post-Re-
NBA regions are processed, iteration over the other regions does not resume until these five regions are
empty.

<!-- Page 70 -->

IEEE Std ### 4.6 Determinism
This standard guarantees a certain scheduling order:
a) Statements within a begin-end block shall be executed in the order in which they appear in that
begin-end block. Execution of statements in a particular begin-end block can be suspended in favor
of other processes in the model; however, in no case shall the statements in a begin-end block be
executed in any order other than that in which they appear in the source.
b) NBAs shall be performed in the order the statements were executed (see 10.4.2).
Consider the following example:
module test;
logic a;
initial begin
a <= 0;
a <= 1;
end
endmodule
When this block is executed, there will be two events added to the NBA region. The previous rule requires
that they be entered in the event region in execution order, which, in a sequential begin-end block, is source
order. This rule requires that they be taken from the NBA region and performed in execution order as well.
Hence, at the end of simulation time 0, the variable a will be assigned 0 and then 1.
### 4.7 Nondeterminism
One source of nondeterminism is the fact that active events can be taken off the Active or Reactive event
region and processed in any order. Another source of nondeterminism is that statements without time-
control constructs in procedural blocks do not have to be executed as one event. Time control statements are
the # expression and @ expression constructs (see 9.4). At any time while evaluating a procedural statement,
the simulator may suspend execution and place the partially completed event as a pending event in the event
region. The effect of this is to allow the interleaving of process execution, although the order of interleaved
execution is nondeterministic and not under control of the user.
### 4.8 Race conditions
Because the execution of expression evaluation and net update events may be intermingled, race conditions
are possible: For example:
assign p = q;
initial begin
q = 1;
#1 q = 0;
$display(p);
end
The simulator is correct in displaying either a 1 or a 0. The assignment of 0 to q enables an update event for
p. The simulator may either continue and execute the $display task or execute the update for p, followed
by the $display task.

<!-- Page 71 -->

IEEE Std ### 4.9 Scheduling implication of assignments
Assignments are translated into processes and events as detailed in 4.9.1 through 4.9.7.
#### 4.9.1 Continuous assignment
A continuous assignment statement (see 10.3) corresponds to a process, sensitive to the source elements in
the expression. When the value of the expression changes, it causes an active update event to be added to the
event region, using current values to determine the target. A continuous assignment process is also evaluated
at time zero in order to propagate constant values. This includes implicit continuous assignments inferred
from port connections (see 4.9.6).
#### 4.9.2 Procedural continuous assignment
A procedural continuous assignment (which is the assign or force statement; see 10.6) corresponds to a
process that is sensitive to the source elements in the expression. When the value of the expression changes,
it causes an active update event to be added to the event region, using current values to determine the target.
A deassign or a release statement deactivates any corresponding assign or force statement(s).
#### 4.9.3 Blocking assignment
A blocking assignment statement (see 10.4.1) with an intra-assignment delay computes the right-hand side
value using the current values, then causes the executing process to be suspended and scheduled as a future
event. If the delay is 0, the process is scheduled as an Inactive event for the current time. If a blocking
assignment with zero delay is executed from a Reactive region, the process is scheduled as a Re-Inactive
event.
When the process is returned (or if it returns immediately if no delay is specified), the process performs the
assignment to the left-hand side and enables any events based upon the update of the left-hand side. The
values at the time the process resumes are used to determine the target(s). Execution may then continue with
the next sequential statement or with other Active or Reactive events.
#### 4.9.4 Nonblocking assignment
A nonblocking assignment statement (see 10.4.2) always computes the updated value and schedules the
update as an NBA update event, either in the current time step if the delay is zero or as a future event if the
delay is nonzero. The values in effect when the update is placed in the event region are used to compute both
the right-hand value and the left-hand target.
#### 4.9.5 Switch (transistor) processing
The event-driven simulation algorithm described in 4.5 depends on unidirectional signal flow and can
process each event independently. The inputs are read, the result is computed, and the update is scheduled.
SystemVerilog provides switch-level modeling in addition to behavioral and gate-level modeling. Switches
provide bidirectional signal flow of wires of both built-in and user-defined net types (see 6.6) and require
coordinated processing of nodes connected by switches.
The source elements that model switches are various forms of transistors, called tran, tranif0, tranif1,
rtran, rtranif0, and rtranif1 (see 28.8).
Switch processing shall consider all the devices in a bidirectional switch-connected net before it can
determine the appropriate value for any node on the net because the inputs and outputs interact. A simulator

<!-- Page 72 -->

IEEE Std can do this using a relaxation technique. The simulator can process bidirectional switches at any time. It can
process a subset of bidirectional switch-connected events at a particular time, intermingled with the
execution of other active events.
For bidirectional switches connecting built-in net types, further refinement is required when some transistors
have gate value x. A conceptually simple technique is to solve the network repeatedly with these transistors
set to all possible combinations of fully conducting and nonconducting transistors. Any node that has a
unique logic level in all cases has steady-state response equal to this level. All other nodes have steady-state
response x.
When connecting user-defined net types, propagation of the signal from one terminal to the other follows the
same rules as in the propagation of built-in net types. If the control input is off, each net is resolved
separately; otherwise, they are resolved as if a single net. The bidirectional switch shall be treated as off for
an x or z control input value. This is different from the behavior of a bidirectional switch connecting built-
in net types, which is described in the preceding paragraph.
#### 4.9.6 Port connections
Ports connect processes through implicit continuous assignment statements or implicit bidirectional
connections. Bidirectional connections are analogous to an always-enabled tran connection between the
two nets, but without any strength reduction (A tran connection does not affect signal strength across the
bidirectional terminals, except that a supply strength is reduced to strong. See 28.13).
Ports can always be represented as declared objects connected as follows:
— If an input port, then a continuous assignment from an outside expression to a local (input) net or
variable
— If an output port, then a continuous assignment from a local output expression to an outside net or
variable
— If an inout port, then a non-strength-reducing transistor connecting the local net to an outside net
Primitive terminals, including UDP terminals, are different from module ports. Primitive output and inout
terminals shall be connected directly to 1-bit nets or 1-bit structural net expressions (see 23.3.3), with no
intervening process that could alter the strength. Changes from primitive evaluations are scheduled as active
update events in the connected nets. Input terminals connected to 1-bit nets or 1-bit structural net
expressions are also connected directly, with no intervening process that could affect the strength. Input
terminals connected to other kinds of expressions are represented as implicit continuous assignments from
the expression to an implicit net that is connected to the input terminal.
#### 4.9.7 Subroutines
Subroutine argument passing is by value, and it copies in on invocation and copies out on return. The copy-
out-on-the-return function behaves in the same manner as does any blocking assignment.
### 4.10 PLI callback control points
There are two kinds of PLI callbacks: those that are executed immediately when some specific activity
occurs, and those that are explicitly registered as a one-shot evaluation event.

<!-- Page 73 -->

IEEE Std Table4-1 provides the mapping from the various PLI callbacks.
Table4-1—PLI callbacks
Callback Event region
cbAfterDelay Pre-Active
cbNextSimTime Pre-Active
cbReadWriteSynch Pre-NBA or Post-NBA
cbAtStartOfSimTime Pre-Active
cbNBASynch Pre-NBA
cbAtEndOfSimTime Pre-Postponed
cbReadOnlySynch Postponed

<!-- Page 74 -->

1800-2023