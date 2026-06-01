# Chapter 25: Interfaces

## 25. Interfaces
### 25.1 General
This clause describes the following:
— Purpose of interfaces
— Interface syntax
— Interface modports
— Interface methods
— Parameterized interfaces
— Virtual interfaces
— Accessing interface objects
### 25.2 Overview
The communication between blocks of a digital system is a critical area that can affect everything from ease
of RTL coding to hardware-software partitioning to performance analysis to bus implementation choices
and protocol checking. The interface construct in SystemVerilog was specifically created to encapsulate the
communication between blocks, allowing a smooth migration from abstract system-level design through
successive refinement down to lower level register-transfer and structural views of the design. By
encapsulating the communication between blocks, the interface construct also facilitates design reuse. The
inclusion of interface capabilities is an important advantage of SystemVerilog.
At its lowest level, an interface is a named bundle of nets or variables. The interface is instantiated in a
design and can be accessed through a port as a single item, and the component nets or variables referenced
where needed. A significant proportion of a design often consists of port lists and port connection lists,
which are just repetitions of names. The ability to replace a group of names by a single name can
significantly reduce the size of a description and improve its maintainability.
Additional power of the interface comes from its ability to encapsulate functionality as well as connectivity,
making an interface, at its highest level, more like a class template. An interface can have parameters,
constants, variables, functions, and tasks. The types of elements in an interface can be declared, or the types
can be passed in as parameters. The member variables and functions are referenced relative to the instance
name of the interface as instance members. Thus, modules that are connected via an interface can simply call
the subroutine members of that interface to drive the communication. With the functionality thus
encapsulated in the interface and isolated from the module, the abstraction level and/or granularity of the
communication protocol can be easily changed by replacing the interface with a different interface
containing the same members, but implemented at a different level of abstraction. The modules connected
via the interface do not need to change at all.
To provide direction information for module ports and to control the use of tasks and functions within
particular modules, the modport construct is provided. As the name indicates, the directions are those seen
from the module.
In addition to subroutine methods, an interface can also contain processes (i.e., initial or always
procedures) and continuous assignments, which are useful for system-level modeling and testbench
applications. This allows the interface to include, for example, its own protocol checker that automatically
verifies that all modules connected via the interface conform to the specified protocol. Other applications,
such as functional coverage recording and reporting, protocol checking, and assertions, can also be built into
the interface.

<!-- Page 782 -->

IEEE Std 1800/-2023

The methods can be abstract, i.e., defined in one module and called in another, using the export and
import constructs. This could be coded using hierarchical path names, but this would impede reuse because
the names would be design-specific. A better way is to declare the subroutine names in the interface and to
use local hierarchical names from the interface instance for both definition and call. Broadcast
communication is modeled by forkjoin tasks, which can be defined in more than one module and
executed concurrently.
### 25.3 Interface syntax
interface_declaration ::= // from A.1.2
interface_nonansi_header [ timeunits_declaration ] { interface_item }
endinterface [ : interface_identifier ]
| interface_ansi_header [ timeunits_declaration ] { non_port_interface_item }
endinterface [ : interface_identifier ]
| { attribute_instance } interface interface_identifier ( . * ) ;
[ timeunits_declaration ] { interface_item }
endinterface [ : interface_identifier ]
| extern interface_nonansi_header
| extern interface_ansi_header
interface_nonansi_header ::=
{ attribute_instance } interface [ lifetime ] interface_identifier
{ package_import_declaration } [ parameter_port_list ] list_of_ports ;
interface_ansi_header ::=
{ attribute_instance } interface [ lifetime ] interface_identifier
{ package_import_declaration }1 [ parameter_port_list ] [ list_of_port_declarations ] ;
interface_item ::= // from A.1.6
port_declaration ;
| non_port_interface_item
non_port_interface_item ::=
generate_region
| interface_or_generate_item
| program_declaration
| modport_declaration
| interface_declaration
| timeunits_declaration3
modport_declaration ::= modport modport_item { , modport_item } ; // from A.2.9
modport_item ::= modport_identifier ( modport_ports_declaration { , modport_ports_declaration } )
modport_ports_declaration ::=
{ attribute_instance } modport_simple_ports_declaration
| { attribute_instance } modport_tf_ports_declaration
| { attribute_instance } modport_clocking_declaration
modport_clocking_declaration ::= clocking clocking_identifier
modport_simple_ports_declaration ::=
port_direction modport_simple_port { , modport_simple_port }
modport_simple_port ::=
port_identifier
| . port_identifier ( [ expression ] )
modport_tf_ports_declaration ::=
import_export modport_tf_port { , modport_tf_port }
modport_tf_port ::=

<!-- Page 783 -->

IEEE Std 1800/-2023

method_prototype
| tf_identifier
import_export ::= import | export
interface_instantiation ::= // from A.4.1.2
interface_identifier [ parameter_value_assignment ] hierarchical_instance { , hierarchical_instance } ;
1) A package_import_declaration in a module_ansi_header, interface_ansi_header, or program_ansi_header shall
be followed by a parameter_port_list or list_of_port_declarations, or both.
3) A timeunits_declaration shall be legal as a non_port_module_item, non_port_interface_item,
non_port_program_item, or package_item only if it repeats and matches a previous timeunits_declaration within
the same time scope.
Syntax25-1—Interface syntax (excerpt from AnnexA)
The interface construct provides a new hierarchical structure. It can contain smaller interfaces and can be
passed through ports.
The aim of interfaces is to encapsulate communication. At the lower level, this means bundling variables
and nets in interfaces and can impose access restrictions with port directions in modports. The modules can
be made generic so that the interfaces can be changed. The following examples show these features. At a
higher level of abstraction, communication can be done by tasks and functions. Interfaces can include
subroutine definitions or just subroutine prototypes, with the definition in one module and the call in another
(see 25.7 and 25.7.3).
A simple interface declaration is as follows (see Syntax25-1 for the complete syntax):
interface identifier;
...
interface_items
...
endinterface [ : identifier ]
An interface can be instantiated hierarchically like a module, with or without ports (see 23.3). For example:
myinterface #(100) scalar1(), vector[9:0]();
In this example, 11 instances of the interface of type myinterface have been instantiated, and the first
parameter within each interface is changed to 100. One myinterface instance is instantiated with the name
scalar1, and an array of 10 myinterface interfaces are instantiated with instance names vector[9] to
vector[0].
Interfaces can be declared and instantiated in modules (either flat or hierarchical), but modules can neither
be declared nor instantiated in interfaces. In contrast to modules (see 23.3) and programs (see 24.3),
interfaces are never implicitly instantiated.
A defparam within an instance whose port actuals refer to an arrayed interface shall not modify a parameter
outside the hierarchy of such an instance. If the actual of an interface port connection is a hierarchical
reference to an interface or a modport of a hierarchically referenced interface, the hierarchical reference
shall refer to an interface instance and shall not resolve through an arrayed instance or a generate block.
The simplest use of an interface is to bundle wires, as illustrated in the following examples.

<!-- Page 784 -->

IEEE Std 1800/-2023

#### 25.3.1 Example without using interfaces
This example shows a simple bus implemented without interfaces.
module memMod( input logic req,
logic clk,
logic start,
logic [1:0] mode,
logic [7:0] addr,
inout wire [7:0] data,
output bit gnt,
bit rdy );
logic avail;
...
endmodule
module cpuMod(
input logic clk,
logic gnt,
logic rdy,
inout wire [7:0] data,
output logic req,
logic start,
logic [7:0] addr,
logic [1:0] mode );
...
endmodule
module top;
logic req, gnt, start, rdy;
logic clk = 0;
logic [1:0] mode;
logic [7:0] addr;
wire [7:0] data;
memMod mem(req, clk, start, mode, addr, data, gnt, rdy);
cpuMod cpu(clk, gnt, rdy, data, req, start, addr, mode);
endmodule
#### 25.3.2 Interface example using a named bundle
The simplest form of a SystemVerilog interface is a bundled collection of variables or nets. When an
interface is referenced as a port, the variables and nets in it are assumed to have ref and inout access,
respectively. The following interface example shows the basic syntax for defining, instantiating, and
connecting an interface. Usage of the SystemVerilog interface capability can significantly reduce the
amount of code required to model port connections.
interface simple_bus; // Define the interface
logic req, gnt;
logic [7:0] addr, data;
logic [1:0] mode;
logic start, rdy;
endinterface: simple_bus
module memMod(simple_bus a, // Access the simple_bus interface
input logic clk);

<!-- Page 785 -->

IEEE Std 1800/-2023

logic avail;
// When memMod is instantiated in module top, a.req is the req
// signal in the sb_intf instance of the 'simple_bus' interface
always @(posedge clk) a.gnt <= a.req & avail;
endmodule
module cpuMod(simple_bus b, input logic clk);
...
endmodule
module top;
logic clk = 0;
simple_bus sb_intf(); // Instantiate the interface
memMod mem(sb_intf, clk); // Connect the interface to the module instance
cpuMod cpu(.b(sb_intf), .clk(clk)); // Either by position or by name
endmodule
In the preceding example, if the same identifier, sb_intf, had been used to name the simple_bus
interface in the memMod and cpuMod module headers, then implicit port connections also could have been
used to instantiate the memMod and cpuMod modules into the top module, as follows:
module memMod (simple_bus sb_intf, input logic clk);
...
endmodule
module cpuMod (simple_bus sb_intf, input logic clk);
...
endmodule
module top;
logic clk = 0;
simple_bus sb_intf();
memMod mem (.*); // implicit port connections
cpuMod cpu (.*); // implicit port connections
endmodule
#### 25.3.3 Interface example using a generic bundle
A module header can be created with an unspecified interface reference as a placeholder for an interface to
be selected when the module itself is instantiated. The unspecified interface is referred to as a generic
interface reference.
This generic interface reference can only be declared using the ANSI style list_of_port_declarations syntax
(see 23.2.2.2). It shall be illegal to declare such a generic interface reference using the non-ANSI style
list_of_ports syntax (see 23.2.2.1).
The following interface example shows how to specify a generic interface reference in a module definition:
// memMod and cpuMod can use any interface
module memMod (interface a, input logic clk);
...
endmodule

<!-- Page 786 -->

IEEE Std 1800/-2023

module cpuMod(interface b, input logic clk);
...
endmodule
interface simple_bus; // Define the interface
logic req, gnt;
logic [7:0] addr, data;
logic [1:0] mode;
logic start, rdy;
endinterface: simple_bus
module top;
logic clk = 0;
simple_bus sb_intf(); // Instantiate the interface
// Reference the sb_intf instance of the simple_bus
// interface from the generic interfaces of the
// memMod and cpuMod modules
memMod mem (.a(sb_intf), .clk(clk));
cpuMod cpu (.b(sb_intf), .clk(clk));
endmodule
An implicit port cannot be used to reference a generic interface. A named port shall be used to reference a
generic interface, as follows:
module memMod (interface a, input logic clk);
...
endmodule
module cpuMod (interface b, input logic clk);
...
endmodule
module top;
logic clk = 0;
simple_bus sb_intf();
memMod mem (.*, .a(sb_intf)); // partial implicit port connections
cpuMod cpu (.*, .b(sb_intf)); // partial implicit port connections
endmodule
### 25.4 Ports in interfaces
One limitation of simple interfaces is that the nets and variables declared within the interface are only used
to connect to a port with the same nets and variables. To share an external net or variable, one that makes a
connection from outside the interface as well as forming a common connection to all module ports that
instantiate the interface, an interface port declaration is required. The difference between nets or variables in
the interface port list and other nets or variables within the interface is that only those in the port list can be
connected externally by name or position when the interface is instantiated. Interface port declaration syntax
and semantics are the same as those of modules (see 23.2.2).

<!-- Page 787 -->

IEEE Std 1800/-2023

interface i1 (input a, output b, inout c);
wire d;
endinterface
The wires a, b, and c can be individually connected to the interface and thus shared with other interfaces.
The following example shows how to specify an interface with inputs, allowing a wire to be shared between
two instances of the interface:
interface simple_bus (input logic clk); // Define the interface
logic req, gnt;
logic [7:0] addr, data;
logic [1:0] mode;
logic start, rdy;
endinterface: simple_bus
module memMod(simple_bus a); // Uses just the interface
logic avail;
always @(posedge a.clk) // the clk signal from the interface
a.gnt <= a.req & avail; // a.req is in the 'simple_bus' interface
endmodule
module cpuMod(simple_bus b);
...
endmodule
module top;
logic clk = 0;
simple_bus sb_intf1(clk); // Instantiate the interface
simple_bus sb_intf2(clk); // Instantiate the interface
memMod mem1(.a(sb_intf1)); // Reference simple_bus 1 to memory 1
cpuMod cpu1(.b(sb_intf1));
memMod mem2(.a(sb_intf2)); // Reference simple_bus 2 to memory 2
cpuMod cpu2(.b(sb_intf2));
endmodule
In the preceding example, the instantiated interface names do not match the interface names used in the
memMod and cpuMod modules; therefore, implicit port connections cannot be used for this example.
### 25.5 Modports
To restrict interface access within a module, there are modport lists with directions declared within the
interface. The keyword modport indicates that the directions are declared as if inside the module.
interface i2;
wire a, b, c, d;
modport initiator (input a, b, output c, d);
modport target (output a, b, input c, d);
endinterface

<!-- Page 788 -->

IEEE Std 1800/-2023

In this example, the modport list name (initiator or target) can be specified in the module header,
where the interface name selects an interface and the modport name selects the appropriate directional
information for the interface signals accessed in the module header.
module m (i2.initiator i);
...
endmodule
module s (i2.target i);
...
endmodule
module top;
i2 i();
m u1(.i(i));
s u2(.i(i));
endmodule
The syntax of interface_name.modport_name reference_name gives a local name for a hierarchical
reference. This technique can be generalized to any interface with a given modport name by writing
interface.modport_name reference_name.
The modport list name (initiator or target) can also be specified in the port connection with the
module instance, where the modport name is hierarchical from the interface instance.
module m (i2 i);
...
endmodule
module s (i2 i);
...
endmodule
module top;
i2 i();
m u1(.i(i.initiator));
s u2(.i(i.target));
endmodule
If a port connection specifies a modport list name in both the module instance and module header
declaration, then the two modport list names shall be identical.
All of the names used in a modport declaration shall be declared by the same interface as the modport itself.
In particular, the names used shall not be those declared by another enclosing interface, and a modport
declaration shall not implicitly declare new ports.
The following interface declarations would be illegal:
interface i;
wire x, y;
interface illegal_i;
wire a, b, c, d;
// x, y not declared by this interface
modport initiator(input a, b, x, output c, d, y);

<!-- Page 789 -->

IEEE Std 1800/-2023

modport target(output a, b, x, input c, d, y);
endinterface : illegal_i
endinterface : i
interface illegal_i;
// a, b, c, d not declared by this interface
modport initiator(input a, b, output c, d);
modport target (output a, b, input c, d);
endinterface : illegal_i
Adding modports to an interface does not require that any of the modports be used when the interface is
used. If no modport is specified in the module header or in the port connection, then all the nets and
variables in the interface are accessible with direction inout or ref, as in the preceding examples.
#### 25.5.1 Example of named port bundle
This interface example shows how to use modports to control signal directions as in port declarations. It uses
the modport name in the module definition.
interface simple_bus (input logic clk); // Define the interface
logic req, gnt;
logic [7:0] addr, data;
logic [1:0] mode;
logic start, rdy;
modport target (input req, addr, mode, start, clk,
output gnt, rdy,
ref data);
modport initiator(input gnt, rdy, clk,
output req, addr, mode, start,
ref data);
endinterface: simple_bus
module memMod (simple_bus.target a); // interface name and modport name
logic avail;
always @(posedge a.clk) // the clk signal from the interface
a.gnt <= a.req & avail; // the gnt and req signal in the interface
endmodule
module cpuMod (simple_bus.initiator b);
...
endmodule
module top;
logic clk = 0;
simple_bus sb_intf(clk); // Instantiate the interface
initial repeat(10) #10 clk++;
memMod mem(.a(sb_intf)); // Connect the interface to the module instance
cpuMod cpu(.b(sb_intf));
endmodule

<!-- Page 790 -->

IEEE Std 1800/-2023

#### 25.5.2 Example of connecting port bundle
This interface example shows how to use modports to restrict interface signal access and control their
direction. It uses the modport name in the module instantiation.
interface simple_bus (input logic clk); // Define the interface
logic req, gnt;
logic [7:0] addr, data;
logic [1:0] mode;
logic start, rdy;
modport target (input req, addr, mode, start, clk,
output gnt, rdy,
ref data);
modport initiator(input gnt, rdy, clk,
output req, addr, mode, start,
ref data);
endinterface: simple_bus
module memMod(simple_bus a); // Uses just the interface name
logic avail;
always @(posedge a.clk) // the clk signal from the interface
a.gnt <= a.req & avail; // the gnt and req signal in the interface
endmodule
module cpuMod(simple_bus b);
...
endmodule
module top;
logic clk = 0;
simple_bus sb_intf(clk); // Instantiate the interface
initial repeat(10) #10 clk++;
memMod mem(sb_intf.target); // Connect the modport to the module instance
cpuMod cpu(sb_intf.initiator);
endmodule
#### 25.5.3 Example of connecting port bundle to generic interface
This interface example shows how to use modports to control signal directions. It shows the use of the
interface keyword in the module definition. The actual interface and modport are specified in the module
instantiation.
interface simple_bus (input logic clk); // Define the interface
logic req, gnt;
logic [7:0] addr, data;
logic [1:0] mode;
logic start, rdy;
modport target (input req, addr, mode, start, clk,
output gnt, rdy,
ref data);

<!-- Page 791 -->

IEEE Std 1800/-2023

modport initiator(input gnt, rdy, clk,
output req, addr, mode, start,
ref data);
endinterface: simple_bus
module memMod(interface a); // Uses just the interface
logic avail;
always @(posedge a.clk) // the clk signal from the interface
a.gnt <= a.req & avail; // the gnt and req signal in the interface
endmodule
module cpuMod(interface b);
...
endmodule
module top;
logic clk = 0;
simple_bus sb_intf(clk); // Instantiate the interface
memMod mem(sb_intf.target); // Connect the modport to the module instance
cpuMod cpu(sb_intf.initiator);
endmodule
#### 25.5.4 Modport expressions
A modport expression allows elements of arrays and structures, concatenations of elements, and assignment
pattern expressions of elements declared in an interface to be included in a modport list. This modport
expression is explicitly named with a port identifier, visible only through the modport connection.
Like explicitly named ports in a module port declaration, port identifiers exist in their own name space for
each modport list. When a modport item is just a simple port identifier, that identifier is used as both a
reference to an interface item and a port identifier. Once a port identifier has been defined, there shall not be
another port definition with this same name.
For example:
interface I;
logic [7:0] r;
const int x=1;
bit R;
modport A (output .P(r[3:0]), input .Q(x), R);
modport B (output .P(r[7:4]), input .Q(2), R);
endinterface
module M ( interface i);
initial i.P = i.Q;
endmodule
module top;
I i1 ();
M u1 (i1.A);
M u2 (i1.B);
initial #1 $display("%b", i1.r); // displays 00100001
endmodule

<!-- Page 792 -->

IEEE Std 1800/-2023

The self-determined type of the port expression becomes the type for the port. The port expression shall not
be considered an assignment-like context. The port expression shall resolve to a legal expression for the type
of module port (see 23.3.3). In the preceding example, the Q port could not be an output or inout because the
port expression is a constant. The port expression is optional because ports can be defined that do not
connect to anything internal to the port.
#### 25.5.5 Clocking blocks and modports
The modport construct can also be used to specify the direction of clocking blocks declared within an
interface. As with other modport declarations, the directions of the clocking block are those seen from the
module in which the interface becomes a port. The syntax for this is shown in Syntax25-2.
modport_declaration ::= modport modport_item { , modport_item } ; // from A.2.9
modport_item ::= modport_identifier ( modport_ports_declaration { , modport_ports_declaration } )
modport_ports_declaration ::=
{ attribute_instance } modport_simple_ports_declaration
| { attribute_instance } modport_tf_ports_declaration
| { attribute_instance } modport_clocking_declaration
modport_clocking_declaration ::= clocking clocking_identifier
Syntax25-2—Modport clocking declaration syntax (excerpt from AnnexA)
All of the clocking blocks used in a modport declaration shall be declared by the same interface as the
modport itself. Like all modport declarations, the direction of the clocking signals are those seen from the
module in which the interface becomes a port. The following example shows how modports can be used to
create both synchronous as well as asynchronous ports. When used in conjunction with virtual interfaces
(see 25.9.2), these constructs facilitate the creation of abstract synchronous models.
interface A_Bus( input logic clk );
wire req, gnt;
wire [7:0] addr, data;
clocking sb @(posedge clk);
input gnt;
output req, addr;
inout data;
property p1; req ##[1:3] gnt; endproperty
endclocking
modport DUT ( input clk, req, addr, // Device under test modport
output gnt,
inout data );
modport STB ( clocking sb ); // synchronous testbench modport
modport TB ( input gnt, // asynchronous testbench modport
output req, addr,
inout data );
endinterface
The preceding interface A_Bus can then be instantiated as follows:

<!-- Page 793 -->

IEEE Std 1800/-2023

module dev1(A_Bus.DUT b); // Some device: Part of the design
...
endmodule
module dev2(A_Bus.DUT b); // Some device: Part of the design
...
endmodule
module top;
logic clk;
A_Bus b1( clk );
A_Bus b2( clk );
dev1 d1( b1 );
dev2 d2( b2 );
T tb( b1, b2 );
endmodule
program T (A_Bus.STB b1, A_Bus.STB b2 ); // testbench: 2 synchronous ports
assert property (b1.sb.p1); // assert property from within program
initial begin
b1.sb.req <= 1;
wait( b1.sb.gnt == 1 );
...
b1.sb.req <= 0;
b2.sb.req <= 1;
wait( b2.sb.gnt == 1 );
...
b2.sb.req <= 0;
end
endprogram
This example shows the program block using the synchronous interface designated by the clocking modport
of interface ports b1 and b2. In addition to the procedural drives and samples of the clocking block signals,
the program asserts the property p1 of one of its interfaces b1.
### 25.6 Interfaces and specify blocks
The specify block is used to describe various paths across a module and perform timing checks to verify
that events occurring at the module inputs satisfy the timing constraints of the device described by the
module. The module paths are from module input ports to output ports, and the timing checks are relative to
the module inputs. The specify block refers to these ports as terminal descriptor. Module inout ports can
function as either an input or output terminal. When one of the port instances is an interface, each signal in
the interface becomes an available terminal, with the default direction as defined for an interface or as
restricted by a modport. A ref port cannot be used as a terminal in a specify block.
The following shows an example of using interfaces together with a specify block:
interface itf;
logic c,q,d;
modport flop (input c,d, output q);
endinterface

<!-- Page 794 -->

IEEE Std 1800/-2023

module dtype (itf.flop ch);
always_ff @(posedge ch.c) ch.q <= ch.d;
specify
( posedge ch.c => (ch.q+:ch.d)) = (5,6);
$setup( ch.d, posedge ch.c, 1 );
endspecify
endmodule
### 25.7 Tasks and functions in interfaces
Subroutines (tasks and functions) can be defined within an interface, or they can be defined within one or
more of the modules connected. This allows a more abstract level of modeling. For example, “read” and
“write” can be defined as tasks, without reference to any wires, and the initiator module can merely call
these tasks. In a modport, these tasks are declared as import tasks.
A function prototype specifies the types and directions of the arguments and the return value of a function
that is defined elsewhere. Similarly, a task prototype specifies the types and directions of the arguments of a
task that is defined elsewhere. In a modport, the import and export constructs can either use subroutine
prototypes or use just the identifiers. The only exceptions are when a modport is used to import a subroutine
from another module and when default argument values or argument binding by name is used, in which
cases a full prototype shall be used.
The number and types of arguments in a prototype shall match the argument types in the subroutine
declaration. The rules for type matching are described in 6.22.1. If a default argument value is needed in a
subroutine call, it shall be specified in the prototype. If an argument has default values specified in both the
prototype and the declaration, the specified values need not be the same, but the default value used shall be
the one specified in the prototype. Formal argument names in a prototype shall be optional unless default
argument values or argument binding by name is used or additional unpacked dimensions are declared. The
formal argument names in the prototype shall be the same as the formal argument names in a declaration.
If a module is connected to a modport containing an exported subroutine and the module does not define that
subroutine, then an elaboration error shall occur. Similarly, if the modport contains an exported subroutine
prototype and the subroutine defined in the module does not exactly match that prototype, then an
elaboration error shall occur.
If the subroutines are defined in a module using a hierarchical name, they shall also be declared as extern
in the interface or as export in a modport.
Tasks (not functions) can be defined in a module that is instantiated twice, e.g., two memories driven from
the same CPU. Such multiple task definitions are allowed by an extern forkjoin declaration in the
interface.
#### 25.7.1 Example of using tasks in interface
interface simple_bus (input logic clk); // Define the interface
logic req, gnt;
logic [7:0] addr, data;
logic [1:0] mode;
logic start, rdy;
task initiatorRead(input logic [7:0] raddr); // initiatorRead method
// ...
endtask: initiatorRead

<!-- Page 795 -->

IEEE Std 1800/-2023

task targetRead; // targetRead method
// ...
endtask: targetRead
endinterface: simple_bus
module memMod(interface a); // Uses any interface
logic avail;
always @(posedge a.clk) // the clk signal from the interface
a.gnt <= a.req & avail // the gnt and req signals in the interface
always @(a.start)
a.targetRead;
endmodule
module cpuMod(interface b);
enum {read, write} instr;
logic [7:0] raddr;
always @(posedge b.clk)
if (instr == read)
b.initiatorRead(raddr); // call the interface method
endmodule
module top;
logic clk = 0;
simple_bus sb_intf(clk); // Instantiate the interface
memMod mem(sb_intf);
cpuMod cpu(sb_intf);
endmodule
#### 25.7.2 Example of using tasks in modports
This interface example shows how to use modports to control signal directions and task access in a full read/
write interface.
interface simple_bus (input logic clk); // Define the interface
logic req, gnt;
logic [7:0] addr, data;
logic [1:0] mode;
logic start, rdy;
modport target (input req, addr, mode, start, clk,
output gnt, rdy,
ref data,
import targetRead,
targetWrite);
// import into module that uses the modport
modport initiator(input gnt, rdy, clk,
output req, addr, mode, start,
ref data,
import initiatorRead,
initiatorWrite);
// import into module that uses the modport

<!-- Page 796 -->

IEEE Std 1800/-2023

task initiatorRead(input logic [7:0] raddr); // initiatorRead method
// ...
endtask
task targetRead; // targetRead method
// ...
endtask
task initiatorWrite(input logic [7:0] waddr);
//...
endtask
task targetWrite;
//...
endtask
endinterface: simple_bus
module memMod(interface a); // Uses just the interface
logic avail;
always @(posedge a.clk) // the clk signal from the interface
a.gnt <= a.req & avail; // the gnt and req signals in the interface
always @(a.start)
if (a.mode[0] == 1'b0)
a.targetRead;
else
a.targetWrite;
endmodule
module cpuMod(interface b);
enum {read, write} instr;
logic [7:0] raddr = $random();
always @(posedge b.clk)
if (instr == read)
b.initiatorRead(raddr); // call the interface method
else
b.initiatorWrite(raddr);
endmodule
module omniMod( interface b);
//...
endmodule: omniMod
module top;
logic clk = 0;
simple_bus sb_intf(clk); // Instantiate the interface
memMod mem(sb_intf.target); // only has access to the target tasks
cpuMod cpu(sb_intf.initiator); // only has access to the initiator tasks
omniMod omni(sb_intf); // has access to all initiator and target tasks
endmodule

<!-- Page 797 -->

IEEE Std 1800/-2023

#### 25.7.3 Example of exporting tasks and functions
This interface example shows how to define tasks in one module and call them in another, using modports to
control task access.
interface simple_bus (input logic clk); // Define the interface
logic req, gnt;
logic [7:0] addr, data;
logic [1:0] mode;
logic start, rdy;
modport target(input req, addr, mode, start, clk,
output gnt, rdy,
ref data,
export Read,
Write);
// export from module that uses the modport
modport initiator(input gnt, rdy, clk,
output req, addr, mode, start,
ref data,
import task Read (input logic [7:0] raddr),
task Write(input logic [7:0] waddr));
// import requires the full task prototype
endinterface: simple_bus
module memMod(interface a); // Uses just the interface keyword
logic avail;
task a.Read; // Read method
avail = 0;
...
avail = 1;
endtask
task a.Write;
avail = 0;
...
avail = 1;
endtask
endmodule
module cpuMod(interface b);
enum {read, write} instr;
logic [7:0] raddr;
always @(posedge b.clk)
if (instr == read)
b.Read(raddr); // call the target method via the interface
else
b.Write(raddr);
endmodule
module top;
logic clk = 0;
simple_bus sb_intf(clk); // Instantiate the interface

<!-- Page 798 -->

IEEE Std 1800/-2023

memMod mem(sb_intf.target); // exports the Read and Write tasks
cpuMod cpu(sb_intf.initiator); // imports the Read and Write tasks
endmodule
#### 25.7.4 Example of multiple task exports
It is normally an error for more than one module to export the same task name. However, several instances
of the same modport type can be connected to an interface, such as memory modules in the previous
example. So that these can still export their read and write tasks, the tasks shall be declared in the interface
using the extern forkjoin keywords.
The call to extern forkjoin task countTargets( ); in the following example behaves as follows:
fork
top.mem1.a.countTargets;
top.mem2.a.countTargets;
join
For a read task, only one module should actively respond to the task call, e.g., the one containing the
appropriate address. The tasks in the other modules should return with no effect. Only then should the active
task write to the result variables.
Unlike tasks, multiple export of functions is not allowed because they always write to the result.
The effect of a disable on an extern forkjoin task is as follows:
— If the task is referenced via the interface instance, all task calls shall be disabled.
— If the task is referenced via the module instance, only the task call to that module instance shall be
disabled.
— If an interface contains an extern forkjoin task and no module connected to that interface
defines the task, then any call to that task shall report a run-time error and return immediately with
no effect.
This interface example shows how to define tasks in more than one module and call them in another using
extern forkjoin. The multiple task export mechanism can also be used to count the instances of a
particular modport that are connected to each interface instance.
interface simple_bus (input logic clk); // Define the interface
logic req, gnt;
logic [7:0] addr, data;
logic [1:0] mode;
logic start, rdy;
int targets = 0;
// tasks executed concurrently as a fork-join block
extern forkjoin task countTargets();
extern forkjoin task Read (input logic [7:0] raddr);
extern forkjoin task Write (input logic [7:0] waddr);
modport target (input req,addr, mode, start, clk,
output gnt, rdy,
ref data, targets,
export Read, Write, countTargets);
// export from module that uses the modport

<!-- Page 799 -->

IEEE Std 1800/-2023

modport initiator (input gnt, rdy, clk,
output req, addr, mode, start,
ref data,
import task Read (input logic [7:0] raddr),
task Write(input logic [7:0] waddr));
// import requires the full task prototype
initial begin
targets = 0;
countTargets;
$display ("number of targets = %d", targets);
end
endinterface: simple_bus
module memMod #(parameter int minaddr=0, maxaddr=0;) (interface a);
logic avail = 1;
logic [7:0] mem[255:0];
task a.countTargets();
a.targets++;
endtask
task a.Read(input logic [7:0] raddr); // Read method
if (raddr >= minaddr && raddr <= maxaddr) begin
avail = 0;
#10 a.data = mem[raddr];
avail = 1;
end
endtask
task a.Write(input logic [7:0] waddr); // Write method
if (waddr >= minaddr && waddr <= maxaddr) begin
avail = 0;
#10 mem[waddr] = a.data;
avail = 1;
end
endtask
endmodule
module cpuMod(interface b);
typedef enum {read, write} instr;
instr inst;
logic [7:0] raddr;
integer seed;
always @(posedge b.clk) begin
inst = instr'($dist_uniform(seed, 0, 1));
raddr = $dist_uniform(seed, 0, 3);
if (inst == read) begin
$display("%t begin read %h @ %h", $time, b.data, raddr);
callr:b.Read(raddr);
$display("%t end read %h @ %h", $time, b.data, raddr);
end
else begin
$display("%t begin write %h @ %h", $time, b.data, raddr);
b.data = raddr;
callw:b.Write(raddr);
$display("%t end write %h @ %h", $time, b.data, raddr);

<!-- Page 800 -->

IEEE Std 1800/-2023

end
end
endmodule
module top;
logic clk = 0;
function void interrupt();
disable mem1.a.Read; // task via module instance
disable sb_intf.Write; // task via interface instance
if (mem1.avail == 0) $display ("mem1 was interrupted");
if (mem2.avail == 0) $display ("mem2 was interrupted");
endfunction
always #5 clk++;
initial begin
#28 interrupt();
#10 interrupt();
#100 $finish;
end
simple_bus sb_intf(clk);
memMod #( 0, 127) mem1(sb_intf.target);
memMod #(128, 255) mem2(sb_intf.target);
cpuMod cpu(sb_intf.initiator);
endmodule
### 25.8 Parameterized interfaces
Interface definitions can take advantage of parameters and parameter redefinition in the same manner as
module definitions. The following example shows how to use parameters in interface definitions.
interface simple_bus #(AWIDTH = 8, DWIDTH = 8)
(input logic clk); // Define the interface
logic req, gnt;
logic [AWIDTH-1:0] addr;
logic [DWIDTH-1:0] data;
logic [1:0] mode;
logic start, rdy;
modport target(input req, addr, mode, start, clk,
output gnt, rdy,
ref data,
import task targetRead,
task targetWrite);
// import into module that uses the modport
modport initiator(input gnt, rdy, clk,
output req, addr, mode, start,
ref data,
import
task initiatorRead (input logic [AWIDTH-1:0] raddr),
task initiatorWrite(input logic [AWIDTH-1:0] waddr));
// import requires the full task prototype
task initiatorRead(input logic [AWIDTH-1:0] raddr);

<!-- Page 801 -->

IEEE Std 1800/-2023

...
endtask
task targetRead; // targetRead method
...
endtask
task initiatorWrite(input logic [AWIDTH-1:0] waddr);
...
endtask
task targetWrite;
...
endtask
endinterface: simple_bus
module memMod(interface a); // Uses just the interface keyword
logic avail;
always @(posedge a.clk) // the clk signal from the interface
a.gnt <= a.req & avail; //the gnt and req signals in the interface
always @(a.start)
if (a.mode[0] == 1'b0)
a.targetRead;
else
a.targetWrite;
endmodule
module cpuMod(interface b);
enum {read, write} instr;
logic [7:0] raddr;
always @(posedge b.clk)
if (instr == read)
b.initiatorRead(raddr); // call the interface method
// ...
else
b.initiatorWrite(raddr);
endmodule
module top;
logic clk = 0;
simple_bus sb_intf(clk); // Instantiate default interface
simple_bus #(.DWIDTH(16)) wide_intf(clk); // Interface with 16-bit data
initial repeat(10) #10 clk++;
memMod mem(sb_intf.target); // only has access to targetRead task
cpuMod cpu(sb_intf.initiator); // only has access to initiatorRead task
memMod memW(wide_intf.target); // 16-bit wide memory
cpuMod cpuW(wide_intf.initiator); // 16-bit wide cpu
endmodule

<!-- Page 802 -->

IEEE Std 1800/-2023

### 25.9 Virtual interfaces
Virtual interfaces provide a mechanism for separating abstract models and test programs from the actual
signals that make up the design. A virtual interface allows the same subprogram to operate on different
portions of a design and to dynamically control the set of signals associated with the subprogram. Instead of
referring to the actual set of signals directly, users are able to manipulate a set of virtual signals. Changes to
the underlying design do not require the code using virtual interfaces to be rewritten. By abstracting the
connectivity and functionality of a set of blocks, virtual interfaces promote code reuse.
A virtual interface is a variable that represents an interface instance. The syntax to declare a virtual interface
variable is given in Syntax25-3.
data_declaration ::= // from A.2.1.3
[ const ] [ var ] [ lifetime ] data_type_or_implicit list_of_variable_decl_assignments ; 14
| ...
data_type ::= // from A.2.2.1
...
| virtual [ interface ] interface_identifier [ parameter_value_assignment ] [ . modport_identifier ]
14) In a data_declaration that is not within a procedural context, it shall be illegal to use the automatic keyword. In
a data_declaration, it shall be illegal to omit the explicit data_type before a list_of_variable_decl_assignments
unless the var keyword is used.
Syntax25-3—Virtual interface declaration syntax (excerpt from AnnexA)
Virtual interface variables may be passed as arguments to tasks, functions, or methods. A single virtual
interface variable can thus represent different interface instances at different times throughout the
simulation. A virtual interface shall be initialized before referencing a component of the virtual interface; it
has the value null before it is initialized. Attempting to use a null virtual interface shall result in a fatal
run-time error.
The type of an interface shall include actual parameters, default or overridden, used in the instantiation of an
interface or the declaration of a virtual interface variable. The actual values and types of those parameters
shall match for an interface and virtual interface to be of the same type and to be assignment compatible (see
6.22.3). A virtual interface declaration may select a modport of an interface in which case the modport is
also part of its type. An interface instance or virtual interface with no modport selected may be assigned to a
virtual interface with a modport selected.
It shall be illegal to assign an interface instance to a virtual interface if there is a defparam to a parameter of
that interface instance or interface hierarchy and that defparam statement is declared outside the interface.
Although an interface may contain hierarchical references to objects outside its body or ports that reference
other interfaces, it shall be illegal to use an interface containing those references in the declaration of a
virtual interface.
Only the following operations are directly allowed on virtual interface variables:
— Assignment ( = ) from the following:
• Another virtual interface of the same type
• An interface instance of the same type
• The special constant null

<!-- Page 803 -->

IEEE Std 1800/-2023

— Equality ( == ) and inequality ( != ) with the following:
• Another virtual interface of the same type
• An interface instance of the same type
• The special constant null
Virtual interfaces shall not be used as ports, interface items, or as members of unions.
Once a virtual interface has been initialized, all the components of the underlying interface instance are
directly available to the virtual interface via the dot notation. These components can only be used in
procedural statements; they cannot be used in continuous assignments or sensitivity lists. In order for a net to
be driven via a virtual interface, the interface itself needs to provide a procedural means to do so. This can be
accomplished either via a clocking block or by including a driver that is updated by a continuous assignment
from a variable within the interface.
Virtual interfaces can be declared as class properties, which can be initialized procedurally or by an
argument to new(). This allows the same virtual interface to be used in different classes. The following
example shows how the same transactor class can be used to interact with various different devices:
interface SBus; // A Simple bus interface
logic req, grant;
logic [7:0] addr, data;
endinterface
class SBusTransactor; // SBus transactor class
virtual SBus bus; // virtual interface of type SBus
function new( virtual SBus s );
bus = s; // initialize the virtual interface
endfunction
task request(); // request the bus
bus.req <= 1'b1;
endtask
task wait_for_bus(); // wait for the bus to be granted
@(posedge bus.grant);
endtask
endclass
module devA( SBus s ) ... endmodule // devices that use SBus
module devB( SBus s ) ... endmodule
module top;
SBus s[1:4] (); // instantiate 4 interfaces
devA a1( s[1] ); // instantiate 4 devices
devB b1( s[2] );
devA a2( s[3] );
devB b2( s[4] );
initial begin
SBusTransactor t[1:4]; // create 4 bus-transactors and bind
t[1] = new( s[1] );
t[2] = new( s[2] );

<!-- Page 804 -->

IEEE Std 1800/-2023

t[3] = new( s[3] );
t[4] = new( s[4] );
// test t[1:4]
end
endmodule
In the preceding example, the transaction class SbusTransctor is a simple reusable component. It is
written without any global or hierarchical references and is unaware of the particular device with which it
will interact. Nevertheless, the class can interact with any number of devices (four in the example) that
adhere to the interface’s protocol.
An interface instance or virtual interface with no modport selected may be assigned to a virtual interface
with a modport selected.
interface PBus #(parameter WIDTH=8); // A parameterized bus interface
logic req, grant;
logic [WIDTH-1:0] addr, data;
modport phy(input addr, ref data);
endinterface
module top;
PBus #(16) p16();
PBus #(32) p32();
virtual PBus v8; // legal declaration, but no legal assignments
virtual PBus #(35) v35; // legal declaration, but no legal assignments
virtual PBus #(16) v16;
virtual PBus #(16).phy v16_phy;
virtual PBus #(32) v32;
virtual PBus #(32).phy v32_phy;
initial begin
v16 = p16; // legal – parameter values match
v32 = p32; // legal – parameter values match
v16 = p32; // illegal – parameter values don't match
v16 = v32; // illegal – parameter values don't match
v16_phy = v16; // legal assignment from no selected modport to
// selected modport
v16 = v16_phy; // illegal assignment from selected modport to
// no selected modport
v32_phy = p32; // legal assignment from no selected modport to
// selected modport
v32 = p32.phy; // illegal assignment from selected modport to
// no selected modport
end
endmodule
#### 25.9.1 Virtual interfaces and clocking blocks
Interfaces and clocking blocks can be combined to represent the interconnect between synchronous blocks.
Moreover, because clocking blocks provide a procedural mechanism to assign values to both nets and
variables, they are ideally suited to be used by virtual interfaces. For example:
interface SyncBus( input logic clk );
wire a, b, c;
clocking sb @(posedge clk);
input a;
output b;
inout c;

<!-- Page 805 -->

IEEE Std 1800/-2023

endclocking
endinterface
typedef virtual SyncBus VI; // A virtual interface type
task do_it( VI v ); // handles any SyncBus via clocking sb
if( v.sb.a == 1 )
v.sb.b <= 0;
else
v.sb.c <= ##1 1;
endtask
In the preceding example, interface SyncBus includes a clocking block, which is used by task do_it to
provide synchronous access to the interface’s signals: a, b, and c. A change to the storage type of the
interface signals (from net to variable and vice versa) requires no changes to the task. The interfaces can be
instantiated as follows:
module top;
logic clk;
SyncBus b1( clk );
SyncBus b2( clk );
initial begin
VI v[2] = '{ b1, b2 };
repeat( 20 )
do_it( v[ $urandom_range( 0, 1 ) ] );
end
endmodule
This top module shows how a virtual interface can be used to randomly select among a set of interfaces to be
manipulated, in this case by the do_it task.
#### 25.9.2 Virtual interface modports and clocking blocks
As shown in the previous example, once a virtual interface is declared, its clocking block can be referenced
using dot notation. However, this only works for interfaces with no modports. Typically, a DUT and its
testbench exhibit modport direction. This common case can be handled by including the clocking in the
corresponding modport as described in 25.5.5.
The following example shows how modports used in conjunction with virtual interfaces facilitate the
creation of abstract synchronous models.
interface A_Bus( input logic clk );
wire req, gnt;
wire [7:0] addr, data;
clocking sb @(posedge clk);
input gnt;
output req, addr;
inout data;
property p1; req ##[1:3] gnt; endproperty
endclocking

<!-- Page 806 -->

IEEE Std 1800/-2023

modport DUT ( input clk, req, addr, // Device under test modport
output gnt,
inout data );
modport STB ( clocking sb ); // synchronous testbench modport
modport TB ( input gnt, // asynchronous testbench modport
output req, addr,
inout data );
endinterface
The preceding interface A_Bus can then be instantiated as follows:
module dev1(A_Bus.DUT b); // Some device: Part of the design
...
endmodule
module dev2(A_Bus.DUT b); // Some device: Part of the design
...
endmodule
program T (A_Bus.STB b1, A_Bus.STB b2 ); // Testbench: 2 synchronous ports
...
endprogram
module top;
logic clk;
A_Bus b1( clk );
A_Bus b2( clk );
dev1 d1( b1 );
dev2 d2( b2 );
T tb( b1, b2 );
endmodule
And, within the testbench program, the virtual interface can refer directly to the clocking block.
program T (A_Bus.STB b1, A_Bus.STB b2 ); // Testbench: 2 synchronous ports
typedef virtual A_Bus.STB SYNCTB;
task request( SYNCTB s );
s.sb.req <= 1;
endtask
task wait_grant( SYNCTB s );
wait( s.sb.gnt == 1 );
endtask
task drive(SYNCTB s, logic [7:0] adr, data );
if( s.sb.gnt == 0 ) begin
request(s); // acquire bus if needed
wait_grant(s);
end

<!-- Page 807 -->

IEEE Std 1800/-2023

s.sb.addr = adr;
s.sb.data = data;
repeat(2) @s.sb;
s.sb.req = 0; //release bus
endtask
assert property (b1.sb.p1); // assert property from within program
initial begin
drive( b1, $random, $random );
drive( b2, $random, $random );
end
endprogram
This example shows how the clocking block is referenced via the virtual interface by the tasks within the
program block.
### 25.10 Access to interface objects
Access to objects declared in an interface shall be available by hierarchical name reference, regardless of
whether the interface is also accessed through a port connection or through a virtual interface, and regardless
of the existence of any declared modports in that interface. A modport may be used to restrict access to
objects declared in an interface that are referenced through a port connection or virtual interface by
explicitly listing the accessible objects in the modport. However, objects that are not permissible to be listed
in a modport shall remain accessible. For example:
interface ebus_i;
integer I; // reference to I not allowed through modport mp
typedef enum {Y,N} choice;
choice Q;
localparam True = 1;
modport mp(input Q);
endinterface
module Top;
ebus_i ebus ();
sub s1 (ebus.mp);
endmodule
module sub(interface.mp i);
typedef i.choice yes_no; // import type from interface
yes_no P;
assign P = i.Q; // refer to Q with a port reference
initial
Top.ebus.Q = i.True; // refer to Q with a hierarchical reference
initial
Top.ebus.I = 0; // referring to i.I would not be legal because
// is not in modport mp
endmodule

<!-- Page 808 -->

IEEE Std 1800-2023