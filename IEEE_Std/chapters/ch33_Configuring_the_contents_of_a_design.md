# Chapter 33: Configuring the contents of a design

## 33. Configuring the contents of a design
### 33.1 General
This clause describes the following:
— Configuration libraries
— Configuration syntax
— Using libraries and configurations
### 33.2 Overview
To facilitate both the sharing of SystemVerilog designs between designers and/or design groups and the
repeatability of the exact contents of a given simulation (or other tool) session, the concept of configurations
is used in the SystemVerilog language. A configuration is simply an explicit set of rules to specify the exact
source description to be used to represent each instance in a design. The operation of selecting a source
representation for an instance is referred to as binding the instance.
The following example shows a simple configuration problem:
file top.v file adder.v file adder.vg
module top(); module adder(...); module adder(...);
adder a1(...); // rtl adder // gate-level adder
adder a2(...); // description // description
endmodule ... ...
endmodule endmodule
Consider using the rtl adder description in adder.v for instance a1 in module top and the gate-level
adder description in adder.vg for instance a2. In order to specify this particular set of instance bindings
and to avoid having to change the source description to specify a new set, a configuration can be used.
config cfg1; // specify rtl adder for top.a1, gate-level adder for top.a2
design rtlLib.top;
default liblist rtlLib;
instance top.a2 liblist gateLib;
endconfig
The elements of a config are explained in subsequent subclauses, but this simple example illustrates some
important points about configs. As evidenced by the config–endconfig syntax, the config is a design
element, similar to a module, which exists in the SystemVerilog name space. The config contains a set of
rules that are applied when searching for a source description to bind to a particular instance of the design.
A design description starts with a top-level module (or modules) (see 23.3.1). From this module’s source
description, the instantiated modules (or children) are found, then the source descriptions for the module
definitions of these subinstances shall be located, and so on until every instance in the design is mapped to a
source description.
#### 33.2.1 Library notation
A library is a named collection of cells. A cell is a design element (see 3.2), such as a module, primitive,
interface, program, package, or configuration. The cell name shall be the same as the name of the design
element being processed.

<!-- Page 936 -->

IEEE Std In order to map a design element instance to a source description, the concept of a symbolic library, which is
simply a logical collection of design elements, can be used. Syntax33-1 specifies a cell from a given library.
cell_clause ::= cell [ library_identifier . ] cell_identifier // from A.1.5
Syntax33-1—Syntax for cell (excerpt from AnnexA)
This notation gives a symbolic method of referring to source descriptions; the method of mapping source
descriptions into libraries is shown in greater detail in 33.3.1. The optional :config extension shall be used
explicitly to refer to a config in the case where a config has the same name as a module/primitive.
For the purposes of this example, suppose the files top.v and adder.v (i.e., the RTL descriptions) have
been mapped into the library rtlLib and the file adder.vg (i.e., the gate-level description of the adder)
has been mapped into the library gateLib. The actual mechanism for mapping source descriptions to
libraries is detailed in 33.3.
#### 33.2.2 Basic configuration elements
The design statement in config cfg1 of the first example of 33.2 specifies the top-level module in the
design and what source description is to be used. In this example, the rtlLib.top notation indicates the
top-level module description shall be taken from rtlLib. Because top.v and adder.v were mapped to
this library, the actual description for the module is known to come from top.v.
The default statement coupled with the liblist clause specifies, by default, all subinstances of top (i.e.,
top.a1 and top.a2) shall be taken from rtlLib, which means the descriptions in top.v and adder.v,
which were mapped to this library, shall be used. For a basic design, which can be completely rtl, this can
be sufficient to specify completely the binding for the entire design. However, here the top.a2 instance of
adder to the gate-level description shall be bound.
The instance statement specifies, for the particular instance top.a2, the source description shall be taken
from gateLib. The instance statement overrides the default rule for this particular instance. Because
adder.vg was mapped to gateLib, this statement dictates the gate-level description in adder.vg be used
for instance top.a2.
### 33.3 Libraries
As mentioned in the previous subclause, a library is a logical collection of cells that are mapped to particular
source description files. The symbolic lib.cell[:config] notation supports the separate compilation of
source files by providing a filesystem-independent name to refer to source descriptions when instances in a
design are bound. It also allows multiple tools, which can have different invocation use models, to share the
same configuration.
#### 33.3.1 Specifying libraries—the library map file
When parsing a source description file (or files), the parser shall first read the library mapping information
from a predefined file prior to reading any source files. The name of this file and the mechanism for reading
it shall be tool-specific, but all compliant tools shall provide a mechanism to specify one or more library
map files to be used for a particular invocation of the tool. If multiple map files are specified, then they shall
be read in the order in which they are specified.

<!-- Page 937 -->

IEEE Std For the purposes of this discussion, assume the existence of a file named lib.map in the current working
directory, which is automatically read by the parser prior to parsing any source files specified on the
command line. The syntax for declaring a library in the library map file is shown in Syntax33-2.
library_text ::= { library_description } // from A.1.1
library_description ::=
library_declaration
| include_statement
| config_declaration
| ;
library_declaration ::=
library library_identifier file_path_spec { , file_path_spec }
[ -incdir file_path_spec { , file_path_spec } ] ;
include_statement ::= include file_path_spec ;
Syntax33-2—Syntax for declaring library in library map file (excerpt from AnnexA)
Library map file details:
1) file_path_spec uses filesystem-specific notation to specify an absolute or relative path to a particular
file or set of files. The following shortcuts/wildcards can be used:
? single character wildcard (matches any single character)
* multiple character wildcard (matches any number of characters in a directory/file name)
... hierarchical wildcard (matches any number of hierarchical directories)
.. specifies the parent directory
. specifies the directory containing the lib.map
Paths that end in / shall include all files in the specified directory. Identical to /*.
Paths that do not begin with / are relative to the directory in which the current lib.map file
is located.
2) The paths ./*.v and *.v are identical, and both specify all files with a .v suffix in the current
directory.
Any file encountered by the compiler that does not match any library’s file_path_spec shall by default be
compiled into a library named work.
To perform the library mapping discussed in the example in 33.2, use the following library definitions in the
lib.map file:
library rtlLib *.v; // matches all files in the current directory with
// a .v suffix
library gateLib ./*.vg; // matches all files in the current directory with
// a .vg suffix
33.3.1.1 File path resolution
If a file name potentially matches multiple file path specifications, the path specifications shall be resolved
in the following order:
a) File path specifications that end with an explicit file name
b) File path specifications that end with a wildcarded file name
c) File path specifications that end with a directory

<!-- Page 938 -->

IEEE Std If a file name matches path specifications in multiple library definitions (after the above resolution rules
have been applied), it shall be an error.
Using these rules with the library definitions in the lib.map file, all source files encountered by the parser/
compiler can be mapped to a unique library. Once the source descriptions have been mapped to libraries, the
cells defined in those libraries are available for binding.
NOTE—Tool implementers may find it convenient to provide a command-line argument to explicitly specify the library
into which the file being parsed is to be mapped, which shall override any library definitions in the lib.map file. If
these libraries do not exist in the lib.map file, they can only be accessed via an explicit config.
If multiple cells with the same name map to the same library, then the last cell encountered shall be written
to the library. This is to support a “separate-compile” use model (see 33.5.3), where it is assumed that
encountering a cell after it has previously been compiled is intended to be a recompiling of the cell. In the
case where multiple modules with the same name are mapped to the same library in a single invocation of
the compiler, then a warning shall be issued.
#### 33.3.2 Using multiple library map files
In addition to specifying library mapping information, a lib.map file can also include references to other
lib.map files. The include command is used to insert the entire contents of a library map file in another
file during parsing. The result is as though the contents of the included map file appear in place of the
include command.
The syntax of a lib.map file is limited to library specifications, include statements, and standard
SystemVerilog comment syntax. Syntax33-3 shows the syntax for the include command.
include_statement ::= include file_path_spec ; // from A.1.1
Syntax33-3—Syntax for include command (excerpt from AnnexA)
If the file path specification, whether in an include or library statement, describes a relative path, it shall be
relative to the location of the file that contains the file path. Library providers shall include a local library
map file in addition to the source contents of the library. Individual users can then simply include the
provider’s library map file in their own map file to gain access to the contents of the provided library.
#### 33.3.3 Mapping source files to libraries
For each cell definition encountered during parsing/compiling, the name of the source file being parsed is
compared to the file path specifications of the library declarations in all of the library map files being used.
The cell is mapped into the library whose file path specification matches the source file name.
### 33.4 Configurations
As mentioned in the introduction of this clause, a configuration is simply a set of rules to apply when
searching for library cells to which to bind instances.
A configuration may change the binding of a module, primitive, interface, or program instance, but shall not
change the binding of a package.
The syntax for configurations is shown in 33.4.1.

<!-- Page 939 -->

IEEE Std #### 33.4.1 Basic configuration syntax
The configuration syntax is shown in Syntax33-4.
config_declaration ::= // from A.1.5
config config_identifier ;
{ local_parameter_declaration ; }
design_statement
{ config_rule_statement }
endconfig [ : config_identifier ]
design_statement ::= design { [ library_identifier . ] cell_identifier } ;
config_rule_statement ::=
default_clause liblist_clause ;
| inst_clause liblist_clause ;
| inst_clause use_clause ;
| cell_clause liblist_clause ;
| cell_clause use_clause ;
default_clause ::= default
inst_clause ::= instance inst_name
inst_name ::= topmodule_identifier { . instance_identifier }
cell_clause ::= cell [ library_identifier . ] cell_identifier
liblist_clause ::= liblist { library_identifier }
use_clause ::=
use [ library_identifier . ] cell_identifier [ : config ]
| use named_parameter_assignment { , named_parameter_assignment } [ : config ]
| use [ library_identifier . ] cell_identifier named_parameter_assignment
{ , named_parameter_assignment } [ : config ]
Syntax33-4—Syntax for configurations (excerpt from AnnexA)
33.4.1.1 Design statement
The design statement names the library and cell of the top-level module or modules in the design hierarchy
configured by the config. There shall be one and only one design statement, but multiple top-level modules
can be listed in the design statement. The cell or cells identified cannot be configurations themselves. It is
possible the design identified can have the same name as configs, however.
The design statement shall appear before any config rule statements in the config.
If the library identifier is omitted, then the library that contains the config shall be used to search for the cell.
33.4.1.2 The default clause
The default clause selects all instances that do not match a more specific selection clause. The use
expansion clause (see 33.4.1.6) cannot be used with a default selection clause. For other expansion
clauses, there cannot be more than one default clause that specifies the expansion clause.
For simple design configurations, it might be sufficient to specify a default liblist (see 33.4.1.5).

<!-- Page 940 -->

IEEE Std 33.4.1.3 The instance clause
The instance clause is used to specify the specific instance to which the expansion clause shall apply. The
instance name associated with the instance clause is a SystemVerilog hierarchical name, starting at the
top-level module of the config (i.e., the name of the cell in the design statement).
33.4.1.4 The cell clause
The cell selection clause names the cell to which it applies. If the optional library name is specified, then
the selection rule applies to any instance that is bound or is under consideration for being bound to the
selected library and cell. It is an error if a library name is included in a cell selection clause and the
corresponding expansion clause is a library list expansion clause.
33.4.1.5 The liblist clause
The liblist clause defines an ordered set of libraries to be searched to find the current instance. liblists are
inherited hierarchically downward as instances are bound. When searching for a cell to bind to the current
unbound instance, and in the absence of an applicable binding expansion clause, the specified library list is
searched in the specified order.
The current library list is selected by the selection clauses. If no library list clause is selected or if the
selected library list is empty, then the library list contains the single name that is the library in which the cell
containing the unbound instance is found (i.e., the parent cell’s library).
33.4.1.6 The use clause
The use clause specifies a specific binding for the selected cell. A use clause can only be used in
conjunction with an instance or cell selection clause. It specifies the exact library and cell to which a
selected cell or instance is bound.
The use clause has no effect on the current value of the library list. It can be common in practice to specify
multiple config rule statements, one of which specifies a binding and the other of which specifies a library
list.
If the lib.cell to which the use clause refers is a config that has the same name as a module/primitive in the
same library, then the optional :config suffix can be added to the lib.cell to specify the config explicitly.
If the library name is omitted, the library shall be inherited from the parent cell.
NOTE—The binding statement can create situations where the unbound instance’s module name and the cell name to
which it is bound are different.
#### 33.4.2 Hierarchical configurations
For situations where it is desirable to specify a special set of configuration rules for a subsection of a design,
it is possible to bind a particular instance directly to a configuration using the binding clause:
instance top.a1.c use lib1.c:config;
// bind to the config c in library lib1
that specifies the instance top.a1.c is to be replaced with the design hierarchy specified by the
configuration lib1.c:config. The design statement in lib1.c:config shall specify the actual binding
for the instance top.a1.c, and the rules specified in the config shall determine the configuration of all other
subinstances under top.a1.c.

<!-- Page 941 -->

IEEE Std It shall be an error for an instance clause to specify a hierarchical path to an instance that occurs within a
hierarchy specified by another config.
config bot;
design lib1.bot;
default liblist lib1 lib2;
instance bot.a1 liblist lib3;
endconfig
config top;
design lib1.top;
default liblist lib2 lib1;
instance top.bot use lib1.bot:config;
instance top.bot.a1 liblist lib4;
// ERROR - cannot set liblist for top.bot.a1 from this config
endconfig
#### 33.4.3 Setting parameters in configurations
Configurations can be used to override parameters declared within a design, or to apply or override
parameter values for specific instantiations of a design. When a common value is desired for overriding a
number of parameters in a configuration, a localparam can be declared in the configuration.
The following is a list of detailed restrictions regarding setting parameters from a configuration:
— A localparam declared in a configuration shall be assigned a value and shall only be set to a literal
value.
— Index expressions in a hierarchical name shall only refer to literals or localparams of the
configuration.
— Parameters identifiers shall be resolved starting in the parent scope of the instance. If a hierarchical
identifier is used, it shall be the only term in the expression, e.g., a.b.c + 7 is invalid.
— Parameter identifiers that bind into the parent of the configured instance shall be identifiers that are
legal in a bind statement.
— Hierarchical references cannot include scopes of generate or array of instances.
— A parameter override in a configuration shall not refer to a constant function other than a built-in
constant system function.
— A parameter override from a configuration shall take precedence over a defparam statement when
both are referencing the same parameter at the same level of hierarchy. In all other conditions,
defparam statements work as defined.
— Configurations may not use positional parameter notation to override parameters.
The following two code sample modules are used by examples in this subclause. Note that module top
includes an instantiation of adder and does not include any direct parameter modifications.
module adder #(parameter ID = "id",
W = 8,
D = 512)
(...);
...
$display("ID = %s, W = %d, D = %d", ID, W, D);
...
endmodule: adder
Example 1:

<!-- Page 942 -->

IEEE Std module top (...);
parameter WIDTH = 16;
adder a1 (...);
endmodule
Example 2:
The preceding code, without any parameter overrides on adder a1, will print the following:
ID = id, W=8, D = 512
A configuration can be used to override a parameter in a module and apply that overridden parameter to
instantiations within that module. The configuration in Example 3 overrides the default value of parameter
WIDTH in module top shown in Example 2. Example 3 then uses parameter WIDTH to override the parameter
for the instantiation of adder. (The code for adder is found in Example 1.)
config cfgl;
design rtlLib.top;
instance top use #(.WIDTH(32));
instance top.a1 use #(.W(top.WIDTH));
endconfig
Example 3:
Using configuration cfg1 above, the $display in adder will print the following:
ID = id, W=32, D = 512
A localparam can be used in a configuration to represent a value sent to multiple instances.
module top4 ();
parameter S = 16;
adder a1 #(.ID("a1"))(...);
adder a2 #(.ID("a2"))(...);
adder a3 #(.ID("a3"))(...);
adder a4 #(.ID("a4"))(...);
endmodule
config cfg2;
localparam S = 24;
design rtlLib.top4;
instance top4.a1 use #(.W(top4.S));
instance top4.a2 use #(.W(S));
endconfig
Example 4:
With cfg2 configuring module top4.a1, the $display in adder will print the following:
ID = a1, W=16, D = 512
ID = a2, W=24, D = 512
ID = a3, W=8, D = 512
ID = a4, W=8, D = 512
In Example 4, the override of adder port ID in top4 is maintained. This example also used the same
parameter identifier and localparam identifier in both module top4 and config cfg2. There is no conflict in
the configuration since the localparam in config cfg2 is directly accessed in the configuration. When config

<!-- Page 943 -->

IEEE Std cfg2 needs to use parameter S in module top, parameter S has to be referenced via the design name in the
configuration.
A parameter override with empty parentheses will set the parameter back to its default as defined in its
module.
module top5 (...);
parameter WIDTH = 64, DEPTH = 1024, ID = "A1";
adder a1 #(.ID(ID), .W(WIDTH), .D(DEPTH))(...);
endmodule
Example 5:
Using the preceding code, the $display in adder will print the following:
ID = A1, W=64, D = 1024
config cfg3;
design rtlLib.top5;
instance top5.a1 use #(.W()); // set only parameter W back to its default
endconfig
Example 6:
When the preceding configuration cfg3 configures module top5.a1, the $display in adder will print the
following:
ID = A1, W=8, D = 1024
Only parameter W is configured to use its default value.
All the parameters can be reset to their default values, meaning their initial values prior to any parameter
overrides. This is shown in the following configuration, which uses the module top from Example 2.
config cfg4;
design rtlLib.top;
instance top.a1 use #(); // set all parameters in instance a1
// back to their defaults
endconfig
Example 7:
This configuration of top.a1 will print the following:
ID = id, W=8, D = 512
When both a configuration and a defparam statement are modifying the same parameter at the same
hierarchal level, the configuration shall take precedence. The following code adds a level of hierarchy,
which includes defparam statements:
module test;
...
top8 t(...);
defparam t.WIDTH = 64;
defparam t.a1.W = 16;
...

<!-- Page 944 -->

IEEE Std endmodule
module top8 (...);
parameter WIDTH = 32;
adder a1 #(.ID("a1")) (...);
adder a2 #(.ID("a2"),.W(WIDTH))(...);
endmodule
module adder #(parameter ID = "id",
W = 8,
D = 512)
(...);
...
$display("ID = %s, W = %d, D = %d", ID, W, D);
...
endmodule
config cfg6;
design rtlLib.test;
instance test.t use #(.WIDTH(48));
endconfig
Example 8:
With cfg6 configuring module top8.a1, the $display in adder will print the following:
ID = a1, W=16, D = 512
ID = a2, W=48, D = 512
### 33.5 Using libraries and configs
This subclause describes potential use models for referencing configs on the command line. It is included for
clarification purposes.
The traditional SystemVerilog simulation use model takes a file-based approach, where the source
descriptions for all cells in the design are specified on the command line for each invocation of the tool.
With the advent of compiled-code simulators, the configuration mechanism shall also support a use model
that allows for the source files to be precompiled and then for the precompiled design objects to be
referenced on the command line. This subclause explains how configurations can be used in both of these
scenarios.
#### 33.5.1 Precompiling in a single-pass use model
The single-pass use model is the traditional use model with which most users are familiar. In this use model,
all of the source description files shall be provided to the simulator via the command line, and only these
source descriptions can be used to bind cell instances in the current design. A precompiling strategy in this
scenario actually parses every cell description provided on the command line and maps it into the library
without regard to whether the cell actually is used in the design. The tool can optionally check to see
whether the cell already exists in the library and, if it is up-to-date (i.e., the source description has not
changed since the last time the cell was compiled), can skip recompiling the cell. After all cells on the
command line have been compiled, then the tool can locate the top-level cell (discussed in 23.3.1) and
proceed down the hierarchy, binding each instance as it is encountered in the hierarchy.
NOTE—With this use model, it is not necessary for library objects to persist from one tool invocation to another
(although for performance considerations it is recommended they do).

<!-- Page 945 -->

IEEE Std #### 33.5.2 Elaboration-time compiling in a single-pass use model
An alternate strategy that can be used with a single-pass tool is to parse the source files only to find the
top-level module(s), without actually compiling anything into the library during this scanning process. Once
the top-level module(s) has been found, then it can be compiled into the library, and the tool can proceed
down the hierarchy, only compiling the source descriptions necessary to bind the design successfully. Based
on the binding rules in place, only the source files that match the current library specification need to be
parsed to find the current cell’s source description to compile. As with the precompiled single-pass use
model, it is not necessary for library cells to persist from one invocation to another using this strategy.
#### 33.5.3 Precompiling using a separate compilation tool
When using a separate compilation tool, it is essential that library cells persist, and the compiled forms shall,
therefore, exist somewhere in the filesystem. The exact format and location for holding these compiled
forms shall be tool-specific. Using this separate compiler strategy, the source descriptions shall be parsed
and compiled into the library using one or more invocations of the compiler tool. The only restriction is that
all cells in a design shall be precompiled prior to binding the design (typically via an invocation of a separate
tool). Using this strategy, the tool that actually does the binding only needs to be told the top-level module(s)
of the design to be bound, and then it shall use the precompiled form of the cell description(s) from the
library to determine the subinstances and descend hierarchically down the design, binding each cell as it is
located.
#### 33.5.4 Command line considerations
In each of the three preceding strategies, either the binding rules can be specified via a config, or the default
rules (from the library map file) can be used. In the single-pass use models, the config can be specified by
including its source description file on the command line. In the case where the config includes a design
statement, then the specified cell shall be the top-level module, regardless of the presence of any
uninstantiated cells in the rest of the source files. When using a separate compilation tool, the tool that
actually does the binding only needs to be given the lib.cell specification for the top-level cell(s) and/or the
config to be used. In this strategy, the config itself shall also be precompiled.
### 33.6 Configuration examples
Consider the following set of source descriptions:
file top.v file adder.v file adder.vg file lib.map
module top(...); module adder(...); module adder(...); library rtlLib top.v;
... ... // rtl ... // gate-level library aLib adder.*;
adder a1(...); m f1(...); m f1(...); library gateLib
adder a2(...); m f2(...); m f2(...); adder.vg;
endmodule endmodule endmodule
module m(...); module m(...); module m(...);
... // rtl ... // rtl ... // gate-level
endmodule endmodule endmodule
All of the examples in this subclause shall assume the top.v, adder.v and adder.vg files get compiled
with the given lib.map file. This yields the following library structure:
rtlLib.top // from top.v
rtlLib.m // from top.v
aLib.adder // from adder.v
aLib.m // rtl from adder.v
gateLib.adder // from adder.vg
gateLib.m // from adder.vg

<!-- Page 946 -->

IEEE Std #### 33.6.1 Default configuration from library map file
With no configuration, the libraries are searched according to the library declaration order in the library map
file. In other words, all instances of module adder shall use aLib.adder (because aLib is the first library
specified that contains a cell named adder), and all instances of module m shall use rtlLib.m (because
rtlLib is the first library that contains m).
#### 33.6.2 Using default clause
To always use the m definition from file adder.v, use the following simple configuration:
config cfg1;
design rtlLib.top ;
default liblist aLib rtlLib;
endconfig
The default liblist statement overrides the library search order in the lib.map file; therefore, aLib is
always searched before rtlLib. Because the gateLib library is not included in the liblist, the
gate-level descriptions of adder and m shall not be used.
To use the gate-level representations of adder and m, add to the config as follows:
config cfg2;
design rtlLib.top ;
default liblist gateLib aLib rtlLib;
endconfig
This shall cause the gate representation always to be taken before the rtl representation, using the module
definitions for adder and m from adder.vg. The rtl view of top shall be taken because there is no gate
representation available.
#### 33.6.3 Using cell clause
To modify the config to use the rtl view of adder and the gate-level representation of m from gateLib,
use the following:
config cfg3;
design rtlLib.top ;
default liblist aLib rtlLib;
cell m use gateLib.m ;
endconfig
The cell clause selects all cells named m and explicitly binds them to the gate representation in gateLib.
#### 33.6.4 Using instance clause
To modify the config so the top.a1 adder (and its descendants) use the gate representation and the
top.a2 adder (and its descendants), use the rtl representation from aLib:
config cfg4
design rtlLib.top ;
default liblist gateLib rtlLib;
instance top.a2 liblist aLib;
endconfig

<!-- Page 947 -->

IEEE Std Because the liblist is inherited, all of the descendants of top.a2 inherit its liblist from the instance
selection clause.
#### 33.6.5 Using hierarchical config
Now suppose all this work has only been on the adder module by itself and a config that uses the rtlLib.m
cell for f1, and the gateLib.m cell for f2 has already been developed. Then, use the following:
config cfg5;
design aLib.adder;
default liblist gateLib aLib;
instance adder.f1 liblist rtlLib;
endconfig
To use this configuration cfg5 for the top.a2 instance of adder and take the full default aLib adder for
the top.a1 instance, use the following config:
config cfg6;
design rtlLib.top;
default liblist aLib rtlLib;
instance top.a2 use work.cfg5:config ;
endconfig
The binding clause specifies the work.cfg5:config configuration is to be used to resolve the bindings of
instance top.a2 and its descendants. It is the design statement in config cfg5 that defines the exact
binding for the top.a2 instance itself. The rest of cfg5 defines the rules to bind the descendants of
top.a2. Notice the instance clause in cfg5 is relative to its own top-level module, adder.
### 33.7 Displaying library binding information
It shall be possible to display the actual library binding information for module instances during simulation.
The format specifier %l or %L shall print out the library.cell binding information for the module
instance containing the display (or other textual output) command. This is similar to the %m format specifier,
which prints out the hierarchical path name of the module containing it.
It shall also be able to use VPI to display the binding information. The following VPI properties shall exist
for objects of type vpiModule:
— vpiLibrary—the library name into which the module was compiled
— vpiCell—the name of the cell bound to the module instance
— vpiConfig—the library.cell name of the config controlling the binding of the module
instance
These properties shall be of string type, similar to the vpiName and vpiFullName properties.
### 33.8 Library mapping examples
In the absence of a configuration, it is possible to perform basic control of the library searching order when
binding a design.
When a config is used, the config overrides the rules specified in this subclause.

<!-- Page 948 -->

IEEE Std #### 33.8.1 Using the command line to control library searching
In the absence of a configuration, it shall be necessary for all compliant tools to provide a mechanism of
specifying a library search order on the command line that overrides the default order from the library map
file. This mechanism shall include specification of library names only, with the definitions of these libraries
to be taken from the library map file.
NOTE—It is recommended all compliant tools use “-L <library_name>” to specify this search order.
#### 33.8.2 File path specification examples
For example, given the following set of files:
/proj/lib1/rtl/a.v
/proj/lib2/gates/a.v
/proj/lib1/rtl/b.v
/proj/lib2/gates/b.v
From the /proj library, the following absolute file_path_specs are resolved as follows:
/proj/lib*/*/a.v = /proj/lib1/rtl/a.v, /proj/lib2/gates/a.v
.../a.v = /proj/lib1/rtl/a.v, /proj/lib2/gates/a.v
/proj/.../b.v = /proj/lib1/rtl/b.v, /proj/lib2/gates/b.v
.../rtl/*.v = /proj/lib1/rtl/a.v, /proj/lib1/rtl/b.v
From the /proj/lib1 directory, the following relative file_path_specs are resolved as follows:
../lib2/gates/*.v = /proj/lib2/gates/a.v, /proj/lib2/gates/b.v
./rtl/?.v = /proj/lib1/rtl/a.v, /proj/lib1/rtl/b.v
./rtl/ = /proj/lib1/rtl/a.v, /proj/lib1/rtl/b.v
#### 33.8.3 Resolving multiple path specifications
For example:
library lib1 "/proj/lib/foo*.v";
library lib2 "/proj/lib/foo.v";
library lib3 "../lib/";
library lib4 "/proj/lib/*ver.v";
When evaluated from the directory /proj/tb directory, the following source files shall map into the
specified library:
../lib/foobar.v // Maps to library lib1. Potentially matches lib1 and
// lib3. Because lib1 includes a filename and lib3 only
// specifies a directory, lib1 takes precedence
// (see 33.3.1.1 b)
/proj/lib/foo.v // Maps to library lib2. Takes precedence over lib1 and
// lib3 path specifications (see 33.3.1.1 a)
/proj/lib/bar.v // Maps to library lib3. (see 33.3.1.1 c)
/proj/lib/barver.v // Maps to library lib4. Takes precedence over lib3 path
// specification (see 33.3.1.1 b)

<!-- Page 949 -->

IEEE Std /proj/lib/foover.v // ERROR, matches lib1 and lib4 (see 33.3.1.1)
/test/tb/tb.v // Maps to library work. Does not match any library
// specification (see 33.3.1)

<!-- Page 950 -->

IEEE Std 1800-2023