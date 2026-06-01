# Chapter 9: Processes

## 9. Processes
### 9.1 General
This clause describes the following:
— Structured procedures (initial procedures, always procedures, final procedures)
— Block statements (begin-end sequential blocks, fork-join parallel blocks)
— Timing control (delays, events, waits, intra-assignment)
— Process threads and process control
### 9.2 Structured procedures
All structured procedures in SystemVerilog are specified within one of the following constructs:
— initial procedure, denoted with the keyword initial (see 9.2.1)
— always procedure, denoted with the keywords:
• always (see 9.2.2.1)
• always_comb (see 9.2.2.2)
• always_latch (see 9.2.2.3)
• always_ff (see 9.2.2.4)
— final procedure, denoted with the keyword final (see 9.2.3)
— Task
— Function
The syntax for these structured procedures is shown in Syntax9-1.
initial_construct ::= initial statement_or_null // from A.6.2
always_construct ::= always_keyword statement
always_keyword ::= always | always_comb | always_latch | always_ff
final_construct ::= final function_statement
function_declaration ::= // from A.2.6
function [ dynamic_override_specifiers ]25 [ lifetime ] function_body_declaration
task_declaration ::= // from A.2.7
task [ dynamic_override_specifiers ]25 [ lifetime ] task_body_declaration
25) The dynamic_override_specifiers shall only be legal on method declarations inside a non-interface class scope.
Syntax9-1—Syntax for structured procedures (excerpt from AnnexA)
The initial and always procedures are enabled at the beginning of a simulation. The initial procedure shall
execute only once, and its activity shall cease when the statement has finished. In contrast, an always
procedure shall execute repeatedly, and its activity shall cease only when the simulation is terminated.
There shall be no implied order of execution between initial and always procedures. The initial procedures
need not be scheduled and executed before the always procedures. There shall be no limit to the number of
initial and always procedures that can be defined in a module. See 6.8 for the order of variable initialization
relative to the execution of procedures.

<!-- Page 222 -->

IEEE Std The final procedures are enabled at the end of simulation time and execute only once.
Tasks and functions are procedures that are enabled from one or more places in other procedures. Tasks and
functions are described in Clause13.
In addition to these structured procedures, SystemVerilog contains other procedural contexts, such as
coverage point expressions (19.5), assertion sequence match items (16.10, 16.11), and action blocks (16.14).
SystemVerilog has the following types of control flow within a procedure:
— Selection, loops, and jumps (see Clause12)
— Subroutine calls (see Clause13)
— Sequential and parallel blocks (see 9.3)
— Timing control (see 9.4)
— Process control (see 9.5 through 9.7)
#### 9.2.1 Initial procedures
An initial procedure shall execute only once, and its activity shall cease when the statement has finished.
The following example illustrates use of an initial procedure for initialization of variables at the start of
simulation.
initial begin
a = 0; // initialize a
for (int index = 0; index < size; index++)
memory[index] = 0; // initialize memory word
end
Another typical usage of the initial procedure is specification of waveform descriptions that execute once to
provide stimulus to the main part of the circuit being simulated.
initial begin
inputs = 'b000000; // initialize at time zero
#10 inputs = 'b011001; // first pattern
#10 inputs = 'b011011; // second pattern
#10 inputs = 'b011000; // third pattern
#10 inputs = 'b001000; // last pattern
end
#### 9.2.2 Always procedures
There are four forms of always procedures: always, always_comb, always_latch, and always_ff. All
forms of always procedures repeat continuously throughout the duration of the simulation.
9.2.2.1 General purpose always procedure
The always keyword represents a general purpose always procedure, which can be used to represent
repetitive behavior such as clock oscillators. The construct can also be used with proper timing controls to
represent combinational, latched, and sequential hardware behavior.
The general purpose always procedure, because of its looping nature, is only useful when used in
conjunction with some form of timing control. If an always procedure has no control for simulation time to
advance, it will create a simulation deadlock condition.

<!-- Page 223 -->

IEEE Std The following code, for example, creates a zero-delay infinite loop:
always areg = ~areg;
Providing a timing control to the preceding code creates a potentially useful description as shown in the
following:
always #half_period areg = ~areg;
9.2.2.2 Combinational logic always_comb procedure
SystemVerilog provides a special always_comb procedure for modeling combinational logic behavior. For
example:
always_comb
a = b & c;
always_comb
d <= #1ns b & c;
The always_comb procedure provides functionality that is different from the general purpose always
procedure, as follows:
— There is an inferred sensitivity list that includes the expressions defined in 9.2.2.2.1.
— The variables assigned on the left-hand side of assignments shall not be assigned by any other
process. This includes variables assigned within functions called by the procedure but not those
assigned within tasks called by the procedure. However, multiple assignments made to independent
elements of a variable are allowed as long as their longest static prefixes do not overlap (see 11.5.3).
For example, an unpacked structure or array can have one element assigned by an always_comb
procedure and another element assigned continuously or by another always_comb procedure. See
### 6.5 for more details.
— The procedure is automatically triggered once at time zero, after all initial and always
procedures have been started so that the outputs of the procedure are consistent with the inputs.
Software tools should perform additional checks to warn if the behavior within an always_comb procedure
does not represent combinational logic, such as if latched behavior can be inferred.
9.2.2.2.1 Implicit always_comb sensitivities
The implicit sensitivity list of an always_comb includes the expansions of the longest static prefix of each
net or variable identifier or select expression that is read within the block or within any function called
within the block with the following exceptions:
a) Any expansion of a variable declared within the block or within any function called within the block
b) Any expression that is also written within the block or within any function called within the block
c) Identifiers that only appear in timing control (wait, event, or delay) expressions
For the definition of the longest static prefix, see 11.5.3.
Hierarchical function calls and function calls from packages are analyzed as normal functions, as are calls to
static method functions referenced with the class scope resolution operator (see 8.23). References to class
objects and method calls of class objects do not add anything to the sensitivity list of an always_comb,
except for any contributions from argument expressions passed to these method calls.

<!-- Page 224 -->

IEEE Std Task calls are allowed in an always_comb. The arguments of the task calls are added to the sensitivity list,
but the contents of the tasks do not add anything to the sensitivity list.
NOTE—A task that does not consume time may be replaced by a void function so that the contents will be analyzed for
sensitivity.
An expression used in an immediate assertion (see 16.3) within the procedure, or in any function called
within the procedure, contributes to the implicit sensitivity list of an always_comb as if that expression
were used as a condition of an if statement. Expressions used in assertion action blocks do not contribute to
the implicit sensitivity list of an always_comb. In the following example, the always_comb shall trigger
whenever b, c or e changes.
always_comb
begin
a = b & c;
A1:assert (a != e) else if (!disable_error) $error("failed");
end
9.2.2.2.2 always_comb compared to always @*
The SystemVerilog always_comb procedure differs from always @* (see 9.4.2.2) in the following ways:
— always_comb automatically executes once at time zero, whereas always @* waits until a change
occurs on a signal in the inferred sensitivity list.
— always_comb is sensitive to changes tovariablesread within a function called within the proce-
dure, whereas always @* is sensitive to changes only to the arguments of such a function.
— Variables on the left-hand side of assignments within an always_comb procedure, including
variables assigned within functions called by the procedure, have restrictions on assignments by
other processes (see 9.2.2.2), whereas always @* permits multiple processes to assign to the same
variable.
— Statements in an always_comb shall not include those that block, have blocking timing or event
controls, or fork-join statements.
— always_comb is sensitive to expressions in immediate assertions within the procedure and within
the contents of a function called in the procedure, whereas always @* is sensitive to expressions in
immediate assertions within the procedure only.
9.2.2.3 Latched logic always_latch procedure
SystemVerilog also provides a special always_latch procedure for modeling latched logic behavior. For
example:
always_latch
if (ck) q <= d;
The always_latch construct is identical to the always_comb construct except that software tools should
perform additional checks and warn if the behavior in an always_latch construct does not represent
latched logic, whereas in an always_comb construct, tools should check and warn if the behavior does not
represent combinational logic. All statements in 9.2.2.2 shall apply to always_latch.
9.2.2.4 Sequential logic always_ff procedure
The always_ff procedure can be used to model synthesizable flip-flop logic behavior. For example:
always_ff @(posedge clock iff reset == 0 or posedge reset) begin
r1 <= reset ? 0 : r2 + 1;

<!-- Page 225 -->

IEEE Std ...
end
The always_ff procedure imposes the restriction that it contains one and only one event control and no
blocking timing controls. Variables on the left-hand side of assignments within an always_ff procedure,
including variables assigned within functions called by the procedure, have the same restrictions on
assignments by other processes as they have in always_comb (see 9.2.2.2).
Software tools should perform additional checks to warn if the behavior within an always_ff procedure
does not represent sequential logic.
#### 9.2.3 Final procedures
The final procedure is like an initial procedure, defining a procedural block of statements, except that
it occurs at the end of simulation time and executes without delays. A final procedure is typically used to
display statistical information about the simulation.
The only statements allowed inside a final procedure are those permitted inside a function declaration, so
that they execute within a single simulation cycle. Unlike an initial procedure, the final procedure does
not execute as a separate process; instead, it executes in zero time, as a series of function calls from a single
process. All final procedures shall execute in an arbitrary order. No remaining scheduled events shall
execute after all final procedures have executed.
A final procedure executes when simulation ends due to an explicit or implicit call to $finish.
final
begin
$display("Number of cycles executed %d",$time/period);
$display("Final PC = %h",PC);
end
Execution of $finish, tf_dofinish(), or vpi_control(vpiFinish,...) from within a final
procedure shall cause the simulation to end immediately. A final procedure can only trigger once in a
simulation.
A final procedure shall execute before any PLI callbacks that indicate the end of simulation.
SystemVerilog final procedures execute in an arbitrary but deterministic sequential order. This is possible
because final procedures are limited to the legal set of statements allowed for functions.
NOTE—SystemVerilog does not specify the ordering in which final procedures are executed, but implementations
should define rules that preserve the ordering between runs. This helps keep the output log file stable because final
procedures are mainly used for displaying statistics.
### 9.3 Block statements
Block statements are a means of grouping statements together so that they act syntactically like a single
statement. There are two types of blocks:
— Sequential block, also called begin-end block
— Parallel block, also called fork-join block
The sequential block shall be delimited by the keywords begin and end. The procedural statements in a
sequential block shall be executed sequentially in the given order.

<!-- Page 226 -->

IEEE Std The parallel block shall be delimited by the keywords fork and join, join_any, or join_none. The
procedural statements in a parallel block shall be executed concurrently.
#### 9.3.1 Sequential blocks
A sequential block shall have the following characteristics:
— Statements shall be executed in sequence, one after another.
— Delay values for each statement shall be treated relative to the simulation time of the execution of
the previous statement.
— Control shall pass out of the block after the last statement executes.
Syntax9-2 gives the formal syntax for a sequential block.
seq_block ::= // from A.6.3
begin [ : block_identifier ] { block_item_declaration } { statement_or_null }
end [ : block_identifier ]
block_item_declaration ::= // from A.2.8
{ attribute_instance } data_declaration
| { attribute_instance } local_parameter_declaration ;
| { attribute_instance } parameter_declaration ;
| { attribute_instance } let_declaration
Syntax9-2—Syntax for sequential block (excerpt from AnnexA)
Example 1: A sequential block enables the following two assignments to have a deterministic result:
begin
areg = breg;
creg = areg; // creg stores the value of breg
end
The first assignment is performed, and areg is updated before control passes to the second assignment.
Example 2: An event control (see 9.4.2) can be used in a sequential block to separate the two assignments in
time:
begin
areg = breg;
@(posedge clock) creg = areg; // assignment delayed until
end // posedge on clock
Example 3: The following example shows how the combination of the sequential block and delay control
can be used to specify a time-sequenced waveform:
parameter d = 50; // d declared as a parameter and
logic [7:0] r; // r declared as an 8-bit variable
begin // a waveform controlled by sequential delays
#d r = 'h35;
#d r = 'hE2;
#d r = 'h00;
#d r = 'hF7;
end

<!-- Page 227 -->

IEEE Std #### 9.3.2 Parallel blocks
The fork-join parallel block construct enables the creation of concurrent processes from each of its parallel
statements. A parallel block shall have the following characteristics:
— Statements shall execute concurrently.
— Delay values for each statement shall be considered relative to the simulation time of entering the
block.
— Delay control can be used to provide time-ordering for assignments.
— Control shall pass out of the block when the last time-ordered statement executes based on the type
of join keyword.
— Has restricted usage inside function calls (see 13.4).
Syntax9-3 gives the formal syntax for a parallel block.
par_block ::= // from A.6.3
fork [ : block_identifier ] { block_item_declaration } { statement_or_null }
join_keyword [ : block_identifier ]
join_keyword ::= join | join_any | join_none
block_item_declaration ::= // from A.2.8
{ attribute_instance } data_declaration
| { attribute_instance } local_parameter_declaration ;
| { attribute_instance } parameter_declaration ;
| { attribute_instance } let_declaration
Syntax9-3—Syntax for parallel block (excerpt from AnnexA)
One or more statements can be specified; each statement shall execute as a concurrent process. The timing
controls in a fork-join block do not have to be ordered sequentially in time.
The following example codes the waveform description shown in Example 3 of 9.3.1 by using a parallel
block instead of a sequential block. The waveform produced on the variable is exactly the same for both
implementations.
fork
#50 r = 'h35;
#100 r = 'hE2;
#150 r = 'h00;
#200 r = 'hF7;
join
SystemVerilog provides three choices for specifying when the parent (forking) process resumes execution,
which are summarized in Table9-1.
Table9-1—fork-join control options
Option Description
join The parent process blocks until all the processes spawned by this fork terminate.
join_any The parent process blocks until any one of the processes spawned by this fork terminates.
join_none The parent process continues to execute concurrently with all the processes spawned by the
fork.

<!-- Page 228 -->

IEEE Std In all cases, processes spawned by a fork-join block shall not start executing until the parent process is
blocked or terminates.
When defining a fork-join block, encapsulating the entire fork within a begin-end block causes the entire
block to execute as a single process, with each statement executing sequentially.
fork
begin
statement1; // one process with 2 statements
statement2;
end
join
In the following example, two processes are forked. The first one waits for 20ns and the second one waits
for the named event eventA to be triggered. Because the join keyword is specified, the parent process shall
block until the two processes terminate, i.e., until 20ns have elapsed and eventA has been triggered.
fork
begin
$display( "First Block\n" );
# 20ns;
end
begin
$display( "Second Block\n" );
@eventA;
end
join
A return statement within the context of a fork-join block is illegal and shall result in a compilation error.
For example:
task wait_20;
fork
# 20;
return ; // Illegal: cannot return; task lives in another process
join_none
endtask
Variables declared in the block_item_declaration of a fork-join block shall be initialized to their
initialization value expression whenever execution enters their scope and before any processes are spawned.
Within a fork-join_any or fork-join_none block, it shall be illegal to refer to formal arguments
passed by reference other than in the initialization value expressions of variables declared in a
block_item_declaration of the fork, unless the argument is declared ref static. These variables are useful
in processes spawned by looping constructs to store unique, per-iteration data. For example:
initial
for (int j = 1; j <= 3; ++j)
fork
automatic int k = j; // local copy, k, for each value of j
#k $write("%0d", k);
begin
automatic int m = j; // the value of m is 4
...
end
join_none

<!-- Page 229 -->

IEEE Std The preceding example generates the output 123. k is declared in a block_item_declaration at the beginning
of the fork-join_none block, so each local copy of k is initialized with the current value of j when
execution enters the scope of the block, but before the processes are spawned. In contrast, m is initialized
only after the spawned processes start execution, which is after the parent thread, the for-loop, terminates. At
that time, j has the value 4.
#### 9.3.3 Statement block start and finish times
Both sequential and parallel blocks have the notion of a start and finish time. For sequential blocks, the start
time is when the first statement is executed, and the finish time is when the last statement has been executed.
For parallel blocks, the start time is the same for all the statements, and the finish time is controlled by the
type of join construct used (see 9.3.2, Table9-1).
Sequential and parallel blocks can be embedded within each other, allowing complex control structures to be
expressed easily and with a high degree of structure. When blocks are embedded within each other, the
timing of when a block starts and finishes is important. Execution shall not continue to the statement
following a block until the finish time for the block has been reached, that is, until the block has completely
finished executing.
Example 1: The following example shows the statements from the example in 9.3.2 written in the reverse
order and still producing the same waveform.
fork
#200 r = 'hF7;
#150 r = 'h00;
#100 r = 'hE2;
#50 r = 'h35;
join
Example 2: When an assignment is to be made after two separate events have occurred, known as the joining
of events, a fork-join block can be useful.
begin
fork
@Aevent;
@Bevent;
join
areg = breg;
end
The two events can occur in any order (or even at the same simulation time), the fork-join block will
complete once both events have occurred, and the assignment will be made. In contrast, if the fork-join
block was a begin-end block and the Bevent occurred before the Aevent, then the block would be
waiting for the next Bevent.
Example 3: This example shows two sequential blocks, each of which will execute when its controlling
event occurs. Because the event controls are within a fork-join block, they execute in parallel, and the
sequential blocks can, therefore, also execute in parallel.
fork
@enable_a
begin
#ta wa = 0;
#ta wa = 1;
#ta wa = 0;
end

<!-- Page 230 -->

IEEE Std @enable_b
begin
#tb wb = 1;
#tb wb = 0;
#tb wb = 1;
end
join
#### 9.3.4 Block names
Both sequential and parallel blocks can be named by adding : name_of_block after the keywords begin
or fork. A named block creates a new hierarchy scope. The naming of blocks serves the following
purposes:
— It allows local variables, parameters, and named events to be referenced hierarchically, using the
block name.
— It allows the block to be referenced in statements such as the disable statement (see 9.6.2).
An unnamed block creates a new hierarchy scope only if it directly contains a block item declaration, such
as a variable declaration or a type declaration. This hierarchy scope is unnamed and the items declared in it
cannot be hierarchically referenced (see 6.21).
All variables shall be static; that is, a unique location exists for all variables, and leaving or entering blocks
shall not affect the values stored in them.
The block names give a means of uniquely identifying all variables at any simulation time.
A matching block name may be specified after the block end, join, join_any, or join_none keyword,
preceded by a colon. This can help document which end or join, join_any, or join_none is associated
with which begin or fork when there are nested blocks. A name at the end of the block is not required. It
shall be an error if the name at the end is different from the block name at the beginning.
begin: blockB // block name after the begin or fork
...
end: blockB
Similarly, a matching block name may be specified after the following block end keywords, preceded by a
colon:
— endchecker (see 17.2)
— endclass (see 8.3)
— endclocking (see 14.3)
— endconfig (see 33.4)
— endfunction (see 13.4)
— endgroup (see 19.2)
— endinterface (see 25.3)
— endmodule (see 23.2.1)
— endpackage (see 26.2)
— endprimitive (see 29.3)
— endprogram (see 24.3)
— endproperty (see 16.2)
— endsequence (see 16.8)
— endtask (see 13.3)

<!-- Page 231 -->

IEEE Std A matching block name may also follow the keyword end at the end of a generate block (see 27.3). A name
at the end of the block is not required. It shall be an error if the name at the end is different from the block
name at the beginning.
#### 9.3.5 Statement labels
A label can be specified before any procedural statement (any non-declaration statement that can appear
inside a begin-end block), as in C. A statement label is used to identify a single statement. The label name is
specified before the statement, followed by a colon.
labelA: statement
A begin-end or fork-join block is considered a statement and can have a statement label before the block.
Specifying a statement label before a begin or fork keyword is equivalent to specifying a block name after
the keyword, and a matching block name may be specified after the block end, join, join_any, or
join_none keyword. For example:
labelB: fork // label before the begin or fork
...
join_none : labelB
It shall be illegal to have both a label before a begin or fork and a block name after the begin or fork. A
label cannot appear before the end, join, join_any, or join_none, as these keywords do not form a
statement.
A statement label on a foreach loop, or on a for loop with variables declared as part of the
for_initialization, names the implicit block created by the loop. For other types of statements, a statement
label creates a named begin-end block around the statement and creates a new hierarchy scope.
A label may also be specified before a generate begin-end block (see 27.3).
A label may also be specified before a concurrent assertion (see 16.5).
A statement with a label can be disabled using a disable statement. Disabling a statement shall have the
same behavior as disabling a named block. See 9.6.2 on disable statements and process control.
### 9.4 Procedural timing controls
SystemVerilog has two types of explicit timing control over when procedural statements can occur. The first
type is a delay control, in which an expression specifies the time duration between initially encountering the
statement and when the statement actually executes. The delay expression can be a dynamic function of the
state of the circuit, or it can be a simple number that separates statement executions in time. The delay
control is an important feature when specifying stimulus waveform descriptions. It is described in 9.4.1 and
9.4.5.
The second type of timing control is the event expression, which allows statement execution to be delayed
until the occurrence of some simulation event occurring in a procedure executing concurrently with this
procedure. A simulation event can be a change of value of an expression (an implicit event) or the
occurrence of a named event, clocking block event, or sequence instance match (see 15.5.2, 14.10, and
9.4.2.4, respectively). Most often, an event control is a positive or negative edge on a clock signal. Event
control is discussed in 9.4.2 through 9.4.5.
The procedural statements encountered so far all execute without advancing simulation time. Simulation
time can advance by one of the following three methods:

<!-- Page 232 -->

IEEE Std — A delay control, which is introduced by the symbol #
— An event control, which is introduced by the symbol @
— The wait statement, which operates like a combination of the event control and the while loop
The three procedural timing control methods are discussed in 9.4.1 through 9.4.5. Syntax9-4 shows the
syntax of timing control in procedural statements.
procedural_timing_control_statement ::= // from A.6.5
procedural_timing_control statement_or_null
delay_or_event_control ::=
delay_control
| event_control
| repeat ( expression ) event_control
delay_control ::=
# delay_value
| # ( mintypmax_expression )
event_control ::=
clocking_event
| @ *
| @ ( * )
clocking_event ::=
@ ps_identifier
| @ hierarchical_identifier
| @ ( event_expression )
event_expression36 ::=
[ edge_identifier ] expression [ iff expression ]
| sequence_instance [ iff expression ]
| event_expression or event_expression
| event_expression , event_expression
| ( event_expression )
procedural_timing_control ::=
delay_control
| event_control
| cycle_delay
wait_statement ::=
wait ( expression ) statement_or_null
| wait fork ;
| wait_order ( hierarchical_identifier { , hierarchical_identifier } ) action_block
edge_identifier ::= posedge | negedge | edge // from A.7.2
36) Parentheses are required when an event expression that contains comma-separated event expressions is passed as an
actual argument using positional binding.
Syntax9-4—Delay and event control syntax (excerpt from AnnexA)
The gate and net delays also advance simulation time, as discussed in Clause28.

<!-- Page 233 -->

IEEE Std #### 9.4.1 Delay control
A procedural statement following the delay control shall be delayed in its execution with respect to the
procedural statement preceding the delay control by the specified delay. If the delay expression evaluates to
an unknown or high-impedance value, it shall be interpreted as zero delay. If the delay expression evaluates
to a negative value, it shall be interpreted as a two’s-complement unsigned integer of the same size as a time
variable. Specify parameters are permitted in the delay expression. They can be overridden by SDF
annotation, in which case the expression is reevaluated.
Example 1: The following example delays the execution of the assignment by 10 time units.
#10 rega = regb;
Example 2: The next three examples provide an expression following the number sign (#). Execution of the
assignment is delayed by the amount of simulation time specified by the value of the expression.
#d rega = regb; // d is defined as a parameter
#((d+e)/2) rega = regb; // delay is average of d and e
#regr regr = regr + 1; // delay is the value in regr
#### 9.4.2 Event control
The execution of a procedural statement can be synchronized with a value change of an expression, known
as an implicit event, or with the occurrence of a named event (see 15.5.2), a clocking block event (see 14.10),
or a sequence instance match (see 9.4.2.4).
The synchronization can also be based on the direction of an implicit event’s change, that is, toward the
value 1 (posedge) or toward the value 0 (negedge), in which case it is referred to as an edge event. The
conditions for posedge and negedge events are shown in Table9-2 and can be described as follows:
— A negedge shall be detected on the transition from 1 to x, z, or 0, and from x or z to 0
— A posedge shall be detected on the transition from 0 to x, z, or 1, and from x or z to 1
Table9-2—Detecting posedge and negedge
To
From
0 1 x z
0 No edge posedge posedge posedge
1 negedge No edge negedge negedge
x negedge posedge No edge No edge
z negedge posedge No edge No edge
In addition to posedge and negedge, a third edge event, edge, indicates a change towards either 1 or 0.
More precisely, an edge event can be described as:
— An edge shall be detected whenever negedge or posedge is detected.
A non-edge implicit event shall be detected on any change in the value of the expression. However, an edge
event shall be detected only on the LSB of the expression. A change of value in any operand of the
expression without a change in the result of the expression shall not be detected as an event.

<!-- Page 234 -->

IEEE Std The following example shows illustrations of edge-controlled statements:
@r rega = regb; // controlled by any value change in reg r
@(posedge clock) rega = regb; // controlled by posedge on clock
forever @(negedge clock) rega = regb; // controlled by negedge on clock
forever @(edge clock) rega = regb; // controlled by edge on clock
If the expression denotes a clocking block input or inout, the event control operator uses the synchronous
values, that is, the values sampled by the clocking event (see 14.13).
Event expressions shall return singular values. Aggregate types can be used in an event expression provided
the expression reduces to a singular value. Object (class instance) members or aggregate elements can be
any type as long as the result of the expression is a singular value.
If the event expression is a reference to a simple object handle or chandle variable, an event is created
when a write to that variable is not equal to its previous value.
Non-virtual methods of an object and built-in methods or system functions for an aggregate type are allowed
in event control expressions as long as the type of the return value is singular and the method is defined as a
function, not a task.
Changing the value of object data members, aggregate elements, or the size of a dynamically sized array
referenced by a method or function shall cause the event expression to be reevaluated. An implementation
may cause the event expression to be reevaluated when changing the value or size even if the members are
not referenced by the method or function.
real AOR[]; // dynamic array of reals
byte stream[$]; // queue of bytes
initial wait(AOR.size() > 0)...; // waits for array to be allocated
initial wait($bits(stream) > 60)...; // waits for total number of bits
// in stream greater than 60
Packet p = new; // Packet 1 -- Packet is defined in 8.2
Packet q = new; // Packet 2
initial fork
@(p.status); // Wait for status in Packet 1 to change
@p; // Wait for a change to handle p
# 10 p = q; // triggers @p
// @(p.status) now waits for status in Packet 2 to change,
// if not already different from Packet 1
join
9.4.2.1 Event OR operator
The logical OR of any number of events can be expressed so that the occurrence of any one of the events
triggers the execution of the procedural statement that follows it. The keyword oror a comma character (,)
is used as an event logical OR operator. A combination of these can be used in the same event expression.
Comma-separated sensitivity lists shall be synonymous to or-separated sensitivity lists.
The next two examples show the logical OR of two and three events, respectively:
@(trig or enable) rega = regb; // controlled by trig or enable
@(posedge clk_a or posedge clk_b or trig) rega = regb;
The following examples show the use of the comma (,) as an event logical OR operator:

<!-- Page 235 -->

IEEE Std always @(a, b, c, d, e)
always @(posedge clk, negedge rstn)
always @(a or b, c, d or e)
9.4.2.2 Implicit event_expression list
An incomplete event_expression list of an event control is a common source of bugs in register transfer level
(RTL) simulations. The implicit event_expression, @*, is a convenient shorthand that eliminates these
problems by adding all nets and variables that are read by the statement (which can be a statement group) of
a procedural_timing_ control_statement to the event_expression.
NOTE—The always_comb procedure (see 9.2.2.2) is preferred over using the @* implicit event_expression list when
used at the beginning of an always procedure as a sensitivity list. See 9.2.2.2.2 for a comparison of always_comb and
@*.
All net and variable identifiers that appear in the statement will be automatically added to the event
expression with the following exceptions:
— Identifiers that only appear in timing control (wait, event, or delay) expressions.
— Identifiers that only appear as a hierarchical_variable_identifier in the variable_lvalue of the
left-hand side of assignments.
Nets and variables that appear on the right-hand side of assignments, in subroutine calls, in case and
conditional expressions, as an index variable on the left-hand side of assignments, or as variables in case
item expressions shall all be included by these rules.
Example 1:
always @(*) // equivalent to @(a or b or c or d or f)
y = (a & b) | (c & d) | myfunction(f);
Example 2:
always @* begin // equivalent to @(a or b or c or d or tmp1 or tmp2)
tmp1 = a & b;
tmp2 = c & d;
y = tmp1 | tmp2;
end
Example 3:
always @* begin // equivalent to @(b)
@(i) kid = b; // i is not added to @*
end
Example 4:
always @* begin // equivalent to @(a or b or c or d)
x = a ^ b;
@* // equivalent to @(c or d)
x = c ^ d;
end
Example 5:

<!-- Page 236 -->

IEEE Std always @* begin // same as @(a or en)
y = 8'hff;
y[a] = !en;
end
Example 6:
always @* begin // same as @(state or go or ws)
next = 4'b0;
case (1'b1)
state[IDLE]: if (go) next[READ] = 1'b1;
else next[IDLE] = 1'b1;
state[READ]: next[DLY ] = 1'b1;
state[DLY ]: if (!ws) next[DONE] = 1'b1;
else next[READ] = 1'b1;
state[DONE]: next[IDLE] = 1'b1;
endcase
end
9.4.2.3 Conditional event controls
The @ event control can have an iff qualifier.
module latch (output logic [31:0] y, input [31:0] a, input enable);
always @(a iff enable == 1)
y <= a; //latch is in transparent mode
endmodule
The event expression only triggers if the expression after the iff is true (as defined in 12.4), in this case
when enable is equal to 1. This type of expression is evaluated when a changes and not when enable
changes. Also, in similar event expressions of this type, iff has precedence over or. This can be made
clearer by the use of parentheses.
9.4.2.4 Sequence events
A sequence instance can be used in event expressions to control the execution of procedural statements
based on the successful match of the sequence. This allows the end point of a named sequence (see 16.7) to
trigger multiple actions in other processes. Syntax16-3 and Syntax16-5 describe the syntax for declaring
named sequences and sequence instances. A sequence instance can be used directly in an event expression,
as shown in Syntax9-4.
When a sequence instance is specified in an event expression, the process executing the event control shall
block until the specified sequence reaches its end point. A process resumes execution following the
Observed region in which the end point is detected.
An example of using a sequence as an event control follows:
sequence abc;
@(posedge clk) a ##1 b ##1 c;
endsequence
program test;
initial begin
@ abc $display( "Saw a-b-c" );
L1 : ...
end
endprogram

<!-- Page 237 -->

IEEE Std In the preceding example, when the named sequence abc reaches its end point, the initial procedure in
the program block test is unblocked, then displays the string "Saw a-b-c", and continues execution with
the statement labeled L1. In this case, the end of the sequence acts as the trigger to unblock the event.
A sequence used in an event control is instantiated (as if by an assert property statement); the event control is
used to synchronize to the end of the sequence, regardless of its start time. Arguments to these sequences
shall be static; automatic variables used as sequence arguments shall result in an error.
#### 9.4.3 Level-sensitive event control
The execution of a procedural statement can also be delayed until a condition becomes true. This is
accomplished using the wait statement, which is a special form of event control. The nature of the wait
statement is level-sensitive, as opposed to basic event control (specified by the @ character), which is edge-
sensitive.
The wait statement shall evaluate a condition; and, if it is not true (as defined in 12.4), the procedural
statements following the wait statement shall remain blocked until that condition becomes true before
continuing. The wait statement has the form given in Syntax9-5.
wait_statement ::= // from A.6.5
wait ( expression ) statement_or_null
| wait fork ;
| wait_order ( hierarchical_identifier { , hierarchical_identifier } ) action_block
Syntax9-5—Syntax for wait statement (excerpt from AnnexA)
The following example shows the use of the wait statement to accomplish level-sensitive event control:
begin
wait (!enable) #10 a = b;
#10 c = d;
end
If the value of enable is 1 when the block is entered, the wait statement will delay the evaluation of the next
statement (#10 a = b;) until the value of enable changes to 0. If enable is already 0 when the begin-end
block is entered, then the assignment “a = b;” is evaluated after a delay of 10 and no additional delay
occurs.
See also 9.6 on process control.
#### 9.4.4 Level-sensitive sequence controls
The execution of procedural code can be delayed until a sequence termination status is true. This is
accomplished using the level-sensitive wait statement in conjunction with the built-in method that returns
the current end status of a named sequence: triggered.
The triggered sequence method evaluates to true (1'b1) if the given sequence has reached its end point
(see 16.7) at that particular point in time (in the current time step) and false (1'b0) otherwise. The triggered
status of a sequence is set during the Observed region and persists through the remainder of the time step
(i.e., until simulation time advances).

<!-- Page 238 -->

IEEE Std For example:
sequence abc;
@(posedge clk) a ##1 b ##1 c;
endsequence
sequence de;
@(negedge clk) d ##[2:5] e;
endsequence
program check;
initial begin
wait( abc.triggered || de.triggered );
if( abc.triggered )
$display( "abc succeeded" );
if( de.triggered )
$display( "de succeeded" );
L2 : ...
end
endprogram
In the preceding example, the initial procedure in program check waits for the end point of either
sequence abc or sequence de. When either condition evaluates to true, the wait statement unblocks the
process, displays the sequences that caused the process to unblock, and then continues to execute the
statement labeled L2.
See 16.9.11and 16.13.6 for a definition of sequence methods.
#### 9.4.5 Intra-assignment timing controls
The delay and event control constructs previously described precede a statement and delay its execution. In
contrast, intra-assignment delay and event controls are contained within an assignment statement and
modify the flow of activity in a different way. This subclause describes the purpose of intra-assignment
timing controls and the repeat timing control that can be used in intra-assignment delays.
An intra-assignment delay or event control shall delay the assignment of the new value to the left-hand side,
but the right-hand expression shall be evaluated before the delay, instead of after the delay. The syntax for
intra-assignment delay and event control is given in Syntax9-6.
blocking_assignment ::= // from A.6.2
variable_lvalue = delay_or_event_control expression
| ...
nonblocking_assignment ::=
variable_lvalue <= [ delay_or_event_control ] expression
Syntax9-6—Syntax for intra-assignment delay and event control (excerpt from AnnexA)
The delay_or_event_control syntax is shown in Syntax9-4 in 9.4.
The intra-assignment delay and event control can be applied to both blocking assignments and nonblocking
assignments. The repeat event control shall specify an intra-assignment delay of a specified number of
occurrences of an event. The repeat construct evaluates its expression once before scheduling the
assignment—changing any part of the expression once scheduled has no effect. If the repeat count

<!-- Page 239 -->

IEEE Std expression is less than or equal to zero,unknown, or high impedance at the time of evaluation, the
assignment occurs as if there is no repeat construct.
For example:
repeat (3) @(event_expression)
// will execute @(event_expression) three times
repeat (-3) @(event_expression)
// will not execute @(event_expression)
repeat (a) @(event_expression)
// if a is assigned -3, it will execute @(event_expression)
// if a is declared as an unsigned variable, but not if a is signed
This construct is convenient when events have to be synchronized with counts of clock signals.
Table9-3 illustrates the philosophy of intra-assignment timing controls by showing the code that could
accomplish the same timing effect without using intra-assignment timing.
Table9-3—Intra-assignment timing control equivalence
Intra-assignment timing control
With intra-assignment construct Without intra-assignment construct
begin
a = #5 b; temp = b;
#5 a = temp;
end
begin
a = @(posedge clk) b; temp = b;
@(posedge clk) a = temp;
end
begin
a = repeat(3) @(posedge clk) b; temp = b;
@(posedge clk);
@(posedge clk);
@(posedge clk) a = temp;
end
The next three examples use the fork-join behavioral construct. All statements between the keywords fork
and join execute concurrently. This construct is described in more detail in 9.3.2.
The following example shows a race condition that could be prevented by using intra-assignment timing
control:
fork
#5 a = b;
#5 b = a;
join

<!-- Page 240 -->

IEEE Std The code in this example samples and sets the values of both a and b at the same simulation time, thereby
creating a race condition. The intra-assignment form of timing control used in the next example prevents this
race condition.
fork // data swap
a = #5 b;
b = #5 a;
join
Intra-assignment timing control works because the intra-assignment delay causes the values of a and b to be
evaluated before the delay and causes the assignments to be made after the delay.
Intra-assignment waiting for events is also effective. In the following example, the right-hand expressions
are evaluated when the assignment statements are encountered, but the assignments are delayed until the
rising edge of the clock signal:
fork // data shift
a = @(posedge clk) b;
b = @(posedge clk) c;
join
The following is an example of a repeat event control as the intra-assignment delay of a nonblocking
assignment:
a <= repeat(5) @(posedge clk) data;
Figure9-1 illustrates the activities that result from this repeat event control.
data is evaluated
clk
data
a
Figure9-1—Intra-assignment repeat event control utilizing a clock edge
In this example, the value of data is evaluated when the assignment is encountered. After five occurrences
of posedge clk, a is assigned the value of data.
The following is an example of a repeat event control as the intra-assignment delay of a procedural
assignment:
a = repeat(num) @(clk) data;
In this example, the value of data is evaluated when the assignment is encountered. After the number of
transitions of clk equals the value of num, a is assigned the value of data.

<!-- Page 241 -->

IEEE Std The following is an example of a repeat event control with expressions containing operations to specify both
the number of event occurrences and the event that is counted:
a <= repeat(a+b) @(posedge phi1 or negedge phi2) data;
In this example, the value of data is evaluated when the assignment is encountered. After the sum of the
positive edges of phi1 and the negative edges of phi2 equals the sum of a and b, a is assigned the value of
data. Even if posedge phi1 and negedge phi2 occurred at the same simulation time, each will be
detected and counted separately.
If phi1 and phi2 refer to the same signal, then the preceding assignment can be simplified as:
a <= repeat(a+b) @(edge phi1) data;
### 9.5 Process execution threads
SystemVerilog creates a thread of execution for the following:
— Each initial procedure
— Each final procedure
— Each always, always_comb, always_latch, and always_ff procedure
— Each parallel statement in a fork-join (or join_any or join_none) statement group
— Each dynamic process
Each continuous assignment can also be considered its own thread (see 10.3).
### 9.6 Process control
SystemVerilog provides constructs that allow one process to terminate or wait for the termination of other
processes. The wait fork construct waits for the termination of processes. The disable construct stops
the execution of all activity within a named block or task, without regard to parent-child relationship (a child
process can terminate execution of a parent or one process can terminate execution of an unrelated process).
The disable fork construct stops the execution of processes, but with consideration of parent-child
relationships.
The process control statements have the syntax form shown in Syntax9-7.
wait_statement ::= // from A.6.5
wait ( expression ) statement_or_null
| wait fork ;
| wait_order ( hierarchical_identifier { , hierarchical_identifier } ) action_block
disable_statement ::=
disable hierarchical_task_identifier ;
| disable hierarchical_block_identifier ;
| disable fork ;
Syntax9-7—Syntax for process control statements (excerpt from AnnexA)

<!-- Page 242 -->

IEEE Std #### 9.6.1 Wait fork statement
The wait fork statement blocks process execution flow until all immediate child subprocesses (processes
created by the current process, excluding their descendants) have terminated.
The syntax for wait fork is as follows:
wait fork ; // from A.6.5
Simulation automatically terminates when there is no further activity of any kind. Simulation also
automatically terminates when all its program blocks finish executing (i.e., they reach the end of their
execute block), regardless of the status of any child processes (see 24.7). The wait fork statement allows a
program block to wait for the completion of all its concurrent threads before exiting.
In the following example, two immediate child processes (child1 and child2) are spawned before calling
the task do_test. In the task do_test, three more immediate child processes (child3, child4, and
child5) and two descendant processes (descendant1 and descendant2) are spawned. Next, two more
immediate child processes (child6 and child7) are spawned by the function do_sequence. The wait
fork statement blocks the execution flow of the task do_test until all seven immediate child processes
terminate before returning to its caller. The wait fork statement does not directly depend on the
descendant processes spawned by child5.
initial begin : test
fork
child1();
child2();
join_none
do_test();
end : test
task do_test();
fork
child3();
child4();
fork : child5 // nested fork-join_none is a child process
descendant1();
descendant2();
join_none
join_none
do_sequence();
wait fork; // block until child1 ... child7 terminate
endtask
function void do_sequence();
fork
child6();
child7();
join_none
endfunction
#### 9.6.2 Disable statement
The disable statement provides the ability to terminate the activity associated with concurrently active
processes, while maintaining the structured nature of procedural descriptions. The disable statement gives a
mechanism for terminating a task before it executes all its statements, breaking from a looping statement, or
skipping statements in order to continue with another iteration of a looping statement. It is useful for

<!-- Page 243 -->

IEEE Std handling exception conditions such as hardware interrupts and global resets. The disable statement can also
be used to terminate execution of a labeled statement, including a deferred assertion (see 16.4) or a
procedural concurrent assertion (see 16.14.6).
The disable statement shall terminate the activity of a task or a named block. Execution shall resume at the
statement following the block or following the task-enabling statement. All activities enabled within the
named block or task shall be terminated as well. If task enable statements are nested (that is, one task enables
another, and that one enables yet another), then disabling a task within the chain shall disable all tasks
downward on the chain. If a task is enabled more than once, then disabling such a task shall disable all
activations of the task.
The results of the following activities that can be initiated by a task are not specified if the task is disabled:
— Results of output and inout arguments
— Scheduled, but not executed, nonblocking assignments
— Procedural continuous assignments (assign and force statements)
The disable statement can be used within blocks and tasks to disable the particular block or task containing
the disable statement. The disable statement can be used to disable named blocks within a function, but
cannot be used to disable functions. In cases where a disable statement within a function disables a block or
a task that called the function, the behavior is undefined. Disabling an automatic task or a block inside an
automatic task proceeds as for regular tasks for all concurrent executions of the task.
Example 1: This example illustrates how a block disables itself.
begin : block_name
rega = regb;
disable block_name;
regc = rega; // this assignment will never execute
end
Example 2: This example shows the disable statement being used within a named block in a manner similar
to a forward goto. The next statement executed after the disable statement is the one following the named
block.
begin : block_name
...
...
if (a == 0)
disable block_name;
...
end // end of named block
// continue with code following named block
...
Example 3: This example illustrates using the disable construct to terminate execution of a named block that
does not contain the disable statement. If the block is currently executing, this causes control to jump to the
statement immediately after the block. If the block is a loop body, it acts like a continue (see 12.8). If the
block is not currently executing, the disable has no effect.
module m (...);
always
begin : always1
...
t1: task1( ); // task call
...

<!-- Page 244 -->

IEEE Std end
...
always
begin
...
disable m.always1; // exit always1, which will exit task1,
// if it was currently executing
end
endmodule
Example 4: This example shows the disable statement being used as an early return from a task.
SystemVerilog also has return from a task, which shall terminate execution of the process in which the
return is executed (see 12.8). However, a task disabling itself using a disable statement is not a shorthand for
the return statement. If disable is applied to a task, all currently active executions of the task are disabled.
task proc_a;
begin
...
...
if (a == 0)
disable proc_a; // return if true
...
...
end
endtask
Example 5: This example shows the disable statement being used in an equivalent way to the two statements
continue and break (see 12.8). The example illustrates control code that would allow a named block to
execute until a loop counter reaches n iterations or until the variable a is set to the value of b. The named
block outer_block contains the code that executes until a == b, at which point the disable
outer_block; statement terminates execution of that block. The named block inner_block contains the
code that executes for each iteration of the for loop. Each time this code executes the
disable inner_block; statement, the inner_block block terminates, and execution passes to the next
iteration of the for loop. For each iteration of the inner_block block, a set of statements executes if
(a != 0). Another set of statements executes if (a! = b).
begin : outer_block
for (i = 0; i < n; i = i+1) begin : inner_block
@clk
if (a == 0) // "continue" loop
disable inner_block ;
... // statements
... // statements
@clk
if (a == b) // "break" from loop
disable outer_block;
... // statements
... // statements
end
end
NOTE—The C-like break and continue statements (see 12.8) may be a more intuitive way to code the preceding
example.
Example 6: This example shows the disable statement being used to disable concurrently a sequence of
timing controls and the task named action when the reset event occurs. The example shows a fork-join
block within which are a named sequential block (event_expr) and a disable statement that waits for

<!-- Page 245 -->

IEEE Std occurrence of the event reset. The sequential block and the wait for reset execute in parallel. The
event_expr block waits for one occurrence of event ev1 and three occurrences of event trig. When these
four events have happened, plus a delay of d time units, the task action executes. When the event reset
occurs, regardless of events within the sequential block, the fork-join block terminates—including the task
action.
fork
begin : event_expr
@ev1;
repeat (3) @trig;
#d action (areg, breg);
end
@reset disable event_expr;
join
Example 7: The next example is a behavioral description of a retriggerable monostable. The named event
retrig restarts the monostable time period. If retrig continues to occur within 250 time units, then q
will remain at 1.
always begin : monostable
#250 q = 0;
end
always @retrig begin
disable monostable;
q = 1;
end
#### 9.6.3 Disable fork statement
The disable fork statement terminates all descendant subprocesses (not just immediate children) of the
calling process, including descendants of subprocesses that have already terminated.
The syntax for disable fork is as follows:
disable fork ; // from A.6.5
In the following example, the task get_first spawns three versions of a task that wait for a particular
device (1, 7, or 13). The task wait_device waits for a particular device to become ready and then returns
the device’s address. When the first device becomes available, the get_first task shall resume execution
and proceed to kill the outstanding wait_device processes.
task get_first( output int adr );
fork
wait_device( 1, adr );
wait_device( 7, adr );
wait_device( 13, adr );
join_any
disable fork;
endtask
The disable construct terminates a process when applied to the named block or statement being executed
by the process. The disable fork statement differs from disable in that disable fork considers the
dynamic parent-child relationship of the processes, whereas disable uses the static, syntactical information
of the disabled block. Thus, disable shall end all processes executing a particular block, whether the

<!-- Page 246 -->

IEEE Std processes were forked by the calling thread or not, while disable fork shall end only the processes that
were spawned by the calling thread.
### 9.7 Fine-grain process control
process is a built-in class that allows one process to access and control another process once it has started.
Users can declare variables of type process and safely pass them through tasks or incorporate them into
other objects. The prototype for the process class is as follows:
class :final process;
typedef enum {FINISHED, RUNNING, WAITING, SUSPENDED, KILLED} state;
static function process self();
function state status();
function void kill();
task await();
function void suspend();
function void resume();
function void srandom(int seed);
function string get_randstate();
function void set_randstate(string state);
endclass
Objects of type process are created internally when processes are spawned. Users cannot create objects of
type process; attempts to call new shall not create a new process and shall instead result in an error. The
process class cannot be extended. Attempts to extend it shall result in a compilation error.
The self() function returns a handle to the current process, that is, a handle to the process making the call.
The status() function returns the process status, as defined by the state enumeration:
— FINISHED means the process terminated normally.
— RUNNING means the process is currently running (not in a blocking statement).
— WAITING means the process is waiting in a blocking statement.
— SUSPENDED means the process is stopped awaiting a resume.
— KILLED means the process was forcibly terminated (via kill() or disable (see 9.6.2)).
The kill() function forcibly terminates the given process and all its descendant subprocesses, that is,
processes spawned using fork statements by the process being killed or by its descendants. All such
processes shall be terminated before kill() returns; however, due to nondeterminism (see 4.7), the
processes may terminate normally while kill() is executing. Note that calling kill() on a process in
either the FINISHED or KILLED state will forcibly terminate any descendant subprocesses of that process
that are not already in the FINISHED or KILLED state.
The await() task allows one process to wait for the normalor forceful termination of another process. It
shall be an error to call this task on the current process, i.e., a process cannot wait for its own termination.
The suspend() function allows a process to suspend either its own execution or that of another process.
The process shall be suspended before suspend() returns; however, due to nondeterminism (see 4.7), the
processes may terminate normally or be forcibly terminated while suspend() is executing. Suspending a
process in either the SUSPENDED, FINISHED or KILLED state shall have no effect. Suspending a process in
the WAITING state shall cause the process to be desensitized to the event expression, wait condition, or delay
expiration on which it is blocked. It shall be an error for a function to call suspend() on the current
process, i.e., a function cannot suspend its own execution.

<!-- Page 247 -->

IEEE Std The resume() function restarts a previously suspended process. Calling resume() on a process that was
suspended while in the RUNNING state shall schedule the process into the Active or Reactive region to
continue its execution in the current time step. Calling resume() on a process that was suspended while in
the WAITING state shall resensitize the process to the event expression or to wait for the wait condition to
become true or for the delay to expire. If the wait condition is now true or the original delay has transpired,
the process is scheduled into the Active or Reactive region to continue its execution in the current time step.
Calling resume() on a process that is not in the SUSPENDED state shall have no effect.
The methods kill(), await(), suspend(), and resume() shall be restricted to a process created by an
initial procedure, always procedure, or fork block from one of those procedures.
The methods srandom(), get_randstate(), and set_randstate() are described in 18.13.3, 18.13.4,
and 18.13.5, respectively.
The following example starts an arbitrary number of processes, as specified by the task argument N. Next,
the task waits for all processes to start executing and then waits for the first process to terminate. At that
point, the parent process forcibly terminates all forked processes that have not yet terminated normally.
task automatic do_n_way(int N);
process job[] = new [N];
foreach (job[j])
fork
automatic int k = j;
begin
job[k] = process::self();
... ;
end
join_none
foreach (job[j]) // wait for all processes to start
wait(job[j] != null);
job[0].await(); // wait for first process to finish
foreach (job[j]) begin
if (job[j].status != process::FINISHED)
job[j].kill();
end
endtask

<!-- Page 248 -->

IEEE Std 1800-2023