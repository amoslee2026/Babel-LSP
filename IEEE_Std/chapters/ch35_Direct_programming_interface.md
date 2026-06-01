# Chapter 35: Direct programming interface

## 35. Direct programming interface
### 35.1 General
This clause describes the following:
— Direct programming interface (DPI) tasks and functions
— DPI layers
— Importing and exporting functions
— Importing and exporting tasks
— Disabling DPI tasks and functions
### 35.2 Overview
This clause highlights the DPI and provides a detailed description of the SystemVerilog layer of the
interface. The C layer is defined in AnnexH.
DPI is an interface between SystemVerilog and a foreign programming language. It consists of two separate
layers: the SystemVerilog layer and a foreign language layer. Both sides of DPI are fully isolated. Which
programming language is actually used as the foreign language is transparent and irrelevant for the
SystemVerilog side of this interface. Neither the SystemVerilog compiler nor the foreign language compiler
is required to analyze the source code in the other’s language. Different programming languages can be used
and supported with the same intact SystemVerilog layer. For now, however, SystemVerilog defines a
foreign language layer only for the C programming language. See AnnexH for more details.
The motivation for this interface is two-fold. The methodological requirement is that the interface should
allow a heterogeneous system to be built (a design or a testbench) in which some components can be written
in a language (or more languages) other than SystemVerilog, hereinafter called the foreign language. On the
other hand, there is also a practical need for an easy and efficient way to connect existing code, usually
written in C or C++, without the knowledge and the overhead of VPI.
DPI follows the principle of a black box: the specification and the implementation of a component are
clearly separated, and the actual implementation is transparent to the rest of the system. Therefore, the actual
programming language of the implementation is also transparent, although this standard defines only C
linkage semantics. The separation between SystemVerilog code and the foreign language is based on using
functions as the natural encapsulation unit in SystemVerilog. By and large, any function can be treated as a
black box and implemented either in SystemVerilog or in the foreign language in a transparent way, without
changing its calls.
#### 35.2.1 Tasks and functions
DPI allows direct inter-language function calls between the languages on either side of the interface.
Specifically, functions implemented in a foreign language can be called from SystemVerilog; such functions
are referred to as imported functions. SystemVerilog functions that are to be called from a foreign code shall
be specified in export declarations (see 35.7 for more details). DPI allows for passing SystemVerilog data
between the two domains through function arguments and results. There is no intrinsic overhead in this
interface.
It is also possible to perform task enables across the language boundary. Foreign code can call
SystemVerilog tasks, and native SystemVerilog code can call imported tasks. An imported task has the same
semantics as a native SystemVerilog task: it never returns a value, and it can consume simulation time.

<!-- Page 972 -->

IEEE Std All functions used in DPI are assumed to complete their execution instantly and consume zero simulation
time, just as normal SystemVerilog functions. DPI provides no means of synchronization other than by data
exchange and explicit transfer of control.
Every imported subroutine needs to be declared. A declaration of an imported subroutine is referred to as an
import declaration. Import declarations are very similar to SystemVerilog subroutine declarations. Import
declarations can occur anywhere where SystemVerilog subroutine definitions are permitted. An import
declaration is considered to be a definition of a SystemVerilog subroutine with a foreign language
implementation. The same foreign subroutine can be used to implement multiple SystemVerilog tasks and
functions (this can be a useful way of providing differing default argument values for the same basic
subroutine), but a given SystemVerilog name can only be defined once per scope. Imported subroutines can
have zero or more formal input, output, and inout arguments. Imported tasks always return a void value
and thus can only be used in statement context. Imported functions can return a result or be defined as void
functions.
DPI is based entirely upon SystemVerilog constructs. The usage of imported functions is identical to the
usage of native SystemVerilog functions. With few exceptions, imported functions and native functions are
mutually exchangeable. Calls of imported functions are indistinguishable from calls of SystemVerilog
functions. This facilitates ease of use and minimizes the learning curve. Similar interchangeable semantics
exist between native SystemVerilog tasks and imported tasks.
#### 35.2.2 Data types
SystemVerilog data types are the sole data types that can cross the boundary between SystemVerilog and a
foreign language in either direction (i.e., when an imported function is called from SystemVerilog code or
an exported SystemVerilog function is called from a foreign code). It is not possible to import the data types
or directly use the type syntax from another language. A rich subset of SystemVerilog data types is allowed
for formal arguments of import and export functions, although with some restrictions and with some
notational extensions. Function result types are restricted to small values, however (see 35.5.5).
Formal arguments of an imported function can be declared as open arrays as specified in 35.5.6.1.
35.2.2.1 Data representation
DPI does not add any constraints on how SystemVerilog-specific data types are actually implemented.
Optimal representation can be platform dependent. The layout of 2- or 4-state packed structures and arrays is
implementation and platform dependent.
The implementation (representation and layout) of 4-state values, structures, and arrays is irrelevant for
SystemVerilog semantics and can only impact the foreign side of the interface.
### 35.3 Two layers of DPI
DPI consists of two separate layers: the SystemVerilog layer and a foreign language layer. The
SystemVerilog layer does not depend on which programming language is actually used as the foreign
language. Although different programming languages can be supported and used with the intact
SystemVerilog layer, SystemVerilog defines a foreign language layer only for the C programming language.
Nevertheless, SystemVerilog code shall look identical and its semantics shall be unchanged for any foreign
language layer. Different foreign languages can require that the SystemVerilog implementation shall use the
appropriate function call protocol and argument passing and linking mechanisms. This shall be, however,
transparent to SystemVerilog users. SystemVerilog requires only that its implementation shall support C
protocols and linkage.

<!-- Page 973 -->

IEEE Std #### 35.3.1 DPI SystemVerilog layer
The SystemVerilog side of DPI does not depend on the foreign programming language. In particular, the
actual function call protocol and argument passing mechanisms used in the foreign language are transparent
and irrelevant to SystemVerilog. SystemVerilog code shall look identical regardless of what code the
foreign side of the interface is using. The semantics of the SystemVerilog side of the interface is
independent from the foreign side of the interface.
This clause does not constitute a complete interface specification. It only describes the functionality,
semantics, and syntax of the SystemVerilog layer of the interface. The other half of the interface, the foreign
language layer, defines the actual argument passing mechanism and the methods to access (read/write)
formal arguments from the foreign code. See AnnexH for more details.
#### 35.3.2 DPI foreign language layer
The foreign language layer of the interface (which is transparent to SystemVerilog) shall specify how actual
arguments are passed, how they can be accessed from the foreign code, how SystemVerilog-specific data
types (such as logic and packed) are represented, and how they are translated to and from some
predefined C-like types.
The data types allowed for formal arguments and results of imported functions or exported functions are
generally SystemVerilog types (with some restrictions and with notational extensions for open arrays).
Users are responsible for specifying in their foreign code the native types equivalent to the SystemVerilog
types used in imported declarations or export declarations. Software tools, like a SystemVerilog compiler,
can facilitate the mapping of SystemVerilog types onto foreign native types by generating the appropriate
function headers.
The SystemVerilog compiler or simulator shall generate and/or use the function call protocol and argument
passing mechanisms required for the intended foreign language layer. The same SystemVerilog code
(compiled accordingly) shall be usable with different foreign language layers, regardless of the data access
method assumed in a specific layer. AnnexH defines the DPI foreign language layer for the C programming
language.
### 35.4 Global name space of imported and exported functions
Every subroutine imported to SystemVerilog shall eventually resolve to a global symbol. Similarly, every
subroutine exported from SystemVerilog defines a global symbol. Thus the tasks and functions imported to
and exported from SystemVerilog have their own global name space of linkage names, different from
compilation-unit scope name space. Global names of imported and exported tasks and functions shall be
unique (no overloading is allowed) and shall follow C conventions for naming; specifically, such names
shall start with a letter or underscore, and they can be followed by alphanumeric characters or underscores.
Exported and imported tasks and functions, however, can be declared with local SystemVerilog names.
Import and export declarations allow users to specify a global name for a function in addition to its declared
name. Should a global name clash with a SystemVerilog keyword or a reserved name, it shall take the form
of an escaped identifier. The leading backslash ( \ ) character and the trailing white space shall be stripped
off by the SystemVerilog tool to create the linkage identifier. After this stripping, the linkage identifier so
formed shall comply with the normal rules for C identifier construction. If a global name is not explicitly
given, it shall be the same as the SystemVerilog subroutine name. For example:
export "DPI-C" f_plus = function \f+ ; // "f+" exported as "f_plus"
export "DPI-C" function f; // "f" exported under its own name
import "DPI-C" init_1 = function void \init[1] (); // "init_1" is a linkage name
import "DPI-C" \begin = function void \init[2] (); // "begin" is a linkage name

<!-- Page 974 -->

IEEE Std The same global subroutine can be referred to in multiple import declarations in different scopes or/and with
different SystemVerilog names (see 35.5.4).
Multiple export declarations are allowed with the same c_identifier, explicit or implicit, as long as they are
in different scopes and have the equivalent type signature (as defined in 35.5.4 for imported tasks and
functions). Multiple export declarations with the same c_identifier in the same scope are forbidden.
It is possible to use the deprecated "DPI" version string syntax in an import or export declaration. This
syntax indicates that the SystemVerilog 2-state and 4-state packed array argument passing convention is to
be used (see H.14). In such cases, all declarations using the same c_identifier shall be declared with the same
DPI version string syntax.
### 35.5 Imported tasks and functions
The usage of imported functions is similar as for native SystemVerilog functions.
#### 35.5.1 Required properties of imported tasks and functions—semantic constraints
This subclause defines the semantic constraints imposed on imported subroutines. Some semantic
restrictions are shared by all imported subroutines. Other restrictions depend on whether the special
properties pure (see 35.5.2) or context (see 35.5.3) are specified for an imported subroutine. A
SystemVerilog compiler is not able to verify that those restrictions are observed; and if those restrictions are
not satisfied, the effects of such imported subroutine calls can be unpredictable.
35.5.1.1 Instant completion of imported functions
Imported functions shall complete their execution instantly and consume zero simulation time, similarly to
native functions.
NOTE—Imported tasks can consume time, similar to native SystemVerilog tasks.
35.5.1.2 input, output, and inout arguments
Imported functions can have input, output, and inout arguments. The formal input arguments shall not
be modified. If such arguments are changed within a function, the changes shall not be visible outside the
function; the actual arguments shall not be changed.
The imported function shall not assume anything about the initial values of formal output arguments. The
initial values of output arguments are undetermined and implementation dependent.
The imported function can access the initial value of a formal inout argument. Changes that the imported
function makes to a formal inout argument shall be visible outside the function.
35.5.1.3 Special properties pure and context
Special properties can be specified for an imported subroutine as pure or as context (see also 35.5.2 or
35.5.3).
A function whose result depends solely on the values of its input arguments and with no side effects can be
specified as pure. This can usually allow for more optimizations and thus can result in improved simulation
performance. Subclause 35.5.2 details the rules that shall be obeyed by pure functions. An imported task
can never be declared pure.

<!-- Page 975 -->

IEEE Std An imported subroutine that is intended to call exported subroutines or to access SystemVerilog data objects
other than its actual arguments (e.g., via VPI calls) shall be specified as context. Calls of context tasks
and functions are specially instrumented and can impair SystemVerilog compiler optimizations; therefore,
simulation performance can decrease if the context property is specified when not necessary. A subroutine
not specified as context shall not read or write any data objects from SystemVerilog other than its actual
arguments. For subroutines not specified as context, the effects of calling VPI or exported SystemVerilog
subroutines can be unpredictable and can lead to unexpected behavior; such calls can even crash. Subclause
#### 35.5.3 details the restrictions that shall be obeyed by noncontext subroutines.
If neither the pure nor the context attribute is used on an imported subroutine, the subroutine shall not
access SystemVerilog data objects; however, it can perform side effects such as writing to a file or
manipulating a global variable.
35.5.1.4 Memory management
The memory spaces owned and allocated by the foreign code and SystemVerilog code are disjoined. Each
side is responsible for its own allocated memory. Specifically, an imported function shall not free the
memory allocated by SystemVerilog code (or the SystemVerilog compiler) nor expect SystemVerilog code
to free the memory allocated by the foreign code (or the foreign compiler). This does not exclude scenarios
where foreign code allocates a block of memory and then passes a handle (i.e., a pointer) to that block to
SystemVerilog code, which in turn calls an imported function (e.g., C standard function free) that directly
or indirectly frees that block.
NOTE—In this last scenario, a block of memory is allocated and freed in the foreign code, even when the standard
functions malloc and free are called directly from SystemVerilog code.
35.5.1.5 Reentrancy of imported tasks
A call to an imported task can result in the suspension of the currently executing thread. This occurs when an
imported task calls an exported task, and the exported task executes a delay control, event control, or wait
statement. Thus it is possible for an imported task’s C code to be simultaneously active in multiple execution
threads. Standard reentrancy considerations need to be made by the C programmer. Some examples of such
considerations include the use of static variables and ensuring that only thread-safe C standard library calls
(multi-thread safe) are used.
35.5.1.6 C++ exceptions
It is possible to implement DPI imported tasks and functions using C++, as long as C linkage conventions
are observed at the language boundary. If C++ is used, exceptions shall not propagate out of any imported
subroutine. Undefined behavior can result if an exception crosses the language boundary from C++ into
SystemVerilog.
#### 35.5.2 Pure functions
A pure function call can be eliminated if its result is not needed or if the previous result for the same values
of input arguments is available somehow and can be reused without needing to recalculate. Only nonvoid
functions with no output or inout arguments can be specified as pure. Functions specified as pure shall
have no side effects whatsoever; their results need to depend solely on the values of their input arguments.
Calls to such functions can be removed by SystemVerilog compiler optimizations or replaced with the
values previously computed for the same values of the input arguments.
Specifically, a pure function is assumed not to directly or indirectly (i.e., by calling other functions)
perform the following:
— Perform any file operations.

<!-- Page 976 -->

IEEE Std — Read or write anything in the broadest possible meaning, including input/output (I/O), environment
variables, objects from the operating system or from the program or other processes, shared
memory, sockets, etc.
— Access any persistent data, like global or static variables.
If a pure function does not obey the preceding restrictions, SystemVerilog compiler optimizations can lead
to unexpected behavior, due to eliminated calls or incorrect results being used.
#### 35.5.3 Context tasks and functions
Some DPI imported subroutines require that the context of their call be known. It takes special
instrumentation of their call instances to provide such context; for example, an internal variable referring to
the “current instance” might need to be set. To avoid any unnecessary overhead, imported subroutine calls in
SystemVerilog code are not instrumented unless the imported subroutine is specified as context.
The SystemVerilog context of DPI export tasks and functions needs to be known when they are called,
including when they are called by imports. When an import invokes the svSetScope utility prior to calling
the export, it sets the context explicitly. Otherwise, the context will be the context of the instantiated scope
where the import declaration is located. Because imports with diverse instantiated scopes can export the
same subroutine, multiple instances of such an export can exist after elaboration. Prior to any invocations of
svSetScope, these export instances would have different contexts, which would reflect their imported
caller’s instantiated scope.
A foreign language subroutine supported through some other interface (a VPI callback for example), can
also make a call to svSetScope or to other DPI scope-related APIs. This foreign language subroutine can
also call an export subroutine declared in a specific instantiated scope by first making a call to svSetScope.
The behavior of the DPI scope-related APIs and invocation of DPI export subroutines will be simulator
defined and is beyond the scope of the DPI specification.
The concept of call chains is useful for understanding how context works as control weaves in and out of
SystemVerilog and another language through a DPI interface. For the purpose of this description, an inter-
language call is between SystemVerilog and a DPI supported language, or vice versa. An intra-language
call is between SystemVerilog subroutines themselves, or between subroutines in the DPI support language,
i.e., the call does not cross the language boundary.
A DPI import call chain is an inter-language call chain starting from SystemVerilog into a subroutine that is
defined in a DPI supported language. The starting point of the call chain from SystemVerilog is called the
root of the call chain.This call chain can comprise multiple intra-language and inter-language calls between
SystemVerilog and DPI supported language before it unwinds and returns to the calling SystemVerilog
subroutine at the root of the import call chain.
The subroutine in the DPI supported language could make intra-language calls within the language and then
could unwind and return back to the calling SystemVerilog subroutine. Alternatively, the called import
subroutine could make an inter-language call to an export DPI subroutine in the imported caller’s
instantiated scope, or by setting another instantiated scope and calling the export DPI routine in that scope.
The called export DPI subroutine can make intra-language calls in SystemVerilog, or make an inter-
language call to yet another import subroutine or simply return to the calling import subroutine. This nested
invocation of inter-language and intra-language calls is considered a part of a single import call chain.
Another key point to note is that the context property applies to each import subroutine call that is made
from SystemVerilog. This implies that the context property at the root of the DPI import call chain or any
intermediate import call in the call chain is not transitively promoted to subsequent import calls in the DPI

<!-- Page 977 -->

IEEE Std import call chain. Since a noncontext imported DPI subroutine cannot make a call to a SystemVerilog export
subroutine, the behavior of making any such calls in the DPI import call chain is an error.
The following behavior characterizes context mechanics for imported call chains:
— The following actions determine an import call chain’s context value:
• When a SystemVerilog subroutine calls a DPI context import, a context for the import call chain
is created that is equal to the instantiated scope of the import declaration.
• When a routine in an import call chain invokes svSetScope with a legal argument, the call
chain’s context is set to the indicated scope.
• When a call from an import call chain to an exported SystemVerilog subroutine finishes and
returns to the chain, the call chain’s context is set equal to its value when the call to the export
was made.
— Detecting when control moves across the language boundary between SystemVerilog and an
imported language is critical for simulators managing DPI context. Therefore, if user code
circumvents unwinding an export call chain back to its import chain caller (e.g., by using C setjmp/
longjmp constructs), the results are undefined.
— Whether a specific import subroutine call in the DPI import call chain is context or not is governed
by the context property of the import subroutine to which the call was made. The context property of
a previous import subroutine call in the DPI import call chain is not transitively promoted to all
subsequent import function calls in the call chain.
— The context characteristic of a DPI import call cannot be dynamically changed after the initial call to
the import subroutine in the DPI supported language.
— The context characteristic adheres to the calling chain, not to an individual imported subroutine;
thus, the same imported subroutine can appear in both context and noncontext call chains.
For the sake of simulation performance, an imported subroutine call shall not block SystemVerilog compiler
optimizations. An imported subroutine not specified as context shall not access any data objects from
SystemVerilog other than its actual arguments. Only the actual arguments can be affected (read or written)
by its call. Therefore, a call of a noncontext subroutine is not a barrier for optimizations. A context imported
subroutine, however, can access (read or write) any SystemVerilog data objects by calling VPI or by calling
an export subroutine. Therefore, a call to a context subroutine is a barrier for SystemVerilog compiler
optimizations.
Only calls of context imported subroutines are properly instrumented and cause conservative optimizations;
therefore, only those subroutines can safely call all subroutines from other APIs, including VPI functions or
exported SystemVerilog subroutines. For imported subroutines not specified as context, the effects of
calling VPI functions or SystemVerilog subroutines can be unpredictable; and such calls can crash if the
callee requires a context that has not been properly set. (See H.9.5 for the list of VPI functions that can
safely be called from imported subroutines not specified as context.)
However, declaring an imported subroutine context does not automatically make any other simulator
interface automatically available. For VPI access (or any other interface access) to be possible, the
appropriate implementation-defined mechanism shall still be used to enable these interface(s). Realize also
that DPI calls do not automatically create or provide any handles or any special environment that can be
needed by those other interfaces. It is the user’s responsibility to create, manage, or otherwise manipulate
the required handles or environment(s) needed by the other interfaces.
Context imported subroutines are always implicitly supplied a scope representing the fully qualified instance
name within which the import declaration was present. This scope defines which exported SystemVerilog
subroutines can be called directly from the imported subroutine; only subroutines defined and exported from
the same scope as the import can be called directly. To call any other exported SystemVerilog subroutines,

<!-- Page 978 -->

IEEE Std the imported subroutine shall first have to modify its current scope, in essence performing the foreign
language equivalent of a SystemVerilog hierarchical subroutine call.
Special DPI utility functions exist that allow imported subroutines to retrieve and operate on their scope. See
AnnexH for more details.
#### 35.5.4 Import declarations
Each imported subroutine shall be declared. Such declarations are referred to as import declarations.
Imported subroutines are similar to SystemVerilog subroutines. Imported subroutines can have zero or more
formal input, output, and inout arguments. Imported functions can return a result or be defined as void
functions. Imported tasks always return an int result as part of the DPI disable protocol and, thus, are
declared in foreign code as int functions (see 35.8 and 35.9).
dpi_import_export ::= // from A.2.6
import dpi_spec_string [ dpi_function_import_property ] [ c_identifier = ] dpi_function_proto ;
| import dpi_spec_string [ dpi_task_import_property ] [ c_identifier = ] dpi_task_proto ;
| export dpi_spec_string [ c_identifier = ] function function_identifier ;
| export dpi_spec_string [ c_identifier = ] task task_identifier ;
dpi_spec_string ::= "DPI-C" | "DPI"
dpi_function_import_property ::= context | pure
dpi_task_import_property ::= context
dpi_function_proto26,27 ::= function_prototype
dpi_task_proto27 ::= task_prototype
function_prototype ::=
function [ dynamic_override_specifiers ]25 data_type_or_void function_identifier
[ ( [ tf_port_list ] ) ]
task_prototype ::=
task [ dynamic_override_specifiers ]25 task_identifier [ ( [ tf_port_list ] ) ] // from A.2.7
25) The dynamic_override_specifiers shall only be legal on method declarations inside a non-interface class scope.
26) dpi_function_proto return types are restricted to small values, per 35.5.5.
27) Formals of dpi_function_proto and dpi_task_proto cannot use pass by reference mode and class types cannot be
passed at all; see 35.5.6 for a description of allowed types for DPI formal arguments.
Syntax35-1—DPI import declaration syntax (excerpt from AnnexA)
An import declaration specifies the subroutine name, function result type, and types and directions of formal
arguments. It can also provide optional default values for formal arguments. Formal argument names are
optional unless argument binding by name is needed. An import declaration can also specify an optional
subroutine property. Imported functions can have the properties context or pure; imported tasks can have
the property context.
Because an import declaration is equivalent to defining a subroutine of that name in the SystemVerilog
scope in which the import declaration occurs, multiple imports of the same subroutine name into the same
scope are forbidden.
NOTE—This declaration scope is particularly important in the case of imported context subroutines (see 35.5.3); for
noncontext imported subroutines the declaration scope has no other implications other than defining the visibility of the
subroutine.

<!-- Page 979 -->

IEEE Std The dpi_spec_string can take values "DPI-C" and "DPI". "DPI" is used to indicate that the deprecated
SystemVerilog packed array argument passing semantics is to be used. In this semantics, arguments are
passed in actual simulator representation format rather than in canonical format, as is the case with
"DPI-C".
Use of the string "DPI" shall generate a compile-time warning or error. The tool-generated message shall
contain the following information:
— "DPI" is deprecated and should be replaced with "DPI-C".
— Use of the "DPI-C" string may require changes in the DPI application’s C code.
For more information on using deprecated "DPI" access to packed data, see H.14.
The c_identifier provides the linkage name for this subroutine in the foreign language. If not provided, this
defaults to the same identifier as the SystemVerilog subroutine name. In either case, this linkage name shall
conform to C identifier syntax. An error shall occur if the c_identifier, either directly or indirectly, does not
conform to these rules.
For any given c_identifier (whether explicitly defined with c_identifier= or automatically determined from
the subroutine name), all declarations, regardless of scope, shall have exactly the same type signature. The
signature includes the return type and the number, order, direction, and types of each and every argument.
The type includes dimensions and bounds of any arrays or array dimensions. The signature also includes the
pure/context qualifiers that can be associated with an import definition, and it includes the value of the
dpi_spec_string.
It is permitted to have multiple declarations of the same imported or exported subroutine in different scopes;
therefore, argument names and default values can vary, provided the type compatibility constraints are met.
A formal argument name is required to separate the packed and the unpacked dimensions of an array.
The qualifier ref cannot be used in import declarations. The actual implementation of argument passing
depends solely on the foreign language layer and its implementation and shall be transparent to the
SystemVerilog side of the interface.
The following are examples of import declarations:
import "DPI-C" function void myInit();
// from standard math library
import "DPI-C" pure function real sin(real);
// from standard C library: memory management
import "DPI-C" function chandle malloc(int size); // standard C function
import "DPI-C" function void free(chandle ptr); // standard C function
// abstract data structure: queue
import "DPI-C" function chandle newQueue(input string name_of_queue);
// Note the following import uses the same foreign function for
// implementation as the prior import, but has different SystemVerilog name
// and provides a default value for the argument.
import "DPI-C" newQueue=function chandle newAnonQueue(input string s=null);
import "DPI-C" function chandle newElem(bit [15:0]);
import "DPI-C" function void enqueue(chandle queue, chandle elem);
import "DPI-C" function chandle dequeue(chandle queue);

<!-- Page 980 -->

IEEE Std // miscellanea
import "DPI-C" function bit [15:0] getStimulus();
import "DPI-C” context function void processTransaction(chandle elem,
output logic [64:1] arr [0:63]);
import "DPI-C" task checkResults(input string s, bit [511:0] packet);
#### 35.5.5 Function result
An imported function declaration shall explicitly specify a data type or void for the type of the function’s
return result. Function result types are restricted to small values. The following SystemVerilog data types
are allowed for imported function results:
— void, byte, shortint, int, longint, real, shortreal, chandle, and string
— Scalar values of type bit and logic
The same restrictions apply for the result types of exported functions.
#### 35.5.6 Types of formal arguments
A rich subset of SystemVerilog data types is allowed for formal arguments of import and export subroutines.
Generally, C-compatible types, packed types, and user-defined types built of types from these two
categories can be used for formal arguments of DPI subroutines. The set of permitted types is defined
inductively.
The following SystemVerilog types are the only permitted types for formal arguments of import and export
subroutines:
— void, byte, shortint, int, longint, real, shortreal, chandle, time, integer, and
string
— Scalar values of type bit and logic
— Packed arrays, structs, and unions composed of types bit and logic. Every packed type is
eventually equivalent to a packed one-dimensional array. On the foreign language side of the DPI,
all packed types are perceived as packed one-dimensional arrays regardless of their declaration in
the SystemVerilog code.
— Enumeration types interpreted as the type associated with that enumeration
— Types constructed from the supported types with the help of the following constructs:
• struct
• union (packed forms only)
• Unpacked array
• typedef
The following caveats apply for the types permitted in DPI:
— Enumerated data types are not supported directly. Instead, an enumerated data type is interpreted as
the type associated with that enumerated type.
— SystemVerilog does not specify the actual memory representation of packed structures or any
arrays, packed or unpacked. Unpacked structures have an implementation-dependent packing,
normally matching the C compiler.
— In exported DPI subroutines, it is erroneous to declare formal arguments of dynamic array types.
— The actual memory representation of SystemVerilog data types is transparent for SystemVerilog
semantics and irrelevant for SystemVerilog code. It can be relevant for the foreign language code on
the other side of the interface, however; a particular representation of the SystemVerilog data types
can be assumed. This shall not restrict the types of formal arguments of imported subroutines, with

<!-- Page 981 -->

IEEE Std the exception of unpacked arrays. SystemVerilog implementation can restrict which SystemVerilog
unpacked arrays are passed as actual arguments for a formal argument that is a sized array, although
they can be always passed for an unsized (i.e., open) array. Therefore, the correctness of an actual
argument might be implementation dependent. Nevertheless, an open array provides an
implementation-independent solution.
35.5.6.1 Open arrays
The size of the packed dimension, the unpacked dimension, or both dimensions can remain unspecified;
such cases are referred to as open arrays (or unsized arrays). Open arrays allow the use of generic code to
handle different sizes.
Formal arguments of imported functions can be specified as open arrays. (Exported SystemVerilog
functions cannot have formal arguments specified as open arrays.) A formal argument is an open array when
a range of one or more of its dimensions is unspecified (denoted by using square brackets, []). This is solely
a relaxation of the argument-matching rules. An actual argument shall match the formal one regardless of
the range(s) for its corresponding dimension(s), which facilitates writing generalized code that can handle
SystemVerilog arrays of different sizes.
Although the packed part of an array can have an arbitrary number of sized dimensions, an unsized
dimension shall be the sole packed dimension of a formal argument. This is not very restrictive, because any
packed type is essentially equivalent to a one-dimensional packed array. The number of unpacked
dimensions is not restricted.
If a formal argument has an unsized, packed dimension, it will match any collection of actual argument
packed dimensions. Formal argument unpacked dimensions are matched on a dimension-by-dimension basis
(see 7.7) with each unsized formal dimension matching a corresponding actual dimension of any size.
The following are examples of types of formal arguments (empty square brackets [] denote open array):
logic
bit [8:1]
bit[]
bit [7:0] array8x10 [1:10] // array8x10 is a formal arg name
logic [31:0] array32xN [] // array32xN is a formal arg name
logic [] arrayNx3 [3:1] // arrayNx3 is a formal arg name
bit [] arrayNxN [] // arrayNxN is a formal arg name
The following is an example of complete import declarations:
import "DPI-C" function void f1(input logic [127:0]);
import "DPI-C" function void f2(logic [127:0] i []); //open array of 128-bit
The following is an example of the use of open arrays for different sizes of actual arguments:
typedef struct {int i; ... } MyType;
import "DPI-C" function void f3(input MyType i [][]);
/* 2-dimensional unsized unpacked array of MyType */
MyType a_10x5 [11:20][6:2];
MyType a_64x8 [64:1][-1:-8];
f3(a_10x5);
f3(a_64x8);

<!-- Page 982 -->

IEEE Std ### 35.6 Calling imported functions
The usage of imported functions is identical to the usage of native SystemVerilog functions. Hence the
usage and syntax for calling imported functions is identical to the usage and syntax of native SystemVerilog
functions. Specifically, arguments with default values can be omitted from the call; arguments can be bound
by name if all formal arguments are named.
#### 35.6.1 Argument passing
Argument passing for imported functions is ruled by the WYSIWYG principle: What You Specify Is What
You Get (see 35.6.1.1). The evaluation order of formal arguments follows general SystemVerilog rules.
Argument compatibility and coercion rules are the same as for native SystemVerilog functions. If a coercion
is needed, a temporary variable is created and passed as the actual argument. For input and inout
arguments, the temporary variable is initialized with the value of the actual argument with the appropriate
coercion. For output or inout arguments, the value of the temporary variable is assigned to the actual
argument with the appropriate conversion. The assignments between a temporary and the actual argument
follow general SystemVerilog rules for assignments and automatic coercion.
On the SystemVerilog side of the interface, the values of actual arguments for formal input arguments of
imported functions shall not be affected by the callee. The initial values of formal output arguments of
imported functions are unspecified (and can be implementation dependent), and the necessary coercions, if
any, are applied as for assignments. Imported functions shall not modify the values of their input arguments.
For the SystemVerilog side of the interface, the semantics of arguments passing is as if input arguments are
passed by copy-in, output arguments are passed by copy-out, and inout arguments are passed by copy-in,
copy-out. The terms copy-in and copy-out do not impose the actual implementation; they refer only to
“hypothetical assignment.”
The actual implementation of argument passing is transparent to the SystemVerilog side of the interface. In
particular, it is transparent to SystemVerilog whether an argument is actually passed by value or by
reference. The actual argument passing mechanism is defined in the foreign language layer. See AnnexH
for more details.
35.6.1.1 WYSIWYG principle
The WYSIWYG principle guarantees the types of formal arguments of imported functions: an actual
argument is guaranteed to be of the type specified for the formal argument, with the exception of open arrays
(for which unspecified ranges are statically unknown). Formal arguments, other than open arrays, are fully
defined by import declaration; they shall have ranges of packed or unpacked arrays exactly as specified in
the import declaration. Only the declaration site of the imported function is relevant for such formal
arguments.
Another way to state this is that no compiler (either C or SystemVerilog) can make argument coercions
between a caller’s declared formal and the callee’s declared formals. This is because the callee’s formal
arguments are declared in a different language from the caller’s formal arguments; hence there is no visible
relationship between the two sets of formals. Users are expected to understand all argument relationships
and provide properly matched types on both sides of the interface.
The unsized dimensions of open array formal arguments have the size of the corresponding actual argument
dimensions. A formal’s unsized, unpacked dimensions take on the ranges of the corresponding actual
dimension. A solitary, unsized, packed dimension assumes the linearized, normalized range of the actual’s
packed dimensions (see H.7.6). The unsized ranges of open arrays are determined at a call site; the rest of
the type information is specified at the import declaration.

<!-- Page 983 -->

IEEE Std Therefore, if a formal argument is declared as bit [15:8] b [], then the import declaration specifies that
the formal argument is an unpacked array of packed bit array with bounds 15 to 8, while the actual argument
used at a particular call site defines the bounds for the unpacked part for that call.
It is sometimes permissible to pass a dynamic array as an actual argument to an imported DPI subroutine.
The rules for passing dynamic array actual arguments to imported DPI tasks and functions are identical to
the rules for native SystemVerilog tasks and functions. Refer to 7.7 for details on such use of dynamic
arrays.
#### 35.6.2 Value changes for output and inout arguments
The SystemVerilog simulator is responsible for handling value changes for output and inout arguments.
Such changes shall be detected and handled after control returns from imported functions to SystemVerilog
code.
For output and inout arguments, the value propagation (i.e., value change events) happens as if an actual
argument was assigned a formal argument immediately after control returns from imported functions. If
there is more than one argument, the order of such assignments and the related value change propagation
follows general SystemVerilog rules.
### 35.7 Exported functions
DPI allows calling SystemVerilog functions from another language. However, such functions shall adhere to
the same restrictions on argument types and results as imposed on imported functions. It is an error to export
a function that does not satisfy such constraints. Declaring a SystemVerilog function to be exported does not
change its semantics or behavior from the SystemVerilog perspective; there is no effect on SystemVerilog
usage other than making it possible for foreign language tasks and functions in a DPI call-chain to call the
exported function.
SystemVerilog functions that can be called from foreign code need to be specified in export declarations.
Export declarations are allowed to occur only in the scope in which the function being exported is defined.
Only one export declaration per function is allowed in a given scope.
One important restriction exists. Class member functions cannot be exported, but all other SystemVerilog
functions can be exported.
Similar to import declarations, export declarations can define an optional c_identifier to be used in the
foreign language when calling an exported function.
dpi_import_export ::= // from A.2.6
...
| export dpi_spec_string [ c_identifier = ] function function_identifier ;
...
dpi_spec_string ::= "DPI-C" | "DPI"
Syntax35-2—DPI export declaration syntax (excerpt from AnnexA)
The c_identifier is optional here. It defaults to function_identifier. For rules describing c_identifier, see 35.4.
No two functions in the same SystemVerilog scope can be exported with the same explicit or implicit
c_identifier. The export declaration and the definition of the corresponding SystemVerilog function can
occur in any order. Only one export declaration is permitted per SystemVerilog function, and all export
functions are always context functions.

<!-- Page 984 -->

IEEE Std ### 35.8 Exported tasks
SystemVerilog allows tasks to be called from a foreign language, similar to functions. Such tasks are termed
exported tasks.
All aspects of exported functions described above in 35.7 apply to exported tasks. This includes legal
declaration scopes as well as usage of the optional c_identifier.
It is never legal to call an exported task from within an imported function. This semantics is identical to
native SystemVerilog semantics, in which it is illegal for a function to perform a task enable.
It is legal for an imported task to call an exported task only if the imported task is declared with the context
property. See 35.5.3 for more details.
One difference between exported tasks and exported functions is that SystemVerilog tasks do not have
return value types. The return value of an exported task is an int value that indicates if a disable is active or
not on the current execution thread.
Similarly, imported tasks return an int value that is used to indicate that the imported task has
acknowledged a disable. See 35.9 for more detail on disables in DPI.
### 35.9 Disabling DPI tasks and functions
It is possible for a disable statement to disable a block that is currently executing a mixed language call
chain. When a DPI import subroutine is disabled, the C code is required to follow a simple disable protocol.
The protocol gives the C code the opportunity to perform any necessary resource cleanup, such as closing
open file handles, closing open VPI handles, or freeing heap memory.
An imported subroutine is said to be in the disabled state when a disable statement somewhere in the
design targets either it or a parent for disabling. An imported subroutine can only enter the disabled state
immediately after the return of a call to an exported subroutine. An important aspect of the protocol is that
disabled import tasks and functions shall programmatically acknowledge that they have been disabled. A
subroutine can determine that it is in the disabled state by calling the API function
svIsDisabledState().
The protocol is composed of the following items:
a) When an exported task returns due to a disable, it shall return a value of 1. Otherwise, it shall
return0.
b) When an imported task returns due to a disable, it shall return a value of 1. Otherwise, it shall
return0.
c) Before an imported function returns due to a disable, it shall call the API function
svAckDisabledState().
d) Once an imported subroutine enters the disabled state, it is illegal for the current function call to
make any further calls to exported subroutines.
Item b), item c), and item d) are mandatory behavior for imported DPI tasks and functions. It is the
responsibility of the DPI programmer to correctly implement the behavior.
Item a) is guaranteed by SystemVerilog simulators. In addition, simulators shall implement checks to verify
that item b), item c), and item d) are correctly followed by imported tasks and functions. If any protocol item
is not correctly followed, a fatal simulation error is issued.

<!-- Page 985 -->

IEEE Std The foreign language side of the DPI contains a disable protocol that is realized by user code working
together with a simulator. The disable protocol allows for foreign models to participate in SystemVerilog
disable processing. The participation is done through special return values for DPI tasks and special API
calls for DPI functions.
The special return values do not require a change in call syntax of either import or export DPI tasks in the
SystemVerilog code. While the return value for an export task is guaranteed by the simulator, for the import
task the return value has to be ensured by the DPI application.
Calls to import tasks in SystemVerilog code are indistinguishable from calls to native SystemVerilog tasks.
Likewise, calls to DPI export tasks in SystemVerilog code are indistinguishable from calls to non DPI
SystemVerilog tasks.
If an exported task itself is the target of a disable, its parent imported task is not considered to be in the
disabled state when the exported task returns. In such cases, the exported task shall return value 0, and calls
to svIsDisabledState() shall return 0 as well.
When a DPI imported subroutine returns due to a disable, the values of its output and inout parameters
are undefined. Similarly, function return values are undefined when an imported function returns due to a
disable. C programmers can return values from disabled functions, and C programmers can write values into
the locations of output and inout parameters of imported subroutines. However, SystemVerilog
simulators are not obligated to propagate any such values to the calling SystemVerilog code if a disable is in
effect.

<!-- Page 986 -->

IEEE Std 1800-2023