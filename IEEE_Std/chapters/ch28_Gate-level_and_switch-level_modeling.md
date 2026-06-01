# Chapter 28: Gate-level and switch-level modeling

## 28. Gate-level and switch-level modeling
### 28.1 General
This clause describes the following:
— Gate and switch primitives
— Logic strength modeling
— Gate and net delays
### 28.2 Overview
This clause describes the syntax and semantics of the built-in primitives of gate- and switch-level modeling
and how a hardware design can be described using these primitives.
There are 14 logic gates and 12 switches predefined in the SystemVerilog to provide the gate- and
switch-level modeling facility. Modeling with logic gates and switches has the following advantages:
— Gates provide a much closer one-to-one mapping between the actual circuit and the model.
— There is no continuous assignment equivalent to the bidirectional transfer gate.
### 28.3 Gate and switch declaration syntax
Syntax28-1 shows the gate and switch declaration syntax.
A gate or a switch instance declaration shall have the following specifications:
— The keyword that names the type of gate or switch primitive
— An optional drive strength
— An optional propagation delay
— An optional identifier that names each gate or switch instance
— An optional range for array of instances
— The terminal connection list
Multiple instances of the one type of gate or switch primitive can be declared as a comma-separated list. All
such instances shall have the same drive strength and delay specification.
gate_instantiation ::= // from A.3.1
cmos_switchtype [ delay3 ] cmos_switch_instance { , cmos_switch_instance } ;
| mos_switchtype [ delay3 ] mos_switch_instance { , mos_switch_instance } ;
| enable_gatetype [ drive_strength ] [ delay3 ] enable_gate_instance { , enable_gate_instance } ;
| n_input_gatetype [ drive_strength ] [ delay2 ] n_input_gate_instance { , n_input_gate_instance } ;
| n_output_gatetype [ drive_strength ] [ delay2 ] n_output_gate_instance { , n_output_gate_instance } ;
| pass_en_switchtype [ delay2 ] pass_enable_switch_instance { , pass_enable_switch_instance } ;
| pass_switchtype pass_switch_instance { , pass_switch_instance } ;
| pulldown [ pulldown_strength ] pull_gate_instance { , pull_gate_instance } ;
| pullup [ pullup_strength ] pull_gate_instance { , pull_gate_instance } ;
cmos_switch_instance ::= [ name_of_instance ] ( output_terminal , input_terminal ,
ncontrol_terminal , pcontrol_terminal )

<!-- Page 831 -->

IEEE Std enable_gate_instance ::= [ name_of_instance ] ( output_terminal , input_terminal , enable_terminal )
mos_switch_instance ::= [ name_of_instance ] ( output_terminal , input_terminal , enable_terminal )
n_input_gate_instance ::= [ name_of_instance ] ( output_terminal , input_terminal { , input_terminal } )
n_output_gate_instance ::= [ name_of_instance ] ( output_terminal { , output_terminal } ,
input_terminal )
pass_switch_instance ::= [ name_of_instance ] ( inout_terminal , inout_terminal )
pass_enable_switch_instance ::= [ name_of_instance ] ( inout_terminal , inout_terminal ,
enable_terminal )
pull_gate_instance ::= [ name_of_instance ] ( output_terminal )
pulldown_strength ::= // from A.3.2
( strength0 , strength1 )
| ( strength1 , strength0 )
| ( strength0 )
pullup_strength ::=
( strength0 , strength1 )
| ( strength1 , strength0 )
| ( strength1 )
enable_terminal ::= expression // from A.3.3
inout_terminal ::= net_lvalue
input_terminal ::= expression
ncontrol_terminal ::= expression
output_terminal ::= net_lvalue
pcontrol_terminal ::= expression
cmos_switchtype ::= cmos | rcmos // from A.3.4
enable_gatetype ::= bufif0 | bufif1 | notif0 | notif1
mos_switchtype ::= nmos | pmos | rnmos | rpmos
n_input_gatetype ::= and | nand | or | nor | xor | xnor
n_output_gatetype ::= buf | not
pass_en_switchtype ::= tranif0 | tranif1 | rtranif1 | rtranif0
pass_switchtype ::= tran | rtran
name_of_instance ::= instance_identifier { unpacked_dimension } // from A.4.1.1
Syntax28-1—Syntax for gate instantiation (excerpt from AnnexA)
#### 28.3.1 The gate type specification
A gate or switch instance declaration shall begin with the keyword that specifies the gate or switch primitive
being used by the instances that follow in the declaration. Table28-1 lists the keywords that shall begin a
gate or a switch instance declaration.
Explanations of the built-in gates and switches shown in Table28-1 begin in 28.4.

<!-- Page 832 -->

IEEE Std Table28-1—Built-in gates and switches
Three-state Bidirectional
n_input gates n_output gates Pull gates MOS switches
gates switches
and buf bufif0 pulldown cmos tran
nand not bufif1 pullup nmos tranif0
nor notif0 pmos tranif1
or notif1 rcmos rtran
xnor rnmos rtranif0
xor rpmos rtranif1
#### 28.3.2 The drive strength specification
An optional drive strength specification shall specify the strength of the logic values on the output terminals
of the gate instance. Only the instances of the gate primitives shown in Table28-2 can have the drive
strength specification.
Table28-2—Valid gate types for strength specifications
and nand buf not pulldown
or nor bufif0 notif0 pullup
xor xnor bufif1 notif1 —
The drive strength specification for a gate instance, with the exception of pullup and pulldown, shall have
a strength1 specification and a strength0 specification. The strength1 specification shall specify the strength
of signals with a logic value 1, and the strength0 specification shall specify the strength of signals with a
logic value 0. The strength specification shall follow the gate type keyword and precede any delay
specification. The strength0 specification can precede or follow the strength1 specification. The strength1
and strength0 specifications shall be separated by a comma and enclosed within a pair of parentheses.
The pullup gate shall have one of the following: no strength specification, only a strength1 specification,
or both strength1 and strength0 specifications. The pulldown gate shall have one of the following: no
strength specification, only a strength0 specification, or both strength1 and strength0 specifications. See
### 28.10 for more details.
The strength1 specification shall be one of the following keywords:
supply1 strong1 pull1 weak1
The strength0 specification shall be one of the following keywords:
supply0 strong0 pull0 weak0
Specifying highz1 as strength1 shall cause the gate or switch to output a logic value z in place of a 1.
Specifying highz0 shall cause the gate to output a logic value z in place of a 0. The strength specifications
(highz0, highz1) and (highz1, highz0) shall be considered invalid.

<!-- Page 833 -->

IEEE Std In the absence of a strength specification, the instances shall have the default strengths strong1 and
strong0.
The following example shows a drive strength specification in a declaration of an open-collector nor gate:
nor (highz1,strong0) n1(out1,in1,in2);
In this example, the nor gate outputs a z in place of a 1.
Logic strength modeling is discussed in more detail in 28.11 through 28.15.
#### 28.3.3 The delay specification
An optional delay specification shall specify the propagation delay through the gates and switches in a
declaration. Gates and switches in declarations with no delay specification shall have no propagation delay.
A delay specification can contain up to three delay values, depending on the gate type. The pullup and
pulldown instance declarations shall not include delay specifications. Delays are discussed in more detail
in 28.16.
#### 28.3.4 The primitive instance identifier
An optional name can be given to a gate or switch instance. If multiple instances are declared as an array of
instances, an identifier shall be used to name the instances.
#### 28.3.5 The range specification
There are many situations when repetitive instances are required. These instances shall differ from each
other only by the index of the vector to which they are connected.
In order to specify an array of instances, the instance name shall be followed by the range specification. The
range shall be specified by two constant expressions, left-hand index (lhi) and right-hand index (rhi),
separated by a colon and enclosed within a pair of square brackets. A [lhi:rhi] range specification shall
represent an array of abs(lhi-rhi)+1 instances. Neither of the two constant expressions are required to be
zero, and lhi is not required to be larger than rhi. If both constant expressions are equal, only one instance
shall be generated.
An array of instances shall have a continuous range. One instance identifier shall be associated with only
one range to declare an array of instances.
The range specification shall be optional. If no range specification is given, a single instance shall be
created.
For example:
The following declaration is illegal:
nand #2 t_nand[0:3] ( ... ), t_nand[4:7] ( ... );
It could be declared correctly as one array of eight instances or as two arrays with unique names of four
elements each, as follows:
nand #2 t_nand[0:7]( ... );
nand #2 x_nand[0:3] ( ... ), y_nand[4:7] ( ... );

<!-- Page 834 -->

IEEE Std #### 28.3.6 Primitive instance connection list
The terminal list describes how the gate or switch connects to the rest of the model. The gate or switch type
can limit these expressions. The connection list shall be enclosed in a pair of parentheses, and the terminals
shall be separated by commas. The output or bidirectional terminals shall always come first in the terminal
list, followed by the input terminals.
The terminal connections for an array of instances shall follow these rules:
— The bit length of each port expression in the declared instance-array shall be compared with the bit
length of each single-instance port or terminal in the instantiated module or primitive.
— For each port or terminal where the bit length of the instance-array port expression is the same as the
bit length of the single-instance port, the instance-array port expression shall be connected to each
single-instance port.
— If the instance-array port expression is an interconnect port or interconnect net expression,
the bit-length of the port expression shall be the same as the instance array length.
— If bit lengths are different, each instance shall get a part-select of the port expression, of a bit length
equal to the instance port bit length. The LSB of the port expression shall be connected to the
instance corresponding to the right-hand index of the array range.
— Too many or too few bits to connect to all the instances shall be considered an error.
An individual instance from an array of instances shall be referenced in the same manner as referencing an
element of an array of logic types.
For example:
Example 1: The following declaration of nand_array declares four instances that can be referenced by
nand_array[1], nand_array[2], nand_array[3], and nand_array[4], respectively.
nand #2 nand_array[1:4]( ... ) ;
Example 2: The two module descriptions that follow are equivalent except for indexed instance names, and
they demonstrate the range specification and connection rules for declaring an array of instances.
module driver (in, out, en);
input [3:0] in;
output [3:0] out;
input en;
bufif0 ar[3:0] (out, in, en); // array of three-state buffers
endmodule
module driver_equiv (in, out, en);
input [3:0] in;
output [3:0] out;
input en;
bufif0 ar3 (out[3], in[3], en); // each buffer declared separately
bufif0 ar2 (out[2], in[2], en);
bufif0 ar1 (out[1], in[1], en);
bufif0 ar0 (out[0], in[0], en);
endmodule

<!-- Page 835 -->

IEEE Std Example 3: The two module descriptions that follow are equivalent except for indexed instance names, and
they demonstrate how different instances within an array of instances are connected when the port sizes do
not match.
module busdriver (busin, bushigh, buslow, enh, enl);
input [15:0] busin;
output [ 7:0] bushigh, buslow;
input enh, enl;
driver busar3 (busin[15:12], bushigh[7:4], enh);
driver busar2 (busin[11:8], bushigh[3:0], enh);
driver busar1 (busin[7:4], buslow[7:4], enl);
driver busar0 (busin[3:0], buslow[3:0], enl);
endmodule
module busdriver_equiv (busin, bushigh, buslow, enh, enl);
input [15:0] busin;
output [ 7:0] bushigh, buslow;
input enh, enl;
driver busar[3:0] (.out({bushigh, buslow}), .in(busin),
.en({enh, enh, enl, enl}));
endmodule
Example 4: This example demonstrates how a series of modules can be chained together. Figure28-1 shows
an equivalent schematic interconnection of DFF instances.
module dffn (q, d, clk);
parameter bits = 1;
input [bits-1:0] d;
output [bits-1:0] q;
input clk ;
DFF dff[bits-1:0] (q, d, clk); // create a row of D flip-flops
endmodule
module MxN_pipeline (in, out, clk);
parameter M = 3, N = 4; // M=width,N=depth
input [M-1:0] in;
output [M-1:0] out;
input clk;
wire [M*(N-1):1] t;
// #(M) redefines the bits parameter for dffn
// create p[1:N] columns of dffn rows (pipeline)
dffn #(M) p[1:N] ({out, t}, {t, in}, clk);
endmodule

<!-- Page 836 -->

IEEE Std p[4] p[3] p[2] p[1]
t[3] t[6] t[9] out[2]
in[2]
dff[2] dff[2] dff[2] dff[2]
in[2:0] t[2] t[5] t[8]
out[2:0]
in[1]
dff[1] dff[1] dff[1] dff[1] out[1]
clk
t[1] t[4] t[7]
in[0]
out[0]
dff[0] dff[0] dff[0] dff[0]
Figure28-1—Schematic diagram of interconnections in array of instances
### 28.4 and, nand, nor, or, xor, and xnor gates
The instance declaration of a multiple input logic gate shall begin with one of the following keywords:
and nand nor or xor xnor
The delay specification shall be zero, one, or two delays. If the specification contains two delays, the first
delay shall determine the output rise delay, the second delay shall determine the output fall delay, and the
smaller of the two delays shall apply to output transitions to x. If only one delay is specified, it shall specify
both the rise delay and the fall delay. If there is no delay specification, there shall be no propagation delay
through the gate.
These six logic gates shall have one output and one or more inputs. The first terminal in the terminal list
shall connect to the output of the gate and all other terminals connect to its inputs.
The truth tables for these gates, showing the result of two input values, appear in Table28-3.

<!-- Page 837 -->

IEEE Std Table28-3—Truth tables for multiple input logic gates
and 0 1 x z or 0 1 x z xor 0 1 x z
0 0 0 0 0 0 0 1 x x 0 0 1 x x
1 0 1 x x 1 1 1 1 1 1 1 0 x x
x 0 x x x x x 1 x x x x x x x
z 0 x x x z x 1 x x z x x x x
nand 0 1 x z nor 0 1 x z xnor 0 1 x z
0 1 1 1 1 0 1 0 x x 0 1 0 x x
1 1 0 x x 1 0 0 0 0 1 0 1 x x
x 1 x x x x x 0 x x x x x x x
z 1 x x x z x 0 x x z x x x x
Versions of these six logic gates having more than two inputs shall have a natural extension, but the number
of inputs shall not alter propagation delays.
The following example declares a two-input and gate:
and a1 (out, in1, in2);
The inputs are in1 and in2. The output is out. The instance name is a1.
### 28.5 buf and not gates
The instance declaration of a multiple output logic gate shall begin with one of the following keywords:
buf not
The delay specification shall be zero, one, or two delays. If the specification contains two delays, the first
delay shall determine the output rise delay, the second delay shall determine the output fall delay, and the
smaller of the two delays shall apply to output transitions to x. If only one delay is specified, it shall specify
both the rise delay and the fall delay. If there is no delay specification, there shall be no propagation delay
through the gate.
These two logic gates shall have one input and one or more outputs. The last terminal in the terminal list
shall connect to the input of the logic gate, and the other terminals shall connect to the outputs of the logic
gate.
Truth tables for these logic gates with one input and one output are shown in Table28-4.

<!-- Page 838 -->

IEEE Std Table28-4—Truth tables for multiple output logic gates
buf not
input output input output
0 0 0 1
1 1 1 0
x x x x
z x z x
The following example declares a two-output buf:
buf b1 (out1, out2, in);
The input is in. The outputs are out1 and out2. The instance name is b1.
### 28.6 bufif1, bufif0, notif1, and notif0 gates
The instance declaration of these three-state logic gates shall begin with one of the following keywords:
bufif0 bufif1 notif1 notif0
These four logic gates model three-state drivers. In addition to logic values 1 and 0, these gates can output z.
The delay specification shall be zero, one, two, or three delays. If the delay specification contains three
delays, the first delay shall determine the rise delay, the second delay shall determine the fall delay, the third
delay shall determine the delay of transitions to z, and the smallest of the three delays shall determine the
delay of transitions to x. If the specification contains two delays, the first delay shall determine the output
rise delay, the second delay shall determine the output fall delay, and the smaller of the two delays shall
apply to output transitions to x and z. If only one delay is specified, it shall specify the delay for all output
transitions. If there is no delay specification, there shall be no propagation delay through the gate.
Some combinations of data input values and control input values can cause these gates to output either of
two values, without a preference for either value (see 28.12.2). The logic tables for these gates include two
symbols representing such unknown results. The symbol L shall represent a result that has a value 0 or z.
The symbol H shall represent a result that has a value 1 or z. Delays on transitions to H or L shall be treated
the same as delays on transitions to x.
These four logic gates shall have one output, one data input, and one control input. The first terminal in the
terminal list shall connect to the output, the second terminal shall connect to the data input, and the third
terminal shall connect to the control input.
Table28-5 presents the logic tables for these gates.

<!-- Page 839 -->

IEEE Std Table28-5—Truth tables for three-state logic gates
CONTROL CONTROL
bufif0 bufif1
0 1 x z 0 1 x z
D 0 0 z L L D 0 z 0 L L
A 1 1 z H H A 1 z 1 H H
T x x z x x T x z x x x
A z x z x x A z z x x x
CONTROL CONTROL
notif0 notif1
0 1 x z 0 1 x z
D 0 1 z H H D 0 z 1 H H
A 1 0 z L L A 1 z 0 L L
T x x z x x T x z x x x
A z x z x x A z z x x x
The following example declares an instance of bufif1:
bufif1 bf1 (outw, inw, controlw);
The output is outw, the input is inw, and the control is controlw. The instance name is bf1.
### 28.7 MOS switches
The instance declaration of a metal-oxide semiconductor (MOS) switch shall begin with one of the
following keywords:
cmos nmos pmos rcmos rnmos rpmos
The cmos and rcmos switches are described in 28.9.
The pmos keyword stands for the P-type metal-oxide semiconductor (PMOS) transistor and the nmos
keyword stands for the N-type metal-oxide semiconductor (NMOS) transistor. PMOS and NMOS transistors
have relatively low impedance between their sources and drains when they conduct. The rpmos keyword
stands for resistive PMOS transistor and the rnmos keyword stands for resistive NMOS transistor. Resistive
PMOS and resistive NMOS transistors have significantly higher impedance between their sources and
drains when they conduct than PMOS and NMOS transistors have. The load devices in static MOS networks
are examples of rpmos and rnmos transistors. These four switches are unidirectional channels for data
similar to the bufif gates.
The delay specification shall be zero, one, two, or three delays. If the delay specification contains three
delays, the first delay shall determine the rise delay, the second delay shall determine the fall delay, the third
delay shall determine the delay of transitions to z, and the smallest of the three delays shall determine the

<!-- Page 840 -->

IEEE Std delay of transitions to x. If the specification contains two delays, the first delay shall determine the output
rise delay, the second delay shall determine the output fall delay, and the smaller of the two delays shall
apply to output transitions to x and z. If only one delay is specified, it shall specify the delay for all output
transitions. If there is no delay specification, there shall be no propagation delay through the switch.
Some combinations of data input values and control input values can cause these switches to output either of
two values, without a preference for either value. The logic tables for these switches include two symbols
representing such unknown results. The symbol L represents a result that has a value 0 or z. The symbol H
represents a result that has a value 1 or z. Delays on transitions to H and L shall be the same as delays on
transitions to x.
These four switches shall have one output, one data input, and one control input. The first terminal in the
terminal list shall connect to the output, the second terminal shall connect to the data input, and the third
terminal shall connect to the control input.
The nmos and pmos switches shall pass signals from their inputs and through their outputs with a change in
the strength of the signal in only one case, as discussed in 28.13. The rnmos and rpmos switches shall
reduce the strength of signals that propagate through them, as discussed in 28.14.
Table28-6 presents the logic tables for these switches.
Table28-6—Truth tables for MOS switches
CONTROL CONTROL
pmos nmos
rpmos rnmos
0 1 x z 0 1 x z
D 0 0 z L L D 0 z 0 L L
A 1 1 z H H A 1 z 1 H H
T x x z x x T x z x x x
A z z z z z A z z z z z
The following example declares a pmos switch:
pmos p1 (out, data, control);
The output is out, the data input is data, and the control input is control. The instance name is p1.
### 28.8 Bidirectional pass switches
The instance declaration of a bidirectional pass switch shall begin with one of the following keywords:
tran tranif1 tranif0
rtran rtranif1 rtranif0
When tranif0, tranif1, rtranif0, or rtranif1 devices are turned off, they shall block signals; and
when they are turned on, they shall pass signals. The tran and rtran devices cannot be turned off; they
shall always pass signals.

<!-- Page 841 -->

IEEE Std The tran and rtran devices shall have terminal lists containing two bidirectional terminals. Both
bidirectional terminals shall unconditionally conduct signals to and from the devices, allowing signals to
pass in either direction through the devices.
The tranif1, tranif0, rtranif1, and rtranif0 devices shall have three items in their terminal lists.
The first two shall be bidirectional terminals that conduct signals to and from the devices, and the third
terminal shall connect to a control input. The control input shall be one of three types: A 4-state net, a 4-state
variable, or a 2-state variable.
The bidirectional terminals of rtran, rtranif1, and rtranif0 devices shall be connected only to scalar
nets or bit-selects of vector nets. tran, tranif1, and tranif0 devices may also be connected to nets of
user-defined net types. A bidirectional switch may not connect nets of two different user-defined net types
or a user-defined net type and a built-in net type. Bidirectional switches connecting user-defined net types
shall be treated as off for x and z control input values, This is different from the behavior of a bidirectional
switch connecting built-in net types (see 4.9.5).
These devices shall have no propagation delay through the bidirectional terminals.
The tran and rtran devices shall not accept delay specifications.
The tranif1, tranif0, rtranif1, and rtranif0 devices may accept delay specifications of one or two
delays. If the specification contains two delays, the first delay shall determine the control input turn-on delay
and the second delay shall determine the control input turn-off delay. For bidirectional switches connecting
built-in net types, the smaller of the two delays shall apply to control input transitions to x and z. If only one
delay is specified, it shall specify both the turn-on and the turn-off delays. If there is no delay specification,
there shall be no turn-on or turn-off delay for the bidirectional pass switch.
For bidirectional switches connecting built-in net types, the tran, tranif0, and tranif1 devices shall
pass signals with an alteration in their strength in only one case, as discussed in 28.13. The rtran,
rtranif0, and rtranif1 devices shall reduce the strength of the signals passing through them according
to rules discussed in 28.14. There shall be no strength reduction in bidirectional switches connecting user-
defined net types.
The following example declares an instance of tranif1:
tranif1 t1 (inout1, inout2, control);
The bidirectional terminals are inout1 and inout2. The control input is control. The instance name is
t1.
### 28.9 CMOS switches
The instance declaration of a CMOS switch shall begin with one of the following keywords:
cmos rcmos
The delay specification shall be zero, one, two, or three delays. If the delay specification contains three
delays, the first delay shall determine the rise delay, the second delay shall determine the fall delay, the third
delay shall determine the delay of transitions to z, and the smallest of the three delays shall determine the
delay of transitions to x. Delays in transitions to H or L are the same as delays in transitions to x. If the
specification contains two delays, the first delay shall determine the output rise delay, the second delay shall
determine the output fall delay, and the smaller of the two delays shall apply to output transitions to x and z.

<!-- Page 842 -->

IEEE Std If only one delay is specified, it shall specify the delay for all output transitions. If there is no delay
specification, there shall be no propagation delay through the switch.
The cmos and rcmos switches shall have a data input, a data output, and two control inputs. In the terminal
list, the first terminal shall connect to the data output, the second terminal shall connect to the data input, the
third terminal shall connect to the n-channel control input, and the last terminal shall connect to the
p-channel control input.
The cmos gate shall pass signals with an alteration in their strength in only one case, as discussed in 28.13.
The rcmos gate shall reduce the strength of signals passing through it according to rules described in 28.14.
The cmos switch shall be treated as the combination of a pmos switch and an nmos switch. The rcmos
switch shall be treated as the combination of an rpmos switch and an rnmos switch. The combined switches
in these configurations shall share data input and data output terminals, but they shall have separate control
inputs.
The equivalence of the cmos gate to the pairing of an nmos gate and a pmos gate is shown in the following
example:
cmos (w, datain, ncontrol, pcontrol);
ncontrol
nmos
is equivalent to:
w datain
pmos
nmos (w, datain, ncontrol);
pmos (w, datain, pcontrol);
pcontrol
### 28.10 pullup and pulldown sources
The instance declaration of a pullup or a pulldown source shall begin with one of the following keywords:
pullup pulldown
A pullup source shall place a logic value 1 on the nets connected in its terminal list. A pulldown source
shall place a logic value 0 on the nets connected in its terminal list.
The signals that these sources place on nets shall have pull strength in the absence of a strength
specification. If there is a strength1 specification on a pullup source or a strength0 specification on a
pulldown source, the signals shall have the strength specified. A strength0 specification on a pullup
source and a strength1 specification on a pulldown source shall be ignored.
There shall be no delay specifications for these sources.
The following example declares two pullup instances:
pullup (strong1) p1 (neta), p2 (netb);
In this example, the p1 instance drives neta and the p2 instance drives netb with strong strength.

<!-- Page 843 -->

IEEE Std ### 28.11 Logic strength modeling
SystemVerilog provides for accurate modeling of signal contention, bidirectional pass gates, resistive MOS
devices, dynamic MOS, charge sharing, and other technology-dependent network configurations by
allowing scalar net signal values to have a full range of unknown values and different levels of strength or
combinations of levels of strength. This multiple-level logic strength modeling resolves combinations of
signals into known or unknown values to represent the behavior of hardware with improved accuracy.
A strength specification shall have the following two components:
a) The strength of the 0 portion of the net value, called strength0, designated as one of the following:
supply0 strong0 pull0 weak0 highz0
b) The strength of the 1 portion of the net value, called strength1, designated as one of the following:
supply1 strong1 pull1 weak1 highz1
The combinations (highz0, highz1) and (highz1, highz0) shall be considered illegal.
Despite this division of the strength specification, it is helpful to consider strength as a property occupying
regions of a continuum in order to predict the results of combinations of signals.
Table28-7 demonstrates the continuum of strengths. The left column lists the keywords used in specifying
strengths. The right column gives correlated strength levels.
Table28-7—Strength levels for scalar net signal values
Strength name Strength level
supply0 7
strong0 6
pull0 5
large0 4
weak0 3
medium0 2
small0 1
highz0 0
highz1 0
small1 1
medium1 2
weak1 3
large1 4
pull1 5
strong1 6
supply1 7

<!-- Page 844 -->

IEEE Std In Table28-7, there are four driving strengths:
supply strong pull weak
Signals with driving strengths shall propagate from gate outputs and continuous assignment outputs.
In Table28-7, there are three charge storage strengths:
large medium small
Signals with the charge storage strengths shall originate in the trireg net type.
It is possible to think of the strengths of signals in Table28-7 as locations on the scale in Figure28-2.
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0 HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-2—Scale of strengths
Discussions of signal combinations later in this clause employ graphics similar to those used in Figure28-2.
If the signal value of a net is known, all of its strength levels shall be in either the strength0 part of the scale
represented by Figure28-2, or all strength levels shall be in its strength1 part. If the signal value of a net is
unknown, it shall have strength levels in both the strength0 and the strength1 parts. A net with a signal value
z shall have a strength level only in one of the 0 subdivisions of the parts of the scale.
### 28.12 Strengths and values of combined signals
In addition to a signal value, a net shall have either a single unambiguous strength level or an ambiguous
strength consisting of more than one level. When signals combine, their strengths and values shall determine
the strength and value of the resulting signal in accordance with the principles in 28.12.1 through 28.12.4.
Nets with user-defined nettypes shall not have strength levels.
Combining signal values for nets with user-defined nettypes shall follow the rules in 6.6.7. Any strength
associated with any drivers of a net with a user-defined nettype shall be ignored.
#### 28.12.1 Combined signals of unambiguous strength
This subclause deals with combinations of signals in which each signal has a known value and a single
strength level.
If two or more signals of unequal strength combine in a wired net configuration, the stronger signal shall
dominate all the weaker drivers and determine the result. The combination of two or more signals of like
value shall result in the same value with the greater of all the strengths. The combination of signals identical
in strength and value shall result in the same signal.
The combination of signals with unlike values and the same strength can have three possible results. Two of
the results occur in the presence of wired logic, and the third occurs in its absence. Wired logic is discussed
in 28.12.4. The result in the absence of wired logic is the subject of Figure28-4 (in 28.12.2).

<!-- Page 845 -->

IEEE Std In Figure28-3, the numbers in parentheses indicate the relative strengths of the signals. The combination of
a pull1 and a strong0 results in a strong0, which is the stronger of the two signals.
Pu1(5)
St0(6)
St0(6)
Su1(7)
Su1(7)
La1(4)
Figure28-3—Combining unequal strengths
#### 28.12.2 Ambiguous strengths: sources and combinations
There are several classifications of signals possessing ambiguous strengths, as follows:
— Signals with known values and multiple strength levels
— Signals with a value x, which have strength levels consisting of subdivisions of both the strength1
and the strength0 parts of the scale of strengths in Figure28-2
— Signals with a value L, which have strength levels that consist of high impedance joined with
strength levels in the strength0 part of the scale of strengths in Figure28-2
— Signals with a value H, which have strength levels that consist of high impedance joined with
strength levels in the strength1 part of the scale of strengths in Figure28-2
Many configurations can produce signals of ambiguous strength. When two signals of equal strength and
opposite value combine, the result shall be a value x, along with the strength levels of both signals and all
the smaller strength levels.
Figure28-4 shows the combination of a weak signal with a value 1 and a weak signal with a value
0yielding a signal with weak strength and a value x.
We1
WeX
We0
Figure28-4—Combination of signals of equal strength and opposite values
This output signal is described in Figure28-5.
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0 HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-5—Weak x signal strength

<!-- Page 846 -->

IEEE Std An ambiguous signal strength can be a range of possible values. An example is the strength of the output
from the three-state drivers with unknown control inputs as shown in Figure28-6.
X
bufif1
St1 StH
X
bufif0
We0 StL
Figure28-6—Bufifs with control inputs of x
The output of the bufif1 in Figure28-6 is a strong H, composed of the range of values described in
Figure28-7.
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0 HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-7—Strong H range of values
The output of the bufif0 in Figure28-6 is a strong L, composed of the range of values described in
Figure28-8.
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0 HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-8—Strong L range of values
The combination of two signals of ambiguous strength shall result in a signal of ambiguous strength. The
resulting signal shall have a range of strength levels that includes the strength levels in its component
signals. The combination of outputs from two three-state drivers with unknown control inputs, shown in
Figure28-9, is an example.

<!-- Page 847 -->

IEEE Std X
PuH
Pu1
X
35X
We0
WeL
Figure28-9—Combined signals of ambiguous strength
In Figure28-9, the combination of signals of ambiguous strengths produces a range that includes the
extremes of the signals and all the strengths between them, as described in Figure28-10.
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0 HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-10—Range of strengths for an unknown signal
The result is a value x because its range includes the values 1 and 0. The number 35, which precedes the x,
is a concatenation of two digits. The first is the digit 3, which corresponds to the highest strength0 level for
the result. The second digit, 5, corresponds to the highest strength1 level for the result.
Switch networks can produce a ranges of strengths of the same value, such as the signals from the upper and
lower configurations in Figure28-11.
Vcc pullup
logic b = x
Pu1(5)
logic a = 1 (6)
logic g = x
56X
Pu0 (5)
logic d = 0
and
We0 (3)
logic d = 0
pulldown ground
Figure28-11—Ambiguous strengths from switch networks

<!-- Page 848 -->

IEEE Std In Figure28-11, the upper combination of a logic type, a gate controlled by a logic type of unspecified
value, and a pullup produces a signal with a value of 1 and a range of strengths (651) described in
Figure28-12.
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0 HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-12—Range of two strengths of a defined value
In Figure28-11, the lower combination of a pulldown, a gate controlled by a logic type of unspecified
value, and an and gate produces a signal with a value 0 and a range of strengths (530) described in
Figure28-13.
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0 HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-13—Range of three strengths of a defined value
When the signals from the upper and lower configurations in Figure28-11 combine, the result is an
unknown with a range (56x) determined by the extremes of the two signals shown in Figure28-14.
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0 HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-14—Unknown value with a range of strengths
In Figure28-11, replacing the pulldown in the lower configuration with a supply0 would change the
range of the result to the range (StX) described in Figure28-15.
The range in Figure28-15 is strong x because it is unknown and the extremes of both its components are
strong. The extreme of the output of the lower configuration is strong because the lower pmos reduces
the strength of the supply0 signal. This modeling feature is discussed in 28.13.

<!-- Page 849 -->

IEEE Std strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0 HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-15—Strong X range
Logic gates produce results with ambiguous strengths as well as three-state drivers. Such a case appears in
Figure28-16. The and gate N1 is declared with highz0 strength, and N2 is declared with weak0 strength.
a=1 StH and ( strong1 , highz0 ) N1(a,b);
N1 and ( strong1 , weak0 ) N2(c,d);
b=X
36X
c=0
N2
d=0 We0
Figure28-16—Ambiguous strength from gates
In Figure28-16, logic type b has an unspecified value; therefore, input to the upper and gate is strong x.
The upper and gate has a strength specification including highz0. The signal from the upper and gate is a
strong H composed of the values as described in Figure28-17.
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0 HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-17—Ambiguous strength signal from a gate
HiZ0 is part of the result because the strength specification for the gate in question specified that strength for
an output with a value 0. A strength specification other than high impedance for the 0 value output results in
a gate output value x. The output of the lower and gate is a weak 0 as described in Figure28-18.

<!-- Page 850 -->

IEEE Std strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0 HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-18—Weak 0
When the signals combine, the result is the range (36x) as described in Figure28-19.
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0 HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-19—Ambiguous strength in combined gate signals
Figure28-19 presents the combination of an ambiguous signal and an unambiguous signal. Such
combinations are the topic of 28.12.3.
#### 28.12.3 Ambiguous strength signals and unambiguous signals
The combination of a signal with unambiguous strength and known value with another signal of ambiguous
strength presents several possible cases. To understand a set of rules governing this type of combination, it is
necessary to consider the strength levels of the ambiguous strength signal separately from each other and
relative to the unambiguous strength signal. When a signal of known value and unambiguous strength
combines with a component of a signal of ambiguous strength, the rules shall be as follows:
a) The strength levels of the ambiguous strength signal that are greater than the strength level of the
unambiguous signal shall remain in the result.
b) The strength levels of the ambiguous strength signal that are smaller than or equal to the strength
level of the unambiguous signal shall disappear from the result, subject to rule c).
c) If the operation of rule a) and rule b) results in a gap in strength levels because the signals are of
opposite value, the signals in the gap shall be part of the result.
The following figures show some applications of the rules.
In Figure28-20, the strength levels in the ambiguous strength signal that are smaller than or equal to the
strength level of the unambiguous strength signal disappear from the result, demonstrating rule b).

<!-- Page 851 -->

IEEE Std strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Combining the two signals above results in the following signal:
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-20—Elimination of strength levels
In Figure28-21, rule a), rule b), and rule c) apply. The strength levels of the ambiguous strength signal that
are of opposite value and lesser strength than the unambiguous strength signal disappear from the result. The
strength levels in the ambiguous strength signal that are less than the strength level of the unambiguous
strength signal, and of the same value, disappear from the result. The strength level of the unambiguous
strength signal and the greater extreme of the ambiguous strength signal define a range in the result.
In Figure28-22, rule a) and rule b) apply. The strength levels in the ambiguous strength signal that are less
than the strength level of the unambiguous strength signal disappear from the result. The strength level of
the unambiguous strength signal and the strength level at the greater extreme of the ambiguous strength
signal define a range in the result.
In Figure28-23, rule a), rule b), and rule c) apply. The greater extreme of the range of strengths for the
ambiguous strength signal is larger than the strength level of the unambiguous strength signal. The result is a
range defined by the greatest strength in the range of the ambiguous strength signal and by the strength level
of the unambiguous strength signal.

<!-- Page 852 -->

IEEE Std strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Combining the two signals above results in the following signal:
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-21—Result showing a range and the elimination of strength levels of two values

<!-- Page 853 -->

IEEE Std strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Combining the two signals above results in the following signal:
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-22—Result showing a range and the elimination of strength levels of one value
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Combining the two signals above results in the following signal:
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-23—A range of both values

<!-- Page 854 -->

IEEE Std #### 28.12.4 Wired logic net types
The net types triand, wand, trior, and wor (see ) shall resolve conflicts when multiple drivers have the
same strength. These net types shall resolve signal values by treating signals as inputs of logic functions.
Consider the combination of two signals of unambiguous strength in Figure28-24.
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
wired AND logic value result: 0
wired OR logic value result: 1
Figure28-24—Wired logic with unambiguous strength signals
The combination of the signals in Figure28-24, using wired and logic, produces a result with the same value
as the result produced by an and gate with the value of the two signals as its inputs. The combination of
signals using wired or logic produces a result with the same value as the result produced by an or gate with
the values of the two signals as its inputs. The strength of the result is the same as the strength of the
combined signals in both cases. If the value of the upper signal changes so that both signals in Figure28-24
possess a value 1, then the results of both types of logic have a value 1.
When ambiguous strength signals combine in wired logic, it is necessary to consider the results of all
combinations of each of the strength levels in the first signal with each of the strength levels in the second
signal, as shown in Figure28-25.

<!-- Page 855 -->

IEEE Std strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Signal 1
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Signal 2
The combinations of strength levels for and logic appear in the following chart:
signal1 signal2 result
strength value strength value strength value
5 0 5 1 5 0
6 0 5 1 6 0
The result is the following signal:
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
The combinations of strength levels for or logic appear in the following chart:
signal1 signal2 result
strength value strength value strength value
5 0 5 1 5 1
6 0 5 1 6 0
The result is the following signal:
strength0 strength1
7 6 5 4 3 2 1 0 0 1 2 3 4 5 6 7
Su0 St0 Pu0 La0 We0 Me0 Sm0 HiZ0HiZ1 Sm1 Me1 We1 La1 Pu1 St1 Su1
Figure28-25—Wired logic and ambiguous strengths

<!-- Page 856 -->

IEEE Std ### 28.13 Strength reduction by nonresistive devices
The nmos, pmos, and cmos switches shall pass the strength from the data input to the output, except that a
supply strength shall be reduced to a strong strength.
The tran, tranif0, and tranif1 switches shall not affect signal strength across the bidirectional
terminals, except that a supply strength shall be reduced to a strong strength.
### 28.14 Strength reduction by resistive devices
The rnmos, rpmos, rcmos, rtran, rtranif1, and rtranif0 devices shall reduce the strength of signals
that pass through them according to Table28-8.
Table28-8—Strength reduction rules
Inputstrength Reducedstrength
Supplydrive Pulldrive
Strongdrive Pulldrive
Pulldrive Weakdrive
Largecapacitor Mediumcapacitor
Weak drive Mediumcapacitor
Mediumcapacitor Smallcapacitor
Smallcapacitor Smallcapacitor
Highimpedance Highimpedance
### 28.15 Strengths of net types
The tri0, tri1, supply0, and supply1 net types(see ) shall generate signals with specific strength
levels. The trireg declaration (see) can specify either of two signal strength levels other than a default
strength level.
#### 28.15.1 tri0 and tri1 net strengths
The tri0 net type models a net connected to a resistive pulldown device. In the absence of an overriding
source, such a signal shall have a value 0 and a pull strength. The tri1 net type models a net connected to
a resistive pullup device. In the absence of an overriding source, such a signal shall have a value 1 and a
pull strength.
#### 28.15.2 trireg strength
The trireg net type models charge storage nodes. The strength of the drive resulting from a trireg net
that is in the charge storage state (that is, a driver charged the net and then went to high impedance) shall be
one of these three strengths: large, medium, or small. The specific strength associated with a particular
trireg net shall be specified by the user in the net declaration. The default shall be medium. The syntax of
this specification is described in 6.7.

<!-- Page 857 -->

IEEE Std #### 28.15.3 supply0 and supply1 net strengths
The supply0 net type models ground connections. The supply1 net type models connections to power
supplies. The supply0 and supply1 net types shall have supply driving strengths.
### 28.16 Gate and net delays
Gate and net delays provide a means of more accurately describing delays through a circuit. The gate delays
specify the signal propagation delay from any gate input to the gate output. Up to three values per output
representing rise, fall, and turn-off delays can be specified (see 28.4 through 28.9).
Net delays refer to the time it takes from any driver on the net changing value to the time when the net value
is updated and propagated further. Up to three delay values per net can be specified.
For both gates and nets, the default delay shall be zero when no delay specification is given. When one delay
value is given, then this value shall be used for all propagation delays associated with the gate or the net.
When two delays are given, the first delay shall specify the rise delay, and the second delay shall specify the
fall delay. The delay when the signal changes to high impedance or to unknown shall be the lesser of the two
delay values.
Forathree-delayspecification
— The first delayreferstothetransitiontothe1value(rise delay).
— The second delay refers to the transition to the 0 value (fall delay).
— The third delay refers to the transition to the high-impedance value.
When a value changes to the unknown (x) value, the delay is the smallest of the three delays. The strength of
the input signal shall not affect the propagation delay from an input to an output.
Table28-9 summarizes the from-to propagation delay choice for the two- and three-delay specifications.
Table28-9—Rules for propagation delays
Delayused if there are
Fromvalue: Tovalue:
2delays 3delays
0 1 d1 d1
0 x min(d1, d2) min(d1, d2, d3)
0 z min(d1, d2) d3
1 0 d2 d2
1 x min(d1, d2) min(d1, d2, d3)
1 z min(d1, d2) d3
x 0 d2 d2
x 1 d1 d1
x z min(d1, d2) d3
z 0 d2 d2

<!-- Page 858 -->

IEEE Std Table28-9—Rules for propagation delays (continued)
Delayused if there are
Fromvalue: Tovalue:
2delays 3delays
z 1 d1 d1
z x min(d1, d2) min(d1, d2, d3)
Example 1: The following is an example of a delay specification with one, two, and three delays.
and #(10) a1 (out, in1, in2); // only one delay
and #(10,12) a2 (out, in1, in2); // rise and fall delays
bufif0 #(10,12,11) b3 (out, in, ctrl); // rise, fall, and turn-off delays
Example 2: The following example specifies a simple latch module with three-state outputs, where
individual delays are given to the gates. The propagation delay from the inputs to the outputs of the module
will be cumulative, and it depends on the signal path through the network.
module tri_latch (qout, nqout, clock, data, enable);
output qout, nqout;
input clock, data, enable;
tri qout, nqout;
not #5 n1 (ndata, data);
nand #(3,5) n2 (wa, data, clock),
n3 (wb, ndata, clock);
nand #(12,15) n4 (q, nq, wa),
n5 (nq, q, wb);
bufif1 #(3,7,13) q_drive (qout, q, enable),
nq_drive (nqout, nq, enable);
endmodule
#### 28.16.1 min:typ:max delays
The syntax for delays on gate primitives (including UDPs; see Clause29), nets, and continuous assignments
shall allow three values each for the rising, falling, and turn-off delays. The minimum, typical, and
maximum values for each delay shall be specified as expressions separated by colons. There shall be no
required relation (e.g., min ≤ typ ≤ max) between the expressions for minimum, typical, and maximum
delays. These can be any three expressions.
The following example shows min:typ:max values for rising, falling, and turn-off delays:
module iobuf (io1, io2, dir);
. . .
bufif0 #(5:7:9, 8:10:12, 15:18:21) b1 (io1, io2, dir);
bufif1 #(6:8:10, 5:7:9, 13:17:19) b2 (io2, io1, dir);
. . .
endmodule
The syntax for delay controls in procedural statements (see 9.4) also allows minimum, typical, and
maximum values. These are specified by expressions separated by colons. The following example illustrates
this concept:

<!-- Page 859 -->

IEEE Std parameter min_hi = 97, typ_hi = 100, max_hi = 107;
logic clk;
always begin
#(95:100:105) clk = 1;
#(min_hi:typ_hi:max_hi) clk = 0;
end
#### 28.16.2 trireg net charge decay
Like all nets, the delay specification in a trireg net declaration can contain up to three delays. The first two
delays shall specify the delay for transition to the 1 and 0 logic states when the trireg net is driven to these
states by a driver. The third delay shall specify the charge decay time instead of the delay in a transition to
the z logic state. The charge decay time specifies the delay between when the drivers of a trireg net turn
off and when its stored charge can no longer be determined.
A trireg net does not need a turn-off delay specification because a trireg net never makes a transition to
the z logic state. When the drivers of a trireg net make transitions from the 1, 0, or x logic states to off,
the trireg net shall retain the previous 1, 0, or x logic state that was on its drivers. The z value shall not
propagate from the drivers of a trireg net to a trireg net. A trireg net can only hold a z logic state
when z is the initial logic state of the trireg net or when the trireg net is forced to the z state with a force
statement (see 10.6.2).
A delay specification for charge decay models a charge storage node that is not ideal, i.e., a charge storage
node whose charge leaks out through its surrounding devices and connections.
The charge decay process and the delay specification for charge decay are described in 28.16.2.1 and
28.16.2.2, respectively.
28.16.2.1 Charge decay process
Charge decay is the cause of transition of a 1 or 0 that is stored in a trireg net to an unknown value (x)
after a specified delay. The charge decay process shall begin when the drivers of the trireg net turn off and
the trireg net starts to hold charge. The charge decay process shall end under the following two
conditions:
a) The delay specified by charge decay time elapses, and the trireg net makes a transition from 1 or
0 to x.
b) The drivers of trireg net turn on and propagate a 1, 0, or x into the trireg net.
28.16.2.2 Delay specification for charge decay time
The third delay in a trireg net declaration shall specify the charge decay time. A three-valued delay
specification in a trireg net declaration shall have the following form:
#(d1, d2, d3) // (rise_delay, fall_delay, charge_decay_time)
The charge decay time specification in a trireg net declaration shall be preceded by a rise and a fall delay
specification.
Example 1: The following example shows a specification of the charge decay time in a trireg net
declaration:
trireg (large) #(0,0,50) cap1;

<!-- Page 860 -->

IEEE Std This example declares a trireg net named cap1. This trireg net stores a large charge. The delay
specifications for the rise delay is 0, the fall delay is 0, and the charge decay time specification is 50 time
units.
Example 2: The next example presents a source description file that contains a trireg net declaration with
a charge decay time specification. Figure28-26 shows an equivalent schematic for the source description.
module capacitor;
logic data, gate;
// trireg declaration with a charge decay time of 50 time units
trireg (large) #(0,0,50) cap1;
nmos nmos1 (cap1, data, gate); // nmos that drives the trireg
initial begin
$monitor("%0d data=%v gate=%v cap1=%v", $time, data, gate, cap1);
data = 1;
// Toggle the driver of the control input to the nmos switch
gate = 1;
#10 gate = 0;
#30 gate = 1;
#10 gate = 0;
#100 $finish;
end
endmodule
gate
data
nmos1
trireg
Figure28-26—Trireg net with capacitance

<!-- Page 861 -->

IEEE Std 1800-2023