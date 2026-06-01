# Chapter 20: Utility system tasks and system functions

## 20. Utility system tasks and system functions
### 20.1 General
This clause describes the utility system tasks and system functions that are part of SystemVerilog. Clause21
presents additional system tasks and system functions that are specific to I/O operations. The system tasks
and system functions described in this clause are divided into several categories, as follows:
Simulation control tasks (20.2) Assertion control tasks (20.11)
$finish $stop $asserton $assertoff
$exit $assertkill $assertcontrol
$assertpasson $assertpassoff
Simulation time functions (20.3)
$assertfailon $assertfailoff
$realtime $stime $assertnonvacuouson $assertvacuousoff
$time
Sampled value functions (20.12)
Timescale tasksand functions (20.4)
$sampled $rose
$timeunit $timeprecision $fell $stable
$printtimescale $timeformat $changed $past
Conversion functions (20.5) $past_gclk $rose_gclk
$fell_gclk $stable_gclk
$bitstoreal $realtobits
$changed_gclk $future_gclk
$bitstoshortreal $shortrealtobits
$rising_gclk $falling_gclk
$itor $rtoi
$steady_gclk $changing_gclk
$signed $unsigned
$cast Coverage control functions (20.13)
Data query functions (20.6) $coverage_control $coverage_get_max
$coverage_get $coverage_merge
$bits $isunbounded
$coverage_save $get_coverage
$typename
$set_coverage_db_name $load_coverage_db
Array query functions (20.7)
Probabilistic distribution functions (20.14)
$unpacked_dimensions $dimensions
$random $dist_chi_square
$left $right
$dist_erlang $dist_exponential
$low $high
$dist_normal $dist_poisson
$increment $size
$dist_t $dist_uniform
Math functions (20.8)
Stochastic analysis tasks and functions (20.15)
$clog2 $asin
$q_initialize $q_add
$ln $acos
$q_remove $q_full
$log10 $atan
$q_exam
$exp $atan2
$sqrt $hypot PLA modeling tasks (20.16)
$pow $sinh
$async$and$array $async$and$plane
$floor $cosh
$async$nand$array $async$nand$plane
$ceil $tanh
$async$or$array $async$or$plane
$sin $asinh
$async$nor$array $async$nor$plane
$cos $acosh
$sync$and$array $sync$and$plane
$tan $atanh
$sync$nand$array $sync$nand$plane
Bit vector functions (20.9) $sync$or$array $sync$or$plane
$sync$nor$array $sync$nor$plane
$countbits $countones
$onehot $onehot0 Miscellaneous tasks and functions (20.17)
$isunknown
$system $stacktrace
Severity tasks (20.10)
$fatal $error
$warning $info

<!-- Page 621 -->

IEEE Std ### 20.2 Simulation control system tasks
This subclause defines the following three simulation control system tasks:
a) $finish
b) $stop
c) $exit
simulation_control_task ::=
$stop [ ( n ) ] ;
$finish [ ( n ) ] ;
$exit [ ( ) ] ;
Syntax20-1—Syntax for simulation control tasks (not in AnnexA)
The $stop system task causes simulation to be suspended.
The $finish system task causes the simulator to exit and pass control back to the host operating system.
The $exit control task waits for all program blocks to complete, and then makes an implicit call to
$finish. The usage of $exit is presented in 24.7 on program blocks.
The $stop and $finish system tasks take an optional expression argument (0, 1, or 2) that determines
what type of diagnostic message is printed, as shown in Table20-1. If no argument is supplied, then a value
of 1 is taken as the default.
Table20-1—Diagnostics for $finish
Argument value Diagnostic message
0 Prints nothing
1 Prints simulation time and location
Prints simulation time, location, and statistics about the memory
and central processing unit (CPU) time used in simulation
### 20.3 Simulation time system functions
The following system functions provide access to current simulation time:
$time $stime $realtime
The syntax for time system functions is shown in Syntax20-2.
time_function ::=
$time
| $stime
| $realtime
Syntax20-2—Syntax for time system functions (not in AnnexA)

<!-- Page 622 -->

IEEE Std #### 20.3.1 $time
The $time system function returns an integer that is a 64-bit time, scaled to the time unit of the module that
invoked it.
For example:
`timescale 10 ns / 1 ns
module test;
logic set;
parameter p = 1.55;
initial begin
$monitor($time,,"set=", set);
#p set = 0;
#p set = 1;
end
endmodule
The output from this example is as follows:
0 set=x
2 set=0
3 set=1
In this example, the variable set is assigned the value 0 at simulation time 16 ns, and the value 1 at
simulation time 32 ns. The time values returned by the $time system function are determined by the
following steps:
a) The simulation times 16 ns and 32 ns are scaled to 1.6 and 3.2 because the time unit for the module
is 10 ns; therefore, time values reported by this module are multiples of 10 ns.
b) The value 1.6 is rounded to 2, and 3.2 is rounded to 3 because the $time system function returns
an integer. The time precision does not cause rounding of these values.
NOTE—The times at which the assignments take place in this example do not match the times reported by $time.
#### 20.3.2 $stime
The $stime system function returns an unsigned integer that is a 32-bit time, scaled to the time unit of the
module that invoked it. If the actual simulation time does not fit in 32 bits, the low order 32 bits of the
current simulation time are returned.
#### 20.3.3 $realtime
The $realtime system function returns a real number time that, like $time, is scaled to the time unit of the
module that invoked it.
For example:
`timescale 10 ns / 1 ns
module test;
logic set;
parameter p = 1.55;
initial begin
$monitor($realtime,,"set=", set);
#p set = 0;
#p set = 1;
end
endmodule

<!-- Page 623 -->

IEEE Std The output from this example is as follows:
0 set=x
### 1.6 set=0
### 3.2 set=1
In this example, the event times in the variable set are multiples of 10 ns because 10 ns is the time unit of
the module. They are real numbers because $realtime returns a real number.
### 20.4 Timescale system tasks and system functions
This subclause defines the system tasks and functions that retrieve and display the timescale, and set
timescale printing information:
a) $timeunit
b) $timeprecision
c) $printtimescale
d) $timeformat
See 22.7 for a discussion of timescale and time units.
#### 20.4.1 Timescale retrieval system functions
The $timeunit and $timeprecision system functions return the time unit and time precision,
respectively, for a particular design element.
The syntax for these functions is as follows:
timeunit_function ::= $timeunit [ ( hierarchical_identifier ) ] ;
timeprecision_function ::= $timeprecision [ ( hierarchical_identifier ) ] ;
Syntax20-3—Syntax for $timeunit and $timeprecision (not in AnnexA)
These system functions can be specified with or without an argument.
— When no argument is specified, the return value is the time unit or precision of the designelement
that is the current scope.
— When an argument is specified, the return value is the time unit or precision of the designelement
passed to it.
— When the argument specified is $unit, the return value is the time unit or precision of the
compilation unit.
— When the argument specified is $root, the return value for both functions is the simulation time
unit (see 3.14.3).
The return value shall be an integer in the range from 2 to -15. This argument represents the time unit and
time precision as shown in Table20-2.

<!-- Page 624 -->

IEEE Std Table20-2—Time unit and precision number values
Time unit Time unit
Value Value
or precision or precision
2 100 s –7 100 ns
1 10 s –8 10 ns
0 1 s –9 1 ns
–1 100 ms –10 100 ps
–2 10 ms –11 10 ps
–3 1 ms –12 1 ps
–4 100 us –13 100 fs
–5 10 us –14 10 fs
–6 1 us –15 1 fs
NOTE—While s, ms, ns, ps, and fs are the usual SI unit symbols for second, millisecond, nanosecond, picosecond, and
femtosecond, due to lack of the Greek letter μ (mu) in coding character sets, “us” represents the SI unit symbol for
microsecond, properlyμs.
For example:
timeunit 100ps / 10fs;
module a_dat;
timeunit 1ms / 1us;
initial begin
$display("Simulation time unit: %0d, precision: %0d",
$timeunit($root), $timeprecision($root));
$display("Compilation time unit: %0d, precision: %0d",
$timeunit($unit), $timeprecision($unit));
$display("a_dat time unit: %0d, precision: %0d",
$timeunit, $timeprecision);
$display("b_dat.c1 time unit: %0d, precision: %0d",
$timeunit(b_dat.c1), $timeprecision(b_dat.c1));
end
endmodule
`timescale 10 fs / 1 fs
module b_dat;
c_dat c1 ();
endmodule
`timescale 1 ns / 1 ns
module c_dat;
.
.
.
endmodule
In this example, module a_dat invokes $timeunit and $timeprecision to retrieve the globaltimescale
information and that of the compilation unit, as well as its own timescale information and that of another
module, c_dat, which is instantiated in module b_dat.

<!-- Page 625 -->

IEEE Std The simulation results are as follows:
Simulation time unit: -15, precision: -15
Compilation time unit: -10, precision: -14
a_dat time unit: -3, precision -6
b_dat.c1 time unit: -9, precision: -9
#### 20.4.2 $printtimescale
The $printtimescale system task displays the time unit and precision for a particular design element.
The syntax for the system task is shown in Syntax20-4.
printtimescale_task ::= $printtimescale [ ( hierarchical_identifier ) ] ;
Syntax20-4—Syntax for $printtimescale (not in AnnexA)
This system task can be specified with or without an argument.
— When no argument is specified, $printtimescale displays the time unit and precision of the
designelement that is the current scope.
— When an argument is specified, $printtimescale displays the time unit and precision of the
designelement passed to it.
— When the argument specified is $unit, $printtimescale displays the time unit and precision of
the compilation unit.
— When the argument specified is $root, $printtimescale displays the simulation time unit (see
3.14.3).
The timescale information shall appear in the following format:
Time scale of (<design_element_name>) is <unit> / <precision>
When the argument specified to $printtimescale is $unit or $root, the design element name in the
output shall be “$unit” or “$root”, respectively.
For example:
timeunit 100ps / 10fs;
module a_dat;
timeunit 1ms / 1us;
initial begin
$printtimescale($root);
$printtimescale($unit);
$printtimescale();
$printtimescale(b_dat.c1);
end
endmodule
`timescale 10 fs / 1 fs
module b_dat;
c_dat c1 ();
endmodule
`timescale 1 ns / 1 ns
module c_dat;
.

<!-- Page 626 -->

IEEE Std .
.
endmodule
In this example, module a_dat invokes the $printtimescale system task to displaythe globaltimescale
information and that of the compilation unit, as well as its own timescale information and that of another
module, c_dat, which is instantiated in module b_dat.
The information will be displayed in the following format:
Time scale of ($root) is 1fs / 1fs
Time scale of ($unit) is 100ps / 10fs
Time scale of (a_dat) is 1ms / 1us
Time scale of (b_dat.c1) is 1ns / 1ns
NOTE—As the simulation time unit and the global time precision are synonymous, the time unit and precision values
displayed for $root will always be identical.
#### 20.4.3 $timeformat
The syntax for the $timeformat system task is shown in Syntax20-5.
timeformat_task ::=
$timeformat [ ( units_number , precision_number , suffix_string , minimum_field_width ) ] ;
Syntax20-5—Syntax for $timeformat (not in AnnexA)
The $timeformat system task specifies how the %t format specification reports time information for the
display and file output system tasks and system functions in 21.2 and 21.3. It sets the time unit, precision
number, suffix string, and minimum field width for all %t formats in the design until another $timeformat
system task is invoked.
The units number and precision number argument shall be integers in the range from 2 to -15. These
arguments represent the units for time as shown in Table20-2 in 20.4.1.
The default $timeformat system task arguments are given in Table20-3.
Table20-3—$timeformat default values for arguments
Argument Default
units_number The smallest time precision argument of all the `timescale
compiler directives in the source description
precision_number 0
suffix_string A null character string
minimum_field_width 20
The following example shows the use of %t with the $timeformat system task to specify a uniform time
unit, time precision, and format for timing information.
`timescale 1 ms / 1 ns
module cntrl;

<!-- Page 627 -->

IEEE Std initial
$timeformat(-9, 5, " ns", 10);
endmodule
`timescale 1 fs / 1 fs
module a1_dat;
logic in1;
integer file;
buf #10000000 (o1,in1);
initial begin
file = $fopen("a1.dat");
#00000000 $fmonitor(file,"%m: %t in1=%d o1=%h", $realtime,in1,o1);
#10000000 in1 = 0;
#10000000 in1 = 1;
end
endmodule
`timescale 1 ps / 1 ps
module a2_dat;
logic in2;
integer file2;
buf #10000 (o2,in2);
initial begin
file2=$fopen("a2.dat");
#00000 $fmonitor(file2,"%m: %t in2=%d o2=%h",$realtime,in2,o2);
#10000 in2 = 0;
#10000 in2 = 1;
end
endmodule
The contents of file a1.dat are as follows:
a1_dat: 0.00000 ns in1=x o1=x
a1_dat: 10.00000 ns in1=0 o1=x
a1_dat: 20.00000 ns in1=1 o1=0
a1_dat: 30.00000 ns in1=1 o1=1
The contents of file a2.dat are as follows:
a2_dat: 0.00000 ns in2=x o2=x
a2_dat: 10.00000 ns in2=0 o2=x
a2_dat: 20.00000 ns in2=1 o2=0
a2_dat: 30.00000 ns in2=1 o2=1
In this example, the times of events written to the files by the $fmonitor system task in modules a1_dat
and a2_dat are reported as multiples of 1 ns—even though the time units for these modules are 1 fs and
1 ps, respectively—because the first argument of the $timeformat system task is -9 and the %t format
specification is included in the arguments to $fmonitor. This time information is reported after the
module names with five fractional digits, followed by an ns character string in a space wide enough for
10 ASCII characters.
### 20.5 Conversion functions
System functions are provided to convert values to and from real number values, and to convert values to
signed or unsigned values.

<!-- Page 628 -->

IEEE Std The following system functions handle real number values (the real and shortreal types).
integer $rtoi ( real_val )
real $itor ( int_val )
[63:0] $realtobits ( real_val )
real $bitstoreal ( bit_val )
[31:0] $shortrealtobits ( shortreal_val )
shortreal $bitstoshortreal ( bit_val )
These conversion system functions may be used in constant expressions, as specified in 11.2.1.
$rtoi converts real values to an integer type by truncating the real value (for example, 123.45 becomes
123). $rtoi differs from casting a real value to an integer or other integral type in that casting will
perform rounding instead of truncation. Directly assigning a real value to an integral type will also round
instead of truncate.
$itor converts integer values to real values (for example, 123 becomes 123.0).
$realtobits converts values from a real type to a 64-bit vector representation of the real number.
$bitstoreal converts a bit pattern created by $realtobits to a value of the real type.
$shortrealtobits converts values from a shortreal type to the 32-bit vector representation of the real
number.
$bitstoshortreal converts a bit pattern created by $shortrealtobits to a value of the shortreal
type.
NOTE 1—The real numbers accepted or generated by these functions shall conform to the IEEE Std 754 representation
of the single precision and double precision floating-point numbers. The conversion shall round the result to the nearest
valid representation.
The following example shows how the $realtobits and $bitstoreal functions can be used in port
connections:
module driver (net_r);
output [64:1] net_r;
real r;
wire [64:1] net_r = $realtobits(r);
endmodule
module receiver (net_r);
input [64:1] net_r;
wire [64:1] net_r;
real r;
initial assign r = $bitstoreal(net_r);
endmodule
NOTE 2—SystemVerilog allows directly passing real values across module, interface, and program ports; it is not
necessary to use the $realtobits and $bitstoreal conversion functions as shown in this example. IEEE Std
1364-2005 did not allow directly passing real values across module ports, and therefore utilized these system functions.
The $signed and $unsigned system functions can be used to cast the signedness (but not the type) of
expressions. These functions shall evaluate the input expression and return a value with the same size and
value of the input expression and the type defined by the function.

<!-- Page 629 -->

IEEE Std $signed—returned value is signed
$unsigned—returned value is unsigned
See 11.7 for examples of using $signed and $unsigned. The cast operator can also be used to change the
signedness of an expression (see 6.24.1).
The $cast system function performs a dynamic cast of an expression type. $cast is described in 6.24.2 and
8.16.
### 20.6 Data query functions
SystemVerilog provides system functions to query information about expressions $typename, $bits, and
$isunbounded.
#### 20.6.1 Type name function
The $typename system function returns a string that represents the resolved type of its argument.
typename_function ::=
$typename ( expression )
| $typename ( data_type )
Syntax20-6—Type name function syntax (not in AnnexA)
The return string is constructed in the following steps:
a) A typedef that creates an equivalent type is resolved back to built-in or user-defined types.
b) The default signing is removed, even if present explicitly in the source.
c) System-generated names are created for anonymous structs, unions, and enums.
d) A “$” is used as the placeholder for the name of an anonymous unpacked array.
e) Actual encoded values are appended with enumeration named constants.
f) User-defined type names are prefixed with their defining package or scope name space.
g) Array ranges are represented as unsized decimal numbers.
h) White space in the source is removed and a single space is added to separate identifiers and
keywords from each other.
This process is similar to the way that type matching (see 6.22.1) is computed, except that simple bit vector
types with predefined widths are distinguished from those with user-defined widths. Thus $typename can
be used in string comparisons for stricter type comparison of arrays than with type references.
When called with an expression as its argument, $typename returns a string that represents the self-
determined type result of the expression. The expression’s return type is determined during elaboration, but
never evaluated. When used as an elaboration-time constant, the expression shall not contain any
hierarchical references or references to elements of dynamic objects.
// source code // $typename would return
typedef bit node; // "bit"
node [2:0] X; // "bit [2:0]"
int signed Y; // "int"
package A;
enum {A,B,C=99} X; // "enum{A=32'sd0,B=32'sd1,C=32'sd99}A::e$1"

<!-- Page 630 -->

IEEE Std typedef bit [9:1'b1] word; // "A::bit[9:1]"
endpackage : A
import A::*;
module top;
typedef struct {node A,B;} AB_t;
AB_t AB[10]; // "struct{bit A;bit B;}top.AB_t$[0:9]"
...
endmodule
#### 20.6.2 Expression size system function
The $bits system function returns the number of bits required to hold an expression as a bit stream. The
return type is integer. See 6.24.3 for a definition of legal types. A 4-state value counts as 1 bit.
size_function ::=
$bits ( expression )
| $bits ( data_type )
Syntax20-7—Size function syntax (not in AnnexA)
Given the declaration
logic [31:0] v;
then $bits(v) shall return 32, even if the implementation uses more than 32 bits of storage to represent the
4-state values. Given the declaration:
typedef struct {
logic valid;
bit [8:1] data;
} MyType;
the expression $bits(MyType) shall return 9, the number of data bits needed by a variable of type MyType.
The $bits function can be used as an elaboration-time constant when used on fixed-size data types; hence,
it can be used in the declaration of other data types, variables, or nets.
typedef bit[$bits(MyType):1] MyBits; //same as typedef bit [9:1] MyBits;
MyBits b;
Variable b can be used to hold the bit pattern of a variable of type MyType without loss of information.
The value returned by $bits shall be determined without actual evaluation of the expression it encloses. It
shall be an error to enclose a function that returns a dynamically sized data type. The $bits return value
shall be valid at elaboration only if the expression contains fixed-size data types.
The $bits system function returns 0 when called with a dynamically sized expression that is currently
empty. It shall be an error to:
— Use the $bits system function directly with a dynamically sized data type identifier.
— Use the $bits system function on an object of an interface class type (see 8.26).

<!-- Page 631 -->

IEEE Std #### 20.6.3 Range system function
The $isunbounded system function returns true (1'b1) if the argument is $. Otherwise, it returns false
(1'b0). The argument shall be a parameter name.
range_function ::= $isunbounded ( ps_parameter_identifier | hierarchical_parameter_identifier )
Syntax20-8—Range function syntax (not in AnnexA)
Given the declaration
parameter int i = $;
then $isunbounded(i) returns true.
### 20.7 Array query functions
array_query_function ::=
array_dimension_function ( array_expression [ , dimension_expression ] )
| array_dimension_function ( data_type [ , dimension_expression ] )
| array_dimensions_function ( array_expression )
| array_dimensions_function ( data_type )
array_dimensions_function ::=
$dimensions
| $unpacked_dimensions
array_dimension_function ::=
$left
| $right
| $low
| $high
| $increment
| $size
array_expression ::= expression
dimension_expression ::= expression
Syntax20-9—Array querying function syntax (not in AnnexA)
SystemVerilog provides system functions to return information about a particular dimension of an array (see
Clause7) or integral (see 6.11.1) data type or of data objects of such a data type.
The return type is integer, and the default for the optional dimension expression is 1. The dimension
expression can specify any fixed-size dimension (packed or unpacked) or any dynamically sized dimension
(dynamic, associative, or queue). When used on a dynamic array or queue dimension, these functions return
information about the current state of the array. For any dimension other than an associative array
dimension:
— $left shall return the left bound of the dimension. For a packed dimension, this is the index of the
most significant element. For a queue or dynamic array dimension, $left shall return 0.
— $right shall return the right bound of the dimension. For a packed dimension, this is the index of
the least significant element. For a queue or dynamic array dimension whose current size is zero,
$right shall return –1.

<!-- Page 632 -->

IEEE Std — For a fixed-size dimension, $increment shall return 1 if $left is greater than or equal to $right
and –1 if $left is less than $right. For a queue or dynamic array dimension, $increment shall
return –1.
— $low shall return the same value as $left if $increment returns –1, and the same value as
$right if $increment returns 1.
— $high shall return the same value as $right if $increment returns –1, and the same value as
$left if $increment returns 1.
— $size shall return the number of elements in the dimension, which is equivalent to:
$high – $low + 1.
— $dimensions shall return the following:
• The total number of dimensions in the array (packed and unpacked, static or dynamic)
• 1 for the string data type or any other nonarray type that is equivalent to a simple bit vector
type (see 6.11.1)
• 0 for any other type
— $unpacked_dimensions shall return the following:
• The total number of unpacked dimensions for an array (static or dynamic)
• 0 for any other type
The dimensions of an array shall be numbered as follows: the slowest varying dimension (packed or
unpacked) is dimension 1. Successively faster varying dimensions have sequentially higher dimension
numbers. Intermediate type definitions are expanded first before numbering the dimensions.
For example:
// Dimension numbers
// 3 4 1 2
logic [3:0][2:1] n [1:5][2:8];
typedef logic [3:0][2:1] packed_reg;
packed_reg n[1:5][2:8]; // same dimensions as in the lines above
An integer type with predefined width (byte, shortint, int, longint, integer, and time) is treated as
a packed array with a single [n-1:0] dimension, where n is the type’s width.
If the first argument to an array query function would cause $dimensions to return 0 or if the second
argument is out of range, then 'x shall be returned.
It is an error to use these functions directly on a dynamically sized type identifier.
Use on associative array dimensions is restricted to index types with integral values. With integral indices,
these functions shall return the following:
— $left shall return 0.
— $right shall return the highest possible index value.
— $low shall return the lowest currently allocated index value, but shall return 'x if there are no
elements currently allocated.
— $high shall return the largest currently allocated index value, but shall return 'x if there are no
elements currently allocated.
— $increment shall return –1.
— $size shall return the number of elements currently allocated.

<!-- Page 633 -->

IEEE Std It shall be legal to call any of these query functions within a constant expression if all three of the following
conditions are true: (1) the call would be legal in an expression, (2) the type operator (see 6.23) applied to
the first argument would be legal and return some fixed-size type, and (3) any optional dimension
expression is a constant expression.
Given the declaration
typedef logic [16:1] Word;
Word Ram[0:9];
the following system functions return 16:
$size(Word)
$size(Ram,2)
#### 20.7.1 Queries over multiple variable dimensions
If any of the functions described in 20.7 are called with arguments (v, n) where v denotes some array
variable and n is greater than 1, then it shall be an error if the dimension indicated by n is a variable-sized
dimension. The following examples illustrate this restriction. This restriction does not affect the
$dimensions or $unpacked_dimensions functions, since they cannot accept a second argument.
int a[3][][5]; // array dimension 2 has variable size
$display( $unpacked_dimensions(a) ); // displays 3
a[2] = new[4];
a[2][2][0] = 220; // OK, a[2][2] is a 5-element array
$display( $size(a, 1) ); // OK, displays 3
$display( $size(a, 2) ); // ERROR, dimension 2 is dynamic
$display( $size(a[2], 1) ); // OK, displays 4 (a[2] is
// a 4-element dynamic array)
$display( $size(a[1], 1) ); // OK, displays 0 (a[1] is
// an empty dynamic array)
$display( $size(a, 3) ); // OK, displays 5 (fixed-size dimension)
### 20.8 Math functions
There are integer and real math functions. The math system functions may be used in constant expressions,
as specified in 11.2.1.
#### 20.8.1 Integer math functions
The system function $clog2 shall return the ceiling of the log base 2 of the argument (the log rounded up to
an integer value). The argument can be an integer or an arbitrary sized vector value. The argument shall be
treated as an unsigned value, and an argument value of 0 shall produce a result of 0.
This system function can be used to compute the minimum address width necessary to address a memory of
a given size or the minimum vector width necessary to represent a given number of states.
For example:
integer result;
result = $clog2(n);

<!-- Page 634 -->

IEEE Std #### 20.8.2 Real math functions
The system functions in Table20-4 shall accept real value arguments and return a real result type. Their
behavior shall match the equivalent C language standard math library function indicated.
Table20-4—SystemVerilog to C real math function cross-listing
SystemVerilog
Equivalent C function Description
function
$ln(x) log(x) Natural logarithm
$log10(x) log10(x) Decimal logarithm
$exp(x) exp(x) Exponential
$sqrt(x) sqrt(x) Square root
$pow(x,y) pow(x,y) xy
$floor(x) floor(x) Floor
$ceil(x) ceil(x) Ceiling
$sin(x) sin(x) Sine
$cos(x) cos(x) Cosine
$tan(x) tan(x) Tangent
$asin(x) asin(x) Arc-sine
$acos(x) acos(x) Arc-cosine
$atan(x) atan(x) Arc-tangent
$atan2(y,x) atan2(y,x) Arc-tangent of y/x
$hypot(x,y) hypot(x,y) sqrt(x*x+y*y)
$sinh(x) sinh(x) Hyperbolic sine
$cosh(x) cosh(x) Hyperbolic cosine
$tanh(x) tanh(x) Hyperbolic tangent
$asinh(x) asinh(x) Arc-hyperbolic sine
$acosh(x) acosh(x) Arc-hyperbolic cosine
$atanh(x) atanh(x) Arc-hyperbolic tangent
### 20.9 Bit vector system functions
bit_vector_function ::=
$countbits ( expression , list_of_control_bits )
| $countones ( expression )
| $onehot ( expression )
| $onehot0 ( expression )
| $isunknown ( expression )
list_of_control_bits ::= control_bit { , control_bit }

<!-- Page 635 -->

IEEE Std Syntax20-10—Bit vector system function syntax (not in AnnexA)
The function $countbits counts the number of bits that have a specific set of values (e.g., 0, 1, x, z) in a
bit vector.
— $countbits ( expression , control_bit { , control_bit } )
This function returns an int equal to the number of bits in expression whose values match one of the
control_bit entries. For example:
— $countbits (expression, '1) returns the number of bits in expression having value 1.
— $countbits (expression, '1, '0) returns the number of bits in expression having values
1or 0.
— $countbits (expression, 'x, 'z) returns the number of bits in expression having values x
or z.
The argument type for the control_bit argument is 1-bit logic and represents one of the values being counted
in the vector. If a value with a width greater than 1 is passed in, only the LSB is used. If any individual value
appears more than once in the control bits, it is treated exactly as if it had appeared once.
The expression argument to $countbits shall be of a bit-stream type. For the purpose of calculating the
return value, the argument is treated as a vector of equal size assigned from {>>{expression}} (see
11.4.14).
For convenience, the following related functions are also provided:
— $countones ( expression ) is equivalent to $countbits(expression,'1).
— $onehot ( expression ) returns true (1'b1) if $countbits(expression,'1)==1, otherwise it
returns false (1'b0).
— $onehot0 ( expression ) returns true (1'b1) if $countbits(expression,'1)<=1, otherwise it
returns false (1'b0).
— $isunknown ( expression ) returns true (1'b1) if $countbits(expression,'x,'z)!=0,
otherwise it returns false (1'b0).
The expression argument to each of the preceding functions follows the same rules as the expression
argument to $countbits. The return type of $countones is int. The return type of $onehot, $onehot0,
and $isunknown is bit. The functions $countbits, $countones, $onehot, $onehot0, and
$isunknown may be used in any context where a value of their return type is legal. These functions may be
used as constant expressions as specified in 11.2.1 as long as all their arguments are also constant
expressions.
The following example shows how $countbits can be used to replicate common one-hot and related
checks, with special handling possible for 4-valued logic:
// Custom one-hot that, unlike $onehot, fails on any x or z
let my_one_hot_known(myvec) = (
($countones(myvec) == 1) &&
($countbits(myvec,'x,'z) == 0) );
The control bit arguments to $countbits may be variables, as shown in the following example:
logic [1:0] bad_bits;
logic [31:0] myvec;
logic design_initialization_done;

<!-- Page 636 -->

IEEE Std ...
always_comb begin
if (!design_initialization_done) begin
bad_bits[0] = 'x;
bad_bits[1] = 'x; // Repeated control_bit same as single occurrence
end else begin
bad_bits[0] = 'x;
bad_bits[1] = 'z;
end
// z allowed during initialization, but no z or x allowed afterwards
a1: assert ($countbits(myvec,bad_bits[0],bad_bits[1]) == 0);
end
### 20.10 Severity system tasks
severity_system_task ::= // from A.1.4
$fatal [ ( finish_number [, list_of_arguments ] ) ] ;
| $error [ ( [ list_of_arguments ] ) ] ;
| $warning [ ( [ list_of_arguments ] ) ] ;
| $info [ ( [ list_of_arguments ] ) ] ;
finish_number ::= 0 | 1 | 2
Syntax20-11—Severity system task syntax (excerpt from AnnexA)
SystemVerilog provides special text messaging system tasks that can be used to flag various exception
conditions. The tasks are defined as follows:
— $fatal shall generate a run-time fatal error, which terminates the simulation with an error code.
The first argument passed to $fatal shall be consistent with the corresponding argument to the
$finish system task (see 20.2), which sets the level of diagnostic information reported by the tool.
Calling $fatal results in an implicit call to $finish.
— $error shall generate a run-time error.
— $warning shall generate a run-time warning.
— $info shall generate a run-time message of no specific severity.
Each of the severity system tasks can include optional user-defined information to be reported. The user-
defined message shall use the same syntax as the $display system task (see 21.2.1) and thus can include
any number of arguments.
All of the severity system tasks shall print a tool-specific message, indicating the severity of the exception
condition and specific information about the condition, which shall include the following information:
— The file name and line number of the severity system task call. The file name and line number shall
be the same as the `__FILE__ and `__LINE__ compiler directives, respectively (see 22.13).
— The hierarchical name of the scope in which the severity system task call is made.
— For simulation tools, the simulation run time at which the severity system task is called.
The tool-specific message shall include the user-defined message if specified.
assert and assume assertion statements (see Clause16), expect statements (see 16.17) and wait_order
statements (see 15.5.4) contain action_blocks that specify the action(s) to take if the construct succeeds or

<!-- Page 637 -->

IEEE Std fails. If the action_block does not contain an else clause (the fail action), the default fail action is a call to
$error. For assertion and expect statements, this default behavior can be suppressed by a call to
$assertcontrol with control_type 9 (FailOff, see 20.11).
#### 20.10.1 Elaboration severity system tasks
It is often necessary to validate the actual parameter values used in a SystemVerilog model and report any
error without generating the executable simulation model. This is achieved by using elaboration severity
system tasks. These tasks have the same names and syntax as the run-time severity system tasks (see 20.10)
that can be used during simulation. However, the elaboration severity system tasks shall be called outside
procedural code and their activation can be controlled by conditional generate constructs. If such a task is
called from within a procedure, then it becomes a run-time severity system task.
In elaboration severity system tasks, the list_of_arguments may only contain a formatting string and
constant expressions, including constant function calls. If a call to such an elaboration severity system task
remains in the elaborated model after any generate construct expansion, the task is executed. Depending on
the task severity, the elaboration may be aborted or continue to successful completion. If more than one
elaboration severity system task call is present, they may be executed in any order.
If $fatal is executed, then after outputting the message, the elaboration may be aborted, and simulation
shall not be started. Some of the elaboration severity system task calls may not be executed either. The
finish_number may be used in an implementation-specific manner.
If $error is executed, then the message is issued and the elaboration continues. However, simulation shall
not be started.
The other two tasks, $warning and $info, only output their text message but do not affect the rest of the
elaboration and the simulation.
All of the elaboration severity system tasks shall print a tool-specific message, indicating the severity of the
exception condition and specific information about the condition, which shall include the following
information:
— The file name and line number of the elaboration severity system task call. The file name and line
number shall be the same as the `__FILE__ and `__LINE__ compiler directives, respectively (see
22.13).
— The hierarchical name of the scope in which the elaboration severity system task call is made.
The tool-specific message shall include the user-defined message if specified.
Example 1: Sometimes it is desirable to validate elaboration-time constants, such as bounds on a parameter,
in a way that can be enforced during model elaboration. In this example, if the module parameter value is
outside the range 1 to 8, an error is issued and the model elaboration is aborted.
module test #(N = 1) (input [N-1:0] in, output [N-1:0] out);
if ((N < 1) || (N > 8)) // conditional generate construct
$error("Parameter N has an invalid value of %0d", N);
assign out = in;
endmodule
Example 2: In this simple example, the generate construct builds a concatenation (##1) of subsequences,
each of length 1, over a bit from a vector passed as argument to the top sequence definition. An $error
elaboration severity system task is executed if the vector is only a 1-bit vector; otherwise, $info elaboration
severity system tasks issue informational messages that indicate which conditional branches were generated.

<!-- Page 638 -->

IEEE Std generate
if ($bits(vect) == 1)
$error("Only a 1-bit vector");
for (genvar i = 0; i < $bits(vect); i++) begin : Loop
if (i==0) begin : Cond
sequence t;
vect[0];
endsequence
$info("i=0 branch generated");
end : Cond
else begin : Cond
sequence t;
vect[i] ##1 Loop[i-1].Cond.t;
endsequence
$info("i = %0d branch generated", i);
end : Cond
end : Loop
endgenerate
// instantiate the last generated sequence in a property
property p;
@(posedge clk) trig |-> Loop[$bits(vect)-1].Cond.t;
endproperty
### 20.11 Assertion control system tasks
assert_control_task ::=
assert_task [ ( levels [ , list_of_scopes_or_assertions ] ) ] ;
| assert_action_task [ ( levels [ , list_of_scopes_or_assertions ] ) ] ;
| $assertcontrol ( control_type [ , [ assertion_type ] [ , [ directive_type ] [ , [ levels ]
[ , list_of_scopes_or_assertions ] ] ] ] ) ;
assert_task ::=
$asserton
| $assertoff
| $assertkill
assert_action_task ::=
$assertpasson
| $assertpassoff
| $assertfailon
| $assertfailoff
| $assertnonvacuouson
| $assertvacuousoff
list_of_scopes_or_assertions ::= scope_or_assertion { , scope_or_assertion }
scope_or_assertion ::= hierarchical_identifier
Syntax20-12—Assertion control syntax (not in AnnexA)
SystemVerilog provides the $assertcontrol system task to control the evaluation of assertions (see
16.2). The $assertcontrol system task can also be used to control the execution of assertion action
blocks associated with assertions and expect statements. This system task provides the capability to enable/
disable/kill the assertions based on assertion type or directive type. Similarly, this task also provides the
capability to enable/disable action block execution of assertions and expect statements based on assertion

<!-- Page 639 -->

IEEE Std type or directive type. The violation reporting for unique, unique0 and priority if and case constructs
(see 12.4.2 and 12.5.3) can also be controlled using these tasks. The arguments for the $assertcontrol
system task are described as follows:
— control_type: This argument controls the effect of the $assertcontrol system task. This
argument shall be an integer expression. The valid values for this argument are described in
Table20-5.
— assertion_type: This argument selects the assertion types and violation report types that are
affected by the $assertcontrol system task. This argument shall be an integer expression. The
valid values for this argument are described in Table20-6. Multiple assertion_type values can
be specified at a time by OR-ing different values. For example, a task with assertion_type value
of 3 (which is the same as Concurrent|SimpleImmediate) shall apply to concurrent and simple
immediate assertions. Similarly, a task with assertion_type value of 96 (which is the same as
Unique|Unique0) shall apply to unique and unique0 if and case constructs. If
assertion_type is not specified, then it defaults to 255 and the system task applies to all types of
assertions, expect statements, and violation reports.
— directive_type: This argument selects the directive types that are affected by the
$assertcontrol system task. This argument shall be an integer expression. The valid values for
this argument are described in Table20-7. This argument is checked only for assertions. Multiple
directive_type values can be specified at a time by OR-ing different values. For example, a task
with directive_type value of 3 (which is the same as Assert|Cover) shall apply to assert and
cover directives. If directive_type is not specified, then it defaults to 7 (Assert|Cover|Assume)
and the system task applies to all types of directives.
— levels: This argument specifies the levels of hierarchy, consistent with the corresponding
argument to the $dumpvars system task (see 21.7.1.2). If this argument is not specified, it defaults
to 0. This argument shall be an integer expression.
— list_of_scopes_or_assertions: This argument specifies which scopes of the model to
control. These arguments can specify any scopes or individual assertions.
Table20-5—Values for control_type for assertion control tasks
control_type values Effect
1 Lock
2 Unlock
3 On
4 Off
5 Kill
6 PassOn
7 PassOff
8 FailOn
9 FailOff
10 NonvacuousOn
11 VacuousOff

<!-- Page 640 -->

IEEE Std Table20-6—Values for assertion_type for assertion control tasks
assertion_type values Types of assertions affected
1 Concurrent
2 Simple Immediate
4 Observed Deferred Immediate
8 Final Deferred Immediate
16 Expect
32 Unique
64 Unique0
128 Priority
Table20-7—Values for directive_type for assertion control tasks
directive_type values Types of directives affected
1 Assert directives
2 Cover directives
4 Assume directives
The effect of the $assertcontrol system task is determined by the value of its first argument
control_type, which shall be an integer expression. The effect of the system task based on the value of
control_type is described as follows:
— Lock: A value of 1 for this argument prevents status change of all specified assertions, expect
statements, and violation reports until they are unlocked. Once an $assertcontrol with
control_type of value 1 (Lock) is applied to an assertion, expect statement, or violation report,
it becomes locked and no $assertcontrol shall affect it until the locked state is removed by a
subsequent $assertcontrol with a control_type value of 2 (Unlock).
— Unlock: A value of 2 for this argument shall remove the locked status of all specified assertions,
expect statements, and violation reports.
— On: A value of 3 for this argument shall re-enable the execution of all specified assertions. A value
of 3 for this argument shall also re-enable violation reporting from all the specified violation report
types. This control_type value does not affect expect statements.
— Off: A value of 4 for this argument shall stop the checking of all specified assertions until a
subsequent $assertcontrol with a control_type of 3 (On). No new attempts will be started.
Attempts that are already executing for the assertions, and their pass or fail statements, are not
affected. In the case of a deferred assertion (see 16.4), currently queued reports are not flushed and
may still mature, though further checking is prevented until a subsequent $assertcontrol with a
control_type of 3 (On). In the case of a pending procedural assertion instance (see 16.14.6),
currently queued instances are not flushed and may still mature, though no new instances may be
queued until a subsequent $assertcontrol with a control_type of 3 (On). A value of 4 for this
argument shall also disable the violation reporting from all the specified violation report types.

<!-- Page 641 -->

IEEE Std Currently queued violation reports are not flushed and may still mature, though no new violation
reports shall be added to the pending violation report queue until a subsequent $assertcontrol
with a control_type value of 3 (On). The violation reporting can be re-enabled subsequently by
$assertcontrol with a control_type value of 3 (On). This control_type value does not
affect expect statements.
— Kill: A value of 5 for this argument shall abort execution of any currently executing attempts for the
specified assertions and then stop the checking of all specified assertions until a subsequent
$assertcontrol with a control_type of 3 (On). This also flushes any queued pending reports
of deferred assertions (see 16.4) or pending procedural assertion instances (see 16.14.6) that have
not yet matured. A value of 5 for this argument shall also abort violation reporting from all the
specified violation report types. Currently queued violation reports that have not yet matured are
also flushed, and no new violation reports shall be added to the pending violation report queue until
a subsequent $assertcontrol with a control_type value of 3 (On). This control_type value
does not affect expect statements.
— PassOn: A value of 6 for this argument shall enable execution of the pass action for vacuous and
nonvacuous success of all the specified assertions and expect statements. An assertion that is
already executing, including execution of the pass or fail action, is not affected. This
control_type value does not affect violation report types.
— PassOff: A value of 7 for this argument shall stop execution of the pass action for vacuous and
nonvacuous success of all the specified assertions and expect statements. Execution of the pass
action for both vacuous and nonvacuous successes can be re-enabled subsequently by
$assertcontrol with a control_type value of 6 (PassOn), while the execution of the pass
action for only nonvacuous successes can be enabled subsequently by $assertcontrol with a
control_type value of 10 (NonvacuousOn). An assertion that is already executing, including
execution of the pass or fail action, is not affected. By default, the pass action is executed. This
control_type value does not affect violation report types.
— FailOn: A value of 8 for this argument shall enable execution of the fail action of all the specified
assertions and expect statements. An assertion that is already executing, including execution of the
pass or fail action, is not affected. This task also affects the execution of the default fail action block,
i.e., $error, which is called if no else clause is specified for the assertion. This control_type
value does not affect violation report types.
— FailOff: A value of 9 for this argument shall stop execution of the fail action of all the specified
assertions and expect statements until a subsequent $assertcontrol with a control_type
value of 8 (FailOn). An assertion that is already executing, including execution of the pass or fail
action, is not affected. By default, the fail action is executed. This task also affects the execution of
the default fail action block, i.e., $error, which is called if no else clause is specified for the
assertion. This control_type value does not affect violation report types.
— NonvacuousOn: A value of 10 for this argument shall enable execution of the pass action of all the
specified assertions and expect statements on nonvacuous success. An assertion that is already
executing, including execution of the pass or fail action, is not affected. Refer to 16.14.8 for the
definition of vacuous success. This control_type value does not affect violation report types.
— VacuousOff: A value of 11 for this argument shall stop execution of the pass action of all the
specified assertions and expect statements on vacuous success until a subsequent
$assertcontrol with a control_type value of 6 (PassOn). An assertion that is already
executing, including execution of the pass or fail action, is not affected. By default, the pass action is
executed on vacuous success. Refer to 16.14.8 for the definition of vacuous success. This
control_type value does not affect violation report types.
The assertion action control tasks or $assertcontrol with control_type values of 6 (PassOn) to 11
(VacuousOff) do not affect statistics counters for the assertions.
The details related to the behavior of $assertcontrol for assertions referring to global clocking future
sampled value functions are explained in 16.9.4.

<!-- Page 642 -->

IEEE Std The $assertcontrol system task provides finer grain assertion selection controls than the $asserton,
$assertoff, and $assertkill system tasks. The $asserton, $assertoff, and $assertkill system
tasks are provided for convenience and backward compatibility. They can be defined as follows:
— $asserton [ (levels [, list ] ) ] is equivalent to $assertcontrol(3, 15, 7, levels [, list ] )
— $assertoff [ (levels [, list ] ) ] is equivalent to $assertcontrol(4, 15, 7, levels [, list ] )
— $assertkill [ (levels [, list ] ) ] is equivalent to $assertcontrol(5, 15, 7, levels [, list ] )
Similarly, assertion action control tasks $assertpasson, $assertpassoff, $assertfailon,
$assertfailoff, $assertnonvacuouson, and $assertvacuousoff are provided for convenience and
backward compatibility. These tasks can be defined as follows:
— $assertpasson [ (levels [, list ] ) ] is equivalent to $assertcontrol(6, 31, 7, levels [, list ] )
— $assertpassoff [ (levels [, list ] ) ] is equivalent to $assertcontrol(7, 31, 7, levels [, list ] )
— $assertfailon [ (levels [, list ] ) ] is equivalent to $assertcontrol(8, 31, 7, levels [, list ] )
— $assertfailoff [ (levels [, list ] ) ] is equivalent to $assertcontrol(9, 31, 7, levels [, list ] )
— $assertnonvacuouson[(levels[, list ])] is equivalent to $assertcontrol(10, 31, 7, levels [,list])
— $assertvacuousoff [ (levels [, list ] ) ] is equivalent to $assertcontrol(11, 31, 7, levels [,list])
In the following example, assertion control tasks are used to control the directive behavior.
module test;
logic clk;
logic a, b;
logic c, d;
// Define lets to make the code more readable.
let LOCK = 1;
let UNLOCK = 2;
let ON = 3;
let OFF = 4;
let KILL = 5;
let CONCURRENT = 1;
let S_IMMEDIATE = 2; // simple immediate
let D_IMMEDIATE = 12; // Final and Observed deferred immediate
let EXPECT = 16;
let UNIQUE = 32; // unique if and case violation
let UNIQUE0 = 64; // unique0 if and case violation
let PRIORITY = 128; // priority if and case violation
let ASSERT = 1;
let COVER = 2;
let ASSUME = 4;
let ALL_DIRECTIVES = (ASSERT|COVER|ASSUME);
let ALL_ASSERTS = (CONCURRENT|S_IMMEDIATE|D_IMMEDIATE|EXPECT);
let VACUOUSOFF = 11;
a1: assert property (@(posedge clk) a |=> b) $info("assert passed");
else $error("assert failed");
c1: cover property (@(posedge clk) a ##1 b);
always @(posedge clk) begin
ia1: assert (a);
end

<!-- Page 643 -->

IEEE Std always_comb begin
if (c)
df1: assert #0 (d);
unique if ((a==0) || (a==1)) $display("0 or 1");
else if (a == 2) $display("2");
else if (a == 4) $display("4"); // values 3,5,6,7 cause a violation
// report
end
initial begin
// The following systasks affect the whole design so no modules
// are specified
// Disable vacuous pass action for all the concurrent asserts,
// covers and assumes in the design. Also disable vacuous pass
// action for expect statements.
$assertcontrol(VACUOUSOFF, CONCURRENT | EXPECT);
// Disable concurrent and immediate asserts and covers.
// This will also disable violation reporting.
// The following systask does not affect expect
// statements as control type is Off.
$assertcontrol(OFF); // using default values of all the
// arguments after first argument
// After 20 time units, enable assertions,
// This will not enable violation reporting.
// explicitly specifying second, third and fourth arguments
// in the following task call
#20 $assertcontrol(ON, CONCURRENT|S_IMMEDIATE|D_IMMEDIATE,
ASSERT|COVER|ASSUME, 0);
// Enable violation reporting after 20 time units.
#20 $assertcontrol(ON, UNIQUE|UNIQUE0|PRIORITY);
// Kill currently executing concurrent assertions after
// 100 time units but do not kill concurrent covers/assumes
// and immediate/deferred asserts/covers/assumes
// using appropriate values of second and third arguments.
#100 $assertcontrol(KILL, CONCURRENT, ASSERT, 0);
// The following assertion control task does not have any effect as
// directive_type is assert but it has selected cover directive c1.
#10 $assertcontrol(ON, CONCURRENT|S_IMMEDIATE|D_IMMEDIATE, ASSERT, 0,
c1);
// Now, after 10 time units, enable all the assertions except a1.
// To accomplish this, first we’ll lock a1 and then we’ll enable all
// the assertions and then unlock a1 as we want future assertion
// control tasks to affect a1.
#10 $assertcontrol(LOCK, ALL_ASSERTS, ALL_DIRECTIVES, 0, a1);
$assertcontrol(ON); // enable all the assertions except a1
$assertcontrol(UNLOCK, ALL_ASSERTS, ALL_DIRECTIVES, 0, a1);
end
endmodule

<!-- Page 644 -->

IEEE Std Table20-8 lists the VPI callbacks (see 36.9.2 and 39.4) corresponding to the assertion control system task’s
invocation.
Table20-8—VPI callbacks for assertion control tasks
No arguments—assertion system With arguments—assertion callback
Task
callback (see 39.4.1) (see 39.4.2)
$asserton cbAssertionSysOn cbAssertionEnable
$assertoff cbAssertionSysOff cbAssertionDisable
$assertkill cbAssertionSysKill cbAssertionReset +
cbAssertionDisable
$assertpasson cbAssertionSysEnablePassAction cbAssertionEnablePassAction
$assertfailon cbAssertionSysEnableFailAction cbAssertionEnableFailAction
$assertpassoff cbAssertionSysDisablePassAction cbAssertionDisablePassAction
$assertfailoff cbAssertionSysDisableFailAction cbAssertionDisableFailAction
$assertnonvacuouson cbAssertionSysEnableNonvacuousAction cbAssertionEnableNonvacuousAction
$assertvacuousoff cbAssertionSysDisableVacuousAction cbAssertionDisableVacuousAction
$assertcontrol with cbAssertionSysLock cbAssertionLock
control_type 1 (Lock)
$assertcontrol with cbAssertionSysUnlock cbAssertionUnlock
control_type 2
(Unlock)
$assertcontrol with cbAssertionSysOn cbAssertionEnable
control_type 3 (On)
$assertcontrol with cbAssertionSysOff cbAssertionDisable
control_type 4 (Off)
$assertcontrol with cbAssertionSysKill cbAssertionReset + cbAssertionDisable
control_type 5 (Kill)
$assertcontrol with cbAssertionSysEnablePassAction cbAssertionEnablePassAction
control_type 6
(PassOn)
$assertcontrol with cbAssertionSysEnableFailAction cbAssertionEnableFailAction
control_type 8
(FailOn)
$assertcontrol with cbAssertionSysDisablePassAction cbAssertionDisablePassAction
control_type 7
(PassOff)
$assertcontrol with cbAssertionSysDisableFailAction cbAssertionDisableFailAction
control_type 9
(FailOff)
$assertcontrol with cbAssertionSysEnableNonvacuousAction cbAssertionEnableNonvacuousAction
control_type 10
(NonvacuousOn)
$assertcontrol with cbAssertionSysDisableVacuousAction cbAssertionDisableVacuousAction
control_type 11
(VacuousOff)

<!-- Page 645 -->

IEEE Std ### 20.12 Sampled value system functions
sampled_value_function ::=
$sampled ( expression )
| $rose ( expression [ , [ clocking_event ] ] )
| $fell ( expression [ , [ clocking_event ] ] )
| $stable ( expression [ , [ clocking_event ] ] )
| $changed ( expression [ , [ clocking_event ] ] )
| $past ( expression1 [ , [ number_of_ticks ] [ , [ expression2 ] [ , [ clocking_event ] ] ] ] )
global_clocking_past_function ::=
$past_gclk ( expression )
| $rose_gclk ( expression )
| $fell_gclk ( expression )
| $stable_gclk ( expression )
| $changed_gclk ( expression )
global_clocking_future_function ::=
$future_gclk ( expression )
| $rising_gclk ( expression )
| $falling_gclk ( expression )
| $steady_gclk ( expression )
| $changing_gclk ( expression )
Syntax20-13—Sampled value system function syntax (not in AnnexA)
System functions based on sampled values (16.5) and global clocking (14.14) are provided to perform
various temporal calculations. These functions are fully described in other clauses as follows:
Sampled value functions $sampled, $rose, $fell, $stable, $changed, and $past are described in
16.9.3.
Global clocking functions $past_gclk, $rose_gclk, $fell_gclk, $stable_gclk, $changed_gclk,
$future_gclk, $rising_gclk, $falling_gclk, $steady_gclk, and $changing_gclk are described
in 16.9.4.
### 20.13 Coverage system functions
SystemVerilog has several built-in system functions for obtaining test coverage information: $coverage_
control, $coverage_get_max, $coverage_get, $coverage_merge, and $coverage_save. The
coverage system functions are described in 40.3.2.
System tasks and system functions are also provided to help manage coverage data collection and reporting:
$set_coverage_db_name, $load_coverage_db, and $get_coverage. The coverage data system tasks
and system functions are described in 19.9.
### 20.14 Probabilistic distribution functions
Constrained pseudo-random value generation (see Clause18) uses the .randomize method and two special
system functions, $urandom and $urandom_range (see 18.13).

<!-- Page 646 -->

IEEE Std In addition to the constrained random value generation discussed in Clause18, SystemVerilog provides a set
of RNGs that return integer values distributed according to standard probabilistic functions. These are:
$random, $dist_uniform, $dist_normal, $dist_exponential, $dist_poisson,
$dist_chi_square, $dist_t, and $dist_erlang.
The value generation algorithm for these system functions is part of this standard, ensuring repeatable
random value sets across different implementations. The C source code for this algorithm is included in
AnnexN.
#### 20.14.1 $random function
The syntax for the system function $random is shown in Syntax20-14.
random_function ::=
$random [ ( seed ) ]
Syntax20-14—Syntax for $random (not in AnnexA)
The system function $random provides a mechanism for generating random numbers. The function returns
a new 32-bit random number each time it is called. The random number is a signed integer; it can be positive
or negative. For further information on probabilistic RNGs, see 20.14.2.
The seed argument controls the numbers that $random returns so that different seeds generate different
random streams. The seed argument shall be an integral variable. The seed value should be assigned to this
variable prior to calling $random.
Example 1: Where b is greater than 0, the expression ($random % b) gives a number in the following
range: [(-b+1):(b-1)].
The following code fragment shows an example of random number generation between –59 and 59:
int randvar;
randvar = $random % 60;
Example 2: The following example shows how adding the concatenation operator to the preceding example
gives randvar a positive value from 0 to 59:
int randvar;
randvar = {$random} % 60;
#### 20.14.2 Distribution functions
The syntax for the probabilistic distribution functions is shown in Syntax20-15.
dist_functions ::=
$dist_uniform ( seed , start , end )
| $dist_normal ( seed , mean , standard_deviation )
| $dist_exponential ( seed , mean )
| $dist_poisson ( seed , mean )
| $dist_chi_square ( seed , degree_of_freedom )
| $dist_t ( seed , degree_of_freedom )
| $dist_erlang ( seed , k_stage , mean )

<!-- Page 647 -->

IEEE Std Syntax20-15—Syntax for probabilistic distribution functions (not in AnnexA)
All arguments to the system functions are integer values. For the exponential, poisson, chi-square, t, and
erlang functions, the arguments mean, degree_of_freedom, and k_stage shall be greater than 0.
Each of these functions returns a pseudo-random number whose characteristics are described by the function
name. In other words, $dist_uniform returns random numbers uniformly distributed in the interval
specified by its arguments.
For each system function, the seed argument is an inout argument; that is, a value is passed to the
function, and a different value is returned. The system functions shall always return the same value given the
same seed. This facilitates debugging by making the operation of the system repeatable. The seed
argument should be an integral variable that is initialized by the user and only updated by the system
function so that the desired distribution is achieved.
In the $dist_uniform function, the start and end arguments are integer inputs that bound the values
returned. The start value should be smaller than the end value.
The mean argument, used by $dist_normal, $dist_exponential, $dist_poisson, and
$dist_erlang, is an integer input that causes the average value returned by the function to approach the
value specified.
The standard_deviation argument used with the $dist_normal function is an integer input that helps
determine the shape of the density function. Larger numbers for standard_deviation spread the returned
values over a wider range.
The degree_of_freedom argument used with the $dist_chi_square and $dist_t functions is an
integer input that helps determine the shape of the density function. Larger numbers spread the returned
values over a wider range.
### 20.15 Stochastic analysis tasks and functions
This subclause describes a set of system tasks and system functions that manage queues. These tasks
facilitate implementation of stochastic queueing models.
The set of system tasks and system functions that create and manage queues follows:
$q_initialize ( q_id , q_type , max_length , status ) ;
$q_add ( q_id , job_id , inform_id , status ) ;
$q_remove ( q_id , job_id , inform_id , status ) ;
$q_full (q_id , status )
$q_exam (q_id , q_stat_code , q_stat_value , status ) ;

<!-- Page 648 -->

IEEE Std #### 20.15.1 $q_initialize
The $q_initialize system task creates new queues. The q_id argument is an integer input that shall
uniquely identify the new queue. The q_type argument is an integer input. The value of the q_type
argument specifies the type of the queue as shown in Table20-9.
Table20-9—Types of queues of $q_type values
q_type value Type of queue
1 First-in, first-out
2 Last-in, first-out
The max_length argument is an integer input that specifies the maximum number of entries allowed on the
queue. The success or failure of the creation of the queue is returned as an integer value in status. The error
conditions and corresponding values of status are described in Table20-11 in 20.15.6.
#### 20.15.2 $q_add
The $q_add system task places an entry on a queue. The q_id argument is an integer input that indicates to
which queue to add the entry. The job_id argument is an integer input that identifies the job.
The inform_id argument is an integer input that is associated with the queue entry. Its meaning is user-
defined. For example, the inform_id argument can represent execution time for an entry in a CPU model.
The status code reports on the success of the operation or error conditions as described in Table20-11.
#### 20.15.3 $q_remove
The $q_remove system task receives an entry from a queue. The q_id argument is an integer input that
indicates from which queue to remove. The job_id argument is an integer output that identifies the entry
being removed. The inform_id argument is an integer output that the queue manager stored during
$q_add. Its meaning is user-defined. The status code reports on the success of the operation or error
conditions as described in Table20-11.
#### 20.15.4 $q_full
The $q_full system function checks whether there is room for another entry on a queue. It returns 0 when
the queue is not full and 1 when the queue is full. The status code reports on the success of the operation
or error conditions as described in Table20-11.
#### 20.15.5 $q_exam
The $q_exam system task provides statistical information about activity at the queue q_id. It returns a
value in q_stat_value depending on the information requested in q_stat_code. The values of
q_stat_code and the corresponding information returned in q_stat_value are described in Table20-10.

<!-- Page 649 -->

IEEE Std The status code reports on the success of the operation or error conditions as described in Table20-11.
Table20-10—Argument values for $q_exam system task
Value requested in Information received back
q_stat_code from q_stat_value
1 Current queue length
2 Mean interarrival time
3 Maximum queue length
4 Shortest wait time ever
5 Longest wait time for jobs still in the queue
6 Average wait time in the queue
#### 20.15.6 Status codes
All of the queue management tasks and functions return an output status code. The status code values and
corresponding information are described in Table20-11.
Table20-11—Status code values
Status code values What it means
0 OK
1 Queue full, cannot add
2 Undefined q_id
3 Queue empty, cannot remove
4 Unsupported queue type, cannot create queue
5 Specified length <= 0, cannot create queue
6 Duplicate q_id, cannot create queue
7 Not enough memory, cannot create queue
### 20.16 Programmable logic array modeling system tasks
The modeling of programmable logic array (PLA) devices is provided by a group of system tasks. This
subclause describes the syntax and use of these system tasks and the formats of the logic array personality
file. The syntax for PLA modeling system task is shown in Syntax20-16.
pla_system_task ::=
$array_type$logic$format ( memory_identifier , input_terms , output_terms ) ;
array_type ::=
sync | async
logic ::=
and | or | nand | nor
format ::=

<!-- Page 650 -->

IEEE Std array | plane
memory_identifier ::=
identifier
input_terms ::=
expression
output_terms ::=
variable_lvalue
Syntax20-16—Syntax for PLA modeling system task (not in AnnexA)
The input terms can be nets or variables whereas the output terms shall only be variables.
The PLA syntax allows for the system tasks as shown in Table20-12.
Table20-12—PLA modeling system tasks
$async$and$array $sync$and$array $async$and$plane $sync$and$plane
$async$nand$array $sync$nand$array $async$nand$plane $sync$nand$plane
$async$or$array $sync$or$array $async$or$plane $sync$or$plane
$async$nor$array $sync$nor$array $async$nor$plane $sync$nor$plane
#### 20.16.1 Array types
The modeling of both synchronous and asynchronous arrays is provided by the PLA system tasks. The
synchronous forms control the time at which the logic array shall be evaluated and the outputs shall be
updated. For the asynchronous forms, the evaluations are automatically performed whenever an input term
changes value or any word in the personality memory is changed.
For both the synchronous and asynchronous forms, the output terms are updated without any delay.
An example of an asynchronous system call is as follows:
wire a1, a2, a3, a4, a5, a6, a7;
logic b1, b2, b3;
wire [1:7] awire;
logic [1:3] breg;
$async$and$array(mem,{a1,a2,a3,a4,a5,a6,a7},{b1,b2,b3});
or
$async$and$array(mem,awire, breg);
An example of a synchronous system call is as follows:
$sync$or$plane(mem,{a1,a2,a3,a4,a5,a6,a7}, {b1,b2,b3});
#### 20.16.2 Array logic types
The logic arrays are modeled with and, or, nand, and nor logic planes. This applies to all array types and
formats.

<!-- Page 651 -->

IEEE Std An example of a nor plane system call is as follows:
$async$nor$plane(mem,{a1,a2,a3,a4,a5,a6,a7},{b1,b2,b3});
An example of a nand plane system call is as follows:
$sync$nand$plane(mem,{a1,a2,a3,a4,a5,a6,a7}, {b1,b2,b3});
#### 20.16.3 Logic array personality declaration and loading
The logic array personality is declared as an array of variables that is as wide as the number of input terms
and as deep as the number of output terms.
The personality of the logic array is normally loaded into the memory from a text data file using the system
tasks $readmemb or $readmemh (see 21.4). Alternatively, the personality data can be written directly into
the memory using the procedural assignment statements. PLA personalities can be changed dynamically at
any time during simulation simply by changing the contents of the memory. The new personality shall be
reflected on the outputs of the logic array at the next evaluation.
The following example shows a logic array with n input terms and m output terms:
logic [1:n] mem[1:m];
As shown in the examples in 20.16, PLA input terms, output terms, and memory shall be specified in
ascending order.
#### 20.16.4 Logic array personality formats
Two separate personality formats are supported and are differentiated by using either an array system call or
a plane system call. The array system call allows for a 1 or 0 in the memory that has been declared. A
1means take the input value, and a 0 means do not take the input value.
The plane system call complies with the University of California at Berkeley format for Espresso.20 Each bit
of the data stored in the array has the following meaning:
— 0 Take the complemented input value.
— 1 Take the true input value.
— x Take the “worst case” of the input value.
— z Do-not-care; the input value is of no significance.
— ? Same as z.
Example 1: The following example illustrates an array with logic equations:
b1 = a1 & a2
b2 = a3 & a4 & a5
b3 = a5 & a6 & a7
The PLA personality is as follows:
1100000 in mem[1]
0011100 in mem[2]
0000111 in mem[3]
The module for the PLA is as follows:
20Information on Espresso can be found at http://embedded.eecs.berkeley.edu/pubs/downloads/espresso/index.htm.

<!-- Page 652 -->

IEEE Std module async_array(a1,a2,a3,a4,a5,a6,a7,b1,b2,b3);
input a1, a2, a3, a4, a5, a6, a7 ;
output b1, b2, b3;
logic [1:7] mem[1:3]; // memory declaration for array personality
logic b1, b2, b3;
initial begin
// set up the personality from the file array.dat
$readmemb("array.dat", mem);
// set up an asynchronous logic array with the input
// and output terms expressed as concatenations
$async$and$array(mem,{a1,a2,a3,a4,a5,a6,a7},{b1,b2,b3});
end
endmodule
Where the file array.dat contains the binary data for the PLA personality:
0011100
A synchronous version of this example has the following description:
module sync_array(a1,a2,a3,a4,a5,a6,a7,b1,b2,b3,clk);
input a1, a2, a3, a4, a5, a6, a7, clk;
output b1, b2, b3;
logic [1:7] mem[1:3]; // memory declaration
logic b1, b2, b3;
initial begin
// set up the personality
$readmemb("array.dat", mem);
// set up a synchronous logic array to be evaluated
// when a positive edge on the clock occurs
forever @(posedge clk)
$async$and$array(mem,{a1,a2,a3,a4,a5,a6,a7},{b1,b2,b3});
end
endmodule
Example 2: An example of the usage of the plane format tasks follows. The logical function of this PLA is
shown first, followed by the PLA personality in the new format, the SystemVerilog description using the
$async$and$plane system task, and finally the result of running the simulation.
The logical function of the PLA is as follows:
b[1] = a[1] & ~a[2];
b[2] = a[3];
b[3] = ~a[1] & ~a[3];
b[4] = 1;
The PLA personality is as follows:
3'b10?
3'b??1
3'b0?0
3'b???
An example of using the $async$and$plane system task is as follows:

<!-- Page 653 -->

IEEE Std module pla;
`define rows 4
`define cols 3
logic [1:`cols] a, mem[1:`rows];
logic [1:`rows] b;
initial begin
// PLA system call
$async$and$plane(mem,a[1:3],b[1:4]);
mem[1] = 3'b10?;
mem[2] = 3'b??1;
mem[3] = 3'b0?0;
mem[4] = 3'b???;
// stimulus and display
#10 a = 3'b111;
#10 $displayb(a, " -> ", b);
#10 a = 3'b000;
#10 $displayb(a, " -> ", b);
#10 a = 3'bxxx;
#10 $displayb(a, " -> ", b);
#10 a = 3'b101;
#10 $displayb(a, " -> ", b);
end
endmodule
The output is as follows:
111 -> 0101
000 -> 0011
xxx -> xxx1
101 -> 1101
### 20.17 Miscellaneous tasks and functions
#### 20.17.1 $system
The syntax for $system is shown in Syntax20-17.
system_call ::=
$system ( [ " terminal_command_line " ] )
Syntax20-17—$system function syntax (not in AnnexA)
$system makes a call to the C function system(). The C function executes the argument passed to it as if the
argument was executed from the terminal. $system can be called as either a task or a function. When called
as a function, it returns the return value of the call to system() with data type int. If $system is called with
no string argument, the C function system() will be called with the NULL string.
The following example calls $system as a task to rename a file.
module top;
initial $system("mv design.v adder.v");
endmodule

<!-- Page 654 -->

IEEE Std #### 20.17.2 $stacktrace
The $stacktrace system task can be used to retrieve the call stack from the context that is calling
$stacktrace up to the top-level process. $stacktrace can be called as either a task or a function.
The syntax for $stacktrace is shown in Syntax20-18.
stacktrace_call ::= $stacktrace
Syntax20-18—$stacktrace function syntax (not in AnnexA)
When called as a task, $stacktrace displays the call stack information. When called as a function,
$stacktrace returns a string containing the call stack information. The content of the call stack
information is implementation dependent.
For example:
initial begin
string trace;
$stacktrace; // Call stack information is displayed
trace = $stacktrace; // Call stack information is in 'trace' variable
// for later processing
...
end

<!-- Page 655 -->

IEEE Std 1800-2023