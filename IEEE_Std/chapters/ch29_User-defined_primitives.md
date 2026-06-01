# Chapter 29: User-defined primitives

## 29. User-defined primitives
### 29.1 General
This clause describes the following:
— User-defined primitive (UDP) definitions
— Combinational UDPs
— Level-sensitive sequential UDPs
— Edge-sensitive sequential UDPs
— Sequential UDP initialization
— UDP instantiation
### 29.2 Overview
This clause describes a modeling technique to augment the set of predefined gate primitives by designing
and specifying new primitive elements called UDPs. Instances of these new UDPs can be used in exactly the
same manner as the gate primitives to represent the circuit being modeled.
The following two types of behavior can be represented in a UDP:
a) Combinational—modeled by a combinational UDP
b) Sequential—modeled by a sequential UDP
A combinational UDP uses the value of its inputs to determine the next value of its output. A sequential
UDP uses the value of its inputs and the current value of its output to determine the value of its output.
Sequential UDPs provide a way to model sequential circuits such as flip-flops and latches. A sequential
UDP can model both level-sensitive and edge-sensitive behavior.
Each UDP has exactly one output, which can be in one of three states: 0, 1, or x. The high-impedance value
z is not supported. In sequential UDPs, the output always has the same value as the internal state.
The z values passed to UDP inputs shall be treated the same as x values.
### 29.3 UDP definition
UDP definitions are independent of modules; they are at the same level as module definitions in the syntax
hierarchy. They can appear anywhere in the source text, either before or after they are instantiated inside a
module. They shall not appear between the keywords module...endmodule, program...endprogram,
interface...endinterface, or package...endpackage.
Implementations may limit the maximum number of UDP definitions in a model, but they shall allow at
least 256.
The formalsyntax of the UDP definition is given in Syntax29-1.
udp_nonansi_declaration ::= // from A.5.1
{ attribute_instance } primitive udp_identifier ( udp_port_list ) ;
udp_ansi_declaration ::=
{ attribute_instance } primitive udp_identifier ( udp_declaration_port_list ) ;

<!-- Page 862 -->

IEEE Std udp_declaration ::=
udp_nonansi_declaration udp_port_declaration { udp_port_declaration }
udp_body
endprimitive [ : udp_identifier ]
| udp_ansi_declaration
udp_body
endprimitive [ : udp_identifier ]
| extern udp_nonansi_declaration
| extern udp_ansi_declaration
| { attribute_instance } primitive udp_identifier ( . * ) ;
{ udp_port_declaration }
udp_body
endprimitive [ : udp_identifier ]
udp_port_list ::= output_port_identifier , input_port_identifier { , input_port_identifier } // from A.5.2
udp_declaration_port_list ::= udp_output_declaration , udp_input_declaration { , udp_input_declaration }
udp_port_declaration ::=
udp_output_declaration ;
| udp_input_declaration ;
| udp_reg_declaration ;
udp_output_declaration ::=
{ attribute_instance } output port_identifier
| { attribute_instance } output reg port_identifier [ = constant_expression ]
udp_input_declaration ::= { attribute_instance } input list_of_udp_port_identifiers
udp_reg_declaration ::= { attribute_instance } reg variable_identifier
udp_body ::= combinational_body | sequential_body // from A.5.3
combinational_body ::= table combinational_entry { combinational_entry } endtable
combinational_entry ::= level_input_list : output_symbol ;
sequential_body ::= [ udp_initial_statement ] table sequential_entry { sequential_entry } endtable
udp_initial_statement ::= initial output_port_identifier = init_val ;
init_val ::= 1'b0 | 1'b1 | 1'bx | 1'bX | 1'B0 | 1'B1 | 1'Bx | 1'BX | 1 | 0
sequential_entry ::= seq_input_list : current_state : next_state ;
seq_input_list ::= level_input_list | edge_input_list
level_input_list ::= level_symbol { level_symbol }
edge_input_list ::= { level_symbol } edge_indicator { level_symbol }
edge_indicator ::= ( level_symbol level_symbol ) | edge_symbol
current_state ::= level_symbol
next_state ::= output_symbol | -
output_symbol ::= 0 | 1 | x | X
level_symbol ::= 0 | 1 | x | X | ? | b | B
edge_symbol ::= r | R | f | F | p | P | n | N | *
udp_instantiation ::=
udp_identifier [ drive_strength ] [ delay2 ] udp_instance { , udp_instance } ; // from A.5.4
udp_instance ::= [ name_of_instance ] ( output_terminal , input_terminal { , input_terminal } )
Syntax29-1—Syntax for UDPs (excerpt from AnnexA)

<!-- Page 863 -->

IEEE Std #### 29.3.1 UDP header
A UDP definition shall have one of two alternate forms. The first form, udp_nonansi_declaration, shall
begin with the keyword primitive, followed by an identifier, which shall be the name of the UDP. This in
turn shall be followed by a comma-separated list of port names enclosed in parentheses, which shall be
followed by a semicolon. The UDP definition header shall be followed by port declarations and a state table.
The UDP definition shall be terminated by the keyword endprimitive.
The second form, udp_ansi_declaration, shall begin with the keyword primitive, followed by an
identifier, which shall be the name of the UDP. This in turn shall be followed by a comma-separated list of
port declarations enclosed in parentheses, followed by a semicolon. The UDP definition header shall be
followed by a state table. The UDP definition shall be terminated by the keyword endprimitive.
UDPs have multiple input ports and exactly one output port; bidirectional inout ports are not permitted on
UDPs. All ports of a UDP shall be scalar; vector ports are not permitted.
The output port shall be the first port in the port list.
#### 29.3.2 UDP port declarations
UDPs shall contain input and output port declarations. The output port declaration begins with the keyword
output, followed by one output port name. The input port declaration begins with the keyword input,
followed by one or more input port names.
Sequential UDPs shall contain a reg declaration for the output port, either in addition to the output
declaration, when the UDP is declared using the first form of a UDP header, or as part of the
output_declaration. Combinational UDPs cannot contain a reg declaration. The initial value of the output
port can be specified in an initial statement in a sequential UDP (see 29.3.3).
Implementations may limit the maximum number of inputs to a UDP, but they shall allow at least 9 inputs
for sequential UDPs and 10 inputs for combinational UDPs.
When UDPs are discussed from the instantiation point of view, UDP ports are referred to as terminals. This
is because they are consistent with terminals of other primitives, rather than module ports. Wherever
primitive terminals are mentioned, the text shall also apply to UDP terminals.
#### 29.3.3 Sequential UDP initial statement
The sequential UDP initial statement specifies the value of the output port when simulation begins. This
statement begins with the keyword initial. The statement that follows shall be an assignment statement
that assigns a single-bit literal value to the output port.
#### 29.3.4 UDP state table
The state table defines the behavior of a UDP. It begins with the keyword table and is terminated with the
keyword endtable. Each row of the table is terminated by a semicolon.
Each row of the table is created using a variety of characters (see Table29-1), which indicate input values
and output state. Three states—0, 1, and x—are supported. The z state is explicitly excluded from
consideration in UDPs. A number of special characters are defined to represent certain combinations of state
possibilities. These are described in Table29-1.
The order of the input state fields of each row of the state table is taken directly from the port list in the UDP
definition header. It is not related to the order of the input port declarations.

<!-- Page 864 -->

IEEE Std Combinational UDPs have one field per input and one field for the output. The input fields are separated
from the output field by a colon (:). Each row defines the output for a particular combination of the input
values (see 29.4).
Sequential UDPs have an additional field inserted between the input fields and the output field. This
additional field represents the current state of the UDP and is considered equivalent to the current output
value. It is delimited by colons. Each row defines the output based on the current state, particular
combinations of input values, and at most one input transition (see 29.6). A row such as the following one is
illegal:
(01) (10) 0 : 0 : 1 ;
If all input values are specified as x, then the output state shall be specified as x.
It is not necessary to explicitly specify every possible input combination. All combinations of input values
that are not explicitly specified result in a default output state of x.
It shall be illegal to have the same combination of inputs, including edges, specify different output values.
#### 29.3.5 z values in UDP
The z value in a table entry is not supported, and it is considered illegal. The z values passed to UDP inputs
shall be treated the same as x values.
#### 29.3.6 Summary of symbols
To improve the readability and to ease writing of the state table, several special symbols are provided.
Table29-1 summarizes the meaning of all the value symbols that are valid in the table part of a UDP
definition.
Table29-1—UDP table symbols
Symbol Interpretation Comments
0 Logic0 —
1 Logic1 —
x Unknown Permitted in the input and output fields of all
UDPs and in the current state field of
sequential UDPs.
? Iterationof 0,1,andx Not permitted in output field.
b Iterationof 0 and1 Permitted in the input fields of all UDPs and
in the current state field of sequential UDPs.
Not permitted in the output field.
- Nochange Permitted only in the output field of a
sequential UDP.
(vw) Valuechange fromvtow vandwcanbeanyoneof0, 1,x,?,orb,
and are only permitted in the input field.
* Sameas(??) Anyvaluechangeoninput.
r Sameas(01) Risingedgeoninput.
f Sameas(10) Fallingedgeoninput.

<!-- Page 865 -->

IEEE Std Table29-1—UDP table symbols (continued)
Symbol Interpretation Comments
p Iterationof (01),(0x), and (x1) Potentialpositiveedgeonthe input.
n Iterationof (10), (1x), and Potentialnegativeedgeonthe input.
(x0)
### 29.4 Combinational UDPs
In combinational UDPs, the output state is determined solely as a function of the current input states.
Whenever an input state changes, the UDP is evaluated and the output state is set to the value indicated by
the row in the state table that matches all the input states. All combinations of the inputs that are not
explicitly specified will drive the output state to the unknown value x.
The following example defines a multiplexer with two data inputs and a control input:
primitive multiplexer (mux, control, dataA, dataB);
output mux;
input control, dataA, dataB;
table
// control dataA dataB mux
0 1 0 : 1 ;
0 1 1 : 1 ;
0 1 x : 1 ;
0 0 0 : 0 ;
0 0 1 : 0 ;
0 0 x : 0 ;
1 0 1 : 1 ;
1 1 1 : 1 ;
1 x 1 : 1 ;
1 0 0 : 0 ;
1 1 0 : 0 ;
1 x 0 : 0 ;
x 0 0 : 0 ;
x 1 1 : 1 ;
endtable
endprimitive
The first entry in this example can be explained as follows: when control equals 0, dataA equals 1, and
dataB equals 0, then output mux equals 1.
The input combination 0xx (control=0, dataA=x, dataB=x) is not specified. If this combination occurs
during simulation, the value of output port mux will become x.
Using ?, the description of a multiplexer can be abbreviated as follows:
primitive multiplexer (mux, control, dataA, dataB);
output mux;
input control, dataA, dataB;
table
// control dataA dataB mux
0 1 ? : 1 ; // ? = 0 1 x
0 0 ? : 0 ;
1 ? 1 : 1 ;
1 ? 0 : 0 ;

<!-- Page 866 -->

IEEE Std x 0 0 : 0 ;
x 1 1 : 1 ;
endtable
endprimitive
### 29.5 Level-sensitive sequential UDPs
Level-sensitive sequential behavior is represented the same way as combinational behavior, except that the
output is declared to be of type reg and there is an additional field in each table entry. This new field
represents the current state of the UDP. The output field in a sequential UDP represents the next state.
Consider the following example of a latch:
primitive latch (q, ena_, data);
output q; reg q;
input ena_, data;
table
// ena_ data : q : q+
0 1 : ? : 1 ;
0 0 : ? : 0 ;
1 ? : ? : - ; // - = no change
endtable
endprimitive
This description differs from a combinational UDP model in two ways. First, the output identifier q has an
additional reg declaration to indicate that there is an internal state q. The output value of the UDP is always
the same as the internal state. Second, a field for the current state, which is separated by colons from the
inputs and the output, has been added.
### 29.6 Edge-sensitive sequential UDPs
In level-sensitive behavior, the values of the inputs and the current state are sufficient to determine the
output value. Edge-sensitive behavior differs in that changes in the output are triggered by specific
transitions of the inputs. This makes the state table a transition table.
Each table entry can have a transition specification on at most one input. A transition is specified by a pair of
values in parentheses such as (01) or a transition symbol such as r. Entries such as the following are illegal:
(01)(01)0 : 0 : 1 ;
All transitions that do not affect the output shall be explicitly specified. Otherwise, such transitions cause the
value of the output to change to x. All unspecified transitions default to the output value x.
If the behavior of the UDP is sensitive to edges of any input, the desired output state shall be specified for all
edges of all inputs.
The following example describes a rising edge D flip-flop:
primitive d_edge_ff (q, clock, data);
output q; reg q;
input clock, data;
table
// clock data q q+

<!-- Page 867 -->

IEEE Std // obtain output on rising edge of clock
(01) 0 : ? : 0 ;
(01) 1 : ? : 1 ;
(0?) 1 : 1 : 1 ;
(0?) 0 : 0 : 0 ;
// ignore negative edge of clock
(?0) ? : ? : - ;
// ignore data changes on steady clock
? (??) : ? : - ;
endtable
endprimitive
The terms such as (01) represent transitions of the input values. Specifically, (01) represents a transition
from 0 to 1. The first line in the table of the preceding UDP definition is interpreted as follows: when clock
changes value from 0 to 1 and data equals 0, the output goes to 0 no matter what the current state.
The transition of clock from 0 to x with data equal to 0 and current state equal to 1 will result in the output
q going to x.
### 29.7 Sequential UDP initialization
The initial value on the output port of a sequential UDP can be specified with an initial statement that
provides a procedural assignment. The initial statement is optional.
Like initial statements in modules, the initial statement in UDPs begins with the keyword initial. The
valid contents of initial statements in UDPs and the valid left-hand and right-hand sides of their procedural
assignment statements differ from initial statements in modules. A partial list of differences between these
two types of initial statements is described in Table29-2.
Table29-2—Initial statements in UDPs and modules
Initial statements in UDPs Initial statements in modules
Contents limited to one procedural assignment Contents can be one procedural statement of any type
statement or a block statement that contains more than one
procedural statement
The procedural assignment statement shall assign a Procedural assignment statements in initial statements
value to a reg whose identifier matches the identifier can assign values to a variable whose identifier does
of the output port not match the identifier of an output port
The procedural assignment statement shall assign one Procedural assignment statements can assign values of
of the following values: 1'b1, 1'b0, 1'bx, 1, 0 any size, radix, and value
Example 1: The following example shows a sequential UDP that contains an initial statement.
primitive srff (q, s, r);
output q; reg q;
input s, r;
initial q = 1'b1;
table
// s r q q+
1 0 : ? : 1 ;
f 0 : 1 : - ;
0 r : ? : 0 ;
0 f : 0 : - ;

<!-- Page 868 -->

IEEE Std 1 1 : ? : 0 ;
endtable
endprimitive
The output q has an initial value of 1 at the start of the simulation; a delay specification on an instantiated
UDP does not delay the simulation time of the assignment of this initial value to the output. When
simulation starts, this value is the current state in the state table. Delays are not permitted in a UDP initial
statement.
Example 2: The following example and Figure29-1 show how values are applied in a module that
instantiates a sequential UDP with an initial statement.
primitive dff1 (q, clk, d);
input clk, d;
output q; reg q;
initial q = 1'b1;
table
// clk d q q+
r 0 : ? : 0 ;
r 1 : ? : 1 ;
f ? : ? : - ;
? * : ? : - ;
endtable
endprimitive
module dff (q, qb, clk, d);
input clk, d;
output q, qb;
dff1 g1 (qi, clk, d);
buf #3 g2 (q, qi);
not #5 g3 (qb, qi);
endmodule
The UDP dff1 contains an initial statement that sets the initial value of its output to 1. The module dff
contains an instance of UDP dff1.
Figure29-1 shows the schematic of the preceding module and the simulation propagation times of the initial
value of the UDP output.
In Figure29-1, the fanout from the UDP output qi includes nets q and qb. At simulation time 0, qi changes
value to 1. That initial value of qi does not propagate to net q until simulation time 3, and it does not
propagate to net qb until simulation time 5.

<!-- Page 869 -->

IEEE Std module dff
buf g2
q
d
UDP dff1 g1 #3
qi
not g3
clk qb
#5
qi
1
q
1
qb
0 3 5
simulation time
Figure29-1—Module schematic and simulation times of initial value propagation
### 29.8 UDP instances
The syntax for creating a UDP instance is shown in Syntax29-2.
udp_instantiation ::= // from A.5.4
udp_identifier [ drive_strength ] [ delay2 ] udp_instance { , udp_instance } ;
udp_instance ::= [ name_of_instance ] ( output_terminal , input_terminal { , input_terminal } )
name_of_instance ::= instance_identifier { unpacked_dimension } // from A.4.1.1
Syntax29-2—Syntax for UDP instances (excerpt from AnnexA)
Instances of UDPs are specified inside modules in the same manner as gates (see 28.3). The instance name is
optional, just as for gates. The terminal connection order is as specified in the UDP definition. Only two
delays may be specified because z is not supported for UDPs. An optional range may be specified for an
array of UDP instances. The terminal connection rules remain the same as outlined in 28.3.6.
The following example creates an instance of the D-type flip-flop d_edge_ff (defined in 29.6).
module flip;
reg clock, data;
parameter p1 = 10;
parameter p2 = 33;

<!-- Page 870 -->

IEEE Std parameter p3 = 12;
d_edge_ff #p3 d_inst (q, clock, data);
initial begin
data = 1;
clock = 1;
#(20 * p1) $finish;
end
always #p1 clock = ~clock;
always #p2 data = ~data;
endmodule
### 29.9 Mixing level-sensitive and edge-sensitive descriptions
UDP definitions allow a mixing of the level-sensitive and the edge-sensitive constructs in the same table.
When the input changes, the edge-sensitive cases are processed first, followed by level-sensitive cases.
Thus, when level-sensitive and edge-sensitive cases specify different output values, the result is specified by
the level-sensitive case.
For example:
primitive jk_edge_ff (q, clock, j, k, preset, clear);
output q; reg q;
input clock, j, k, preset, clear;
table
// clock jk pc state output/next state
? ?? 01 : ? : 1 ; // preset logic
? ?? *1 : 1 : 1 ;
? ?? 10 : ? : 0 ; // clear logic
? ?? 1* : 0 : 0 ;
r 00 00 : 0 : 1 ; // normal clocking cases
r 00 11 : ? : - ;
r 01 11 : ? : 0 ;
r 10 11 : ? : 1 ;
r 11 11 : 0 : 1 ;
r 11 11 : 1 : 0 ;
f ?? ?? : ? : - ;
b *? ?? : ? : - ; // j and k transition cases
b ?* ?? : ? : - ;
endtable
endprimitive
In this example, the preset and clear logic is level-sensitive. Whenever the preset and clear combination
is 01, the output has value 1. Similarly, whenever the preset and clear combination has value 10, the output
has value 0.
The remaining logic is sensitive to edges of the clock. In the normal clocking cases, the flip-flop is sensitive
to the rising clock edge, as indicated by an r in the clock field in those entries. The insensitivity to the falling
edge of clock is indicated by a hyphen (-) in the output field (see Table29-1) for the entry with an f as the
value of clock. Remember that the desired output for this input transition shall be specified to avoid
unwanted x values at the output. The last two entries show that the transitions in j and k inputs do not
change the output on a steady low or high clock.

<!-- Page 871 -->

IEEE Std ### 29.10 Level-sensitive dominance
Table29-3 shows level-sensitive and edge-sensitive entries in the example from 29.9, their level-sensitive or
edge-sensitive behavior, and a case of input values that each includes.
Table29-3—Mixing of level-sensitive and edge-sensitive entries
Entry Included case Behavior
? ?? 01: ?: 1; 0 00 01: 0: 1; Level-sensitive
f ?? ??: ?: -; f 00 01: 0: 0; Edge-sensitive
The included cases specify opposite next state values for the same input and current state combination. The
level-sensitive included case specifies that when the inputs clock, jk, and pc values are 0, 00, and 01 and
the current state is 0, the output changes to 1. The edge-sensitive included case specifies that when clock
falls from 1 to 0, the other inputs jk and pc are 00 and 01, and the current state is 0, then the output changes
to 0.
When the edge-sensitive case is processed first, followed by the level-sensitive case, the output changes to 1.

<!-- Page 872 -->

IEEE Std 1800-2023