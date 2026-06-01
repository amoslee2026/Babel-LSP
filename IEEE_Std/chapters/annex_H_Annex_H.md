# Annex H: Annex H

## Annex H
(normative)
DPI C layer
H.1 General
This annex describes the foreign language side of the direct programming interface (DPI).
H.2 Overview
The SystemVerilog DPI allows direct inter-language function calls between SystemVerilog and any foreign
programming language with a C function call protocol and linking model, as follows:
— Functions implemented in C and given import declarations in SystemVerilog can be called from
SystemVerilog; such functions are referred to as imported functions.
— Functions implemented in SystemVerilog and specified in export declarations can be called from C;
such functions are referred to as exported functions.
— Tasks implemented in SystemVerilog and specified in export declarations can be called from C;
such functions are referred to as exported tasks.
— Functions implemented in C that can be called from SystemVerilog and can in turn call exported
tasks; such functions are referred to as imported tasks.
The SystemVerilog DPI supports only SystemVerilog data types, which are the sole data types that can cross
the boundary between SystemVerilog and a foreign language in either direction. On the other hand, the data
types used in C code shall be C types; hence, the duality of types.
A value that is passed through the DPI is specified in SystemVerilog code as a value of SystemVerilog type,
while the same value shall be specified in C code as a value of C type. Therefore, a pair of matching type
definitions is required to pass a value through DPI: the SystemVerilog definition and the C definition.
It is the user’s responsibility to provide these matching definitions. A tool (such as a SystemVerilog
compiler) can facilitate this by generating C type definitions for the SystemVerilog definitions used in DPI
for imported and exported functions.
Some SystemVerilog types are directly compatible with C types; defining a matching C type for them is
straightforward. There are, however, SystemVerilog-specific types, namely packed types (arrays, structures,
and unions), 2-state or 4-state, which have no natural correspondence in C. DPI defines a canonical
representation of 4-state types that is exactly the same as the representation used by the VPI’s avalue/bvalue
representation of 4-state vectors. DPI defines a 2-state representation model that is consistent with the VPI
4-state model. DPI defines library functions to assist users in working with the canonical data representation.
The DPI C interface includes deprecated functions and definitions related to implementation-specific
representation of packed array arguments. These functions are enabled by using the "DPI" specification
string in import and export declarations (see 35.5). Refer to H.14 for details on the deprecated functionality.
Formal arguments in SystemVerilog can be specified as open arrays solely in import declarations; exported
SystemVerilog subroutines cannot have formal arguments specified as open arrays. A formal argument is an
open array when a range of one or more of its dimensions is unspecified (denoted in SystemVerilog by using

<!-- Page 1261 -->

IEEE Std empty square brackets, []). This corresponds to a relaxation of the DPI argument-matching rules (see
35.5.6.1). Actual arguments’ packed dimensions shall collectively match a solitary, unsized formal packed
dimension. Similarly, any actual unpacked dimension shall match a corresponding formal argument
dimension that is unsized. This facilitates writing generalized C code that can handle SystemVerilog arrays
of different sizes.
The C layer of DPI typically uses normalized ranges. The term normalized ranges means [n-1:0] indexing
for the packed part (which may involve linearizing multiple packed dimensions) and means [0:n-1]
indexing for an unpacked dimension. Normalized ranges are used for the canonical representation of packed
arrays in C and for SystemVerilog arrays passed as actual arguments to C. Standard open array query
functions (see H.12.2) return the original, SystemVerilog ranges for unpacked dimensions and return a
linearized, normalized range for the packed dimension.
Function arguments are generally passed by some form of reference or by value. All formal arguments,
except open arrays, are passed by direct reference or value, and, therefore, are directly accessible in C code.
Only small values of SystemVerilog input arguments (see H.8.7) are passed by value. Formal arguments
declared in SystemVerilog as open arrays are passed by a handle (type svOpenArrayHandle) and are
accessible via library functions. Array-querying functions are provided for open arrays.
The C layer of DPI defines a portable binary interface. Once DPI C code is compiled into object code, the
resulting object code shall work without recompilation in any compliant SystemVerilog implementation.
One normative include file, svdpi.h, is provided as part of the DPI C layer. This file defines all basic types,
the canonical 2-state and 4-state data representation, and all interface functions.
H.3 Naming conventions
All names introduced by this interface shall conform to the following conventions:
— All names defined in this interface are prefixed with sv or SV_.
— Function and type names start with sv, followed by initially capitalized words with no separators,
e.g., svLogicVecVal.
— Names of symbolic constants start with sv_, e.g., sv_x.
— Names of macro definitions start with SV_, followed by all uppercase words separated by a
underscore (_), e.g., SV_GET_UNSIGNED_BITS.
H.4 Portability
DPI applications are always portable at the binary level. When compiled on a given platform, DPI object
code shall work with every SystemVerilog simulator on that platform.
H.5 svdpi.h include file
The C layer of the DPI defines include file svdpi.h.
Applications that use the DPI with C code usually need this main include file. The include file svdpi.h
defines the types for canonical representation of 2-state (bit) and 4-state (logic) values and passing
references to SystemVerilog data objects. The file also provides function headers and defines a number of
helper macros and constants.

<!-- Page 1262 -->

IEEE Std The svdpi.h file is fully defined in AnnexI. The content of svdpi.h does not depend on any particular
implementation; all simulators shall use the same file. For more details on svdpi.h, see H.10.1 and
AnnexI.
This file may also contain the deprecated functions and data representations described in H.14. H.14 also
describes the deprecated header svdpi_src.h, which defines the implementation-dependent representation
of packed values.
H.6 Semantic constraints
NOTE—Constraints expressed here merely restate those expressed in 35.5.1.
Formal and actual arguments of both imported subroutines and exported subroutines are bound by the
WYSIWYG principle: What You Specify Is What You Get. This principle is binding both for the caller and
for the callee, in C code and in SystemVerilog code. For the callee, it guarantees the actual arguments are as
specified for the formal ones. For the caller, it means the function call arguments shall conform with the
types of the formal arguments, which might require type-coercion on the caller side.
Another way to state this is that no compiler (either C or SystemVerilog) can make argument coercions
between a caller’s declared formals and the callee’s declared formals. This is because the callee’s formal
arguments are declared in a different language from the caller’s formal arguments; hence there is no visible
relationship between the two sets of formals. Users are expected to understand all argument relationships
and provide properly matched types on both sides of the interface (see H.7.2).
In SystemVerilog code, the compiler can change the formal arguments of a native SystemVerilog subroutine
and modify its code accordingly because of optimizations, compiler pragmas, or command line switches.
The situation is different for imported tasks and functions. A SystemVerilog compiler cannot modify the C
code, perform any coercions, or make any changes whatsoever to the formal arguments of an imported
subroutine.
A SystemVerilog compiler shall provide any necessary coercions for the actual arguments of every imported
subroutine call. For example, a SystemVerilog compiler might truncate or extend bits of a packed array if
the widths of the actual and formal arguments are different.
Similarly, a C compiler can provide coercion for C types based on the relationship of the arguments in an
exported subroutine’s C prototype (formals) and the exported subroutine’s C call site (actuals). However, a
C compiler cannot provide such coercion for SystemVerilog types.
Coercion can be necessary when a SystemVerilog actual argument’s data type is ordinarily accepted by DPI
(H.7.4) and the argument is modified by an optional qualifier (such as rand), which has semantics unrelated
to the type’s representation. If a SystemVerilog compiler associates extra bits with such a data type, it shall
coerce an actual argument of that type to match the unqualified SystemVerilog form that lacks such bits.
When such a qualifier is associated with a DPI import function’s formal arguments, the DPI interface shall
not implement the qualifier’s semantics, shall expect the unqualified form of the type from SystemVerilog,
and shall deliver data for that type in the unqualified form back to SystemVerilog without manipulating any
extra bits associated with the qualified form.
For an inter-language function call between SystemVerilog and C in either direction, the compilers expect,
but cannot enforce, that the types on either side are compatible. Each compiler can coerce data to an
expected form for its side of the inter-language boundary. However, the imported or exported function types
shall match the types of the corresponding subroutines in the foreign language, ignoring the presence of the
kind of qualifiers previously described.

<!-- Page 1263 -->

IEEE Std H.6.1 Types of formal arguments
The WYSIWYG principle verifies the types of formal arguments of imported functions: an actual argument
is required to be of the type specified for the formal argument, with the exception of open arrays (for which
unspecified ranges are statically unknown). Formal arguments, other than open arrays, are fully defined by
imported declaration; they shall have ranges of packed or unpacked arrays exactly as specified in the
imported declaration. Only the SystemVerilog declaration site of the imported function is relevant for such
formal arguments.
Formal arguments defined as open arrays in the C layer are passed by handle (see H.12). Their unpacked
dimensions match those of the corresponding actual argument, while their packed dimension is a linearized,
normalized version of all the actual argument’s packed dimensions. The unsized ranges of open arrays are
determined at a call site; the rest of the type information is specified at the import declaration. See also
H.7.1.
Therefore, if a formal argument is declared as bit [15:8] b [], then the import declaration specifies that
the formal argument is an unpacked array of packed bit array with bounds 15 to 8, while the actual argument
used at a particular call site defines the bounds for the unpacked part for that call.
H.6.2 Input arguments
Formal arguments specified in SystemVerilog as input shall not be modified by the foreign language code.
See also 35.5.1.2.
H.6.3 Output arguments
The initial values of formal arguments specified in SystemVerilog as output are undetermined and
implementation dependent. See also 35.5.1.2.
H.6.4 Value changes for output and inout arguments
The SystemVerilog simulator is responsible for handling value changes for output and inout arguments.
Such changes shall be detected and handled after the control returns from C code to SystemVerilog code.
H.6.5 Context and noncontext tasks and functions
Also refer to 35.5.3.
Some DPI imported subroutines or other interface functions called from them require that the context of
their call be known. It takes special instrumentation of their call instances to provide such context; for
example, a variable referring to the “current instance” might need to be set. To avoid any unnecessary
overhead, imported tasks and function calls in SystemVerilog code are not instrumented unless the imported
tasks or function is specified as context in its SystemVerilog import declaration.
The SystemVerilog context of DPI export subroutines needs to be known when they are called by
SystemVerilog subroutines, or they are called by DPI imports. When an import invokes the svSetScope
utility prior to calling the export, it sets the context explicitly. Otherwise, the context will be the context of
the instantiated scope where the import declaration is located. Because imports with diverse instantiated
scopes can export the same subroutine, multiple instances of such an export can exist after elaboration. Prior
to any invocations of svSetScope, these export instances would have different contexts, which would
reflect their imported caller’s instantiated scope.

<!-- Page 1264 -->

IEEE Std For the sake of simulation performance, a noncontext imported subroutine call shall not block
SystemVerilog compiler optimizations. An imported subroutine not specified as context shall not access any
data objects from SystemVerilog other than its actual arguments. Only the actual arguments can be affected
(read or written) by its call. Therefore, a call of noncontext imported subroutine is not a barrier for
optimizations. A context imported subroutine, however, can access (read or write) any SystemVerilog data
objects by calling VPI or by calling an embedded export subroutine. Therefore, a call to a context subroutine
is a barrier for SystemVerilog compiler optimizations.
Only the calls of context imported tasks and functions are properly instrumented and cause conservative
optimizations; therefore, only those tasks and functions can safely call all functions from other APIs,
including VPI functions or exported SystemVerilog functions. For imported subroutines not specified as
context, the effects of calling VPI or SystemVerilog functions can be unpredictable; and such calls can crash
if the callee requires a context that has not been properly set.
Special DPI utility functions exist that allow imported subroutines to retrieve and operate on their context.
For example, the C implementation of an imported subroutine can use svGetScope() to retrieve an
svScope corresponding to the instance scope of its corresponding SystemVerilog import declaration. See
H.9 for more details.
H.6.6 Memory management
See also 35.5.1.4.
The memory spaces owned and allocated by C code and SystemVerilog code are disjoined. Each side is
responsible for its own allocated memory. Specifically, C code shall not free the memory allocated by
SystemVerilog code (or the SystemVerilog compiler) nor expect SystemVerilog code to free the memory
allocated by C code (or the C compiler). This does not exclude scenarios in which C code allocates a block
of memory and then passes a handle (i.e., a pointer) to that block to SystemVerilog code, which in turn calls
a C function that directly (if it is the standard function free) or indirectly frees that block.
NOTE—In this last scenario, a block of memory is allocated and freed in C code, even when the standard functions
malloc and free are called directly from SystemVerilog code.
H.7 Data types
This subclause defines the data types of the C layer of the DPI.
H.7.1 Limitations
Packed arrays can have an arbitrary number of dimensions although they are eventually always equivalent to
a one-dimensional packed array and treated as such. If the packed part of an array in the type of a formal
argument in SystemVerilog is specified as multidimensional, the SystemVerilog compiler linearizes it.
Although the original ranges are generally preserved for open arrays, if the actual argument has a
multidimensional packed part of the array, it shall be linearized and normalized into an equivalent
one-dimensional packed array. (See H.7.5).
NOTE—The actual argument can have both packed and unpacked parts of an array; either can be multidimensional.

<!-- Page 1265 -->

IEEE Std H.7.2 Duality of types: SystemVerilog types versus C types
A value that crosses the DPI is specified in SystemVerilog code as a value of SystemVerilog type, while the
same value shall be specified in C code as a value of C type. Therefore, each data type that is passed through
the DPI requires two matching type definitions: the SystemVerilog definition and C definition.
The user needs to provide such matching definitions. Specifically, for each SystemVerilog type used in the
import declarations or export declarations in SystemVerilog code, the user shall provide the equivalent type
definition in C reflecting the argument passing mode for the particular type of SystemVerilog value and the
direction (input, output, or inout) of the formal SystemVerilog argument.
H.7.3 Data representation
DPI imposes the following additional restrictions on the representation of SystemVerilog data types:
— SystemVerilog types that are not packed and that do not contain packed elements have C-compatible
representation.
— Basic integer and real data types are represented as defined in H.7.4.
— Packed types, including time, integer and appropriate user-defined types, are represented using
the canonical format defined in H.7.7.
— Enumeration types are represented by C base types that correspond to the enumeration types’
SystemVerilog base types (see TableH.1). integer and time base types are represented as 4-state
packed arrays. The base type determines whether an enumeration type is considered a small value
(see 35.5.5). DPI supports all the SystemVerilog enumeration base types (see 6.19 and A.2.2.1).
Enumerated names are not available on the C side of the interface.
— Unpacked arrays embedded in a structure have C-compatible layout regardless of the type of
elements. Similarly, stand-alone arrays passed as actuals to a sized formal argument have
C-compatible representation.
— For a stand-alone array passed as an actual to an open array formal
• If the element type is a 2- or 4-state scalar or packed type, then the representation is in canonical
form.
• Otherwise, the representation is C compatible. Therefore, an element of an array shall have the
same representation as an individual value of the same type. Hence, an array’s elements can be
accessed from C code via normal C array indexing similarly to doing so for individual values.
— The natural order of elements for each dimension in the layout of an unpacked array shall be used,
i.e., elements with lower indices go first. For SystemVerilog range [L:R], the element with
SystemVerilog index min(L,R) has the C index 0 and the element with SystemVerilog index
max(L,R) has the C index abs(L-R).
H.7.4 Basic types
TableH.1 defines the mapping between the basic SystemVerilog data types and the corresponding C types.
Table H.1—Mapping data types
SystemVerilog type C type
byte char
shortint short int
int int

<!-- Page 1266 -->

IEEE Std Table H.1—Mapping data types (continued)
SystemVerilog type C type
longint long long
real double
shortreal float
chandle void *
string const char *
bita unsigned char
logica/reg unsigned char
aEncodings for bit and logic are given in file svdpi.h. reg
parameters can use the same encodings as logic parameters.
The DPI also supports the SystemVerilog and C unsigned integer data types that correspond to the mappings
TableH.1 shows for their signed equivalents.
The input mode arguments of type byteunsigned and shortintunsigned are not equivalent to
bit[7:0] or bit[15:0], respectively, because the former are passed as C types unsignedchar and
unsignedshort and the latter are both passed by reference as svBitVecVal types. A similar lack of
equivalence applies to passing such parameters by reference for output and inout modes, e.g.,
byte unsigned is passed as C type unsigned char* while bit[7:0] is passed by reference as
svBitVecVal*.
In addition to declaring DPI formal arguments of packed bit and logic arrays, it is also possible to declare
formal arguments of packed struct and union types. DPI handles these types as if they were declared with
equivalent one-dimensional packed array syntax. See 6.22.2. The tag value for both 2- and 4-state packed
unions is a 2-state value, stored in the MSBs of the svBitVecVal canonical form for 2-state packed arrays
and in the most significant aval field bits of the svLogicVecVal canonical form for 4-state packed arrays.
See 7.3.2 for tag values and size, H.7.7 and H.10.1.2 for canonical forms.
Refer to H.7.8 for details on unpacked aggregate types that are composed of the basic types described in this
subclause.
The SystemVerilog rand and randc qualifiers can appear in DPI struct and union formal argument
declarations and can be associated with SystemVerilog actual arguments to DPI imports. In both cases these
qualifiers do not affect processing on the C side and the arguments associated with them are subjected to
DPI type coercion principles (see H.6).
The handling of string types varies depending on the argument passing mode. Refer to H.8.10 for further
details.
H.7.5 Normalized and linearized ranges
Packed arrays are treated as one-dimensional; the unpacked part of an array can have an arbitrary number of
dimensions. Normalized ranges mean [n-1:0] indexing for the packed part and [0:n-1] indexing for a
dimension of the unpacked part of an array. Normalized ranges are used for accessing all array arguments,
except for the unpacked dimensions of open arrays. The canonical representation of packed arrays also uses
normalized ranges.

<!-- Page 1267 -->

IEEE Std Linearizing a SystemVerilog array with multiple packed dimensions consists of treating an array with
dimension sizes (i, j, k) as if it had a single dimension with size (i * j * k) and had been stored as a
one-dimensional array. The one-dimensional array has the same layout as the corresponding
multidimensional array stored in row-major order. User C code can take the original dimensions into
account when referencing a linearized array element. For example, the bit in a SystemVerilog packed 2-state
array with dimension sizes (i, j, k) and a SystemVerilog reference myArray[l][m][n] (where the
ranges for l, m, and n have been normalized) maps to linearized C array
bit (n + (m * k) + (l * j * k)).
H.7.6 Mapping between SystemVerilog ranges and C ranges
The range of a sized dimension in an open array formal argument is specified by the import or export
declaration. Each unsized, unpacked dimension has the same range as the corresponding dimension of the
actual argument. An open array formal argument’s unsized, packed dimension has the linearized,
normalized range of all the actual’s packed dimensions (see H.7.5). Utility functions provide the original
ranges of open array unpacked dimensions and the normalized range of the packed dimension (see H.12.2).
For all types of formal argument other than open arrays, the SystemVerilog ranges are defined in the
corresponding SystemVerilog import or export declaration. Normalized ranges are used for accessing such
arguments in C code. C ranges for multiple packed dimensions are linearized and normalized. The mapping
between SystemVerilog ranges and C ranges is defined as follows:
a) If a packed part of an array has more than one dimension, it is linearized as specified by the
equivalence of packed types (see H.7.5 and 6.22.2).
b) A packed array of range [L:R] is normalized as [abs(L-R):0]; its MSB has a normalized index
abs(L-R) and its LSB has a normalized index 0.
c) The natural order of elements for each dimension in the layout of an unpacked array shall be used,
i.e., elements with lower indices go first. For SystemVerilog range [L:R], the element with
SystemVerilog index min(L,R) has the C index 0 and the element with SystemVerilog index
max(L,R) has the C index abs(L-R).
The above range mapping from SystemVerilog to C applies to calls made in both directions, i.e.,
SystemVerilog calls to C and C calls to SystemVerilog.
For example, if logic[2:3][1:3][2:0]b[1:10][31:0] is used in SystemVerilog, it needs to be
defined in C as if it were declared in SystemVerilog in the following normalized form: logic[17:0]b
[0:9][0:31].
H.7.7 Canonical representation of packed arrays
The DPI defines the canonical representation of packed 2-state (type svBitVecVal) and 4-state arrays (type
svLogicVecVal). svLogicVecVal is fully equivalent to type s_vpi_vecval, which is used to represent
4-state logic in VPI.
A packed array is represented as an array of one or more elements (of type svBitVecVal for 2-state values
and svLogicVecVal for 4-state values), each element representing a group of 32 bits. The first element of
an array contains the 32 LSBs, next element contains the 32 more significant bits, and so on. The last
element can contain a number of unused bits. The contents of these unused bits are undetermined, and the
user is responsible for the masking or the sign extension (depending on the sign) for the unused bits.

<!-- Page 1268 -->

IEEE Std H.7.8 Unpacked aggregate arguments
Imported and exported DPI tasks and functions can make use of unpacked aggregate types as formal or
actual arguments. Aggregate types include unpacked arrays and structures. Such types can be composed of
packed elements, unpacked elements, or combinations of either kind of element, including subaggregates.
Refer to TableH.1 for a list of legal basic types that can be used as nonaggregate elements in aggregate
types. Also refer to 35.5.6.
In the case of an unpacked type that consists purely of unpacked elements (including subaggregates), the
layout presented to the C programmer is guaranteed to be compatible with the C compiler’s layout on the
given operating system. It is also possible for unpacked aggregate types to include packed elements.
H.8 Argument passing modes
This subclause defines the ways to pass arguments in the C layer of the DPI.
H.8.1 Overview
Imported and exported function arguments are generally passed by some form of a reference, with the
exception of small values of SystemVerilog input arguments (see H.8.7), which are passed by value.
Similarly, the function result, which is restricted to small values, is passed by value, i.e., directly returned.
Formal arguments, except open arrays, are passed by direct reference or value and, therefore, are directly
accessible in C code. Formal arguments declared in SystemVerilog as open arrays are passed by a handle
(type svOpenArrayHandle) and are accessible via library functions.
H.8.2 Calling SystemVerilog tasks and functions from C
There is no difference in argument passing between calls from SystemVerilog to C and calls from C to
SystemVerilog. Tasks and functions exported from SystemVerilog cannot have open arrays as arguments.
Apart from this restriction, the same types of formal arguments can be declared in SystemVerilog for
exported tasks and functions and imported tasks and functions. A subroutine exported from SystemVerilog
shall have the same function header in C as would an imported function with the same function result type
and same formal argument list. In the case of arguments passed by reference, an actual argument to
SystemVerilog subroutine called from C shall be allocated using the same layout of data as SystemVerilog
uses for that type of argument; the caller is responsible for the allocation. It can be done while preserving the
binary compatibility (see H.12.5 and H.14).
Calling a SystemVerilog task from C is the same as calling a SystemVerilog function from C with the
exception that the return type of an exported task is an int value that has a special meaning related to
disable statements. See 35.9 for details on disable processing by DPI imported tasks and functions.
H.8.3 Argument passing by value
Only small values of formal input arguments (see H.8.7) are passed by value. Function results are also
directly passed by value. The user needs to provide the C type equivalent to the SystemVerilog type of a
formal argument if an argument is passed by value.

<!-- Page 1269 -->

IEEE Std H.8.4 Argument passing by reference
For arguments passed by reference, a reference (a pointer) to the actual data object is passed. In the case of
packed data, a reference to a canonical data object is passed. The actual argument is usually allocated by a
caller. The caller can also pass a reference to an object already allocated somewhere else, for example, its
own formal argument passed by reference.
If an argument of type T is passed by reference, the formal argument shall be of type T*. Packed arrays are
passed using a pointer to the appropriate canonical type definition, either svLogicVecVal* or
svBitVecVal*.
There shall be no assumptions made in DPI C applications about the lifetime of pass-by-reference
arguments. If it is required to store a pass-by-reference argument’s value across multiple DPI calls, then the
value needs to be copied into memory owned and managed by the C application.
H.8.5 Allocating actual arguments for SystemVerilog-specific types
This is relevant only for calling exported SystemVerilog subroutines from C code. The caller is responsible
for allocating any actual arguments that are passed by reference.
Static allocation requires knowledge of the relevant data type. If such a type involves SystemVerilog packed
arrays, corresponding C arrays of canonical data types (either svLogicVecVal or svBitVecVal) need to
be allocated and initialized before being passed by reference to the exported SystemVerilog subroutine.
H.8.6 Argument passing by handle—open arrays
Arguments specified as open (unsized) arrays are always passed by a handle, regardless of the direction of
the SystemVerilog formal argument, and are accessible via library functions. The actual implementation of a
handle is tool-specific and transparent to the user. A handle is represented by the generic pointer void
*(typedefed to svOpenArrayHandle). Arguments passed by handle shall always have a const qualifier
because the user shall not modify the contents of a handle.
H.8.7 Input arguments
input arguments of imported functions implemented in C shall always have a const qualifier.
input arguments, with the exception of open arrays, are passed by value or by reference, depending on the
size. Small values of formal input arguments are passed by value. The following data types are considered
small:
— byte, shortint, int, longint, real, shortreal
— Scalar bit and logic
— chandle, string
input arguments of other types are passed by reference.
H.8.8 Inout and output arguments
inout and output arguments, with the exception of open arrays, are always passed by reference.
Specifically, packed arrays are passed, accordingly, as svBitVecVal* or svLogicVecVal*. The same
rules about unused bits apply as in H.7.7.

<!-- Page 1270 -->

IEEE Std H.8.9 Function result
Types of a function result are restricted to the following SystemVerilog data types (see TableH.1 for the
corresponding C types):
— byte, shortint, int, longint, real, shortreal, chandle, string
— Scalar values of type bit and logic
Encodings for bit and logic are given in file svdpi.h. Refer to H.10.1.1.
H.8.10 String arguments
The layout of SystemVerilog string objects is implementation dependent. However, when a string value is
passed from SystemVerilog to C, implementations shall lay out all characters in memory per C string
conventions, including a trailing null character present at the end of the C string. Similarly, users shall make
sure that any C strings passed to SystemVerilog are properly null-terminated.
The direction mode for string arguments applies to the pointer to the string (i.e., the const char* variable
in TableH.1), not to the characters that compose the string.
Thus, the direction modes have the following meanings for imported tasks and functions:
— An input mode string is accessed through a pointer value that is provided by SystemVerilog and
that shall not be freed by the DPI C application. There shall be no assumptions made in the C
application about the lifetime of this string storage. No user changes to this pointer value are
propagated back to the SystemVerilog sphere.
— An output mode string does not arrive at the C interface with a meaningful value. It is represented
by a const char** variable. Upon return to SystemVerilog, the DPI C application shall have writ-
ten a valid and initialized const char* address into the const char** variable. SystemVerilog
shall not free memory accessed through this address.
— An inout mode string arrives at the C interface with a valid string address value stored in a const
char** variable. The string’s storage shall not be freed by the DPI C application. There shall be no
assumptions made in the C application about the lifetime of the string storage. Any changes to the
string shall be effected by the C application providing a new pointer value, which points to new
string contents and which SystemVerilog shall not attempt to free. The C application provides a new
string pointer value by writing the string’s address into the const char** variable. If the pointer
value is modified by the C application, SystemVerilog copies the indicated string contents into its
memory space and undertakes any actions sensitive to this change.
The direction modes have the following meanings for exported tasks and functions:
— An input mode string is passed to SystemVerilog through a const char* pointer. SystemVerilog
only reads from the string. It shall not modify the characters that compose the string.
— An output mode string is represented by a const char** variable. No meaningful initial value is
stored in the pointer variable. SystemVerilog shall write a valid string address into the output const
char** variable. The user shall not make any assumptions about the lifetime of the output string’s
storage, and the C code shall not free the string memory. If it is desired to refer to the string’s value
at some point in the future, the user shall copy the string value to memory owned by the C domain.
— An inout mode string is represented by a const char** variable that contains a pointer to
memory allocated and initialized by the user. SystemVerilog only reads from the user’s string
storage, and it will not attempt to modify or free this storage. If SystemVerilog needs to effect a
change in the value of the inout mode string, then a valid SystemVerilog string address is written
into the const char** variable. The user shall not make any assumptions about the lifetime of this
string storage, nor should the SystemVerilog storage be freed by C code. If it is desired to refer to
the modified string value at some point in the future, the user shall copy the string value to memory
owned by the C domain.

<!-- Page 1271 -->

IEEE Std H.8.10.1 String types in aggregate arguments
When strings are contained in aggregate arguments, those string members shall also be represented by
const char* variables. All the same stipulations apply to string members of aggregate arguments as apply
to stand-alone string arguments.
NOTE—With arrays of string arguments, there is no need for the extra level of indirection that occurs with stand-alone
string output and inout arguments. By the rules specified in H.7.8, all arrays of string arguments are represented in C as
const char**, regardless of their directionality.
H.9 Context tasks and functions
Some DPI imported tasks and functions require that the context of their call be known. For example, those
calls can be associated with instances of C models that have a one-to-one correspondence with instances of
SystemVerilog modules that are making the calls. Alternatively, a DPI imported subroutine might need to
access or modify simulator data structures using VPI calls or by making a call back into SystemVerilog via
an export subroutine. Context knowledge is required for such calls to function properly. It can take special
instrumentation of their call to provide such context.
To avoid any unnecessary overhead, imported subroutine calls in SystemVerilog code are not instrumented
unless the imported subroutine is specified as context in its SystemVerilog import declaration. A DPI-C
context call chain is a sequence of C subroutine invocations that starts with a SystemVerilog entity calling a
DPI-C import declared with the context keyword and continues in C, unbroken by a call back into
SystemVerilog. A small set of DPI utility functions is available to assist programmers when working with
context subroutines (see H.9.3). The behavior of DPI utility functions that manipulate context is undefined
when they are invoked by any subroutine that is not part of a DPI context call chain (see 35.5.3). Similarly,
the behavior of exported subroutines is undefined when they are invoked by a DPI call chain that lacks the
context characteristic.
H.9.1 Overview of DPI and VPI context
Both DPI subroutines and VPI functions might need to understand their context. However, the meaning of
the term is different for the two categories of subroutines.
DPI imported tasks and functions are essentially proxies for native SystemVerilog tasks and functions.
Native SystemVerilog tasks and functions always operate in the scope of their declaration site. For example,
a native SystemVerilog function f() can be declared in a module m, which is instantiated as top.i1_m. The
top.i1_m instance of f() can be called via hierarchical reference from code in a distant design region.
Function f() is said to execute in the context (i.e., instantiated scope) of top.i1_m because it has
unqualified visibility only for variables local to that specific instance of m. Function f() does not have
unqualified visibility for any variables in the calling code’s scope.
DPI imported tasks and functions follow the same model as native SystemVerilog tasks and functions. They
execute in the context of their surrounding declarative scope, rather than the context of their call sites. This
type of context is termed DPI context.
This is in contrast to VPI functions. Such functions execute in a context associated with their call sites. The
VPI programming model relies on C code’s ability to retrieve a context handle associated with the
associated system task’s call site and then to work with the context handle to glean information about
arguments, items in the call site’s surrounding declarative scope, etc. This type of context is termed VPI
context.

<!-- Page 1272 -->

IEEE Std The SystemVerilog context of DPI export tasks and functions needs to be known when they are called,
including when they are called by imports. When an import invokes the svSetScope utility prior to calling
the export, it sets the context explicitly. Otherwise, the context will be the context of the instantiated scope
where the import declaration is located. Because imports with diverse instantiated scopes can export the
same subroutine, multiple instances of such an export can exist after elaboration. Prior to any invocations of
svSetScope, these export instances would have different contexts, which would reflect their imported
caller’s instantiated scope.
H.9.2 Context of imported and exported tasks and functions
DPI imported and exported tasks and functions can be declared in a module, program, interface,
package, compilation-unit scope, or generate declarative scope.
A context imported subroutine executes in the context of the instantiated scope surrounding its declaration.
In other words, such tasks and functions can see other variables in that scope without qualification. As
explained in H.9.1, this should not be confused with the context of the task’s or function’s call site, which
can actually be anywhere in the SystemVerilog design hierarchy. The context of an imported or exported
subroutine corresponds to the fully qualified name of the subroutine, minus the subroutine name itself.
The context property is transitive through imported and exported context tasks and functions declared in the
same scope. In other words, if an imported subroutine is running in a certain context and if it in turn calls an
exported subroutine that is available in the same context, the exported subroutine can be called without any
use of svSetScope(). For example, consider a SystemVerilog call to a native function f(), which in turn
calls a native function g(). Now replace the native function f() with an equivalent imported context C
function, f'(). The system shall behave identically regardless if f() or f'() is in the call chain above g().
g() has the proper execution context in both cases.
When control passes across the boundary between SystemVerilog and a DPI import call chain with the
context property, the value of the import’s context is potentially either set or reset (see 35.5.3). Therefore,
user code behavior is undefined for DPI import C code that circumvents SystemVerilog exports unwinding
across the boundary to their import caller (e.g., by using C setjmp and longjmp constructs).
H.9.3 Working with DPI context tasks and functions in C code
DPI defines a small set of functions to help programmers work with DPI context tasks and functions. The
term scope is used in the subroutine names for consistency with other SystemVerilog terminology. The
terms scope and context are equivalent for DPI tasks and functions. A DPI context imported subroutine is
declared with the context keyword. A DPI-C context call chain is a sequence of calls to C subroutines that
begins with a SystemVerilog entity calling a DPI context import and continues in C, unbroken by a call back
into SystemVerilog.
There are functions that allow the user to retrieve and manipulate the current operational scope. The
behavior of these functions is undefined if they are invoked by an entity other than a member of a DPI
context call chain. The behavior of exported subroutines is undefined when they are invoked by a member of
a DPI call chain that lacks the context characteristic.
There are also functions that provide users with the power to set data specific to C models into the
SystemVerilog simulator for later retrieval. These are the “put” and “get” user data functions, which are
similar to facilities provided in VPI.
The put and get user data functions are flexible and allow for a number of use models. Users might wish to
share user data across multiple context imported functions defined in the same SystemVerilog scope. Users

<!-- Page 1273 -->

IEEE Std might wish to have unique data storage on a per-function basis. Shared or unique data storage is controllable
by a user-defined key.
To achieve shared data storage, a related set of context imported tasks and functions should all use the same
user key. To achieve unique data storage, a context import subroutine should use a unique key, and it is a
requirement on the user that such a key be truly unique from all other keys that could possibly be used by C
code. This includes completely unknown C code that could be running in the same simulation. It is
suggested that taking addresses of static C symbols (such as a function pointer or an address of some static C
data) always be done for user key generation. Generating keys based on arbitrary integers is not a safe
practice.
It is never possible to share user data storage across different contexts. For example, if a SystemVerilog
module m declares a context imported subroutine f, and m is instantiated more than once in the
SystemVerilog design, then f shall execute under different values of svScope. No such executing instances
of f can share user data with each other, at least not using the system-provided user data storage area
accessible via svPutUserData().
A user wanting to share a data area across multiple contexts has to do so by allocating the common data area
and then storing the pointer to it individually for each of the contexts in question via multiple calls to
svPutUserData(). This is because, although a common user key can be used, the data needs to be
associated with the individual scopes (denoted by svScope) of those contexts.
/* Functions for working with DPI context functions */
/* Retrieve the active instance scope currently associated with the executing
* imported function.
* Unless a prior call to svSetScope has occurred, this is the scope of the
* function's declaration site, not call site.
* The return value is undefined if this function is invoked from a noncontext
* imported function.
*/
svScope svGetScope();
/* Set context for subsequent export function execution.
* This function shall be called before calling an export function, unless
* the export function is called while executing an import function. In that
* case the export function shall inherit the scope of the surrounding import
* function. This is known as the “default scope”.
* The return is the previous active scope (per svGetScope)
*/
svScope svSetScope(const svScope scope);
/* Gets the fully qualified name of a scope handle */
const char* svGetNameFromScope(const svScope);
/* Retrieve svScope to instance scope of an arbitrary function declaration.
* (can be either module, program, interface, or generate scope)
* The return value shall be NULL for unrecognized scope names.
*/
svScope svGetScopeFromName(const char* scopeName);
/* Store an arbitrary user data pointer for later retrieval by svGetUserData()
* The userKey is generated by the user. It needs to be guaranteed by the user to
* be unique from all other userKey's for all unique data storage requirements
* It is recommended that the address of static functions or variables in the
* user's C code be used as the userKey.
* It is illegal to pass in NULL values for either the scope or userData
* arguments. It is also an error to call svPutUserData() with an invalid

<!-- Page 1274 -->

IEEE Std * svScope. This function returns -1 for all error cases, 0 upon success. It is
* suggested that userData values of 0 (NULL) not be used as otherwise it can
* be impossible to discern error status returns when calling svGetUserData()
*/
int svPutUserData(const svScope scope, void *userKey, void* userData);
/* Retrieve an arbitrary user data pointer that was previously
* stored by a call to svPutUserData(). See the comment above
* svPutUserData() for an explanation of userKey, as well as
* restrictions on NULL and illegal svScope and userKey values.
* This function returns NULL for all error cases, and a non-Null
* user data pointer upon success.
* This function also returns NULL in the event that a prior call
* to svPutUserData() was never made.
*/
void* svGetUserData(const svScope scope, void* userKey);
/* Returns the file and line number in the SV code from which the import call
* was made. If this information available, returns TRUE and updates fileName
* and lineNumber to the appropriate values. Behavior is unpredictable if
* fileName or lineNumber are not appropriate pointers. If this information is
* not available return FALSE and contents of fileName and lineNumber not
* modified. Whether this information is available or not is implementation-
* specific. Note that the string provided (if any) is owned by the SV
* implementation and is valid only until the next call to any SV function.
* Applications shall not modify this string or free it.
*/
int svGetCallerInfo(const char **fileName, int *lineNumber);
H.9.4 Example 1—Using DPI context functions
SV side:
// Declare an imported context sensitive C function with cname "MyCFunc"
import "DPI-C" context MyCFunc = function integer MapID(int portID);
C side:
// Define the function and model class on the C++ side:
class MyCModel {
private:
int locallyMapped(int portID); // Does something interesting...
public:
// Constructor
MyCModel(const char* instancePath) {
svScope svScope = svGetScopeByName(instancePath);
// Associate "this" with the corresponding SystemVerilog scope
// for fast retrieval during run time.
svPutUserData(svScope, (void*) MyCFunc, this);
}
friend int MyCFunc(int portID);
};
// Implementation of imported context function callable in SV
int MyCFunc(int portID) {

<!-- Page 1275 -->

IEEE Std // Retrieve SV instance scope (i.e., this function's context).
svScope = svGetScope();
// Retrieve and make use of user data stored in SV scope
MyCModel* me = (MyCModel*)svGetUserData(svScope, (void*) MyCFunc);
return me->locallyMapped(portID);
}
H.9.5 Relationship between DPI and VPI
DPI allows C code to run in the context of a SystemVerilog simulation; thus it is natural for users to consider
using VPI C code from within imported tasks and functions.
There is no specific relationship defined between DPI and VPI. Programmers may make no assumptions
about how DPI and the other interfaces interact. For example, a vpiHandle is not equivalent to an
svOpenArrayHandle, and the two may not be interchanged and passed between functions defined in the
two different interfaces.
If a user wants to call VPI functions from within an imported subroutine, the imported subroutine shall be
flagged with the context qualifier, with the following exceptions (see Table36-9):
— The vpi_printf, vpi_vprintf, and vpi_flush I/O routines
— The vpi_mcd_open, vpi_mcd_close, vpi_mcd_name, vpi_mcd_printf, vpi_mcd_vprintf, and
vpi_mcd_flush I/O routines
— The vpi_get_vlog_info utility routine
These methods do not require the context qualifier, as they do not access the Verilog model and therefore
do not require additional instrumentation or conservative optimizations (see 35.5.3).
Not all VPI functionality is available from within DPI context imported tasks and functions. For example, a
SystemVerilog imported subroutine is not a system task, and thus making the following call from within an
imported subroutine would result in an error:
/* Get handle to system task call site in preparation for argument scan */
vpiHandle myHandle = vpi_handle(vpiSysTfCall, NULL);
Similarly, callbacks and other activities associated with system tasks are not supported inside DPI imported
tasks and functions. Users should use VPI if they wish to accomplish such actions.
However, the following kind of code will work reliably from within DPI context imported tasks and
functions:
/* Prepare to scan all top-level modules */
vpiHandle myHandle = vpi_iterate(vpiModule, NULL);
H.10 Include files
The C layer of the DPI defines one include file, svdpi.h. This file is implementation independent and
defines the canonical representation, all basic types, and all interface functions. The actual file is shown in
AnnexI.
H.10.1 Include file svdpi.h

<!-- Page 1276 -->

IEEE Std Applications that use the DPI with C code usually need this main include file. The include file svdpi.h
defines the types for canonical representation of 2-state (bit) and 4-state (logic) values and passing
references to SystemVerilog data objects, provides function headers, and defines a number of helper macros
and constants.
This standard fully defines the svdpi.h file. The content of svdpi.h does not depend on any particular
implementation or platform; all simulators shall use the same file. Subclauses H.10.1.1, H.10.1.2, and
H.10.1.3 (and H.14) detail the contents of the svdpi.h file.
H.10.1.1 Scalars of type bit and logic
/* canonical representation */
#define sv_0 0
#define sv_1 1
#define sv_z 2 /* representation of 4-st scalar z */
#define sv_x 3 /* representation of 4-st scalar x */
/* common type for 'bit' and 'logic' scalars. */
typedef unsigned char svScalar;
typedef svScalar svBit; /* scalar */
typedef svScalar svLogic; /* scalar */
H.10.1.2 Canonical representation of packed arrays
/*
* DPI representation of packed arrays.
* 2-state and 4-state vectors, exactly the same as PLI's avalue/bvalue.
*/
#ifndef VPI_VECVAL
#define VPI_VECVAL
typedef struct t_vpi_vecval {
uint32_t aval;
uint32_t bval;
} s_vpi_vecval, *p_vpi_vecval;
#endif
/* (a chunk of) packed logic array */
typedef s_vpi_vecval svLogicVecVal;
/* (a chunk of) packed bit array */
typedef uint32_t svBitVecVal;
/* Number of chunks required to represent the given width packed array */
#define SV_PACKED_DATA_NELEMS(WIDTH) (((WIDTH) + 31) >> 5)
/*
* Because the contents of the unused bits is undetermined,
* the following macros can be handy.
*/
#define SV_MASK(N) (~(-1 << (N)))
#define SV_GET_UNSIGNED_BITS(VALUE, N) \
((N) == 32 ? (VALUE) : ((VALUE) & SV_MASK(N)))
#define SV_GET_SIGNED_BITS(VALUE, N) \
((N) == 32 ? (VALUE) : \
(((VALUE) & (1 << (N))) ? ((VALUE) | ~SV_MASK(N)) : ((VALUE) & SV_MASK(N))))

<!-- Page 1277 -->

IEEE Std H.10.1.3 Implementation-dependent representation
The svDpiVersion() function returns a string indicating which DPI standard is supported by the simulator
and in particular which canonical value representation is being provided. For example, a tool that is based on
IEEE Std 1800-2005, i.e., the VPI-based canonical value, shall return the string "1800-2005". Simulators
implementing to the prior Accellera SV3.1a standard [B4], and thus using the svLogicVec32 value
representation, shall return the string "SV3.1a".
/* Returns either version string "1800-2005" or "SV3.1a" */
const char* svDpiVersion();
/* a handle to a scope (an instance of a module or an interface) */
typedef void *svScope;
/* a handle to a generic object (actually, unsized array) */
typedef void* svOpenArrayHandle;
H.10.2 Example 2—Simple packed array application
SystemVerilog:
typedef struct {int x; int y;} pair;
import "DPI-C" function void f1(input int i1, pair i2,
output logic [63:0] o3);
export "DPI-C" function exported_sv_func;
function void exported_sv_func(input int i, output int o [0:7]);
begin ... end
endfunction
C:
#include "svdpi.h"
typedef struct {int x; int y;} pair;
extern void exported_sv_func(int, int *); /* imported from SystemVerilog */
void f1(const int i1, const pair *i2, svLogicVecVal* o3)
{
int tab[8];
printf("%d\n", i1);
o3[0].aval = i2->x;
o3[0].bval = 0;
o3[1].aval = i2->y;
o3[1].b = 0;
/* call SystemVerilog */
exported_sv_func(i1, tab); /* tab passed by reference */
...
}
H.10.3 Example 3—Application with complex mix of types
SystemVerilog:

<!-- Page 1278 -->

IEEE Std typedef struct {int a; bit [6:1][1:8] b [65:2]; int c;} triple;
// troublesome mix of C types and packed arrays
import "DPI-C" function void f1(input triple t);
export "DPI-C" function exported_sv_func;
function void exported_sv_func(input int i, output logic [63:0] o);
begin ... end
endfunction
C:
#include "svdpi.h"
typedef struct {
int a;
svBitVecVal b[64][SV_PACKED_DATA_NELEMS(6*8)];
int c;
} triple;
/* Note that 'b' is defined as for 'bit [6*8-1:0] b [63:0]' */
extern void exported_sv_func(int, svLogicVecVal*); /* imported from
SystemVerilog */
void f1(const triple *t)
{
int i;
svBitVecVal aB;
svLogicVecVal aL[SV_PACKED_DATA_NELEMS(64)];
/* aB holds results of part-select from packed bit array 'b' in
struct triple. */
/* aL holds the packed logic array filled in by the export function. */
printf("%d %d\n", t->a, t->c);
for (i = 0; i < 64; i++) {
/* Read least significant byte of each word of b into aB, then
process... */
svGetPartselBit(&aB, t->b[i], 0, 8);
...
}
...
/* Call SystemVerilog */
exported_sv_func(2, aL); /* Export function writes data into
output arg "aL" */
...
}
H.11 Arrays
Normalized ranges are used for accessing SystemVerilog arrays, with the exception of formal arguments
specified as open arrays.

<!-- Page 1279 -->

IEEE Std H.11.1 Example 4—Using packed 2-state arguments
This example shows two alternatives for working with 2-state packed data types. The first argument shows
classical int-to-int correspondence per TableH.1. The second argument demonstrates that a DPI formal
argument can be of a C-compatible type and that arbitrary 2-state bit vector actual arguments can be
associated with that C-compatible formal argument. The third argument shows a portable technique for
handling an arbitrary width 2-state vector. This technique is less efficient than techniques involving
C-compatible formal arguments, but it is required when 2-state vectors exceed 64 bits in length.
// SV code
module m;
parameter W = 33;
int abv1;
bit [29:0] abv2;
bit [W-1:0] abv3;
// Two ways of handling 2-state packed array arguments
import "DPI-C" function void f7 (input int unsigned fbv1,
input int unsigned fbv2,
input [W-1:0] fbv3);
initial
f7(abv1, abv2, abv3);
endmodule
/* C code */
void f7(unsigned int fbv1, unsigned int fbv2,
const svBitVecVal* fbv3)
{
printf("fbv1 is %d, fbv2 is %d\n", fbv1, fbv2);
/* Use of the 2-state svdpi utilities is needed to transform fbv3 into a
C representation */
}
H.11.2 Multidimensional arrays
Multiple packed dimensions of a SystemVerilog array are linearized (see H.7.5). Unpacked arrays can have
an arbitrary number of dimensions.
H.11.3 Example 5—Using packed struct and union arguments
This example shows how packed struct and union arguments correspond to one-dimensional packed
array arguments.
// SV code
module m;
typedef bit [2:0] A;
typedef struct packed { bit a; bit b; bit c; } S;
typedef union packed { A a; S s; } U;
S s;
U u;
A a;

<!-- Page 1280 -->

IEEE Std // Import function takes three arguments
import "DPI-C" function void f8(input A fa, input S fs, input U fu);
initial begin
s.a = 1'b1;
s.b = 1'b0;
s.c = 1'b0;
a = 3'b100;
u.a = 3'b100;
f8(a, s, u);
end
endmodule
/* C code */
void f8(
const svBitVecVal* fa,
const svBitVecVal* fs,
const svBitVecVal* fu)
{
printf("fa is %d, fs is %d, fu is %d\n", *fa, *fs, *fu);
}
The output of the printf will be “fa is 4, fs is 4, fu is 4”.
H.11.4 Direct access to unpacked arrays
Unpacked arrays, with the exception of formal arguments specified as open arrays, shall have the same
layout as used by a C compiler; they are accessed using C indexing (see H.7.6).
H.11.5 Utility functions for working with the canonical representation
Packed arrays are accessible via canonical representation. This C layer interface provides utility functions
for working with bit-selects and limited (up to 32-bit) part-selects in the canonical representation.
A part-select is a slice of a packed array of types bit or logic. Array slices are not supported for unpacked
arrays. Functions for part-selects only allow access (read/write) to a narrow subrange of up to 32 bits. If the
specified range of a part-select is not fully contained within the normalized range of an array, the behavior is
undetermined.
DPI utilities behave in the following way, given part-select arguments of width w and starting index i. A
utility puts part-select source bits [w-1:0] into destination bits [(i+w-1):i] without changing the values
of destination bits that surround the part-select. A utility gets part-select source bits [(i+w-1):i] and
copies them into destination bits [w-1:0]. If w < 32, destination bits [31:w] shall be left unchanged by
the get part-select operation.
/*
* Bit-select utility functions.
*
* Packed arrays are assumed to be indexed n-1:0,
* where 0 is the index of LSB
*/
/* s=source, i=bit-index */
svBit svGetBitselBit(const svBitVecVal* s, int i);
svLogic svGetBitselLogic(const svLogicVecVal* s, int i);

<!-- Page 1281 -->

IEEE Std /* d=destination, i=bit-index, s=scalar */
void svPutBitselBit(svBitVecVal* d, int i, svBit s);
void svPutBitselLogic(svLogicVecVal* d, int i, svLogic s);
/*
* Part-select utility functions.
*
* A narrow (<=32 bits) part-select is extracted from the
* source representation and written into the destination word.
*
* Normalized ranges and indexing [n-1:0] are used for both arrays.
*
* s=source, d=destination, i=starting bit index, w=width
* like for variable part-selects; limitations: w <= 32
*/
void svGetPartselBit(svBitVecVal* d, const svBitVecVal* s, int i, int w);
void svGetPartselLogic(svLogicVecVal* d, const svLogicVecVal* s, int i, int w);
void svPutPartselBit(svBitVecVal* d, const svBitVecVal s, int i, int w);
void svPutPartselLogic(svLogicVecVal* d, const svLogicVecVal s, int i, int w);
H.12 Open arrays
Formal arguments specified as open arrays allows passing actual arguments of different sizes (i.e., different
range and/or different number of elements), which facilitates writing more general C code that can handle
SystemVerilog arrays of different sizes. The elements of an open array can be accessed in C by using the
same range of indices and the same indexing as in SystemVerilog. Plus, inquiries about the dimensions and
the original boundaries of SystemVerilog actual arguments are supported for open arrays.
Both the sole packed dimension (see H.7.1) and multiple unpacked dimensions can be unsized (see
35.5.6.1).
All formal arguments declared in SystemVerilog as open arrays are passed by handle
(type svOpenArrayHandle), regardless of the direction of a SystemVerilog formal argument. Such
arguments are accessible via interface functions that accept the handle. For example, the array address is
provided by a call to svGetArrayPtr.
For inout or output mode open array arguments the space available for user C code output is determined by
the actual argument’s size. The result of user C code writing more data to an open array address than the
actual argument’s capacity can accommodate is undefined.
H.12.1 Actual ranges
Formal arguments defined as open arrays have sizes and ranges determined by the actual argument on a per-
call basis. The programmer shall always have a choice about whether to specify a formal argument as a sized
array or as an open (unsized) array.
For sized formal array dimensions, all indices are normalized on the C side (i.e., 0 and up); the programmer
needs to know the size of an array and be capable of determining how the ranges of the actual argument map
onto C-style ranges (see H.7.6).
Tip: Programmers can decide to use [n:0]name[0:k] style ranges in SystemVerilog.

<!-- Page 1282 -->

IEEE Std For unsized, unpacked formal array dimensions, the actual argument’s original range and indices are
available via query functions (see H.12.2). For unsized, packed formal array dimensions, the query functions
provide a linearized, normalized form of the actual’s packed dimensions. Thus, the actual argument’s
original indices can be retrieved from query functions and used as arguments to copying and access
functions (see H.12.4 and H.12.5). Similarly, the normalized indices of the actual argument’s packed
dimensions can be retrieved and used with the standard functions for accessing packed array canonical
representations (see H.11.5).
If a formal argument is specified as a sized array, then it shall be passed by reference, with no overhead, and
is directly accessible as a normalized array. If a formal argument is specified as an open (unsized) array, then
it shall be passed by handle, with some overhead, and is mostly indirectly accessible, again with some
overhead.
NOTE—This provides some degree of flexibility and allows the programmer to control the trade-off of performance
versus convenience.
The following example shows the use of sized versus unsized arrays in SystemVerilog code:
// both unpacked arrays are 64 by 8 elements, packed 16-bit each
logic [15: 0] a_64x8 [63:0][7:0];
logic [31:16] b_64x8 [64:1][-1:-8];
import "DPI-C" function void f1(input logic [] i [][]);
// 2-dimensional unsized unpacked array of unsized packed logic
import "DPI-C" function void f2(input logic [31:16] i [64:1][-1:-8]);
// 2-dimensional sized unpacked array of sized packed logic
f1(a_64x8);
f1(b_64x8); // C code can use normalized packed and original unpacked
// ranges [15:0][64:1][-1:-8]
f2(b_64x8); // C code must use normalized ranges [15:0][0:63][0:7]
H.12.2 Array querying functions
These functions are modeled upon the SystemVerilog array querying functions and use the same semantics
(see 20.7).
If the dimension is 0, then the query refers to the packed part (which is one-dimensional) of an array, and
dimensions > 0 refer to the unpacked part of an array.
/* h= handle to open array, d=dimension */
int svLeft(const svOpenArrayHandle h, int d);
int svRight(const svOpenArrayHandle h, int d);
int svLow(const svOpenArrayHandle h, int d);
int svHigh(const svOpenArrayHandle h, int d);
int svIncrement(const svOpenArrayHandle h, int d);
int svSize(const svOpenArrayHandle h, int d);
int svDimensions(const svOpenArrayHandle h);
H.12.3 Access functions
There are library functions available for copying data between open array handles and canonical form
buffers provided by the C programmer. Likewise, there are functions to obtain the actual address of
SystemVerilog data objects or of an individual element of an unpacked array.

<!-- Page 1283 -->

IEEE Std Depending on the type of an element of an unpacked array, different access methods shall be used when
working with elements, as follows:
— Packed arrays ( bit or logic) are accessed via copying to or from the canonical representation.
— Scalars (1-bit value of type bit or logic) are accessed (read or written) directly.
— Other types of values (e.g., structures) are accessed via generic pointers; a library function calculates
an address, and the user needs to provide the appropriate casting.
— Scalars and packed arrays are accessible via pointers only if the implementation supports this
functionality (per array), e.g., one array can be represented in a form that allows such access, while
another array might use a compacted representation that renders this functionality unfeasible (both
occurring within the same simulator).
SystemVerilog allows arbitrary dimensions and, hence, an arbitrary number of indices. To facilitate this,
variable argument list functions shall be used. For the sake of performance, specialized versions of all
indexing functions are provided for one, two, or three indices.
H.12.4 Access to actual representation
The following functions provide an actual address of the whole array or of its individual elements. These
functions shall be used for accessing elements of arrays of types compatible with C. These functions are also
useful for vendors because they provide access to the actual representation for all types of arrays.
If the actual layout of the SystemVerilog array passed as an argument for an open unpacked array is different
from the C layout, then it is not possible to access such an array as a whole; therefore, the address and size of
such an array shall be undefined (0, to be exact). Nonetheless, the addresses of individual elements of an
array shall be always supported.
NOTE—No specific representation of an array is assumed here; hence, all functions use a generic pointer void *.
/* a pointer to the actual representation of the whole array of any type */
/* NULL if not in C layout */
void *svGetArrayPtr(const svOpenArrayHandle);
int svSizeOfArray(const svOpenArrayHandle); /* total size in bytes or 0 if not
in C layout */
/* Return a pointer to an element of the array
or NULL if index outside the range or null pointer */
void *svGetArrElemPtr(const svOpenArrayHandle, int indx1, ...);
/* specialized versions for 1-, 2- and 3-dimensional arrays: */
void *svGetArrElemPtr1(const svOpenArrayHandle, int indx1);
void *svGetArrElemPtr2(const svOpenArrayHandle, int indx1, int indx2);
void *svGetArrElemPtr3(const svOpenArrayHandle, int indx1, int indx2,
int indx3);
Access to an individual array element via pointer makes sense only if the representation of such an element
is the same as it would be for an individual value of the same type. Representation of array elements of type
scalar or packed value is implementation dependent; the above functions shall return NULL if the
representation of the array elements differs from the representation of individual values of the same type.
H.12.5 Access to elements via canonical representation
This group of functions is meant for accessing elements that are packed arrays (bit or logic).

<!-- Page 1284 -->

IEEE Std The following functions copy a whole packed array (a single vector) from a canonical representation to an
element of an open array or they copy in the other direction. The actual argument’s original SystemVerilog
ranges are used to index the open array. The user is responsible for ensuring that the canonical
representation has an adequate size for the copy operation.
/* functions for translation between simulator and canonical representations*/
/* s=source, d=destination */
/* From user space into simulator storage */
void svPutBitArrElemVecVal(const svOpenArrayHandle d, const svBitVecVal* s,
int indx1, ...);
void svPutBitArrElem1VecVal(const svOpenArrayHandle d, const svBitVecVal* s,
int indx1);
void svPutBitArrElem2VecVal(const svOpenArrayHandle d, const svBitVecVal* s,
int indx1, int indx2);
void svPutBitArrElem3VecVal(const svOpenArrayHandle d, const svBitVecVal* s,
int indx1, int indx2, int indx3);
void svPutLogicArrElemVecVal(const svOpenArrayHandle d, const svLogicVecVal* s,
int indx1, ...);
void svPutLogicArrElem1VecVal(const svOpenArrayHandle d, const svLogicVecVal* s,
int indx1);
void svPutLogicArrElem2VecVal(const svOpenArrayHandle d, const svLogicVecVal* s,
int indx1, int indx2);
void svPutLogicArrElem3VecVal(const svOpenArrayHandle d, const svLogicVecVal* s,
int indx1, int indx2, int indx3);
/* From simulator storage into user space */
void svGetBitArrElemVecVal(svBitVecVal* d, const svOpenArrayHandle s,
int indx1, ...);
void svGetBitArrElem1VecVal(svBitVecVal* d, const svOpenArrayHandle s,
int indx1);
void svGetBitArrElem2VecVal(svBitVecVal* d, const svOpenArrayHandle s,
int indx1, int indx2);
void svGetBitArrElem3VecVal(svBitVecVal* d, const svOpenArrayHandle s,
int indx1, int indx2, int indx3);
void svGetLogicArrElemVecVal(svLogicVecVal* d, const svOpenArrayHandle s,
int indx1, ...);
void svGetLogicArrElem1VecVal(svLogicVecVal* d, const svOpenArrayHandle s,
int indx1);
void svGetLogicArrElem2VecVal(svLogicVecVal* d, const svOpenArrayHandle s,
int indx1, int indx2);
void svGetLogicArrElem3VecVal(svLogicVecVal* d, const svOpenArrayHandle s,
int indx1, int indx2, int indx3);
H.12.6 Access to scalar elements (bit and logic)
Another group of functions is needed for scalars (i.e., when an element of an array is a simple scalar, bit, or
logic):
svBit svGetBitArrElem (const svOpenArrayHandle s, int indx1, ...);
svBit svGetBitArrElem1(const svOpenArrayHandle s, int indx1);
svBit svGetBitArrElem2(const svOpenArrayHandle s, int indx1, int indx2);
svBit svGetBitArrElem3(const svOpenArrayHandle s, int indx1, int indx2,
int indx3);
svLogic svGetLogicArrElem (const svOpenArrayHandle s, int indx1, ...);
svLogic svGetLogicArrElem1(const svOpenArrayHandle s, int indx1);
svLogic svGetLogicArrElem2(const svOpenArrayHandle s, int indx1, int indx2);
svLogic svGetLogicArrElem3(const svOpenArrayHandle s, int indx1, int indx2,

<!-- Page 1285 -->

IEEE Std int indx3);
void svPutLogicArrElem (const svOpenArrayHandle d, svLogic value, int indx1,
...);
void svPutLogicArrElem1(const svOpenArrayHandle d, svLogic value, int indx1);
void svPutLogicArrElem2(const svOpenArrayHandle d, svLogic value, int indx1,
int indx2);
void svPutLogicArrElem3(const svOpenArrayHandle d, svLogic value, int indx1,
int indx2, int indx3);
void svPutBitArrElem (const svOpenArrayHandle d, svBit value, int indx1, ...);
void svPutBitArrElem1(const svOpenArrayHandle d, svBit value, int indx1);
void svPutBitArrElem2(const svOpenArrayHandle d, svBit value, int indx1,
int indx2);
void svPutBitArrElem3(const svOpenArrayHandle d, svBit value, int indx1,
int indx2, int indx3);
H.12.7 Access to array elements of other types
If an array’s elements are of a type compatible with C, there is no need to use canonical representation. In
such situations, the elements are accessed via pointers, i.e., the actual address of an element shall be
computed first and then used to access the desired element.
H.12.8 Example 6—Two-dimensional open array
SystemVerilog:
typedef struct {int i; ... } MyType;
import "DPI-C" function void f1(input MyType i [][]);
/* 2-dimensional unsized unpacked array of MyType */
MyType a_10x5 [11:20][6:2];
MyType a_64x8 [64:1][-1:-8];
f1(a_10x5);
f1(a_64x8);
C:
#include "svdpi.h"
typedef struct {int i; ... } MyType;
void f1(const svOpenArrayHandle h)
{
MyType my_value;
int i, j;
int lo1 = svLow(h, 1);
int hi1 = svHigh(h, 1);
int lo2 = svLow(h, 2);
int hi2 = svHigh(h, 2);
for (i = lo1; i <= hi1; i++) {
for (j = lo2; j <= hi2; j++) {
my_value = *(MyType *)svGetArrElemPtr2(h, i, j);

<!-- Page 1286 -->

IEEE Std ...
*(MyType *)svGetArrElemPtr2(h, i, j) = my_value;
...
}
...
}
}
H.12.9 Example 7—Open array
SystemVerilog:
typedef struct { ... } MyType;
import "DPI-C" function void f1(input MyType i [], output MyType o []);
MyType source [11:20];
MyType target [11:20];
f1(source, target);
C:
#include "svdpi.h"
typedef struct { ... } MyType;
void f1(const svOpenArrayHandle hin, const svOpenArrayHandle hout)
{
int count = svSize(hin, 1);
MyType *s = (MyType *)svGetArrayPtr(hin);
MyType *d = (MyType *)svGetArrayPtr(hout);
if (s && d) { /* both arrays have C layout */
/* an efficient solution using pointer arithmetic */
while (count--)
*d++ = *s++;
/* even more efficient:
memcpy(d, s, svSizeOfArray(hin));
*/
} else { /* less efficient yet implementation independent */
int i = svLow(hin, 1);
int j = svLow(hout, 1);
while (i <= svHigh(hin, 1)) {
*(MyType *)svGetArrElemPtr1(hout, j++) =
*(MyType *)svGetArrElemPtr1(hin, i++);
}
}
}

<!-- Page 1287 -->

IEEE Std H.12.10 Example 8—Access to packed arrays
SystemVerilog:
import "DPI-C" function void f1(input logic [127:0]);
import "DPI-C" function void f2(input logic [127:0] i []); // open array of
// 128-bit
C:
#include "svdpi.h"
/* Copy out one 128-bit packed vector */
void f1(const svLogicVecVal* packed_vec_128_bit)
{
svLogicVecVal arr[SV_PACKED_DATA_NELEMS(128)]; /* canonical rep */
memcpy(arr, packed_vec_128_bit, sizeof(arr));
...
}
/* Copy out each word of an open array of 128-bit packed vectors */
void f2(const svOpenArrayHandle h)
{
int i;
svLogicVecVal arr[SV_PACKED_DATA_NELEMS(128)]; /* canonical rep */
for (i = svLow(h, 1); i <= svHigh(h, 1); i++) {
const svLogicVecVal* ptr = (svLogicVecVal*)svGetArrElemPtr1(h, i);
memcpy(arr, ptr, sizeof(arr));
...
}
...
}
H.13 Time and timescale
A subroutine can use svGetTime() to retrieve the current simulation time, scaled to the time unit of the
instance scope associated with an svScope. The type field of the associated svTimeVal value shall be set to
indicate whether scaled real or simulation time is desired. Calling svGetTime() with a NULL scope value
shall retrieve the current time scaled to the simulation time unit.
The svTimeVal structure used by svGetTime() is defined in svdpi.h (see AnnexI), and is fully
equivalent to type s_vpi_time, which is used to represent time in VPI.
A subroutine can use svGetTimeUnit() and svGetTimePrecision() to retrieve the current time unit
and precision, respectively, for the instance scope associated with an svScope. Calling these methods with
a NULL scope value shall cause the simulation time unit to be retrieved.
The values returned by svGetTimeUnit() and svGetTimePrecision() are fully equivalent to values
retrieved via vpi_get() using vpiTimeUnit and vpiTimePrecision.
H.14 SV3.1a-compatible access to packed data (deprecated functionality)
The functionality described in this subclause is deprecated and need not be implemented by an IEEE Std
1800 simulator. The functionality provides backwards compatibility with Accellera SystemVerilog 3.1a

<!-- Page 1288 -->

IEEE Std (SV3.1a) [B4] regarding the semantics of packed array arguments. This subclause will describe the SV3.1a
semantics.
The main difference between SV3.1a and IEEE Std 1800 semantics is that in SV3.1a, packed data
arguments are passed by opaque handle types svLogicPackedArrRef and svBitPackedArrRef. An
implementation need not do any conversion or marshalling of data into the canonical format. The C
programmer is provided a set of utility functions that copies data between actual vendor format and
canonical format. Other utilities are provided that put and get bit-selects and part-selects from actual vendor
representation.
H.14.1 Determining the compatibility level of an implementation
Function svDpiVersion() is provided to allow the determination of an implementation’s support for this
standard. In simulators that only support the SV3.1a standard, users shall make use of the opaque handle
types for all 2-state and 4-state arguments. See H.10.1.3.
When using an IEEE Std 1800 implementation, it is possible for users to make use of SV3.1a-compatible
semantics on a per-function basis. Import and export declarations annotated with the "DPI" syntax shall
yield the SV3.1a argument passing semantics on the C side of the interface. Import and export declarations
annotated with the "DPI-C" syntax shall yield the IEEE Std 1800 argument passing semantics. See 35.4 and
35.5.4.
The svdpi.h file may contain definitions and function prototypes for use with SV3.1a-compliant packed
data access. IEEE Std 1800 implementations are not obligated to provide these definitions and prototypes in
the include file.
If an IEEE Std 1800 implementation does not support the functionality in this subclause, it is possible that
the DPI C code may not successfully bind to the implementation.
H.14.2 svdpi.h definitions for SV3.1a-style packed data processing
The following definitions are used to define SV3.1a-style canonical access to packed data:
/* 2-state and 4-state vectors, modeled upon PLI's avalue/bvalue */
#define SV_CANONICAL_SIZE(WIDTH) (((WIDTH)+31)>>5)
typedef uint32_t
svBitVec32; /* (a chunk of) packed bit array */
typedef struct { unsigned int c; unsigned int d;}
svLogicVec32; /* (a chunk of) packed logic array */
The following definitions describe implementation-dependent packed data representation:
/* reference to a standalone packed array */
typedef void* svBitPackedArrRef;
typedef void* svLogicPackedArrRef;
/* total size in bytes of the simulator's representation of a packed array */
/* width in bits */
int svSizeOfBitPackedArr(int width);
int svSizeOfLogicPackedArr(int width);

<!-- Page 1289 -->

IEEE Std The following functions provide translation between actual vendor representation and canonical
representation. The functions copy the whole array in either direction. The user is responsible for providing
the correct width and for allocating an array in the canonical representation. The contents of the unused bits
are undetermined.
Although the put and get functionality provided for bit and logic packed arrays is sufficient, yet basic, it
requires unnecessary copying of the whole packed array when perhaps only some bits are needed. For the
sake of convenience and improved performance, bit-selects and limited (up to 32 bits) part-selects are also
supported.
/* s=source, d=destination, w=width */
/* actual <-- canonical */
void svPutBitVec32 (svBitPackedArrRef d, const svBitVec32* s, int w);
void svPutLogicVec32 (svLogicPackedArrRef d, const svLogicVec32* s, int w);
/* canonical <-- actual */
void svGetBitVec32 (svBitVec32* d, const svBitPackedArrRef s, int w);
void svGetLogicVec32 (svLogicVec32* d, const svLogicPackedArrRef s, int w);
The following functions provide support for bit-select processing on actual vendor data representation:
/* Packed arrays are assumed to be indexed n-1:0, where 0 is the index of
LSB */
/* functions for bit-select */
/* s=source, i=bit-index */
svBit svGetSelectBit(const svBitPackedArrRef s, int i);
svLogic svGetSelectLogic(const svLogicPackedArrRef s, int i);
/* d=destination, i=bit-index, s=scalar */
void svPutSelectBit(svBitPackedArrRef d, int i, svBit s);
void svPutSelectLogic(svLogicPackedArrRef d, int i, svLogic s);
Limited (up to 32-bit) part-selects are supported. A part-select is a slice of a packed array of types bit or
logic. Array slices are not supported for unpacked arrays. Functions for part-selects only allow access
(read/write) to a narrow subrange of up to 32 bits. Canonical representation shall be used for such narrow
vectors. If the specified range of a part-select is not fully contained within the normalized range of an array,
the behavior is undetermined.
/*
* functions for part-select
*
* a narrow (<=32 bits) part-select is copied between
* the implementation representation and a single chunk of
* canonical representation
* Normalized ranges and indexing [n-1:0] are used for both arrays:
* the array in the implementation representation and the canonical array.
*
* s=source, d=destination, i=starting bit index, w=width
* like for variable part-selects; limitations: w <= 32
*
* In part-select operations, the data are copied to or from the
* canonical representation part ('chunk') designated by range [w-1:0]
* and the implementation representation part designated by range [w+i-1:i].
*/
/* canonical <-- actual */
void svGetPartSelectBit(svBitVec32* d, const svBitPackedArrRef s, int i,
int w);
svBitVec32 svGetBits(const svBitPackedArrRef s, int i, int w);

<!-- Page 1290 -->

IEEE Std svBitVec32 svGet32Bits(const svBitPackedArrRef s, int i); // 32-bits
uint64_t svGet64Bits(const svBitPackedArrRef s, int i); // 64-bits
void svGetPartSelectLogic(svLogicVec32* d, const svLogicPackedArrRef s, int i,
int w);
/* actual <-- canonical */
void svPutPartSelectBit(svBitPackedArrRef d, const svBitVec32 s, int i,
int w);
void svPutPartSelectLogic(svLogicPackedArrRef d, const svLogicVec32 s, int i,
int w);
H.14.3 Source-level compatibility include file svdpi_src.h
Only two symbols are defined: the macros that allow declaring variables to represent the SystemVerilog
packed arrays of type bit or logic. Applications that do not need this file to compile are deemed binary
compatible. In other words, the DPI C code does not need to be recompiled to run on different simulators.
Applications that make use of svdpi_src.h have to be recompiled for each simulator on which they are to
be run.
#define SV_BIT_PACKED_ARRAY(WIDTH,NAME) ...
#define SV_LOGIC_PACKED_ARRAY(WIDTH,NAME) ...
The actual definitions are implementation-specific, but shall not define an array type (see definition in 6.2.5
in ISO/IEC 9899:1999 [B3]). For example, a SystemVerilog simulator might define the latter macro as
follows:
#define SV_LOGIC_PACKED_ARRAY(WIDTH,NAME) \
struct { svLogicVec32 __unnamed [SV_CANONICAL_SIZE(WIDTH)]; } NAME
H.14.4 Example 9—Deprecated SV3.1a binary-compatible application
SystemVerilog:
typedef struct {int x; int y;} pair;
import "DPI" function void f1(input int i1, pair i2, output logic [63:0] o3);
export "DPI" function exported_sv_func;
function void exported_sv_func(input int i, output int o [0:7]);
begin ... end
endfunction
C:
include "svdpi.h"
typedef struct {int x; int y;} pair;
extern void exported_sv_func(int, int *); /* imported from SystemVerilog */
void f1(const int i1, const pair *i2, svLogicPackedArrRef* o3)
{
svLogicVec32 arr[SV_CANONICAL_SIZE(64)]; /* 2 chunks needed */
int tab[8];
printf("%d\n", i1);
arr[0].c = i2->x;
arr[0].d = 0;
arr[1].c = i2->y;
arr[1].d = 0;

<!-- Page 1291 -->

IEEE Std svPutLogicVec32(o3, arr, 64);
/* call SystemVerilog */
exported_sv_func(i1, tab); /* tab passed by reference */
...
}
H.14.5 Example 10—Deprecated SV3.1a source-compatible application
SystemVerilog:
typedef struct {int a; bit [6:1][1:8] b [65:2]; int c;} triple;
// troublesome mix of C types and packed arrays
import "DPI" function void f1(input triple t);
export "DPI" function exported_sv_func;
function void exported_sv_func(input int i, output logic [63:0] o);
begin ... end
endfunction
C:
#include "svdpi.h"
#include "svdpi_src.h"
typedef struct {
int a;
SV_BIT_PACKED_ARRAY(6*8, b) [64]; /* implementation-specific
representation */
int c;
} triple;
/* Note that 'b' is defined as for 'bit [6*8-1:0] b [63:0]' */
extern void exported_sv_func(int, svLogicPackedArrRef); /* imported from
SystemVerilog */
void f1(const triple *t)
{
int j;
/* canonical representation */
svBitVec32 aB[SV_CANONICAL_SIZE(6*8)]; /* 6*8 packed bits */
svLogicVec32 aL[SV_CANONICAL_SIZE(64)];
/* implementation-specific representation */
SV_LOGIC_PACKED_ARRAY(64, my_tab);
printf("%d %d\n", t->a, t->c);
for (i = 0; i < 64; i++) {
svGetBitVec32(aB, (svBitPackedArrRef)&(t->b[i]), 6*8);
...
}
...
/* call SystemVerilog */
exported_sv_func(2, (svLogicPackedArrRef)&my_tab); /* by reference */
svGetLogicVec32(aL, (svLogicPackedArrRef)&my_tab, 64);
...

<!-- Page 1292 -->

IEEE Std }
H.14.6 Example 11—Deprecated SV3.1a binary-compatible calls of export functions
This example demonstrates that the source compatibility include file svdpi_src.h is not needed if a C
function dynamically allocates the data structure for simulator representation of a packed array to be passed
to an exported SystemVerilog function.
SystemVerilog:
export "DPI" function myfunc;
...
function void myfunc (output logic [31:0] r); ...
...
C:
#include "svdpi.h"
extern void myfunc (svLogicPackedArrRef r); /* exported from SV */
/* output logic packed 32-bits */
...
svLogicVec32 my_r[SV_CANONICAL_SIZE(32)];
/* my array, canonical representation */
/* allocate memory for logic packed 32-bits in simulator's representation */
svLogicPackedArrRef r =
(svLogicPackedArrRef)malloc(svSizeOfLogicPackedArr(32));
myfunc(r);
/* canonical <-- actual */
svGetLogicVec32(my_r, r, 32);
/* shall use only the canonical representation from now on */
free(r); /* do not need any more */
...

<!-- Page 1293 -->

IEEE Std 1800-2023