# Chapter 6: Data types

## 6. Data types
### 6.1 General
This clause describes the following:
— SystemVerilog logic value and strength set
— Net declarations
— Singular variable declarations
— Singular net and variable data types
— Constants
— Scope and lifetime of data
— Type compatibility
— Type operator and type casting
### 6.2 Data types and data objects
SystemVerilog makes a distinction between an object and its data type. A data type is a set of values and a
set of operations that can be performed on those values. Data types can be used to declare data objects or to
define user-defined data types that are constructed from other data types. A data object is a named entity that
has a data value and a data type associated with it, such as a parameter, a variable, or a net.
### 6.3 Value set
#### 6.3.1 Logic values
The SystemVerilog value set consists of the following four basic values:
0—represents a logic zero or a false condition
1—represents a logic one or a true condition
x—represents an unknown logic value
z—represents a high-impedance state
The values 0 and 1 are logical complements of one another.
When the z value is present at the input of a gate or when it is encountered in an expression, the effect is
usually the same as an x value. Notable exceptions are the metal-oxide semiconductor (MOS) primitives,
which can pass the z value.
The name of the basic 4-state data type is logic. This name can be used to declare objects and to construct
other data types from the 4-state data type.
Several SystemVerilog data types are 4-state types, which can store all four logic values. All bits of 4-state
vectors can be independently set to one of the four basic values. Some SystemVerilog data types are 2-state,
and only store 0 or 1 values in each bit of a vector. Other exceptions are the event type (see 6.17), which has
no storage, and the real types (see 6.12).

<!-- Page 90 -->

IEEE Std #### 6.3.2 Strengths
The language includes strength information in addition to the basic value information for nets. This is
described in detail in Clause28. The additional strength information associated with bits of a net is not
considered part of the data type.
Two types of strengths can be specified in a net declaration:
— Charge strength shall only be used when declaring a net of type trireg.
— Drive strength shall only be used when placing a continuous assignment on a net in the same
statement that declares the net.
Gate declarations can also specify a drive strength. See Clause28 for more information on gates and for
information on strengths.
6.3.2.1 Charge strength
The charge strength specification shall be used only with trireg nets. A trireg net shall be used to model
charge storage; charge strength shall specify the relative size of the capacitance indicated by one of the
following keywords:
— small
— medium
— large
The default charge strength of a trireg net shall be medium.
A trireg net can model a charge storage node whose charge decays over time. The simulation time of a
charge decay shall be specified in the delay specification for the trireg net (see 28.16.2).
For example:
trireg a; // trireg net of charge strength medium
trireg (large) #(0,0,50) cap1; // trireg net of charge strength large
// with charge decay time 50 time units
trireg (small) signed [3:0] cap2; // signed 4-bit trireg vector of
// charge strength small
6.3.2.2 Drive strength
The drive strength specification allows a continuous assignment to be placed on a net in the same statement
that declares that net. See Clause10 for more details. Net drive strength properties are described in detail in
Clause28.
### 6.4 Singular and aggregate types
Data types are categorized as either singular or aggregate. A singular type shall be any data type except an
unpacked structure, unpacked union, or unpacked array (see 7.4 on arrays). An aggregate type shall be any
unpacked structure, unpacked union, or unpacked array data type. A singular variable or expression
represents a single value, symbol, or handle. Aggregate expressions and variables represent a set or
collection of singular values. Integral types (see 6.11.1) are always singular even though they can be sliced
into multiple singular values. The string data type is singular even though a string can be indexed in a
similar way to an unpacked array of bytes.

<!-- Page 91 -->

IEEE Std These categories are defined so that operators and functions can simply refer to these data types as a
collective group. For example, some functions recursively descend into an aggregate variable until reaching
a singular value and then perform an operation on each singular value.
Although a class is a type, there are no variables or expressions of class type directly, only class object
handles that are singular. Therefore, classes need not be categorized in this manner (see Clause8 on classes).
### 6.5 Nets and variables
There are two main groups of data objects: variables and nets. These two groups differ in the way in which
they are assigned and hold values.
A net can be written by one or more continuous assignments, by primitive outputs, or through module ports.
The resultant value of multiple drivers is determined by the resolution function of the net type. A net cannot
be procedurally assigned. If a net on one side of a port is driven by a variable on the other side, a continuous
assignment is implied. A force statement can override the value of a net. When released, the net returns to
the resolved value.
Variables can be written by one or more procedural statements, including procedural continuous
assignments. The last write determines the value. Alternatively, variables can be written by one continuous
assignment or one port.
Variables can be packed or unpacked aggregates of other types (see 7.4 for packed and unpacked types).
Multiple assignments made to independent elements of a variable are examined individually. Independent
elements include different members of a structure or different elements of an array.
Thus, a structure or array can have one element assigned procedurally and another element assigned
continuously, and elements of a structure or array can be assigned with multiple continuous assignments,
provided that each element is covered by no more than a single continuous assignment.
Each bit in a packed type is also an independent element. Thus, in an aggregate (packed or unpacked) of
packed types, each bit in the aggregate is an independent element.
The precise rule is that it shall be an error to have multiple continuous assignments or a mixture of
procedural and continuous assignments writing to any term in the expansion of the longest static prefix of a
variable (see 11.5.3 for the definition of a longest static prefix).
For example, assume the following structure declaration:
struct {
bit [7:0] A;
bit [7:0] B;
byte C;
} abc;
The following statements are legal assignments to struct abc:
assign abc.C = sel ? 8'hBE : 8'hEF;
not (abc.A[0],abc.B[0]),
(abc.A[1],abc.B[1]),
(abc.A[2],abc.B[2]),
(abc.A[3],abc.B[3]);
always @(posedge clk) abc.B <= abc.B + 1;

<!-- Page 92 -->

IEEE Std The following additional statements are illegal assignments to struct abc:
// Multiple continuous assignments to abc.C
assign abc.C = sel ? 8'hDE : 8'hED;
// Mixing continuous and procedural assignments to abc.A[3]
always @(posedge clk) abc.A[7:3] <= !abc.B[7:3];
For the purposes of the preceding rule, a declared variable initialization or a procedural assign
statement(see 10.6.1) is considered a procedural assignment. The force statement overrides the procedural
assign statement, which in turn overrides the normal assignments. A force statement is neither a
continuous nor a procedural assignment.
A continuous assignment shall be implied when a variable is connected to an input port declaration. This
makes assignments to a variable declared as an input port illegal. A continuous assignment shall be implied
when a variable is connected to the output port of an instance. This makes additional procedural or
continuous assignments to a variable connected to the output port of an instance illegal.
Variables cannot be connected to either side of an inout port. Variables can be shared across ports with the
ref port type. See 23.3.3 for more information about ports and port connection rules.
The compiler can issue a warning if a continuous assignment could drive strengths other than St0, St1,
StX, or HiZ to a variable. In any case, automatic type conversion shall be applied to the assignment, and the
strength is lost.
Unlike nets, a variable cannot have an implicit continuous assignment as part of its declaration. An
assignment as part of the declaration of a variable is a variable initialization, not a continuous assignment.
For example:
wire w = vara & varb; // net with a continuous assignment
logic v = consta & constb; // variable with initialization
logic vw; // no initial assignment
assign vw = vara & varb; // continuous assignment to a variable
real circ;
assign circ = 2.0 * PI * R; // continuous assignment to a variable
Data shall be declared before they are used, apart from implicit nets (see 6.10).
Within a name space (see 3.13), it shall be illegal to redeclare a name already declared by a net, variable, or
other declaration.
### 6.6 Net types
There are two different kinds of net types: built-in and user-defined. The net types can represent physical
connections between structural entities, such as gates. A net shall not store a value (except for the trireg
net). Instead, its value shall be determined by the values of its drivers, such as a continuous assignment or a
gate. See Clause10 and Clause28 for definitions of these constructs. If no driver is connected to a net, its
value shall be high-impedance (z) unless the net is a trireg, in which case it shall hold the previously
driven value.

<!-- Page 93 -->

IEEE Std There are several distinct types of built-in net types, as shown in Table6-1.
Table6-1—Built-in net types
wire tri tri0 supply0
wand triand tri1 supply1
wor trior trireg uwire
#### 6.6.1 Wire and tri nets
The wire and tri nets connect elements. The net types wire and tri shall be identical in their syntax and
functions; two names are provided so that the name of a net can indicate the purpose of the net in that model.
A wire net can be used for nets that are driven by a single gate or continuous assignment. The tri net type
can be used where multiple drivers drive a net.
Logical conflicts from multiple sources of the same strength on a wire or a tri net result in x (unknown)
values.
Table6-2 is a truth table for resolving multiple drivers on wire and tri nets. It assumes equal strengths for
both drivers. See 28.11 for a discussion of logic strength modeling.
Table6-2—Truth table for wire and tri nets
wire/tri 0 1 x z
0 0 x x 0
1 x 1 x 1
x x x x x
z 0 1 x z
#### 6.6.2 Unresolved nets
The uwire net is an unresolved or unidriver wire and is used to model nets that allow only a single driver.
The uwire type can be used to enforce this restriction. It shall be an error to connect any bit of a uwire net
to more than one driver. It shall be an error to connect a uwire net to a bidirectional terminal of a
bidirectional pass switch.
The port connection rule in 23.3.3.6 enforces this restriction across the net hierarchy or shall issue a warning
if not.
#### 6.6.3 Wired nets
Wired nets are of type wor, wand, trior, and triand and are used to model wired logic configurations.
Wired nets use different truth tables to resolve the conflicts that result when multiple drivers drive the same
net. The wor and trior nets shall create wired or configurations so that when any of the drivers is 1, the
resulting value of the net is 1. The wand and triand nets shall create wired and configurations so that if any
driver is 0, the value of the net is 0.

<!-- Page 94 -->

IEEE Std The net types wor and trior shall be identical in their syntax and functionality. The net types wand and
triand shall be identical in their syntax and functionality. Table6-3 and Table6-4 give the truth tables for
wired nets, assuming equal strengths for both drivers. See 28.11 for a discussion of logic strength modeling.
See 28.12 and 28.12.4 for the case of drivers with different or ambiguous strengths.
Table6-3—Truth table for wand and triand nets
wand/triand 0 1 x z
0 0 0 0 0
1 0 1 x 1
x 0 x x x
z 0 1 x z
Table6-4—Truth table for wor and trior nets
wor/trior 0 1 x z
0 0 1 x 0
1 1 1 1 1
x x 1 x x
z 0 1 x z
#### 6.6.4 Trireg net
The trireg net stores a value and is used to model charge storage nodes. A trireg net can be in one of two
states:
Driven state When at least one driver of a trireg net has a value of 1, 0, or x, the resolved
value propagates into the trireg net and is the driven value of the trireg net.
Capacitive state When all the drivers of a trireg net are at the high-impedance value (z), the
trireg net retains its last driven value; the high-impedance value does not
propagate from the driver to the trireg.
The strength of the value on the trireg net in the capacitive state can be small, medium, or large,
depending on the size specified in the declaration of the trireg net. The strength of a trireg net in the
driven state can be supply, strong, pull, or weak, depending on the strength of the driver. See also
For example, Figure6-1 shows a schematic that includes a trireg net whose size is medium, its driver, and
the simulation results.

<!-- Page 95 -->

IEEE Std wire a wire b
wire c
nmos1 nmos2
trireg d
simulation time wire a wire b wire c trireg d
0 1 1 strong 1 strong 1
10 0 1 HiZ medium 1
Figure6-1—Simulation values of a trireg and its driver
a) At simulation time 0, wire a and wire b have a value of 1. A value of 1 with a strong strength
propagates from the and gate through the nmos switches connected to each other by wire c into
trireg net d.
b) At simulation time 10, wire a changes value to 0, disconnecting wire c from the and gate. When
wire c is no longer connected to the and gate, the value of wire c changes to HiZ. The value of wire
b remains 1 so wire c remains connected to trireg net d through the nmos2 switch. The HiZ value
does not propagate from wire c into trireg net d. Instead, trireg net d enters the capacitive state,
storing its last driven value of 1. It stores the 1 with a medium strength.
6.6.4.1 Capacitive networks
A capacitive network is a connection between two or more trireg nets. In a capacitive network whose trireg
nets are in the capacitive state, logic and strength values can propagate between trireg nets.
For example, Figure6-2 shows a capacitive network in which the logic value of some trireg nets change the
logic value of other trireg nets of equal or smaller size.
In Figure6-2, the capacitive strength of trireg_la net is large, trireg_me1 and trireg_me2 are
medium, and trireg_sm is small. Simulation reports the following sequence of events:
a) At simulation time 0, wire a and wire b have a value of 1. The wire c drives a value of 1 into
trireg_la and trireg_sm; wire d drives a value of 1 into trireg_me1 and trireg_me2.
b) At simulation time 10, the value of wire b changes to 0, disconnecting trireg_sm and
trireg_me2 from their drivers. These trireg nets enter the capacitive state and store the value 1,
their last driven value.
c) At simulation time 20, wire c drives a value of 0 into trireg_la.
d) At simulation time 30, wire d drives a value of 0 into trireg_me1.
e) At simulation time 40, the value of wire a changes to 0, disconnecting trireg_la and
trireg_me1 from their drivers. These trireg nets enter the capacitive state and store the value 0.
f) At simulation time 50, the value of wire b changes to 1.
g) This change of value in wire b connects trireg_sm to trireg_la; these trireg nets have different
sizes and stored different values. This connection causes the smaller trireg net to store the value of
the larger trireg net, and trireg_sm now stores a value of 0.
This change of value in wire b also connects trireg_me1 to trireg_me2; these trireg nets have
the same size and stored different values. The connection causes both trireg_me1 and
trireg_me2 to change value to x.

<!-- Page 96 -->

IEEE Std wire a
wire b
wire c
nmos_1 tranif1_1
trireg_la trireg_sm
wire d
nmos_2 tranif1_2
trireg_me1 trireg_me2
simulation
wire a wire b wire c wire d trireg_la trireg_sm trireg_me1 trireg_me2
time
0 1 1 1 1 1 1 1 1
10 1 0 1 1 1 1 1 1
20 1 0 0 1 0 1 1 1
30 1 0 0 0 0 1 0 1
40 0 0 0 0 0 1 0 1
50 0 1 0 0 0 0 x x
Figure6-2—Simulation results of a capacitive network
In a capacitive network, charge strengths propagate from a larger trireg net to a smaller trireg net. Figure6-3
shows a capacitive network and its simulation results.
In Figure6-3, the capacitive strength of trireg_la is large, and the capacitive strength of trireg_sm is
small. Simulation reports the following results:
a) At simulation time 0, the values of wire a, wire b, and wire c are 1, and wire a drives a strong 1
into trireg_la and trireg_sm.
b) At simulation time 10, the value of wire b changes to 0, disconnecting trireg_la and trireg_sm
from wire a. The trireg_la and trireg_sm nets enter the capacitive state. Both trireg nets share
the large charge of trireg_la because they remain connected through tranif1_2.
c) At simulation time 20, the value of wire c changes to 0, disconnecting trireg_sm from
trireg_la. The trireg_sm no longer shares large charge of trireg_la and now stores a
small charge.
d) At simulation time 30, the value of wire c changes to 1, connecting the two trireg nets. These trireg
nets now share the same charge.
e) At simulation time 40, the value of wire c changes again to 0, disconnecting trireg_sm from
trireg_la. Once again, trireg_sm no longer shares the large charge of trireg_la and now
stores a small charge.

<!-- Page 97 -->

IEEE Std wire b wire c
wire a
tranif1_1 tranif1_2
trireg_la trireg_sm
simulation
wire a wire b wire c trireg_la trireg_sm
time
0 strong 1 1 1 strong 1 strong 1
10 strong 1 0 1 large 1 large 1
20 strong 1 0 0 large 1 small 1
30 strong 1 0 1 large 1 large 1
40 strong 1 0 0 large 1 small 1
Figure6-3—Simulation results of charge sharing
6.6.4.2 Ideal capacitive state and charge decay
A trireg net can retain its value indefinitely, or its charge can decay over time. The simulation time of
charge decay is specified in the delay specification of the trireg net. See 28.16.2 for charge decay
explanation.
#### 6.6.5 Tri0 and tri1 nets
The tri0 and tri1 nets model nets with resistive pulldown and resistive pullup devices on them. A tri0
net is equivalent to a wire net with a continuous 0 value of pull strength driving it. A tri1 net is equivalent
to a wire net with a continuous 1 value of pull strength driving it.
When no driver drives a tri0 net, its value is 0 with strength pull. When no driver drives a tri1 net, its
value is 1 with strength pull. When there are drivers on a tri0 or tri1 net, the drivers combine with the
strength pull value implicitly driven on the net to determine the net’s value. See 28.11 for a discussion of
logic strength modeling. See also
Table6-5 and Table6-6 are truth tables for modeling multiple drivers of strength strong on tri0 and
tri1 nets. The resulting value on the net has strength strong, unless both drivers are z, in which case the
net has strength pull.

<!-- Page 98 -->

IEEE Std Table6-5—Truth table for tri0 net
tri0 0 1 x z
0 0 x x 0
1 x 1 x 1
x x x x x
z 0 1 x 0
Table6-6—Truth table for tri1 net
tri1 0 1 x z
0 0 x x 0
1 x 1 x 1
x x x x x
z 0 1 x 1
#### 6.6.6 Supply nets
The supply0 and supply1 nets can be used to model the power supplies in a circuit. These nets shall have
supply strengths. See also
#### 6.6.7 User-defined nettypes
A user-defined nettype allows users to describe more general abstract values for a wire, including its
resolution function. This nettype is similar to a typedef (see 6.18) in some ways, but shall only be used
in declaring a net. It provides a name for a particular data type and optionally an associated resolution
function.
The syntax for nettype declarations is given in Syntax6-1.
nettype_declaration ::= // from A.2.1.3
nettype data_type nettype_identifier
[ with [ package_scope | class_scope ] tf_identifier ] ;
| nettype [ package_scope | class_scope ] nettype_identifier nettype_identifier ;
Syntax6-1—Syntax for net type declarations (excerpt from AnnexA)
A net declared with a nettype therefore uses that data type and, if specified, the associated resolution
function. An explicit data type is required for a user-defined nettype.
Certain restrictions apply to the data type of a net with a user-defined nettype. A valid data type shall be
one of the following:

<!-- Page 99 -->

IEEE Std a) A 4-state integral type, including a packed array, packed structure or union.
b) A 2-state integral type, including a packed array, packed structure or union with 2-state data type
members.
c) A real or shortreal type.
d) A fixed-size unpacked array, unpacked structure or union, where each element has a valid data type
for a net of a user-defined nettype.
A second form of a nettype declaration is to create another name for an existing nettype.
An atomic net is a net whose value is updated and resolved as a whole. A net declared with a user-defined
nettype is an atomic net. Similarly, a logic net is an atomic net, but a logic vector net is not an atomic
net as each logic element is resolved and updated independently. While an atomic net may have a singular
or aggregate value, each atomic net is intended to describe a single connection point in the design.
The resolution for a user-defined nettype is specified using a SystemVerilog function declaration. If a
resolution function is specified, then when a driver of the net changes value, an update event is scheduled on
the net in the Active (or Reactive) region. When the update event matures, the simulator calls the resolution
function to compute the value of the net from the values of the drivers. The return type of the function shall
match the data type of the nettype. The function shall accept an arbitrary number of drivers, since different
instances of the net could be connected to different numbers of drivers. Any change in the value of one or
more of the drivers shall trigger the evaluation of the resolution function associated with that nettype.
A user-defined resolution function for a net of a user-defined nettype with a data type T shall be a function
with a return type of T and a single input argument whose type is a dynamic array of elements of type T. A
resolution function shall be automatic (or preserve no state information) and have no side effects. A
resolution function shall not resize the dynamic array input argument nor shall it write to any part of the
dynamic array input argument. While a class function method may be used for a resolution function, such
functions shall be class static methods as the method call occurs in a context where no class object is
involved in the call. Parameterized variants of such methods can be created through the use of parameterized
class methods as described in 13.8.
Two different nettypes can use the same data type, but have different resolution functions A nettype may
be declared without a resolution function, in which case it shall be an error for a net of that nettype to have
multiple drivers.
Due to nondeterminism within scheduling regions, if there are multiple driver updates within a scheduling
region, there may be multiple evaluations of the resolution function.
A force statement can override the value of a net of a user-defined nettype. When released, the net
returns to the resolved value.
// user-defined data type T
typedef struct {
real field1;
bit field2;
} T;
// user-defined resolution function Tsum
function automatic T Tsum (input T driver[]);
Tsum.field1 = 0.0;
foreach (driver[i])
Tsum.field1 += driver[i].field1;
endfunction
nettype T wT; // an unresolved nettype wT whose data type is T

<!-- Page 100 -->

IEEE Std // a nettype wTsum whose data type is T and
// resolution function is Tsum
nettype T wTsum with Tsum;
// user-defined data type TR
typedef real TR[5];
// an unresolved nettype wTR whose data type
// is an array of real
nettype TR wTR;
// declare another name nettypeid2 for nettype wTsum
nettype wTsum nettypeid2;
The following example shows how to use a combination of a parameterized class definition with class static
methods to parameterize the data type of a user-defined nettype.
class Base #(parameter p = 1);
typedef struct {
real r;
bit[p-1:0] data;
} T;
static function T Tsum (input T driver[]);
Tsum.r = 0.0;
Tsum.data = 0;
foreach (driver[i])
Tsum.data += driver[i].data;
Tsum.r = $itor(Tsum.data);
endfunction
endclass
typedef Base#(32) MyBaseT;
nettype MyBaseT::T narrowTsum with MyBaseT::Tsum;
typedef Base#(64) MyBaseType;
nettype MyBaseType::T wideTsum with MyBaseType::Tsum;
narrowTsum net1; // data is 32 bits wide
wideTsum net2; // data is 64 bits wide
#### 6.6.8 Generic interconnect
In SystemVerilog it is possible to use net types and configurations to create design models with varying
levels of abstraction. In order to support netlist designs, which primarily specify design element instances
and the net connections between the design elements, SystemVerilog defines a generic form of net. Such
generic nets allow the separation of the specification of the net connections from the types of the
connections.
A net or port declared as interconnect (an interconnect net or port) indicates a typeless or generic net.
Such nets or ports are only able to express net port and terminal connections and shall not be used in any
procedural context nor in any continuous or procedural continuous assignments. An interconnect net or
port shall not be used in any expression other than a net_lvalue expression in which all nets or ports in the
expression are also interconnect nets. An interconnect array shall be considered valid even if
different bits in the array are resolved to different net types as demonstrated in the following example. It
shall be legal to specify a net_alias statement with an interconnect net_lvalue. See 23.3.3.7.1 and
23.3.3.7.2 for port and terminal connection rules for interconnect nets.

<!-- Page 101 -->

IEEE Std package NetsPkg;
nettype real realNet;
endpackage : NetsPkg
module top();
interconnect iBus[0:1];
lDriver l1(iBus[0]);
rDriver r1(iBus[1]);
rlMod m1(iBus);
endmodule : top
module lDriver(output wire logic out);
endmodule : lDriver
module rDriver
import NetsPkg::*;
(output realNet out);
endmodule : rDriver
module rlMod(input interconnect iBus[0:1]);
lMod l1(iBus[0]);
rMod r1(iBus[1]);
endmodule : rlMod
The following simple example serves to illustrate the usefulness of an interconnect net. The example
contains a top level module (top) that instantiates a stimulus module (driver) and a comparator module
(cmp). This configuration is intended to compare two elements and determine if they are equal. There are
two different versions of the configuration, as described by the two different config blocks: one that works
on real values and one that works on logic values. By using the typeless interconnect net, we can use
the same testbench with both configurations, without having to change anything in the testbench itself.
The interconnect net aBus takes its data type from the type of its connections.The output port of module
driver in file driver.svr is an unpacked array of real values, so the corresponding output port of
module driver in file driver.sv and the interconnect port also have to be unpacked arrays.
<file lib.map>
library realLib *.svr;
library logicLib *.sv;
config cfgReal;
design logicLib.top;
default liblist realLib logicLib;
endconfig
config cfgLogic;
design logicLib.top;
default liblist logicLib realLib;
endconfig
<file top.sv>
module top();
interconnect aBus[0:3][0:1];
logic [0:3] dBus;
driver driverArray[0:3](aBus);
cmp cmpArray[0:3](aBus,rst,dBus);
endmodule : top
<file nets.pkg>
package NetsPkg;

<!-- Page 102 -->

IEEE Std nettype real realNet;
endpackage : NetsPkg
<file driver.svr>
module driver
import NetsPkg::*;
#(parameter int delay = 30,
int iterations = 256)
(output realNet out[0:1]);
timeunit 1ns / 1ps;
real outR[1:0];
assign out = outR;
initial begin
outR[0] = 0.0;
outR[1] = 3.3;
for (int i = 0; i < iterations; i++) begin
#delay outR[0] += 0.2;
outR[1] -= 0.2;
end
end
endmodule : driver
<file driver.sv>
module driver #(parameter int delay = 30,
int iterations = 256)
(output wire logic out[0:1]);
timeunit 1ns / 1ps;
logic [0:1] outvar;
assign out[0] = outvar[0];
assign out[1] = outvar[1];
initial begin
outvar = '0;
for (int i = 0; i < iterations; i++)
#delay outvar++;
end
endmodule : driver
<file cmp.svr>
module cmp
import NetsPkg::*;
#(parameter real hyst = 0.65)
(input realNet inA[0:1],
input logic rst,
output logic out);
timeunit 1ns / 1ps;
real updatePeriod = 100.0;
initial out = 1'b0;
always #updatePeriod begin
if (rst) out <= 1'b0;
else if (inA[0] > inA[1]) out <= 1'b1;
else if (inA[0] < inA[1] - hyst) out <= 1'b0;
end
endmodule : cmp

<!-- Page 103 -->

IEEE Std <file cmp.sv>
module cmp #(parameter real hyst = 0.65)
(input wire logic inA[0:1],
input logic rst,
output logic out);
initial out = 1'b0;
always @(inA, rst) begin
if (rst) out <= 1'b0;
else if (inA[0] & ~inA[1]) out <= 1'b1;
else out <= 1'b0;
end
endmodule : cmp
### 6.7 Net declarations
The syntax for net declarations is given in Syntax6-2.
net_declaration16 ::= // from A.2.1.3
net_type [ drive_strength | charge_strength ] [ vectored | scalared ]
data_type_or_implicit [ delay3 ] list_of_net_decl_assignments ;
| nettype_identifier [ delay_control ] list_of_net_decl_assignments ;
| interconnect implicit_data_type [ # delay_value ]
net_identifier { unpacked_dimension } [ , net_identifier { unpacked_dimension } ] ;
net_type ::= // from A.2.2.1
supply0 | supply1 | tri | triand | trior | trireg | tri0 | tri1 | uwire | wire | wand | wor
drive_strength ::= // from A.2.2.2
( strength0 , strength1 )
| ( strength1 , strength0 )
| ( strength0 , highz1 )
| ( strength1 , highz0 )
| ( highz0 , strength1 )
| ( highz1 , strength0 )
strength0 ::= supply0 | strong0 | pull0 | weak0
strength1 ::= supply1 | strong1 | pull1 | weak1
charge_strength ::= ( small ) | ( medium ) | ( large )
delay3 ::= // from A.2.2.3
# delay_value
| # ( mintypmax_expression [ , mintypmax_expression [ , mintypmax_expression ] ] )
delay_value ::=
unsigned_number
| real_number
| ps_identifier
| time_literal
| 1step
list_of_net_decl_assignments ::= net_decl_assignment { , net_decl_assignment } // from A.2.3
net_decl_assignment ::= net_identifier { unpacked_dimension } [ = expression ] // from A.2.4
16) A charge strength shall only be used with the trireg keyword. When the vectored or scalared keyword is
used, there shall be at least one packed dimension.
Syntax6-2—Syntax for net declarations (excerpt from AnnexA)

<!-- Page 104 -->

IEEE Std #### 6.7.1 Net declarations with built-in net types
Net declarations without assignments and whose net type is not a user-defined nettype are described in this
subclause. Net declarations with assignments are described in Clause10.
A net declaration begins with a net type that determines how the values of the nets in the declaration are
resolved. The declaration can include optional information such as delay values, drive or charge strength,
and a data type.
If a set of nets share the same characteristics, they can be declared in the same declaration statement.
Any 4-state data type can be used to declare a net. For example:
trireg (large) logic #(0,0,0) cap1;
typedef logic [31:0] addressT;
wire addressT w1;
wire struct packed {logic ecc; logic [7:0] data;} memsig;
If a data type is not specified in the net declaration or if only a range and/or signing is specified, then the data
type of the net is implicitly declared as logic. For example:
wire w; // equivalent to "wire logic w;"
wire [15:0] ww; // equivalent to "wire logic [15:0] ww;"
A net declared as an interconnect net shall:
— have no data type but may have optional packed or unpacked dimensions;
— not specify drive_strength or charge_strength;
— not have assignment expressions;
— specify at most one delay value.
Certain restrictions apply to the data type of a net. A valid data type for a net shall be one of the following:
a) A 4-state integral type, including, for example, a packed array or packed structure (see 6.11.1).
b) A fixed-size unpacked array or unpacked structure or union, where each element has a valid data
type for a net.
The effect of this recursive definition is that a net is composed entirely of 4-state bits and is treated
accordingly. In addition to a signal value, each bit of a net shall have additional strength information. When
bits of signals combine, the strength and value of the resulting signal shall be determined as described in
28.12.
A lexical restriction applies to the use of the reg keyword in a net or port declaration. A net type keyword
shall not be followed directly by the reg keyword. Thus, the following declarations are in error:
tri reg r;
inout wire reg p;
The reg keyword can be used in a net or port declaration if there are lexical elements between the net type
keyword and the reg keyword.
The default initialization value for a net shall be the value z. Nets with drivers shall assume the output value
of their drivers. The trireg net is an exception. The trireg net shall default to the value x, with the
strength specified in the net declaration (small, medium, or large).

<!-- Page 105 -->

IEEE Std As described in 6.6.8, an interconnect net is restricted in terms of its declaration and use. The following
are some examples of legal and illegal interconnect net declarations:
interconnect w1; // legal
interconnect [3:0] w2; // legal
interconnect [3:0] w3 [1:0]; // legal
interconnect logic [3:0] w4; // illegal – data type specified
interconnect #(1,2,3) w5; // illegal – only one delay permitted
assign w1 = 1; // illegal – not allowed in a
// continuous assign
initial $display(w1); // illegal – not allowed in a
// procedural context
#### 6.7.2 Net declarations with user-defined nettypes
A net with a user-defined nettype allows users to describe more general abstract values for a wire. A net
declared with a nettype uses the data type and any associated resolution function for that nettype.
// an unresolved nettype wT whose data type is T
// Refer to example in 6.6.7 for declaration of the data type T
nettype T wT;
// a nettype wTsum whose data type is T and
// resolution function is Tsum
// Refer to example in 6.6.7 for the declaration of Tsum
nettype T wTsum with Tsum;
// a net of unresolved nettype wT
wT w1;
// an array of nets, each net element is of unresolved nettype wT
wT w2[8];
// a net of resolved nettype wTsum and resolution function Tsum
wTsum w3;
// an array of nets, each net is of resolved nettype wTsum
wTsum w4[8];
// user-defined data type TR which is an array of reals
typedef real TR[5];
// an unresolved nettype wTR with data type TR
nettype TR wTR;
// a net with unresolved nettype wTR and data type TR
wTR w5;
// an array of nets, each net has an unresolved nettype wTR
// and data type TR
wTR w6[8];
#### 6.7.3 Initialization of nets with user-defined nettypes
The resolution function for any net of a user-defined nettype shall be activated at time zero at least once.
This activation occurs even for such nets with no drivers or no value changes on drivers at time zero. Since
the actual evaluation of the resolution function is subject to scheduling nondeterminism, no assumptions can

<!-- Page 106 -->

IEEE Std be made regarding the state of driven values during the guaranteed call, which may precede or follow any
driver changes at time zero.
The initial value of a net with a user-defined nettype shall be set before any initial or always procedures
are started and before the activation of the guaranteed time zero resolution call. The default initialization
value for a net with a user-defined nettype shall be the default value defined by the data type. Table6-7
defines the default value for data types of variables if no initializer is provided; those default values shall
also apply to nets of user-defined nettypes for valid data types of a net. For a net with a user-defined
nettype whose data type is a struct type, any initialization expressions for the members within the
struct shall be applied.
NOTE— The default value for a logic net of a user-defined nettype is x. This default means that a bit of a logic
data type in an unresolved user-defined nettype will be x if it has no drivers, not z. For a net with a resolved
nettype, the value would be determined by the resolution function executed with an empty array of driver values.
### 6.8 Variable declarations
A variable is an abstraction of a data storage element. A variable shall store a value from one assignment to
the next. An assignment statement in a procedure acts as a trigger that changes the value in the data storage
element.
The syntax for variable declarations is given in Syntax6-3.
data_declaration ::= // from A.2.1.3
[ const ] [ var ] [ lifetime ] data_type_or_implicit list_of_variable_decl_assignments ; 14
| ...
data_type_or_implicit ::= // from A.2.2.1
data_type
| implicit_data_type
data_type ::=
integer_vector_type [ signing ] { packed_dimension }
| integer_atom_type [ signing ]
| non_integer_type
| struct_union [ packed [ signing ] ] { struct_union_member { struct_union_member } }
{ packed_dimension }17
| enum [ enum_base_type ] { enum_name_declaration { , enum_name_declaration } }
{ packed_dimension }
| string
| chandle
| virtual [ interface ] interface_identifier [ parameter_value_assignment ] [ . modport_identifier ]
| [ class_scope | package_scope ] type_identifier { packed_dimension }
| class_type
| event
| ps_covergroup_identifier
| type_reference18
integer_atom_type ::= byte | shortint | int | longint | integer | time
integer_vector_type ::= bit | logic | reg
non_integer_type ::= shortreal | real | realtime
signing ::= signed | unsigned
implicit_data_type ::= [ signing ] { packed_dimension }

<!-- Page 107 -->

IEEE Std variable_decl_assignment ::= // from A.2.4
variable_identifier { variable_dimension } [ = expression ]
| dynamic_array_variable_identifier unsized_dimension { variable_dimension }
[ = dynamic_array_new ]
| class_variable_identifier [ = class_new ]
14) In a data_declaration that is not within a procedural context, it shall be illegal to use the automatic keyword. In
a data_declaration, it shall be illegal to omit the explicit data_type before a list_of_variable_decl_assignments
unless the var keyword is used.
17) When a packed dimension is used with the struct or union keyword, the packed keyword shall also be used.
18) When a type_reference is used in a net declaration, it shall be preceded by a net type keyword; and when it is used
in a variable declaration, it shall be preceded by the var keyword.
Syntax6-3—Syntax for variable declarations (excerpt from AnnexA)
One form of variable declaration consists of a data type followed by one or more instances.
shortint s1, s2[0:9];
Another form of variable declaration begins with the keyword var. The data type is optional in this case. If
a data type is not specified or if only a range and/or signing is specified, then the data type is implicitly
declared as logic.
var byte my_byte; // equivalent to "byte my_byte;"
var v; // equivalent to "var logic v;"
var [15:0] vw; // equivalent to "var logic [15:0] vw;"
var enum bit { clear, error } status;
input var logic data_in;
var reg r;
If a set of variables share the same characteristics, they can be declared in the same declaration statement.
A variable can be declared with an initializer, for example:
int i = 0;
Setting the initial value of a static variable as part of the variable declaration (including static class
members) shall occur before any initial or always procedures are started (also see 6.21 and 10.5 on variable
initialization with static and automatic lifetimes).
NOTE—In IEEE Std 1364-2005, an initialization value specified as part of the declaration was executed as if the
assignment were made from an initial procedure, after simulation has started.
Initial values are not constrained to simple constants; they can include run-time expressions, including
dynamic memory allocation. For example, a static class handle or a mailbox can be created and initialized by
calling its new method (see 15.4.1), or static variables can be initialized to random values by calling the
$urandom system task. This may require a special pre-initial pass at run time.

<!-- Page 108 -->

IEEE Std Table6-7 contains the default values for variables if no initializer is specified.
Table6-7—Default variable initial values
Type Default initial value
4-state integral 'x
2-state integral '0
real, shortreal 0.0
Enumeration Base type default initial value
string "" (empty string)
event New event
class null
interface class null
chandle (Opaque handle) null
virtual interface null
Nets and variables can be assigned negative values, but only signed types shall retain the significance of the
sign. The byte, shortint, int, integer, and longint types are signed types by default. Other net and
variable types can be explicitly declared as signed. See 11.4.3.1 for a description of how signed and
unsigned nets and variables are treated by certain operators.
### 6.9 Vector declarations
A data object declared as reg, logic, or bit (or as a matching user-defined type or implicitly as logic)
without a range specification shall be considered 1-bit wide and is known as a scalar. A multibit data object
of one of these types shall be declared by specifying a range and is known as a vector. Vectors are packed
arrays of scalars (see 7.4).
#### 6.9.1 Specifying vectors
The range specification ( [msb_constant_expression : lsb_constant_expression] ) gives addresses to the
individual bits in a multibit reg, logic, or bit vector. The most significant bit, specified by the msb
constant expression, is the left-hand value in the range, and the least significant bit, specified by the lsb
constant expression, is the right-hand value in the range.
Both the msb constant expression and the lsb constant expression shall be constant integer expressions. The
msb and lsb constant expressions (see 11.2.1) may be any integer value—positive, negative, or zero. It shall
be illegal for them to contain any unknown (x) or high-impedance (z) bits. The lsb value may be greater
than, equal to, or less than the msb value.
Vectors shall obey laws of arithmetic modulo-2 to the power n (2n), where n is the number of bits in the
vector. Vectors of reg, logic, and bit types shall be treated as unsigned quantities, unless declared to be
signed or connected to a port that is declared to be signed (see 23.2.2.1 and 23.3.3.8).

<!-- Page 109 -->

IEEE Std Examples:
wand w; // a scalar "wand" net
tri [15:0] busa; // a 16-bit bus
trireg (small) storeit; // a charge storage node of strength small
logic a; // a scalar variable
logic[3:0] v; // a 4-bit vector made up of (from most to
// least significant)v[3], v[2], v[1], and v[0]
logic signed [3:0] signed_reg; // a 4-bit vector in range -8 to 7
logic [-1:4] b; // a 6-bit vector
wire w1, w2; // declares two nets
logic [4:0] x, y, z; // declares three 5-bit variables
Implementations may set a limit on the maximum length of a vector, but the limit shall be at least
65536(216) bits.
Implementations are not required to detect overflow of integer operations.
#### 6.9.2 Vector net accessibility
Vectored and scalared shall be optional advisory keywords to be used in vector net declarations. If these
keywords are implemented, certain operations on vector nets may be restricted. If the keyword vectored is
used, bit-selects and part-selects and strength specifications may not be permitted, and the PLI may consider
the net unexpanded. If the keyword scalared is used, bit-selects and part-selects of the net shall be
permitted, and the PLI shall consider the net expanded.
For example:
tri1 scalared [63:0] bus64; //a bus that will be expanded
tri vectored [31:0] data; //a bus that may or may not be expanded
### 6.10 Implicit declarations
The syntax shown in 6.7 and 6.8 shall be used to declare nets and variables explicitly. In the absence of an
explicit declaration, an implicit net of default net type shall be assumed in the following circumstances:
— If an identifier is used in a port expression declaration, then an implicit net of default net type shall
be assumed, with the vector width of the port expression declaration. See 23.2.2.1 for a discussion of
port expression declarations.
— If an identifier is used in the terminal list of a primitive instance or in the port connection list of a
module, interface, program, or static checker instance (but not a procedural checker instance, see
17.3), and that identifier has not been declared previously in the scope where the instantiation
appears or in any scope whose declarations can be directly referenced from the scope where the
instantiation appears (see 23.9), then an implicit scalar net of default net type shall be assumed.
— If an identifier appears on the left-hand side of a continuous assignment statement, and that identifier
has not been declared previously in the scope where the continuous assignment statement appears or
in any scope whose declarations can be directly referenced from the scope where the continuous
assignment statement appears (see 23.9), then an implicit scalar net of default net type shall be
assumed. See 10.3 for a discussion of continuous assignment statements.
The implicit net declaration shall belong to the scope in which the net reference appears. For example, if the
implicit net is declared by a reference in a generate block, then the net is implicitly declared only in that
generate block. Subsequent references to the net from outside the generate block or in another generate
block within the same module either would be illegal or would create another implicit declaration of a

<!-- Page 110 -->

IEEE Std different net (depending on whether the reference meets the preceding criteria). See Clause27 for
information about generate blocks.
See 22.8 for a discussion of control of the type for implicitly declared nets with the `default_nettype
compiler directive.
### 6.11 Integer data types
SystemVerilog provides several integer data types, as shown in Table6-8.
Table6-8—Integer data types
shortint 2-state data type, 16-bit signed integer
int 2-state data type, 32-bit signed integer
longint 2-state data type, 64-bit signed integer
byte 2-state data type, 8-bit signed integer or ASCII character
bit 2-state data type, user-defined vector size, unsigned
logic 4-state data type, user-defined vector size, unsigned
reg 4-state data type, user-defined vector size, unsigned
integer 4-state data type, 32-bit signed integer
time 4-state data type, 64-bit unsigned integer
#### 6.11.1 Integral types
The term integral is used throughout this standard to refer to the data types that can represent a single basic
integer data type, packed array, packed structure, packed union, or enum.
The term simple bit vector type is used throughout this standard to refer to the data types that can directly
represent a one-dimensional packed array of bits. The integer types listed in Table6-8 are simple bit vector
types with predefined widths. The packed structure (see 7.2.1), packed union (see 7.2.2), and
multidimensional packed array types (see 7.4) are not simple bit vector types, but each is equivalent (see
6.22.2) to some simple bit vector type, to and from which it can be easily converted.
#### 6.11.2 2-state (two-value) and 4-state (four-value) data types
Types that can have unknown and high-impedance values are called 4-state types. These are logic, reg,
integer, and time. The other types do not have unknown values and are called 2-state types, for example,
bit and int.
The difference between int and integer is that int is a 2-state type and integer is a 4-state type. The
4-state values have additional bits, which encode the x and z states. The 2-state data types can simulate
faster, take less memory, and are preferred in some design styles.
The keyword reg does not always accurately describe user intent, as it could be perceived to imply a
hardware register. The keyword logic is a more descriptive term. logic and reg denote the same type.
Automatic type conversions from a smaller number of bits to a larger number of bits involve zero extensions
if unsigned or sign extensions if signed. Automatic type conversions from a larger number of bits to a

<!-- Page 111 -->

IEEE Std smaller number of bits involve truncations of the most significant bits (MSBs). When a 4-state value is
automatically converted to a 2-state value, any unknown or high-impedance bits shall be converted to zeros.
#### 6.11.3 Signed and unsigned integer types
Integer types use integer arithmetic and can be signed or unsigned. This affects the meaning of certain
operators (see Clause11 on operators and expressions).
The data types byte, shortint, int, integer, and longint default to signed. The data types time, bit,
reg, and logic default to unsigned, as do arrays of these types. The signedness can be explicitly defined
using the keywords signed and unsigned.
int unsigned ui;
int signed si;
### 6.12 Real, shortreal, and realtime data types
The real 19 data type is the same as a C double. The shortreal data type is the same as a C float. The
realtime declarations shall be treated synonymously with real declarations and can be used
interchangeably. Variables of these three types are collectively referred to as real variables.
Not all operators can be used with expressions involving real numbers and real variables (see 11.3.1). Real
numbers and real variables are also prohibited in the following cases:
— Edge event controls (posedge, negedge, edge) applied to real variables (see 9.4.2)
— Bit-select or part-select references of real variables (see 11.5.1)
— Real index expressions of bit-selects or part-selects of vectors (see 11.5.1)
#### 6.12.1 Conversion
Real numbers shall be converted to integers by rounding the real number to the nearest integer, rather than
by truncating it. Implicit conversion shall take place when a real number is assigned to an integer. If the
fractional part of the real number is exactly 0.5, it shall be rounded away from zero. For example:
— The real numbers 35.7 and 35.5 both become 36 when converted to an integer, and 35.2 becomes 35.
— Converting –1.5 to integer yields –2, and converting 1.5 to integer yields 2.
Implicit conversion shall also take place when an expression is assigned to a real. Individual bits that are x
or z in the net or the variable shall be treated as zero upon conversion.
Explicit conversion can be specified using casting (see 6.24) or using system tasks (see 20.5).
### 6.13 Void data type
The void data type represents nonexistent data. This type can be specified as the return type of functions to
indicate no return value. This type can also be used for members of tagged unions (see 7.3.2).
19The real and shortreal types are represented as described by IEEE Std 754.

<!-- Page 112 -->

IEEE Std ### 6.14 Chandle data type
The chandle data type represents storage for pointers passed using the DPI (see Clause35). The size of a
value of this data type is platform dependent, but shall be at least large enough to hold a pointer on the
machine on which the tool is running.
The syntax to declare a handle is as follows:
chandle variable_name ;
where variable_name is a valid identifier. Chandles shall always be initialized to the value null, which
has a value of 0 on the C side. Chandles are restricted in their usage, with the only legal uses being as
follows:
— Only the following operators are valid on chandle variables:
• Equality (==), inequality (!=) with another chandle or with null
• Case equality (===), case inequality (!==) with another chandle or with null (same semantics
as == and !=)
— Chandles can be tested for a Boolean value, which shall be 0 if the chandle is null and
1otherwise.
— Only the following assignments can be made to a chandle:
• Assignment from another chandle
• Assignment to null
— Chandles can be inserted into associative arrays (refer to 7.8), but the relative ordering of any two
entries in such an associative array can vary, even between successive runs of the same tool.
— Chandles can be used within a class.
— Chandles can be passed as arguments to subroutines.
— Chandles can be returned from functions.
The use of chandles is restricted as follows:
— Ports shall not have the chandle data type.
— Chandles shall not be assigned to variables of any other type.
— Chandles shall not be used as follows:
• In any expression other than as permitted in this subclause
• As ports
• In sensitivity lists or event expressions
• In continuous assignments
• In untagged unions
• In packed types
### 6.15 Class
A class variable can hold a handle to a class object. Defining classes and creating objects is discussed in
Clause8.

<!-- Page 113 -->

IEEE Std ### 6.16 String data type
The string data type is an ordered collection of characters. The length of a string variable is the number
of characters in the collection. Variables of type string are dynamic as their length may vary during
simulation. A single character of a string variable may be selected for reading or writing by indexing the
variable. A single character of a string variable is of type byte.
SystemVerilog also includes a number of special methods to work with strings, which are defined in this
subclause.
A string variable does not represent a string in the same way as a string literal (see 5.9). String literals
behave like packed arrays of a width that is a multiple of 8 bits. A string literal assigned to a packed array of
an integral variable of a different size is either truncated to the size of the variable or padded with zeros to
the left as necessary. When using the string data type instead of an integral variable, strings can be of
arbitrary length and no truncation occurs. String literals are implicitly converted to the string type when
assigned to a string type or used in an expression involving string type operands.
The indices of string variables shall be numbered from 0 to N–1 (where N is the length of the string) so that
index 0 corresponds to the first (leftmost) character of the string and index N–1 corresponds to the last
(rightmost) character of the string. The string variables can take on the special value "", which is the empty
string. Indexing an empty string variable shall be an out-of-bounds access.
A string variable shall not contain the special character "\0". Assigning the value 0 to a string character
shall be ignored.
The syntax to declare a string variable is as follows:
string variable_name [ = initial_value ] ;
where variable_name is a valid identifier and the optional initial_value can be a string literal, the
value "" for an empty string, or a string data type expression. For example:
parameter string default_name = "John Smith";
string myName = default_name;
If an initial value is not specified in the declaration, the variable is initialized to "", the empty string. An
empty string has zero length.
SystemVerilog provides a set of operators that can be used to manipulate combinations of string variables
and string literals. The basic operators defined on the string data type are listed in Table6-9.
A string literal can be assigned to a variable of a string or an integral data type. When assigning to a
variable of integral data type, if the number of bits of the data object is not equal to the number of characters
in the string literal multiplied by 8, the literal is right justified and either truncated on the left or zero-filled
on the left, as necessary. For example:
byte c = "A"; // assigns to c "A"
bit [10:0] b = "\x41"; // assigns to b 'b000_0100_0001
bit [1:4][7:0] h = "hello" ; // assigns to h "ello"
A string literal or an expression of string type can be assigned directly to a variable of string type (a
string variable). Values of integral type can be assigned to a string variable, but require a cast. When casting
an integral value to a string variable, that variable shall grow or shrink to accommodate the integral value. If

<!-- Page 114 -->

IEEE Std the size of the integral value is not a multiple of 8 bits, then the value shall be zero-filled on the left so that its
size is a multiple of 8 bits.
A string literal assigned to a string variable is converted according to the following steps:
— All "\0" characters in the string literal are ignored (i.e., removed from the string).
— If the result of the first step is an empty string literal, the string is assigned the empty string.
— Otherwise, the string is assigned the remaining characters in the string literal.
Casting an integral value to a string variable proceeds in the following steps:
— If the size (in bits) of the integral value is not a multiple of 8, the integral value is left extended and
filled with zeros until its bit size is a multiple of 8. The extended value is then treated the same as a
string literal, where each successive 8 bits represent a character.
— The steps described previously for string literal conversion are then applied to the extended value.
For example:
string s0 = "String literal assign";// sets s0 to "String literal assign"
string s1 = "hello\0world"; // sets s1 to "helloworld"
bit [11:0] b = 12'ha41;
string s2 = string'(b); // sets s2 to 16'h0a41
As a second example:
typedef logic [15:0] r_t;
r_t r;
integer i = 1;
string b = "";
string a = {"Hi", b};
r = r_t'(a); // OK
b = string'(r); // OK
b = "Hi"; // OK
b = {5{"Hi"}}; // OK
a = {i{"Hi"}}; // OK (non-constant replication)
r = {i{"Hi"}}; // invalid (non-constant replication)
a = {i{b}}; // OK
a = {a,b}; // OK
a = {"Hi",b}; // OK
r = {"H",""}; // yields "H\0". "" is converted to 8'b0
b = {"H",""}; // yields "H". "" is the empty string
a[0] = "h"; // OK, same as a[0] = "cough"
a[0] = b; // invalid, requires a cast
a[1] = "\0"; // ignored, a is unchanged
Table6-9—String operators
Operator Semantics
Str1 == Str2 Equality. Checks whether the two string operands are equal. Result is 1 if they
are equal and 0 if they are not. Both operands can be expressions of string
type, or one can be an expression of string type and the other can be a string
literal, which shall be implicitly converted to string type for the comparison.
If both operands are string literals, the operator is the same equality operator as
for integral types.
Str1 != Str2 Inequality. Logical negation of ==

<!-- Page 115 -->

IEEE Std Table6-9—String operators (continued)
Operator Semantics
Str1 < Str2 Comparison: Relational operators return 1 if the corresponding condition is true
Str1 <= Str2 using the lexicographic ordering of the two strings Str1 and Str2. The
Str1 > Str2 comparison uses the compare string method. Both operands can be expressions
Str1 >= Str2 of string type, or one can be an expression of string type and the other can
be a string literal, which shall be implicitly converted to string type for the
comparison. If both operands are string literals, the operator is the same
comparison operator as for integral types.
{Str1,Str2,...,Strn} Concatenation: Each operand can be a string literal or an expression of string
type. If all the operands are string literals the expression shall behave as a
concatenation of integral values (see 11.4.12); if the result of such a
concatenation is used in an expression involving string types then it shall be
implicitly converted to string type. If at least one operand is an expression of
string type, then any operands that are string literals shall be converted to
string type before the concatenation is performed, and the result of the
concatenation shall be of string type. (See also 11.4.12.2)
{multiplier{Str1, Replication (Multiple Concatenation): There shall be at least one operand in the
Str2,...,Strn}} inner pair of braces. Each Stri operand can be a string literal or an expression
of string type. multiplier shall be a non-negative, non-x, non-z
expression of integral type and is not required to be a constant expression.
If the value of multiplier is zero, then the result shall be of string type
and shall be the empty string.
If all the Stri operands are string literals and multiplier is a nonzero
constant expression, the expression shall behave as a multiple concatenation of
integral values (see 11.4.12); if the result of such a replication is used in an
expression involving string types, then it shall be implicitly converted to
string type.
If at least one Stri operand is an expression of string type or if
multiplier is nonconstant, then any operands that are string literals shall be
converted to string type before the concatenation is performed, and the result
of the replication shall be M concatenated copies of the inner concatenation
(where M is the value of multiplier). (See also 11.4.12.2)
Str[index] Indexing. Returns a byte, the ASCII code at the given index. Indices range from
0 to N–1, where N is the number of characters in the string. If given an index out
of range, returns 0. Semantically equivalent to Str.getc(index) in 6.16.3.
Str.method(...) The dot (.) operator is used to invoke a specified method on strings.
SystemVerilog also includes a number of special methods to work with strings, which use the built-in
method notation. These methods are described in 6.16.1 through 6.16.15.
#### 6.16.1 Len()
function int len();
— str.len() returns the length of the string, i.e., the number of characters in the string.
— If str is "", then str.len() returns 0.
#### 6.16.2 Putc()
function void putc(int i, byte c);
— str.putc(i, c) replaces the ith character in str with the given integral value.
— putc does not change the size of str: If i < 0 or i >= str.len(), then str is unchanged.

<!-- Page 116 -->

IEEE Std — If the second argument to putc is zero, the string is unaffected.
The putc method assignment str.putc(j, x) is semantically equivalent to str[j] = x.
#### 6.16.3 Getc()
function byte getc(int i);
— str.getc(i) returns the ASCII code of the ith character in str.
— If i < 0 or i >= str.len(), then str.getc(i) returns 0.
The getc method assignment x = str.getc(j) is semantically equivalent to x = str[j].
#### 6.16.4 Toupper()
function string toupper();
— str.toupper() returns a string with characters in str converted to uppercase.
— str is unchanged.
#### 6.16.5 Tolower()
function string tolower();
— str.tolower() returns a string with characters in str converted to lowercase.
— str is unchanged.
#### 6.16.6 Compare()
function int compare(string s);
— str.compare(s) compares str and s, as in the ANSI C strcmp function with regard to lexical
ordering and return value.
See the relational string operators in Table6-9.
#### 6.16.7 Icompare()
function int icompare(string s);
— str.icompare(s) compares str and s, like the ANSI C strcmp function with regard to lexical
ordering and return value, but the comparison is case insensitive.
#### 6.16.8 Substr()
function string substr(int i, int j);
— str.substr(i, j) returns a new string that is a substring formed by characters in position i
through j of str.
— If i < 0, j < i, or j >= str.len(), substr() returns "" (the empty string).

<!-- Page 117 -->

IEEE Std #### 6.16.9 Atoi(), atohex(), atooct(), atobin()
function integer atoi();
function integer atohex();
function integer atooct();
function integer atobin();
— str.atoi() returns the integer corresponding to the ASCII decimal representation in str. For
example:
str = "123";
int i = str.atoi(); // assigns 123 to i.
The conversion scans all leading digits and underscore characters ( _ ) and stops as soon as it encounters any
other character or the end of the string. It returns zero if no digits were encountered. It does not parse the full
syntax for integer literals (sign, size, apostrophe, base).
— atohex interprets the string as hexadecimal.
— atooct interprets the string as octal.
— atobin interprets the string as binary.
NOTE—These ASCII conversion functions return a 32-bit integer value. Truncation is possible without warning. For
converting integer values greater than 32 bits, see $sscanf in 21.3.4.
#### 6.16.10 Atoreal()
function real atoreal();
— str.atoreal() returns the real number corresponding to the ASCII decimal representation in
str.
The conversion parses for real constants. The scan stops as soon as it encounters any character that does not
conform to this syntax or the end of the string. It returns zero if no digits were encountered.
#### 6.16.11 Itoa()
function void itoa(integer i);
— str.itoa(i) stores the ASCII decimal representation of i into str (inverse of atoi).
#### 6.16.12 Hextoa()
function void hextoa(integer i);
— str.hextoa(i) stores the ASCII hexadecimal representation of i into str (inverse of atohex).
#### 6.16.13 Octtoa()
function void octtoa(integer i);
— str.octtoa(i) stores the ASCII octal representation of i into str (inverse of atooct).

<!-- Page 118 -->

IEEE Std #### 6.16.14 Bintoa()
function void bintoa(integer i);
— str.bintoa(i) stores the ASCII binary representation of i into str (inverse of atobin).
#### 6.16.15 Realtoa()
function void realtoa(real r);
— str.realtoa(r) stores the ASCII real representation of r into str (inverse of atoreal).
### 6.17 Event data type
An event object gives a powerful and efficient means of describing the communication between, and
synchronization of, two or more concurrently active processes. A basic example of this is a small waveform
clock generator that synchronizes control of a synchronous circuit by signaling the occurrence of an explicit
event periodically while the circuit waits for the event to occur.
The event data type provides a handle to a synchronization object. The object referenced by an event
variable can be explicitly triggered and waited for. Furthermore, event variables have a persistent triggered
state that lasts for the duration of the entire time step. Its occurrence can be recognized by using the event
control syntax described in 9.4.2.
An event variable can be assigned or compared to another event variable or assigned the special value null.
When assigned another event variable, both event variables refer to the same synchronization object. When
assigned null, the association between the synchronization object and the event variable is broken.
If an initial value is not specified in the declaration of an event variable, then the variable is initialized to a
new synchronization object.
Examples:
event done; // declare a new event called done
event done_too = done; // declare done_too as alias to done
event empty = null; // event variable with no synchronization object
Event operations and semantics are discussed in detail in 15.5.
### 6.18 User-defined types
SystemVerilog’s data types can be extended with user-defined types using typedef. The syntax for
declaring user-defined types is shown in Syntax6-4.
type_declaration ::= // from A.2.1.3
typedef data_type_or_incomplete_class_scoped_type type_identifier { variable_dimension } ;
| typedef interface_port_identifier constant_bit_select . type_identifier type_identifier ;
| typedef [ forward_type ] type_identifier ;
forward_type ::= enum | struct | union | class | interface class
Syntax6-4—User-defined types (excerpt from AnnexA)

<!-- Page 119 -->

IEEE Std A typedef may be used to give a user-defined name to an existing data type. For example:
typedef int intP;
The named data type can then be used as follows:
intP a, b;
User-defined data type names need to be used for complex data types in casting (see 6.24), which only
allows simple data type names, and as type parameter values (see 6.20.3) when unpacked array types are
used.
A type parameter may also be used to declare a type_identifier. The declaration of a user-defined data type
shall precede any reference to its type_identifier. User-defined data type identifiers have the same scoping
rules as data identifiers, except that hierarchical references to type identifiers shall not be allowed.
References to type identifiers defined within an interface through ports are not considered hierarchical
references and are allowed provided they are locally redefined before being used. Such a typedef is called
an interface-based typedef.
interface intf_i;
typedef int data_t;
endinterface
module sub(intf_i p);
typedef p.data_t my_data_t;
my_data_t data;
// type of 'data' will be int when connected to interface above
endmodule
Sometimes a user-defined type needs to be declared before the contents of the type have been defined. This
is of use with user-defined types derived from the basic data types: enum, struct, union, class, and
interface class. Support for this is provided by the following forms for a forward typedef:
typedef enum type_identifier;
typedef struct type_identifier;
typedef union type_identifier;
typedef class type_identifier;
typedef interface class type_identifier;
typedef type_identifier;
NOTE—While an empty user-defined type declaration is useful for mutuallyreferentialdefinitions of classes as shown
in 8.27, it cannot be used for mutuallyreferential definitions of structures because structures are statically declared and
there is no support for handles to structures.
The last form shows that the basic data type of the user-defined type does not have to be defined in the
forward declaration.
The actual data type definition of a forward typedef declaration shall be resolved within the same local
scope or generate block. It shall be an error if the type_identifier does not resolve to a data type. It shall be
an error if a basic data type was specified by the forward type declaration and the actual type definition does
not conform to the specified basic data type. It shall be legal to have a forward type declaration in the same
scope, either before or after the final type definition. It shall be legal to have multiple forward type
declarations for the same type identifier in the same scope. The use of the term forward type declaration
does not require the forward type declaration to precede the final type definition.

<!-- Page 120 -->

IEEE Std A forward typedef shall be considered incomplete prior to the final type definition. While incomplete
forward types and types defined by an interface-based typedef may resolve to class types, use of the class
scope resolution operator (see 8.23) to select a type with such a prefix shall be restricted to typedef
declarations, the type operator (see 6.23), and type parameter assignments (see 6.20.3). It shall be an error
if the prefix does not resolve to a class.
Example:
typedef C;
C::T x; // illegal; C is an incomplete forward type
typedef C::T c_t; // legal; reference to C::T is made by a typedef
c_t y;
class C;
typedef int T;
endclass
### 6.19 Enumerations
Enumerated types shall be defined using the syntax shown in Syntax6-5.
data_type ::= // from A.2.2.1
...
| enum [ enum_base_type ] { enum_name_declaration { , enum_name_declaration } }
{ packed_dimension }
...
enum_base_type ::=
integer_atom_type [ signing ]
| integer_vector_type [ signing ] [ packed_dimension ]
| type_identifier [ packed_dimension ] 19
enum_name_declaration ::=
enum_identifier [ [ integral_number [ : integral_number ] ] ] [ = constant_expression ]
19) A type_identifier shall be legal as an enum_base_type if it denotes an integer_atom_type, with which an additional
packed dimension is not permitted, or an integer_vector_type.
Syntax6-5—Enumerated types (excerpt from AnnexA)
An enumerated type declares a set of integral named constants. Enumerated data types provide the capability
to abstractly declare strongly typed variables without either a data type or data value(s) and later add the
required data type and value(s) for designs that require more definition. Enumerated data types also can be
easily referenced or displayed using the enumerated names as opposed to the enumerated values.
In the absence of a data type declaration, the default data type shall be int. Any other data type used with
enumerated types shall require an explicit data type declaration.
An enumerated type defines a set of named values. In the following example, light1 and light2 are
defined to be variables of the anonymous (unnamed) enumerated int type that includes the three members:
red, yellow, and green.
enum {red, yellow, green} light1, light2; // anonymous int type

<!-- Page 121 -->

IEEE Std An enumerated name with x or z assignments assigned to an enum with no explicit data type or an explicit
2-state declaration shall be a syntax error.
// Syntax error: IDLE=2'b00, XX=2'bx <ERROR>, S1=2'b01, S2=2'b10
enum bit [1:0] {IDLE, XX='x, S1=2'b01, S2=2'b10} state, next;
An enum declaration of a 4-state type, such as integer, that includes one or more names with x or z
assignments shall be permitted.
// Correct: IDLE=0, XX='x, S1=1, S2=2
enum integer {IDLE, XX='x, S1='b01, S2='b10} state, next;
The values can be cast to integer types and increment from an initial value of 0. This can be overridden.
enum {bronze=3, silver, gold} medal; // silver=4, gold=5
The values can be set for some of the names and not set for other names. The optional value of an enum
named constant is an elaboration-time constant expression (see 6.20) and can include references to
parameters, local parameters, genvars, other enum named constants, and constant functions of these.
Hierarchical names and const variables are not allowed. A name without a value is automatically assigned
an increment of the value of the previous name. It shall be an error to automatically increment the maximum
representable value of the enum.
// c is automatically assigned the increment-value of 8
enum {a=3, b=7, c} alphabet;
An unassigned enumerated name that follows an enum name with x or z assignments shall be a syntax error.
// Syntax error: IDLE=0, XX='x, S1=??, S2=??
enum integer {IDLE, XX='x, S1, S2} state, next;
Both the enumeration names and their integer values shall be unique. It shall be an error to set two values to
the same name or to set the same value to two names, regardless of whether the values are set explicitly or
by automatic incrementing.
// Error: c and d are both assigned 8
enum {a=0, b=7, c, d=8} alphabet;
If the first name is not assigned a value, it is given the initial value of 0.
// a=0, b=7, c=8
enum {a, b=7, c} alphabet;
The integer value expressions are evaluated in the context of a cast to the enum base type. Any enumeration
encoding value that is outside the representable range of the enum base type shall be an error. For an
unsigned base type, this occurs if the cast truncates the value and any of the discarded bits are nonzero. For a
signed base type, this occurs if the cast truncates the value and any of the discarded bits are not equal to the
sign bit of the result. If the integer value expression is a sized literal constant, it shall be an error if the size is
different from the enum base type, even if the value is within the representable range. The value after the
cast is the value used for the name, including in the uniqueness check and automatic incrementing to get a
value for the next name.
// Correct declaration - bronze and gold are unsized
enum bit [3:0] {bronze='h3, silver, gold='h5} medal2;
// Correct declaration - bronze and gold sizes are redundant

<!-- Page 122 -->

IEEE Std enum bit [3:0] {bronze=4'h3, silver, gold=4'h5} medal3;
// Error in the bronze and gold member declarations
enum bit [3:0] {bronze=5'h13, silver, gold=3'h5} medal4;
// Error in c declaration, requires at least 2 bits
enum bit [0:0] {a,b,c} alphabet;
Type checking of enumerated types used in assignments, as arguments, and with operators is covered in
6.19.3. As in C, there is no overloading of literals; therefore, medal2 and medal3 cannot be defined in the
same scope because they contain the same names.
#### 6.19.1 Defining new data types as enumerated types
A type name can be given so that the same type can be used in many places.
typedef enum {NO, YES} boolean;
boolean myvar; // named type
#### 6.19.2 Enumerated type ranges
A range of enumeration elements can be specified automatically, via the syntax shown in Table6-10.
Table6-10—Enumeration element ranges
name Associates the next consecutive number with name.
name = C Associates the constant C to name.
name[N] Generates N named constants in the sequence: name0, name1,..., nameN–1. N shall be a
positive integral number.
name[N] = C Optionally, a constant can be assigned to the generated named constants to associate that
constant to the first generated named constant; subsequent generated named constants are
associated consecutive values.
N shall be a positive integral number.
name[N:M] Creates a sequence of named constants starting with nameN and incrementing or
decrementing until reaching named constant nameM.
N and M shall be non-negative integral numbers.
name[N:M] = C Optionally, a constant can be assigned to the generated named constants to associate that
constant to the first generated named constant; subsequent generated named constants are
associated consecutive values.
N and M shall be non-negative integral numbers.
For example:
typedef enum {add=10, sub[5], jmp[6:8]} E1;
This example defines the enumerated type E1, which assigns the number 10 to the enumerated named
constant add. It also creates the enumerated named constants sub0, sub1, sub2, sub3, and sub4 and
assigns them the values 11...15, respectively. Finally, the example creates the enumerated named constants
jmp6, jmp7, and jmp8 and assigns them the values 16 through 18, respectively.
enum {register[2] = 1, register[2:4] = 10} vr;
The preceding example declares enumerated variable vr, which creates the enumerated named constants
register0 and register1, which are assigned the values 1 and 2, respectively. Next, it creates the

<!-- Page 123 -->

IEEE Std enumerated named constants register2, register3, and register4 and assigns them the values 10, 11,
and 12.
#### 6.19.3 Type checking
Enumerated types are strongly typed; thus, a variable of type enum cannot be directly assigned a value that
lies outside the enumeration set unless an explicit cast is used or unless the enum variable is a member of a
union. This is a powerful type-checking aid, which prevents users from accidentally assigning nonexistent
values to variables of an enumerated type. The enumeration values can still be used as constants in
expressions, and the results can be assigned to any variable of a compatible integral type.
Enumerated variables are type-checked in assignments, arguments, and relational operators. Enumerated
variables are auto-cast into integral values, but assignment of arbitrary expressions to an enumerated
variable requires an explicit cast.
For example:
typedef enum { red, green, blue, yellow, white, black } Colors;
This operation assigns a unique number to each of the color identifiers and creates the new data type
Colors. This type can then be used to create variables of that type.
Colors c;
c = green;
c = 1; // Invalid assignment
if ( 1 == c ) // OK. c is auto-cast to integer
In the preceding example, the value green is assigned to the variable c of type Colors. The second
assignment is invalid because of the strict typing rules enforced by enumerated types.
Casting can be used to perform an assignment of a different data type, or an out-of-range value, to an
enumerated type. Casting is discussed in 6.19.4, 6.24.1, and 6.24.2.
#### 6.19.4 Enumerated types in numerical expressions
Elements of enumerated type variables can be used in numerical expressions. The value used in the
expression is the numerical value associated with the enumerated value. For example:
typedef enum { red, green, blue, yellow, white, black } Colors;
Colors col;
integer a, b;
a = blue * 3;
col = yellow;
b = col + green;
From the previous declaration, blue has the numerical value 2. This example assigns a the value of 6 (2*3),
and it assigns b a value of 4 (3+1).
An enum variable or identifier used as part of an expression is automatically cast to the base type of the
enum declaration (either explicitly or using int as the default). A cast shall be required for an expression
that is assigned to an enum variable where the type of the expression is not equivalent to the enumeration
type of the variable.

<!-- Page 124 -->

IEEE Std Casting to an enum type shall cause a conversion of the expression to its base type without checking the
validity of the value (unless a dynamic cast is used as described in 6.24.2).
typedef enum {Red, Green, Blue} Colors;
typedef enum {Mo,Tu,We,Th,Fr,Sa,Su} Week;
Colors C;
Week W;
int I;
C = Colors'(C+1); // C is converted to an integer, then added to
// one, then converted back to a Colors type
C = C + 1; C++; C+=2; C = I; // Illegal because they would all be
// assignments of expressions without a cast
C = Colors'(Su); // Legal; puts an out of range value into C
I = C + W; // Legal; C and W are automatically cast to int
#### 6.19.5 Enumerated type methods
SystemVerilog includes a set of specialized methods to enable iterating over the values of enumerated types,
which are defined in 6.19.5.1 through 6.19.5.6.
6.19.5.1 First()
The prototype for the first() method is as follows:
function enum first();
The first() method returns the value of the first member of the enumeration.
6.19.5.2 Last()
The prototype for the last() method is as follows:
function enum last();
The last() method returns the value of the last member of the enumeration.
6.19.5.3 Next()
The prototype for the next() method is as follows:
function enum next( int unsigned N = 1 );
The next() method returns the Nth next enumeration value (default is the next one) starting from the
current value of the given variable. A wrap to the start of the enumeration occurs when the end of the
enumeration is reached. If the given value is not a member of the enumeration, the next() method returns
the default initial value for the enumeration (see Table6-7).
6.19.5.4 Prev()
The prototype for the prev() method is as follows:
function enum prev( int unsigned N = 1 );

<!-- Page 125 -->

IEEE Std The prev() method returns the Nth previous enumeration value (default is the previous one) starting from
the current value of the given variable. A wrap to the end of the enumeration occurs when the start of the
enumeration is reached. If the given value is not a member of the enumeration, the prev() method returns
the default initial value for the enumeration (see Table6-7).
6.19.5.5 Num()
The prototype for the num() method is as follows:
function int num();
The num() method returns the number of elements in the given enumeration.
6.19.5.6 Name()
The prototype for the name() method is as follows:
function string name();
The name() method returns the string representation of the given enumeration value. If the given value is
not a member of the enumeration, the name() method returns the empty string.
6.19.5.7 Using enumerated type methods
The following code fragment shows how to display the name and value of all the members of an
enumeration:
typedef enum { red, green, blue, yellow } Colors;
Colors c = c.first;
forever begin
$display( "%s : %d\n", c.name, c );
if( c == c.last ) break;
c = c.next;
end
### 6.20 Constants
Constants are named data objects that never change. SystemVerilog provides three elaboration-time
constants: parameter, localparam, and specparam. SystemVerilog also provides a run-time constant,
const (see 6.20.6).
The parameter, localparam, and specparam constants are collectively referred to as parameter
constants.
Parameter constants can be initialized with a literal.
localparam byte colon1 = ":" ;
specparam delay = 10 ; // specparams are used for specify blocks
parameter logic flag = 1 ;
SystemVerilog provides several methods for setting the value of parameter constants. Each parameter may
be assigned a default value when declared. The value of a parameter of an instantiated module, interface, or
program can be overridden in each instance using one of the following:
— Assignment by ordered list (e.g., m #(value, value) u1 (...); ) (see 23.10.2.1)

<!-- Page 126 -->

IEEE Std — Assignment by name
(e.g., m #(.param1(value), .param2(value)) u1 (...); ) (see 23.10.2.2)
— defparam statements, using hierarchical path names to redefine each parameter (see 23.10.1)
NOTE—The defparam statement might be removed from future versions of the language. See C.4.1.
#### 6.20.1 Parameter declaration syntax
local_parameter_declaration ::= // from A.2.1.1
localparam data_type_or_implicit list_of_param_assignments
| localparam type_parameter_declaration
parameter_declaration ::=
parameter data_type_or_implicit list_of_param_assignments
| parameter type_parameter_declaration
type_parameter_declaration ::= type [ forward_type ] list_of_type_assignments
specparam_declaration ::= specparam [ packed_dimension ] list_of_specparam_assignments ;
data_type_or_implicit ::= // from A.2.2.1
data_type
| implicit_data_type
implicit_data_type ::= [ signing ] { packed_dimension }
forward_type ::= enum | struct | union | class | interface class // from A.2.1.3
list_of_param_assignments ::= param_assignment { , param_assignment } // from A.2.3
list_of_specparam_assignments ::= specparam_assignment { , specparam_assignment }
list_of_type_assignments ::= type_assignment { , type_assignment }
param_assignment ::= // from A.2.4
parameter_identifier { variable_dimension } [ = constant_param_expression ]22
specparam_assignment ::=
specparam_identifier = constant_mintypmax_expression
| pulse_control_specparam
type_assignment ::= type_identifier [ = data_type_or_incomplete_class_scoped_type ]22
parameter_port_list ::= // from A.1.3
# ( list_of_param_assignments { , parameter_port_declaration } )
| # ( parameter_port_declaration { , parameter_port_declaration } )
| #( )
parameter_port_declaration ::=
parameter_declaration
| local_parameter_declaration
| data_type list_of_param_assignments
| type_parameter_declaration
22) It shall be legal to omit the constant_param_expression from a param_assignment or the data_type from a
type_assignment only within a parameter_port_list. However, it shall not be legal to omit them from localparam
declarations in a parameter_port_list.
Syntax6-6—Parameter declaration syntax (excerpt from AnnexA)
The list_of_param_assignments can appear in a module, interface, program, class, or package or in the
parameter_port_list of a module (see 23.2), interface, program, or class. If the declaration of a design

<!-- Page 127 -->

IEEE Std element uses a parameter_port_list (even an empty one), then in any parameter_declaration directly
contained within the declaration, the parameter keyword shall be a synonym for the localparam
keyword (see 6.20.4). All param_assignments appearing within a class body shall become localparam
declarations regardless of the presence or absence of a parameter_port_list. All param_assignments
appearing within a generate block, package, or compilation-unit scope shall become localparam
declarations.
The parameter keyword can be omitted in a parameter port list. For example:
class vector #(size = 1); // size is a parameter in a parameter port list
logic [size-1:0] v;
endclass
interface simple_bus #(AWIDTH = 64, type T = word) // parameter port list
(input logic clk) ; // port list
...
endinterface
In a list of parameter constants, a parameter can depend on earlier parameters. In the following declaration,
the default value of the second parameter depends on the value of the first parameter. The third parameter is
a type, and the fourth parameter is a value of that type.
module mc #(int N = 5, M = N*16, type T = int, T x = 0)
( ... );
...
endmodule
In the declaration of a parameter in a parameter port list, the specification for its default value may be
omitted, in which case the parameter shall have no default value. If no default value is specified for a
parameter of a design element, then an overriding parameter value shall be specified in every instantiation of
that design element (see 23.10). Also, if no default value is specified for a parameter of a design element,
then a tool shall not implicitly instantiate that design element (see 23.3, 23.4, and 24.3). If no default value is
specified for a parameter of a class, then an overriding parameter value shall be specified in every
specialization of that class (see 8.25).
class Mem #(type T, int size);
T words[size];
...
endclass
typedef Mem#(byte, 1024) Kbyte;
#### 6.20.2 Value parameters
A parameter constant can have a type specification and a range specification. The type and range of
parameters shall be in accordance with the following rules:
— A parameter declaration with no type or range specification shall default to the type and range of the
final value assigned to the parameter, after any value overrides have been applied. If the expression
is real, the parameter is real. If the expression is integral, the parameter is a logic vector of the
same size with range [size-1:0].
— A parameter with a range specification, but with no type specification, shall have the range of the
parameter declaration and shall be unsigned. The sign and range shall not be affected by value
overrides.

<!-- Page 128 -->

IEEE Std — A parameter with a type specification, but with no range specification, shall be of the type specified.
A signed parameter shall default to the range of the final value assigned to the parameter, after any
value overrides have been applied.
— A parameter with a signed type specification and with a range specification shall be signed and shall
have the range of its declaration. The sign and range shall not be affected by value overrides.
— A parameter with no range specification and with either a signed type specification or no type
specification shall have an implied range with an LSB equal to 0 and an MSB equal to one less than
the size of the final value assigned to the parameter.
— A parameter with no range specification, with either a signed type specification or no type
specification, and for which the final value assigned to it is unsized shall have an implied range with
an LSB equal to 0 and an MSB equal to an implementation-dependent value of at least 31.
In an assignment to, or override of, a parameter with an explicit type declaration, the type of the right-hand
expression shall be assignment compatible with the declared type (see 6.22.3).
The conversion rules between real and integer values described in 6.12.1 apply to parameters as well.
Bit-selects and part-selects of parameters that are of integral types shall be allowed (see 6.11.1).
A value parameter (parameter, localparam, or specparam) can only be set to an expression of literals,
value parameters or local parameters, genvars, enumerated names, or a constant function of these. Package
references are allowed. Hierarchical names are not allowed. A specparam can also be set to an expression
containing one or more specparams.
Examples:
parameter msb = 7; // defines msb as a constant value 7
parameter e = 25, f = 9; // defines two constant numbers
parameter r = 5.7; // declares r as a real parameter
parameter byte_size = 8,
byte_mask = byte_size - 1;
parameter average_delay = (r + f) / 2;
parameter signed [3:0] mux_selector = 0;
parameter real r1 = 3.5e17;
parameter p1 = 13'h7e;
parameter [31:0] dec_const = 1'b1; // value converted to 32 bits
parameter newconst = 3'h4; // implied range of [2:0]
parameter newconst = 4; // implied range of at least [31:0]
A parameter can also be declared as an aggregate type, such as an unpacked array or an unpacked structure.
An aggregate parameter shall be assigned to or overridden as a whole; individual members of an aggregate
parameter may not be assigned or overridden separately. However, an individual member of an aggregate
parameter may be used in an expression. For example:
parameter logic [31:0] P1 [3:0] = '{ 1, 2, 3, 4 } ; // unpacked array
// parameter declaration
initial begin
if (P1[2][7:0]) ... // use part-select of individual element of the array
...
#### 6.20.3 Type parameters
A parameter constant can also specify a data type, allowing modules, interfaces, or programs to have ports
and data objects whose type is set for each instance.

<!-- Page 129 -->

IEEE Std module ma #(parameter p1 = 1, parameter type p2 = shortint)
(input logic [p1:0] i, output logic [p1:0] o);
p2 j = 0; // type of j is set by a parameter, (shortint unless redefined)
always @(i) begin
o = i;
j++;
end
endmodule
module mb;
logic [3:0] i,o;
ma #(.p1(3), .p2(int)) u1(i,o); //redefines p2 to a type of int
endmodule
A data type parameter (parameter type) can only be set to a data type. Package references are allowed.
Hierarchical names are not allowed.
It shall be illegal to override a type parameter with a defparam statement.
Similar to a forward typedef declaration (see 6.18), a type parameter declaration may restrict its valid types
by including the basic data type enum, struct, union, class, or interface class keyword before the
type parameter identifier:
type enum parameter_identifier
type struct parameter_identifier
type union parameter_identifier
type class parameter_identifier
type interface class parameter_identifier
It shall be an error if the type parameter is assigned a type definition that does not conform to the specified
basic data type.
While type parameters may resolve to class types, use of the class scope resolution operator (see 8.23) to
select a type with such a prefix shall be restricted to typedef declarations (see 6.18), the type operator (see
6.23), and type parameter assignments. It shall be an error if the prefix does not resolve to a class.
Example:
class P#(type C);
C::T x; // Illegal, C is an incomplete type
localparam type C_t = C::T; // Legal, reference to C::T is made
C_t y; // by parameter assignment
endclass : P
class X;
typedef int T;
endclass : X
typedef P#(X) P_X;
#### 6.20.4 Local parameters (localparam)
Local parameters are identical to parameters except that they cannot directly be modified by defparam
statements (see 23.10.1) or instance parameter value assignments (see 23.10.2). Local parameters can be

<!-- Page 130 -->

IEEE Std assigned constant expressions (see 11.2.1) containing parameters, which in turn can be modified with
defparam statements or instance parameter value assignments.
Unlike nonlocal parameters, local parameters can be declared in a generate block, package, class body, or
compilation-unit scope. In these contexts, the parameter keyword shall be a synonym for the localparam
keyword.
Local parameters may be declared in a module’s parameter_port_list. Any parameter declaration appearing
in such a list between a localparam keyword and the next parameter keyword (or the end of the list, if
there is no next parameter keyword) shall be a local parameter. Any other parameter declaration in such a
list shall be a nonlocal parameter that may be overridden as described in 23.10.
#### 6.20.5 Specify parameters
The keyword specparam declares a special type of parameter that is intended only for providing timing and
delay values, but can appear in any expression that is not assigned to a parameter and is not part of the range
specification of a declaration. Specify parameters (also called specparams) are permitted both within the
specify block (see Clause30) and in the main module body.
A specify parameter declared outside a specify block shall be declared before it is referenced. The value
assigned to a specify parameter can be any constant expression. A specify parameter can be used as part of a
constant expression for a subsequent specify parameter declaration. Unlike the parameter constant, a
specify parameter cannot be modified from within the language, but it can be modified through SDF
annotation (see Clause32).
Specify parameters and parameter constants are not interchangeable. In addition, parameter and
localparam shall not be assigned a constant expression that includes any specify parameters. Table6-11
summarizes the differences between the two types of parameter declarations.
Table6-11—Differences between specparams and parameters
Specparams (specify parameter) Parameters
Use keyword specparam Use keyword parameter
Shall be declared inside a module or specify block Shall be declared outside specify blocks
May only be used inside a module or specify block May not be used inside specify blocks
May be assigned specparams and parameters May not be assigned specparams
Use SDF annotation to override values Use defparam or instance declaration
parameter value passing to override values
A specify parameter can have a range specification. The range of specify parameters shall be in accordance
with the following rules:
— A specparam declaration with no range specification shall default to the range of the final value
assigned to the parameter, after any value overrides have been applied.
— A specparam with a range specification shall have the range of the parameter declaration. The
range shall not be affected by value overrides.
Examples:
specify
specparam tRise_clk_q = 150, tFall_clk_q = 200;

<!-- Page 131 -->

IEEE Std specparam tRise_control = 40, tFall_control = 50;
endspecify
The lines between the keywords specify and endspecify declare four specify parameters. The first line
declares specify parameters called tRise_clk_q and tFall_clk_q with values 150 and 200, respectively;
the second line declares tRise_control and tFall_control specify parameters with values 40 and 50,
respectively.
module RAM16GEN ( output [7:0] DOUT,
input [7:0] DIN,
input [5:0] ADR,
input WE, CE);
specparam dhold = 1.0;
specparam ddly = 1.0;
parameter width = 1;
parameter regsize = dhold + 1.0; // Illegal - cannot assign
// specparams to parameters
endmodule
#### 6.20.6 Const constants
A const form of constant differs from a localparam constant in that the localparam shall be set during
elaboration, whereas a const can be set during simulation, such as in an automatic task.
A static constant declared with the const keyword can be set to an expression of literals, parameters, local
parameters, genvars, enumerated names, a constant function of these, or other constants. Hierarchical names
are allowed because constants declared with the const keyword are calculated after elaboration.
const logic option = a.b.c;
An automatic constant declared with the const keyword can be set to any expression that would be legal
without the const keyword.
An instance of a class (an object handle) can also be declared with the const keyword.
const class_name object = new(5,3);
In other words, the object acts like a variable that cannot be written. The members of the object can be
written (except for those members that are declared const).
#### 6.20.7 $ as a constant
The symbol $ represents a special constant in certain contexts. Its meaning is context-dependent.
$ can be used only in the following contexts with the following meanings:
— In an unpacked array declaration, [$] indicates that the array dimension is a queue (see 7.10). For
example, "byte q1[$];" is a declaration of a queue of bytes.
— In a queue select expression, $ denotes the last element in the queue (e.g., q[$]). See 7.10.
— In a value_range of the form [$:expression] or [expression:$], the first form means the set of val-
ues less than or equal to expression, whereas the second form means the set of values greater than or
equal to expression. This can occur in a range_list (see 11.4.13, 12.5.4) or in a dist_item (see
18.5.3).

<!-- Page 132 -->

IEEE Std — In a covergroup_value_range of the form [$:expression] or [expression:$], as in a value_range,
the first form means the set of values less than or equal to expression, whereas the second form
means the set of values greater than or equal to expression (see 19.5.1, 19.5.2, 19.6.1).
— In an integer_covergroup_expression in a cross coverage bin select_expression, $ specifies that all
value tuples are required to satisfy the expression (see 19.6.1.2).
— In a cycle_delay_const_range_expression of the form [constant_expression:$] in a sequence or
property, $ denotes a finite but unbounded maximum.
— As a sequence_actual_arg or property_actual_arg actual argument value to a sequence, property, or
checker, that is used as an actual argument in a sequence, property, or checker instance or used as
the upper bound in a cycle_delay_const_range_expression.
— As the value assigned to a parameter, as described below.
$ may only be used as an entire self-contained expression, except in a queue select expression, in which $
may be used with operators (e.g., q[$+1], see 7.10).
$ may be assigned to a value parameter of a simple bit vector type (see 6.11.1). A parameter to which $ is
assigned may be used only where $ may be specified as a literal constant, with the exception of queue
contexts, where $ parameters are not permitted. Thus, it is legal, for example, to assign a $ parameter to
another parameter (parameter P=Q;).
Note that $ does not represent a specific numeric value. $ is a symbolic value whose meaning depends on
the context in which it is used.
In the following example, $ represents an unbounded range specification, where the upper index can be any
non-negative integer, greater than or equal to the lower bound.
parameter r2 = $;
property inq1(r1,r2);
@(posedge clk) a ##[r1:r2] b ##1 c |=> d;
endproperty
assert property (inq1(3, r2));
A system function $isunbounded() is provided to test whether a parameter is $ (see 20.6.3). The syntax of
the system function is:
function bit $isunbounded( ps_parameter_identifier | hierarchical_parameter_identifier );
$isunbounded() returns true (1'b1) if the argument value is $. Typically, $isunbounded() would be
used as a condition in a generate construct.
The following example illustrates the benefit of using $ in writing properties concisely where the range is
parameterized. The checker in the example verifies that a bus driven by signal en remains 0, i.e., quiet, for
the specified minimum (min_quiet) and maximum (max_quiet) quiet time. The function
$isunbounded() is used for checking the validity of the actual arguments. It is illegal to evaluate
max_quiet == 0 if max_quiet has the value $, so the short-circuiting of the && operator (see 11.3.5)
ensures that max_quiet == 0 is skipped if $isunbounded(max_quiet) is true.
interface quiet_time_checker #(parameter int min_quiet = 0,
parameter int max_quiet = 0)
(input logic clk, reset_n, logic [1:0] en);
generate
if (!$isunbounded(max_quiet) && (max_quiet == 0)) begin
property quiet_time;
@(posedge clk) reset_n |-> ($countones(en) == 1);

<!-- Page 133 -->

IEEE Std endproperty
a1: assert property (quiet_time);
end
else begin
property quiet_time;
@(posedge clk)
(reset_n && ($past(en) != 0) && en == 0)
|->(en == 0)[*min_quiet:max_quiet]
##1 ($countones(en) == 1);
endproperty
a1: assert property (quiet_time);
end
if ((min_quiet == 0) && $isunbounded(max_quiet))
$warning(warning_msg);
endgenerate
endinterface
quiet_time_checker #(0, 0) quiet_never (clk,1,enables);
quiet_time_checker #(2, 4) quiet_in_window (clk,1,enables);
quiet_time_checker #(0, $) quiet_any (clk,1,enables);
Another example below illustrates that by testing for $, a property can be configured according to the
requirements. When parameter max_cks is unbounded, it is not required to test for expr to become false.
interface width_checker #(parameter min_cks = 1, parameter max_cks = 1)
(input logic clk, reset_n, expr);
generate
if ($isunbounded(max_cks)) begin
property width;
@(posedge clk)
(reset_n && $rose(expr)) |-> (expr [*min_cks]);
endproperty
a2: assert property (width);
end
else begin
property width;
@(posedge clk)
(reset_n && $rose(expr)) |-> (expr[*min_cks:max_cks])
##1 (!expr);
endproperty
a2: assert property (width);
end
endgenerate
endinterface
width_checker #(3, $) max_width_unspecified (clk,1,enables);
width_checker #(2, 4) width_specified (clk,1,enables);
### 6.21 Scope and lifetime
Variables declared outside a module, program, interface, checker, task, or function are local to the
compilation unit and have a static lifetime (exist for the whole simulation). This is roughly equivalent to C
static variables declared outside a function, which are local to a file. Variables declared inside a module,
interface, program, or checker, but outside a task, process, or function, are local in scope and have a static
lifetime.

<!-- Page 134 -->

IEEE Std Variables declared inside a static task, function, or block are local in scope and default to a static lifetime.
Specific variables within a static task, function, or block can be explicitly declared as automatic. Such
variables have the lifetime of the call or block and are initialized on each entry to the call or block (also see
### 6.8 on variable initialization). This is roughly equivalent to a C automatic variable.
Tasks and functions may be declared as automatic. Variables declared in an automatic task, function, or
block are local in scope, default to the lifetime of the call or block, and are initialized on each entry to the
call or block (also see 6.8 on variable initialization). An automatic block is one in which declarations are
automatic by default. Specific variables within an automatic task, function, or block can be explicitly
declared as static. Such variables have a static lifetime. This is roughly equivalent to C static variables
declared within a function.
The lifetime of a fork-join block (see 9.3.2) shall encompass the execution of all processes spawned by the
block. The lifetime of a scope enclosing any fork-join block includes the lifetime of the fork-join block.
A variable declaration shall precede any simple reference (non-hierarchical) to that variable. Variable
declarations shall precede any statements within a procedural block. Variables may also be declared in
unnamed blocks. These variables are visible to the unnamed block and any nested blocks below it.
Hierarchical references shall not be used to access these variables by name.
module msl;
int st0; // static
initial begin
int st1; // static
static int st2; // static
automatic int auto1; // automatic
end
task automatic t1();
int auto2; // automatic
static int st3; // static
automatic int auto3; // automatic
endtask
endmodule
Variables declared in a static task, function, or procedural block default to a static lifetime and a local scope.
However, an explicit static keyword shall be required when an initialization value is specified as part of a
static variable’s declaration to indicate the user’s intent of executing that initialization only once at the
beginning of simulation. The static keyword shall be optional where it would not be legal to declare the
variables as automatic. For example:
module top_legal;
int svar1 = 1; // static keyword optional
initial begin
for (int i=0; i<3; i++) begin
automatic int loop3 = 0; // executes every loop
for (int j=0; j<3; j++) begin
loop3++;
$display(loop3);
end
end // prints 1 2 3 1 2 3 1 2 3
for (int i=0; i<3; i++) begin
static int loop2 = 0; // executes once at time zero
for (int j=0; j<3; j++) begin
loop2++;
$display(loop2);
end
end // prints 1 2 3 4 5 6 7 8 9

<!-- Page 135 -->

IEEE Std end
endmodule : top_legal
module top_illegal; // should not compile
initial begin
int svar2 = 2; // static/automatic needed to show intent
for (int i=0; i<3; i++) begin
int loop3 = 0; // illegal statement
for (int i=0; i<3; i++) begin
loop3++;
$display(loop3);
end
end
end
endmodule : top_illegal
An optional qualifier can be used to specify the default lifetime of all variables declared in a task, function,
or block defined within a module, interface, package, or program. The lifetime qualifier is automatic or
static. The default lifetime is static.
program automatic test ;
int i; // not within a procedural block - static
task t ( int a ); // arguments and variables in t are automatic
... // unless explicitly declared static
endtask
endprogram
It is permissible to hierarchically reference any static variable unless the variable is declared inside an
unnamed block. This includes static variables declared inside automatic tasks and functions.
Class methods (see Clause8) and declared for loop variables (see 12.7.1) are by default automatic,
regardless of the lifetime attribute of the scope in which they are declared.
Automatic variables and elements of dynamically sized array variables shall not be written with
nonblocking, continuous, or procedural continuous assignments. Non-static class properties shall not be
written with continuous or procedural continuous assignments. References to automatic variables and
elements or members of dynamic variables shall be limited to procedural blocks.
See also Clause13 on tasks and functions.
### 6.22 Type compatibility
Some constructs and operations require a certain level of type compatibility for their operands to be legal.
There are five levels of type compatibility, formally defined here: matching, equivalent, assignment
compatible, cast compatible, and nonequivalent.
SystemVerilog does not require a category for identical types to be defined here because there is no
construct in the SystemVerilog language that requires it. For example, as defined below, int can be
interchanged with bit signed [31:0] wherever it is syntactically legal to do so. Users can define their
own level of type identity by using the $typename system function (see 20.6.1) or through use of the PLI.
The scope of a data type identifier shall include the hierarchical instance scope. In other words, each
instance with a user-defined type declared inside the instance creates a unique type. To have type matching
or equivalence among multiple instances of the same module, interface, program, or checker, a class, enum,
unpacked structure, or unpacked union type shall be imported from a package or declared at a higher level in

<!-- Page 136 -->

IEEE Std the compilation-unit scope than the declaration of the module, interface, program, or checker. For type
matching, this is true even for packed structure and packed union types.
#### 6.22.1 Matching types
Two data types shall be defined as matching data types using the following inductive definition. If two data
types do not match using the following definition, then they shall be defined to be nonmatching.
a) Any built-in type matches every other occurrence of itself, in every scope.
b) A simple typedef or type parameter override that renames a built-in or user-defined type matches
that built-in or user-defined type within the scope of the type identifier.
typedef bit node; // 'bit' and 'node' are matching types
typedef type1 type2; // 'type1' and 'type2' are matching types
c) An anonymous enum, struct, or union type matches itself among data objects declared within the
same declaration statement and no other data types.
struct packed {int A; int B;} AB1, AB2; // AB1, AB2 have matching types
struct packed {int A; int B;} AB3; // the type of AB3 does not match
// the type of AB1
d) A typedef for an enum, struct, union, or class matches itself and the type of data objects
declared using that data type within the scope of the data type identifier.
typedef struct packed {int A; int B;} AB_t;
AB_t AB1; AB_t AB2; // AB1 and AB2 have matching types
typedef struct packed {int A; int B;} otherAB_t;
otherAB_t AB3; // the type of AB3 does not match the type of AB1 or AB2
e) A simple bit vector type that does not have a predefined width and one that does have a predefined
width match if both are 2-state or both are 4-state, both are signed or both are unsigned, both have
the same width, and the range of the simple bit vector type without a predefined width is [width–
1:0].
typedef bit signed [7:0] BYTE; // matches the byte type
typedef bit signed [0:7] ETYB; // does not match the byte type
f) Two array types match if they are both packed or both unpacked, are the same kind of array (fixed-
size, dynamic, associative, or queue), have matching index types (for associative arrays), and have
matching element types. Fixed-size arrays shall also have the same left and right range bounds. Note
that the element type of a multidimensional array is itself an array type.
typedef byte MEM_BYTES [256];
typedef bit signed [7:0] MY_MEM_BYTES [256]; // MY_MEM_BYTES matches
// MEM_BYTES
typedef logic [1:0] [3:0] NIBBLES;
typedef logic [7:0] MY_BYTE; // MY_BYTE and NIBBLES are not matching types
typedef logic MD_ARY [][2:0];
typedef logic MD_ARY_TOO [][0:2]; // Does not match MD_ARY

<!-- Page 137 -->

IEEE Std g) Explicitly adding signed or unsigned modifiers to a type that does not change its default signing
creates a type that matches the type without the explicit signing specification.
typedef byte signed MY_CHAR; // MY_CHAR matches the byte type
h) A typedef for an enum, struct, union, or class type declared in a package always matches
itself, regardless of the scope into which the type is imported.
#### 6.22.2 Equivalent types
Two data types shall be defined as equivalent data types using the following inductive definition. If the two
data types are not defined as equivalent using the following definition, then they shall be defined to be
nonequivalent.
a) If two types match, they are equivalent.
b) An anonymous enum, unpacked struct, or unpacked union type is equivalent to itself among data
objects declared within the same declaration statement and no other data types.
struct {int A; int B;} AB1, AB2; // AB1, AB2 have equivalent types
struct {int A; int B;} AB3; // AB3 is not type equivalent to AB1
c) Packed arrays, packed structures, packed unions, and built-in integral types are equivalent if they
contain the same number of total bits, are either all 2-state or all 4-state, and are either all signed or
all unsigned.
NOTE—If any bit of a packed structure or union is 4-state, the entire structure or union is considered 4-state.
typedef bit signed [7:0] BYTE; // equivalent to the byte type
typedef struct packed signed {bit[3:0] a, b;} uint8;
// equivalent to the byte type
d) Unpacked fixed-size array types are equivalent if they have equivalent element types and equal size;
the actual range bounds may differ. Note that the element type of a multidimensional array is itself
an array type.
bit [9:0] A [0:5];
bit [1:10] B [6];
typedef bit [10:1] uint10;
uint10 C [6:1]; // A, B and C have equivalent types
typedef int anint [0:0]; // anint is not type equivalent to int
e) Dynamic array, associative array, and queue types are equivalent if they are the same kind of array
(dynamic, associative, or queue), have equivalent index types (for associative arrays), and have
equivalent element types.
The following example is assumed to be within one compilation unit, although the package declaration need
not be in the same unit:
package p1;
typedef struct {int A;} t_1;
endpackage
typedef struct {int A;} t_2;
module sub();
import p1::t_1;

<!-- Page 138 -->

IEEE Std parameter type t_3 = int;
parameter type t_4 = int;
typedef struct {int A;} t_5;
t_1 v1; t_2 v2; t_3 v3; t_4 v4; t_5 v5;
endmodule
module top();
typedef struct {int A;} t_6;
sub #(.t_3(t_6)) s1 ();
sub #(.t_3(t_6)) s2 ();
initial begin
s1.v1 = s2.v1; // legal - both types from package p1 (rule 8)
s1.v2 = s2.v2; // legal - both types from $unit (rule 4)
s1.v3 = s2.v3; // legal - both types from top (rule 2)
s1.v4 = s2.v4; // legal - both types are int (rule 1)
s1.v5 = s2.v5; // illegal - types from s1 and s2 (rule 4)
end
endmodule
#### 6.22.3 Assignment compatible
All equivalent types, and all nonequivalent types that have implicit casting rules defined between them, are
assignment-compatible types. For example, all integral types are assignment compatible. Conversion
between assignment-compatible types can involve loss of data by truncation or rounding.
Unpacked arrays are assignment compatible with certain other arrays that are not of equivalent type.
Assignment compatibility of unpacked arrays is discussed in detail in 7.6.
Compatibility can be in one direction only. For example, an enum can be converted to an integral type
without a cast, but not the other way around. Implicit casting rules are defined in 6.24.
#### 6.22.4 Cast compatible
All assignment-compatible types, plus all nonequivalent types that have defined explicit casting rules, are
cast-compatible types. For example, an integral type requires a cast to be assigned to an enum.
Explicit casting rules are defined in 6.24.
#### 6.22.5 Type incompatible
Type incompatible includes all the remaining nonequivalent types that have no defined implicit or explicit
casting rules. Class handles, interface class handles, and chandles are type incompatible with all other types.
#### 6.22.6 Matching nettypes
a) A nettype matches itself and the nettype of nets declared using that nettype within the scope of
the nettype type identifier.
b) A simple nettype that renames a user-defined nettype matches that user-defined nettype
within the scope of the nettype identifier.
// declare another name nettypeid2 for nettype nettypeid1
nettype nettypeid1 nettypeid2;

<!-- Page 139 -->

IEEE Std ### 6.23 Type operator
The type operator provides a way to refer to the data type of an expression. A type reference can be used
like a type name or local type parameter, for example, in casts, data object declarations, and type parameter
assignments and overrides. It can also be used in equality/inequality and case equality/inequality
comparisons with other type references, and such comparisons are considered to be constant expressions
(see 11.2.1). When a type reference is used in a net declaration, it shall be preceded by a net type keyword;
and when it is used in a variable declaration, it shall be preceded by the var keyword.
var type(a+b) c, d;
c = type(i+3)'(v[15:0]);
The type operator applied to an expression shall represent the self-determined result type of that expression.
The expression shall not be evaluated and shall not contain any hierarchical references or references to
elements of dynamic objects.
type(this) shall represent the type of the enclosing class (see 8.11).
class registry #(type T=int);
…
static function type(this) get(); // calls to get() return a registry #(T)
static type(this) m_inst;
if (m_inst == null) m_inst = new();
return m_inst;
endfunction
…
endclass
class my_int_registry extends registry #();
…
function type(this) other(); // calls to other() return my_int_registry
endfunction
…
endclass
The type operator can also be applied to a data type.
localparam type T = type(bit[12:0]);
When a type reference is used in an equality/inequality or case equality/inequality comparison, it shall only
be compared with another type reference. Two type references shall be considered equal in such
comparisons if, and only if, the types to which they refer match (see 6.22.1).
bit [12:0] A_bus, B_bus;
parameter type bus_t = type(A_bus);
generate
case (type(bus_t))
type(bit[12:0]): addfixed_int #(bus_t) (A_bus,B_bus);
type(real): add_float #(type(A_bus)) (A_bus,B_bus);
endcase
endgenerate

<!-- Page 140 -->

IEEE Std ### 6.24 Casting
#### 6.24.1 Cast operator
A data type can be changed by using a cast (') operation. The syntax for cast operations is shown in
Syntax6-7.
cast ::= casting_type ' ( expression ) // from A.8.4
constant_cast ::= casting_type ' ( constant_expression )
casting_type ::= simple_type | constant_primary | signing | string | const // from A.2.2.1
simple_type ::= integer_type | non_integer_type | ps_type_identifier | ps_parameter_identifier
Syntax6-7—Casting (excerpt from AnnexA)
In a static cast, the expression to be cast shall be enclosed in parentheses that are prefixed with the casting
type and an apostrophe. If the expression is assignment compatible with the casting type, then the cast shall
return the value that a variable of the casting type would hold after being assigned the expression. If the
expression is not assignment compatible with the casting type, then if the casting type is an enumerated type,
the behavior shall be as described as in 6.19.4, and if the casting type is a bit-stream type, the behavior shall
be as described in 6.24.3.
int'(2.0 * 3.0)
shortint'({8'hFA,8'hCE})
Thus, in the following example, if expressions expr_1 and expr_2 are assignment compatible with data
types cast_t1 and cast_t2, respectively, then
A = cast_t1'(expr_1) + cast_t2'(expr_2);
is the same as
cast_t1 temp1;
cast_t2 temp2;
temp1 = expr_1;
temp2 = expr_2;
A = temp1 + temp2;
Thus, an implicit cast (e.g., temp1 = expr1), if defined, gives the same results as the corresponding
explicit cast (cast_t1'(expr1)).
If the casting type is a constant expression with a positive integral value, the expression in parentheses shall
be padded or truncated to the size specified. It shall be an error if the size specified is zero or negative.
Examples:
17'(x - 2)
parameter P = 17;
parameter Q = 16;
P'(x - 2)
(Q+1)'(x – 2)

<!-- Page 141 -->

IEEE Std The signedness can also be changed.
signed'(x)
The expression inside the cast shall be an integral value when changing the size or signing.
When changing the size, the cast shall return the value that a packed array type with a single [n-1:0]
dimension would hold after being assigned the expression, where n is the cast size. The signedness shall
pass through unchanged, i.e., the signedness of the result shall be the self-determined signedness of the
expression inside the cast. The array elements shall be of type bit if the expression inside the cast is 2-state,
otherwise they shall be of type logic.
When changing the signing, the cast shall return the value that a packed array type with a single [n-1:0]
dimension would hold after being assigned the expression, where n is the number of bits in the expression to
be cast ($bits(expression)). The signedness of the result shall be the signedness specified by the cast
type. The array elements shall be of type bit if the expression inside the cast is 2-state; otherwise, they shall
be of type logic.
NOTE—The $signed() and $unsigned() system functions (see 11.7) return the same results as signed'() and
unsigned'(), respectively.
Examples:
logic [7:0] regA;
logic signed [7:0] regS;
regA = unsigned'(-4); // regA = 8'b11111100
regS = signed'(4'b1100); // regS = -4
An expression may be changed to a constant with a const cast.
const'(x)
When casting an expression as a constant, the type of the expression to be cast shall pass through unchanged.
The only effect is to treat the value as though it had been used to define a const variable of the type of the
expression.
When casting to a predefined type, the prefix of the cast shall be the predefined type keyword. When casting
to a user-defined type, the prefix of the cast shall be the user-defined type identifier.
When a shortreal is converted to an int or to 32 bits using either casting or assignment, its value is
rounded (see 6.12). Therefore, the conversion can lose information. To convert a shortreal to its
underlying bit representation without a loss of information, use $shortrealtobits as defined in 20.5. To
convert from the bit representation of a shortreal value into a shortreal, use $bitstoshortreal as
defined in 20.5.
Structures can be converted to bits preserving the bit pattern. In other words, they can be converted back to
the same value without any loss of information. When unpacked data are converted to the packed
representation, the order of the data in the packed representation is such that the first field in the structure
occupies the MSBs. The effect is the same as a concatenation of the data items (struct fields or array
elements) in order. The type of the elements in an unpacked structure or array shall be valid for a packed
representation in order to be cast to any other type, whether packed or unpacked.
An explicit cast between packed types is not required because they are implicitly cast as integral values, but
a cast can be used by tools to perform stronger type checking.

<!-- Page 142 -->

IEEE Std The following example demonstrates how $bits can be used to obtain the size of a structure in bits (the
$bits system function is discussed in 20.6.2), which facilitates conversion of the structure into a packed
array:
typedef struct {
bit isfloat;
union { int i; shortreal f; } n; // anonymous type
} tagged_st; // named structure
typedef bit [$bits(tagged_st) - 1 : 0] tagbits; // tagged_st defined above
tagged_st a [7:0]; // unpacked array of structures
tagbits t = tagbits'(a[3]); // convert structure to array of bits
a[4] = tagged_st'(t); // convert array of bits back to structure
Note that the bit data type loses x values. If these are to be preserved, the logic type should be used
instead.
The size of a union in bits is the size of its largest member. The size of a logic in bits is 1.
The functions $itor, $rtoi, $bitstoreal, $realtobits, $signed, and $unsigned can also be used
to perform type conversions (see Clause20).
#### 6.24.2 $cast dynamic casting
The $cast system task can be used to assign values to variables that might not ordinarily be valid because
of differing data type. $cast can be called as either a task or a function.
The syntax for $cast is as follows:
function int $cast(data_type dest_variable, data_type source_expression);
or
task $cast(data_type dest_variable, data_type source_expression);
The dest_variable is the variable to which the assignment is made.
The source_expression is the expression that is to be assigned to the destination variable.
Use of $cast as either a task or a function determines how invalid assignments are handled. The assignment
is invalid if the arguments are singular and not cast compatible or the arguments are not singular and not
assignment compatible.
When called as a task, $cast attempts to assign the source expression to the destination variable. If the
assignment is invalid, a run-time error occurs, and the destination variable is left unchanged. No type
checking is done by the compiler.
When called as a function, $cast attempts to assign the source expression to the destination variable and
returns 1 if the assignment is valid. If the assignment is invalid, the function does not make the assignment
and returns 0. When called as a function, $cast will never issue a run-time or compile-time error.
The $cast behavior when applied to class handles is described in 8.16.

<!-- Page 143 -->

IEEE Std For example:
typedef enum {red, green, blue, yellow, white, black} Colors;
Colors col;
$cast(col, 2 + 3);
This example assigns the expression (5 => black) to the enumerated type. Without $cast or a static
compile-time cast operation, this type of assignment is illegal.
The following example shows how to use $cast to check whether an assignment will succeed:
if (! $cast(col, 2 + 8)) // 10: invalid cast
$display("Error in cast");
Alternatively, the preceding examples can be cast using a static cast operation. For example:
col = Colors'(2 + 3);
However, this is a compile-time cast, i.e., a coercion that always succeeds at run time and does not provide
for error checking or warn if the expression lies outside the enumeration values.
Allowing both types of casts gives full control to the user. If users know that certain expressions assigned to
an enumerated variable lie within the enumeration values, the faster static compile-time cast can be used. If
users need to check whether an expression lies within the enumeration values, it is not necessary to write a
lengthy case statement manually. The compiler automatically provides that functionality via the $cast
function. By providing both types of casts, SystemVerilog enables users to balance the trade-offs of
performance and checking associated with each cast type.
NOTE—$cast is similar to the dynamic_cast function available in C++. However, $cast allows users to check
whether the operation will succeed, whereas dynamic_cast always raises a C++ exception.
#### 6.24.3 Bit-stream casting
Type casting can also be applied to unpacked arrays and structs. It is thus possible to convert freely between
bit-stream types using explicit casts. Types that can be packed into a stream of bits are called bit-stream
types. A bit-stream type is a type consisting of the following:
— Any integral, packed, or string type
— Unpacked arrays, structures, or classes of the preceding types
— Dynamically sized arrays (dynamic, associative, or queues) of any of the preceding types
This definition is recursive so that, for example, a structure containing a queue of int is a bit-stream type.
Assuming A is of bit-stream type source_t and B is of bit-stream type dest_t, it is legal to convert A into
B by an explicit cast:
B = dest_t'(A);
The conversion from A of type source_t to B of type dest_t proceeds in two steps:
a) Conversion from source_t to a generic packed value containing the same number of bits as
source_t. If source_t contains any 4-state data, the entire packed value is 4-state; otherwise, it is
2-state.
b) Conversion from the generic packed value to dest_t. If the generic packed value is a 4-state type
and parts of dest_t designate 2-state types, then those parts in dest_t are assigned as if cast to a
2-state.

<!-- Page 144 -->

IEEE Std When a dynamic array, queue, or string type is converted to the packed representation, the item at index
0occupies the MSBs. When an associative array is converted to the packed representation, items are packed
in index-sorted order with the first indexed element occupying the MSBs. An associative array type or class
shall be illegal as a destination type. A class handle with local or protected members shall be illegal as a
source type except when the handle is the current instance this (see 8.11 and 8.18).
Both source_t and dest_t can include one or more dynamically sized data in any position (for example, a
structure containing a dynamic array followed by a queue of bytes). If the source type, source_t, includes
dynamically sized variables, they are all included in the bit stream. If the destination type, dest_t, includes
unbounded dynamically sized types, the conversion process is greedy: compute the size of the source_t,
subtract the size of the fixed-size data items in the destination, and then adjust the size of the first
dynamically sized item in the destination to the remaining size; any remaining dynamically sized items are
left empty.
For the purposes of a bit-stream cast, a string type is considered a dynamic array of bytes.
Regardless of whether the destination type contains only fixed-size items or dynamically sized items, data
are extracted into the destination in left-to-right order. It is thus legal to fill a dynamically sized item with
data extracted from the middle of the packed representation.
If both source_t and dest_t are fixed-size types of different sizes and either type is unpacked, then a cast
generates a compile-time error. If source_t or dest_t contain dynamically sized types, then a difference
in their sizes will issue an error either at compile time or at run time, as soon as it is possible to determine the
size mismatch. For example:
// Illegal conversion from 24-bit struct to 32 bit int - compile-time error
struct {bit[7:0] a; shortint b;} a;
int b = int'(a);
// Illegal conversion from 20-bit struct to int (32 bits) - run-time error
struct {bit a[$]; shortint b;} a = '{'{0,1,0,1}, 67};
int b = int'(a);
// Illegal conversion from int (32 bits) to struct dest_t (25 or 33 bits),
// compile-time error
typedef struct {byte a[$]; bit b;} dest_t;
int a;
dest_t b = dest_t'(a);
Bit-stream casting can be used to convert between different aggregate types, such as two structure types, or
a structure and an array or queue type. This conversion can be useful to model packet data transmission over
serial communication streams. For example, the following code uses bit-stream casting to model a control
packet transfer over a data stream:
typedef struct {
shortint address;
logic [3:0] code;
byte command [2];
} Control;
typedef bit Bits [36:1];
Control p;
Bits stream[$];
p = ... // initialize control packet

<!-- Page 145 -->

IEEE Std stream.push_back(Bits'(p)); // append packet to unpacked queue of Bits
Bits b;
Control q;
b = stream.pop_front(); // get packet (as Bits) from stream
q = Control'(b); // convert packet bits back to a Control packet
The following example uses bit-stream casting to model a data packet transfer over a byte stream:
typedef struct {
byte length;
shortint address;
byte payload[];
byte chksum;
} Packet;
The preceding type defines a generic data packet in which the size of the payload field is stored in the length
field. Following is a function that randomly initializes the packet and computes the checksum.
function Packet genPkt();
Packet p;
void'( randomize( p.address, p.length, p.payload )
with { p.length > 1 && p.payload.size == p.length; } );
p.chksum = p.payload.xor();
return p;
endfunction
The byte stream is modeled using a queue, and a bit-stream cast is used to send the packet over the stream.
typedef byte channel_type[$];
channel_type channel;
channel = {channel, channel_type'(genPkt())};
And the code to receive the packet:
Packet p;
int size;
size = channel[0] + 4;
p = Packet'( channel[0 : size - 1] ); // convert stream to Packet
channel = channel[ size : $ ]; // update the stream so it now
// lacks that packet
### 6.25 Parameterized data types
SystemVerilog provides a way to create parameterized data types. A parameterized data type allows the user
to generically define a data type and then conveniently create many varieties of that data type. When using
such a data type one may provide the parameters that fully define its sets of values and operations. This
allows for only one definition to be written and maintained instead of multiple definitions.
Parameterized data types are implemented through the use of type definitions in parameterized classes (see
8.25). The following example shows how to use type definitions and class parameterization to implement
parameterized data types. The example has one class with three data types. The class may be declared
virtual in order to prevent object construction and enforce its usage only for data type definition.

<!-- Page 146 -->

IEEE Std virtual class C#(parameter type T = logic, parameter SIZE = 1);
typedef logic [SIZE-1:0] t_vector;
typedef T t_array [SIZE-1:0];
typedef struct {
t_vector m0 [2*SIZE-1:0];
t_array m1;
} t_struct;
endclass
Class C contains three data types: t_vector, t_array, and t_struct. Each data type is parameterized by
reusing the class parameters T and SIZE.
module top ();
typedef logic [7:0] t_t0;
C#(t_t0,3)::t_vector v0;
C#(t_t0,3)::t_array a0;
C#(bit,4)::t_struct s0;
endmodule
The top level module first defines a data type t_t0. Data type t_t0 and the constant 3 are then used to
declare variable v0. The number of bits in variable t_vector is determined by the specialized class
parameter value of 3. Data type t_vector is referenced inside class C using the static class scope resolution
operator :: (see 8.23). Similarly for variable a0, the specialized class parameter values of t_t0 and 3,
declare a0 as an unpacked array of 3 elements of type t_t0. Finally, variable s0 is declared as an unpacked
struct whose member data types are defined through the values of specialized class parameter values bit
and 4.

<!-- Page 147 -->

IEEE Std 1800-2023