# Annex P: Annex P

## Annex P
(informative)
Glossary
For the purposes of this document, the following terms and definitions apply. The IEEE Standards
Dictionary Online should be consulted for terms not defined in this clause. 21
aggregate: A set or collection of singular values, e.g., an aggregate expression, data object, or data type. An
aggregate data type is any unpacked structure, unpacked union, or unpacked array data type. Aggregates
may be copied or compared as a whole, but not typically used in an expression as a whole.
assertion: An assertion statement.
assertion statement: A statement that specifies the verification function to be performed on an underlying
property. An assertion statement is of one of the following kinds:
— assert, to specify the property as an obligation for the design that is to be checked to verify that the
property holds.
— assume, to specify the property as an assumption for the environment. Simulators check that the
property holds, while formal tools use the information to generate input stimulus.
— cover, to monitor the property evaluation for coverage.
— restrict, to specify the property as a constraint on formal verification computations. Simulators
do not check the property.
The underlying property describes the behavioral criterion that is evaluated by the assertion statement. The
property may be an immediate condition, e.g., that the read_enable and write_enable signals are
mutually exclusive, or it may be a temporal condition, e.g., that if a read_request occurs, then a
read_grant occurs within two clock cycles. An assertion statement is either immediate, for which the
underlying property shall be an immediate condition, or concurrent, for which the underlying property may
be either an immediate or a temporal condition. There is no immediate restrict assertion statement.
Assertion statements can generate automatic messages to report that the disposition of the evaluation of the
underlying property is of interest for the kind of the assertion statement, e.g., a failing evaluation disposition
for an assert or assume, or a passing disposition for a cover.
NOTE—SystemVerilog provides special assertion constructs, which are discussed in Clause16. See 16.2 for a
discussion of assertion statements.
bit-stream data type: Any data type whose values can be represented as a serial stream of bits. To qualify
as a bit-stream data type, each and every bit of the values shall be individually addressable. In other words, a
bit-stream data type can be any data type except for a handle, chandle, real, shortreal, or event.
blocking statement: A construct having the potential to suspend a process. This potential is determined
through lexical analysis of the source syntax alone, not by execution semantics. For example, the statement
wait(1) is considered a blocking statement even though evaluation of the expression '1' will be true at
execution. All statements with procedural event controls (see 9.4) become blocking statements. A task
enable is also a blocking statement because the task may itself contain a blocking statement. The task caller
is not required to recursively investigate the task body.
21IEEE Standards Dictionary Online is available at: http://dictionary.ieee.org.

<!-- Page 1351 -->

IEEE Std A blocking assignment (see 10.4.1) is only considered a blocking statement when the syntax contains an
optional intra-assignment delay. Without the delay, a blocking assignment is not a blocking statement. A
nonblocking assignment (see 10.4.2) is never a blocking statement.
canonical representation: A data representation format established by convention into which and from
which translations can be made with specialized representations.
constant: Either of two types of constants in SystemVerilog: elaboration constant or run-time constant.
Parameters and local parameters are elaboration constants. Their values are calculated before elaboration is
complete. Elaboration constants can be used to set the range of array types. Run-time constants are variables
that can only be set in an initialization expression using the const qualifier.
context imported task: A direct programming interface (DPI) imported task declared with the “context”
property that is capable of calling exported subroutines and capable of accessing SystemVerilog objects via
the SystemVerilog Verification Procedural Interface (VPI) or Programming Language Interface (PLI) calls.
data object: A named entity that has a data value associated with it. Examples of data objects are nets,
variables, and parameters. A data object has a data type that determines which values the data object can
have.
data type: A set of values and a set of operations that can be performed on those values. Examples of data
types are logic, real, and string. Data types can be used to declare data objects or to define user-defined
data types that are constructed from other data types.
direct programming interface (DPI): An interface between SystemVerilog and foreign programming
languages permitting direct function calls from SystemVerilog to foreign code and from foreign code to
SystemVerilog. It has been designed to have low inherent overhead and permit direct exchange of data
between SystemVerilog and foreign code.
disable protocol: A set of conventions for setting, checking, and handling disable status.
dynamic: Having values that can be resized or reallocated at run time. Dynamic arrays, associative arrays,
queues, class handles, and data types that include such data types are dynamic data types.
elaboration: The process of binding together the components that make up a design. These components can
include module instances, primitive instances, interfaces, and the top level of the design hierarchy.
enumerated type: Data types that can declare a data object that can have one of a set of named values. The
numerical equivalents of these values can be specified. Values of an enumerated data type can be easily
referenced or displayed using the enumerated names, as opposed to the enumerated values.
exported task: A SystemVerilog task that is declared in an export declaration and can be enabled from an
imported task.
imported task: A direct programming interface (DPI) foreign code subprogram that can call exported tasks
and can directly or indirectly consume simulation time.
integral: (A) A data type representing integer values. (B) A integer value that may be signed or unsigned,
sliced into smaller integral values, or concatenated into larger values. Syn: vectored value. (C) An
expression of an integral data type. (D) An object of an integral data type.
interface: An encapsulation of the communication between blocks of a design, allowing a smooth migration
from abstract system-level design through successive refinement down to lower level register transfer and
structural views of the design. By encapsulating the communication between blocks, the interface construct

<!-- Page 1352 -->

IEEE Std also facilitates design reuse. The inclusion of interface capabilities is one of the major advantages of
SystemVerilog.
Language Reference Manual (LRM): A document describing the syntax, semantics, and usage of a
programming language. SystemVerilog LRM refers to this standard.
open array: A direct programming interface (DPI) array formal argument for which the packed or unpacked
dimension size (or both) is not specified and for which interface routines describe the size of corresponding
actual arguments at run time.
packed array: An array where the dimensions are declared before an object name. Packed arrays can have
any number of dimensions. A one-dimensional packed array is the same as a vector width declaration in
IEEE Std 1364-2005 Verilog. Packed arrays provide a mechanism for subdividing a vector into subfields,
which can be conveniently accessed as array elements. A packed array differs from an unpacked array, in
that the whole array is treated as a single vector for arithmetic operations.
process: A thread of one or more programming statements that can be executed independently of other
programming statements. Each elaborated instance of an initial procedure, always, always_comb,
always_latch, always_ff procedure, or continuous assignment statement in SystemVerilog is a separate
process. These are static processes; their existence is determined by the static instance hierarchy, their
execution begins at the start of simulation, and they cannot be created at run time. SystemVerilog also has
dynamic processes that can be created, stopped, restarted, and destroyed at run time.
signal: An informal term, usually meaning either a variable or net. The context where it is used may imply
further restrictions on allowed types.
singular: An expression, data object, or data type that represents a single value, symbol, or handle. A
singular data type is any data type except an unpacked structure, unpacked union, or unpacked array data
type.
subroutine: An encapsulation of executable code that can be invoked from one or more places. There are
two forms of subroutines, tasks and functions.
unpacked array: An array where the dimensions are declared after an object name. Unpacked arrays are the
same as arrays in IEEE Std 1364-2005 Verilog and can have any number of dimensions. An unpacked array
differs from a packed array in that the whole array cannot be used for arithmetic operations. Each element
shall be treated separately.
Verification Procedural Interface (VPI): The third generation programming language interface (PLI)
access libraries, providing object-oriented access to SystemVerilog behavioral, structural, assertion, and
coverage objects.
Verilog: The hardware description language (HDL) in IEEE Std 1364-2005.

<!-- Page 1353 -->

IEEE Std 1800-2023