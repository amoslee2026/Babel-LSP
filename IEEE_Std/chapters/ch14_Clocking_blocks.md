# Chapter 14: Clocking blocks

## 14. Clocking blocks
### 14.1 General
This clause describes the following:
— Clocking block declarations
— Input and output skews
— Clocking block signal events
— Cycle delays
— Synchronous events
— Synchronous drives
### 14.2 Overview
Module port connections and interfaces can specify the signals or nets through which a testbench
communicates with a device under test (DUT). However, such specification does not explicitly denote any
timing disciplines, synchronization requirements, or clocking paradigms.
The clocking block construct identifies clock signals and captures the timing and synchronization
requirements of the blocks being modeled. A clocking block is defined between the keywords clocking
and endclocking.
A clocking block assembles signals that are synchronous to a particular clock and makes their timing
explicit. The clocking block is a key element in a cycle-based methodology, which enables users to write
testbenches at a higher level of abstraction. Rather than focusing on signals and transitions in time, the test
can be defined in terms of cycles and transactions. Depending on the environment, a testbench can contain
one or more clocking blocks, each containing its own clock plus an arbitrary number of signals.
The clocking block separates the timing and synchronization details from the structural, functional, and
procedural elements of a testbench. Thus, the timing for sampling and driving clocking block signals is
implicit and relative to the clocking block’s clock. This enables a set of key operations to be written very
succinctly, without explicitly using clocks or specifying timing. These operations are as follows:
— Synchronous events
— Input sampling
— Synchronous drives
### 14.3 Clocking block declaration
The syntax for the clocking block is as follows in Syntax14-1.
clocking_declaration ::= // from A.6.11
[ default ] clocking [ clocking_identifier ] clocking_event ;
{ clocking_item }
endclocking [ : clocking_identifier ]
| global clocking [ clocking_identifier ] clocking_event ; endclocking [ : clocking_identifier ]
clocking_event ::= // from A.6.5
@ ps_identifier
| @ hierarchical_identifier

<!-- Page 356 -->

| @ ( event_expression )
clocking_item ::= // from A.6.11
default default_skew ;
| clocking_direction list_of_clocking_decl_assign ;
| { attribute_instance } assertion_item_declaration
default_skew ::=
input clocking_skew
| output clocking_skew
| input clocking_skew output clocking_skew
clocking_direction ::=
input [ clocking_skew ]
| output [ clocking_skew ]
| input [ clocking_skew ] output [ clocking_skew ]
| inout
list_of_clocking_decl_assign ::= clocking_decl_assign { , clocking_decl_assign }
clocking_decl_assign ::= signal_identifier [ = expression ]
clocking_skew ::=
edge_identifier [ delay_control ]
| delay_control
edge_identifier ::= posedge | negedge | edge // from A.7.2
delay_control ::= // from A.6.5
# delay_value
| # ( mintypmax_expression )
Syntax14-1—Clocking block syntax (excerpt from AnnexA)
The delay_control shall be either a time literal or a constant expression that evaluates to a non-negative
integer value.
The clocking_identifier specifies the name of the clocking block being declared. Only default clocking
blocks may be unnamed. Declarations in unnamed clocking blocks may not be referenced.
The signal_identifier specifies a signal (a net or variable) in the scope enclosing the clocking block
declaration, and defines a clockvar in the clocking block. The specified signal is called a clocking signal.
Unless a hierarchical expression is used (see 14.5), both the clocking signal and the clockvar names shall be
the same. It shall be illegal for a clocking signal to designate a variable restricted to a procedural block (see
6.21).
The clocking_event designates a particular event to act as the clock for the clocking block. The timing used
to drive and sample all other signals specified in a given clocking block is governed by its clocking event.
See 14.13 and 14.16 for details on the precise timing semantics of sampling and driving clocking signals.
It shall be illegal to write to any clockvar whose clocking_direction is input.
It shall be illegal to read the value of any clockvar whose clocking_direction is output.
A clockvar whose clocking_direction is inout shall behave as if it were two clockvars, one input and one
output, having the same name and the same clocking_signal. Reading the value of such an inout clockvar
shall be equivalent to reading the corresponding input clockvar. Writing to such an inout clockvar shall
be equivalent to writing to the corresponding output clockvar.

<!-- Page 357 -->

The clocking_skew determines how many time units away from the clock event a signal is to be sampled or
driven. Input skews are implicitly negative, that is, they always refer to a time before the clock, whereas
output skews always refer to a time after the clock (see 14.4). When the clocking event specifies a simple
edge, instead of a number, the skew can be specified as the specific edge of the signal. A single skew can be
specified for the entire block by using a default clocking item.
clocking ck1 @(posedge clk);
default input #1step output negedge; // legal
// outputs driven on the negedge clk
input ... ;
output ... ;
endclocking
clocking ck2 @(clk); // no edge specified!
default input #1step output negedge; // legal
input ... ;
output ... ;
endclocking
The expression assigned to the clockvar specifies that the signal to be associated with the clocking block is
associated with the specified hierarchical expression. For example, a cross-module reference can be used
instead of a local port. See 14.5 for more information.
Example:
clocking bus @(posedge clock1);
default input #10ns output #2ns;
input data, ready, enable = top.mem1.enable;
output negedge ack;
input #1step addr;
endclocking
In the preceding example, the first line declares a clocking block called bus that is to be clocked on the
positive edge of the signal clock1. The second line specifies that by default all signals in the clocking block
shall use a 10ns input skew and a 2ns output skew. The next line adds three input signals to the clocking
block: data, ready, and enable; the last signal refers to the hierarchical signal top.mem1.enable. The
fourth line adds the signal ack to the clocking block and overrides the default output skew so that ack is
driven on the negative edge of the clock. The last line adds the signal addr and overrides the default input
skew so that addr is sampled one step before the positive edge of the clock.
Unless otherwise specified, the default input skew is 1step and the default output skew is 0. A step is a
special time unit whose value is defined in 3.14.3. A 1step input skew allows input signals to sample their
steady-state values in the time step immediately before the clock event (i.e., in the preceding Postponed
region).
### 14.4 Input and output skews
Input (or inout) signals are sampled at the designated clock event. If an input skew is specified, then the
signal is sampled at skew time units before the clock event. Similarly, output (or inout) signals are driven
skew simulation time units after the corresponding clock event. Figure14-1 shows the basic sample and
drive timing for a positive edge clock.

<!-- Page 358 -->

signal sampled here signal driven here
clock
input skew output skew
Figure14-1—Sample and drive times including skew
with respect to the positive edge of the clock
A skew shall be a constant expression and can be specified as a parameter. If the skew does not specify a
time unit, the current time unit is used. If a number is used, the skew is interpreted using the timescale of the
current scope.
clocking dram @(clk);
input #1ps address;
input #5 output #6 data;
endclocking
An input skew of 1step indicates that the signal is to be sampled at the end of the previous time step. In
other words, the value sampled is always the signal’s last value immediately before the corresponding clock
edge.
NOTE—A clocking block does not eliminate potential races when an event control outside a program block is sensitive
to the same clock as the clocking block and a statement after the event control attempts to read a member of the clocking
block. The race is between reading the old sampled value and the new sampled value.
Inputs with explicit #0 skew shall be sampled at the same time as their corresponding clocking event, but to
avoid races, they are sampled in the Observed region. Likewise, clocking block outputs with no skew (or
explicit #0 skew) shall be driven at the same time as their specified clocking event, in the Re-NBA region.
Skews are declarative constructs; thus, they are semantically very different from the syntactically similar
procedural delay statement. In particular, an explicit #0 skew does not suspend any process, nor does it
execute or sample values in the Inactive region.
### 14.5 Hierarchical expressions
Any signal in a clocking block can be associated with an arbitrary hierarchical expression. As described in
14.3, a hierarchical expression is introduced by appending an equal sign (=) followed by the hierarchical
expression:
clocking cd1 @(posedge phi1);
input #1step state = top.cpu1.state;
endclocking
However, hierarchical expressions are not limited to simple names or signals in other scopes. They can be
used to declare slices and concatenations (or combinations thereof) of signals in other scopes or in the
current scope.

<!-- Page 359 -->

clocking mem @(clock);
input instruction = { opcode, regA, regB[3:1] };
endclocking
In a clocking block, any expression assigned to a signal in its declaration shall be an expression that would
be legal in a port connection to a port of appropriate direction. Any expression assigned to a signal in a
clocking input or inout declaration shall be an expression that would be legal for connection to a
module’s input port. Any expression assigned to a signal in a clocking output or inout declaration shall
be an expression that would be legal for connection to a module’s output port.
A clocking inout declaration is not an inout port; it is shorthand for two clocking declarations, one input
and one output, with the same signal. Consequently, such a signal needs to meet the requirements for both a
clocking input and a clocking output, but it is not required to meet the stricter requirements for connection to
a module’s inout port. In particular, it is acceptable to specify a variable as a clocking inout signal.
### 14.6 Signals in multiple clocking blocks
The same signals—clock, inputs, inouts, or outputs—can appear in more than one clocking block. When
clocking blocks use the same clock (or clocking expression), they shall share the same synchronization
event, in the same manner as several latches can be controlled by the same clock. Input semantics is
described in 14.13, and output semantics is described in 14.16.
### 14.7 Clocking block scope and lifetime
A clocking block is both a declaration and an instance of that declaration. A separate instantiation step is not
necessary. Instead, one copy is created for each instance of the block containing the declaration (like an
always procedure). Once declared, the clocking signals are available via the clocking block name and the dot
(.) operator:
dom.sig // signal sig in clocking dom
Multiple clocking blocks cannot be nested. They cannot be declared inside functions, tasks, or packages or
outside all declarations in a compilation unit. A clocking block can only be declared inside a module,
interface, checker, or program (see Clause24).
A clocking block has static lifetime and scope local to its enclosing module, interface, or program.
### 14.8 Multiple clocking blocks example
In this example, a simple test program includes two clocking blocks. The program construct used in this
example is discussed in Clause24.
program test( input phi1, input [15:0] data, output logic write,
input phi2, inout [8:1] cmd, input enable
);
reg [8:1] cmd_reg;
clocking cd1 @(posedge phi1);
input data;
output write;
input state = top.cpu1.state;
endclocking

<!-- Page 360 -->

clocking cd2 @(posedge phi2);
input #2 output #4ps cmd;
input enable;
endclocking
initial begin
// program begins here
...
// user can access cd1.data, cd2.cmd, etc…
end
assign cmd = enable ? cmd_reg: 'x;
endprogram
The test program can be instantiated and connected to a DUT (cpu and mem).
module top;
logic phi1, phi2;
wire [8:1] cmd; // cannot be logic (two bidirectional drivers)
logic [15:0] data;
test main (phi1, data, write, phi2, cmd, enable);
cpu cpu1 (phi1, data, write);
mem mem1 (phi2, cmd, enable);
endmodule
### 14.9 Interfaces and clocking blocks
A clocking block encapsulates a set of signals that share a common clock; therefore, specifying a clocking
block using a SystemVerilog interface (see Clause25) can significantly reduce the amount of code
needed to connect the testbench. Furthermore, because the signal directions in the clocking block within the
testbench are with respect to the testbench and not the design under test, a modport declaration (see 25.5)
can appropriately describe either direction. A testbench can be contained within a program, and its ports can
be interfaces that correspond to the signals declared in each clocking block. The interface’s wires would
have the same direction as specified in the clocking block when viewed from the testbench side (i.e.,
modport test) and reversed when viewed from the DUT (i.e., modport dut).
For example, the previous example could be rewritten using interfaces as follows:
interface bus_A (input clk);
logic [15:0] data;
logic write;
modport test (input data, output write, input clk);
modport dut (output data, input write, input clk);
endinterface
interface bus_B (input clk);
logic [8:1] cmd;
logic enable;
modport test (input enable, input clk, inout cmd);
modport dut (output enable, input clk, input cmd);
endinterface
program test(bus_A.test a, bus_B.test b);
reg [8:1] cmd_reg;
clocking cd1 @(posedge a.clk);
input data = a.data;
output write = a.write;

<!-- Page 361 -->

input state = top.cpu1.state;
endclocking
clocking cd2 @(posedge b.clk);
input #2 output #4ps cmd = b.cmd;
input en = b.enable;
endclocking
initial begin
// program begins here
...
// user can access cd1.data, cd1.write, cd1.state,
// cd2.cmd, and cd2.en
end
assign cmd = enable ? cmd_reg: 'x;
endprogram
The test module can be instantiated and connected as before:
module top;
logic phi1, phi2;
bus_A a (phi1);
bus_B b (phi2);
test main (a, b);
cpu cpu1 (a);
mem mem1 (b);
endmodule
### 14.10 Clocking block events
Upon processing its specified clocking event, a clocking block shall trigger the event associated with the
clocking block name. This event shall be triggered in the Observed region and is referred to as a clocking
block event.
For example:
module foo (input phi1, input [7:0] data);
clocking dram @(posedge phi1);
input data;
output negedge #1 address;
endclocking
always @(posedge phi1) $display("clocking event");
always @(dram) $display("clocking block event");
endmodule
The preceding first always procedure executes in the same event region within the active region set (see
4.4.1) as the update event for the positive edge of phi1. In contrast, the second always procedure executes
in the first Active region after the Observed region that follows the update event's active region set. This
behavior can be used to avoid race conditions when sampling input data. See 14.13 for more details.

<!-- Page 362 -->

### 14.11 Cycle delay: ##
The ## operator can be used to delay execution by a specified number of clocking block events or clock
cycles.
The syntax for the cycle delay statement is as follows in Syntax14-2.
procedural_timing_control_statement ::= // from A.6.5
procedural_timing_control statement_or_null
procedural_timing_control ::=
...
| cycle_delay
cycle_delay ::= // from A.6.11
## integral_number
| ## identifier
| ## ( expression )
Syntax14-2—Cycle delay syntax (excerpt from AnnexA)
The cycle_delay can be any SystemVerilog expression that evaluates to a non-negative integer value.
What constitutes a cycle is determined by the default clocking in effect (see 14.12). If no default clocking
has been specified for the current module, interface, checker, or program, then the compiler shall issue an
error.
Example:
##5; // wait 5 cycles (clocking block events) using default clocking
##(j + 1); // wait j+1 cycles (clocking block events) using default clocking
The cycle delay timing control shall wait for the specified number of clocking block events. This implies
that for a ##1 statement that is executed at a simulation time that is not coincident with the associated
clocking block event, the calling process shall be delayed a fraction of the associated clock cycle.
Cycle delays of ##0 are treated specially. If a clocking block event has not yet occurred in the current time
step, a ##0 cycle delay shall suspend the calling process until the clocking block event occurs. When a
process executes a ##0 cycle delay and the associated clocking block event has already occurred in the
current time step, the process shall continue execution without suspension. When used on the right-hand side
of a synchronous drive, a ##0 cycle delay shall have no effect, as if it were not present.
Cycle delay timing controls shall not be legal for use in intra-assignment delays in either blocking or
nonblocking assignment statements.
### 14.12 Default clocking
One clocking block can be specified as the default for all cycle delay operations within a given module,
interface, program, or checker.
The syntax forthe default clocking specification statement is as follows in Syntax14-3.

<!-- Page 363 -->

module_or_generate_item_declaration ::= // from A.1.4
...
| default clocking clocking_identifier ;
...
checker_or_generate_item_declaration ::= // from A.1.8
...
| default clocking clocking_identifier ;
...
clocking_declaration ::= // from A.6.11
[ default ] clocking [ clocking_identifier ] clocking_event ;
{ clocking_item }
endclocking [ : clocking_identifier ]
...
Syntax14-3—Default clocking syntax (excerpt from AnnexA)
The clocking_identifier shall be the name of a clocking block.
Only one default clocking can be specified in a module, interface, program, or checker. Specifying a default
clocking more than once in the same module, interface, program, or checker shall result in a compiler error.
A default clocking is valid only within the scope containing the default clocking specification statement.
This scope includes the module, interface, program, or checker that contains the declaration as well as any
nested modules, interfaces, or checkers. It does not include instantiated modules, interfaces, or checkers.
Example 1: Declaring a clocking as the default:
program test(input logic clk, input logic [15:0] data);
default clocking bus @(posedge clk);
inout data;
endclocking
initial begin
## 5;
if (bus.data == 10)
## 1;
else
...
end
endprogram
Example 2: Assigning an existing clocking to be the default:
module processor ...
clocking busA @(posedge clk1); ... endclocking
clocking busB @(negedge clk2); ... endclocking
module cpu( interface y );
default clocking busA ;
initial begin
## 5; // use busA => (posedge clk1)
...
end
endmodule

<!-- Page 364 -->

endmodule
### 14.13 Input sampling
All clocking block inputs (input or inout) are sampled at the corresponding clocking event. If the input
skew is not an explicit #0, then the value sampled corresponds to the signal value at the Postponed region of
the time step skew time units prior to the clocking event (see Figure14-1 in 14.4). If the input skew is an
explicit #0, then the value sampled corresponds to the signal value in the Observed region. In this case, the
newly sampled values shall be available for reading at the end of the Observed region processing. If upon
processing the Reactive region, the simulation needs to process Active events without advancing time
(thereby executing the Observed region more than once), clocking inputs sampled with #0 skew shall not be
resampled unless a new clocking event occurs in the active region set.
NOTE—When the clocking event is triggered by the execution of a program, there is a potential race between the update
of a clocking block input value and programs that read that value without synchronizing with the corresponding clocking
event. This race does not exist when the clocking event is triggered from within a module.
Upon processing its specified clocking event, a clocking block shall update its sampled values before
triggering the clocking block event (see 14.10).Since the clocking block event is triggered in the Observed
region,a process that waits for the clocking block itself is guaranteed to read the updated sampled values,
regardless of the scheduling region in which either the waiting or the triggering processes execute. For
example:
clocking cb @(negedge clk);
input v;
endclocking
always @(cb) $display(cb.v);
always @(negedge clk) $display(cb.v);
The preceding first always procedure is guaranteed to display the updated sampled value of signal v. In
contrast, the second always exhibits a potential race and may display the old or the newly updated sampled
value.
When an input or inout clockvar appears in any expression its value is the signal’s sampled value; that is,
the value that the clocking block sampled at the most recent clocking event.
When the same signal is an input to multiple clocking blocks, the semantics is straightforward; each
clocking block samples the corresponding signal with its own clocking event.
### 14.14 Global clocking
A clocking block may be declared as the global clocking for all or part of the design hierarchy. The main
purpose of global clocking is to specify which clocking event in simulation corresponds to the primary
system clock used in formal verification (see F.3.1). Such a specification may be done for a whole design, or
separately for different subsystems in a design, when there are multiple clocks and building a single global
clocking event is challenging.
The syntax for the global clocking declaration is as follows in Syntax14-4.
clocking_declaration ::= // from A.6.11
...

<!-- Page 365 -->

| global clocking [ clocking_identifier ] clocking_event ; endclocking [ : clocking_identifier ]
Syntax14-4—Global clocking syntax (excerpt from AnnexA)
Global clocking may be declared in a module, an interface, a checker, or a program. A given module,
interface, checker, or program shall contain at most one global clocking declaration. Global clocking shall
not be declared in a generate block.
Although more than one global clocking declaration may appear in different parts of the design hierarchy, at
most one global clocking declaration is effective at each point in the elaborated design hierarchy. The
$global_clock system function shall be used to explicitly refer to the event expression in the effective
global clocking declaration. The effective global clocking declaration for a specific reference is determined
using the following hierarchical lookup rules, which iteratively check the design hierarchy to find the global
clocking declaration closest to the point of reference.
a) Look for a global clocking declaration in the enclosing module, interface, checker, or program
instance scope. If found, the lookup terminates with the result being the event expression of that
global clocking declaration. If not found and the current scope is a top-level hierarchy block (see
3.11), the lookup terminates and shall result in an error. Otherwise, proceed to step b).
b) Look for a global clocking declaration in the parent module, interface, or checker instance scope of
the enclosing instantiation. If found, the lookup terminates with the result being the event expression
of that global clocking declaration. Otherwise, continue up the hierarchy until a global clocking dec-
laration is found or a top-level hierarchy block is reached. If no global clocking declaration is found
and a top-level hierarchy block is reached, the lookup terminates and shall result in an error.
When global clocking is referenced in a sequence declaration, a property declaration, or as an actual
argument to a named sequence instance, a named property instance, or a checker instance, the point of
reference shall be considered after the application of the rewriting algorithms defined in F.4. As a result,
when a property or a sequence declaration is instantiated in an assertion statement, the hierarchical lookup
rules described previously shall be applied from the point of the assertion statement appearance in the source
description, not from the point of the sequence or the property declaration. Similarly, the lookup rules shall
be applied after the substitution of the actual argument in place of the corresponding formal argument inside
the checker body.
The following is an example of a globalclocking declaration:
module top;
logic clk1, clk2;
global clocking sys @(clk1 or clk2); endclocking
// ...
endmodule
In this example, sys is declared as the global clocking event and is defined to occur if, and only if, there is a
change of either of two signals, clk1 and clk2. Specification of the name sys in the global clocking
declaration is optional since the global clocking event may be referenced by $global_clock .
In the following example the design hierarchy contains two global clocking declarations. The call to
$global_clock in top.sub1.common resolves to the clocking event top.sub1.sub_sys1. The call to
$global_clock in top.sub2.common resolves to the clocking event top.sub2.sub_sys2.
module top;
subsystem1 sub1();
subsystem2 sub2();
endmodule

<!-- Page 366 -->

module subsystem1;
logic subclk1;
global clocking sub_sys1 @(subclk1); endclocking
// ...
common_sub common();
endmodule
module subsystem2;
logic subclk2;
global clocking sub_sys2 @(subclk2); endclocking
// ...
common_sub common();
endmodule
module common_sub;
always @($global_clock) begin
// ...
end
endmodule
In the following example the property p is declared in a module containing a global clocking declaration.
However, that global clocking declaration is not effective where the property p is instantiated. Similar to the
previous example, the call to $global_clock in top.sub1.checks resolves to the clocking event
top.sub1.sub_sys1, and the call to $global_clock in top.sub2.checks resolves to the clocking
event top.sub2.sub_sys2.
module top;
subsystem1 sub1();
subsystem2 sub2();
endmodule
module subsystem1;
logic subclk1, req, ack;
global clocking sub_sys1 @(subclk1); endclocking
// ...
common_checks checks(req, ack);
endmodule
module subsystem2;
logic subclk2, req, ack;
global clocking sub_sys2 @(subclk2); endclocking
// ...
common_checks checks(req, ack);
endmodule
module another_module;
logic another_clk;
global clocking another_clocking @(another_clk); endclocking
// ...
property p(req, ack);
@($global_clock) req |=> ack;
endproperty
endmodule
checker common_checks(logic req, logic ack);
assert property (another_module.p(req, ack));
endchecker

<!-- Page 367 -->

In the following example, $global_clock is used in a task. The call to $global_clock in the task
another_module.t resolves to the clocking event another_module.another_clocking.
module top;
subsystem1 sub1();
subsystem2 sub2();
endmodule
module subsystem1;
logic subclk1, req, ack;
global clocking sub_sys1 @(subclk1); endclocking
// ...
always another_module.t(req, ack);
endmodule
module subsystem2;
logic subclk2, req, ack;
global clocking sub_sys2 @(subclk2); endclocking
// ...
always another_module.t(req, ack);
endmodule
module another_module;
logic another_clk;
global clocking another_clocking @(another_clk); endclocking
task t(input req, input ack);
@($global_clock);
// ...
endtask
endmodule
The following example demonstrates the usage of $global_clock in checker arguments. The resolution of
the calls to $global_clock is performed after the substitution of the actual checker arguments in place of
the corresponding formal arguments, and flattening of the properties in the assertion statements. All calls to
$global_clock in this example refer to the clocking event top.check.checker_clocking.
module top;
logic a, b, c, clk;
global clocking top_clocking @(clk); endclocking
// ...
property p1(req, ack);
@($global_clock) req |=> ack;
endproperty
property p2(req, ack, interrupt);
@($global_clock) accept_on(interrupt) p1(req, ack);
endproperty
my_checker check(
p2(a, b, c),
@$global_clock a[*1:$] ##1 b);
endmodule
checker my_checker(property p, sequence s);
logic checker_clk;
global clocking checker_clocking @(checker_clk); endclocking

<!-- Page 368 -->

// ...
assert property (p);
cover property (s);
endchecker
NOTE—This is an area of backward incompatibility between this standard and 14.14 of IEEE Std 1800-2009. In the
2009 definition, only one global clocking declaration was allowed in the elaborated design description; it could be
specified in any module, interface, or checker and referenced everywhere in the description. A design conforming to
IEEE Std 1800-2009 could have a global clocking declaration defined in a non-top-level module and use
$global_clock outside the subhierarchy of that module. Such a design shall not conform to this standard.
### 14.15 Synchronous events
Explicit synchronization is done via the event control operator, @, which allows a process to wait for a
particular signal value change or a clocking event (see 14.10).
The syntax for the synchronization operator is given in 9.4.2.
The expression used with the event control can denote clocking block input (input or inout) or a slice
thereof. Slices can include dynamic indices, which are evaluated once when the @ expression executes.
Following are some examples of synchronization statements:
— Wait for the next change of signal ack_1 of clocking block ram_bus
@(ram_bus.ack_l);
— Wait for the next clocking event in clocking block ram_bus
@(ram_bus);
— Wait for the positive edge of the signal ram_bus.enable
@(posedge ram_bus.enable);
— Wait for the falling edge of the specified 1-bit slice dom.sign[a]
@(negedge dom.sign[a]);
NOTE—The index a is evaluated at run time.
— Wait for either the next positive edge of dom.sig1 or the next change of dom.sig2, whichever
happens first
@(posedge dom.sig1 or dom.sig2);
— Wait for either the negative edge of dom.sig1 or the positive edge of dom.sig2, whichever
happens first
@(negedge dom.sig1 or posedge dom.sig2);
— Wait for the edge (either the negative edge or the positive edge, whichever happens first) of
dom.sig1.
@(edge dom.sig1);
Or equivalently
@(negedge dom.sig1 or posedge dom.sig1);
The values used by the synchronization event control are the synchronous values, that is, the values sampled
at the corresponding clocking event.

<!-- Page 369 -->

### 14.16 Synchronous drives
Clocking block outputs (output or inout) are used to drive values onto their corresponding signals, but at
a specified time. In other words, the corresponding signal changes value at the indicated clocking event as
modified by the output skew.
For zero skew clocking block outputs with no cycle delay, synchronous drives shall schedule new values in
the Re-NBA region of the time step corresponding to the clocking event. For clocking block outputs with
nonzero skew, or drives with nonzero cycle delay, the corresponding signal shall be scheduled to change
value in the Re-NBA region of a future time step.
For each clocking block output whose target is a net, a driver on that net shall be created. The driver so
created shall have (strong1, strong0) drive strength and shall be updated as if by a continuous
assignment from a variable inside the clocking block. This implicit variable, which is invisible to user code,
shall be updated in the Re-NBA region by the execution of a synchronous drive to the corresponding
clockvar. The created driver shall be initialized to 'z; hence, the driver has no influence on its target net
until a synchronous drive is performed to the corresponding clockvar.
The syntax to specify a synchronous drive is similar to an assignment and is shown in Syntax14-5.
statement ::= [ block_identifier : ] { attribute_instance } statement_item // from A.6.4
statement_item ::=
...
| clocking_drive ;
clocking_drive ::= // from A.6.11
clockvar_expression <= [ cycle_delay ] expression
cycle_delay ::=
## integral_number
| ## identifier
| ## ( expression )
clockvar ::= hierarchical_identifier
clockvar_expression ::= clockvar select
Syntax14-5—Synchronous drive syntax (excerpt from AnnexA)
The clockvar_expression is a bit-select, slice, or the entire clocking block output whose corresponding
signal is to be driven (concatenation is not allowed):
dom.sig // entire clockvar
dom.sig[2] // bit-select
dom.sig[8:2] // slice
The expression (in the clocking_drive production) can be any valid expression that is assignment compatible
with the type of the corresponding signal.
The optional cycle_delay construct, appearing on the right-hand side of a clocking_drive statement, is
syntactically similar to an intra-assignment delay in a nonblocking assignment. Like a nonblocking intra-
assignment delay, it shall not cause execution of the statement to block. The right-hand side expression shall
be evaluated immediately even when a cycle_delay is present. However, updating of the target signal shall

<!-- Page 370 -->

be postponed for the specified number of cycles of the target clockvar’s clocking block, plus any clocking
output skew specified for that clockvar.
No other form of intra-assignment delay syntax shall be legal in a synchronous drive to a clockvar.
A procedural cycle delay, as described in 14.11, can be used as a prefix to any procedural statement. If a
procedural cycle delay is used as a prefix to a synchronous drive, it shall block for its specified number of
cycles of the default clocking exactly as it would if used as a prefix to any other procedural statement.
Examples:
bus.data[3:0] <= 4'h5; // drive data in Re-NBA region of the current cycle
##1 bus.data <= 8'hz; // wait 1 default clocking cycle, then drive data
##2; bus.data <= 2; // wait 2 default clocking cycles, then drive data
bus.data <= ##2 r; // remember the value of r and then drive
// data 2 (bus) cycles later
bus.data <= #4 r; // error: regular intra-assignment delay not allowed
// in synchronous drives
Regardless of whether the synchronous drive takes effect on the current clocking event or at some future
clocking event as a result of a cycle_delay, the corresponding signal shall be updated at a time after that
clocking event as specified by the output skew.
It is possible for a drive statement to execute at a time that is not coincident with its clocking event. Such
drive statements shall execute without blocking, but shall perform their drive action as if they had executed
at the time of the next clocking event. The expression on the right-hand side of the drive statement shall be
evaluated immediately, but the processing of the drive is delayed until the time of the next clocking event.
For example:
default clocking cb @(posedge clk); // Assume clk has a period of #10 units
output v;
endclocking
initial begin
#3 cb.v <= expr1; // Matures in cycle 1; equivalent to ##1 cb.v <= expr1
end
It shall be an error to write to a clockvar except by using the synchronous drive syntax described in this
subclause. Thus, it is illegal to use a continuous assignment or procedural continuous assignment (assign
or force) to write to a clockvar.
#### 14.16.1 Drives and nonblocking assignments
Although synchronous drives use the same operator syntax as nonblocking variable assignments, they are
not the same. One difference is that synchronous drives do not support intra-assignment delay syntax. A key
feature of synchronous drives to inout clockvars is that a drive does not change the clocking block input.
This is because reading the input always yields the last sampled value, and not the driven value.
For example, consider the following code:
clocking cb @(posedge clk);

<!-- Page 371 -->

inout a;
output b;
endclocking
initial begin
cb.a <= c; // The value of a will change in the Re-NBA region
cb.b <= cb.a; // b is assigned the value of a before the change
end
#### 14.16.2 Driving clocking output signals
When more than one synchronous drive on the same clocking block output (or inout) is scheduled to mature
in the same Re-NBA region of the same time step, the last value is the only value driven onto the output
signal. This is true whether the synchronous drives execute at times coincident with clocking events, or at
times in between clocking events (within the same clock cycle).
For example:
default clocking pe @(posedge clk);
output nibble; // four bit output
endclocking
initial begin
##2;
pe.nibble <= 4'b0101;
pe.nibble <= 4'b0011;
end
The driven value of nibble is 4'b0011, regardless of whether nibble is a variable or a net.
It is possible for the scheduling loop described in 4.4 to iterate through the Re-NBA region more than once
in a given time step. If this happens, synchronous drives will cause their associated clocking signal to glitch
(i.e., change value more than once in a time step) if they assign different values to their associated clockvar
in different iterations of the scheduling loop.
In the following example, variable a will glitch 1 –> 0 –> 1 at the first posedge of clk.
module m;
bit a = 1'b1;
default clocking cb @(posedge clk);
output a;
endclocking
initial begin
## 1;
cb.a <= 1'b0;
@(x); // x is triggered by reactive stimulus running in same time step
cb.a <= 1'b1;
end
endmodule
If a given clocking output is driven by multiple synchronous drives that are scheduled to mature at different
future times due to the use of cycle delay, the drives shall each mature in their corresponding future cycles.
For example:
bit v;

<!-- Page 372 -->

default clocking cb @(posedge clk);
output v;
endclocking
initial begin
##1; // Wait until cycle 1
cb.v <= expr1; // Matures in cycle 1, v is assigned expr1
cb.v <= ##2 expr2; // Matures in cycle 3
#1 cb.v <= ##2 expr3; // Matures in cycle 3
##1 cb.v <= ##1 expr4; // Matures in cycle 3, v is assigned expr4
end
When the same variable is an output from multiple clocking blocks, the last drive determines the value of the
variable. This allows a single module to model multirate devices, such as a DDR memory, using a different
clocking block to model each active edge. For example:
reg j;
clocking pe @(posedge clk);
output j;
endclocking
clocking ne @(negedge clk);
output j;
endclocking
The variable j is an output from two clocking blocks using different clocking events, @(posedge clk)
versus @(negedge clk). When driven, the variable j shall take on the value most recently assigned by
either clocking block. A clocking block output only assigns a value to its associated signal in clock cycles
where a synchronous drive occurs.
With the edge event, this is equivalent to the following simplified declaration:
reg j;
clocking e @(edge clk);
output j;
endclocking
Multiple clocking block outputs driving a net cause the net to be driven to its resolved signal value. As
described in 14.16, when a clocking block output corresponds to a net, a driver on that net is created. This
semantic model applies to each clocking block output that drives the net. The driving values of all these
driver(s), together with any other drivers on the net, shall be resolved as determined by the net’s type.
It is possible to use a procedural assignment to assign to a signal associated with an output clockvar. When
the associated signal is a variable, the procedural assignment assigns a new value to the variable, and the
variable shall hold that value until another assignment occurs (either from a drive to a clocking block output
or another procedural assignment).
If a synchronous drive and a procedural nonblocking assignment write to the same variable in the same time
step, the writes shall take place in an arbitrary order.
It shall be illegal to write to a variable with a continuous assignment, a procedural continuous assignment, or
a primitive when that variable is associated with an output clockvar.

<!-- Page 373 -->

IEEE Std 1800-2023