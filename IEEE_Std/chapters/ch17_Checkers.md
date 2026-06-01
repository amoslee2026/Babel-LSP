# Chapter 17: Checkers

## 17. Checkers
### 17.1 Overview
Assertions provide building blocks to validate the behavior of the design. In many cases there is a need to
group several assertions together into bigger blocks having a well-defined functionality. These verification
blocks may also need to contain modeling code to compute values of auxiliary variables used in assertions or
covergroup instances to be integrated with cover statements. The checker construct in SystemVerilog was
specifically created to represent such verification blocks encapsulating assertions along with the modeling
code. The intended use of checkers is to serve as verification library units, or as building blocks for creating
abstract auxiliary models used in formal verification.
The modeling mechanism in checkers is similar to the modeling mechanism in modules and interfaces,
though several limitations apply. For example, no nets can be declared and assigned in checkers. On the
other hand, checkers allow nondeterministic modeling, which does not exist in modules and interfaces. Each
variable declared in a checker may be either deterministic or random. Checker modeling is explained in
17.7. Random variables are useful to build abstract nondeterministic models for formal verification.
Reasoning about nondeterministic models is sometimes much easier than reasoning about deterministic RTL
models.
Deterministic variables allow a conventional (deterministic) modeling for assertions. Using random
variables instead of regular variables in checkers has the advantage that the same checker may be used for
both deterministic and nondeterministic cases.
### 17.2 Checker declaration
checker_declaration ::= // from A.1.2
checker checker_identifier [ ( [ checker_port_list ] ) ] ;
{ { attribute_instance } checker_or_generate_item }
endchecker [ : checker_identifier ]
checker_port_list ::= checker_port_item { , checker_port_item } // from A.1.8
checker_port_item ::=
{ attribute_instance } [ checker_port_direction ] property_formal_type formal_port_identifier
{ variable_dimension } [ = property_actual_arg ]
checker_port_direction ::= input | output
checker_or_generate_item ::=
checker_or_generate_item_declaration
| initial_construct
| always_construct
| final_construct
| assertion_item
| continuous_assign
| checker_generate_item
checker_or_generate_item_declaration ::=
[ rand ] data_declaration
| function_declaration
| checker_declaration
| assertion_item_declaration
| covergroup_declaration
| genvar_declaration

<!-- Page 505 -->

IEEE Std | clocking_declaration
| default clocking clocking_identifier ;
| default disable iff expression_or_dist ;
| ;
checker_generate_item6 ::=
loop_generate_construct
| conditional_generate_construct
| generate_region
| elaboration_severity_system_task
checker_identifier ::=identifier // from A.9.3
6) It shall be illegal for a checker_generate_item to include any item that would be illegal in a checker_declaration
outside a checker_generate_item.
Syntax17-1—Checker declaration syntax (excerpt from AnnexA)
A checker may be declared in one of the following:
— A module
— An interface
— A program
— A checker
— A package
— A generate block
— A compilation-unit scope
A checker is declared using the keyword checker followed by a name and optional formal argument list,
and ending with the keyword endchecker.
The following elements from the scope enclosing the checker declaration shall not be referenced in a
checker:
— Automatic variables and members or elements of dynamic variables (see 6.21).
— Elements of fork-join, fork-join_any, or fork-join_none blocks.
Action blocks of assertions within a checker will be referred to as checker action blocks, and the rest of the
checker will be referred to as a checker body.
A checker body may contain the following elements:
— Declarations of let constructs, sequences, properties, and functions
— Deferred assertions (see 16.4)
— Concurrent assertions (see 16.14)
— Checker declarations
— Other checker instantiations
— Covergroup declarations and instances
— Checker variable declarations and assignments (see 17.7)
— default clocking and default disable iff declarations
— initial, always_comb, always_latch, always_ff, and final procedures (see 9.2)
— Generate blocks, containing any of the above elements

<!-- Page 506 -->

IEEE Std Modules, interfaces, programs, and packages shall not be declared inside checkers. Modules, interfaces, and
programs shall not be instantiated inside checkers.
A formal argument of a checker may be optionally preceded by a direction qualifier: input or output. If
no direction is specified explicitly then the direction of the previous argument shall be inferred. If the
direction of the first checker argument is omitted, it shall default to input. An input checker formal
argument shall not be modified by a checker.
The legal data types for checker formal arguments are those legal for a property (see 16.12). The type of an
output argument shall not be of untyped, sequence, or property. If the type of a checker formal
argument is omitted, it is inferred according to the following rules:
— If the argument has an explicit direction qualifier, it shall be an error to omit its type.
— Otherwise, if the argument is the first argument of the checker, it is assumed to be input untyped.
— Otherwise, the type of the previous formal argument is inferred as described for sequences and
properties (see 16.8 and 16.12).
In a similar manner to sequences and properties, a checker declaration may specify a default value for each
singular input port, as described in 16.8. A checker declaration may also specify an initial value for each
singular output port using the same syntax as the default value specification for input arguments. Checker
output port initialization has the same semantics as a variable initialization (see 6.8).
Following are examples of simple checkers:
Example 1:
// Simple checker containing concurrent assertions
checker my_check1 (logic test_sig, event clock);
default clocking @clock; endclocking
property p(logic sig);
...
endproperty
a1: assert property (p (test_sig));
c1: cover property (!test_sig ##1 test_sig);
endchecker : my_check1
Example 2:
// Simple checker containing deferred assertions
checker my_check2 (logic a, b);
a1: assert #0 ($onehot0({a, b});
c1: cover #0 (a == 0 && b == 0);
c2: cover #0 (a == 1);
c3: cover #0 (b == 1);
endchecker : my_check2
Example 3:
// Simple checker with output arguments
checker my_check3 (logic a, b, event clock, output bit failure, undef);
default clocking @clock; endclocking
a1: assert property ($onehot0({a, b}) failure = 1'b0; else failure = 1'b1;
a2: assert property ($isunknown({a, b}) undef = 1'b0; else undef = 1'b1;
endchecker : my_check3

<!-- Page 507 -->

IEEE Std Example 4:
// Checker with default input and initialized output arguments
checker my_check4 (input logic in,
en = 1'b1, // default value
event clock,
output int ctr = 0); // initial value
default clocking @clock; endclocking
always_ff @clock
if (en && in) ctr <= ctr + 1;
a1: assert property (ctr < 5);
endchecker : my_check4
Type and data declarations within the checker are local to the checker scope and are static. Clock and
disableiff contexts are inherited from the scope of the checker declaration (but see 17.4 for usage of
context value functions for passing the instantiation context to the checker). For example:
module m;
default clocking @clk1; endclocking
default disable iff rst1;
checker c1;
// Inherits @clk1 and rst1
...
endchecker : c1
checker c2;
// Explicitly redefines its default values
default clocking @clk2; endclocking
default disable iff rst2;
...
endchecker : c2
...
endmodule : m
Variables used in a checker that are neither formal arguments to the checker nor internal variables of the
checker are resolved according to the scoping rules from the scope in which the checker is declared.
### 17.3 Checker instantiation
concurrent_assertion_item ::= // from A.2.10
...
| checker_instantiation
checker_instantiation ::= // from A.4.1.4
ps_checker_identifier name_of_instance ( [ list_of_checker_port_connections ] ) ;
list_of_checker_port_connections34 ::=
ordered_checker_port_connection { , ordered_checker_port_connection }
| named_checker_port_connection { , named_checker_port_connection }
ordered_checker_port_connection ::= { attribute_instance } [ property_actual_arg ]
named_checker_port_connection ::=
{ attribute_instance } . formal_port_identifier [ ( [ property_actual_arg ] ) ]
| { attribute_instance } . *

<!-- Page 508 -->

IEEE Std ps_checker_identifier ::= [ package_scope ] checker_identifier // from A.9.3
34) The . * token pair shall appear at most once in a list of port connections.
Syntax17-2—Checker instantiation syntax (excerpt from AnnexA)
A checker may be instantiated wherever a concurrent assertion may appear (see 16.14) with the following
exceptions:
— It shall be illegal to instantiate checkers in fork-join, fork-join_any, or fork-join_none
blocks.
— It shall be illegal to instantiate a checker in a procedure of another checker.
A checker has different behavior depending on whether it is instantiated inside or outside procedural code. A
checker instantiation in procedural code is referred to as a procedural checker instance. A checker
instantiation outside procedural code is referred to as a static checker instance. See 16.14.6 for the
corresponding definitions of procedural and static assertion statements.
When a checker is instantiated, actual arguments are passed to the checker. The mechanism for passing input
arguments to a checker is similar to the mechanism for passing arguments to a property (see 16.12). The
rewriting algorithm for checkers (see F.4.2) applies to checker input arguments. The rewriting algorithm
substitutes actual arguments for references to the corresponding formal arguments in the body of the
declaration of the checker. The rewriting algorithm does not itself account for name resolution and assumes
that names have been resolved prior to the substitution of actual arguments. If the flattened checker is not
legal, then the instance is not legal and there shall be an error.
The following restrictions apply:
— As in the case of sequences and properties, the terminal $ may be an actual argument to an instance
of a checker, either declared as a default actual argument or passed in the list of arguments of the
instance. If $ is an actual input argument to a checker instance, then the corresponding formal
argument shall be untyped and each of its references either shall be an upper bound in a
cycle_delay_const_range_expression or shall itself be an actual argument in an instance of a named
sequence or property, in a checker instance, or as a default argument to a nested checker.
— If an actual input argument contains any subexpression that is a const cast or automatic value from
procedural code, then the corresponding formal argument shall not be used in a continuous
assignment or in the procedural code within the checker.
Each checker actual output argument shall bea variable_lvalue oran net_lvalue. The checker instantiation
should be treated as if there were continuous assignments, executed in the Reactive region, of the checker’s
output formal arguments to their corresponding actual arguments.
Checker formal arguments may be connected to their actual arguments in ways similar to module ports (see
23.3.2):
— Positional connections by port order.
— Named port connections using fully explicit connections.
— Named port connections using implicit connections.
— Named port connections using a wildcard port name.
The following example illustrates a checker instantiation:
checker mutex (logic [31:0] sig, event clock, output bit failure);
assert property (@clock $onehot0(sig))

<!-- Page 509 -->

IEEE Std failure = 1'b0; else failure = 1'b1;
endchecker : mutex
module m(wire [31:0] bus, logic clk);
logic res, scan;
// ...
mutex check_bus(bus, posedge clk, res);
always @(posedge clk) scan <= res;
endmodule : m
On each rising edge of clk the bits of bus are checked for mutual exclusion and the result is assigned to res
in the Reactive region. If clk is changed in the Active region, scan will capture the value of res generated
on the previous rising edge of clk.
All contents of a checker instance other than static assertion statements are considered to exist during every
time step, regardless of whether the checker is static or procedural. One copy of these contents exists for
each instantiation. Procedural concurrent assertion statements in a checker shall be treated just like other
procedural assertion statements as described in 16.14.6. However, static assertion statements within a
checker are treated as if they appear at the checker’s instantiation point. If the checker is instantiated inside
some scope, any of its static assertions, both concurrent and deferred, are treated as if instantiated in this
scope. Therefore, the following applies for static assertions within a checker:
— If the checker is static, the concurrent assertions are continually monitored, and begin execution on
any time step matching their initial clock event. The deferred assertions are monitored whenever
their expressions change.
— If the checker is procedural, all static concurrent assertions in the checker are added to the pending
procedural assertion queue when the checker instantiation is reached in process execution, and then
may mature or be flushed like any procedural concurrent assertion (see 16.14.6.2). Similarly all
static deferred assertions in the checker are added to the pending deferred assertion report when the
checker instantiation is reached in its process execution, and may mature or be flushed like any
procedural deferred assertion (see 16.4.1).
— If the checker is statically instantiated inside another checker, any of its static assertions, concurrent
or deferred, are treated as if instantiated in the parent checker, and thus will be treated as procedural
assertions when an instantiation of its top-level ancestor in the checker hierarchy is visited in
procedural code.
The following example illustrates this behavior:
checker c1(event clk, logic[7:0] a, b);
logic [7:0] sum;
always_ff @(clk) begin
sum <= a + 1'b1;
p0: assert property (sum < `MAX_SUM);
end
p1: assert property (@clk sum < `MAX_SUM);
p2: assert property (@clk a != b);
p3: assert #0 ($onehot(a));
endchecker
module m(input logic rst, clk, logic en, logic[7:0] in1, in2,
in_array [20:0]);
c1 check_outside(posedge clk, in1, in2);
always @(posedge clk) begin
automatic logic [7:0] v1=0;
if (en) begin
// v1 is automatic, so current procedural value is used

<!-- Page 510 -->

IEEE Std c1 check_inside(posedge clk, in1, v1);
end
for (int i = 0; i < 4; i++) begin
v1 = v1+5;
if (i != 2) begin
// v1 is automatic, so current procedural value is used
c1 check_loop(posedge clk, in1, in_array[v1]);
end
end
end
endmodule : m
In this example, there are three instantiations of c1: check_outside, check_inside, and check_loop.
They have the following characteristics:
— check_outside is a static instantiation, while check_inside and check_loop are procedural.
— Each of the three instantiations has its own version of sum, which is updated at every positive clock
edge, regardless of whether that instance was visited in procedural code. Even in the case of
check_loop, there is only one instance of sum, and it will be updated using the sampled value of
in1.
— Each of the three instantiations will queue an evaluation of p0 at every posedge of the clock
(according to the rules in 16.14.6), which will mature and report a violation during any time step
when sum is not less than MAX_SUM, regardless of the behavior of the procedural code in module m.
— For checker instance check_outside, p1 and p2 are checked at every positive clock edge. For
checker instance check_inside, p1 and p2 are queued to mature and be checked on any positive
clock edge when en is true. For check_loop, three procedural instances of p1 and p2 are queued to
mature on any positive clock edge. For p1, all three instances are identical, using the sampled value
of sum; but for p2, the three instances compare the sampled value of in1 to the sampled value of
in_array indexed by constant v1 values of 5, 10, and 20, respectively.
— For checker instance check_outside, p3 is checked whenever a changes. In checker instances
check_inside and check_loop, deferred assertion p3 behaves as a procedural deferred assertion
placed at the instantiation point of its checker.
### 17.4 Context inference
Context value functions (see 16.14.7) may be used as default values of formal arguments in a checker
declaration. These functions enable adjusting the checker behavior depending on its instantiation context.
For example:
// Context inference in a checker
checker check_in_context (logic test_sig,
event clock = $inferred_clock,
logic reset = $inferred_disable);
property p(logic sig);
...
endproperty
a1: assert property (@clock disable iff (reset) p(test_sig));
c1: cover property (@clock !reset throughout !test_sig ##1 test_sig);
endchecker : check_in_context
module m(logic rst);
wire clk;
logic a, en;
wire b = a && en;
// No context inference

<!-- Page 511 -->

IEEE Std check_in_context my_check1(.test_sig(b), .clock(clk), .reset(rst));
always @(posedge clk) begin
a <= ...;
if (en) begin
...
// inferred from context:
// .clock(posedge clk)
// .reset(1'b0)
check_in_context my_check2(a);
end
en <= ...;
end
endmodule : m
In the preceding example the default values of clock and reset in check_in_context are taken from the
instantiation context. In the instantiation my_check1 all formal arguments are provided explicitly. In the
instantiation my_check2 all optional arguments are passed their default value: the clock is inferred from the
clock of the always procedure of the module m, the disable condition is inferred to be 1'b0.
### 17.5 Checker procedures
The following procedures are allowed inside a checker body:
— initial procedure
— always procedure
— final procedure
An initial procedure in a checker body may contain let declarations, immediate, deferred, and
concurrent assertions, and a procedural timing control statement using an event control only.
The following forms of always procedures are allowed in checkers: always_comb, always_latch, and
always_ff. Checker always procedures may contain the following statements:
— Blocking assignments (see 10.4.1; always_comb and always_latch procedures only)
— Nonblocking assignments (see 10.4.2)
— Selection statements (see 12.4 and 12.5)
— Loop statements (see 12.7)
— Timing event control (see 9.4.2; always_ff procedures only)
— Subroutine calls (see Clause13)
— Immediate, deferred, and concurrent assertions
— let declarations
Except for the variables used in the event control, all other expressions in always_ff procedures are
sampled (see 16.5.1). It follows from this rule that the expressions in immediate and deferred assertions
instantiated in this procedure are also sampled. Expressions in always_comb and always_latch
procedures are not implicitly sampled and the assignments appearing in these procedures use the current
values of their expressions. For example:
checker check(logic a, b, c, clk, rst);
logic x, y, z, v, t;
assign x = a; // current value of a
always_ff @(posedge clk or negedge rst) // current values of clk and rst

<!-- Page 512 -->

IEEE Std begin
a1: assert (b); // sampled value of b
if (rst) // current value of rst
z <= b; // sampled value of b
else z <= !c; // sampled value of c
end
always_comb begin
a2: assert (b); // current value of b
if (a) // current value of a
v = b; // current value of b
else v = !b; // current value of b
end
always_latch begin
a3: assert (b); // current value of b
if (clk) // current value of clk
t <= b; // current value of b
end
// ...
endchecker : check
The following example illustrates clock inference for checker procedures, following the rules in 16.14.6.
checker clocking_example (logic sig1, sig2, default_clk, rst,
event e1, e2, e3 );
bit local_sig;
default clocking @(posedge default_clk); endclocking
always_ff @(e1) begin: p1_block
p1a: assert property (sig1 == sig2);
p1b: assert property (@(e1) (sig1 == sig2));
end
always_ff @(e2 or e3) begin: p2_block
local_sig <= rst;
p2a: assert property (sig1 == sig2);
p2b: assert property (@(e2) (sig1 == sig2));
end
always_ff @(rst or e3) begin: p3_block
local_sig <= rst;
p3a: assert property (sig1 == sig2);
p3b: assert property (@(e3) (sig1 == sig2));
end
...
endchecker
...
clocking_example c1 (s1, s2, default_clk, rst,
posedge clk1 or posedge clk2,
posedge clk1,
negedge rst);
In instance c1 of clocking_example, the assertions will be clocked as follows:
— Assertion p1a will be clocked by posedge default_clk. This is because after the substitution of
the actual argument posedge clk1 or posedge clk2 for the formal argument e1, it does not

<!-- Page 513 -->

IEEE Std satisfy the clock inference conditions in 16.14.6, particularly condition (b). If clocking based on e1
is desired, it hasto be done explicitly, as in property p1b.
— Assertion p2a will be clocked by posedge clk1. This is because the event control of p2_block
satisfies the conditions in 16.14.6, including condition (c 2), after the formal arguments are
substituted with the actual arguments. Thus assertions p2a and p2b are equivalent.
— Assertion p3a will be clocked by posedge default_clk. This is because the event control of
p3_block does not satisfy the conditions in 16.14.6, particularly condition (c 2). If clocking based
on e3 is desired, it has to be done explicitly, as in property p3b.
A final procedure may be specified within a checker in the same manner as in a module (see 9.2.3). This
allows for the checker to check conditions with immediate assertions or print out statistics at the end of
simulation. The operation of the final procedure is independent of the instantiation context of the checker
that contains it. It will be executed once at the end of simulation for every instantiation of that checker.
There is no implied ordering in the execution of multiple final procedures. A final procedure within a
checker may include any construct allowed in a non-checker final procedure.
### 17.6 Covergroups in checkers
One or more covergroup declarations or instances (see 19.3) are permitted within a checker. These
declarations and instances shall not appear in any procedural block in the checker. A covergroup may
reference any variable visible in its scope, including checker formal arguments and checker variables.
However, it shall be an error if a formal argument referenced by a covergroup has a const actual
argument. For example:
checker my_check(logic clk, active);
bit active_d1 = 1'b0;
always_ff @(posedge clk) begin
active_d1 <= active;
end
covergroup cg_active @(posedge clk);
cp_active : coverpoint active
{
bins idle = { 1'b0 };
bins active = { 1'b1 };
}
cp_active_d1 : coverpoint active_d1
{
bins idle = { 1'b0 };
bins active = { 1'b1 };
}
option.per_instance = 1;
endgroup
cg_active cg_active_1 = new();
endchecker : my_check
A covergroup may also be triggered by a procedural call to its sample() method (see 19.8). The following
examples show how the sample() method may be called from a sequence match item to trigger a
covergroup.
checker op_test (logic clk, vld_1, vld_2, logic [3:0] opcode);
bit [3:0] opcode_d1;
always_ff @(posedge clk) opcode_d1 <= opcode;

<!-- Page 514 -->

IEEE Std covergroup cg_op;
cp_op : coverpoint opcode_d1;
endgroup: cg_op
cg_op cg_op_1 = new();
sequence op_accept;
@(posedge clk) vld_1 ##1 (vld_2, cg_op_1.sample());
endsequence
cover property (op_accept);
endchecker
In this example, the coverpoint cp_op refers to the checker variable opcode_d1 directly. It is triggered by a
call to the default sample() method from a sequence match item. This function call occurs in the Reactive
region, while nonblocking assignments to checker variables will occur in the Re-NBA region. As a result,
the covergroup will sample the old value of the checker variable opcode_d1.
It is also possible to define a custom sample() method for a covergroup (see 19.8.1). The following is an
example of this:
checker op_test (logic clk, vld_1, vld_2, logic [3:0] opcode);
bit [3:0] opcode_d1;
always_ff @(posedge clk) opcode_d1 <= opcode;
covergroup cg_op with function sample(bit [3:0] opcode_d1);
cp_op : coverpoint opcode_d1;
endgroup: cg_op
cg_op cg_op_1 = new();
sequence op_accept;
@(posedge clk) vld_1 ##1 (vld_2, cg_op_1.sample(opcode_d1));
endsequence
cover property (op_accept);
endchecker
In this example, a custom sample() method has been defined for the covergroup cg_op, and the coverpoint
cp_op references the formal argument of the custom sample() method. This custom method will be called
in the Reactive region upon a sequence match, but the sampled value of the sequential checker variable
opcode_d1 will be passed to the sample() function. As a result, the covergroup will sample the value
from the Preponed region.
### 17.7 Checker variables
Variables may be defined in checkers, but defining nets in the checker body shall be illegal. All variables
defined in a checker body shall have static lifetimes (see 17.2). The variables defined in the checker body are
referred to as checker variables. The following example illustrates checker variable usage:
checker counter_model(logic flag);
bit [2:0] counter = '0;
always_ff @$global_clock
counter <= counter + 1'b1;
assert property (@$global_clock counter == 0 |-> flag);
endchecker : counter_model
Checker variables may have an optional rand qualifier. In this case, they are called free variables; free
variables may behave nondeterministically.

<!-- Page 515 -->

IEEE Std Formal analysis tools shall take into account all possible values of the free checker variables imposed by the
assumptions and assignments (see 17.7.1). Simulators shall assign random values to the free variables as
explained in 17.7.2.
The following example shows how free variables can be used for modeling for formal verification:
checker observer_model(bit valid, reset);
default clocking @$global_clock; endclocking
rand bit flag;
m1: assume property (reset |=> !flag);
m2: assume property (!reset && flag |=> flag);
m3: assume property ($rising_gclk(flag) |-> valid);
endchecker : observer_model
In this example, the following constraints are imposed on the free variable flag:
— If it is high, it remains high as long as there is no reset.
— If there is a reset, it becomes low at the next tick of the clock.
— It may rise only when valid is high.
Although the behavior of the free variable flag has been restricted by the assumptions m1, m2, and m3, it is
still nondeterministic because it does not have to rise when valid is high. Figure17-1 shows two possible
legal behaviors of this variable given the same behaviors of reset and valid. Formal analysis tools shall
take all possible legal behaviors of flag into account. Simulators shall assign random values to the variable
flag as explained in 17.7.2.
Figure17-1—Nondeterministic free checker variable
The following example shows how free variables may be used to implement a nondeterministic choice:
// a may assume values 3 and 5 only
rand bit r;
let a = r ? 3'd3 : 3'd5;
A free variable declaration may have a const qualifier. If a constant free variable is initialized, it retains its
initial value forever. An uninitialized constant free variable has a nondeterministic value at the initialization,
and this value does not change. The following examples demonstrate the usage of constant free checker
variables.
Formal analysis tools shall take into account any possible values of a constant free checker variable
consistent with the imposed assumptions. Simulators shall assign a random constant value to a constant free
variable as explained in 17.7.2.

<!-- Page 516 -->

IEEE Std Examples:
Reasoning about a representative bit:
checker reason_about_one_bit(bit [63:0] data1, bit [63:0] data2,
event clock);
rand const bit [5:0] idx;
a1: assert property (@clock data1[idx] == data2[idx]);
endchecker : reason_about_one_bit
In this example the assertion a1 states that any fixed bit of data1 has the same value as the corresponding
bit of data2. Therefore, the checker reason_about_one_bit is equivalent in formal verification to the
following checker (these two checkers are not equivalent in simulation):
checker reason_about_all_bits(bit [63:0] data1, bit [63:0] data2,
event clock);
a1: assert property (@clock data1 == data2);
endchecker : reason_about_all_bits
The second realization of the checker compares two 64-bit values while the first one compares only 1-bit
values, for every possible index. The first version may be more efficient for some formal tools.
Data integrity checking:
// If start_ev is asserted then the value of out_data at the next assertion
// of end_ev has to be equal to the current value of in_data at start_ev.
//
// It is assumed that in_data and out_data have the same size
checker data_legal(start_ev, end_ev, in_data, out_data);
rand const bit [$bits(in_data)-1:0] mem_data;
sequence transaction;
start_ev && (in_data == mem_data) ##1 end_ev[->1];
endsequence
a1: assert property (@clock transaction |-> out_data == mem_data);
endchecker : data_legal
Since mem_data is a constant free variable, if in_data is equal to mem_data at the beginning of the
transaction, then mem_data records that value and keeps it throughout the trace. In particular, at the end of
the transaction, mem_data still holds that value and the assertion checks that it is equal to out_data.
Moreover, mem_data was initialized with a nondeterministic value; it follows that for every value of
in_data, there exists a computation in which mem_data is equal to that value of in_data, which in turn
implies that the corresponding legality of data transfer through that transaction is being checked for formal
verification. In simulation mem_data will be randomly initialized (see 17.7.2), and it will only be checked
that if at the transaction beginning in_data equals to mem_data then at the transaction end out_data will
have the same value as in_data at the beginning of the transaction.
The latter example may be rewritten for formal verification using local variables instead of constant free
variables (see 16.10; these implementations are not equivalent in simulation):
// If start_ev is asserted then the value of in_data has to be
// equal to the value of out_data at the next assertion of end_ev
//
// It is assumed that in_data and out_data have the same size
checker data_legal_with_loc(start_ev, end_ev, in_data, out_data);
sequence transaction (loc_var);
(start_ev, loc_var = in_data) ##1 end_ev[->1];

<!-- Page 517 -->

IEEE Std endsequence
property data_legal;
bit [$bits(in_data)-1:0] mem_data;
transaction(mem_data) |-> out_data == mem_data;
endproperty
a1: assert property (@clock data_legal);
endchecker : data_legal_with_loc
There is a difference between a constant and a non-constant free variable: a constant free variable does not
change its value, while a non-constant free variable can assume a new value any time. If a non-constant free
variable has been initialized but is never assigned then it can assume any value at any time step in formal
verification, or be randomized in subsequent time steps in simulation (see 17.7.2), except the first one where
its value is defined by the initialization. Consider the following declaration:
rand bit a = 1'b0, b;
The free variable a has initial value 0, but in other time steps its value may change. The free checker variable
b may assume any value 0 or 1 at any time (in formal verification or randomized in simulation), as opposed
to an uninitialized constant free checker variable, which keeps one specific value.
#### 17.7.1 Checker variable assignments
Checker variables may be assigned using blocking and nonblocking procedural assignments, or
non-procedural continuous assignments.
The following rules and restrictions apply:
— In always_ff procedures, only nonblocking assignments are allowed.
— Referencing a checker variable using its hierarchical name in assignments (see 23.6) shall be illegal.
For example:
checker check(...);
bit a;
...
endchecker
module m(...);
...
check my_check(...);
...
wire x = my_check.a; // Illegal
bit y;
...
always @(posedge clk) begin
my_check.a = y; // Illegal
...
end
...
endmodule
— Continuous assignments and blocking procedural assignments to free checker variables shall be
illegal.
checker check1(bit a, b, event clk, ...);
rand bit x, y, z, v;
...

<!-- Page 518 -->

IEEE Std assign x = a & b; // Illegal
always_comb
y = a & b; // Illegal
always_ff @clk
z <= a & b; // OK
endchecker : check1
— A checker variable may not be assigned in an initial procedure, but may be initialized in its
declaration. For example:
bit v;
initial v = 1'b0; // Illegal
bit w = 1'b0; // OK
— The right-hand side of a checker variable assignment may contain the sequence method triggered
(see 16.13.6).
— The left-hand side of a nonblocking assignment may contain a free checker variable. The following
example illustrates usage of free variable assignments.
// Toggling variable:
// a may have either 0101... or 1010... pattern
rand bit a;
always_ff @clk a <= !a;
#### 17.7.2 Checker variable randomization with assumptions
Checker assume statements are used to describe assumptions that may be made about the values of
variables. They may be used by simulators to constrain the random generation of free checker variable
values or by formal tools to constrain the formal computation. As with normal assume statements, checker
assume statements shall also be checked for violation during simulation.
Assume-based checker variable randomization is the process of periodically solving a set of properties
appearing in assume statements (called an assume set) to find satisfying values for the free checker
variables, and updating those variables with the newfound values. Unlike class-based constrained random
generation, solving is triggered by any of the clock events of the properties in the assume set (called an
assume set clock event) rather than by an explicit procedural call [e.g., there is no randomize() for
checkers]. Once updated with solution values, free checker variables shall remain constant until the next
assume set clock event or the end of the time step, whichever comes first.
All non-const free checker variables are treated as either active or inactive for assume-based
randomization, in the same way as rand variables for class-based constrained random generation (see 17.9),
but without an explicit control facility [such as rand_mode()]. All other variables (such as non-free
checker variables and checker formals) are always treated as inactive. Any free checker variables that appear
on the left-hand side of a checker variable assignment (see 17.7.1) are inactive; all other free checker
variables are active. Free checker variables are active or inactive for each singular element of the variable.
For example, a packed array or structure is active or inactive monolithically, whereas the elements of an
unpacked array or structure are separately active or inactive.
All free checker variables, both const and non-const, active and inactive, are initialized with
unconstrained random values unless explicitly initialized in their declaration.
Each checker instance has one and only one assume set, which may be empty. Like checker procedures and
variables, checker assume sets are considered to exist at every time step, regardless of whether the checker
instance is static or procedural (see 17.3).

<!-- Page 519 -->

IEEE Std The assume set of a checker instance is formed from the checker assume statements and child checker
assume statements. Any of these assume statements that references a formal whose actual argument
contains any subexpression that is a const cast or automatic value (see 17.3) is excluded from the assume
set. This restriction allows a single copy of the assume set to exist for each instantiation that is valid for the
entire simulation, as described in 17.3. Among the remaining assume statements, those that reference active
free variables of the checker are included in the assume set. For example:
module my_mod();
bit mclk, v1, v2;
checker c1(bit fclk, bit a, bit b);
default clocking @ (posedge fclk); endclocking
checker c2(bit bclk, bit x, bit y);
default clocking @ (posedge bclk); endclocking
rand bit m, n;
u1: assume property (f1(x,m));
u2: assume property (f2(y,n));
endchecker
rand bit q, r;
c2 B1(fclk, q+r, r);
always_ff @ (posedge fclk)
r <= a || q; // assignment makes r inactive
u3: assume property (f3(a, q));
u4: assume property (f4(b, r));
endchecker
...
c1 F1(mclk, v1, const'(v2));
endmodule
The assume set of F1 consists of F1.u3 and F1.B1.u1. The property F1.B1.u1 is included because it
references the formal x, whose actual expression q+r involves an active free checker variable. F1.u4 is
excluded because it references the formal b, which is associated with the const cast actual v2. F1.B1.u2
is excluded because the only formal referenced is y, which is not associated with an active free variable
actual (the actual r is inactive). However, checker instance F1.B1 has its own assume set, which includes
u2 as well as u1; neither of those assume statements involve formals with const cast or automatic actuals.
When a solution attempt is made on an assume set, values shall be sought for all active checker variables
such that, together with the inactive variables and state, none of the assumptions will fail in that time step. If
a set of such values is found, the solution attempt is successful. Otherwise, any values may be chosen for the
active variables and the solution attempt is unsuccessful. There is no requirement that a solution be found if
it exists or that “dead end” states (states where no solution exists) be avoided. For example,
u_deadend: assume property (@(posedge clk) x |=> ##5 1'b0);
If the value 1 is chosen for x, the property would not fail in the current time step; however, it would
inevitably fail six clock cycles later. Such an inevitable future failure is called a dead end. Despite the dead
end, selecting 1 for x is considered a successful solution attempt.
Empty assume sets shall be considered to have an implicit assume set clock event in every time step before
the Observed region. Active variables in checkers with empty assume sets are called implicitly clocked
active free variables; those with nonempty assume sets are explicitly clocked. Implicitly clocked active
variables may be updated with unconstrained random values at every time step. Once updated, the variables
stay constant until the end of the time step.
Active variables that do not appear in any property in a nonempty assume set are unconstrained but
explicitly clocked. They may be updated with random values at every assume set clock event.

<!-- Page 520 -->

IEEE Std When an implementation is about to begin the Observed region, it shall solve for all the active free checker
variables using sampled values of all other variables. A sampled value of an active checker variable is
defined as its current value (see 16.5.1). Note that checker procedures and properties execute in the Reactive
and Observed regions (see 17.7.3), and so have the new values available.
When a solution attempt is unsuccessful, any resulting assumption failure(s) do not occur until an
unsatisfied property is clocked and checked in the Observed region.
#### 17.7.3 Scheduling semantics
Statements and constructs within a checker that are sensitive to changes (e.g., clocking events, continuous
assignments) and all blocking statements are scheduled in the Reactive region (similarly to programs, see
24.3.1). The nonblocking assignments of checker variables schedule their updates in the Re-NBA region.
The Re-NBA region is processed after the Reactive and Re-Inactive regions have been emptied of events
(see 4.2). These scheduling rules make possible assignment of sequence end point values to checker
variables. For example:
checker my_check(...);
...
sequence s; ...; endsequence
always_ff @clk a <= s.triggered;
endchecker
For every transition of signal clk, the simulator will update the variable a in the Re-NBA region with the
value of s.triggered captured in the Reactive region. Had the checker captured the value of
s.triggered in the Active region, a would always be assigned 1'b0, since s.triggered is evaluated in
the Observed region, and the preceding code would be meaningless.
If a free variable is referenced in several assumptions, and some of these assumptions are governed by a
clock changing in the Active region, and others by a clock changing in the Reactive region, assertions and
assumptions referencing this free variable can be executed twice in the same time step. This can result in an
undesired behavior in simulation. For example:
checker check(bit clk1); // clk1 assigned in the Active region
rand bit v, w;
assign clk2 = clk1;
m1: assume property (@clk1 !(v && w));
m2: assume property (@clk2 v || w);
a1: assert property (@clk1 v != w);
// ...
endchecker : check
After clk1 changes in the Active region, assumption m1 and assertion a1 are executed in the Observed
region. Both free variables v and w may be assigned the value 0, which causes assertion a1 to fail. Then
clk2 changes in the Reactive region, and the simulator enters the Observed region again, where assumption
m2 is evaluated. As a result, new values are assigned to v and w (see 17.7.2), and these values can become 0
and 1, respectively. The free variables v and w then keep their values until the next tick of clk1.
Concurrent assertions have invariant scheduling semantics, whether present in checker code or design code.
### 17.8 Functions in checkers
The formal arguments and internal variables of functions used in checkers shall not be declared as free
variables. However, free variables are allowed to be passed in as actual arguments to a function.

<!-- Page 521 -->

IEEE Std Expressions at the right-hand side of checker variable assignments are allowed to include function calls with
the same restrictions that are imposed on function calls in concurrent assertions (see 16.6):
— Functions that appear in expressions shall not contain output, inout, or ref arguments
(constref is allowed).
— Functions shall be automatic (or preserve no state information) and have no side effects.
See an example of a function used in a checker in 17.9.
### 17.9 Complex checker example
The checkers in the following examples make sure that the expression is true in a window delimited by
start_event and end_event. When start_event and end_event are Boolean, the checker may be
implemented as shown in Example 1.
Example 1:
typedef enum { cover_none, cover_all } coverage_level;
checker assert_window1 (
logic test_expr, // Expression to be true in the window
untyped start_event, // Window opens at the completion of the start_event
untyped end_event, // Window closes at the completion of the end_event
event clock = $inferred_clock,
logic reset = $inferred_disable,
string error_msg = "violation",
coverage_level clevel = cover_all // This argument should be bound to an
// elaboration-time constant expression
);
default clocking @clock; endclocking
default disable iff reset;
bit window = 1'b0, next_window = 1'b1;
// Compute next value of window
always_comb begin
if (reset || window && end_event)
next_window = 1'b0;
else if (!window && start_event)
next_window = 1'b1;
else
next_window = window;
end
always_ff @clock
window <= next_window;
property p_window;
start_event && !window |=> test_expr[+] ##0 end_event;
endproperty
a_window: assert property (p_window) else $error(error_msg);
generate if (clevel != cover_none) begin : cover_b
cover_window_open: cover property (start_event && !window)
$display("window_open covered");
cover_window: cover property (
start_event && !window
##1 (!end_event && window) [*]
##1 end_event && window

<!-- Page 522 -->

IEEE Std ) $display("window covered");
end : cover_b
endgenerate
endchecker : assert_window1
If start_event and end_event may be arbitrary sequences, and not necessarily Boolean values, the
checker needs to be implemented differently, as shown in Example 2. This case requires a different
implementation because the reset of the triggered status of a sequence does not create an event (see 9.4.4),
and therefore a sequence triggered method should not be used in the right-hand side of a continuous
assignment or of an assignment in an always_comb procedure.
Example 2:
typedef enum { cover_none, cover_all } coverage_level;
checker assert_window2 (
logic test_expr, // Expression to be true in the window
sequence start_event, // Window opens at the completion of the start_event
sequence end_event, // Window closes at the completion of the end_event
event clock = $inferred_clock,
logic reset = $inferred_disable,
string error_msg = "violation",
coverage_level clevel = cover_all // This argument should be bound to an
// elaboration-time constant expression
);
default clocking @clock; endclocking
default disable iff reset;
bit window = 0;
let start_flag = start_event.triggered;
let end_flag = end_event.triggered;
// Compute next value of window
function bit next_window (bit win);
if (reset || win && end_flag)
return 1'b0;
if (!win && start_flag)
return 1'b1;
return win;
endfunction
always_ff @clock
window <= next_window(window);
property p_window;
start_flag && !window |=> test_expr[+] ##0 end_flag;
endproperty
a_window: assert property (p_window) else $error(error_msg);
generate if (clevel != cover_none) begin : cover_b
cover_window_open: cover property (start_flag && !window)
$display("window_open covered");
cover_window: cover property (
start_flag && !window
##1 (!end_flag && window) [*]
##1 end_flag && window
) $display("window covered");
end : cover_b
endgenerate
endchecker : assert_window2

<!-- Page 523 -->

IEEE Std 1800-2023