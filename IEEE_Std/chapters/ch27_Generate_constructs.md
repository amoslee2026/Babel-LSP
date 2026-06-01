# Chapter 27: Generate constructs

## 27. Generate constructs
### 27.1 General
This clause describes the following:
— Loop generate constructs
— Conditional generate constructs
— External names in unnamed generate constructs
### 27.2 Overview
Generate constructs are used to either conditionally or multiply instantiate generate blocks into a model. A
generate block is a collection of one or more module items. A generate block may not contain port
declarations, specify blocks, or specparam declarations. Parameters declared in generate blocks shall be
treated as localparams (see 6.20.4). All other module items, including other generate constructs, are allowed
in a generate block. Generate constructs provide the ability for parameter values to affect the structure of the
design. They also allow for modules with repetitive structure to be described more concisely, and they make
recursive module instantiation possible.
### 27.3 Generate construct syntax
There are two kinds of generate constructs: loops and conditionals. Loop generate constructs allow a single
generate block to be instantiated into a model multiple times. Conditional generate constructs, which
include if-generate and case-generate constructs, instantiate at most one generate block from a set of
alternative generate blocks. The term generate scheme refers to the method for determining which or how
many generate blocks are instantiated. It includes the conditional expressions, case alternatives, and loop
control statements that appear in a generate construct.
Generate schemes are evaluated during elaboration of the design. Although generate schemes use syntax that
is similar to behavioral statements, it is important to recognize that they do not execute at simulation time.
They are evaluated at elaboration time, and the result is determined before simulation begins. Therefore, all
expressions in generate schemes shall be constant expressions, deterministic at elaboration time. For more
details on elaboration, see 3.12.
The elaboration of a generate construct results in zero or more instances of a generate block. An instance of
a generate block is similar in some ways to an instance of a module. It creates a new level of hierarchy. It
brings the objects, behavioral constructs, and module instances within the block into existence. These
constructs act the same as they would if they were in a module brought into existence with a module
instantiation, except that object declarations from the enclosing scope can be referenced directly (see 23.9).
Names in instantiated named generate blocks can be referenced hierarchically as described in 23.6.
The keywords generate and endgenerate may be used in a module to define a generate region. A
generate region is a textual span in the module description where generate constructs may appear. Use of
generate regions is optional. There is no semantic difference in the module when a generate region is used. A
parser may choose to recognize the generate region to produce different error messages for misused generate
construct keywords. Generate regions do not nest, and they may only occur directly within a module. If the
generate keyword is used, it shall be matched by an endgenerate keyword.
The syntax for generate constructs is given in Syntax27-1.

<!-- Page 820 -->

IEEE Std generate_region ::= // from A.4.2
generate { generate_item } endgenerate
loop_generate_construct ::=
for ( genvar_initialization ; genvar_expression ; genvar_iteration )
generate_block
genvar_initialization ::=
[ genvar ] genvar_identifier = constant_expression
genvar_iteration ::=
genvar_identifier assignment_operator genvar_expression
| inc_or_dec_operator genvar_identifier
| genvar_identifier inc_or_dec_operator
conditional_generate_construct ::=
if_generate_construct
| case_generate_construct
if_generate_construct ::=
if ( constant_expression ) generate_block [ else generate_block ]
case_generate_construct ::=
case ( constant_expression ) case_generate_item { case_generate_item } endcase
case_generate_item ::=
constant_expression { , constant_expression } : generate_block
| default [ : ] generate_block
generate_block ::=
generate_item
| [ generate_block_identifier : ] begin [ : generate_block_identifier ]
{ generate_item }
end [ : generate_block_identifier ]
generate_item35 ::=
module_or_generate_item
| interface_or_generate_item
| checker_or_generate_item
module_or_generate_item ::= // from A.1.4
{ attribute_instance } parameter_override
| { attribute_instance } gate_instantiation
| { attribute_instance } udp_instantiation
| { attribute_instance } module_instantiation
| { attribute_instance } module_common_item
module_or_generate_item_declaration ::=
package_or_generate_item_declaration
| genvar_declaration
| clocking_declaration
| default clocking clocking_identifier ;
| default disable iff expression_or_dist ;
module_common_item ::=
module_or_generate_item_declaration
| interface_instantiation
| program_instantiation
| assertion_item
| bind_directive
| continuous_assign

<!-- Page 821 -->

IEEE Std | net_alias
| initial_construct
| final_construct
| always_construct
| loop_generate_construct
| conditional_generate_construct
interface_or_generate_item ::= // from A.1.6
{ attribute_instance } module_common_item
| { attribute_instance } extern_tf_declaration
package_or_generate_item_declaration ::= // from A.1.11
net_declaration
| data_declaration
| task_declaration
| function_declaration
| checker_declaration
| dpi_import_export
| extern_constraint_declaration
| class_declaration
| interface_class_declaration
| class_constructor_declaration
| local_parameter_declaration ;
| parameter_declaration ;
| covergroup_declaration
| assertion_item_declaration
| ;
35) Within an interface_declaration, it shall only be legal for a generate_item to be an interface_or_generate_item.
Within a module_declaration, except when also within an interface_declaration, it shall only be legal for a
generate_item to be a module_or_generate_item. Within a checker_declaration, it shall only be legal for a
generate_item to be a checker_or_generate_item.
Syntax27-1—Syntax for generate constructs (excerpt from AnnexA)
### 27.4 Loop generate constructs
A loop generate construct permits a generate block to be instantiated multiple times using syntax that is
similar to a for loop statement. The loop index shall be declared in a genvar declaration prior to its use in a
loop generate scheme.
The genvar is used as an integer during elaboration to evaluate the generate loop and create instances of the
generate block, but it does not exist at simulation time. A genvar shall not be referenced anywhere other
than in a loop generate scheme.
Both the initialization and iteration assignments in the loop generate scheme shall assign to the same
genvar. The initialization assignment shall not reference the loop index on the right-hand side.
Within the generate block of a loop generate construct, there is an implicit localparam declaration. This is
an integer parameter that has the same name and type as the loop index, and its value within each instance of
the generate block is the value of the loop index at the time the instance was elaborated. This parameter can
be used anywhere within the generate block that a normal parameter with an integer value can be used. It can
be referenced with a hierarchical name.

<!-- Page 822 -->

IEEE Std Because this implicit localparam has the same name as the genvar, any reference to this name inside the
loop generate block will be a reference to the localparam, not to the genvar. As a consequence, it is not
possible to have two nested loop generate constructs that use the same genvar.
Generate blocks in loop generate constructs can be named or unnamed, and they can consist of only one
item, which need not be surrounded by begin-end keywords. Even if the begin-end keywords are absent,
it is still a generate block, which, like all generate blocks, comprises a separate scope and a new level of
hierarchy when it is instantiated.
If the generate block is named, it is a declaration of an array of generate block instances. The index values in
this array are the values assumed by the genvar during elaboration. This can be a sparse array because the
genvar values do not have to form a contiguous range of integers. The array is considered to be declared
even if the loop generate scheme resulted in no instances of the generate block. If the generate block is not
named, the declarations within it cannot be referenced using hierarchical names other than from within the
hierarchy instantiated by the generate block itself.
It shall be an error if the name of a generate block instance array conflicts with any other declaration,
including any other generate block instance array. It shall be an error if the loop generate scheme does not
terminate. It shall be an error if a genvar value is repeated during the evaluation of the loop generate
scheme. It shall be an error if any bit of the genvar is set to x or z during the evaluation of the loop
generate scheme.
Example 1: Examples of legal and illegal generate loops
module mod_a;
genvar i;
// "generate", "endgenerate" keywords are not required
for (i=0; i<5; i=i+1) begin:a
for (i=0; i<5; i=i+1) begin:b
... // error -- using "i" as loop index for
... // two nested generate loops
end
end
endmodule
module mod_b;
genvar i;
logic a;
for (i=1; i<0; i=i+1) begin: a
... // error -- "a" conflicts with name of variable "a"
end
endmodule
module mod_c;
genvar i;
for (i=1; i<5; i=i+1) begin: a
...
end
for (i=10; i<15; i=i+1) begin: a
... // error -- "a" conflicts with name of previous

<!-- Page 823 -->

IEEE Std ... // loop even though indices are unique
end
endmodule
Example 2: A parameterized gray-code–to–binary-code converter module using a loop to generate
continuous assignments
module gray2bin1 (bin, gray);
parameter SIZE = 8; // this module is parameterizable
output [SIZE-1:0] bin;
input [SIZE-1:0] gray;
genvar i;
generate
for (i=0; i<SIZE; i=i+1) begin:bitnum
assign bin[i] = ^gray[SIZE-1:i];
// i refers to the implicitly defined localparam whose
// value in each instance of the generate block is
// the value of the genvar when it was elaborated.
end
endgenerate
endmodule
The models in Example 3 and Example 4 are parameterized modules of ripple adders using a loop to
generate SystemVerilog gate primitives. Example 3 uses a two-dimensional net declaration outside the
generate loop to make the connections between the gate primitives while Example 4 makes the net
declaration inside the generate loop to generate the wires needed to connect the gate primitives for each
iteration of the loop.
Example 3: Generated ripple adder with two-dimensional net declaration outside the generate loop
module addergen1 (co, sum, a, b, ci);
parameter SIZE = 4;
output [SIZE-1:0] sum;
output co;
input [SIZE-1:0] a, b;
input ci;
wire [SIZE :0] c;
wire [SIZE-1:0] t [1:3];
genvar i;
assign c[0] = ci;
// Hierarchical gate instance names are:
// xor gates: bitnum[0].g1 bitnum[1].g1 bitnum[2].g1 bitnum[3].g1
// bitnum[0].g2 bitnum[1].g2 bitnum[2].g2 bitnum[3].g2
// and gates: bitnum[0].g3 bitnum[1].g3 bitnum[2].g3 bitnum[3].g3
// bitnum[0].g4 bitnum[1].g4 bitnum[2].g4 bitnum[3].g4
// or gates: bitnum[0].g5 bitnum[1].g5 bitnum[2].g5 bitnum[3].g5
// Generated instances are connected with
// multidimensional nets t[1][3:0] t[2][3:0] t[3][3:0]
// (12 nets total)
for(i=0; i<SIZE; i=i+1) begin:bitnum
xor g1 ( t[1][i], a[i], b[i]);
xor g2 ( sum[i], t[1][i], c[i]);
and g3 ( t[2][i], a[i], b[i]);
and g4 ( t[3][i], t[1][i], c[i]);

<!-- Page 824 -->

IEEE Std or g5 ( c[i+1], t[2][i], t[3][i]);
end
assign co = c[SIZE];
endmodule
Example 4: Generated ripple adder with net declaration inside the generate loop
module addergen1 (co, sum, a, b, ci);
parameter SIZE = 4;
output [SIZE-1:0] sum;
output co;
input [SIZE-1:0] a, b;
input ci;
wire [SIZE :0] c;
genvar i;
assign c[0] = ci;
// Hierarchical gate instance names are:
// xor gates: bitnum[0].g1 bitnum[1].g1 bitnum[2].g1 bitnum[3].g1
// bitnum[0].g2 bitnum[1].g2 bitnum[2].g2 bitnum[3].g2
// and gates: bitnum[0].g3 bitnum[1].g3 bitnum[2].g3 bitnum[3].g3
// bitnum[0].g4 bitnum[1].g4 bitnum[2].g4 bitnum[3].g4
// or gates: bitnum[0].g5 bitnum[1].g5 bitnum[2].g5 bitnum[3].g5
// Gate instances are connected with nets named:
// bitnum[0].t1 bitnum[1].t1 bitnum[2].t1 bitnum[3].t1
// bitnum[0].t2 bitnum[1].t2 bitnum[2].t2 bitnum[3].t2
// bitnum[0].t3 bitnum[1].t3 bitnum[2].t3 bitnum[3].t3
for(i=0; i<SIZE; i=i+1) begin:bitnum
wire t1, t2, t3;
xor g1 ( t1, a[i], b[i]);
xor g2 ( sum[i], t1, c[i]);
and g3 ( t2, a[i], b[i]);
and g4 ( t3, t1, c[i]);
or g5 ( c[i+1], t2, t3);
end
assign co = c[SIZE];
endmodule
The hierarchical generate block instance names in a multilevel generate loop are shown in Example 5. For
each block instance created by the generate loop, the generate block identifier for the loop is indexed by
adding the “[genvar value]” to the end of the generate block identifier. These names can be used in
hierarchical path names (see 23.6).
Example 5: A multilevel generate loop
parameter SIZE = 2;
genvar i, j, k, m;
generate
for (i=0; i<SIZE; i=i+1) begin:B1 // scope B1[i]
M1 N1(); // instantiates B1[i].N1
for (j=0; j<SIZE; j=j+1) begin:B2 // scope B1[i].B2[j]
M2 N2(); // instantiates B1[i].B2[j].N2

<!-- Page 825 -->

IEEE Std for (k=0; k<SIZE; k=k+1) begin:B3 // scope B1[i].B2[j].B3[k]
M3 N3(); // instantiates
end // B1[i].B2[j].B3[k].N3
end
if (i>0) begin:B4 // scope B1[i].B4
for (m=0; m<SIZE; m=m+1) begin:B5 // scope B1[i].B4.B5[m]
M4 N4(); // instantiates
end // B1[i].B4.B5[m].N4
end
end
endgenerate
// Some examples of hierarchical names for the module instances:
// B1[0].N1 B1[1].N1
// B1[0].B2[0].N2 B1[0].B2[1].N2
// B1[0].B2[0].B3[0].N3 B1[0].B2[0].B3[1].N3
// B1[0].B2[1].B3[0].N3
// B1[1].B4.B5[0].N4 B1[1].B4.B5[1].N4
### 27.5 Conditional generate constructs
The conditional generate constructs, if-generate and case-generate, select at most one generate block from a
set of alternative generate blocks based on constant expressions evaluated during elaboration. The selected
generate block, if any, is instantiated into the model.
Generate blocks in conditional generate constructs can be named or unnamed, and they may consist of only
one item, which need not be surrounded by begin-end keywords. Even if the begin-end keywords are
absent, it is still a generate block, which, like all generate blocks, comprises a separate scope and a new level
of hierarchy when it is instantiated.
Because at most one of the alternative generate blocks is instantiated, it is permissible for there to be more
than one block with the same name within a single conditional generate construct. It is not permissible for
any of the named generate blocks to have the same name as generate blocks in any other conditional or loop
generate construct in the same scope, even if the blocks with the same name are not selected for
instantiation. It is not permissible for any of the named generate blocks to have the same name as any other
declaration in the same scope, even if that block is not selected for instantiation.
If the generate block selected for instantiation is named, then this name declares a generate block instance
and is the name for the scope it creates. Normal rules for hierarchical naming apply. If the generate block
selected for instantiation is not named, it still creates a scope; but the declarations within it cannot be
referenced using hierarchical names other than from within the hierarchy instantiated by the generate block
itself.
If a generate block in a conditional generate construct consists of only one item that is itself a conditional
generate construct and if that item is not surrounded by begin-end keywords, then this generate block is
not treated as a separate scope. The generate construct within this block is said to be directly nested. The
generate blocks of the directly nested construct are treated as if they belong to the outer construct. Therefore,
they can have the same name as the generate blocks of the outer construct, and they cannot have the same
name as any declaration in the scope enclosing the outer construct (including other generate blocks in other
generate constructs in that scope). This allows complex conditional generate schemes to be expressed
without creating unnecessary levels of generate block hierarchy.
The most common use of this would be to create an if–else–if generate scheme with any number of
else–if clauses, all of which can have generate blocks with the same name because only one will be
selected for instantiation. It is permissible to combine if-generate and case-generate constructs in the same

<!-- Page 826 -->

IEEE Std complex generate scheme. Direct nesting applies only to conditional generate constructs nested in
conditional generate constructs. It does not apply in any way to loop generate constructs.
Example 1:
module test;
parameter p = 0, q = 0;
wire a, b, c;
//---------------------------------------------------------
// Code to either generate a u1.g1 instance or no instance.
// The u1.g1 instance of one of the following gates:
// (and, or, xor, xnor) is generated if
// {p,q} == {1,0}, {1,2}, {2,0}, {2,1}, {2,2}, {2, default}
//---------------------------------------------------------
if (p == 1)
if (q == 0)
begin : u1 // If p==1 and q==0, then instantiate
and g1(a, b, c); // AND with hierarchical name test.u1.g1
end
else if (q == 2)
begin : u1 // If p==1 and q==2, then instantiate
or g1(a, b, c); // OR with hierarchical name test.u1.g1
end
// "else" added to end "if (q == 2)" statement
else ; // If p==1 and q!=0 or 2, then no instantiation
else if (p == 2)
case (q)
0, 1, 2:
begin : u1 // If p==2 and q==0,1, or 2, then instantiate
xor g1(a, b, c); // XOR with hierarchical name test.u1.g1
end
default:
begin : u1 // If p==2 and q!=0,1, or 2, then instantiate
xnor g1(a, b, c); // XNOR with hierarchical name test.u1.g1
end
endcase
endmodule
This generate construct will select at most one of the generate blocks named u1. The hierarchical name of
the gate instantiation in that block would be test.u1.g1. When nesting if-generate constructs, the else
always belongs to the nearest if construct.
NOTE—As in the preceding example, an else with a null generate block can be inserted to make a subsequent else
belong to an outer if construct. begin-end keywords can also be used to disambiguate. However, this would violate
the criteria for direct nesting, and an extra level of generate block hierarchy would be created.
Conditional generate constructs make it possible for a module to contain an instantiation of itself. The same
can be said of loop generate constructs, but it is more easily done with conditional generates. With proper
use of parameters, the resulting recursion can be made to terminate, resulting in a legitimate model
hierarchy. Because of the rules for determining top-level modules, a module containing an instantiation of
itself will not be a top-level module.
Example 2: An implementation of a parameterized multiplier module
module multiplier(a,b,product);
parameter a_width = 8, b_width = 8;

<!-- Page 827 -->

IEEE Std localparam product_width = a_width+b_width;
// cannot be modified directly with the defparam
// statement or the module instance statement #
input [a_width-1:0] a;
input [b_width-1:0] b;
output [product_width-1:0] product;
generate
if((a_width < 8) || (b_width < 8)) begin: mult
CLA_multiplier #(a_width,b_width) u1(a, b, product);
// instantiate a CLA multiplier
end
else begin: mult
WALLACE_multiplier #(a_width,b_width) u1(a, b, product);
// instantiate a Wallace-tree multiplier
end
endgenerate
// The hierarchical instance name is mult.u1
endmodule
Example 3: Generate with a case to handle widths less than 3
generate
case (WIDTH)
1: begin: adder // 1-bit adder implementation
adder_1bit x1(co, sum, a, b, ci);
end
2: begin: adder // 2-bit adder implementation
adder_2bit x1(co, sum, a, b, ci);
end
default:
begin: adder // others - carry look-ahead adder
adder_cla #(WIDTH) x1(co, sum, a, b, ci);
end
endcase
// The hierarchical instance name is adder.x1
endgenerate
Example 4: A module of memory dimm
module dimm(addr, ba, rasx, casx, csx, wex, cke, clk, dqm, data, dev_id);
parameter [31:0] MEM_WIDTH = 16, MEM_SIZE = 8; // in mbytes
input [10:0] addr;
input ba, rasx, casx, csx, wex, cke, clk;
input [ 7:0] dqm;
inout [63:0] data;
input [ 4:0] dev_id;
genvar i;
case ({MEM_SIZE, MEM_WIDTH})
{32'd8, 32'd16}: // 8Meg x 16 bits wide
begin: memory
for (i=0; i<4; i=i+1) begin:word16
sms_08b216t0 p(.clk(clk), .csb(csx), .cke(cke),.ba(ba),
.addr(addr), .rasb(rasx), .casb(casx),

<!-- Page 828 -->

IEEE Std .web(wex), .udqm(dqm[2*i+1]), .ldqm(dqm[2*i]),
.dqi(data[15+16*i:16*i]), .dev_id(dev_id));
// The hierarchical instance names are:
// memory.word16[3].p, memory.word16[2].p,
// memory.word16[1].p, memory.word16[0].p,
// and the task memory.read_mem
end
task read_mem;
input [31:0] address;
output [63:0] data;
begin // call read_mem in sms module
word[3].p.read_mem(address, data[63:48]);
word[2].p.read_mem(address, data[47:32]);
word[1].p.read_mem(address, data[31:16]);
word[0].p.read_mem(address, data[15: 0]);
end
endtask
end
{32'd16, 32'd8}: // 16Meg x 8 bits wide
begin: memory
for (i=0; i<8; i=i+1) begin:word8
sms_16b208t0 p(.clk(clk), .csb(csx), .cke(cke),.ba(ba),
.addr(addr), .rasb(rasx), .casb(casx),
.web(wex), .dqm(dqm[i]),
.dqi(data[7+8*i:8*i]), .dev_id(dev_id));
// The hierarchical instance names are
// memory.word8[7].p, memory.word8[6].p,
// ...
// memory.word8[1].p, memory.word8[0].p,
// and the task memory.read_mem
end
task read_mem;
input [31:0] address;
output [63:0] data;
begin // call read_mem in sms module
byte[7].p.read_mem(address, data[63:56]);
byte[6].p.read_mem(address, data[55:48]);
byte[5].p.read_mem(address, data[47:40]);
byte[4].p.read_mem(address, data[39:32]);
byte[3].p.read_mem(address, data[31:24]);
byte[2].p.read_mem(address, data[23:16]);
byte[1].p.read_mem(address, data[15: 8]);
byte[0].p.read_mem(address, data[ 7: 0]);
end
endtask
end
// Other memory cases ...
endcase
endmodule
### 27.6 External names for unnamed generate blocks
Although an unnamed generate block has no name that can be used in a hierarchical name, it needs to have a
name by which external interfaces can refer to it. A name will be assigned for this purpose to each unnamed
generate block as described in the next paragraph.

<!-- Page 829 -->

IEEE Std Each generate construct in a given scope is assigned a number. The number will be 1 for the construct that
appears textually first in that scope and will increase by 1 for each subsequent generate construct in that
scope. All unnamed generate blocks will be given the name “genblk<n>” where <n> is the number
assigned to its enclosing generate construct. If such a name would conflict with an explicitly declared name,
then leading zeros are added in front of the number until the name does not conflict.
NOTE—Each generate construct is assigned its number as described in the previous paragraph even if it does not
contain any unnamed generate blocks.
For example:
module top;
parameter genblk2 = 0;
genvar i;
// The following generate block is implicitly named genblk1
if (genblk2) logic a; // top.genblk1.a
else logic b; // top.genblk1.b
// The following generate block is implicitly named genblk02
// as genblk2 is already a declared identifier
if (genblk2) logic a; // top.genblk02.a
else logic b; // top.genblk02.b
// The following generate block would have been named genblk3
// but is explicitly named g1
for (i = 0; i < 1; i = i + 1) begin : g1 // block name
// The following generate block is implicitly named genblk1
// as the first nested scope inside g1
if (1) logic a; // top.g1[0].genblk1.a
end
// The following generate block is implicitly named genblk4 since
// it belongs to the fourth generate construct in scope "top".
// The previous generate block would have been
// named genblk3 if it had not been explicitly named g1
for (i = 0; i < 1; i = i + 1)
// The following generate block is implicitly named genblk1
// as the first nested generate block in genblk4
if (1) logic a; // top.genblk4[0].genblk1.a
// The following generate block is implicitly named genblk5
if (1) logic a; // top.genblk5.a
endmodule

<!-- Page 830 -->

IEEE Std 1800-2023