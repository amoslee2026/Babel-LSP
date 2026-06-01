# Chapter 10: Assignment statements

## 10. Assignment statements
### 10.1 General
This clause describes the following:
— Continuous assignments
— Procedural blocking and nonblocking assignments
— Procedural continuous assignments (assign, deassign, force, release)
— Net aliasing
### 10.2 Overview
The assignment is the basic mechanism for placing values into nets and variables. There are two basic forms
of assignments:
— The continuous assignment, which assigns values to nets or variables
— The procedural assignment, which assigns values to variables
Continuous assignments drive nets or variables in a manner similar to the way gates drive nets or variables.
The expression on the right-hand side can be thought of as a combinational circuit that drives the net or
variable continuously. In contrast, procedural assignments put values in variables. The assignment does not
have duration; instead, the variable holds the value of the assignment until the next procedural assignment to
that variable.
There are two additional forms of assignments, assign/deassign and force/release, which are called
procedural continuous assignments, described in 10.6.
An assignment consists of two parts, a left-hand side and a right-hand side, separated by the equals ( = )
character; or, in the case of nonblocking procedural assignment, the less-than-equals ( <= ) character pair.
The right-hand side can be any expression that evaluates to a value. The left-hand side indicates the net or
variable to which the right-hand side value is to be assigned. The left-hand side can take one of the forms
given in Table10-1, depending on whether the assignment is a continuous assignment or a procedural
assignment.
Table10-1—Legal left-hand forms in assignment statements
Statement type Left-hand side
Continuous assignment Net or variable (vector or scalar)
Constant bit-select of a vector net or packed variable
Constant part-select of a vector net or packed variable
Concatenation or nested concatenation of any of the above left-hand sides
Procedural assignment Variable (vector or scalar)
Bit-select of a packed variable
Part-select of a packed variable
Memory word
Array
Array element select
Array slice
Concatenation or nested concatenation of any of the above left-hand sides

<!-- Page 249 -->

IEEE Std SystemVerilog also allows a time unit to be specified in the assignment statement, as follows:
#1ns r = a;
r = #1ns a;
r <= #1ns a;
assign #2.5ns sum = a + b;
### 10.3 Continuous assignments
Continuous assignments shall drive values onto nets or variables, both vector (packed) and scalar. This
assignment shall occur whenever the value of the right-hand side changes. Continuous assignments provide
a way to model combinational logic without specifying an interconnection of gates. Instead, the model
specifies the logical expression that drives the net or variable.
There are two forms of continuous assignments: net declaration assignments (see 10.3.1) and continuous
assign statements (see 10.3.2).
The syntax for continuous assignments is given in Syntax10-1.
net_declaration16 ::= // from A.2.1.3
net_type [ drive_strength | charge_strength ] [ vectored | scalared ]
data_type_or_implicit [ delay3 ] list_of_net_decl_assignments ;
| nettype_identifier [ delay_control ] list_of_net_decl_assignments ;
| interconnect implicit_data_type [ # delay_value ]
net_identifier { unpacked_dimension } [ , net_identifier { unpacked_dimension } ] ;
list_of_net_decl_assignments ::= net_decl_assignment { , net_decl_assignment } // from A.2.3
net_decl_assignment ::= net_identifier { unpacked_dimension } [ = expression ] // from A.2.4
continuous_assign ::= // from A.6.1
assign [ drive_strength ] [ delay3 ] list_of_net_assignments ;
| assign [ delay_control ] list_of_variable_assignments ;
list_of_net_assignments ::= net_assignment { , net_assignment }
list_of_variable_assignments ::= variable_assignment { , variable_assignment }
net_assignment ::= net_lvalue = expression
16) A charge strength shall only be used with the trireg keyword. When the vectored or scalared keyword is
used, there shall be at least one packed dimension.
Syntax10-1—Syntax for continuous assignment (excerpt from AnnexA)
#### 10.3.1 The net declaration assignment
The net declaration assignment allows a continuous assignment to be placed on a net in the same statement
that declares the net.
The following is an example of the net declaration form of a continuous assignment:
wire (strong1, pull0) mynet = enable;
Because a net can be declared only once, only one net declaration assignment can be made for a particular
net. This contrasts with the continuous assignment statement; one net can receive multiple assignments of
the continuous assignment form.

<!-- Page 250 -->

IEEE Std An interconnect net (see 6.6.8) shall not have a net declaration assignment.
#### 10.3.2 The continuous assignment statement
The continuous assignment statement shall place a continuous assignment on a net or variable data type. The
net may be explicitly declared or may inherit an implicit declaration in accordance with the implicit
declaration rules defined in 6.10. Variables shall be explicitly declared prior to the continuous assignment
statement.
Assignments on nets or variables shall be continuous and automatic. In other words, whenever an operand in
the right-hand expression changes value, the whole right-hand side shall be evaluated. If the new value is
different from the previous value, then the new value shall be assigned to the left-hand side.
Nets can be driven by multiple continuous assignments or by a mixture of primitive outputs, module outputs,
and continuous assignments. Variables can only be driven by one continuous assignment or by one primitive
output or module output. It shall be an error for a variable driven by a continuous assignment or output to
have an initializer in the declaration or any procedural assignment. See also 6.5.
A continuous assignment to an atomic net shall not drive part of the net; the entire nettype value shall be
driven. Thus the left-hand side of a continuous assignment to a net of a user-defined nettype shall not
contain any indexing or select operations into the data type of the nettype.
Example 1: The following is an example of a continuous assignment to a net that has been previously
declared.
wire mynet ;
assign (strong1, pull0) mynet = enable;
Example 2: The following is an example of the use of a continuous assignment to model a 4-bit adder with
carry. The assignment could not be specified directly in the declaration of the nets because it requires a
concatenation on the left-hand side.
module adder (sum_out, carry_out, carry_in, ina, inb);
output [3:0] sum_out;
output carry_out;
input [3:0] ina, inb;
input carry_in;
wire carry_out, carry_in;
wire [3:0] sum_out, ina, inb;
assign {carry_out, sum_out} = ina + inb + carry_in;
endmodule
Example 3: The following example describes a module with one 16-bit output bus. It selects between one of
four input busses and connects the selected bus to the output bus.
module select_bus(busout, bus0, bus1, bus2, bus3, enable, s);
parameter n = 16;
parameter Zee = 16'bz;
output [1:n] busout;
input [1:n] bus0, bus1, bus2, bus3;
input enable;
input [1:2] s;
tri [1:n] data; // net declaration

<!-- Page 251 -->

IEEE Std // net declaration with continuous assignment
tri [1:n] busout = enable ? data : Zee;
// assignment statement with four continuous assignments
assign
data = (s == 0) ? bus0 : Zee,
data = (s == 1) ? bus1 : Zee,
data = (s == 2) ? bus2 : Zee,
data = (s == 3) ? bus3 : Zee;
endmodule
The following sequence of events is experienced during simulation of this example:
a) The value of s, a bus selector input variable, is checked in the assign statement. Based on the value
of s, the net data receives the data from one of the four input buses.
b) The setting of net data triggers the continuous assignment in the net declaration for busout. If
enable is set, the contents of data are assigned to busout; if enable is 0, the contents of Zee are
assigned to busout.
#### 10.3.3 Continuous assignment delays
A delay given to a continuous assignment shall specify the time duration between a right-hand operand
value change and the assignment made to the left-hand side. If the left-hand references a scalar net, then the
delay shall be treated in the same way as for gate delays; that is, different delays can be given for the output
rising, falling, and changing to high impedance (see 28.16).
If the left-hand references a vector net, then up to three delays can be applied. The following rules determine
which delay controls the assignment:
— If the right-hand side makes a transition from nonzero to zero, then the falling delay shall be used.
— If the right-hand side makes a transition to z, then the turn-off delay shall be used.
— For all other cases, the rising delay shall be used.
If the left-hand side references a net of a user-defined nettype or an array of such nets, then only a single
delay may be applied. The specific delay is used when any change occurs to the value of the net.
Specifying the delay in a continuous assignment that is part of the net declaration shall be treated differently
from specifying a net delay and then making a continuous assignment to the net. A delay value can be
applied to a net in a net declaration, as in the following example:
wire #10 wireA;
This syntax, called a net delay, means that any value change that is to be applied to wireA by some other
statement shall be delayed for ten time units before it takes effect. When there is a continuous assignment in
a declaration, the delay is part of the continuous assignment and is not a net delay. Thus, it shall not be added
to the delay of other drivers on the net. Furthermore, if the assignment is to a vector net, then the rising and
falling delays shall not be applied to the individual bits if the assignment is included in the declaration.
In situations where a right-hand operand changes before a previous change has had time to propagate to the
left-hand side, then the following steps are taken:
a) The value of the right-hand expression is evaluated.
b) If this right-hand side value differs from the value currently scheduled to propagate to the left-hand
side, then the currently scheduled propagation event is descheduled.
c) If the new right-hand side value equals the current left-hand side value, no event is scheduled.

<!-- Page 252 -->

IEEE Std d) If the new right-hand side value differs from the current left-hand side value, a delay is calculated in
the standard way using the current value of the left-hand side, the newly calculated value of the
right-hand side, and the delays indicated on the statement; a new propagation event is then sched-
uled to occur delay time units in the future.
#### 10.3.4 Continuous assignment strengths
The driving strength of a continuous assignment can be specified by the user. This applies only to
assignments to scalar nets, except for nets of types supply0 and supply1.
Continuous assignments driving strengths can be specified either in a net declaration or in a stand-alone
assignment, using the assign keyword. The strength specification, if provided, shall immediately follow
the keyword (either the keyword for the net type or assign) and precede any delay specified. Whenever the
continuous assignment drives the net, the strength of the value shall be simulated as specified.
A drive strength specification shall contain one strength value that applies when the value being assigned to
the net is 1 and a second strength value that applies when the assigned value is 0. The following keywords
shall specify the strength value for an assignment of 1:
supply1 strong1 pull1 weak1 highz1
The following keywords shall specify the strength value for an assignment of 0:
supply0 strong0 pull0 weak0 highz0
The order of the two strength specifications shall be arbitrary. The following two rules shall constrain the
use of drive strength specifications:
— The strength specifications (highz1, highz0) and (highz0, highz1) shall be treated as illegal
constructs.
— If drive strength is not specified, it shall default to (strong1, strong0).
### 10.4 Procedural assignments
Procedural assignments occur within procedures such as always, initial (see 9.2), task, and function
(see Clause13) and can be thought of as “triggered” assignments. The trigger occurs when the flow of
execution in the simulation reaches an assignment within a procedure. Reaching the assignment can be
controlled by conditional statements. Event controls, delay controls, if statements, case statements, and
looping statements can all be used to control whether assignments are evaluated. Clause12 gives details and
examples.
The right-hand side of a procedural assignment can be any expression that evaluates to a value, but the
variable type on the left-hand side may restrict what is a legal expression on the right-hand side. The left-
hand side shall be a variable that receives the assignment from the right-hand side. The left-hand side of a
procedural assignment can take one of the following forms:
— Singular variables, as described in 6.4
— Aggregate variables, as described in Clause7
— Bit-selects, part-selects, and slices of packed arrays
— Slices of unpacked arrays
SystemVerilog contains the following three types of procedural assignment statements:
— Blocking procedural assignment statements (see 10.4.1)
— Nonblocking procedural assignment statements (see 10.4.2)

<!-- Page 253 -->

IEEE Std — Assignment operators (see 11.4.1)
Blocking and nonblocking procedural assignment statements specify different procedural flows in
sequential blocks.
#### 10.4.1 Blocking procedural assignments
A blocking procedural assignment statement shall be executed before the execution of the statements that
follow it in a sequential block (see 9.3.1). A blocking procedural assignment statement shall not prevent the
execution of statements that follow it in a parallel block (see 9.3.2).
The syntax for a blocking procedural assignment is given in Syntax10-2.
blocking_assignment ::= // from A.6.2
variable_lvalue = delay_or_event_control expression
| nonrange_variable_lvalue = dynamic_array_new
| [ implicit_class_handle . | class_scope | package_scope ] hierarchical_variable_identifier
select = class_new
| operator_assignment
| inc_or_dec_expression
operator_assignment ::= variable_lvalue assignment_operator expression
assignment_operator ::= = | += | -= | *= | /= | %= | &= | |= | ^= | <<= | >>= | <<<= | >>>=
Syntax10-2—Blocking assignment syntax (excerpt from AnnexA)
In this syntax, variable_lvalue is a data type that is valid for a procedural assignment statement, = is the
assignment operator, and delay_or_event_control is the optional intra-assignment timing control (see 9.4.5).
The expression is the right-hand side value that shall be assigned to the left-hand side. If the variable_lvalue
requires an evaluation, such as an index expression, class handle, or virtual interface reference, it shall be
evaluated at the time specified by the intra-assignment timing control. The order of evaluation of the
variable_lvalue and the expression on the right-hand side is undefined if a timing control is not specified.
See 4.9.3.
The = assignment operator used by blocking procedural assignments is also used by procedural continuous
assignments and continuous assignments.
The following examples show blocking procedural assignments:
rega = 0;
rega[3] = 1; // a bit-select
rega[3:5] = 7; // a part-select
mema[address] = 8'hff; // assignment to a mem element
{carry, acc} = rega + regb; // a concatenation
Additional assignment operators, such as +=, are described in 11.4.1.
#### 10.4.2 Nonblocking procedural assignments
The nonblocking procedural assignment allows assignment scheduling without blocking the procedural
flow. The nonblocking procedural assignment statement can be used whenever several variable assignments
within the same time step can be made without regard to order or dependence upon each other.

<!-- Page 254 -->

IEEE Std It shall be illegal to make nonblocking assignments to automatic variables or to elements of dynamically
sized array variables.
The syntax for a nonblocking procedural assignment is given in Syntax10-3.
nonblocking_assignment ::= variable_lvalue <= [ delay_or_event_control ] expression // from A.6.2
Syntax10-3—Nonblocking assignment syntax (excerpt from AnnexA)
In this syntax, variable_lvalue is a data type that is valid for a procedural assignment statement, <= is the
nonblocking assignment operator, and delay_or_event_control is the optional intra-assignment timing
control (see 9.4.5). If the variable_lvalue requires an evaluation, such as an index expression, class handle,
or virtual interface reference, it shall be evaluated at the same time as the expression on the right-hand side.
The order of evaluation of the variable_lvalue and the expression on the right-hand side is undefined (see
4.9.4).
The nonblocking assignment operator is the same operator as the “less than or equal to” relational operator.
The interpretation shall be decided from the context in which <= appears. When <= is used in an expression,
it shall be interpreted as a relational operator; and when it is used in a nonblocking procedural assignment, it
shall be interpreted as an assignment operator.
The nonblocking procedural assignments shall be evaluated in two steps as discussed in Clause4. These two
steps are shown in the following example:
Example 1:
module evaluates (out); Step 1: At posedge c, the simulator Nonblocking
output out; evaluates the right-hand sides of
assignment
logic a, b, c; the nonblocking assignments and
schedules
schedules the assignments of the
change at
new values at the end of the
initial begin
nonblocking assign update events time 5
a = 0;
NBA region (see 4.5).
b = 1; a = 0
c = 0;
Step 2: When the simulator activates the b = 1
end
nonblocking assign update events,
the simulator updates the left-hand assignment
always c = #5 ~c; side of each nonblocking
values
assignment statement.
always @(posedge c) begin a = 1
a <= b; // evaluates, schedules,
b = 0
b <= a; // and executes in two steps
end
endmodule
At the end of the time step means that the nonblocking assignments are the last assignments executed in a
time step—with one exception. Nonblocking assignment events can create blocking assignment events.
These blocking assignment events shall be processed after the scheduled nonblocking events.
Unlike an event or delay control for blocking assignments, the nonblocking assignment does not block the
procedural flow. The nonblocking assignment evaluates and schedules the assignment, but it does not block
the execution of subsequent statements in a begin-end block.

<!-- Page 255 -->

IEEE Std Example 2:
module nonblock1; scheduled
logic a, b, c, d, e, f; changes at
time 2
// blocking assignments
initial begin e = 0
a = #10 1; // a will be assigned 1 at time 10
b = #2 0; // b will be assigned 0 at time 12 scheduled
c = #4 1; // c will be assigned 1 at time 16 changes at
end time 4
// nonblocking assignments f = 1
initial begin
d <= #10 1; // d will be assigned 1 at time 10 scheduled
e <= #2 0; // e will be assigned 0 at time 2 changes at
f <= #4 1; // f will be assigned 1 at time 4 time 10
end
d = 1
endmodule
As shown in the previous example, the simulator evaluates and schedules assignments for the end of the
current time step and can perform swapping operations with the nonblocking procedural assignments.
Example 3:
module nonblock2; Step 1: The simulator evaluates the right-hand
logic a, b; side of the nonblocking assignments
initial begin and schedules the assignments for the
a = 0; end of the current time step.
b = 1;
a <= b; // evaluates, schedules, Step 2: At the end of the current time step, the
simulator updates the left-hand side of
b <= a; // and executes in two steps
each nonblocking assignment
end
statement.
initial begin
assignment
$monitor ($time, ,"a = %b b = %b", a, b);
values
#100 $finish;
end a = 1
endmodule
b = 0
The order of the execution of distinct nonblocking assignments to a given variable shall be preserved. In
other words, if there is clear ordering of the execution of a set of nonblocking assignments, then the order of
the resulting updates of the destination of the nonblocking assignments shall be the same as the ordering of
the execution (see 4.6).
Example 4:
module multiple;
logic a;
initial a = 1;
// The assigned value of the variable is determinate
initial begin
a <= #4 0; // schedules a = 0 at time 4

<!-- Page 256 -->

IEEE Std a <= #4 1; // schedules a = 1 at time 4
end // At time 4, a = 1
endmodule
If the simulator executes two procedural blocks concurrently and if these procedural blocks contain
nonblocking assignment operators to the same variable, the final value of that variable is indeterminate. For
example, the value of variable a is indeterminate in the following example:
Example 5:
module multiple2;
logic a;
initial a = 1;
initial a <= #4 0; // schedules 0 at time 4
initial a <= #4 1; // schedules 1 at time 4
// At time 4, a = ??
// The assigned value of the variable is indeterminate
endmodule
The fact that two nonblocking assignments targeting the same variable are in different blocks is not by itself
sufficient to make the order of assignments to a variable indeterminate. For example, the value of variable a
at the end of time cycle 16 is determinate in the following example:
Example 6:
module multiple3;
logic a;
initial #8 a <= #8 1; // executed at time 8;
// schedules an update of 1 at time 16
initial #12 a <= #4 0; // executed at time 12;
// schedules an update of 0 at time 16
// Because it is determinate that the update of a to the value 1
// is scheduled before the update of a to the value 0,
// then it is determinate that a will have the value 0
// at the end of time slot 16.
endmodule
The following example shows how the value of i[0] is assigned to r1 and how the assignments are
scheduled to occur after each time delay:
Example 7:
module multiple4;
logic r1;
logic [2:0] i;
initial begin
// makes assignments to r1 without cancelling previous assignments
for (i = 0; i <= 5; i++)
r1 <= # (i*10) i[0];
end
endmodule

<!-- Page 257 -->

IEEE Std r1
0 10 20 30 40 50
### 10.5 Variable declaration assignment (variable initialization)
Unlike nets, a variable cannot have an implicit continuous assignment as part of its declaration. An
assignment as part of the declaration of a variable is a variable initialization, not a continuous assignment.
The variable declaration assignment is a special case of procedural assignment as it assigns a value to a
variable. It allows an initial value to be placed in a variable in the same statement that declares the variable
(see 6.8). The assignment does not have duration; instead, the variable holds the value until the next
assignment to that variable.
For example:
wire w = vara & varb; // net with a continuous assignment
logic v = consta & constb; // variable with initialization
Setting the initial value of a static variable as part of the variable declaration (including static class
members) shall occur before any initial or always procedures are started. See also 6.21.
### 10.6 Procedural continuous assignments
The procedural continuous assignments (using keywords assign and force) are procedural statements
that allow expressions to be driven continuously onto variables or nets. The syntax for these statements is
given in Syntax10-4.
procedural_continuous_assignment ::= // from A.6.2
assign variable_assignment
| deassign variable_lvalue
| force variable_assignment
| force net_assignment
| release variable_lvalue
| release net_lvalue
variable_assignment ::= variable_lvalue = expression
net_assignment ::= net_lvalue = expression // from A.6.1
Syntax10-4—Syntax for procedural continuous assignments (excerpt from AnnexA)
The right-hand side of an assign or force procedural continuous assignment can be an expression. This
shall be treated just as a continuous assignment; that is, if any variable on the right-hand side of the
assignment changes, the assignment shall be reevaluated while the assign or force is in effect. For example:
force a = b + f(c);

<!-- Page 258 -->

IEEE Std Here, if b or c changes, a will be forced to the new value of the expression b + f(c).
#### 10.6.1 The assign and deassign procedural statements
The assign procedural continuous assignment statement shall override all procedural assignments to a
variable. The deassign procedural statement shall end an assign procedural continuous assignment to a
variable. The value of the variable shall remain the same until the variable is assigned a new value through a
procedural assignment or a procedural continuous assignment. The assign and deassign procedural
statements allow, for example, modeling of asynchronous clear/preset on a D-type edge-triggered flip-flop,
where the clock is inhibited when the clear or preset is active.
The left-hand side of the assignment in the assign statement shall be a singular variable reference or a
concatenation of variables. It shall not be a bit-select or a part-select of a variable.
If the keyword assign is applied to a variable for which there is already an assign procedural continuous
assignment, then this new procedural continuous assignment shall deassign the variable before making the
new procedural continuous assignment.
The following example shows a use of the assign and deassign procedural statements in a behavioral
description of a D-type flip-flop with preset and clear inputs:
module dff (q, d, clear, preset, clock);
output q;
input d, clear, preset, clock;
logic q;
always @(clear or preset)
if (!clear)
assign q = 0;
else if (!preset)
assign q = 1;
else
deassign q;
always @(posedge clock)
q = d;
endmodule
If either clear or preset is low, then the output q will be held continuously to the appropriate constant
value, and a positive edge on the clock will not affect q. When both the clear and preset are high, then
q is deassigned.
NOTE—The procedural assign and deassign constructs are under consideration for deprecation. See C.4.2.
#### 10.6.2 The force and release procedural statements
Another form of procedural continuous assignment is provided by the force and release procedural
statements. These statements have a similar effect to the assign-deassign pair, but a force can be applied
to nets as well as to variables. The left-hand side of the assignment can be a reference to a singular variable,
a net, a constant bit-select of a vector net, a constant part-select of a vector net, or a concatenation of these. It
shall not be a bit-select or a part-select of a variable or of a net with a user-defined nettype. A force or
release statement shall not be applied to a variable that is being assigned by a mixture of continuous and
procedural assignments.
A force statement to a variable shall override a procedural assignment, continuous assignment or an
assign procedural continuous assignment to the variable until a release procedural statement is executed

<!-- Page 259 -->

IEEE Std on the variable. When released, then if the variable is not driven by a continuous assignment and does not
currently have an active assign procedural continuous assignment, the variable shall not immediately
change value and shall maintain its current value until the next procedural assignment to the variable is
executed. Releasing a variable that is driven by a continuous assignment or currently has an active assign
procedural continuous assignment shall reestablish that assignment and schedule a reevaluation in the
continuous assignment’s scheduling region.
A force procedural statement on a net shall override all drivers of the net—gate outputs, module outputs,
and continuous assignments—until a release procedural statement is executed on the net. When released,
the net shall immediately be assigned the value determined by the drivers of the net.
For example:
module test;
logic a, b, c, d;
wire e;
and and1 (e, a, b, c);
initial begin
$monitor("%d d=%b,e=%b", $stime, d, e);
assign d = a & b & c;
a = 1;
b = 0;
c = 1;
#10;
force d = (a | b | c);
force e = (a | b | c);
#10;
release d;
release e;
#10 $finish;
end
endmodule
Results:
0 d=0,e=0
10 d=1,e=1
20 d=0,e=0
In this example, an and gate instance, and1, is “patched” to act like an or gate by a force procedural
statement that forces its output to the value of its ORed inputs, and an assign procedural statement of
ANDed values is “patched” to act like an assign statement of ORed values.
### 10.7 Assignment extension and truncation
The size of the left-hand side of an assignment forms the context for the right-hand expression.
The following are the steps for evaluating an assignment:
— Determine the size of the left-hand side and right-hand side by the standard expression size
determination rules (see 11.8.1).
— When the right-hand side evaluates to fewer bits than the left-hand side, the right-hand side value is
padded to the size of the left-hand side. If the right-hand side is unsigned, it is padded according to
the rules specified in 11.6.1. If the right-hand side is signed, it is sign-extended.

<!-- Page 260 -->

IEEE Std — If the left-hand side is smaller than the right-hand side, truncation shall occur, as described in the
following paragraph.
If the width of the right-hand expression is larger than the width of the left-hand side in an assignment, the
MSBs of the right-hand expression shall be discarded to match the size of the left-hand side.
Implementations may, but are not required to, warn or report any errors related to assignment size mismatch
or truncation. Size casting can be used to indicate explicit intent to change the size (see 6.24.1). Truncating
the sign bit of a signed expression can change the sign of the result.
Some examples of assignment truncation follow.
Example 1:
logic [5:0] a;
logic signed [4:0] b;
initial begin
a = 8'hff; // After the assignment, a = 6'h3f
b = 8'hff; // After the assignment, b = 5'h1f
end
Example 2:
logic [0:5] a;
logic signed [0:4] b, c;
initial begin
a = 8'sh8f; // After the assignment, a = 6'h0f
b = 8'sh8f; // After the assignment, b = 5'h0f
c = -113; // After the assignment, c = 15
// 1000_1111 = (-'h71 = -113) truncates to ('h0F = 15)
end
Example 3:
logic [7:0] a;
logic signed [7:0] b;
logic signed [5:0] c, d;
initial begin
a = 8'hff;
c = a; // After the assignment, c = 6'h3f
b = -113;
d = b; // After the assignment, d = 6'h0f
end
### 10.8 Assignment-like contexts
An assignment-like context is as follows:
— A continuous or procedural assignment
— For a parameter with an explicit type declaration:
• A parameter value assignment in a module, interface, program, or class
• A parameter value override in the instantiation of a module, interface, or program

<!-- Page 261 -->

IEEE Std • A parameter value override in the instantiation of a class or in the left-hand side of a class scope
resolution operator
— A port connection to an input or output port of a module, interface, or program
— The passing of a value to a subroutine input, output, or inout argument
— A return statement in a function
— A tagged union expression
— For an expression that is used as the right-hand value in an assignment-like context:
• If a parenthesized expression, then the expression within the parentheses
• If a mintypmax expression, then the colon-separated expressions
• If a conditional operator expression, then the second and third operand
— A nondefault correspondence between an expression in an assignment pattern and a field or element
in a data object or data value
— A static cast of an expression, as if to a variable of the target type
No other contexts shall be considered assignment-like contexts. In particular, none of the following shall be
considered assignment-like contexts:
— A default correspondence between an expression in an assignment pattern and a field or element in a
data object or data value
— A port expression in a module, interface, or program declaration
— The passing of a value to a subroutine ref port
— A port connection to an inout or ref port of a module, interface, or program
### 10.9 Assignment patterns
Assignment patterns are used for assignments to describe patterns of assignments to structure fields and
array elements.
An assignment pattern specifies a correspondence between a collection of expressions and the fields and
elements in a data object or data value. An assignment pattern has no self-determined data type, but can be
used as one of the sides in an assignment-like context (see 10.8) when the other side has a self-determined
data type. An assignment pattern is built from braces, keys, and expressions and is prefixed with an
apostrophe. For example:
var int A[N] = '{default:1};
var integer i = '{31:1, 23:1, 15:1, 8:1, default:0};
typedef struct {real r, th;} C;
var C x = '{th:PI/2.0, r:1.0};
var real y [0:1] = '{0.0, 1.1}, z [0:9] = '{default: 3.1416};
A positional notation without keys can also be used. For example:
var int B[4] = '{a, b, c, d};
var C y = '{1.0, PI/2.0};
'{a, b, c, d} = B;
When an assignment pattern is used as the left-hand side of an assignment-like context, the positional
notation shall be required, and each member expression shall have a bit-stream data type that is assignment
compatible with and has the same number of bits as the data type of the corresponding element on the right-
hand side.

<!-- Page 262 -->

IEEE Std The assignment pattern syntax is listed in Syntax10-5.
assignment_pattern ::= // from A.6.7.1
' { expression { , expression } }
| ' { structure_pattern_key : expression { , structure_pattern_key : expression } }
| ' { array_pattern_key : expression { , array_pattern_key : expression } }
| ' { constant_expression { expression { , expression } } }
structure_pattern_key ::= member_identifier | assignment_pattern_key
array_pattern_key ::= constant_expression | assignment_pattern_key
assignment_pattern_key ::= simple_type | default
assignment_pattern_expression ::= [ assignment_pattern_expression_type ] assignment_pattern
assignment_pattern_expression_type ::=
ps_type_identifier
| ps_parameter_identifier
| integer_atom_type
|type_reference
constant_assignment_pattern_expression37 ::= assignment_pattern_expression
37) In a constant_assignment_pattern_expression, all member expressions shall be constant expressions.
Syntax10-5—Assignment patterns syntax (excerpt from AnnexA)
An assignment pattern can be used to construct or deconstruct a structure or array by prefixing the pattern
with the name of a data type to form an assignment pattern expression. Unlike an assignment pattern, an
assignment pattern expression has a self-determined data type and is not restricted to being one of the sides
in an assignment-like context. When an assignment pattern expression is used in a right-hand expression, it
shall yield the value that a variable of the data type would hold if it were initialized using the assignment
pattern.
typedef logic [1:0] [3:0] T;
shortint'({T'{1,2}, T'{3,4}}) // yields 16'sh1234
When an assignment pattern expression is used in a left-hand expression, the positional notation shall be
required; and each member expression shall have a bit-stream data type that is assignment compatible with
and has the same number of bits as the corresponding element in the data type of the assignment pattern
expression. If the right-hand expression has a self-determined data type, then it shall be assignment
compatible with and have the same number of bits as the data type of the assignment pattern expression.
typedef byte U[3];
var U A = '{1, 2, 3};
var byte a, b, c;
U'{a, b, c} = A;
U'{c, a, b} = '{a+1, b+1, c+1};
An assignment pattern expression shall not be used in a port expression in a module, interface, or program
declaration.
#### 10.9.1 Array assignment patterns
Concatenation braces are used to construct and deconstruct simple bit vectors. A similar syntax is used to
support the construction and deconstruction of arrays. The expressions shall match element for element, and

<!-- Page 263 -->

IEEE Std the braces shall match the array dimensions. Each expression item shall be evaluated in the context of an
assignment to the type of the corresponding element in the array. In other words, the following examples are
not required to cause size warnings:
bit unpackedbits [1:0] = '{1,1}; // no size warning required as
// bit can be set to 1
int unpackedints [1:0] = '{1'b1, 1'b1}; // no size warning required as
// int can be set to 1'b1
A syntax resembling replications (see 11.4.12.1) can be used in array assignment patterns as well. Each
replication shall represent an entire single dimension.
unpackedbits = '{2 {y}} ; // same as '{y, y}
int n[1:2][1:3] = '{2{'{3{y}}}}; // same as '{'{y,y,y},'{y,y,y}}
SystemVerilog determines the context of the braces when used in the context of an assignment.
It can sometimes be useful to set array elements to a value without having to keep track of how many
members there are. This can be done with the default keyword:
initial unpackedints = '{default:2}; // sets elements to 2
For arrays of structures, it is useful to specify one or more matching type keys, as described under structure
assignment patterns following in 10.9.2.
struct {int a; time b;} abkey[1:0];
abkey = '{'{a:1, b:2ns}, '{int:5, time:$time}};
The matching rules are as follows:
— An index:value specifies an explicit value for a keyed element index. The value is evaluated in
the context of an assignment to the indexed element and shall be castable to its type. It shall be an
error to specify the same index more than once in a single array pattern expression.
— For type:value, if the element or subarray type of the array matches this type, then each element
or subarray that has not already been set by an index key above shall be set to the value. The value
shall be castable to the array element or subarray type. Otherwise, if the array is multidimensional,
then there is a recursive descent into each subarray of the array using the rules in this subclause and
the type and default keys. Otherwise, if the array is an array of structures, there is a recursive
descent into each element of the array using the rules for structure assignment patterns and the type
and default keys. If more than one type matches the same element, the last value shall be used.
— The default:value applies to elements or subarrays that are not matched by either index or type
key. If the type of the element or subarray is a simple bit vector type, matches the self-determined
type of the value, or is not an array or structure type, then the value is evaluated in the context of
each assignment to an element or subarray by the default and shall be castable to the type of the
element or subarray; otherwise, an error is generated. For unmatched subarrays, the type and
default specifiers are applied recursively according to the rules in this subclause to each of its
elements or subarrays. For unmatched structure elements, the type and default keys are applied to
the element according to the rules for structures.
Every element shall be covered by one of these rules.
If the type key, default key, or replication operator is used on an expression with side effects, the number
of times that expression evaluates is undefined.

<!-- Page 264 -->

IEEE Std #### 10.9.2 Structure assignment patterns
A structure can be constructed and deconstructed with a structure assignment pattern built from member
expressions using braces and commas, with the members in declaration order. Replication operators can be
used to set the values for the exact number of members. Each member expression shall be evaluated in the
context of an assignment to the type of the corresponding member in the structure. It can also be built with
the names of the members.
module mod1;
typedef struct {
int x;
int y;
} st;
st s1;
int k = 1;
initial begin
#1 s1 = '{1, 2+k}; // by position
#1 $display( s1.x, s1.y);
#1 s1 = '{x:2, y:3+k}; // by name
#1 $display( s1.x, s1.y);
#1 $finish;
end
endmodule
It can sometimes be useful to set structure members to a value without having to keep track of how many
members there are or what the names are. This can be done with the default keyword:
initial s1 = '{default:2}; // sets x and y to 2
The '{member:value} or '{data_type: default_value} syntax can also be used:
ab abkey[1:0] = '{'{a:1, b:1.0}, '{int:2, shortreal:2.0}};
Use of the default keyword applies to members in nested structures or elements in unpacked arrays in
structures.
struct {
int A;
struct {
int B, C;
} BC1, BC2;
} ABC, DEF;
ABC = '{A:1, BC1:'{B:2, C:3}, BC2:'{B:4,C:5}};
DEF = '{default:10};
To deal with the problem of members of different types, a type can be used as the key. This overrides the
default for members of that type:
typedef struct {
logic [7:0] a;
bit b;
bit signed [31:0] c;
string s;

<!-- Page 265 -->

IEEE Std } sa;
sa s2;
initial s2 = '{int:1, default:0, string:""}; // set all to 0 except the
// array of bits to 1 and
// string to ""
Similarly, an individual member can be set to override the general default and the type default:
initial #10 s2 = '{default:'1, s : ""}; // set all to 1 except s to ""
SystemVerilog determines the context of the braces when used in the context of an assignment.
The matching rules are as follows:
— A member:value specifies an explicit value for a named member of the structure. The named
member shall be at the top level of the structure; a member with the same name in some level of
substructure shall not be set. The value shall be castable to the member type and is evaluated in the
context of an assignment to the named member; otherwise, an error is generated.
— The type:value specifies an explicit value for each field in the structure whose type matches the
type (see 6.22.1) and has not been set by a field name key above. If the same type key is mentioned
more than once, the last value is used. The value is evaluated in the context of an assignment to the
matching type.
— The default:value applies to members that are not matched by either member name or type key.
If the member type is a simple bit vector type, matches the self-determined type of the value, or is
not an array or structure type, then the value is evaluated in the context of each assignment to a
member by the default and shall be castable to the member type; otherwise, an error is generated.
For unmatched structure members, the type and default specifiers are applied recursively accord-
ing to the rules in this subclause to each member of the substructure. For unmatched array members,
the type and default keys are applied to the array according to the rules for arrays.
Every member shall be covered by one of these rules.
If the type key, default key, or replication operator is used on an expression with side effects, the number
of times that expression evaluates is undefined.
### 10.10 Unpacked array concatenation
Unpacked array concatenation provides a flexible way to compose an unpacked array value from a
collection of elements and arrays. An unpacked array concatenation may appear as the source expression in
an assignment-like context and shall not appear in any other context. The target of such assignment-like
context shall be an array whose slowest-varying dimension is an unpacked fixed-size, queue, or dynamic
dimension. A target of any other type (including associative array) shall be illegal.
An unpacked array concatenation shall be written as a comma-separated list, enclosed in braces, of zero or
more items. If the list has zero items, then the concatenation shall denote an array value with no elements.
Otherwise, each item shall represent one or more elements of the resulting array value, interpreted as
follows:
— An item whose self-determined type is assignment compatible with the element type of the target
array shall represent a single element;
— An item whose self-determined type is an unpacked array whose slowest-varying dimension’s
element type is assignment compatible with the element type of the target array shall represent as
many elements as exist in that item, arranged in the same left-to-right order as they would appear in
the array item itself;

<!-- Page 266 -->

IEEE Std — An item of any other type, or an item that has no self-determined type, shall be illegal except that the
literal value null shall be legal if the target array’s elements are of event, class, interface class,
chandle or virtual interface type.
The elements thus represented shall be arranged in left-to-right order to form the resulting array. It shall be
an error if the size of the resulting array differs from the number of elements in a fixed-size target. If the size
exceeds the maximum number of elements of a bounded queue, then elements beyond the upper bound of
the target shall be ignored and a warning shall be issued.
#### 10.10.1 Unpacked array concatenations compared with array assignment patterns
Array assignment patterns have the advantage that they can be used to create assignment pattern expressions
of self-determined type by prefixing the pattern with a type name. Furthermore, items in an assignment
pattern can be replicated using syntax, such as '{ n{element} }, and can be defaulted using the
default: syntax. However, every element item in an array assignment pattern shall be of the same type as
the element type of the target array. By contrast, unpacked array concatenations forbid replication,
defaulting, and explicit typing, but they offer the additional flexibility of composing an array value from an
arbitrary mix of elements and arrays. In some simple cases both forms can have the same effect, as in the
following example:
int A3[1:3];
A3 = {1, 2, 3}; // unpacked array concatenation: A3[1]=1, A3[2]=2, A3[3]=3
A3 = '{1, 2, 3}; // array assignment pattern: A3[1]=1, A3[2]=2, A3[3]=3
The next examples illustrate some differences between the two forms:
typedef int AI3[1:3];
AI3 A3;
int A9[1:9];
A3 = '{1, 2, 3};
A9 = '{3{A3}}; // illegal, A3 is wrong element type
A9 = '{A3, 4, 5, 6, 7, 8, 9}; // illegal, A3 is wrong element type
A9 = {A3, 4, 5, A3, 6}; // legal, gives A9='{1,2,3,4,5,1,2,3,6}
A9 = '{9{1}}; // legal, gives A9='{1,1,1,1,1,1,1,1,1}
A9 = {9{1}}; // illegal, no replication in unpacked
// array concatenation
A9 = {A3, {4,5,6,7,8,9}}; // illegal, {...} is not self-determined here
A9 = {A3, '{4,5,6,7,8,9}}; // illegal, '{...} is not self-determined
A9 = {A3, 4, AI3'{5, 6, 7}, 8, 9}; // legal, A9='{1,2,3,4,5,6,7,8,9}
Unpacked array concatenation is especially useful for writing values of queue type, as shown in the
examples in 7.10.4.
#### 10.10.2 Relationship with other constructs that use concatenation syntax
Concatenation syntax with braces can be used in other SystemVerilog constructs, including vector
concatenation and string concatenation. These forms of concatenation are expressions of self-determined
type, unlike an unpacked array concatenation, which does not have a self-determined type and appears only
as the source expression in an assignment-like context. If concatenation braces appear in an assignment-like
context with an unpacked array target, they unambiguously act as an unpacked array concatenation and shall
conform to the rules given in 10.10. Otherwise, they form a vector or string concatenation according to the
rules given in 11.4.12. The following examples illustrate how the same expression can have different
meanings in different contexts without ambiguity.

<!-- Page 267 -->

IEEE Std string S, hello;
string SA[2];
byte B;
byte BA[2];
hello = "hello";
S = {hello, " world"}; // string concatenation: "hello world"
SA = {hello, " world"}; // array concatenation:
// SA[0]="hello", SA[1]=" world"
B = {4'h6, 4'hf}; // vector concatenation: B=8'h6f
BA = {4'h6, 4'hf}; // array concatenation: BA[0]=8'h06, BA[1]=8'h0f
#### 10.10.3 Nesting of unpacked array concatenations
Each item of an unpacked array concatenation shall have a self-determined type (see 10.10), but a complete
unpacked array concatenation has no self-determined type. Consequently it shall be illegal for an unpacked
array concatenation to appear as an item in another unpacked array concatenation. This rule makes it
possible for a vector or string concatenation to appear as an item in an unpacked array concatenation without
ambiguity, as illustrated in the following example.
string S1, S2;
typedef string T_SQ[$];
T_SQ SQ;
S1 = "S1";
S2 = "S2";
SQ = '{"element 0", "element 1"}; // assignment pattern, two strings
SQ = {S1, SQ, {"element 3 is ", S2} };
In the last line of the preceding example, the outer pair of braces encloses an unpacked array concatenation
whereas the inner pair of braces encloses a string concatenation, so that the resulting queue of strings is
'{"S1", "element 0", "element 1", "element 3 is S2"}
Alternatively the third item in the unpacked array concatenation could instead represent an array of strings,
if it were written as an assignment pattern expression. The unpacked array concatenation would still be valid
in this case, but now it would treat its third item as an array of two strings, each forming one element of the
resulting array:
SQ = {S1, SQ, T_SQ'{"element 3 is ", S2} };
// result: '{"S1", "element 0", "element 1", "element 3 is ", "S2"}
With the exception of default: items, each item of an assignment pattern or an assignment pattern
expression is in an assignment-like context (see 10.9). Consequently an unpacked array concatenation may
appear as a non-default item in an assignment pattern. The following example uses a two-dimensional queue
to build a jagged array of arrays of int, using both an assignment pattern expression and unpacked array
concatenations to represent the subarrays:

<!-- Page 268 -->

IEEE Std typedef int T_QI[$];
T_QI jagged_array[$] = '{ {1}, T_QI'{2,3,4}, {5,6} };
// jagged_array[0][0] = 1 -- jagged_array[0] is a queue of 1 int
// jagged_array[1][0] = 2 -- jagged_array[1] is a queue of 3 ints
// jagged_array[1][1] = 3
// jagged_array[1][2] = 4
// jagged_array[2][0] = 5 -- jagged_array[2] is a queue of 2 ints
// jagged_array[2][1] = 6
### 10.11 Net aliasing
An alias statement declares multiple names for the same physical net, or bits within a net. The syntax for an
alias statement is as follows:
net_alias ::= alias net_lvalue = net_lvalue { = net_lvalue } ; // from A.6.1
net_lvalue ::= // from A.8.5
ps_or_hierarchical_net_identifier constant_select
| { net_lvalue { , net_lvalue } }
| [ assignment_pattern_expression_type ] assignment_pattern_net_lvalue
Syntax10-6—Syntax for net aliasing (excerpt from AnnexA)
The continuous assign statement is a unidirectional assignment and can incorporate a delay and strength
change. To model a bidirectional short-circuit connection, it is necessary to use the alias statement. The
members of an alias list are signals whose bits share the same physical nets. The following example
implements a byte order swapping between bus A and bus B:
module byte_swap (inout wire [31:0] A, inout wire [31:0] B);
alias {A[7:0],A[15:8],A[23:16],A[31:24]} = B;
endmodule
This example strips out the LSB and MSB from a 4-byte bus:
module byte_rip (inout wire [31:0] W, inout wire [7:0] LSB, MSB);
alias W[7:0] = LSB;
alias W[31:24] = MSB;
endmodule
The bit overlay rules are the same as for a packed union with the same member types: each member shall be
the same size, and connectivity is independent of the simulation host. The nets connected with an alias
statement shall be type compatible, that is, they have to be of the same net type. For example, it is illegal to
connect a wand net to a wor net with an alias statement. This rule is stricter than the rule applied to nets
joining at ports because the scope of an alias is limited and such connections are more likely to be a design
error. Variables and hierarchical references cannot be used in alias statements. Any violation of these rules
shall be considered a fatal error.
The same nets can appear in multiple alias statements. The effects are cumulative. The following two
examples are equivalent. In either case, low12[11:4] and high12[7:0] share the same wires.

<!-- Page 269 -->

IEEE Std module overlap(inout wire [15:0] bus16, inout wire [11:0] low12, high12);
alias bus16[11:0] = low12;
alias bus16[15:4] = high12;
endmodule
module overlap(inout wire [15:0] bus16, inout wire [11:0] low12, high12);
alias bus16 = {high12, low12[3:0]};
alias high12[7:0] = low12[11:4];
endmodule
To avoid errors in specification, it is not allowed to specify an alias from an individual signal to itself or to
specify a given alias more than once. The following version of the preceding code would be illegal because
the top 4 bits and bottom 4 bits are the same in both statements:
alias bus16 = {high12[11:8], low12};
alias bus16 = {high12, low12[3:0]};
This alternative is also illegal because the bits of bus16 are being aliased to itself:
alias bus16 = {high12, bus16[3:0]} = {bus16[15:12], low12};
alias statements can appear anywhere module instance statements can appear. If an identifier that has not
been declared as a data type appears in an alias statement, then an implicit net is assumed, following the
same rules as implicit nets for a module instance. The following example uses alias along with the
automatic name binding to connect pins on cells from different libraries to create a standard macro:
module lib1_dff(Reset, Clk, Data, Q, Q_Bar);
...
endmodule
module lib2_dff(reset, clock, data, q, qbar);
...
endmodule
module lib3_dff(RST, CLK, D, Q, Q_);
...
endmodule
module my_dff(rst, clk, d, q, q_bar); // wrapper cell
input rst, clk, d;
output q, q_bar;
alias rst = Reset = reset = RST;
alias clk = Clk = clock = CLK;
alias d = Data = data = D;
alias q = Q;
alias Q_ = q_bar = Q_Bar = qbar;
`LIB_DFF my_dff (.*); // LIB_DFF is any of lib1_dff, lib2_dff or lib3_dff
endmodule
Using a net in an alias statement does not modify its syntactic behavior in other statements. Aliasing is
performed at elaboration time and cannot be undone.

<!-- Page 270 -->

IEEE Std 1800-2023