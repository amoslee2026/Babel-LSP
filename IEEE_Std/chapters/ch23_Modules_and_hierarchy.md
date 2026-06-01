# Chapter 23: Modules and hierarchy

## 23. Modules and hierarchy
### 23.1 General
This clause describes the following:
— Formal syntax for module definitions
— Formal syntax for module instantiations
— Nested modules
— Extern modules
— Hierarchical name referencing
— Scope rules
— Parameter redefinition
— Elaboration considerations
— Binding
### 23.2 Module definitions
The module construct is the basic building block of a SystemVerilog design. The primary purpose of a
module is to encapsulate the data, functionality, and timing of digital hardware objects. A module can
represent low-level digital components, such as a simple AND gate, or an entire complex digital system. A
module can represent function and timing at a very detailed level, at a very abstract level, or as a mix of
abstract and detail levels. Modules can instantiate other design elements, thereby creating a design
hierarchy.
A module definition shall be enclosed between the keywords module and endmodule. The identifier
following the keyword module shall be the name of the module being defined. The keyword macromodule
can be used interchangeably with the keyword module to define a module. An implementation may choose
to treat module definitions beginning with the macromodule keyword differently.
#### 23.2.1 Module header definition
The module header defines the following:
— The name of the module
— The port list of the module
— The direction and size of each port
— The type of data passed through each port
— The parameter constants of the module
— A package import list of the module
— The default lifetime (static or automatic) of subroutines defined within the module
There are two styles of module header definitions, the non-ANSI header and the ANSI header.
The non-ANSI header style separates the definition of the module header from the declarations of the
module ports and internal data. The informal syntax of a non-ANSI style module header is as follows:
module_name ( port_list ) ;
parameter_declaration_list
port_direction_and_size_declarations
port_type_declarations

<!-- Page 730 -->

IEEE Std The module header definition is syntactically completed by the semicolon after the closing parenthesis of the
port list. Declarations that define the characteristics of the ports (direction, size, data type, signedness, etc.)
are local definitions within the module.
The ANSI header style makes the declarations of the port characteristics part of the module header (which is
still terminated by a semicolon). The informal general syntax of an ANSI style module header is as follows:
module_name #( parameter_port_list )
( port_direction_and_type_list ) ;
The formal syntax for module declarations is shown in Syntax23-1.
module_declaration ::= // from A.1.2
module_nonansi_header [ timeunits_declaration ] { module_item }
endmodule [ : module_identifier ]
| module_ansi_header [ timeunits_declaration ] { non_port_module_item }
endmodule [ : module_identifier ]
| { attribute_instance } module_keyword [ lifetime ] module_identifier ( . * ) ;
[ timeunits_declaration ] { module_item } endmodule [ : module_identifier ]
| extern module_nonansi_header
| extern module_ansi_header
module_nonansi_header ::=
{ attribute_instance } module_keyword [ lifetime ] module_identifier
{ package_import_declaration } [ parameter_port_list ] list_of_ports ;
module_ansi_header ::=
{ attribute_instance } module_keyword [ lifetime ] module_identifier
{ package_import_declaration }1 [ parameter_port_list ] [ list_of_port_declarations ] ;
module_keyword ::= module | macromodule
timeunits_declaration ::=
timeunit time_literal [ / time_literal ] ;
| timeprecision time_literal ;
| timeunit time_literal ; timeprecision time_literal ;
| timeprecision time_literal ; timeunit time_literal ;
parameter_port_list ::= // from A.1.3
# ( list_of_param_assignments { , parameter_port_declaration } )
| # ( parameter_port_declaration { , parameter_port_declaration } )
| #( )
parameter_port_declaration ::=
parameter_declaration
| local_parameter_declaration
| data_type list_of_param_assignments
| type_parameter_declaration
1) A package_import_declaration in a module_ansi_header, interface_ansi_header, or program_ansi_header shall
be followed by a parameter_port_list or list_of_port_declarations, or both.
Syntax23-1—Module declaration syntax (excerpt from AnnexA)

<!-- Page 731 -->

IEEE Std #### 23.2.2 Port declarations
Ports provide a means of interconnecting a hardware description consisting of modules and primitives. For
example, module A can instantiate module B, using port connections appropriate to module A. These port
names can differ from the names of the internal nets and variables specified in the definition of module B.
A port can be a declaration of an interface, an event, or a variable or net of any allowed data type, including
an array, a structure, or a union.
typedef struct {
bit isfloat;
union { int i; shortreal f; } n;
} tagged_st; // named structure
module mh1 (input var int in1,
input var shortreal in2,
output tagged_st out);
...
endmodule
Implementations may limit the maximum number of ports in a module definition, but the limit shall be at
least 256.
23.2.2.1 Non-ANSI style port declarations
In the non-ANSI style of module header, separate declarations are used for the module list_of_ports and the
declarations of the port characteristics (direction, size, signedness) and the type of data passed through the
ports.
The syntax for the non-ANSI style list_of_ports module header declaration is given in Syntax23-2.
module_nonansi_header ::= // from A.1.2
{ attribute_instance } module_keyword [ lifetime ] module_identifier
{ package_import_declaration } [ parameter_port_list ] list_of_ports ;
list_of_ports ::= ( port { , port } )
port ::=
[ port_expression ]
| . port_identifier ( [ port_expression ] )
port_expression ::=
port_reference
| { port_reference { , port_reference } }
port_reference ::= port_identifier constant_select
Syntax23-2—Non-ANSI style module header declaration syntax (excerpt from AnnexA)
The port reference for each port in the list_of_ports in the module header can be one of the following:
— A simple identifier or escaped identifier
— A bit-select of a vector declared within the module
— A part-select of a vector declared within the module
— A concatenation of any of the above

<!-- Page 732 -->

IEEE Std The port expression is optional because ports can be defined that do not connect to anything internal to the
module. Once a port has been defined, there shall not be another port definition with this same name.
The first type of module port, with only a port_expression, is an implicit port.
The second type is the explicit port. This explicitly specifies the port_identifier used for connecting module
instance ports by name (see 23.3.2.2) and the port_expression that contains identifiers declared inside the
module as described below. Named port connections shall not be used for implicit ports unless the
port_expression is a simple identifier or escaped identifier, which shall be used as the port name.
Each port_identifier in a port_expression in the list of ports for the module declaration shall also be declared
in the body of the module as one of the following port declarations: input, output, inout (bidirectional),
ref, or as an interface port (see Clause25). This is in addition to any net or variable declaration for a
particular port_identifier.
The syntax for non-ANSI style module port_declarations is given in Syntax23-3.
port_declaration ::= // from A.1.3
{ attribute_instance } inout_declaration
| { attribute_instance } input_declaration
| { attribute_instance } output_declaration
| { attribute_instance } ref_declaration
| { attribute_instance } interface_port_declaration
inout_declaration ::= // from A.2.1.2
inout net_port_type list_of_port_identifiers
input_declaration ::=
input net_port_type list_of_port_identifiers
| input variable_port_type list_of_variable_identifiers
output_declaration ::=
output net_port_type list_of_port_identifiers
| output variable_port_type list_of_variable_port_identifiers
ref_declaration ::= ref variable_port_type list_of_variable_identifiers
interface_port_declaration ::=
interface_identifier list_of_interface_identifiers
| interface_identifier . modport_identifier list_of_interface_identifiers
Syntax23-3—Non-ANSI style port declaration syntax (excerpt from AnnexA)
If a port declaration includes a net or variable type, then the port is considered completely declared, and it is
an error for the port to be declared again in a variable or net data type declaration. Because of this, all other
aspects of the port shall be declared in such a port declaration, including the signed and range definitions if
needed.
If a port declaration does not include a net or variable type, then the port can be again declared in a net or
variable declaration. If the net or variable is declared as a vector, the range specification between the two
declarations of a port shall be identical. Once a name is used in a port declaration, it shall not be declared
again in another port declaration or in a data type declaration.

<!-- Page 733 -->

IEEE Std For example:
input aport; // First declaration - okay
input aport; // Error - multiple declaration, port declaration
output aport; // Error - multiple declaration, port declaration
The signed attribute can be attached either to a port declaration or the corresponding net or variable
declaration or to both. If either the port or the net/variable is declared as signed, then the other shall also be
considered signed. It shall be illegal to specify signed for a port declared as an interconnect port.
Nets connected to ports without an explicit net declaration shall be considered unsigned, unless the port is
declared as signed. Other implicit nets (see 6.10) shall be considered unsigned.
Using the non-ANSI header style with a port list followed by separate declarations for each port allows
flexibility on the internal data to be passed through ports.
Example 1: Implicitly named ports connected to internal nets or variables of the same name (non-ANSI style
module header)
module test(a,b,c,d,e,f,g,h);
input [7:0] a; // no explicit net declaration - net is unsigned
input [7:0] b;
input signed [7:0] c;
input signed [7:0] d; // no explicit net declaration - net is signed
output [7:0] e; // no explicit net declaration - net is unsigned
output [7:0] f;
output signed [7:0] g;
output signed [7:0] h; // no explicit net declaration - net is signed
wire signed [7:0] b; // port b inherits signed attribute from net decl.
wire [7:0] c; // net c inherits signed attribute from port
logic signed [7:0] f;// port f inherits signed attribute from logic decl.
logic [7:0] g; // logic g inherits signed attribute from port
endmodule
Example 2: Ports connected to internal nets of a different name (non-ANSI style module header)
module complex_ports ( {c,d}, .e(f) );
// Nets {c,d} receive the first port bits.
// Name 'f' is declared inside the module.
// Name 'e' is defined outside the module.
// Cannot use named port connections of first port.
Example 3: Ports connected to split of internal vector (non-ANSI style module header)
module split_ports (a[7:4], a[3:0]);
// First port is upper 4 bits of 'a'.
// Second port is lower 4 bits of 'a'.
// Cannot use named port connections because
// of part-select port 'a'.
Example 4: Two ports with different names connected to same internal net (non-ANSI style module header)
module same_port (.a(i), .b(i));
// Name 'i' is declared inside the module as an inout port.
// Names 'a' and 'b' are defined for port connections.

<!-- Page 734 -->

IEEE Std Example 5: Explicitly named port connected to concatenation of internal nets or variables (non-ANSI style
module header)
module renamed_concat (.a({b,c}), f, .g(h[1]));
// Names 'b', 'c', 'f', 'h' are defined inside the module.
// Names 'a', 'f', 'g' are defined for port connections.
// Can use named port connections.
Example 6: Two implicitly named ports connected to same internal net (non-ANSI style module header)
module same_input (a,a);
input a; // This is legal. The inputs are tied together.
Example 7: Explicitly named port with mix of input and output directions (non-ANSI style module header)
module mixed_direction (.p({a, e}));
input a; // p contains both input and output directions.
output e;
23.2.2.2 ANSI style list of port declarations
An alternate syntax that minimizes the duplication of data can be used to specify the ports of a module. Each
module shall be declared either entirely with the list_of_ports syntax as described in 23.2.2.1 or entirely with
the list_of_port_declarations syntax as described in this subclause.
The syntax for ANSI style list_of_port_declarations module header is given in Syntax23-4.
module_ansi_header ::= // from A.1.2
{ attribute_instance } module_keyword [ lifetime ] module_identifier
{ package_import_declaration }1 [ parameter_port_list ] [ list_of_port_declarations ] ;
list_of_port_declarations2 ::= // from A.1.3
( [ { attribute_instance } ansi_port_declaration { , { attribute_instance } ansi_port_declaration } ] )
ansi_port_declaration ::=
[ net_port_header | interface_port_header ] port_identifier { unpacked_dimension }
[ = constant_expression ]
| [ variable_port_header ] port_identifier { variable_dimension } [ = constant_expression ]
| [ port_direction ] . port_identifier ( [ expression ] )
net_port_header ::= [ port_direction ] net_port_type
variable_port_header ::= [ port_direction ] variable_port_type
interface_port_header ::=
interface_identifier [ . modport_identifier ]
| interface [ . modport_identifier ]
port_direction ::= input | output | inout | ref
net_port_type ::= // from A.2.2.1
[ net_type ] data_type_or_implicit
| nettype_identifier
| interconnect implicit_data_type
variable_port_type ::= var_data_type
var_data_type ::=
data_type
| var data_type_or_implicit

<!-- Page 735 -->

IEEE Std 1) A package_import_declaration in a module_ansi_header, interface_ansi_header, or program_ansi_header shall
be followed by a parameter_port_list or list_of_port_declarations, or both.
2) The list_of_port_declarations syntax is explained in 23.2.2, which also imposes various semantic restrictions, e.g.,
a ref port shall be of a variable type and an inout port shall not be. It shall be illegal to initialize a port that is not
a variable output port or to specify a default value for a port that is not an input port.
Syntax23-4—ANSI style list_of_port_declarations syntax (excerpt from AnnexA)
Each port declaration provides the complete information about the port. The port’s direction, width, net or
variable type, and signedness are completely described. The port identifier shall not be redeclared, in part or
in full, inside the module body.
The same syntax for input, inout, and output declarations is used in the module header as would be used for
the list of port style declaration, except that the list_of_port_declarations is included in the module header
rather than separately (after the ; that terminates the module header).
As an example, the module named test listed in 23.2.2.1 Example 1 could alternatively be declared as
follows:
module test (
input [7:0] a,
input signed [7:0] b, c, d, // Multiple ports that share all
// attributes can be declared together.
output [7:0] e, // Every attribute of the declaration
// shall be in the one declaration.
output var signed [7:0] f, g,
output signed [7:0] h) ;
// It is illegal to redeclare any ports of
// the module in the body of the module.
endmodule
Generic interface ports (see 25.3.3) cannot be declared using the non-ANSI style list_of_ports syntax (see
23.2.2.1). Generic interface ports can only be declared using the ANSI style list_of_port_declarations
syntax.
module cpuMod(interface d, interface j);
...
endmodule
ANSI style port declarations can be explicitly named, allowing elements of arrays and structures,
concatenations of elements, and assignment pattern expressions of elements declared in a module, interface,
or program to be specified on the port list.
Like explicitly named ports in a module port declaration, port identifiers exist in their own name space for
each port list. When a port item is just a simple port identifier, that identifier is used as both a reference to an
interface item and a port identifier. Once a port identifier has been defined, there shall not be another port
definition with this same name.

<!-- Page 736 -->

IEEE Std For example:
module mymod (
output .P1(r[3:0]),
output .P2(r[7:4]),
ref .Y(x),
input R );
logic [7:0] r;
int x;
...
endmodule
The self-determined type of the port expression becomes the type for the port. The port expression shall not
be considered an assignment-like context. The port expression shall resolve to a legal expression for type of
module port (see 23.3.3). The port expression is optional because ports can be defined that do not connect to
anything internal to the port.
23.2.2.3 Rules for determining port kind, data type, and direction
Within this subclause, the term port kind is used to mean any of the net type keywords, or the keyword var,
which are used to explicitly declare a port of one of these kinds. If these keywords are omitted in a port
declaration, there are default rules for determining the port kind, specified as follows.
Within this subclause, the term data type means both explicit and implicit data type declarations and does
not include unpacked dimensions. An explicit data type declaration uses the data_type syntax. An implicit
data type declaration uses the implicit_data_type syntax and includes only a signedness keyword and/or
packed dimensions. An implicit data type declaration implies a net unless the var keyword is used.
Unpacked dimensions shall not be inherited from the previous port declaration and shall be repeated for each
port with the same dimensions.
If the direction, port kind, and data type are all omitted for the first port in the port list, then all ports shall be
assumed to be non-ANSI style, and port direction and optional type declarations shall be declared after the
port list. Otherwise, all ports shall be assumed to be ANSI style.
For the first port in an ANSI style port list:
— If the direction is omitted, it shall default to inout.
— If the data type is omitted, it shall default to logic, except for interconnect ports, which have no
data type.
— If the port kind is omitted:
• For input and inout ports, the port shall default to a net of default net type. The default net
type can be changed using the `default_nettype compiler directive (see 22.8).
• For output ports, the default port kind depends on how the data type is specified:
— If the data type is omitted or declared with the implicit_data_type syntax, the port kind
shall default to a net of default net type.
— If the data type is declared with the explicit data_type syntax, the port kind shall default to
variable.
• A ref port is always a variable.

<!-- Page 737 -->

IEEE Std Examples:
// Declarations follow the port list because the first port
// does not have a direction, kind, or type specified
module mh_nonansi(x, y);
input wire x;
output tri0 y;
...
endmodule
module mh0 (wire x); // inout wire logic x
module mh1 (integer x); // inout wire integer x
module mh2 (inout integer x); // inout wire integer x
module mh3 ([5:0] x); // inout wire logic [5:0] x
module mh4 (var x); // ERROR: direction defaults to inout,
// which cannot be var
module mh5 (input x); // input wire logic x
module mh6 (input var x); // input var logic x
module mh7 (input var integer x); // input var integer x
module mh8 (output x); // output wire logic x
module mh9 (output var x); // output var logic x
module mh10(output signed [5:0] x); // output wire logic signed [5:0] x
module mh11(output integer x); // output var integer x
module mh12(ref [5:0] x); // ref var logic [5:0] x
module mh13(ref x [5:0]); // ref var logic x [5:0]
For subsequent ports in an ANSI style port list:
— If the direction, port kind and data type are all omitted, then they shall be inherited from the previous
port. If the previous port was an interconnect port, this port shall also be an interconnect port.
Otherwise:
— If the direction is omitted, it shall be inherited from the previous port.
— If the port kind is omitted, it shall be determined as previously specified.
— If the data type is omitted, it shall default to logic, except for interconnect ports, which have no
data type.
Examples:
module mh14(wire x, y[7:0]); // inout wire logic x
// inout wire logic y[7:0]
module mh15(integer x, signed [5:0] y); // inout wire integer x
// inout wire logic signed [5:0] y

<!-- Page 738 -->

IEEE Std module mh16([5:0] x, wire y); // inout wire logic [5:0] x
// inout wire logic y
module mh17(input var integer x, wire y); // input var integer x
// input wire logic y
module mh18(output var x, input y); // output var logic x
// input wire logic y
module mh19(output signed [5:0] x, integer y);
// output wire logic signed [5:0] x
// output var integer y
module mh20(ref [5:0] x, y); // ref var logic [5:0] x
// ref var logic [5:0] y
module mh21(ref x [5:0], y); // ref var logic x [5:0]
// ref var logic y
The preceding rules do not apply to explicit port declarations (i.e., of the form .port_identifier(expression),
see 23.2.2.2). Explicit port declarations shall inherit only the port direction from the preceding port (if not
explicitly specified), but not other properties. The data type of the port is the self-determined data type of the
expression.
A port declaration that immediately follows an explicit port declaration shall inherit only the port direction
(if not explicitly specified) from the explicit port declaration, but not other properties. The port kind and data
type of such a port shall be determined using the same rules as for the first port in the port list.
Example:
module mh22 (input wire integer p_a, .p_b(s_b), p_c);
logic [5:0] s_b;
In this example, port p_a is fully declared. p_b is an explicitly named port that inherits only the direction
input from port p_a. Its data type is that of s_b. Port p_c inherits only the direction from p_b, and defaults
to the net port kind and to the logic data type.
23.2.2.4 Default port values
A module declaration may specify a default value for each singular input port. These default values shall be
constant expressions evaluated in the scope of the module where they are defined, not in the scope of the
instantiating module.
The informal syntax to declare a default input port value in a module is as follows:
module module_name (
...,
[ input ] [ type ] port_identifier = constant_expression,
... ) ;
Defaults can be specified only for input ports and only in ANSI style declarations. A default shall not be
specified for a port declared as interconnect (an interconnect port).
When the module is instantiated, input ports with default values can be omitted from the instantiation, and
the compiler shall insert the corresponding default values. If a connection is not specified for an input port

<!-- Page 739 -->

IEEE Std and the port does not have a default value, then, depending on the connection style (ordered list, named
connections, implicit named connections, or implicit .* connections), the port shall either be left
unconnected or result in an error, as discussed in 23.3.2.1 through 23.3.2.4.
The following example illustrates default port semantics and parameter scope resolution:
parameter logic [7:0] My_DataIn = 8'hFF;
module bus_conn (
output logic [7:0] dataout,
input [7:0] datain = My_DataIn);
assign dataout = datain;
endmodule
module bus_connect1 (
output logic [31:0] dataout,
input [ 7:0] datain);
parameter logic [7:0] My_DataIn = 8'h00;
bus_conn bconn0 (dataout[31:24], 8'h0F);
// Constant literal overrides default in bus_conn definition
bus_conn bconn1 (dataout[23:16]);
// Omitted port for datain, default parameter value 8'hFF in
// bus_conn used
bus_conn bconn2 (dataout[15:8], My_DataIn);
// The parameter value 8'h00 from the instantiating scope is used
bus_conn bconn3 (dataout[7:0]);
endmodule
#### 23.2.3 Parameterized modules
Port declarations can be based on parameter declarations. Parameter types can be redefined for each instance
of a module, providing a means of customizing the characteristics of each instance of a module.
Example 1: Parameterized module declaration using non-ANSI style module header:
module generic_fifo (clk, read, write, reset, out, full, empty );
parameter MSB=3, LSB=0, DEPTH=4; // these parameters can be redefined
input [MSB:LSB] in;
input clk, read, write, reset;
output [MSB:LSB] out;
output full, empty;
wire [MSB:LSB] in;
wire clk, read, write, reset;
logic [MSB:LSB] out;
logic full, empty;
...
endmodule
Example 2: Parameterized module declaration using ANSI style module header:
module generic_fifo
#(parameter MSB=3, LSB=0, DEPTH=4) // these parameters can be redefined

<!-- Page 740 -->

IEEE Std (input wire [MSB:LSB] in,
input wire clk, read, write, reset,
output logic [MSB:LSB] out,
output logic full, empty );
...
endmodule
Parameter redefinition is discussed in 23.10.
The order used in defining the list of parameters can be significant when instantiating the module (see
23.10.2.1).
Example 3: Parameterized module header with local parameters using ANSI style header:
module generic_decoder
#(num_code_bits = 3, localparam num_out_bits = 1 << num_code_bits)
(input [num_code_bits-1:0] A, output reg [num_out_bits-1:0] Y);
#### 23.2.4 Module contents
The module definition can contain zero or more module items. The syntax is shown in Syntax23-5.
module_common_item ::= // from A.1.4
module_or_generate_item_declaration
| interface_instantiation
| program_instantiation
| assertion_item
| bind_directive
| continuous_assign
| net_alias
| initial_construct
| final_construct
| always_construct
| loop_generate_construct
| conditional_generate_construct
module_item ::=
port_declaration ;
| non_port_module_item
module_or_generate_item ::=
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
non_port_module_item ::=
generate_region
| module_or_generate_item

<!-- Page 741 -->

IEEE Std | specify_block
| { attribute_instance } specparam_declaration
| program_declaration
| module_declaration
| interface_declaration
| timeunits_declaration3
parameter_override ::= defparam list_of_defparam_assignments ;
3) A timeunits_declaration shall be legal as a non_port_module_item, non_port_interface_item,
non_port_program_item, or package_item only if it repeats and matches a previous timeunits_declaration within
the same time scope.
Syntax23-5—Module item syntax (excerpt from AnnexA)
The module items define what constitutes a module and can include many different types of declarations and
definitions, which are described in various clauses throughout this document.
### 23.3 Module instances (hierarchy)
A module can be instantiated in two ways, hierarchical or top level. Top-level modules are implicitly
instantiated (see 23.3.1). Hierarchical modules can be instantiated explicitly (see 23.3.2) or implicitly as a
nested module (see 23.4).
#### 23.3.1 Top-level modules and $root
Top-level modules are modules that are included in the SystemVerilog source text, but do not appear in any
module instantiation statement, as described in 23.3.2. This applies even if the module instantiation appears
in a generate block that is not itself instantiated (see 27.3). A design shall contain at least one top-level
module. A top-level module is implicitly instantiated once, and its instance name is the same as the module
name. Such an instance is called a top-level instance.
The name $root is used to unambiguously refer to a top-level instance or to an instance path starting from
the root of the instantiation tree. $root is the root of the instantiation tree.
For example:
$root.A.B // item B within top instance A
$root.A.B.C // item C within instance B within instance A
$root allows explicit access to the top of the instantiation tree. This is useful to disambiguate a local path
(which takes precedence) from the rooted path. If $root is not specified, a hierarchical path is ambiguous.
For example, A.B.C can mean the local A.B.C or the top-level A.B.C (assuming there is an instance A that
contains an instance B at both the top level and in the current module). The ambiguity is resolved by giving
priority to the local scope and thereby preventing access to the top-level path. $root allows explicit access
to the top level in those cases in which the name of the top-level module is insufficient to uniquely identify
the path.
#### 23.3.2 Module instantiation syntax
Explicit module instantiation creates a hierarchical instance of a module. The syntax for explicit module
instantiation is as follows in Syntax23-6.

<!-- Page 742 -->

IEEE Std module_instantiation ::= // from A.4.1.1
module_identifier [ parameter_value_assignment ] hierarchical_instance { , hierarchical_instance };
parameter_value_assignment ::= # ( [ list_of_parameter_value_assignments ] )
list_of_parameter_value_assignments ::=
ordered_parameter_assignment { , ordered_parameter_assignment }
| named_parameter_assignment { , named_parameter_assignment }
ordered_parameter_assignment ::= param_expression
named_parameter_assignment ::= . parameter_identifier ( [ param_expression ] )
hierarchical_instance ::= name_of_instance ( [ list_of_port_connections ] )
name_of_instance ::= instance_identifier { unpacked_dimension }
list_of_port_connections34 ::=
ordered_port_connection { , ordered_port_connection }
| named_port_connection { , named_port_connection }
ordered_port_connection ::= { attribute_instance } [ expression ]
named_port_connection ::=
{ attribute_instance } . port_identifier [ ( [ expression ] ) ]
| { attribute_instance } . *
param_expression ::= mintypmax_expression | data_type | $ // from A.8.3
34) The . * token pair shall appear at most once in a list of port connections.
Syntax23-6—Module instance syntax (excerpt from AnnexA)
Hierarchical instantiation allows more than one instance of the same module. The module name can be a
module previously declared or one declared later. Parameter assignments can be named or ordered. Port
connections can be named, ordered, or implicitly connected. They can be nets, variables, or other kinds of
interfaces, events, or expressions. See 23.3.3 for the connection rules.
The instantiations of modules can contain a range specification. This allows an array of instances to be
created. The array of instances is described in 28.3.5 (also see 23.3.3.5). The syntax and semantics of arrays
of instances defined for gates and primitives apply for modules as well.
The list of port connections shall be provided only for modules defined with ports. The parentheses shall be
required on all module instantiations, even when the instantiated module does not have ports.
One or more module instances (identical copies of a module) can be specified in a single module
instantiation statement. For example, three instances of a module called ffnand can be instantiated as:
ffnand ff1 (.q(), .qbar(out1), .clear(in1), .preset(in2)),
ff2 (.q(), .qbar(out2), .clear(in2), .preset(in1), .q());
ff3 (.q(out3), .qbar(), .clear(in1), .preset(in2));
Connections can be made to module instances in the following four ways:
— Positional connections by port order (see 23.3.2.1)
— Named port connections using fully explicit connections (see 23.3.2.2)
— Named port connections using implicit connections (see 23.3.2.3)
— Named port connections using a wildcard port name (see 23.3.2.4)

<!-- Page 743 -->

IEEE Std Positional and named module port connections shall not be mixed in the same module instantiation;
connections to the ports of a particular module instance shall be all by order or all by name. The three forms
of named port connections can be mixed.
An ALU accumulator (alu_accum) example module is used to illustrate these four forms of port
connections. The ALU accumulator includes instantiations of an ALU module, an accumulator register
(accum) module, and a sign-extension (xtend) module. The module headers for the three instantiated
modules are shown in the following example code:
parameter logic [7:0] My_DataIn = 8'hFF;
module alu (
output reg [7:0] alu_out,
output reg zero,
input [7:0] ain, bin,
input [2:0] opcode);
// RTL code for the alu module
endmodule
module accum (
output reg [7:0] dataout,
input [7:0] datain = My_DataIn,
input clk, rst_n = 1'b1);
// RTL code for the accumulator module
endmodule
module xtend (
output reg [7:0] dout,
input din,
input clk, rst = 1'b0 );
// RTL code for the sign-extension module
endmodule
23.3.2.1 Connecting module instance ports by ordered list
One method of making the connection between the port expressions listed in a module instantiation and the
ports declared within the instantiated module is the ordered list; that is, the port expressions listed for the
module instance shall be in the same order as the ports listed in the module declaration.
A connection can be a simple reference to a variable or a net identifier, an expression, or a blank. An
expression can be used for supplying a value to a module input port. A blank port connection shall represent
the situation where the port is not to be connected. However, if a port connection is omitted (indicated by a
missing argument in the comma-separated list) to an input port with a default value, the default value shall
be used.
Examples of module instantiations with positional port connections and default values are shown in the
following alu_accum1 module example:
module alu_accum1 (
output [15:0] dataout,
input [7:0] ain, bin,
input [2:0] opcode,
input clk, rst_n, rst);
wire [7:0] alu_out;
alu alu (alu_out, , ain, bin, opcode); // zero output is unconnected

<!-- Page 744 -->

IEEE Std accum accum (dataout[7:0], alu_out, clk, rst_n);
xtend xtend (dataout[15:8], alu_out[7], clk); // rst gets default
// value 1'b0
endmodule
Refer to 23.3.3 for additional port connection rules.
23.3.2.2 Connecting module instance ports by name
The second way to connect module ports consists of explicitly linking the two names for each side of the
connection: the port declaration name from the module declaration to the expression, i.e., the name used in
the module declaration, followed by the name used in the instantiating module. This compound name is then
placed in the list of module connections. The informal syntax for named port connections of a module with
two ports is as follows:
module_name instance_name ( .port_name(expression), .port_name(expression) );
The port_name shall be the name specified in the module declaration. The port name cannot be a bit-select,
a part-select, or a concatenation of ports.
The port expression can be any valid expression. The port expression is optional so that the instantiating
module can document the existence of the port without connecting it to anything. The parentheses are
required.
If an input port with a specified default value has an explicit empty named port connection [i.e.,
.port_name()], then the port shall be left unconnected and the default value shall not be used. When
connecting ports by name, an unconnected port can also be indicated by omitting it in the port list providing
there is no default value.
Examples of module instantiations with named port connections and default values are shown in the
following alu_accum2 module example:
module alu_accum2 (
output [15:0] dataout,
input [7:0] ain, bin,
input [2:0] opcode,
input clk, rst_n, rst);
wire [7:0] alu_out;
alu alu (.alu_out(alu_out), .zero(),
.ain(ain), .bin(bin), .opcode(opcode));
// zero output is unconnected
accum accum (.dataout(dataout[7:0]), .datain(alu_out),
.clk(clk));
// rst_n is not in the port list and so gets default value 1'b1
xtend xtend (.dout(dataout[15:8]), .din(alu_out[7]),
.clk(clk), .rst() );
// rst has a default value, but has an empty port connection,
// therefore it is left unconnected
endmodule
Because the connections in the preceding example are made by name, the order in which they appear is
irrelevant.

<!-- Page 745 -->

IEEE Std Multiple module instance port connections are not allowed. The following example instantiation is illegal:
module test;
A ia ( .i (a), .i (b), // illegal connection of input port twice
.o (c), .o (d), // illegal connection of output port twice
.e (e), .e (f)); // illegal connection of inout port twice
endmodule
module A (input i, output o, inout e);
...
endmodule
Refer to 23.3.3 for additional port connection rules.
23.3.2.3 Connecting module instance using implicit named port connections (.name)
SystemVerilog can implicitly instantiate ports using a .name syntax if the instance port name matches the
connecting port name and their data types are equivalent.
This eliminates the requirement to list an identifier name twice when the port name and expression name are
the same, while still listing all of the ports of the instantiated module for documentation purposes.
If a signal of the same name does not exist in the instantiating module, the port connection shall not create an
implicit net declaration and an error shall be issued, even if the port has a specified default value. The
purpose of using default values is to implicitly assign constant expressions to otherwise unconnected input
ports. If an implicit .name port connection is used, it is assumed that the coder’s intent is to connect this port
value and not use the default value. To leave a port with a default value unconnected, empty parentheses
shall be used after .name, i.e., .name().
In the following alu_accum3 example, all of the ports of the instantiated alu module match the names of
the declarations connected to the ports, except for the unconnected zero port, which is listed using a named
port connection, showing that the port is unconnected. Implicit .name port connections are made for all
name and equivalent type matching connections on the instantiated module.
In the same alu_accum3 example, the accum module has an 8-bit port called dataout that is connected to
a 16-bit bus called dataout. Because the internal and external sizes of dataout do not match, the port has
to be connected by name, showing which bits of the 16-bit bus are connected to the 8-bit port. The datain
port on the accum is connected to a bus by a different name (alu_out); therefore, this port is also connected
by name. clk is connected using an implicit .name port connection while the rst_n port is left unconnected
because it uses empty parentheses. Also in the same alu_accum3 example, the xtend module has an 8-bit
output port called dout and a 1-bit input port called din. Because neither of these port names matches the
names (or sizes) of the connecting declarations, both are connected by name. clk is connected using an
implicit .name port connection, but the rst signal does not exist in the instantiation module and hence will
result in an error even though a default port value exists.
module alu_accum3 (
output [15:0] dataout,
input [7:0] ain, bin,
input [2:0] opcode,
input clk, rst_n);
wire [7:0] alu_out;
alu alu (.alu_out, .zero(), .ain, .bin, .opcode);
accum accum (.dataout(dataout[7:0]), .datain(alu_out), .clk, .rst_n());
xtend xtend (.dout(dataout[15:8]), .din(alu_out[7]), .clk, .rst);
// Error: rst does not exist in the instantiation module

<!-- Page 746 -->

IEEE Std endmodule
A .port_identifier port connection is semantically equivalent to the named port connection
.port_identifier(port_identifier) with the following exceptions:
— The port connection shall not create an implicit net declaration.
— The declarations on each side of the port connection shall have equivalent data types.
— An implicit .port_identifier port connection between nets of two dissimilar net types shall issue an
error when it is a warning in an explicit named port connection as required by 23.3.3.7.
It shall be an error if the name port_identifier has not been declared (explicitly or implicitly) or
imported from a package (by explicit or wildcard import) prior to the .port_identifier implicit port
connection.
23.3.2.4 Connecting module instances using wildcard named port connections ( .*)
SystemVerilog can implicitly instantiate ports using a .* wildcard syntax for all ports where the instance
port name matches the connecting port name and their data types are equivalent. This eliminates the
requirement to list any port where the name and type of the connecting declaration match the name and
equivalent type of the instance port. This implicit port connection style is used to indicate that all port names
and types match the connections where emphasis is placed only on the exception ports. A named port
connection can be mixed with a .* connection to override a port connection to a different expression or to
leave a port unconnected. The implicit .* port connection syntax can greatly facilitate rapid block-level
testbench generation where all of the testbench declarations are chosen to match the instantiated module port
names and types.
An implicit .* port connection is semantically equivalent to an implicit .name port connection for every
port declared in the instantiated module, with the following two exceptions:
1) If an instantiation uses a .name port connection, the default value to that port shall not be used. If the
name does not exist in the instantiating scope, an error shall occur. When using .*, however, the
default value shall be used if the name does not exist in the instantiating scope. In this case, if an
unconnected port is truly needed for a specific instantiation, then .name() can be used in addition to
.*.
2) Using .* does not create a sufficient reference for a wildcard import of a name from a package. A
named or implicit .name connection can be mixed with a .* connection to create a sufficient
reference for a wildcard import of a name from a package.
In the following alu_accum4 example, all of the ports of the instantiated alu module match the names of
the variables connected to the ports, except for the unconnected zero port, which is listed using a named
port connection, showing that the port is unconnected. The implicit .* port connection syntax connects all
other ports on the instantiated module.
In the same alu_accum4 example, the accum module has an 8-bit port called dataout that is connected to
a 16-bit bus called dataout. Because the internal and external sizes of dataout do not match, the port has
to be connected by name, showing which bits of the 16-bit bus are connected to the 8-bit port. The datain
port on the accum is connected to a bus by a different name (alu_out); therefore, this port is also connected
by name. The clk port is connected using an implicit .* port connection while rst_n does not exist at the
instantiation level, and therefore the default rst_n value is used. Also in the same alu_accum4 example,
the xtend module has an 8-bit output port called dout and a 1-bit input port called din. Because neither of
these port names matches the names (or sizes) of the connecting declarations, both are connected by name.
The clk port is connected using an implicit .* port connection while again rst does not exist at the
instantiation level, and therefore the default rst value is used.
module alu_accum4 (

<!-- Page 747 -->

IEEE Std output [15:0] dataout,
input [7:0] ain, bin,
input [2:0] opcode,
input clk);
wire [7:0] alu_out;
alu alu (.*, .zero());
accum accum (.*, .dataout(dataout[7:0]), .datain(alu_out));
xtend xtend (.*, .dout(dataout[15:8]), .din(alu_out[7]));
endmodule
When the implicit .* port connection is mixed in the same instantiation with named port connections, it may
be placed anywhere in the port list. The .* token pair may appear at most once in the port list.
Modules can be instantiated into the same parent module using any combination of legal positional, named,
implicit .name connected and implicit .* connected instances, as shown in the following alu_accum5
example:
module alu_accum5 (
output [15:0] dataout,
input [7:0] ain, bin,
input [2:0] opcode,
input clk, rst_n);
wire [7:0] alu_out;
// mixture of named port connections and
// implicit .name port connections
alu alu (.ain(ain), .bin(bin), .alu_out, .zero(), .opcode);
// positional port connections
accum accum (dataout[7:0], alu_out, clk, rst_n);
// mixture of named port connections and implicit .* port connections
xtend xtend (.dout(dataout[15:8]), .*, .din(alu_out[7]));
endmodule
#### 23.3.3 Port connection rules
Values of all data types on variables and nets can be passed through ports. This is accomplished by allowing
both sides of a port connection to have assignment-compatible data types and by allowing continuous
assignments to variables. The ref port type allows shared variable behavior across a port by passing a
hierarchical reference.
Each port connection shall be a continuous assignment of source to sink, where one connected item shall be
a signal source and the other shall be a signal sink. The assignment shall be a continuous assignment from
source to sink for input or output ports. The assignment is a non-strength-reducing transistor connection for
inout ports.
The same rules are used for compatible port types as for assignment compatibility (see 6.22.3).
If the internal and external connections to a port are of user-defined nettypes, they shall be of matching
nettypes and shall be merged into a single simulated net. If only one of the two connections is of a user-
defined nettype then the connections shall have matching data types, the port shall be of mode input or
output and the connection shall be treated as a continuous assignment from source to sink.

<!-- Page 748 -->

IEEE Std 23.3.3.1 Port coercion
A port that is declared as input (output) but used as an output (input) or inout may be coerced to inout. If not
coerced to inout, a warning shall be issued.
23.3.3.2 Port connection rules for variables
If a port declaration has a variable data type, then its direction controls how it can be connected when
instantiated, as follows:
— An input port can be connected to any expression of a compatible data type. A continuous
assignment shall be implied when a variable is connected to an input port declaration. Assignments
to variables declared as input ports shall be illegal. If left unconnected, the port shall have the default
initial value corresponding to the data type.
— An output port can be connected to a variable (or a concatenation) of a compatible data type. A
continuous assignment shall be implied when a variable is connected to the output port of an
instance. Procedural or continuous assignments to a variable connected to the output port of an
instance shall be illegal.
— An output port can be connected to a net (or a concatenation) of a compatible data type. In this
case, multiple drivers shall be permitted on the net.
— A variable data type is not permitted on either side of an inout port.
— A ref port shall be connected to an equivalent variable data type. References to the port variable
shall be treated as hierarchical references to the variable to which it is connected in its instantiation.
This kind of port cannot be left unconnected. See 6.22.2.
— It shall be illegal to connect a port variable to an interconnect port or interconnect net.
23.3.3.3 Port connection rules for nets with built-in net types
If a port declaration has a net type, such as wire, then its direction controls how it can be connected, as
follows:
— An input can be connected to any expression of a compatible data type. If left unconnected, it shall
have the value 'z.
— An output can be connected to a net or variable (or a concatenation of nets or variables) of a
compatible data type.
— An inout can be connected to a net (or a concatenation of nets) of a compatible data type or left
unconnected, but cannot be connected to a variable.
If there is a data type difference between the port declaration and connection, an initial value change event
can be caused at time zero.
See 23.3.3.7 for additional rules when net types or interconnect nets are used on both sides of a port
connection.
23.3.3.4 Port connection rules for interfaces
A port declaration can be a generic interface or named interface type. An interface port instance shall always
be connected to an interface instance or a higher level interface port. An interface port cannot be left
unconnected.
If a port declaration has a generic interface type, then it can be connected to an interface instance of any
type. If a port declaration has a named interface type, then it shall be connected to an interface instance of
the identical type.

<!-- Page 749 -->

IEEE Std 23.3.3.5 Unpacked array ports and arrays of instances
For an unpacked array port, the port and the array connected to the port shall have the same number of
unpacked dimensions, and each dimension of the port shall have the same size as the corresponding
dimension of the array being connected.
If the size and type of the port connection match the size and type of a single instance port, the connection
shall be made to each instance in an array of instances.
If the port connection is an unpacked array, the slowest varying unpacked array dimensions of each port
connection shall be compared with the dimensions of the instance array. If they match exactly in size, each
element of the port connection shall be matched to the port left index to left index, right index to right index.
If they do not match it shall be considered an error.
For example:
module child(output o, input i[5]);
//...
endmodule : child
module parent(output o[8][4],
input i[8][4][5] );
child c[8][4](o,i);
//...
endmodule : parent
If the port connection is a packed array, each instance shall get a part-select of the port connection, starting
with all right-hand indices to match the rightmost part-select and iterating through the rightmost dimension
first. Too many or too few bits to connect all the instances shall be considered an error.
In the following example, a two-dimensional array of DFF instances is connected to form M pipelines with
N stages.
module MxN_pipeline #(M=3,N=4)
(input [M-1:0] in, output [M-1:0] out, input clk);
typedef logic T [M-1:0][1:N];
T Ins, Outs;
DFF dff[M-1:0][1:N](Outs, Ins, clk);
for (genvar I = M-1; I >= 0; I--) begin
for (genvar J = 1; J <= N; J++) begin
case (J)
1: begin
assign out[I] = Outs[I][1];
assign Ins[I][J] = Outs[I][2];
end
default: assign Ins[I][J] = Outs[I][J+1];
N: assign Ins[I][N] = in[I];
endcase
end
end
endmodule : MxN_pipeline

<!-- Page 750 -->

IEEE Std 23.3.3.6 Single source nets (uwire)
If the net on either side of a port has the net type uwire, a warning shall be issued if the nets are not merged
into a single net, as described in 23.3.3.7.
23.3.3.7 Port connections with dissimilar net types (net and port collapsing)
When different net types are connected through a module port, the nets on both sides of the port can take on
the same type. The resulting net type can be determined as shown in Table23-1. In the table, external net
means the net specified in the module instantiation, and internal net means the net specified in the module
definition. The net whose type is used is said to be the dominating net. The net whose type is changed is said
to be the dominated net. It is permissible to merge the dominating and dominated nets into a single net,
whose type shall be that of the dominating net. The resulting net is called the simulated net, and the
dominated net is called a collapsed net.
The simulated net shall take the delay specified for the dominating net. If the dominating net is of the type
trireg, any strength value specified for the trireg net shall apply to the simulated net.
When the two nets connected by a port are of different net types, the resulting single net can be assigned one
of the following:
— The dominating net type if one of the two nets is dominating, or
— The net type external to the module
When a dominating net type does not exist, the external net type shall be used.
The simulated net shall take the net type specified in the table and the delay specified for that net. If the
simulated net selected is a trireg, any strength value specified for the trireg net applies to the simulated
net.

<!-- Page 751 -->

IEEE Std Table23-1—Net types resulting from dissimilar port connections
External net
Internal
net wire, wand, wor,
trireg tri0 tri1 uwire supply0 supply1
tri triand trior
wire, tri external external external external external external external external external
wand, triand internal external external external external external external external external
warn warn warn warn warn
wor, trior internal external external external external external external external external
warn warn warn warn warn
trireg internal external external external external external external external external
warn warn warn
tri0 internal external external internal external external external external external
warn warn warn warn
tri1 internal external external internal external external external external external
warn warn warn warn
uwire internal internal internal internal internal internal external external external
warn warn warn warn warn
supply0 internal internal internal internal internal internal internal external external
warn
supply1 internal internal internal internal internal internal internal external external
warn
KEY:
external = The external net type shall be used.
internal = The internal net type shall be used.
warn = A warning shall be issued.
23.3.3.7.1 Port connections with interconnect net types
Any port connection with an interconnect net shall merge the dominating and dominated nets into a
single net.
If the internal and external nets are both interconnect nets, the merged net shall be an interconnect
net. If only one net is an interconnect net, the merged net shall be the type of the other net.
It shall be illegal for the type of a simulated net (see 23.3.3.7) at the end of elaboration to be an
interconnect net.
Example:
module netlist;
interconnect iwire;
dut1 child1(iwire);
dut2 child2(iwire);
endmodule
module dut1(inout wire w);
assign w = 1;

<!-- Page 752 -->

IEEE Std endmodule
module dut2(inout wand w);
assign w = 0;
endmodule
The interconnect net iwire will merge with the nets from each of the children resulting in a simulation
net with net_type wand.
23.3.3.7.2 Terminal connections with interconnect net types
When connected to a primitive or user-defined primitive (UDP) terminal, an interconnect net shall be
treated as though connecting to a scalar wire.
23.3.3.8 Connecting signed values via ports
The sign attribute shall not cross hierarchy. In order to have the signed type cross hierarchy, the signed
keyword shall be used in the object’s declaration at the different levels of hierarchy. Any expressions on a
port shall be treated as any other expression in an assignment. It shall be typed, sized, and evaluated, and the
resulting value assigned to the object on the other side of the port using the same rules as an assignment.
### 23.4 Nested modules
A module can be declared within another module. The outer name space is visible to the inner module so
that any name declared there can be used, unless hidden by a local name, provided the module is declared
and instantiated in the same scope.
One purpose of nesting modules is to show the logical partitioning of a module without using ports. Names
that are global are in the outermost scope, and names that are only used locally can be limited to local
modules.
// This example shows a D-type flip-flop made of NAND gates
module dff_flat(input d, ck, pr, clr, output q, nq);
wire q1, nq1, q2, nq2;
nand g1b (nq1, d, clr, q1);
nand g1a (q1, ck, nq2, nq1);
nand g2b (nq2, ck, clr, q2);
nand g2a (q2, nq1, pr, nq2);
nand g3a (q, nq2, clr, nq);
nand g3b (nq, q1, pr, q);
endmodule
// This example shows how the flip-flop can be structured into 3 RS latches.
module dff_nested(input d, ck, pr, clr, output q, nq);
wire q1, nq1, nq2;
module ff1;
nand g1b (nq1, d, clr, q1);
nand g1a (q1, ck, nq2, nq1);
endmodule
ff1 i1();

<!-- Page 753 -->

IEEE Std module ff2;
wire q2; // This wire can be encapsulated in ff2
nand g2b (nq2, ck, clr, q2);
nand g2a (q2, nq1, pr, nq2);
endmodule
ff2 i2();
module ff3;
nand g3a (q, nq2, clr, nq);
nand g3b (nq, q1, pr, q);
endmodule
ff3 i3();
endmodule
The nested module declarations can also be used to create a library of modules that is local to part of a
design.
module part1(...);
module and2(input a, b, output z);
...
endmodule
module or2(input a, b, output z);
...
endmodule
...
and2 u1(...), u2(...), u3(...);
...
endmodule
This allows the same module name, e.g., and2, to occur in different parts of the design and represent
different modules. An alternative way of handling this problem is to use configurations.
Nested modules with no ports that are not explicitly instantiated shall be implicitly instantiated once with an
instance name identical to the module name. Otherwise, if they have ports and are not explicitly instantiated,
they are ignored.
### 23.5 Extern modules
To support separate compilation, extern declarations of a module can be used to declare the ports on a
module without defining the module itself. An extern module declaration consists of the keywords extern
module followed by the module name and the list of ports for the module. Both the ANSI style
list_of_port_declarations syntax (possibly with parameters) and the non-ANSI style list_of_ports syntax
may be used.
NOTE—The potential existence of defparams precludes the checking of the port connection information prior to
elaboration time even for the ANSI style list_of_port_declarations syntax.
The following example demonstrates the usage of extern module declarations:
extern module m (a,b,c,d);
extern module a #(parameter size= 8, parameter type TP = logic [7:0])
(input [size:0] a, output TP b);
module top ();
wire [8:0] a;
logic [7:0] b;
wire c, d;

<!-- Page 754 -->

IEEE Std m mm (.*);
a aa (.*);
endmodule
Modules m and a are then assumed to be instantiated as follows:
module top ();
wire [8:0] a;
logic [7:0] b;
wire c, d;
m mm (a,b,c,d);
a aa (a,b);
endmodule
If an extern declaration exists for a module, it is possible to use .* as the ports of the module. This usage
shall be equivalent to placing the ports (and possibly parameters) of the extern declaration on the module.
For example:
extern module m (a,b,c,d);
extern module a #(parameter size = 8, parameter type TP = logic [7:0])
(input [size:0] a, output TP b);
module m (.*);
input a,b,c;
output d;
endmodule
module a (.*);
...
endmodule
is equivalent to writing
module m (a,b,c,d);
input a,b,c;
output d;
endmodule
module a #(parameter size = 8, parameter type TP = logic [7:0])
(input [size:0] a, output TP b);
...
endmodule
Extern module declarations can appear at any level of the instantiation hierarchy, but are visible only within
the level of hierarchy in which they are declared. An extern module declaration shall match the actual
module declaration’s port and parameter lists in correspondence of names, positions, and their equivalent
types.
### 23.6 Hierarchical names
Every identifier in a SystemVerilog description shall have a unique hierarchical path name. The hierarchy
of modules and the definition of items such as tasks and named blocks within the modules shall define these
names. The hierarchy of names can be viewed as a tree structure, where each module instance, generate

<!-- Page 755 -->

IEEE Std block instance, task, function, or named begin-end or fork-join block defines a new hierarchical level, or
scope, in a particular branch of the tree.
A design description contains one or more top-level modules (see 23.3.1). Each such module forms the top
of a name hierarchy. This root or these parallel root modules make up one or more hierarchies in a design
description or description. Inside any module, each module instance (including an arrayed instance),
generate block instance, task definition, function definition, and named begin-end or fork-join block shall
define a new branch of the hierarchy. Named blocks within named blocks and within tasks and functions
shall create new branches. Similarly, named action blocks of assertions shall create new branches. Unnamed
generate blocks are exceptions. They create branches that are visible only from within the block and within
any hierarchy instantiated by the block. See Clause27 for a discussion of unnamed generate blocks.
Each node in the hierarchical name tree shall be a separate scope with respect to identifiers. A particular
identifier can be declared at most once in any scope. See 23.9 for a discussion of scope rules and 3.13 for a
discussion of name spaces.
Any named SystemVerilog object or hierarchical name reference can be referenced uniquely in its full form
by concatenating the names of the modules, module instance names, generate blocks, tasks, functions,
assertion labels, named assertion action blocks, or named blocks that contain it. The period character shall
be used to separate each of the names in the hierarchy, except for escaped identifiers embedded in the
hierarchical name reference, which are followed by separators composed of white space and a period-
character.
The syntax for hierarchical path names is given in Syntax23-7.
hierarchical_identifier ::= [ $root . ] { identifier constant_bit_select . } identifier // from A.9.3
Syntax23-7—Syntax for hierarchical path names (excerpt from AnnexA)
Hierarchical names consist of instance names separated by periods, where an instance name can be an array
element. The instance name $root refers to the top of the instantiated design and is used to unambiguously
gain access to the top of the design.
$root.mymodule.u1 // absolute name
u1.struct1.field1 // u1 shall be visible locally or above, including globally
adder1[5].sum
The complete path name to any object shall start at a top-level (root) module. This path name can be used
from any level in the hierarchy or from a parallel hierarchy.
The first node name in a path name can also be the top of a hierarchy that starts at the level where the path is
being used (which allows and enables downward referencing of items).
Objects declared in automatic tasks and functions are exceptions and cannot be accessed by hierarchical
name references. Objects declared in unnamed generate blocks are also exceptions. They can be referenced
by hierarchical names only from within the block and within any hierarchy instantiated by the block.
Names in a hierarchical path name that refer to instance arrays or loop generate blocks may be followed
immediately by a constant expression in square brackets. This expression selects a particular instance of the
array and is, therefore, called an instance select. The expression shall evaluate to one of the legal index
values of the array. If the array name is not the last path element in the hierarchical name, the instance select
expression is required.

<!-- Page 756 -->

IEEE Std Hierarchical name referencing allows free data access to any object from any level in the hierarchy. If the
unique hierarchical path name of an item is known, its value can be sampled or changed from anywhere
within the description.
Hierarchical names can be read (in expressions), written (in assignments or in subroutine calls) or triggered
off (in event expressions). They can also be used to reference subroutine names.
Example 1: The code in this example defines a hierarchy of module instances and named blocks.
module cct (stim1, stim2);
input stim1, stim2;
// instantiate mod
mod amod(stim1),
bmod(stim2);
endmodule
module mod (in);
input in;
always @(posedge in) begin : keep
logic hold;
hold = in;
end
endmodule
module wave;
logic stim1, stim2;
cct a(stim1, stim2); // instantiate cct
initial begin :wave1
#100 fork :innerwave
reg hold;
join
#150 begin
stim1 = 0;
end
end
endmodule
Figure23-1 illustrates the hierarchy implicit in this code.
wave
a wave1
innerwave
amod bmod
keep keep
Figure23-1—Hierarchy in a model

<!-- Page 757 -->

IEEE Std Following is a list of the hierarchical forms of the names of all the objects defined in the code.
wave
wave.stim1
wave.stim2
wave.a
wave.a.stim1
wave.a.stim2
wave.a.amod
wave.a.amod.in
wave.a.amod.keep
wave.a.amod.keep.hold
wave.a.bmod
wave.a.bmod.in
wave.a.bmod.keep
wave.a.bmod.keep.hold
wave.wave1
wave.wave1.innerwave
wave.wave1.innerwave.hold
Any of the preceding hierarchical names can also be preceded with $root.
Example 2: The following example shows how a pair of named blocks can refer to items declared within
each other.
begin
fork : mod_1
reg x;
mod_2.x = 1;
join
fork : mod_2
reg x;
mod_1.x = 0;
join
end
Example 3: The following example shows when assertions and items in assertion action blocks may or may
not be referred to using hierarchical names.
module top();
logic clk, x, y, z;
m m_i(clk, x, y, z);
endmodule
module m(input logic clk, a, b, c);
assert #0 (a^b); // no label, assertion cannot be referred to
A1: assert #0 (a^b); // assertion can be accessed in control tasks
initial begin : B1
assert (a); // cannot be accessed in control tasks
A1: assert (a) // can be accessed, e.g., top.m_i.B1.A1
begin // unnamed block, d cannot be accessed
bit d;
d = a ^ b;
end
else
begin : B2 // name required to access items in action block
bit d; // d can be accessed using, e.g., top.m_i.B1.A1.B2.d

<!-- Page 758 -->

IEEE Std d = a ^ b;
end
end
logic e;
always_ff @(posedge clk) begin // unnamed block, no scope created
e <= a && c;
C1: cover property(e) // C1 and A2 can be referred to
begin // hierarchical name top.m_i.C1.A2
A2: assert (m_i.B1.A1.B2.d);
end
end
always_ff @(posedge clk) begin // unnamed block, scope created
// declaration of f causes begin-end to create scope
static logic f;
f <= a && c;
C2: cover property(f) // C2 and A3 cannot be referred to
begin
A3: assert (m_i.B1.A1.B2.d);
end
end
always_ff @(posedge clk) begin : B2 // named block and scope created
static logic f;
f <= a && c;
C3: cover property(f) // C3 and A4 can be referred to
begin // hierarchical name top.m_i.B2.C3.A4
A4: assert (m_i.B1.A1.B2.d);
end
end
assert property(@(posedge clk) a |-> b) else // unnamed assertion
begin: B3
static bit d; // d can be referred to, e.g., top.m_i.B3.d
…
A5: assert(d); // hierarchical name top.m_i.B3.A5
end
// Any other labelled object with name B3 at the module
// level shall be an error
endmodule
Hierarchical references into checkers (see Clause17) shall not be permitted.
### 23.7 Member selects and hierarchical names
A hierarchical name and a member select into a structure, union, class or covergroup object share the same
syntactic form of a sequence of name components separated by periods. Such names are called dotted names
prior to the determination of whether the name is a hierarchical name or member select. The distinguishing
aspect of a hierarchical name is that the first component of the name matches a scope name while the first
name component of a member select matches a data object or interface port name. The general approach
used is to attempt to resolve the first name component immediately and to use the results of that resolution
attempt to determine how to treat the overall name.
When a dotted name is encountered at its point of appearance, the first name in the sequence is resolved as
though it were a simple identifier. The following are the possible results:

<!-- Page 759 -->

IEEE Std a) The name resolves to a data object or interface port. The dotted name shall be considered to be a
select of that data object or interface port.
b) The name resolves to a directly visible scope name. The dotted name shall be considered to be a
hierarchical name.
c) The name resolves to an imported scope name. The dotted name shall be resolved in the same
manner as a hierarchical name prefixed by the package name from which the name was imported.
d) The name is not found. The dotted name shall be considered to be a hierarchical name.
It is important to note that resolution to an imported scope name is different than resolution to a directly
visible scope name (see 23.7.1).
Example:
package p;
struct { int x; } s1;
struct { int x; } s2;
function void f();
int x;
endfunction
endpackage
module m;
import p::*;
if (1) begin : s1
initial begin
s1.x = 1; // dotted name 1
s2.x = 1; // dotted name 2
f.x = 1; // dotted name 3
f2.x = 1; // dotted name 4
end
int x;
some_module s2();
end
endmodule
The following describes the resolution of each of the dotted names:
— Dotted name 1: The first name component is s1. Since s1 is a directly visible scope name, rule b)
applies and the name s1.x is considered to be a hierarchical name.
— Dotted name 2: The first name component is s2. Since at the time of analysis the module
instantiation scope s2 (from some_module s2();) is not yet visible, the name s2 binds to the
visible name s2 from package p and rule a) applies. This causes s2 to be imported into module m as
would occur with a normal variable reference.
— Dotted name 3: The first name component is f. Since f is an imported scope name, rule c) applies
and the name f.x is considered to be a hierarchical name equivalent to p::f.x.
— Dotted name 4: The first name component is f2. Since f2 has no visible definition, rule d) applies
and the name f2.x is considered to be a hierarchical name.
#### 23.7.1 Names with package or class scope resolution operator prefixes
A name with a package or class scope resolution prefix (::) shall always resolve in a downwards manner
and shall never be subject to the upwards resolution rules in 23.8. If the prefix name can be resolved using
the normal scope resolution rules, the “::” shall denote the class scope resolution operator. Otherwise the
“::” shall denote the package scope resolution operator.

<!-- Page 760 -->

IEEE Std ### 23.8 Upwards name referencing
The name of a module or module instance is sufficient to identify the module and its location in the
hierarchy. A lower level module can reference items in a module above it in the hierarchy. Variables can be
referenced if the name of the higher level module or its instance name is known. For tasks, functions, named
blocks, and generate blocks, SystemVerilog shall look in the enclosing module for the name until it is found
or until the root of the hierarchy is reached. It shall only search in higher enclosing modules for the name,
not instances.
The syntax for an upward reference is given in Syntax23-8.
upward_name_reference ::=
module_identifier.item_name
item_name ::=
function_identifier
| block_identifier
| net_identifier
| parameter_identifier
| port_identifier
| task_identifier
| variable_identifier
Syntax23-8—Syntax for upward name referencing (not in AnnexA)
Upward name references can also be done with names of the form
scope_name.item_name
where scope_name is either a subroutine name, a module, program, or interface instance name or a
generate block name. A name of this form shall be resolved as follows:
a) Look in the current scope for a scope named scope_name. If not found and the current scope is not
the design element scope, look for the name in the enclosing scope, repeating as necessary until the
name is found or the design element scope is reached. If still not found, proceed to step b).
Otherwise, this name reference shall be treated as a downward reference from the scope in which the
name is found.
b) Look in the instantiation’s parent scope for a scope named scope_name. If found, the item name
shall be resolved in a downwards manner from that scope. If all name components of the item name
are matched, the search terminates with the final matching item. If any component of the item name
matches the name of a structure, union, class, or covergroup object, no further upwards steps shall
occur even if the item name does not find a match. Continue upwards through the enclosing scopes,
repeating as necessary until the name is found or the design element scope is reached.
c) Repeat step b), going up the hierarchy.
There is an exception to these rules for hierarchical names on the left-hand side of defparam statements.
See 23.10.4 for details.
In the following example, there are four modules, a, b, c, and d. Each module contains an integer i. The
highest level modules in this segment of a model hierarchy are a and d. There are two copies of module b
because module a and d instantiate b. There are four copies of c.i because each of the two copies of b
instantiates c twice.

<!-- Page 761 -->

IEEE Std module a;
integer i;
b a_b1();
endmodule
module b;
integer i;
c b_c1(),
b_c2();
initial // downward path references two copies of i:
#10 b_c1.i = 2; // a.a_b1.b_c1.i, d.d_b1.b_c1.i
endmodule
module c;
integer i;
initial begin // local name references four copies of i:
i = 1; // a.a_b1.b_c1.i, a.a_b1.b_c2.i,
// d.d_b1.b_c1.i, d.d_b1.b_c2.i
b.i = 1; // upward path references two copies of i:
// a.a_b1.i, d.d_b1.i
end
endmodule
module d;
integer i;
b d_b1();
initial begin // full path name references each copy of i
a.i = 1; d.i = 5;
a.a_b1.i = 2; d.d_b1.i = 6;
a.a_b1.b_c1.i = 3; d.d_b1.b_c1.i = 7;
a.a_b1.b_c2.i = 4; d.d_b1.b_c2.i = 8;
end
endmodule
#### 23.8.1 Task and function name resolution
Task and function names are resolved following slightly different rules than other references. Task and
function name resolution follows the rules for upwards hierarchical name resolution as described in 23.8,
step a). Then, before proceeding with step b), an implementation shall look in the complete compilation unit
of the reference. If a task or function with a matching name is found there, the name resolves to that task or
function. Only then does the resolution proceed with step b) and iterate as normal. The special matching
within the compilation unit shall only take place the first time through the iteration through steps a) – c); a
task or function name shall never match a task or function in a compilation unit other than the compilation
unit enclosing the reference.
Example 1:
task t;
int x;
x = f(1); // valid reference to function f in $unit scope
endtask
function int f(int y);
return y+1;
endfunction

<!-- Page 762 -->

IEEE Std Example 2:
package p;
function void f();
$display("p::f");
endfunction
endpackage
module top;
import p::*;
if (1) begin : b // generate block
initial f(); // reference to “f”
function void f();
$display("top.b.f");
endfunction
end
endmodule
The resolution of the name f follows the hierarchical rules and therefore is resolved to the function
top.b.f. The output of the example would be the output of the string "top.b.f".
### 23.9 Scope rules
The following elements define a new scope in SystemVerilog:
— Modules
— Interfaces
— Programs
— Checkers
— Packages
— Classes
— Tasks
— Functions
— begin-end blocks (named or unnamed)
— fork-join blocks (named or unnamed)
— Generate blocks
An identifier shall be used to declare only one item within a scope. This rule means it is illegal to declare
two or more variables that have the same name, or to name a task the same as a variable within the same
module, or to give a gate instance the same name as the name of the net connected to its output. For generate
blocks, this rule applies regardless of whether the generate block is instantiated. An exception to this is made
for generate blocks in a conditional generate construct. See 27.6 for a discussion of naming conditional
generate blocks.
If an identifier is referenced directly (without a hierarchical path) within a task, function, named block, or
generate block, it shall be declared either within the task, function, named block, or generate block locally or
within a module, interface, program, checker, task, function, named block, or generate block that is higher in
the same branch of the name tree that contains the task, function, named block, or generate block. If it is
declared locally, then the local item shall be used; if not, the search shall continue upward until an item by
that name is found or until a module, interface, program, or checker boundary is encountered. If the item is a
variable, it shall stop at a module boundary; if the item is a task, function, named block, or generate block, it

<!-- Page 763 -->

IEEE Std continues to search higher level modules until found. This fact means that tasks and functions can use and
modify the variables within the containing module by name, without going through their formal arguments.
If an identifier is referenced with a hierarchical name, the path can start with a module name, interface name,
program name, checker name, instance name, task, function, named block, or named generate block. The
names shall be searched first at the current level and then in higher level modules until found. Because both
module, interface, program, or checker names as well as instance names can be used, precedence is given to
instance names if there is a module, interface, program, or checker named the same as an instance name.
Because of the upward searching, path names that are not strictly on a downward path can be used.
For example:
Example 1: In Figure23-2, each rectangle represents a local scope. The scope available to upward searching
extends outward to all containing rectangles—with the boundary of the module A as the outer limit. Thus
block G can directly reference identifiers in F, E, and A; it cannot directly reference identifiers in H, B, C,
and D.
module A
task E
Scopes not block B
Scopes available
available to
to block G
block G block F
block G
task C
block H
func D
Figure23-2—Scopes available to upward name referencing
Example 2: The following example shows how variables can be accessed directly or with hierarchical
names:
task t;
logic s;
begin : b
logic r;
t.b.r = 0;// These three lines access the same variable r
b.r = 0;
r = 0;
t.s = 0;// These two lines access the same variable s
s = 0;

<!-- Page 764 -->

IEEE Std end
endtask
### 23.10 Overriding module parameters
SystemVerilog provides two types of parameter constants that can be overridden, value parameters (see
6.20.2) and type parameters (see 6.20.3).
There are two different places parameters can be defined within a module (or interface or program). The first
is the module’s parameter_port_list (see 23.2), and the second is as a module_item (see 6.20). A module
declaration can contain parameter definitions of either or both types or can contain no parameter definitions.
If a module has a parameter_port_list, then any additional parameter defined in a module_item is treated as
a local parameter (see 6.20.1).
For example:
module generic_fifo
#(MSB=3, LSB=0, DEPTH=4) // parameter port list parameters
(input wire [MSB:LSB] in,
input wire clk, read, write, reset,
output logic [MSB:LSB] out,
output logic full, empty);
parameter FIFO_MSB = DEPTH * MSB;
localparam FIFO_LSB = LSB;
// These constants are local, and cannot be overridden.
// They can be affected by altering the value parameters above.
logic [FIFO_MSB:FIFO_LSB] fifo;
logic [$clog2(DEPTH):0] depth;
always @(posedge clk or posedge reset) begin
casez ({read,write,reset})
// implementation of fifo
endcase
end
endmodule
There are several ways to alter nonlocal parameters. Two ways are the defparam statement, which allows
assignment to parameters using their hierarchical names, and the module instance parameter value
assignment, which allows values to be assigned inline during module instantiation. The module instance
parameter value assignment comes in two forms, by ordered list or by name. The next two subclauses
describe these two methods. If a defparam assignment conflicts with a module instance parameter, the
parameter in the module will take the value specified by the defparam. A third way is via configurations
(see 33.4.3).
A value parameter (see 6.20.2) can have a type specification and a range specification. The effect of
parameter overrides on a value parameter’s type and range shall be in accordance with the following rules:
— A value parameter declaration with no type or range specification shall default to the type and range
of the final override value assigned to the parameter.
— A value parameter with a range specification, but with no type specification, shall have the range of
the parameter declaration and shall be unsigned. An override value shall be converted to the type
and range of the parameter.

<!-- Page 765 -->

IEEE Std — A value parameter with a type specification, but with no range specification, shall be of the type
specified. An override value shall be converted to the type of the parameter. A signed parameter
shall default to the range of the final override value assigned to the parameter.
— A value parameter with a signed type specification and with a range specification shall be signed
and shall have the range of its declaration. An override value shall be converted to the type and
range of the parameter.
For example:
module m1 (a,b);
real r1,r2;
parameter [2:0] A = 3'h2;
parameter B = 3'h2;
initial begin
r1 = A;
r2 = B;
$display("r1 is %f r2 is %f",r1,r2);
end
endmodule: m1
module m2;
wire a,b;
defparam f1.A = 3.1415;
defparam f1.B = 3.1415;
m1 f1(a,b);
endmodule: m2
Parameter A is a typed and/or ranged parameter; when its value is redefined, the parameter retains its original
type and sign. Therefore, the defparam of f1.A with the value 3.1415 is performed by converting the
floating-point number 3.1415 into a fixed-point number 3, and then the low three bits of 3 are assigned to
A.
Parameter B is not a typed and/or ranged parameter; when its value is redefined, the parameter type and
range take on the type and range of the new value. Therefore, the defparam of f1.B with the value 3.1415
replaces B’s current value of 3'h2 with the floating-point number 3.1415.
#### 23.10.1 defparam statement
Using the defparam statement, parameter values can be changed in any module, interface, or program
instance throughout the design using the hierarchical name of the parameter. See 23.6 for hierarchical
names.
However, a defparam statement in a hierarchy in or under a generate block instance (see Clause27) or an
array of instances (see 28.3.5 and 23.3.2) shall not change a parameter value outside that hierarchy.
Each instantiation of a generate block is considered to be a separate hierarchy scope. Therefore, a defparam
statement in a generate block may not target a parameter in another instantiation of the same generate block,
even when the other instantiation is created by the same loop generate construct. For example, the following
code is not allowed:
genvar i;
generate
for (i = 0; i < 8; i = i + 1) begin : somename
flop my_flop(in[i], in1[i], out1[i]);
defparam somename[i+1].my_flop.xyz = i ;

<!-- Page 766 -->

IEEE Std end
endgenerate
Similarly, a defparam statement in one instance of an array of instances may not target a parameter in
another instance of the array.
The expression on the right-hand side of defparam assignments shall be a constant expression involving
only numbers and references to parameters. The referenced parameters (on the right-hand side of the
defparam) shall be declared in the same module as the defparam statement.
The defparam statement is particularly useful for grouping all of the parameter value override assignments
together in one module.
In the case of multiple defparams for a single parameter, the parameter takes the value of the last defparam
statement encountered in the source text. When defparams are encountered in multiple source files, e.g.,
found by library searching, the defparam from which the parameter takes its value is undefined.
For example:
module top;
logic clk;
logic [0:4] in1;
logic [0:9] in2;
wire [0:4] o1;
wire [0:9] o2;
vdff m1 (o1, in1, clk);
vdff m2 (o2, in2, clk);
endmodule
module vdff (out, in, clk);
parameter size = 1, delay = 1;
input [0:size-1] in;
input clk;
output [0:size-1] out;
logic [0:size-1] out;
always @(posedge clk)
# delay out = in;
endmodule
module annotate;
defparam
top.m1.size = 5,
top.m1.delay = 10,
top.m2.size = 10,
top.m2.delay = 20;
endmodule
The module annotate has the defparam statement, which overrides size and delay parameter values for
instances m1 and m2 in the top-level module top. The modules top and annotate would both be
considered top-level modules.
NOTE—The defparam statement might be removed from future versions of the language. See C.4.1.

<!-- Page 767 -->

IEEE Std #### 23.10.2 Module instance parameter value assignment
An alternative method for assigning values to parameters within module instances is to use one of the two
forms of module instance parameter value assignment: assignment by ordered list and assignment by name.
The two types of module instance parameter value assignment shall not be mixed; parameter assignments to
a particular module instance shall be entirely by order or entirely by name.
Module instance parameter value assignment by ordered list is similar in appearance to the assignment of
delay values to gate instances, and assignment by name is similar to connecting module ports by name. It
supplies values for particular instances of a module to any parameters that have been specified in the
definition of that module.
A parameter declared in a named block, task, or function can only be directly redefined using a defparam
statement. However, if the parameter value is dependent on a second parameter, then redefining the second
parameter will update the value of the first parameter as well (see 23.10.3).
23.10.2.1 Parameter value assignment by ordered list
The order of the assignments in the module instance parameter assignment by ordered list shall follow the
order of declaration of the parameters within the module. It is not necessary to assign values/types to all of
the parameters within a module when using this method. However, it is not possible to skip over a
parameter. Therefore, to assign values to a subset of the parameters declared within a module, the
declarations of the parameters that make up this subset shall precede the declarations of the remaining
parameters. An alternative is to assign values to all of the parameters, but to use the default value (the same
value assigned in the declaration of the parameter within the module definition) for those parameters that do
not need new values.
Consider the following example, where the parameters within module instances mod_a, mod_c, and mod_d
are changed during instantiation:
module tb1;
wire [9:0] out_a, out_d;
wire [4:0] out_b, out_c;
logic [9:0] in_a, in_d;
logic [4:0] in_b, in_c;
logic clk;
// testbench clock & stimulus generation code ...
// Four instances of vdff with parameter value assignment by ordered list
// mod_a has new parameter values size=10 and delay=15
// mod_b has default parameters (size=5, delay=1)
// mod_c has one default size=5 and one new delay=12
// In order to change the value of delay,
// it is necessary to specify the (default) value of size as well.
// mod_d has a new parameter value size=10.
// delay retains its default value
vdff #(10,15) mod_a (.out(out_a), .in(in_a), .clk(clk));
vdff mod_b (.out(out_b), .in(in_b), .clk(clk));
vdff #( 5,12) mod_c (.out(out_c), .in(in_c), .clk(clk));
vdff #(10) mod_d (.out(out_d), .in(in_d), .clk(clk));
endmodule

<!-- Page 768 -->

IEEE Std module vdff (out, in, clk);
parameter size=5, delay=1;
output [size-1:0] out;
input [size-1:0] in;
input clk;
logic [size-1:0] out;
always @(posedge clk)
#delay out = in;
endmodule
Local parameters cannot be overridden; therefore, they are not considered part of the ordered list for
parameter value assignment, even if the local parameter appears in a module’s parameter_port_list. In the
following example, addr_width will be assigned the value 12, and data_width will be assigned the value
16. mem_size will not be explicitly assigned a value due to the ordered list, but will have the value 4096 due
to its declaration expression.
module my_mem (addr, data);
parameter addr_width = 16;
localparam mem_size = 1 << addr_width;
parameter data_width = 8;
...
endmodule
module top;
...
my_mem #(12, 16) m(addr,data);
endmodule
23.10.2.2 Parameter value assignment by name
Parameter assignment by name consists of explicitly linking the parameter name and its new value. The
name of the parameter shall be the name specified in the instantiated module.
It is not necessary to assign values to all of the parameters within a module when using this method. Only
parameters that are assigned new values need to be specified.
The parameter expression is optional so that the instantiating module can document the existence of a
parameter without assigning anything to it. The parentheses are required, and in this case the parameter
retains its default value. Once a parameter is assigned a value, there shall not be another assignment to this
parameter name.
Consider the following example, where both parameters of mod_a and only one parameter of mod_c and
mod_d are changed during instantiation:
module tb2;
wire [9:0] out_a, out_d;
wire [4:0] out_b, out_c;
logic [9:0] in_a, in_d;
logic [4:0] in_b, in_c;
logic clk;
// testbench clock & stimulus generation code ...
// Four instances of vdff with parameter value assignment by name
// mod_a has new parameter values size=10 and delay=15

<!-- Page 769 -->

IEEE Std // mod_b has default parameters (size=5, delay=1)
// mod_c has one default size=5 and one new delay=12
// mod_d has a new parameter value size=10.
// delay retains its default value
vdff #(.size(10),.delay(15)) mod_a (.out(out_a),.in(in_a),.clk(clk));
vdff mod_b (.out(out_b),.in(in_b),.clk(clk));
vdff #(.delay(12)) mod_c (.out(out_c),.in(in_c),.clk(clk));
vdff #(.delay( ),.size(10) ) mod_d (.out(out_d),.in(in_d),.clk(clk));
endmodule
module vdff (out, in, clk);
parameter size=5, delay=1;
output [size-1:0] out;
input [size-1:0] in;
input clk;
logic [size-1:0] out;
always @(posedge clk)
#delay out = in;
endmodule
It shall be legal to instantiate modules using different types of parameter redefinition in the same top-level
module. Consider the following example, where the parameters of mod_a are changed using parameter
redefinition by ordered list and the second parameter of mod_c is changed using parameter redefinition by
name during instantiation:
module tb3;
// declarations & code
// legal mixture of instance with positional parameters and
// another instance with named parameters
vdff #(10, 15) mod_a (.out(out_a), .in(in_a), .clk(clk));
vdff mod_b (.out(out_b), .in(in_b), .clk(clk));
vdff #(.delay(12)) mod_c (.out(out_c), .in(in_c), .clk(clk));
endmodule
It shall be illegal to instantiate any module using a mixture of parameter redefinitions by order and by name
as shown in the instantiation of mod_a below:
// mod_a instance with ILLEGAL mixture of parameter assignments
vdff #(10, .delay(15)) mod_a (.out(out_a), .in(in_a), .clk(clk));
#### 23.10.3 Parameter dependence
A parameter (for example, memory_size) can be defined with an expression containing another parameter
(for example, word_size). However, overriding a parameter, whether by a defparam statement or in a
module instantiation statement, effectively replaces the parameter definition with the new expression.
Because memory_size depends on the value of word_size, a modification of word_size changes the
value of memory_size. For example, in the following parameter declaration, an update of word_size,
whether by defparam statement or in an instantiation statement for the module that defined these
parameters, automatically updates memory_size. If memory_size is updated due to either a defparam or
an instantiation statement, then it will take on that value, regardless of the value of word_size.

<!-- Page 770 -->

IEEE Std parameter
word_size = 32,
memory_size = word_size * 4096;
Parameters can also have type dependencies on other parameters, including type parameters. Examples of
such dependencies are as follows:
parameter p = 1;
parameter [p:0] p2 = 4;
parameter type T = int;
parameter T p3 = 7;
If parameter p changes, the value of p2 is recomputed based on the new size of the type. If the type
parameter T changes, the value of p3 is recomputed. It is possible for an override of a parameter to result in
an illegal parameter assignment. For example, if T in the preceding example was overridden to a class type,
the evaluation of p3 would be illegal and would cause elaboration to fail.
If a module instance overrides a type parameter, assignments to parameters that depend on the type
parameter shall not occur with the default type.
class C ;
endclass
module M #( type T = C, T p = 4,
type T2, T2 p2 = 4
) () ;
endmodule
In the preceding example, if the type parameter T is not overridden to an integral type, the evaluation of the
default value for parameter p is illegal. If T is overridden to an integral type, the default initialization of p
shall occur only with the overridden type resulting in a legal initialization. Similarly, since T2 requires an
instantiation override, the evaluation of p2 shall only occur with the type defined by the parameter override.
#### 23.10.4 Elaboration considerations
Elaboration is the process that occurs between parsing and simulation. It binds modules to module
instances, builds the model hierarchy, computes parameter values, resolves hierarchical names, establishes
net connectivity, and prepares all of this for simulation.
23.10.4.1 Order of elaboration
Because of generate constructs, the model hierarchy can depend on parameter values. Because defparam
statements can alter parameter values from almost anywhere in the hierarchy, the result of elaboration can be
ambiguous when generate constructs are involved. The final model hierarchy can depend on the order in
which defparams and generate constructs are evaluated.
The following algorithm defines an order that produces the correct hierarchy:
a) A list of starting points is initialized with the list of top-level modules.
b) The hierarchy below each starting point is expanded as much as possible without elaborating
generate constructs. All parameters encountered during this expansion are given their final values by
applying initial values, parameter overrides, and defparam statements.
In other words, any defparam statement whose target can be resolved within the hierarchy

<!-- Page 771 -->

IEEE Std elaborated so far shall have its target resolved and its value applied. defparam statements whose
target cannot be resolved are deferred until the next iteration of this step. Because no defparam
inside the hierarchy below a generate construct is allowed to refer to a parameter outside the
generate construct, it is possible for parameters to get their final values before going to step c).
c) Each generate construct encountered in step b) is revisited, and the generate scheme is evaluated.
The resulting generate block instantiations make up the new list of starting points. If the new list of
starting points is not empty, go to step b).
23.10.4.2 Early resolution of hierarchical names
In order to comply with this algorithm, hierarchical names in some defparam statements will need to be
resolved prior to the full elaboration of the hierarchy. It is possible that when elaboration is complete, rules
for name resolution would dictate that a hierarchical name in a defparam statement would have resolved
differently had early resolution not been required. This could result in a situation where an identical
hierarchical name in some other statement in the same scope would resolve differently from the one in the
defparam statement. Following is an example of a design that has this problem:
module m;
m1 n();
endmodule
module m1;
parameter p = 2;
defparam m.n.p = 1;
initial $display(m.n.p);
generate
if (p == 1) begin : m
m2 n();
end
endgenerate
endmodule
module m2;
parameter p = 3;
endmodule
In this example, the defparam needs to be evaluated before the conditional generate is elaborated. At this
point in elaboration, the name resolves to parameter p in module m1, and this parameter is used in the
generate scheme. The result of the defparam is to set that parameter to 1; therefore, the generate condition
is true. After the hierarchy below the generate construct is elaborated, the rules for hierarchical name
resolution would dictate that the name should have resolved to parameter p in module m2. In fact, the
identical name in the $display statement will resolve to that other parameter.
It shall be an error if a hierarchical name in a defparam is resolved before the hierarchy is completely
elaborated and that name would resolve differently once the model is completely elaborated.
This situation will occur very rarely. In order to cause the error, there has to be a named generate block that
has the same name as one of the scopes in its full hierarchical name. Furthermore, there have to be two
instances with the same name, one in the generate block and one in the other scope with the same name as
the generate block. Then, inside these instances there have to be parameters with the same name. If this
problem occurs, it can be easily fixed by changing the name of the generate block.

<!-- Page 772 -->

IEEE Std ### 23.11 Binding auxiliary code to scopes or instances
It is often desired to keep verification code separate from the design code. SystemVerilog provides a bind
construct that is used to specify one or more instantiations of a module, interface, program, or checker
without modifying the code of the target. So, for example, instrumentation code or assertions that are
encapsulated in a module, interface, program, or checker can be instantiated in a target module or a module
instance in a non-intrusive manner. Similarly, instrumentation code that is encapsulated in an interface can
be bound to a target interface or interface instance.
The syntax of the bind construct is as follows in Syntax23-9.
bind_directive4 ::= // from A.1.4
bind bind_target_scope [ : bind_target_instance_list ] bind_instantiation ;
| bind bind_target_instance bind_instantiation ;
bind_target_scope ::=
module_identifier
| interface_identifier
bind_target_instance ::=
hierarchical_identifier constant_bit_select
bind_target_instance_list ::=
bind_target_instance { , bind_target_instance }
bind_instantiation ::=
program_instantiation
| module_instantiation
| interface_instantiation
| checker_instantiation
4) If the bind_target_scope or the bind_target_instance is an interface, then the bind_instantiation shall be an
interface_instantiation or a checker_instantiation.
Syntax23-9—Bind construct syntax (excerpt from AnnexA)
The bind directive can be specified in any of the following:
— A module
— An interface
— A compilation-unit scope
There are two forms of bind syntax. In the first form, bind_target_scope specifies a target scope into which
the bind_instantiation should be inserted. A bind target scope shall be a module or an interface. A bind
target instance shall be an instance of a module or an interface. In the absence of a bind_target_instance_list,
the bind_instantiation is inserted into all instances of the specified target scope, designwide. If a
bind_target_instance_list is present, the bind_instantiation is only inserted into the specified instances of
the target scope. The bind_instantiation is effectively a complete module, interface, program, or checker
instantiation statement.
The second form of bind syntax can be used to specify a single instance into which the bind_instantiation
should be inserted. If the second form of bind syntax is used and the bind_target_instance identifier resolves
to both an instance name and a module name, binding shall only occur to the specified instance.
Example of binding a program instance to a module:

<!-- Page 773 -->

IEEE Std bind cpu fpu_props fpu_rules_1(a,b,c);
where
— cpu is the name of the target module.
— fpu_props is the name of the program to be instantiated.
— fpu_rules_1 is the program instance name to be created in the target scope.
— An instance named fpu_rules_1 is instantiated in every instance of module cpu.
— The first three ports of program fpu_props get bound to objects a, b, and c in module cpu (these
objects are viewed from module cpu’s point of view, and they are completely distinct from any
objects named a, b, and c that are visible in the scope that contains the bind directive).
Example of binding a program instance to a specific instance of a module:
bind cpu: cpu1 fpu_props fpu_rules_1(a, b, c);
In the preceding example, the fpu_rules_1 instance is bound into the cpu1 instance of module cpu.
Example of binding a program instance to multiple instances of a module:
bind cpu: cpu1, cpu2, cpu3 fpu_props fpu_rules_1(a, b, c);
In the preceding example, the fpu_rules_1 instance is bound into instances cpu1, cpu2, and cpu3 of
module cpu.
By binding a program to a module or an instance, the program becomes part of the bound object.
Binding of a module, interface, or checker instance works the same way as described for the previous
programs.
interface range (input clk, enable, input var int minval, expr);
property crange_en;
@(posedge clk) enable |-> (minval <= expr);
endproperty
range_chk: assert property (crange_en);
endinterface
bind cr_unit range r1(c_clk,c_en,v_low,(in1&&in2));
In this example, interface range is instantiated in the module cr_unit. Effectively, every instance of
module cr_unit shall contain the interface instance r1.
The bind_instantiation portion of the bind statement allows the complete range of SystemVerilog
instantiation syntax. In other words, both parameter and port associations may appear in the
bind_instantiation. All actual ports and parameters in the bind_instantiation refer to objects from the
viewpoint of the bind_target_instance.
When an instance is bound into a target scope, the effect will be as if the instance was present at the very end
of the target scope. In other words, all declarations present in the target scope or imported into the target
scope are visible to the bound instance. Wildcard import candidates that have been imported into the scope
are visible, but a bind statement cannot cause the import of a wildcard candidate. Declarations present or
imported into $unit are not visible in the bind statement.

<!-- Page 774 -->

IEEE Std User-defined type names that are used to override type parameters shall be visible and matching in both the
scope containing the bind statement and in the target scope.
If multiple bind statements are present in a given scope, the order of those statements is not important. An
implementation is free to elaborate bind statements in any order it chooses.
The following is an example of a module containing a bind statement with complex instantiation syntax.
All identifiers in the bind instantiation are referenced from the bind target’s point of view in the overall
design hierarchy.
bind targetmod mycheck #(.param1(const4), .param2(8'h44))
i_mycheck(.*, .p1(f1({v1, 1'b0, b1.c}, v2 & v3)), .p2(top.v4));
If any controlling configuration library mapping (see 33.3) is in effect at the time a bind statement is
encountered, the mapping associated with the bind statement shall determine both the selection of the
bind_target_scope and the elaboration of the bind_instantiation statement in the same manner that a cell is
selected as described in 33.4. In all cases, library mapping associated with the bind_target_instance shall be
ignored during elaboration of the bind_instantiation.
Any defparam statement located at a lower level of the bind_instantiation’s hierarchy shall not extend
influence outside the scope of that local hierarchy. This is similar to the rules for use of defparam inside the
scope of generated hierarchy.
Hierarchical references to a bind_instantiation’s parameters may not be used outside the instantiation in any
context that requires a constant expression. Examples of such contexts include type descriptions and
generate conditions.
It is legal for more than one bind statement to bind a bind_instantiation into the same target scope.
However, it shall be an error for a bind_instantiation to introduce an instance name that clashes with another
name in the module name space of the target scope (see 3.13). This applies to both preexisting names as well
as instance names introduced by other bind statements. The latter situation will occur if the design contains
more than one instance of a module containing a bind statement.
It shall be an error for a bind statement to bind a bind_instantiation underneath the scope of another
bind_instantiation.

<!-- Page 775 -->

IEEE Std 1800-2023