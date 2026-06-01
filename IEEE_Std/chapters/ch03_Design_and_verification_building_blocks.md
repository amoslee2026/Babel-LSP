# Chapter 3: Design and verification building blocks

## 3. Design and verification building blocks
### 3.1 General
This clause describes the following:
— The purpose of modules, programs, interfaces, checkers, and primitives
— An overview of subroutines
— An overview of packages
— An overview of configurations
— An overview of design hierarchy
— Definition of compilation and elaboration
— Declaration name spaces
— Simulation time, time units, and time precision
This clause defines several important SystemVerilog terms and concepts that are used throughout this
document. The clause also provides an overview of the purpose and usage of the modeling blocks used to
represent a hardware design and its verification environment.
### 3.2 Design elements
A design element is a SystemVerilog module (see Clause23), program (see Clause24), interface (see
Clause25), checker (see Clause17), package (see Clause26), primitive (see Clause28) or configuration
(see Clause33). These constructs are introduced by the keywords module, program, interface,
checker, package, primitive, and config, respectively.
Design elements are the primary building blocks used to model and build up a design and verification
environment. These building blocks are the containers for the declarations and procedural code that are
discussed in subsequent clauses of this document.
This clause describes the purpose of these building blocks. Full details on the syntax and semantics of these
blocks are defined in later clauses of this standard.
### 3.3 Modules
The basic building block in SystemVerilog is the module, enclosed between the keywords module and
endmodule. Modules are primarily used to represent design blocks, but can also serve as containers for
verification code and interconnections between verification blocks and design blocks. Some of the
constructs that modules can contain include the following:
— Ports, with port declarations
— Data declarations, such as nets, variables, structures, and unions
— Constant declarations
— User-defined type definitions
— Class definitions
— Imports of declarations from packages
— Subroutine definitions
— Instantiations of other modules, programs, interfaces, checkers, and primitives
— Instantiations of class objects
— Continuous assignments

<!-- Page 52 -->

IEEE Std — Procedural blocks
— Generate blocks
— Specify blocks
Each of the constructs in the preceding list is discussed in detail in subsequent clauses of this standard.
NOTE—The preceding list is not all inclusive. Modules can contain additional constructs, which are also discussed in
subsequent clauses of this standard.
Following is a simple example of a module that represents a 2-to-1 multiplexer:
module mux2to1 (input wire a, b, sel, // combined port and type declaration
output logic y);
always_comb begin // procedural block
if (sel) y = a; // procedural statement
else y = b;
end
endmodule: mux2to1
Modules are presented in more detail in Clause23. See also 3.11 on creating design hierarchy with modules.
### 3.4 Programs
The program building block is enclosed between the keywords program...endprogram. This construct is
provided for modeling the testbench environment. The module construct works well for the description of
hardware. However, for the testbench, the emphasis is not on the hardware-level details such as wires,
structural hierarchy, and interconnects, but in modeling the complete environment in which a design is
verified.
The program block serves the following three basic purposes:
— It provides an entry point to the execution of testbenches.
— It creates a scope that encapsulates program-wide data, tasks, and functions.
— It provides a syntactic context that specifies scheduling in the reactive region set.
The program construct serves as a clear separator between design and testbench, and, more importantly, it
specifies specialized simulation execution semantics. Together with clocking blocks (see Clause14), the
program construct provides for race-free interaction between the design and the testbench and enables
cycle-and transaction-level abstractions.
A program block can contain data declarations, class definitions, subroutine definitions, object instances,
and one or more initial or final procedures. It cannot contain always procedures, primitive instances, module
instances, interface instances, or other program instances.
The abstraction and modeling constructs of SystemVerilog simplify the creation and maintenance of
testbenches. The ability to instantiate and individually connect each program instance enables their use as
generalized models.
A sample program declaration is as follows:
program test (input clk, input [16:1] addr, inout [7:0] data);
initial begin
...
end
endprogram

<!-- Page 53 -->

IEEE Std The program construct is discussed more fully in Clause24.
### 3.5 Interfaces
The interface construct, enclosed between the keywords interface...endinterface, encapsulates the
communication between design blocks, and between design and verification blocks, allowing a smooth
migration from abstract system-level design through successive refinement down to lower level register-
transfer and structural views of the design. By encapsulating the communication between blocks, the
interface construct also facilitates design reuse.
At its lowest level, an interface is a named bundle of nets or variables. The interface is instantiated in a
design and can be connected to interface ports of other instantiated modules, interfaces and programs. An
interface can be accessed through a port as a single item, and the component nets or variables referenced
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
To provide direction information for module ports and to control the use of subroutines within particular
modules, the modport construct is provided. As the name indicates, the directions are those seen from the
module.
In addition to subroutine methods, an interface can also contain processes (i.e., initial or always procedures)
and continuous assignments, which are useful for system-level modeling and testbench applications. This
allows the interface to include, for example, its own protocol checker, which automatically verifies that all
modules connected via the interface conform to the specified protocol. Other applications, such as functional
coverage recording and reporting, protocol checking, and assertions can also be built into the interface.
A simple example of an interface definition and usage is as follows:
interface simple_bus(input logic clk); // Define the interface
logic req, gnt;
logic [7:0] addr, data;
logic [1:0] mode;
logic start, rdy;
endinterface: simple_bus
module memMod(simple_bus a); // simple_bus interface port
logic avail;
// When memMod is instantiated in module top, a.req is the req
// signal in the sb_intf instance of the 'simple_bus' interface
always @(posedge a.clk) a.gnt <= a.req & avail;
endmodule

<!-- Page 54 -->

IEEE Std module cpuMod(simple_bus b); // simple_bus interface port
...
endmodule
module top;
logic clk = 0;
simple_bus sb_intf(.clk(clk)); // Instantiate the interface
memMod mem(.a(sb_intf)); // Connect interface to module instance
cpuMod cpu(.b(sb_intf)); // Connect interface to module instance
endmodule
See Clause25 for a full description of interfaces.
### 3.6 Checkers
The checker construct, enclosed by the keywords checker...endchecker, represents a verification block
encapsulating assertions along with the modeling code. The intended use of checkers is to serve as
verification library units or as building blocks for creating abstract auxiliary models used in formal
verification. The checker construct is discussed in detail in Clause17.
### 3.7 Primitives
The primitive building block is used to represent low-level logic gates and switches. SystemVerilog includes
a number of built-in primitive types. Designers can supplement the built-in primitives with user-defined
primitives (UDPs). A UDP is enclosed between the keywords primitive...endprimitive. The built-in
and UDP constructs allow modeling timing-accurate digital circuits, commonly referred to as gate-level
models. Gate-level modeling is discussed more fully in Clause28 through Clause31.
### 3.8 Subroutines
Subroutines provide a mechanism to encapsulate executable code that can be invoked from one or more
places. There are two forms of subroutines, tasks (13.3) and functions (13.4).
A task is called as a statement. A task can have any number of input, output, inout, and ref arguments,
but does not return a value. Tasks can block simulation time during execution. That is, the task exit can
occur at a later simulation time than when the task was called.
A function can return a value or can be defined as a void function, which does not return a value. A nonvoid
function call is used as an operand within an expression. A void function is called as a statement. A function
can have input, output, inout, and ref arguments. Functions execute without blocking simulation time,
but can fork off processes that do block time.
### 3.9 Packages
Modules, interfaces, programs, and checkers provide a local name space for declarations. Identifiers
declared within a module, interface, program, or checker are local to that scope, and do not affect or conflict
with declarations in other building blocks.

<!-- Page 55 -->

IEEE Std Packages provide a declaration space, which can be shared by other building blocks. Package declarations
can be imported into other building blocks, including other packages.
A package is defined between the keywords package...endpackage. For example:
package ComplexPkg;
typedef struct {
shortreal i, r;
} Complex;
function Complex add(Complex a, b);
add.r = a.r + b.r;
add.i = a.i + b.i;
endfunction
function Complex mul(Complex a, b);
mul.r = (a.r * b.r) - (a.i * b.i);
mul.i = (a.r * b.i) + (a.i * b.r);
endfunction
endpackage : ComplexPkg
The full syntax and semantics of packages are described in Clause26.
### 3.10 Configurations
SystemVerilog provides the ability to specify design configurations, which specify the binding information
of module instances to specific SystemVerilog source code. Configurations utilize libraries. A library is a
collection of modules, interfaces, programs, checkers, primitives, packages, and other configurations.
Separate library map files specify the source code location for the cells contained within the libraries. The
names of the library map files are typically specified as invocation options to simulators or other software
tools reading in SystemVerilog source code.
See Clause33 for details of configurations.
### 3.11 Overview of hierarchy
The basic building blocks of modules, programs, interfaces, checkers, and primitives are used to build up a
design hierarchy. Hierarchy is created by one building block instantiating another building block. When a
module contains an instance of another module, interface, program, or checker, a new level of hierarchy is
created. Communication through levels of hierarchy is primarily through connections to the ports of the
instantiated module, interface, program, or checker.
Following is a simple example of two module declarations that utilize some simple declarations. Module
top contains an instance of module mux2to1, creating a design with two levels of hierarchy.
module top; // module with no ports
logic in1, in2, select; // variable declarations
wire out1; // net declaration
mux2to1 m1 (.a(in1), .b(in2), .sel(select), .y(out1)); // module instance
endmodule: top
module mux2to1 (input wire a, b, sel, // combined port and type declaration
output logic y);

<!-- Page 56 -->

IEEE Std // netlist using built-in primitive instances
not g1 (sel_n, sel);
and g2 (a_s, a, sel_n);
and g3 (b_s, b, sel);
or g4 (y, a_s, b_s);
endmodule: mux2to1
Modules can instantiate other modules, programs, interfaces, checkers, and primitives, creating a hierarchy
tree. Interfaces can also instantiate other building blocks and create a hierarchy tree. Programs and checkers
can instantiate other checkers. Primitives cannot instantiate other building blocks; they are leaves in a
hierarchy tree.
Normally, a module or program that is elaborated but not explicitly instantiated is implicitly instantiated
once at the top of the hierarchy tree and becomes a top-level hierarchy block (see 23.3 and 24.3).
SystemVerilog permits multiple top-level blocks.
Identifiers within any level of hierarchy can be referenced from any other level of hierarchy using
hierarchical path names (see 23.6).
Instantiation syntax and design hierarchy are presented in more detail in Clause23.
### 3.12 Compilation and elaboration
Compilation is the process of reading in SystemVerilog source code, decrypting encrypted code, and
analyzing the source code for syntax and semantic errors. Implementations may execute compilation in one
or more passes. Implementations may save compiled results in a proprietary intermediate format, or may
pass the compiled results directly to an elaboration phase. Not all syntax and semantics can be checked
during the compilation process. Some checking can only be done during or at the completion of elaboration.
SystemVerilog supports both single file and multiple file compilation through the use of compilation units
(see 3.12.1).
Elaboration is the process of binding together the components that make up a design. These components can
include module instances, program instances, interface instances, checker instances, primitive instances, and
the top level of the design hierarchy. Elaboration occurs after parsing the source code and before simulation;
and it involves expanding instantiations, computing parameter values, resolving hierarchical names,
establishing net connectivity and in general preparing the design for simulation. See 23.10.4 for additional
details on the elaboration process.
Although this standard defines the results of compilation and elaboration, the compilation and elaboration
steps are not required to be distinct phases in an implementation. Throughout this standard the terms
compilation, compile, and compiler normally refer to the combined compilation and elaboration process. So
for example, when the standard refers to a “compile-time error,” an implementation is permitted to report the
error at any time prior to the start of simulation.
This standard does not normally specify requirements regarding the order of compilation for design
elements. The two exceptions are the rules regarding “compilation units” (see 3.12.1) where actual file
boundaries during compilation are significant, and the rules regarding references to package items (see 26.3)
where the compilation of a package is required to precede references to it.
#### 3.12.1 Compilation units
SystemVerilog supports separate compilation using compiled units. The following terms and definitions are
provided:

<!-- Page 57 -->

IEEE Std — compilation unit: A collection of one or more SystemVerilog source files compiled together.
— compilation-unit scope: A scope that is local to the compilation unit. It contains all declarations
that lie outside any other scope.
— $unit: The name used to explicitly access the identifiers in the compilation-unit scope.
The exact mechanism for defining which files constitute a compilation unit is tool-specific. However,
compliant tools shall provide use models that allow both of the following cases:
a) All files on a given compilation command line make a single compilation unit (in which case the
declarations within those files are accessible following normal visibility rules throughout the entire
set of files).
b) Each file is a separate compilation unit (in which case the declarations in each compilation-unit
scope are accessible only within its corresponding file).
The contents of files included using one or more `include directives become part of the compilation unit
of the file within which they are included.
If there is a declaration that is incomplete at the end of a file, then the compilation unit including that file
will extend through each successive file until there are no incomplete declarations at the end of the group of
files.
There are other possible mappings of files to compilation units, and the mechanisms for defining them are
tool specific and may not be portable.
Although the compilation-unit scope is not a package, it can contain any item that can be defined within a
package (see 26.2) and bind constructs as well (see 23.11). These items are in the compilation-unit scope
name space (see 3.13).
The following items are visible in all compilation units: modules, primitives, programs, interfaces, and
packages. Items defined in the compilation-unit scope cannot be accessed by name from outside the
compilation unit. The items in a compilation-unit scope can be accessed using the PLI, which shall provide
an iterator to traverse all the compilation units.
Items in a compilation-unit scope can have hierarchical references to identifiers. For upwards name
referencing (see 23.8), the compilation-unit scope is treated like a top-level design element. This means that
if these are not references to identifiers created within the compilation-unit scope or made visible by import
of a package into the compilation-unit scope, they are treated as full path names starting at the top of the
design ($root, described in 23.3.1).
Within a separately compiled unit, compiler directives once seen by a tool apply to all subsequent source
text. However, compiler directives from one separately compiled unit shall not affect other compilation
units. This may result in a difference of behavior between compiling the units separately or as a single
compilation unit containing the entire source.
When an identifier is referenced within a scope
— First, the nested scope is searched (see 23.9) (including nested module declarations), including any
identifiers made available through package import declarations.
— Next, the portion of the compilation-unit scope defined prior to the reference is searched (including
any identifiers made available through package import declarations).
— Finally, if the identifier follows hierarchical name resolution rules, the instance hierarchy is
searched (see 23.8 and 23.9).

<!-- Page 58 -->

IEEE Std $unit is the name of the scope that encompasses a compilation unit. Its purpose is to allow the
unambiguous reference to declarations at the outermost level of a compilation unit (i.e., those in the
compilation-unit scope). This is done via the same scope resolution operator used to access package items
(see 26.3).
For example:
bit b;
task t;
int b;
b = 5 + $unit::b; // $unit::b is the one outside
endtask
Other than for task and function names (see 23.8.1), references shall only be made to names already defined
in the compilation unit. The use of an explicit $unit:: prefix only provides for name disambiguation and
does not add the ability to refer to later compilation-unit items.
For example:
task t;
int x;
x = 5 + b; // illegal - "b" is defined later
x = 5 + $unit::b; // illegal - $unit adds no special forward referencing
endtask
bit b;
The compilation-unit scope allows users to easily share declarations (e.g., types) across the unit of
compilation, but without having to declare a package from which the declarations are subsequently
imported. Because it has no name, the compilation-unit scope cannot be used with an import declaration,
and the identifiers declared within the scope are not accessible via hierarchical references. Within a
particular compilation unit, however, the special name $unit can be used to explicitly access the
declarations of its compilation-unit scope.
### 3.13 Name spaces
SystemVerilog has eight name spaces for identifiers: two are global (definitions name space and package
name space), two are global to the compilation unit (compilation unit name space and text macro name
space), and four are local. The eight name spaces are described as follows:
a) The definitions name space unifies all the non-nested module, primitive, program, and
interface identifiers defined outside all other declarations. Once a name is used to define a
module, primitive, program, or interface within one compilation unit, the name shall not be used
again (in any compilation unit) to declare another non-nested module, primitive, program, or
interface outside all other declarations.
b) The package name space unifies all the package identifiers defined among all compilation units.
Once a name is used to define a package within one compilation unit, the name shall not be used
again to declare another package within any compilation unit.
c) The compilation-unit scope name space exists outside the module, interface, package,
checker, program, and primitive constructs. It unifies the definitions of the functions, tasks,
checkers, parameters, named events, net declarations, variable declarations, and user-defined types
within the compilation-unit scope.
d) The text macro name space is global within the compilation unit. Because text macro names are
introduced and used with a leading ‘ character, they remain unambiguous with any other name
space. The text macro names are defined in the linear order of appearance in the set of input files that

<!-- Page 59 -->

IEEE Std make up the compilation unit. Subsequent definitions of the same name override the previous
definitions for the balance of the input files.
e) The module name space is introduced by the module, interface, package, program, checker,
and primitive constructs. It unifies the definition of modules, interfaces, programs, checkers,
functions, tasks, named blocks, instance names, parameters, named events, net declarations, variable
declarations, and user-defined types within the enclosing construct.
f) The block name space is introduced by named or unnamed blocks, the specify, function, and
task constructs. It unifies the definitions of the named blocks, functions, tasks, parameters, named
events, variable type of declaration, and user-defined types within the enclosing construct.
g) The port name space is introduced by the module, interface, primitive, and program
constructs. It provides a means of structurally defining connections between two objects that are in
two different name spaces. The connection can be unidirectional (either input or output) or
bidirectional (inout or ref). The port name space overlaps the module and the block name spaces.
Essentially, the port name space specifies the type of connection between names in different name
spaces. The port type of declarations includes input, output, inout, and ref. A port name
introduced in the port name space can be reintroduced in the module name space by declaring a
variable or a net with the same name as the port name.
h) The attribute name space is enclosed by the (* and *) constructs attached to a language element
(see 5.12). An attribute name can be defined and used only in the attribute name space. Any other
type of name cannot be defined in this name space.
Within a name space, it shall be illegal to redeclare a name already declared by a prior declaration.
### 3.14 Simulation time units and precision
An important aspect of simulation is time. The term simulation time is used to refer to the time value
maintained by the simulator to model the actual time it would take for the system description being
simulated. The term time is used interchangeably with simulation time.
Time values are used in design elements to represent propagation delays and the amount of simulation time
between when procedural statements execute. Time values have two components, a time unit and a time
precision.
— The time unit represents the unit of measurement for times and delays, and can be specified in units
ranging from 100 second units down to 1 femtosecond units.
— The time precision specifies the degree of accuracy for delays.
Both the time units and time precision are represented using one of the character strings: s, ms, us, ns, ps,
and fs with an order of magnitude of 1, 10, or 100. The definition of these character strings is given in
Table3-1.
Table3-1—Time unit strings
Character string Unit of measurement
s seconds
ms milliseconds
us microseconds
ns nanoseconds
ps picoseconds
fs femtoseconds

<!-- Page 60 -->

IEEE Std NOTE—While s, ms, ns, ps, and fs are the usual SI unit symbols for second, millisecond, nanosecond, picosecond, and
femtosecond, due to lack of the Greek letter μ (mu) in coding character sets, “us” represents the SI unit symbol for
microsecond, properly μs.
The time precision of a design element shall be at least as precise as the time unit; it cannot be a longer unit
of time than the time unit.
#### 3.14.1 Time value rounding
Within a design element, such as a module, program or interface, the time precision specifies how delay
values are rounded before being used in simulation.
The time precision is relative to the time units. If the precision is the same as the time units, then delay
values are rounded off to whole numbers (integers). If the precision is one order of magnitude smaller than
the time units, then delay values are rounded off to one decimal place. For example, if the time unit specified
is 1ns and the precision is 100ps, then delay values are rounded off to one decimal place (100ps is
equivalent to 0.1ns). Thus, a delay of 2.75ns would be rounded off to 2.8ns.
The time values in a design element are accurate to within the unit of time precision specified for that design
element, even if there is a smaller time precision specified elsewhere in the design.
#### 3.14.2 Specifying time units and precision
The time unit and time precision can be specified in the following two ways:
— Using the compiler directive `timescale
— Using the keywords timeunit and timeprecision
3.14.2.1 The `timescale compiler directive
The `timescale compiler directive specifies the default time unit and precision for all design elements that
follow this directive and that do not have timeunit and timeprecision constructs specified within the
design element. The `timescale directive remains in effect from when it is encountered in the source code
until another `timescale compiler directive is read. The `timescale directive only affects the current
compilation unit; it does not span multiple compilation units (see 3.12.1).
The general syntax for the `timescale directive is (see 22.7 for more details):
`timescale time_unit / time_precision
The following example specifies a time unit of 1 ns with a precision of 10ps (2 decimal places of accuracy).
The compiler directive affects both module A and module B. A second `timescale directive replaces the
first directive, specifying a time unit of 1ps and precision of 1ps (zero decimal places of accuracy) for
module C.
`timescale 1ns / 10ps
module A (...);
...
endmodule
module B (...);
...
endmodule
`timescale 1ps/1ps
module C (...);
...
endmodule

<!-- Page 61 -->

IEEE Std The `timescale directive can result in file order dependency problems. If the previous three modules were
compiled in the order of A, B, C (as shown) then module B would simulate with time units in nanoseconds. If
the same three files were compiled in the order of C, B, A then module B would simulate with time units in
picoseconds. This could cause very different simulation results, depending on the time values specified in
module B.
3.14.2.2 The timeunit and timeprecision keywords
The time unit and precision can be declared by the timeunit and timeprecision keywords, respectively,
and set to a time literal (see 5.8). The time precision may also be declared using an optional second argument
to the timeunit keyword using the slash separator. For example:
module D (...);
timeunit 100ps;
timeprecision 10fs;
...
endmodule
module E (...);
timeunit 100ps / 10fs; // timeunit with optional second argument
...
endmodule
Defining the timeunit and timeprecision constructs within the design element removes the file order
dependency problems with compiler directives.
There shall be at most one time unit and one time precision for any module, program, package, or interface
definition or in any compilation-unit scope. This shall define a time scope. If specified, the timeunit and
timeprecision declarations shall precede any other items in the current time scope. The timeunit and
timeprecision declarations can be repeated as later items, but shall match the previous declaration within
the current time scope.
3.14.2.3 Precedence of timeunit, timeprecision, and `timescale
If a timeunit is not specified within a module, program, package, or interface definition, then the time unit
shall be determined using the following rules of precedence:
a) If the module or interface definition is nested, then the time unit shall be inherited from the
enclosing module or interface (programs and packages cannot be nested).
b) Else, if a `timescale directive has been previously specified (within the compilation unit), then
the time unit shall be set to the units of the last `timescale directive.
c) Else, if the compilation-unit scope specifies a time unit (outside all other declarations), then the time
unit shall be set to the time units of the compilation unit.
d) Else, the default time unit shall be used.
The time unit of the compilation-unit scope can only be set by a timeunit declaration, not a `timescale
directive. If it is not specified, then the default time unit shall be used.
If a timeprecision is not specified in the current time scope, then the time precision shall be determined
following the same precedence as with time units.
The default time unit and precision are implementation-specific.
It shall be an error if some design elements have a time unit and precision specified and others do not.

<!-- Page 62 -->

IEEE Std #### 3.14.3 Simulation time unit
The global time precision, also called the simulation time unit, is the minimum of all the timeprecision
statements, all the time precision arguments to timeunit declarations, and the smallest time precision
argument of all the `timescale compiler directives in the design.
The step time unit is equal to the global time precision. Unlike other time units, which represent physical
units, a step cannot be used to set or modify either the precision or the time unit.

<!-- Page 63 -->

IEEE Std 1800-2023