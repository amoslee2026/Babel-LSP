# Chapter 21: Input/output system tasks and system functions

## 21. Input/output system tasks and system functions
### 21.1 General
This clause describes input/output (I/O) system tasks and system functions. These system tasks and system
functions are divided into several categories as follows:
Display tasks (21.2) Memory load tasks (21.4)
$display $write $readmemb $readmemh
$displayb $writeb
$displayh $writeh
Memory dump tasks (21.5)
$displayo $writeo
$strobe $monitor $writememb $writememh
$strobeb $monitorb
$strobeh $monitorh Command line input (21.6)
$strobeo $monitoro
$test$plusargs $value$plusargs
$monitoroff
$monitoron
VCD tasks (21.7)
File I/O tasks and functions (21.3) $dumpfile $dumpvars
$dumpoff $dumpon
$fclose $fopen
$dumpall $dumplimit
$fdisplay $fwrite
$dumpflush $dumpports
$fdisplayb $fwriteb
$dumpportsoff $dumpportson
$fdisplayh $fwriteh
$dumpportsall $dumpportslimit
$fdisplayo $fwriteo
$dumpportsflush
$fstrobe $fmonitor
$fstrobeb $fmonitorb
$fstrobeh $fmonitorh
$fstrobeo $fmonitoro
$swrite $sformat
$swriteb $sformatf
$swriteh $fgetc
$swriteo $ungetc
$fscanf $fgets
$fread $sscanf
$fseek $rewind
$fflush $ftell
$feof $ferror
### 21.2 Display system tasks
The display group of system tasks is divided into three categories: the display and write tasks, strobed
monitoring tasks, and continuous monitoring tasks.
#### 21.2.1 The display and write tasks
The syntax forthe $display and $write system tasks is shown in Syntax21-1.

<!-- Page 656 -->

IEEE Std display_tasks ::= display_task_name [ ( list_of_arguments ) ] ;
display_task_name ::=
$display| $displayb | $displayo | $displayh
| $write | $writeb | $writeo | $writeh
Syntax21-1—Syntax for $display and $write system tasks (not in AnnexA)
These are the main system task routines for displaying information. The two sets of tasks are identical
except that $display automatically adds a newline character to the end of its output, whereas the $write
task does not.
The $display and $write tasks display their arguments in the same order as they appear in the argument
list. Each argument can be a string literal or an expression that returns a value.
The contents of string literal arguments are output literally except when certain escape sequences are
inserted to display special characters or to specify the display format for a subsequent expression.
Escape sequences are inserted into a string literal in the following three ways:
— The special character \ indicates that the character to follow shall be interpreted as specified in
Table5-1 in 5.9.1. This is a way to represent special and nonprintable characters.
— The special character % in a string literal indicates that the next character shall be interpreted as a
format specification that establishes the display format for a subsequent expression argument (see
Table21-1). For each % character (except %m, %l, and %%) that appears in a string literal, a
corresponding expression argument shall be supplied after the string literal.
— The special character string %% indicates the display of the percent sign character %. Although escape
sequences beginning with a \ character are represented in a string literal as a single 8-bit ASCII
character, %% is represented by two % ASCII character values.
The following example shows escape sequences using the special characters "\" and "%%":
module disp;
initial begin
$display("\\\t\\\n\"%%\123");
end
endmodule
Simulating this example shall display the following:
\ \
"%S
An empty argument is characterized by a comma at the beginning or end of the argument list or by two
adjacent commas. Any of these commas may be preceded or followed by white space. An empty argument
produces a single space character in the display. An argument list consisting of a single comma is considered
two empty arguments.
For example:
module m;
initial begin
$write("a", );
$write("b",,"c");

<!-- Page 657 -->

IEEE Std $write(,"d");
$write(,);
$write("e");
end
endmodule
displays
a b c d e
The $display task, when invoked without arguments (with or without empty parentheses), simply prints a
newline character. A $write task supplied without arguments prints nothing at all.
21.2.1.1 Format specifications
Table21-1 shows the escape sequences used for format specifications. Each escape sequence, when
included in a string literal argument, specifies the display format for a subsequent expression. For each %
character (except %m, %l, and %%) that appears in a string literal, a corresponding expression shall follow the
string in the argument list. The value of the expression replaces the format specification when the string is
displayed. It shall be an error if an undefined format specifier appears in a string literal argument.
An expression argument that has no corresponding format specification is displayed using the default
decimal format in $display and $write, binary format in $displayb and $writeb, octal format in
$displayo and $writeo, and hexadecimal format in $displayh and $writeh. An expression argument
of a string data type or an unpacked array of byte data type that has no corresponding format
specification shall be formatted as a character string. An expression argument of any other unpacked data
type that has no corresponding format specification shall be illegal.
Table21-1—Escape sequences for format specifications
Argument Description
%h or %H Display in hexadecimal format
%x or %X
%d or %D Display in decimal format
%o or %O Display in octal format
%b or %B Display in binary format
%c or %C Display in ASCII character format
%l or %L Display library binding information
%v or %V Display net signal strength
%m or %M Display hierarchical name
%p or %P Display as an assignment pattern
%s or %S Display as a string
%t or %T Display in current time format
%u or %U Unformatted 2-value data
%z or %Z Unformatted 4-value data

<!-- Page 658 -->

IEEE Std The integer format specifiers, %h, %x, %d, %o, %b, %c, %u, and %z (uppercase or lowercase), may be used with
any of the SystemVerilog integral data types, including enumerated types and packed aggregate data types.
These format specifiers can also be used with user-defined types that have been defined (using typedef) to
be represented using one of these basic types. They shall not be used with any unpacked aggregate type.
The formatting specification %l (or %L) is defined for displaying the library information of the specific
module. This information shall be displayed as "library.cell" corresponding to the library name from
which the current module instance was extracted and the cell name of the current module instance. See
Clause33 for information on libraries and configuring designs.
The formatting specification %u (or %U) is defined for writing data without formatting (binary values). The
application shall transfer the 2-value binary representation of the specified data to the output stream. This
escape sequence can be used with any of the existing display system tasks, although $fwrite (see 21.3.2)
should be the preferred one to use. Any unknown or high-impedance bits in the source shall be treated as
zero. This formatting specifier is intended to be used to support transferring data to and from external
programs that have no concept of x and z. Applications that require preservation of x and z are encouraged
to use the %z I/O format specification.
— For packed data, %u and %z are defined to operate as though the operation were applied to the equiv-
alent vector.
— For unpacked struct data, %u and %z are defined to apply as though the operation were performed on
each member in declaration order.
— For unpacked union data, %u and %z are defined to apply as though the operation were performed on
the first member in declaration order.
— %u and %z are not defined on unpacked arrays.
— The count of data items read by a %u or %z for an aggregate type is always either 1 or 0; the individ-
ual members are not counted separately.
The data shall be written to the file in the native endian format of the underlying system [i.e., in the same
endian order as if the PLI was used and the C language write(2) system call was used]. The data shall be
written in units of 32 bits with the word containing the LSB written first.
NOTE 1—For POSIX applications, it might be necessary to open files for unformatted I/O with the wb, wb+, or w+b
specifiers to avoid the systems implementation of I/O altering patterns in the unformatted stream that match special
characters.
The formatting specification %z (or %Z) is defined for writing data without formatting (binary values). The
application shall transfer the 4-value binary representation of the specified data to the output stream. This
escape sequence can be used with any of the existing display system tasks, although $fwrite (see 21.3.2)
should be the preferred one to use.
This formatting specifier is intended to be used to support transferring data to and from external programs
that recognize and support the concept of x and z. Applications that do not require the preservation of x and
z are encouraged to use the %u I/O format specification.
The data shall be written to the file in the native endian format of the underlying system [i.e., in the same
endian order as if the PLI was used, the data were in a s_vpi_vecval structure (see Figure38-8 in 38.15),
and the C language write(2) system call was used to write the structure to disk]. The data shall be written
in units of 32 bits with the structure containing the LSB written first.
NOTE 2—For POSIX applications, it might be necessary to open files for unformatted I/O with the wb, wb+, or w+b
specifiers to avoid the systems implementation of I/O altering patterns in the unformatted stream that match special
characters.

<!-- Page 659 -->

IEEE Std The format specifications in Table21-2 are used with real numbers (i.e., real and shortreal types) and
have the full formatting capabilities available in the C language. For example, the format specification
%10.3g specifies a minimum field width of 10 with 3 fractional digits.
Table21-2—Format specifications for real numbers
Argument Description
%e or %E Display real numbers in an exponential format
%f or %F Display real numbers in a decimal format
%g or %G Display real numbers in exponential or decimal format,
whichever format results in the shorter printed output
The net signal strength, hierarchical name, assignment pattern, and string format specifications are described
in 21.2.1.4 through 21.2.1.7.
The %t format specification works with the $timeformat system task to specify a uniform time unit, time
precision, and format for reporting timing information from various modules that use different time units
and precisions. The $timeformat task is described in 20.4.3.
For example:
module disp;
logic [31:0] rval;
pulldown (pd);
initial begin
rval = 101;
$display("rval = %h hex %d decimal",rval,rval);
$display("rval = %o octal\nrval = %b bin",rval,rval);
$display("rval has %c ascii character value",rval);
$display("pd strength value is %v",pd);
$display("current scope is %m");
$display("%s is ascii value for 101",101);
$display("simulation time is %t", $time);
end
endmodule
Simulating this example shall display the following:
rval = 00000065 hex 101 decimal
rval = 00000000145 octal
rval = 00000000000000000000000001100101 bin
rval has e ascii character value
pd strength value is StX
current scope is disp
e is ascii value for 101
simulation time is 0
21.2.1.2 Size of displayed data
For expression arguments, the values written to the output file (or terminal) are sized automatically.

<!-- Page 660 -->

IEEE Std For example, the result of a 12-bit expression would be allocated three characters when displayed in
hexadecimal format and four characters when displayed in decimal format because the largest possible value
for the expression is FFF (hexadecimal) and 4095 (decimal).
When displaying decimal values, leading zeros are suppressed and replaced by spaces. In other radices,
leading zeros are always displayed.
The automatic sizing of displayed data can be overridden by inserting a field width between the % character
and the letter that indicates the radix. The field width shall be a non-negative decimal integer constant. If the
field width is 0, the result is displayed in the minimum width, with no leading spaces or zeros, as shown in
the following example:
$display("d=%0h a=%0h", data, addr);
For example:
module printval;
logic [11:0] r1;
initial begin
r1 = 10;
$display( "Printing with maximum size - :%d: :%h:", r1,r1 );
$display( "Printing with minimum size - :%0d: :%0h:", r1,r1 );
end
endmodule
This example will print:
Printing with maximum size - : 10: :00a:
Printing with minimum size - :10: :a:
In this example, the result of a 12-bit expression is displayed. The first call to $display uses the standard
format specifier syntax and produces results requiring four and three columns for the decimal and
hexadecimal radices, respectively. The second $display call uses the %0 form of the format specifier
syntax and produces results requiring two columns and one column, respectively.
If the value to be displayed has fewer characters than the field width, the field is padded on the left to the
length specified by the field width. If the value is wider than the field width, the field is expanded to contain
the converted result. Decimal and string values are expanded with leading spaces while hexadecimal, octal,
and binary values use leading zeros. No truncation occurs.
Examples:
Format Value Displays
%d 32'd10 : 10:
%0d 32'd10 :10:
%h 32'd10 :0000000a:
%0h 32'd10 :a:
%3d 32'd5 : 5:
%3d 32'd100 :100:
%3d 32'd1234 :1234:
%3h 32'h5 :005:
%3h 32'h100 :100:
%3h 32'h1234 :1234:
%s "abc" :abc:
%3s "a" : a:
%3s "abc" :abc:
%3s "abcdef" :abcdef:

<!-- Page 661 -->

IEEE Std 21.2.1.3 Unknown and high-impedance values
When the result of an expression contains an unknown or high-impedance value, certain rules apply to
displaying that value.
In decimal (%d) format, the rules are as follows:
— If all bits are at the unknown value, a single lowercase x character is displayed.
— If all bits are at the high-impedance value, a single lowercase z character is displayed.
— If some, but not all, bits are at the unknown value, the uppercase X character is displayed.
— If some, but not all, bits are at the high-impedance value, the uppercase Z character is displayed,
unless there are also some bits at the unknown value, in which case the uppercase X character is
displayed.
— Decimal numerals always appear right-justified in a fixed-width field.
In hexadecimal (%h, %x) and octal (%o) formats, the rules are as follows:
— Each group of 4 bits is represented as a single hexadecimal digit; each group of 3 bits is represented
as a single octal digit.
— If all bits in a group are at the unknown value, a lowercase x is displayed for that digit.
— If all bits in a group are at a high-impedance state, a lowercase z is printed for that digit.
— If some, but not all, bits in a group are unknown, an uppercase X is displayed for that digit.
— If some, but not all, bits in a group are at a high-impedance state, then an uppercase Z is displayed
for that digit, unless there are also some bits at the unknown value, in which case an uppercase X is
displayed for that digit.
In binary (%b) format, each bit is printed separately using the characters 0, 1, x, and z.
For example:
STATEMENT RESULT
$display("%d", 1'bx); x
$display("%h", 14'bx01010); xxXa
$display("%h %o", 12'b001xxx101x01,
12'b001xxx101x01); XXX 1x5X
21.2.1.4 Strength format
The %v format specification is used to display the strength of scalar nets. For each %v specification that
appears in a string literal, a corresponding scalar reference shall follow the string literal in the argument list.
The strength of a scalar net is reported in a three-character format. The first two characters indicate the
strength. The third character indicates the current logic value of the scalar and can be any one of the values
given in Table21-3.
Table21-3—Logic value component of strength format
Argument Description
0 For a logic 0 value
1 For a logic 1 value
X For an unknown value

<!-- Page 662 -->

IEEE Std Table21-3—Logic value component of strength format (continued)
Z For a high-impedance value
L For a logic 0 or high-impedance value
H For a logic 1 or high-impedance value
The first two characters—the strength characters—are either a two-letter mnemonic or a pair of decimal
digits. Usually, a mnemonic is used to indicate strength information; however, in less typical cases, a pair of
decimal digits can be used to indicate a range of strength levels. Table21-4 shows the mnemonics used to
represent the various strength levels.
Table21-4—Mnemonics for strength levels
Mnemonic Strength name Strength level
Su Supply drive 7
St Strong drive 6
Pu Pull drive 5
La Large capacitor 4
We Weak drive 3
Me Medium capacitor 2
Sm Small capacitor 1
Hi High impedance 0
There are four driving strengths and three charge storage strengths. The driving strengths are associated with
gate outputs and continuous assignment outputs. The charge storage strengths are associated with the
trireg type net. (See Clause28 for strength modeling.)
For the logic values 0 and 1, a mnemonic is used when there is no range of strengths in the signal.
Otherwise, the logic value is preceded by two decimal digits, which indicate the maximum and minimum
strength levels.
For the unknown value, a mnemonic is used when both the 0 and 1 strength components are at the same
strength level. Otherwise, the unknown value X is preceded by two decimal digits, which indicate the 0 and
1 strength levels, respectively.
The high-impedance strength cannot have a known logic value; the only logic value allowed for this level is
Z.
For the values L and H, a mnemonic is always used to indicate the strength level.
For example:
always
#15 $display($time,,"group=%b signals=%v %v %v",{s1,s2,s3},s1,s2,s3);
The following example shows the output that might result from such a call, while Table21-5 explains the
various strength formats that appear in the output.

<!-- Page 663 -->

IEEE Std 0 group=111 signals=St1 Pu1 St1
15 group=011 signals=Pu0 Pu1 St1
30 group=0xz signals=520 PuH HiZ
45 group=0xx signals=Pu0 65X StX
60 group=000 signals=Me0 St0 St0
Table21-5—Explanation of strength formats
Argument Description
St1 A strong driving 1 value
Pu0 A pull driving 0 value
HiZ The high-impedance state
Me0 A 0 charge storage of medium capacitor strength
StX A strong driving unknown value
PuH A pull driving strength of 1 or high-impedance value
65X An unknown value with a strong driving 0 component and a pull driving 1 component
520 An 0 value with a range of possible strength from pull driving to medium capacitor
21.2.1.5 Hierarchical name format
The %m format specifier does not accept an argument. Instead, it causes the display task to print the
hierarchical name of the design element, subroutine, named block, or labeled statement that invokes the
system task containing the format specifier. This is useful when there are many instances of the module that
calls the system task. One obvious application is timing check messages in a flip-flop or latch module; the
%m format specifier pinpoints the module instance responsible for generating the timing check message.
21.2.1.6 Assignment pattern format
The %p format specifier may be used to print aggregate expressions such as unpacked structures, arrays, and
unions. For unpacked structure data types, it shall print the value as an assignment pattern with named
elements. For unions, only the first declared elements shall be printed. In the case of a tagged union, it shall
print “tag:value” along with the currently valid element. For unpacked array data types, it shall print the
value as an assignment pattern, which may include the use of index labels. The use of white space is
implementation dependent; however the output shall be a legal interpretation of the assignment pattern
syntax (see 10.9).
An unpacked data type is traversed until reaching a singular data type. Each element that is a singular type
shall print its value as follows:
— A packed structure data type shall print its value as an assignment pattern with named elements.
Each element shall be printed under one of these rules.
— An enumerated data type shall print its value as an enumeration name if the value is valid for that
type. Otherwise the value shall print according to the base type of the enumeration.
— A string data type or string literal shall print its value as a string enclosed in quotes.
— A chandle, class handle, interface class handle, event, or virtual interface shall print its value in an
implementation-dependent format, except that a null handle value shall print the word null.
— All other singular data types shall print their values as they would unformatted.

<!-- Page 664 -->

IEEE Std An implementation may use a “default” element to reduce its output and may set a limit on the maximum
length of characters output, but that limit shall be at least 1024 characters. If that limit is reached, the output
shall be truncated and a warning issued.
The %0p format specifier may be used to print aggregate expressions such as unpacked structures, arrays,
and unions in a shorter, implementation-specific form. An implementation may set a limit on the maximum
length of characters output as with %p.
The %p and %0p format specifiers can also be used to print singular expressions, in which case the
expression is formatted as an element of an aggregate expression previously described.
For example:
module top;
typedef enum {ON, OFF} switch_e;
typedef struct {switch_e sw; string s;} pair_t;
pair_t va[int] = '{10:'{OFF, "switch10"}, 20:'{ON, "switch20"}};
initial begin
$display("va[int] = %p;",va);
$display("va[int] = %0p;",va);
$display("va[10].s = %p;", va[10].s);
end
endmodule : top
This example may print:
va[int] = '{10:'{sw:OFF, s:"switch10"}, 20:'{sw:ON, s:"switch20"}} ;
va[int] = '{10:'{OFF, "switch10"}, 20:'{ON, "switch20"}} ;
va[10].s = "switch10";
21.2.1.7 String format
The %s format specifier is used to print ASCII codes as characters. For each %s specification that appears in
a string, a corresponding argument shall follow the string in the argument list. The associated argument is
interpreted as a sequence of 8-bit hexadecimal ASCII codes, with each 8 bits representing a single character.
If the argument is a variable, its value should be right-justified so that the rightmost bit of the value is the
LSB of the last character in the string. No termination character or value is required at the end of a string,
and leading zeros are never printed.
The argument corresponding to a %s format specifier may also have a string type or unpacked array of
byte data types. Character ordering of the unpacked array is from left bound to right bound.
#### 21.2.2 Strobed monitoring
The syntax for the $strobe system task is shown in Syntax21-2.
strobe_tasks ::= strobe_task_name [ ( list_of_arguments ) ] ;
strobe_task_name ::= $strobe | $strobeb | $strobeo | $strobeh
Syntax21-2—Syntax for $strobe system tasks (not in AnnexA)

<!-- Page 665 -->

IEEE Std The system task $strobe provides the ability to display simulation data at a selected time. That time is the
end of the current simulation time, when all the simulation events have occurred for that simulation time,
just before simulation time is advanced. The arguments for this task are specified in exactly the same
manner as for the $display system task—including the use of escape sequences for special characters and
format specifications (see 21.2.1).
For example:
forever @(negedge clock)
$strobe ("At time %d, data is %h", $time, data);
In this example, $strobe writes the time and data information to the standard output and the log file at each
negative edge of the clock. The action shall occur just before simulation time is advanced and after all other
events at that time have occurred so that the data written are sure to be the correct data for that simulation
time.
Note that for values that are sampled in the Preponed region (see 16.5.1), a $strobe of a $sampled value
will print the value from the Preponed region of the current time step and not the value from the Preponed
region of the next time step.
For example:
module counter;
int cnt;
bit clk;
initial
forever #10 clk = ~clk;
initial begin
repeat(5) @(posedge clk);
#5 $finish;
end
always_ff @(posedge clk)
cnt <= cnt + 1;
always @(posedge clk) begin
$display("\ndisplay At %0d: cnt=%0h, $sampled cnt=%0h",
$time, cnt, $sampled(cnt));
$strobe ( "strobe At %0d: cnt=%0h, $sampled cnt=%0h",
$realtime, cnt, $sampled(cnt));
end
endmodule: counter
This example will print:
display At 10: cnt=0, $sampled cnt=0
strobe At 10: cnt=1, $sampled cnt=0
display At 30: cnt=1, $sampled cnt=1
strobe At 30: cnt=2, $sampled cnt=1
display At 50: cnt=2, $sampled cnt=2
strobe At 50: cnt=3, $sampled cnt=2
display At 70: cnt=3, $sampled cnt=3

<!-- Page 666 -->

IEEE Std strobe At 70: cnt=4, $sampled cnt=3
display At 90: cnt=4, $sampled cnt=4
strobe At 90: cnt=5, $sampled cnt=4
#### 21.2.3 Continuous monitoring
The syntax for $monitor system task is shown in Syntax21-3.
monitor_tasks ::=
monitor_task_name [ ( list_of_arguments ) ] ;
| $monitoron ;
| $monitoroff ;
monitor_task_name ::= $monitor | $monitorb | $monitoro | $monitorh
Syntax21-3—Syntax for $monitor system tasks (not in AnnexA)
The $monitor task provides the ability to monitor and display the values of any variables or expressions
specified as arguments to the task. The arguments for this task are specified in exactly the same manner as
for the $display system task—including the use of escape sequences for special characters and format
specifications (see 21.2.1).
When a $monitor task is invoked with one or more arguments, the simulator sets up a mechanism whereby
each time a variable or an expression in the argument list changes value—with the exception of the $time,
$stime, or $realtime system functions—the entire argument list is displayed at the end of the time step as
if reported by the $display task. If two or more arguments change value at the same time, only one display
is produced that shows the new values.
Only one $monitor display list can be active at any one time; however, a new $monitor task with a new
display list can be issued any number of times during simulation.
The $monitoron and $monitoroff tasks control a monitor flag that enables and disables the monitoring.
Use $monitoroff to turn off the flag and disable monitoring. The $monitoron system task can be used to
turn on the flag so that monitoring is enabled and the most recent call to $monitor can resume its display.
A call to $monitoron shall produce a display immediately after it is invoked, regardless of whether a value
change has taken place; this is used to establish the initial values at the beginning of a monitoring session.
By default, the monitor flag is turned on at the beginning of simulation.
### 21.3 File input/output system tasks and system functions
The system tasks and system functions for file-based operations are divided into the following categories:
— System tasks and system functions that open and close files
— System tasks that output values into files
— System tasks that output values into variables
— System tasks and system functions that read values from files and load into variables or memories
#### 21.3.1 Opening and closing files
The syntax for the $fopen system function and the $fclose system task is shown in Syntax21-4.

<!-- Page 667 -->

IEEE Std file_open_function ::=
multi_channel_descriptor = $fopen ( filename ) ;
| fd = $fopen ( filename , type ) ;
file_close_task ::=
$fclose ( multi_channel_descriptor ) ;
| $fclose ( fd ) ;
Syntax21-4—Syntax for $fopen and $fclose system tasks (not in AnnexA)
The function $fopen opens the file specified as the filename argument and returns either a 32-bit
multichannel descriptor or a 32-bit file descriptor, determined by the absence or presence of the type
argument.
filename is an expression that is a string literal, string data type, or an integral data type containing a
character string that names the file to be opened.
type is an expression that is a string literal, string data type, or an integral data type containing a character
string of one of the forms in Table21-6 that indicates how the file should be opened. If type is omitted, the
file is opened for writing, and a multichannel descriptor mcd is returned. If type is supplied, the file is
opened as specified by the value of type, and a file descriptor fd is returned.
Table21-6—Types for file descriptors
Argument Description
"r" or "rb" Open for reading
"w" or "wb" Truncate to zero length or create for writing
"a" or "ab" Append; open for writing at end of file (EOF), or create for writing
"r+", "r+b", or "rb+" Open for update (reading and writing)
"w+", "w+b", or "wb+" Truncate or create for update
"a+", "a+b", or "ab+" Append; open or create for update at EOF
The multichannel descriptor mcd is a 32-bit packed array value in which a single bit is set indicating which
file is opened. The LSB (bit 0) of an mcd always refers to the standard output. Output is directed to two or
more files opened with multichannel descriptors by bitwise OR-ing together their mcds and writing to the
resultant value.
The MSB (bit 31) of a multichannel descriptor is reserved and shall always be cleared, limiting an
implementation to at most 31 files opened for output via multichannel descriptors.
The file descriptor fd is a 32-bit packed array value. The MSB (bit 31) of a fd is reserved and shall always
be set; this allows implementations of the file input and output functions to determine how the file was
opened. The remaining bits hold a small number indicating what file is opened. Three file descriptors are
pre-opened; they are STDIN, STDOUT, and STDERR, which have the values 32'h8000_0000,
32'h8000_0001, and 32'h8000_0002, respectively. STDIN is pre-opened for reading, and STDOUT and
STDERR are pre-opened for append.

<!-- Page 668 -->

IEEE Std Unlike multichannel descriptors, file descriptors cannot be combined via bitwise OR in order to direct
output to multiple files. Instead, files are opened via file descriptor for input, output, and both input and
output, as well as for append operations, based on the value of type, according to Table21-6.
If a file cannot be opened (either the file does not exist and the type specified is "r", "rb", "r+", "r+b", or
"rb+", or the permissions do not allow the file to be opened at that path), a zero is returned for the mcd or
fd. Applications can call $ferror to determine the cause of the most recent error (see 21.3.7).
The "b" in the preceding types exists to distinguish binary files from text files. Many systems make no
distinction between binary and text files, and on these systems the "b" is ignored. However, some systems
perform data mappings on certain binary values written to and read from files that are opened for text access.
The $fclose system task closes the file specified by fd or closes the file(s) specified by the multichannel
descriptor mcd. No further output to or input from any file descriptor(s) closed by $fclose is allowed.
Active $fmonitor and/or $fstrobe operations on a file descriptor or multichannel descriptor are
implicitly cancelled by an $fclose operation. The $fopen function shall reuse channels that have been
closed.
NOTE—The number of simultaneous input and output channels that can be open at any one time is dependent on the
operating system. Some operating systems do not support opening files for update.
#### 21.3.2 File output system tasks
The syntax for $fdisplay, $fwrite, $fmonitor, and $fstrobe system tasks is shown in Syntax21-5.
file_output_tasks ::=
file_output_task_name ( multi_channel_descriptor [ , list_of_arguments ] ) ;
| file_output_task_name ( fd [ , list_of_arguments ] ) ;
file_output_task_name ::=
$fdisplay | $fdisplayb | $fdisplayh | $fdisplayo
| $fwrite | $fwriteb | $fwriteh | $fwriteo
| $fstrobe | $fstrobeb | $fstrobeh | $fstrobeo
| $fmonitor | $fmonitorb | $fmonitorh | $fmonitoro
Syntax21-5—Syntax for file output system tasks (not in AnnexA)
Each of the four formatted display tasks—$display, $write, $monitor, and $strobe—has a
counterpart that writes to specific files as opposed to the standard output. These counterpart tasks—
$fdisplay, $fwrite, $fmonitor, and $fstrobe—accept the same type of arguments as the tasks upon
which they are based, with one exception: The first argument shall be either a multichannel descriptor or a
file descriptor, which indicates where to direct the file output. Multichannel descriptors are described in
detail in 21.3.1. A multichannel descriptor is either a variable or the result of an expression that takes the
form of a 32-bit unsigned integer value.
The $fstrobe and $fmonitor system tasks work just like their counterparts, $strobe and $monitor,
except that they write to files using the multichannel descriptor for control. Unlike $monitor, any number
of $fmonitor tasks can be set up to be simultaneously active. However, there is no counterpart to
$monitoron and $monitoroff tasks. The task $fclose is used to cancel an active $fstrobe or
$fmonitor task.
The following example shows how to set up multichannel descriptors. In this example, three different
channels are opened using the $fopen function. The three multichannel descriptors that are returned by the
function are then combined in a bitwise OR operation and assigned to the integer variable messages. The

<!-- Page 669 -->

IEEE Std messages variable can then be used as the first argument in a file output task to direct output to all three
channels at once. To create a descriptor that directs output to the standard output as well, the messages
variable is a bitwise OR with the constant 1, which effectively enables channel 0.
integer
messages, broadcast,
cpu_chann, alu_chann, mem_chann;
initial begin
cpu_chann = $fopen("cpu.dat");
if (cpu_chann == 0) $finish;
alu_chann = $fopen("alu.dat");
if (alu_chann == 0) $finish;
mem_chann = $fopen("mem.dat");
if (mem_chann == 0) $finish;
messages = cpu_chann | alu_chann | mem_chann;
// broadcast includes standard output
broadcast = 1 | messages;
end
The following file output tasks show how the channels opened in the preceding example might be used:
$fdisplay(broadcast, "system reset at time %d", $time);
$fdisplay(messages, "Error occurred on address bus",
" at time %d, address = %h", $time, address);
forever @(posedge clock)
$fdisplay(alu_chann, "acc=%h f=%h a=%h b=%h", acc, f, a, b);
#### 21.3.3 Formatting data to a string
The syntax for the $swrite family of tasks and for $sformat system task is shown in Syntax21-6.
string_output_tasks ::=
string_output_task_name ( output_var [ , list_of_arguments ] ) ;
string_output_task_name ::= $swrite | $swriteb | $swriteh | $swriteo
variable_format_string_output_task ::=
$sformat ( output_var , format_string [ , list_of_arguments ] ) ;
variable_format_string_output_function ::=
$sformatf ( format_string [ , list_of_arguments ] )
Syntax21-6—Syntax for formatting data tasks (not in AnnexA)
The $swrite family of tasks is based on the $fwrite family of tasks and accepts the same type of
arguments as the tasks upon which it is based, with one exception: The first argument to $swrite shall be a
variable of integral, unpacked array of byte, or string data types to which the resulting string shall be
written, instead of a variable specifying the file to which to write the resulting string. Character ordering of
the unpacked array is from left bound to right bound. The variable is assigned using the string literal
assignment to variable rules, as specified in 5.9.
The system task $sformat is similar to the system task $swrite, with one major difference. Unlike the
display and write family of output system tasks, $sformat always interprets its second argument, and only
its second argument, as a format string. This format argument can be a string literal, such as
"data is %d", or may be an expression of integral, unpacked array of byte, or string data types whose

<!-- Page 670 -->

IEEE Std content is interpreted as the formatting string. No other arguments are interpreted as format strings.
$sformat supports all the format specifiers supported by $display, as documented in 21.2.1.1.
The remaining arguments to $sformat, if any, are processed using any format specifiers in the
format_string, until all such format specifiers are used up. If not enough arguments are supplied for the
format specifiers or too many are supplied, then the application shall issue a warning and continue
execution. The application, if possible, can statically determine a mismatch in format specifiers and number
of arguments and issue a compile-time error message.
NOTE—If the format_string is a not a constant expression, it might not be possible to determine its value at compile
time.
The variable output_var is assigned using the string literal assignment to variable rules, as specified in 5.9.
The system function $sformatf behaves like $sformat except that the string result is passed back as the
function result value for $sformatf, not placed in the first argument as for $sformat. Thus $sformatf
can be used where a string value would be valid.
#### 21.3.4 Reading data from a file
file_read_functions ::=
$fgetc ( fd )
| $ungetc ( c , fd )
| $fgets ( str , fd )
| $fscanf ( fd , format , args )
| $sscanf ( str , format , args )
| $fread ( integral_var , fd )
| $fread ( mem , fd [ , [ start ] [ , count ] ] )
Syntax21-7—Syntax for file read system functions (not in AnnexA)
Files opened using file descriptors (fd) can be read from only if they were opened with either the r or r+ type
values. See 21.3.1 for more information about opening files.
21.3.4.1 Reading a character at a time
A single character can be read from a file using $fgetc. For example:
Example 1:
integer c;
c = $fgetc ( fd );
reads a byte from the file specified by fd. If an error occurs reading from the file, then c is set to EOF (-1).
The code defines the width of variable c to be wider than 8 bits so that a return value from $fgetc of EOF
(-1) can be differentiated from the character code 0xFF. Applications can call $ferror to determine the
cause of the most recent error (see 21.3.7).
Example 2:
integer code;
code = $ungetc ( c, fd );

<!-- Page 671 -->

IEEE Std inserts the character specified by c into the buffer specified by file descriptor fd. The character c shall be
returned by the next $fgetc call on that file descriptor. The file itself is unchanged. If an error occurs
pushing a character onto a file descriptor, then code is set to EOF. Otherwise, code is set to zero.
Applications can call $ferror to determine the cause of the most recent error (see 21.3.7).
NOTE—The features of the underlying implementation of file I/O on the host system limit the number of characters that
can be pushed back onto a stream. Operations like $fseek might erase any pushed back characters.
21.3.4.2 Reading a line at a time
One line can be read from a file using $fgets. For example:
integer code;
code = $fgets ( str, fd );
reads characters from the file specified by fd into the variable str until str is filled, or a newline character
is read and transferred to str, or an EOF condition is encountered. If str is not an integral number of bytes
in length, the most significant partial byte is not used in order to determine the size.
If an error occurs reading from the file, then code is set to zero. Otherwise, the number of characters read is
returned in code. Applications can call $ferror to determine the cause of the most recent error (see
21.3.7).
21.3.4.3 Reading formatted data
The $fscanf system function can be used to format data as it is read from a file. For example:
integer code ;
code = $fscanf ( fd, format, args );
code = $sscanf ( str, format, args );
$fscanf reads from the files specified by the file descriptor fd.
$sscanf reads from the argument str, which may be an expression of integral, unpacked array of byte, or
string data type.
Both functions read characters, interpret them according to a format, and store the results. Both expect as
arguments a control string, format, and a set of arguments specifying where to place the results. If there are
insufficient arguments for the format, the behavior is undefined. If the format is exhausted while arguments
remain, the excess arguments are ignored.
If an argument is too small to hold the converted input, then, in general, the LSBs are transferred. Arguments
of any length that are supported by SystemVerilog can be used. However, if the destination is real,
shortreal, or realtime, then the value +Inf (or -Inf) is transferred. The format can be an expression
containing a string, string data type, or an integral data type. The string contains conversion specifications,
which direct the conversion of input into the arguments. The control string can contain the following:
a) White space characters (blanks, tabs, newlines, or formfeeds) that, except in one case described
below, cause input to be read up to the next nonwhite space character. For $sscanf, null characters
shall also be considered white space.
b) An ordinary character (not %) that shall match the next character of the input stream.
c) Conversion specifications consisting of the character %, an optional assignment suppression
character *, a decimal digit string that specifies an optional numerical maximum field width, and a
conversion code.

<!-- Page 672 -->

IEEE Std A conversion specification directs the conversion of the next input field; the result is placed in the variable
specified in the corresponding argument unless assignment suppression was indicated by the character *. In
this case, no argument shall be supplied.
The suppression of assignment provides a way of describing an input field that is to be skipped. An input
field is defined as a string of nonspace characters; it extends to the next inappropriate character or until the
maximum field width, if one is specified, is exhausted. For all descriptors except the character c, white
space leading an input field is ignored. Table21-7 describes the input field characters for $fscanf.
The integer format specifiers, %h (or %H), %d (or %D), %o (or %O), %b (or %B), %c (or %C), %u (or %U), and %z
(or %Z), may be used to read into any of the integral data types, including enumerated data types and packed
aggregate data types. These format specifiers can also be used with user-defined data types that have been
defined (using typedef) to be represented using one of these basic types. They shall not be used with any
unpacked aggregate data type.
Table21-7—$fscanf input field characters
Input
Description
field
% A single % is expected in the input at this point; no assignment is done.
b Matches a binary number, consisting of a sequence from the set 0,1,X,x,Z,z,?, and _.
o Matches a octal number, consisting of a sequence of characters from the set 0,1,2,3,4,5,6,7,X,x,Z,z,?,
and _.
d Matches an optionally signed decimal number, consisting of the optional sign from the set + or –,
followed by a sequence of characters from the set 0,1,2,3,4,5,6,7,8,9, and _, or a single value from the
set x,X,z,Z,?.
h, x Matches a hexadecimal number, consisting of a sequence of characters from the set
0,1,2,3,4,5,6,7,8,9,a,A,b,B,c,C,d,D,e,E,f,F,x,X,z,Z,?, and _.
f, e, g Matches a floating-point number. The format of a floating-point number is an optional sign (either + or
–), followed by a string of digits from the set 0,1,2,3,4,5,6,7,8,9 optionally containing a decimal point
character (.), followed by an optional exponent part including e or E, followed by an optional sign,
followed by a string of digits from the set 0,1,2,3,4,5,6,7,8,9.
v Matches a net signal strength, consisting of a three-character sequence as specified in 21.2.1.4. This
conversion is not extremely useful, as strength values are really only usefully assigned to nets and
$fscanf can only assign values to integral variables (if assigned to integral variables, the values are
converted to the 4-value equivalent).
t Matches a floating-point number. The format of a floating-point number is an optional sign (either + or
–), followed by a string of digits from the set 0,1,2,3,4,5,6,7,8,9 optionally containing a decimal point
character (.), followed by an optional exponent part including e or E, followed by an optional sign,
followed by a string of digits from the set 0,1,2,3,4,5,6,7,8,9. The value matched is then scaled and
rounded according to the current timescale as set by $timeformat. For example, if the timescale is
`timescale 1ns/100ps and the time format is $timeformat(–3,2," ms",10);, then a
value read with $sscanf("10.345", "%t", t) would return 10350000.0.
c Matches a single character, whose 8-bit ASCII value is returned.
s Matches a string, which is a sequence of nonwhite space characters.

<!-- Page 673 -->

IEEE Std Table21-7—$fscanf input field characters (continued)
Input
Description
field
u Matches unformatted (binary) data. The application shall transfer sufficient data from the input to fill
the target variable. Typically, the data are obtained from a matching $fwrite ("%u",data) or
from an external application written in another programming language such as C, Perl, or FORTRAN.
The application shall transfer the 2-value binary data from the input stream to the destination variable,
expanding the data to the 4-value format. This escape sequence can be used with any of the existing
input system tasks, although $fscanf should be the preferred one to use. As the input data cannot
represent x or z, it is not possible to obtain an x or z in the result variable. This formatting specifier is
intended to be used to support transferring data to and from external programs that have no concept of
x and z.
Applications that require preservation of x and z are encouraged to use the %z I/O format
specification.
The data shall be read from the file in the native endian format of the underlying system [i.e., in the
same endian order as if the PLI was used and the C language read(2) system call was used].
For POSIX applications, it might be necessary to open files for unformatted I/O with the "rb",
"rb+", or "r+b" specifiers to avoid the systems implementation of I/O altering patterns in the
unformatted stream that match special characters.
z The formatting specification %z (or %Z) is defined for reading data without formatting (binary values).
The application shall transfer the 4-value binary representation of the specified data from the input
stream to the destination variable. This escape sequence can be used with any of the existing input
system tasks, although $fscanf should be the preferred one to use.
This formatting specifier is intended to be used to support transferring data to and from external
programs that recognize and support the concept of x and z. Applications that do not require the
preservation of x and z are encouraged to use the %u I/O format specification.
The data shall be read from the file in the native endian format of the underlying system [i.e., in the
same endian order as if the PLI was used, the data were in a s_vpi_vecval structure (see
Figure38-8 in 38.15), and the C language read(2) system call was used to read the data from disk].
For POSIX applications, it might be necessary to open files for unformatted I/O with the "rb",
"rb+", or "r+b" specifiers to avoid the systems implementation of I/O altering patterns in the unfor-
matted stream that match special characters.
m Returns the current hierarchical path as a string. Does not read data from the input file or str
argument.
The string format specifier %s (or %S) may be used to read into a variable of integral, unpacked array of
byte, or string data types.
If an invalid conversion character follows the %, the results of the operation are implementation dependent.
If the format string or the str argument to $sscanf contains unknown bits (x or z), then the system
function shall return EOF (-1).
If EOF is encountered during input, conversion is terminated. If EOF occurs before any characters matching
the current directive have been read (other than leading white space, where permitted), execution of the
current directive terminates with an input failure. Otherwise, unless execution of the current directive is
terminated with a matching failure, execution of the following directive (if any) is terminated with an input
failure.

<!-- Page 674 -->

IEEE Std If conversion terminates on a conflicting input character, the offending input character is left unread in the
input stream. Trailing white space (including newline characters) is left unread unless matched by a
directive. The success of literal matches and suppressed assignments is not directly determinable.
The number of successfully matched and assigned input items is returned in code; this number can be 0 in
the event of an early matching failure between an input character and the control string. If the input ends
before the first matching failure or conversion, EOF (-1) is returned. Applications can call $ferror to
determine the cause of the most recent error (see 21.3.7).
21.3.4.4 Reading binary data
$fread can be used to read binary data from a file. For example:
integer code ;
code = $fread( integral_var, fd);
code = $fread( mem, fd);
code = $fread( mem, fd, start);
code = $fread( mem, fd, start, count);
code = $fread( mem, fd, , count);
read binary data from the file specified by fd into the variable integral_var or the memory mem.
The integral variable variant,
$fread(integral_var, fd);
is defined to be the one applied for all packed data.
start is an optional argument. If present, start shall be used as the address of the first element in the
memory to be loaded. If not present, the lowest numbered location in the memory shall be used.
count is an optional argument. If present, count shall be the maximum number of locations in mem that
shall be loaded. If not supplied, the memory shall be filled with what data are available.
start and count are ignored if $fread is loading an integral variable.
If no addressing information is specified within the system task and no address specifications appear within
the data file, then the default start address is the lowest address given in the declaration of the memory.
Consecutive words are loaded toward the highest address until either the memory is full or the data file is
completely read. If the start address is specified in the task without the finish address, then loading starts at
the specified start address and continues toward the highest address given in the declaration of the memory.
start is the address in the memory. For start = 12 and the memory up[10:20], the first data would be
loaded at up[12]. For the memory down[20:10], the first location loaded would be down[12], then
down[13].
The data in the file shall be read byte by byte to fulfill the request. An 8-bit wide memory is loaded using
1byte per memory word, while a 9-bit wide memory is loaded using 2 bytes per memory word. The data are
read from the file in a big endian manner; the first byte read is used to fill the most significant location in the
memory element. If the memory width is not evenly divisible by 8 (8, 16, 24, 32), not all data in the file are
loaded into memory because of truncation.
For unpacked struct data, $fread is defined to apply as though the operation were performed on each
member in declaration order.

<!-- Page 675 -->

IEEE Std For unpacked union data, $fread is defined to apply as though the operation were performed on the first
member in declaration order.
The data loaded from the file are taken as 2-value data. A bit set in the data is interpreted as a 1, and bit not
set is interpreted as a 0. It is not possible to read a value of x or z using $fread.
If an error occurs reading from the file, then code is set to zero. Otherwise, the number of characters read is
returned in code. Applications can call $ferror to determine the cause of the most recent error (see 21.3.7).
NOTE—There is not a “binary” mode and an “ASCII” mode; one can freely intermingle binary and formatted read
commands from the same file.
#### 21.3.5 File positioning
file_positioning_functions ::=
$ftell ( fd )
| $fseek ( fd , offset , operation )
| $rewind ( fd )
Syntax21-8—Syntax for file positioning system functions (not in AnnexA)
$ftell is used to determine the current read or write position within a file. For example:
integer pos ;
pos = $ftell ( fd );
returns in pos the offset from the beginning of the file of the current byte of the file fd, which shall be read
or written by a subsequent operation on that file descriptor.
This value can be used by subsequent $fseek calls to reposition the file to this point. Any repositioning
shall cancel any $ungetc operations. If an error occurs, EOF (-1) is returned. Applications can call
$ferror to determine the cause of the most recent error (see 21.3.7).
$fseek and $rewind can be used to change the current read or write position within a file. For example:
integer code ;
code = $fseek ( fd, offset, operation );
code = $rewind ( fd );
sets the position of the next input or output operation on the file specified by fd. The new position is at the
signed distance offset bytes from the beginning, from the current position, or from the end of the file,
according to an operation value of 0, 1, and 2, as follows:
— 0 sets position equal to offset bytes
— 1 sets position to current location plus offset
— 2 sets position to EOF plus offset
$rewind is equivalent to $fseek (fd,0,0);
Repositioning the current file position with $fseek or $rewind shall cancel any $ungetc operations.
$fseek() allows the file position indicator to be set beyond the end of the existing data in the file. If data
are later written at this point, subsequent reads of data in the gap shall return zero until data are actually
written into the gap. $fseek, by itself, does not extend the size of the file.

<!-- Page 676 -->

IEEE Std When a file is opened for append (that is, when type is "a" or "a+"), it is impossible to overwrite
information already in the file. $fseek can be used to reposition the file pointer to any position in the file,
but when output is written to the file, the current file pointer is disregarded. All output is written at the end of
the file and causes the file pointer to be repositioned at the end of the output.
If an error occurs repositioning the file, then code is set to –1. Otherwise, code is set to 0. Applications can
call $ferror to determine the cause of the most recent error (see 21.3.7).
#### 21.3.6 Flushing output
file_flush_task ::= $fflush ( [ mcd | fd ] ) ;
Syntax21-9—Syntax for file flush system task (not in AnnexA)
The file I/O buffer can be flushed used $fflush. For example:
$fflush ( mcd );
$fflush ( fd );
$fflush ( );
writes any buffered output to the file(s) specified by mcd, to the file specified by fd, or if $fflush is
invoked with no arguments, to all open files.
#### 21.3.7 I/O error status
file_error_detect_function ::= $ferror ( fd , str )
Syntax21-10—Syntax for file I/O error detection system function (not in AnnexA)
Should any error be detected by one of the file I/O routines, an error code is returned. Often this is sufficient
for normal operation (i.e., if the opening of an optional configuration file fails, the application typically
would simply continue using default values). However, sometimes it is useful to obtain more information
about the error for correct application operation. In this case, the $ferror function can be used, as follows:
integer errno ;
errno = $ferror ( fd, str );
A string description of the type of error encountered by the most recent file I/O operation is written into the
variable str, which should be a packed array of at least 640 bits wide or a string type. The integral value
of the error code is returned in errno. If the most recent operation did not result in an error, then the value
returned shall be zero, and the str variable shall be cleared.
#### 21.3.8 Detecting EOF
file_eof_detect_function ::= $feof ( fd )
Syntax21-11—Syntax for end-of-file file detection system function (not in AnnexA)
EOF can be tested for using $feof. For example:

<!-- Page 677 -->

IEEE Std integer code;
code = $feof ( fd );
returns a nonzero value when EOF has previously been detected reading the input file fd. It returns zero
otherwise.
### 21.4 Loading memory array data from a file
Two system tasks—$readmemb and $readmemh—read and load data from a specified text file into a
specified memory array (see 7.4.3). The syntax for $readmemb and $readmemh system tasks is shown in
Syntax21-12.
load_memory_tasks ::=
$readmemb ( filename , memory_name [ , start_addr [ , finish_addr ] ] ) ;
| $readmemh ( filename , memory_name [ , start_addr [ , finish_addr ] ] ) ;
Syntax21-12—Syntax for memory load system tasks (not in AnnexA)
The filename is an expression that is a string literal, string data type, or an integral data type containing a
character string that names the file to be opened.
The memory_name can be an unpacked array or a partially indexed multidimensional unpacked array that
resolves to a lesser dimensioned unpacked array (see 21.4.3). Higher order dimensions shall be specified
with an index, rather than a complete or partial dimension range. The lowest dimension (i.e., the rightmost
specified dimension in the identifier) can be specified with slice syntax. See 7.4.5 for details on legal array
indexing in SystemVerilog.
The start_addr and finish_addr arguments apply to the addresses of the unpacked array selected by
memory_name. This address range represents the highest dimension of data in the filename.
When slice syntax is used in the memory_name argument, any start_addr and finish_addr arguments shall
fall within the bounds of the slice’s range.
The direction of the highest dimension’s file entries is given by the relative magnitudes of start_addr and
finish_addr.
These tasks can be executed at any time during simulation.
The text file to be read shall contain only the following:
— White space (spaces, newlines, tabs, and formfeeds)
— Comments (both types of comment are allowed)
— Binary or hexadecimal numbers
The numbers shall have neither the length nor the base format specified. For $readmemb, each number shall
be binary. For $readmemh, the numbers shall be hexadecimal. The unknown value (x or X), the high-
impedance value (z or Z), and the underscore ( _ ) can be used in specifying a number as in a SystemVerilog
source description. White space and/or comments shall be used to separate the numbers.
In the following discussion, the term address refers to an index into the array that models the memory.

<!-- Page 678 -->

IEEE Std As the file is read, each number encountered is assigned to a successive word element of the memory.
Addressing is controlled both by specifying start and/or finish addresses in the system task invocation and
by specifying addresses in the data file.
When addresses appear in the data file, the format is an at character (@) followed by a hexadecimal number,
as follows:
@hh...h
Both uppercase and lowercase digits are allowed in the number. No white space is allowed between the @
and the number. As many address specifications as needed within the data file can be used. When the system
task encounters an address specification, it loads subsequent data starting at that memory address.
If no addressing information is specified within the system task and no address specifications appear within
the data file, then the default start address shall be the lowest address in the memory. Consecutive words
shall be loaded until either the highest address in the memory is reached or the data file is completely read. If
the start address is specified in the task without the finish address, then loading shall start at the specified
start address and shall continue upward toward the highest address in the memory. In both cases, loading
shall continue upward even after an address specification in the data file.
If both start and finish addresses are specified as arguments to the task, then loading shall begin at the start
address and shall continue toward the finish address. If the start address is greater than the finish address,
then the address will be decremented between consecutive loads rather than being incremented. Loading
shall continue to follow this direction even after an address specification in the data file.
When addressing information is specified both in the system task and in the data file, the addresses in the
data file shall be within the address range specified by the system task arguments; otherwise, an error
message is issued, and the load operation is terminated.
A warning shall be issued if the number of data words in the file differs from the number of words in the
range implied by the start through finish addresses and no address specifications appear within the data file.
In this case, the data words that are contained in the file shall be loaded into the memory beginning at the
start address, and memory addresses for which the file does not contain data words are not modified by the
operation.
For example:
logic [7:0] mem[1:256];
Given this declaration, each of the following statements load data into mem in a different manner:
initial $readmemh("mem.data", mem);
initial $readmemh("mem.data", mem, 16);
initial $readmemh("mem.data", mem, 128, 1);
The first statement loads up the memory at simulation time 0 starting at the memory address 1. The second
statement begins loading at address 16 and continues on toward address 256. For the third and final
statement, loading begins at address 128 and continues down toward address 1.
In the third case, when loading is complete, a final check is performed to verify that exactly 128 numbers are
contained in the file. If the check fails, a warning is issued.

<!-- Page 679 -->

IEEE Std #### 21.4.1 Reading packed data
$readmemb and $readmemh support unpacked arrays of packed data. In such cases, the system tasks treat
each packed element as the vector equivalent and perform the normal operation.
When loading dynamic arrays and queues, the current size of the array is fixed; the array shall not be resized.
When loading associative arrays, indices shall be of integral types. Loading an address creates an element of
that index, if it does not already exist. When an associative array’s index is of an enumerated type, address
entries in the pattern file are in numeric format and correspond to the numeric values associated with the
elements of the enumerated type.
#### 21.4.2 Reading 2-state types
$readmemb and $readmemh support packed data of 2-state types, such as int or enumerated types. For
2-state integer types, reading proceeds the same as for 4-state variable types (e.g., integer), with the
exception that x or z data are converted to 0. For enumerated types, the file data represents the numeric
values associated with each element of the enumerated type (see 6.19). If a numeric value is out of range for
a given type, then an error shall be issued and no further reading shall take place.
#### 21.4.3 File format considerations for multidimensional unpacked arrays
The $readmemb and $readmemh system tasks (and the $writememb and $writememh tasks) can work
with multidimensional unpacked arrays.
The file contents are organized in row-major order, with each dimension’s entries ranging from low to high
address. This is backward compatible with one-dimensional memory arrays.
In this organization, the lowest dimension (i.e., the rightmost dimension in the array declaration) varies the
most rapidly. There is a hierarchical sense to the file data. The higher dimensions contain words of lower
dimension data, sorted in row-major order. Each successive lower dimension is entirely enclosed as part of
higher dimension words.
As an example of file format organization, here is the layout of a file representing words for a memory
declared:
logic [31:0] mem [0:2][0:4][5:8];
In the example word contents, wzyx,
— z corresponds to words of the [0:2] dimension.
— y corresponds to words of the [0:4] dimension.
— x corresponds to words of the [5:8] dimension.
w005 w006 w007 w008
w015 w016 w017 w018
w025 w026 w027 w028
w035 w036 w037 w038
w045 w046 w047 w048
w105 w106 w107 w108
w115 w116 w117 w118
w125 w126 w127 w128
w135 w136 w137 w138
w145 w146 w147 w148
w205 w206 w207 w208

<!-- Page 680 -->

IEEE Std w215 w216 w217 w218
w225 w226 w227 w228
w235 w236 w237 w238
w245 w246 w247 w248
The preceding diagram would be identical if one or more of the unpacked dimension declarations were
reversed, as in the following:
logic [31:0] mem [2:0][0:4][8:5]
Address entries in the file exclusively address the highest dimension’s words. In the preceding case, address
entries in the file could look something as follows:
@0 w005 w006 w007 w008
w015 w016 w017 w018
w025 w026 w027 w028
w035 w036 w037 w038
w045 w046 w047 w048
@1 w105 w106 w107 w108
w115 w116 w117 w118
w125 w126 w127 w128
w135 w136 w137 w138
w145 w146 w147 w148
@2 w205 w206 w207 w208
w215 w216 w217 w218
w225 w226 w227 w228
w235 w236 w237 w238
w245 w246 w247 w248
When $readmemh or $readmemb is given a file without address entries, all data are read assuming that each
dimension has complete data. i.e., each word in each dimension will be initialized with the appropriate value
from the file. If the file contains incomplete data, the read operation will stop at the last initialized word, and
any remaining array words or subwords will be left unchanged.
When $readmemh or $readmemb is given a file with address entries, initialization of the specified highest
dimension words is done. If the file contains insufficient words to completely fill a highest dimension word,
then the remaining subwords are left unchanged.
When a memory contains multiple packed dimensions, the memory words in the pattern file are composed
of the sum total of all bits in the packed dimensions. The layout of packed bits in packed dimensions is
defined in 7.4.4.
### 21.5 Writing memory array data to a file
The $writememb and $writememh system tasks can be used to dump memory array (see 7.4.3) contents to
files that are readable by $readmemb and $readmemh, respectively.
writemem_tasks ::=
$writememb ( filename , memory_name [ , start_addr [ , finish_addr ] ] ) ;
| $writememh ( filename , memory_name [ , start_addr [ , finish_addr ] ] ) ;
Syntax21-13—$writemem system task syntax (not in AnnexA)

<!-- Page 681 -->

IEEE Std If filename exists at the time $writememb or $writememh is called, the file will be overwritten (i.e., there
is no append mode).
#### 21.5.1 Writing packed data
$writememb and $writememh treat packed data identically to $readmemb and $readmemh. See 21.4.1.
#### 21.5.2 Writing 2-state types
$writememb and $writememh can write out data corresponding to unpacked arrays of 2-state types, such
as int or enumerated types. For enumerated types, values in the file correspond to the ordinal values of the
enumerated type (see 6.19).
#### 21.5.3 Writing addresses to output file
When $writememb and $writememh write out data corresponding to unpacked or dynamic arrays, address
specifiers (@-words) shall not be written to the output file.
When $writememb and $writememh write out data corresponding to associative arrays, address specifiers
shall be written to the output file. As specified in 21.4.1, associative arrays shall have indices of integral
types in order to be legal arguments to the $writememb and $writememh calls.
### 21.6 Command line input
An alternative to reading a file to obtain information for use in the simulation is specifying information with
the command to invoke the simulator. This information is in the form of an optional argument provided to
the simulation. These arguments are visually distinguished from other simulator arguments by their starting
with the plus (+) character.
These arguments, referred to below as plusargs, are accessible through the following system functions:
$test$plusargs ( string )
$value$plusargs ( user_string, variable )
The $test$plusargs system function searches the list of plusargs for a user-specified plusarg_string. The
string is specified in the argument to the system function as either a string or an integral variable that is
interpreted as a string. If a variable is used to specify the string, leading nulls in the variable shall be ignored
and shall not be considered as part of the matching string. This string shall not include the leading plus sign
of the command line argument. The plusargs present on the command line are searched in the order
provided. If the prefix of one of the supplied plusargs matches all characters in the provided string, the
function returns a nonzero integer. If no plusarg from the command line matches the string provided, the
function returns the integer value zero.
For example:
Run simulator with command: +HELLO
initial begin
if ($test$plusargs("HELLO")) $display("Hello argument found.");
if ($test$plusargs("HE")) $display("The HE subset string is detected.");
if ($test$plusargs("H")) $display("Argument starting with H found.");
if ($test$plusargs("HELLO_HERE")) $display("Long argument.");
if ($test$plusargs("HI")) $display("Simple greeting.");
if ($test$plusargs("LO")) $display("Does not match.");

<!-- Page 682 -->

IEEE Std end
This code would produce the following output:
Hello argument found.
The HE subset string is detected.
Argument starting with H found.
The $value$plusargs system function searches the list of plusargs (like the $test$plusargs system
function) for a user-specified plusarg_string. The string is specified in the first argument to the system
function as either a string or an integral variable that is interpreted as a string. If a variable is used to specify
the string, leading nulls in the variable shall be ignored and shall not be considered as part of the matching
string. This string shall not include the leading plus sign of the command line argument. The plusargs
present on the command line are searched in the order provided. If the prefix of one of the supplied plusargs
matches all characters in the provided string, the function returns a nonzero integer, the remainder of the
string is converted to the type specified in the user_string, and the resulting value is stored in the variable
provided. If no string is found matching, the function returns the integer value zero, and the variable
provided is not modified. No warnings shall be generated when the function returns zero (0).
The user_string shall be of the following form: "plusarg_string format_string". The format strings are the
same as the $display system tasks. The following are the only valid format strings (uppercase and
lowercase as well as leading 0 forms are valid):
%d decimal conversion
%o octal conversion
%h, %x hexadecimal conversion
%b binary conversion
%e real exponential conversion
%f real decimal conversion
%g real decimal or exponential conversion
%s string (no conversion)
The first string from the list of plusargs provided to the simulator, which matches the plusarg_string portion
of the user_string specified, shall be the plusarg string available for conversion. The remainder string of the
matching plusarg (the remainder is the part of the plusarg string after the portion that matches the user’s
plusarg_string) shall be converted from a string into the format indicated by the format string and stored in
the variable provided. If there is no remaining string, the value stored into the variable shall be either a zero
or an empty string value.
If the size of the variable is larger than the value after conversion, the value stored is zero-padded to the
width of the variable. If the variable cannot contain the value after conversion, the value shall be truncated.
If the value is negative, the value shall be considered larger than the variable provided. If characters exist in
the string available for conversion that are illegal for the specified conversion, the variable shall be written
with the value 'bx.
Given the SystemVerilog code:
`define STRING logic [1024 * 8:1]
module goodtasks;
`STRING str;
integer i1;
logic [31:0] vect;
real realvar;

<!-- Page 683 -->

IEEE Std initial
begin
if ($value$plusargs("TEST=%d", i1))
$display("value was %d", i1);
else
$display("+TEST= not found");
#100 $finish;
end
endmodule
module ieee1364_example;
real frequency;
logic [8*32:1] testname;
logic [64*8:1] pstring;
logic clk;
initial
begin
if ($value$plusargs("TESTNAME=%s",testname))
begin
$display(" TESTNAME= %s.",testname);
$finish;
end
if (!($value$plusargs("FREQ+%0F",frequency)))
frequency = 8.33333; // 166 MHz
$display("frequency = %f",frequency);
pstring = "TEST%d";
if ($value$plusargs(pstring, testname))
$display("Running test number %0d.",testname);
end
endmodule
and adding to the tool’s command line the plusarg
+TEST=5
will result in the following output:
value was 5
frequency = 8.333330
Running text number x.
Adding to the tool’s command line the plusarg
+TESTNAME=t1
will result in the following output:
+TEST= not found
TESTNAME= t1.
Adding to the tool’s command line the plusarg
+FREQ+9.234
will result in the following output:

<!-- Page 684 -->

IEEE Std +TEST= not found
frequency = 9.234000
Adding to the tool’s command line the plusarg
+TEST23
will result in the following output:
+TEST= not found
frequency = 8.333330
Running test number 23.
### 21.7 Value change dump (VCD) files
A VCD file contains information about value changes on selected variables in the design stored by VCD
system tasks. The following two types of VCD files exist:
a) 4-state: to represent variable changes in 0, 1, x, and z with no strength information.
b) Extended: to represent variable changes in all states and strength information.
This subclause describes how to generate both types of VCD files and their format.
#### 21.7.1 Creating 4-state VCD file
The steps involved in creating the 4-state VCD file are listed as follows and illustrated in Figure21-1.
a) Insert the VCD system tasks in the SystemVerilog source file to define the dump file name and to
specify the variables to be dumped.
b) Run the simulation.
SystemVerilog Source File 4-State VCD File
dump1.dump
initial
(Header
$dumpfile("dump1.dump"); Information)
. User
. simulation
(Node
. Postprocessing
$dumpvars(...); Information)
.
. (Value
. Changes)
Figure21-1—Creating the 4-state VCD file
A VCD file is an ASCII file that contains header information, variable definitions, and the value changes for
all variables specified in the task calls.
Several system tasks can be inserted in the source description to create and control the VCD file.

<!-- Page 685 -->

IEEE Std 21.7.1.1 Specifying name of dump file ($dumpfile)
The $dumpfile task shall be used to specify the name of the VCD file. The syntax for the task is given in
Syntax21-14.
dumpfile_task ::=
$dumpfile ( filename ) ;
Syntax21-14—Syntax for $dumpfile task (not in AnnexA)
The filename is an expression that is a string literal, string data type, or an integral data type containing a
character string that names the file to be opened. The filename is optional and defaults to the string literal
"dump.vcd" if not specified.
For example:
initial $dumpfile ("module1.dump") ;
21.7.1.2 Specifying variables to be dumped ($dumpvars)
The $dumpvars task shall be used to list which variables to dump into the file specified by $dumpfile.
The $dumpvars task can be invoked as often as desired throughout the model (for example, within various
blocks), but the execution of all the $dumpvars tasks shall be at the same simulation time.
The $dumpvars task can be used with or without arguments. The syntax for the $dumpvars task is given
in Syntax21-15.
dumpvars_task ::=
$dumpvars ;
| $dumpvars ( levels [ , list_of_modules_or_variables ] ) ;
list_of_modules_or_variables ::=
module_or_variable { , module_or_variable }
module_or_variable ::=
module_identifier
| variable_identifier
Syntax21-15—Syntax for $dumpvars task (not in AnnexA)
When invoked with no arguments, $dumpvars dumps all the variables in the model to the VCD file.
When the $dumpvars task is specified with arguments, the first argument indicates how many levels of the
hierarchy below each specified module instance to dump to the VCD file. Subsequent arguments specify
which scopes of the model to dump to the VCD file. These arguments can specify entire modules or
individual variables within a module.
Setting the first argument to 0 causes a dump of all variables in the specified module and in all module
instances below the specified module. The argument 0 applies only to subsequent arguments that specify
module instances, and not to individual variables.
Example 1:

<!-- Page 686 -->

IEEE Std $dumpvars (1, top);
Because the first argument is a 1, this invocation dumps all variables within the module top; it does not
dump variables in any of the modules instantiated by module top.
Example 2:
$dumpvars (0, top);
In this example, the $dumpvars task shall dump all variables in the module top and in all module instances
below module top in the hierarchy.
Example 3: This example shows how the $dumpvars task can specify both modules and individual
variables.
$dumpvars (0, top.mod1, top.mod2.net1);
This call shall dump all variables in module mod1 and in all module instances below mod1, along with
variable net1 in module mod2. The argument 0 applies only to the module instance top.mod1 and not to
the individual variable top.mod2.net1.
21.7.1.3 Stopping and resuming the dump ($dumpoff/$dumpon)
Executing the $dumpvars task causes the value change dumping to start at the end of the current simulation
time unit. To suspend the dump, the $dumpoff task can be invoked. To resume the dump, the $dumpon task
can be invoked. The syntax of these two tasks is given in Syntax21-16.
dumpoff_task ::=
$dumpoff ;
dumpon_task ::=
$dumpon ;
Syntax21-16—Syntax for $dumpoff and $dumpon tasks (not in AnnexA)
When the $dumpoff task is executed, a checkpoint is made in which every selected variable is dumped as
an x value. When the $dumpon task is later executed, each variable is dumped with its value at that time. In
the interval between $dumpoff and $dumpon, no value changes are dumped.
The $dumpoff and $dumpon tasks provide the mechanism to control the simulation period during which the
dump shall take place.
For example:
initial begin
#10 $dumpvars( . . . );
#200 $dumpoff;
#800 $dumpon;
#900 $dumpoff;
end

<!-- Page 687 -->

IEEE Std This example starts the VCD after 10 time units, stops it 200 time units later (at time 210), restarts it again
800 time units later (at time 1010), and stops it 900 time units later (at time 1910).
21.7.1.4 Generating a checkpoint ($dumpall)
The $dumpall task creates a checkpoint in the VCD file that shows the current value of all selected
variables. The syntax is given in Syntax21-17.
dumpall_task ::=
$dumpall ;
Syntax21-17—Syntax for $dumpall task (not in AnnexA)
When dumping is enabled, the value change dumper records the values of the variables that change during
each time increment. Values of variables that do not change during a time increment are not dumped.
21.7.1.5 Limiting size of dump file ($dumplimit)
The $dumplimit task can be used to set the size of the VCD file. The syntax for this task is given in
Syntax21-18.
dumplimit_task ::=
$dumplimit ( filesize ) ;
Syntax21-18—Syntax for $dumplimit task (not in AnnexA)
The filesize argument specifies the maximum size of the VCD file in bytes. When the size of the VCD file
reaches this number of bytes, the dumping stops, and a comment is inserted in the VCD file indicating the
dump limit was reached.
21.7.1.6 Reading dump file during simulation ($dumpflush)
The $dumpflush task can be used to empty the VCD file buffer of the operating system to verify all the
data in that buffer are stored in the VCD file. After executing a $dumpflush task, dumping is resumed as
before so no value changes are lost. The syntax for the task is given in Syntax21-19.
dumpflush_task ::=
$dumpflush ;
Syntax21-19—Syntax for $dumpflush task (not in AnnexA)
A common application is to call $dumpflush to update the dump file so an application program can read
the VCD file during a simulation.
Example 1: This example shows how the $dumpflush task can be used in a SystemVerilog source file.
initial begin
$dumpvars ;
.
.

<!-- Page 688 -->

IEEE Std .
$dumpflush ;
$(applications program) ;
end
Example 2: The following is a simple source description example to produce a VCD file:
In this example, the name of the dump file is verilog.dump. It dumps value changes for all variables in the
model. Dumping begins when an event do_dump occurs. The dumping continues for 500 clock cycles and
then stops and waits for the event do_dump to be triggered again. At every 10 000 time steps, the current
values of all VCD variables are dumped.
module dump;
event do_dump;
initial $dumpfile("verilog.dump");
initial @do_dump
$dumpvars; //dump variables in the design
always @do_dump //to begin the dump at event do_dump
begin
$dumpon; //no effect the first time through
repeat (500) @(posedge clock); //dump for 500 cycles
$dumpoff; //stop the dump
end
initial @(do_dump)
forever #10000 $dumpall; // checkpoint all variables
endmodule
#### 21.7.2 Format of 4-state VCD file
The dump file is structured in a free format. White space is used to separate commands and to make the file
easily readable by a text editor.
21.7.2.1 Syntax of 4-state VCD file
The syntax of the 4-state VCD file is given in Syntax21-20.
value_change_dump_definitions ::=
{ declaration_command }{ simulation_command }
declaration_command ::=
$comment [ comment_text ] $end
| $date [ date_text ] $end
| $enddefinitions $end
| $scope [ scope_type scope_identifier ] $end
| $timescale [ time_number time_unit ] $end
| $upscope $end
| $var [ var_type size identifier_code reference ] $end
| $version [ version_text system_task ] $end
simulation_command ::=

<!-- Page 689 -->

IEEE Std $dumpall { value_change } $end
| $dumpoff { value_change } $end
| $dumpon { value_change } $end
| $dumpvars { value_change } $end
| $comment [ comment_text ] $end
| simulation_time
| value_change
scope_type ::=
begin
| fork
| function
| module
| task
time_number ::= 1 | 10 | 100
time_unit ::= s | ms | us | ns | ps | fs
var_type ::=
event | integer | parameter | real | realtime | reg | supply0 | supply1 | time
| tri | triand | trior | trireg | tri0 | tri1 | wand | wire | wor
simulation_time ::= # decimal_number
value_change ::=
scalar_value_change
| vector_value_change
scalar_value_change ::= value identifier_code
value ::= 0 | 1 | x | X | z | Z
vector_value_change ::=
b binary_number identifier_code
| B binary_number identifier_code
| r real_number identifier_code
| R real_number identifier_code
identifier_code ::= { ASCII character }
size ::= decimal_number
reference ::=
identifier
| identifier [ bit_select_index ]
| identifier [ msb_index : lsb_index ]
index ::= decimal_number
scope_identifier ::= { ASCII character }
comment_text ::= { ASCII character }
date_text ::= { ASCII character }
version_text ::= { ASCII character }
system_task ::= ${ASCII character}
Syntax21-20—Syntax for output 4-state VCD file (not in AnnexA)
The VCD file starts with header information giving the date, the version number of the simulator used for
the simulation, and the timescale used. Next, the file contains definitions of the scope and type of variables
being dumped, followed by the actual value changes at each simulation time increment. Only the variables
that change value during a time increment are listed.

<!-- Page 690 -->

IEEE Std The simulation time recorded in the VCD file is the absolute value of the simulation time for the changes in
variable values that follow.
Value changes for real variables are specified by real numbers. Value changes for all other variables are
specified in binary format by 0, 1, x, or z values. Strength information and memories are not dumped.
A real number is dumped using a %.16g printf() format. This preserves the precision of that number by
outputting all 53 bits in the mantissa of a 64-bit IEEE Std 754 double-precision number. Application
programs can read a real number using a %g format to scanf().
The value change dumper generates character identifier codes to represent variables. The identifier code is a
code composed of the printable characters except white space, which are in the ASCII character set from ! to
~ (decimal 33 to 126).
The VCD format does not support a mechanism to dump part of a vector. For example, bits 8 to 15
([8:15]) of a 16-bit vector cannot be dumped in VCD file; instead, the entire vector ([0:15]) has to be
dumped. In addition, expressions, such as a + b, cannot be dumped in the VCD file.
Data in the VCD file are case sensitive.
21.7.2.2 Formats of variable values
Variables can be either scalars or vectors. Each type is dumped in its own format. Dumps of value changes
to scalar variables shall not have any white space between the value and the identifier code.
Dumps of value changes to vectors shall not have any white space between the base letter and the value
digits, but they shall have one white space between the value digits and the identifier code.
The output format for each value is right-justified. Vector values appear in the shortest form possible:
redundant bit values that result from left-extending values to fill a particular vector size are eliminated.
The rules for left-extending vector values are given in Table21-8.
Table21-8—Rules for left-extending vector values
When the value is VCD left-extends with
1 0
0 0
Z Z
X X
Table21-9 shows how the VCD can shorten values.
Table21-9—How the VCD can shorten values
Extends to fill a Appears in the
Binary value
4-bit reg as VCD file as
10 0010 b10
X10 XX10 bX10

<!-- Page 691 -->

IEEE Std Table21-9—How the VCD can shorten values (continued)
Extends to fill a Appears in the
Binary value
4-bit reg as VCD file as
ZX0 ZZX0 bZX0
0X10 0X10 b0X10
Events are dumped in the same format as scalars; for example, 1*%. For events, however, the value (1 in this
example) is irrelevant. Only the identifier code (*% in this example) is significant. It appears in the VCD file
as a marker to indicate the event was triggered during the time step.
For example:
1*@ No space between the value 1 and the identifier code *@
b1100x01z (k No space between the b and 1100x01z,
but a space between b1100x01z and (k
21.7.2.3 Description of keyword commands
The general information in the VCD file is presented as a series of sections surrounded by keywords.
Keyword commands provide a means of inserting information in the VCD file. Keyword commands can be
inserted either by the dumper or manually.
This subclause deals with the keyword commands given in Table21-10.
Table21-10—Keyword commands
Declaration keywords Simulation keywords
$comment $timescale $dumpall
$date $upscope $dumpoff
$enddefinitions $var $dumpon
$scope $version $dumpvars
The $comment section provides a means of inserting a comment in the VCD file. For example:
$comment This is a single-line comment $end
$comment This is a
multiple-line comment
$end
The $date section indicates the date on which the VCD file was generated. For example:
$date
June 25, 1989 09:24:35
$end
The $enddefinitions section marks the end of the header information and definitions.

<!-- Page 692 -->

IEEE Std The $scope section defines the scope of the variables being dumped. The scope type indicates one of the
following scopes:
module Top-level module and module instances
task Tasks
function Functions
begin Named sequential blocks
fork Named parallel blocks
For example:
$scope
module top
$end
The $timescale keyword specifies what timescale was used for the simulation. For example:
$timescale 10 ns $end
The $upscope section indicates a change of scope to the next higher level in the design hierarchy.
The $var section prints the names and identifier codes of the variables being dumped. The size specifies
how many bits are in the variable. The identifiercode specifies the name of the variable using printable
ASCII characters, as previously described.
a) The msbindex indicates the most significant index; the lsbindex indicates the least significant
index.
b) More than one reference name can be mapped to the same identifier code. For example, net10 and
net15 can be interconnected in the circuit and, therefore, have the same identifier code.
c) The individual bits of vector nets can be dumped individually.
d) The identifier is the name of the variable being dumped in the model.
In the $var section, a net of net type uwire shall have a var_type of wire.
For example:
$var
integer 32 (2 index
$end
The $version section indicates which version of the VCD writer was used to produce the VCD file and the
$dumpfile system task used to create the file. If a variable or an expression was used to specify the
filename within $dumpfile, the unevaluated variable or expression literal shall appear in the $version
string. For example:
$version
VERILOG-SIMULATOR 1.0a
$dumpfile("dump1.dump")
$end
The $dumpall keyword specifies current values of all variables dumped. For example:
$dumpall 1*@ x*# 0*$ bx (k $end
The $dumpoff keyword indicates all variables dumped with x values. For example:

<!-- Page 693 -->

IEEE Std $dumpoff x*@ x*# x*$ bx (k $end
The $dumpon keyword indicates resumption of dumping and lists current values of all variables dumped.
For example:
$dumpon x*@ 0*# x*$ b1 (k $end
The section beginning with $dumpvars keyword lists initial values of all variables dumped. For example:
$dumpvars x*@ z*$ b0 (k $end
21.7.2.4 4-state VCD file format example
The following example illustrates the format of the 4-state VCD file:
$date June 26, 1989 10:05:41
$end
$version VERILOG-SIMULATOR 1.0a
$end
$timescale 1 ns
$end
$scope module top $end
$scope module m1 $end
$var trireg 1 *@ net1 $end
$var trireg 1 *# net2 $end
$var trireg 1 *$ net3 $end
$upscope $end
$scope task t1 $end
$var reg 32 (k accumulator[31:0] $end
$var integer 32 {2 index $end
$upscope $end
$upscope $end
$enddefinitions $end
$comment
$dumpvars was executed at time '#500'.
All initial values are dumped at this time.
$end
#500
$dumpvars
x*@
x*#
x*$
bx (k
bx {2
$end
#505
0*@
1*#
1*$
b10zx1110x11100 (k
b1111000101z01x {2
#510
0*$
#520
1*$
#530
0*$

<!-- Page 694 -->

IEEE Std bz (k
#535
$dumpall 0*@ 1*# 0*$
bz (k
b1111000101z01x {2
$end
#540
1*$
#1000
$dumpoff
x*@
x*#
x*$
bx (k
bx {2
$end
#2000
$dumpon
z*@
1*#
0*$
b0 (k
bx {2
$end
#2010
1*$
#### 21.7.3 Creating extended VCD file
The steps involved in creating the extended VCD file are listed as follows and illustrated in Figure21-2.
a) Insert the extended VCD system tasks in the SystemVerilog source file to define the dump file name
and to specify the variables to be dumped.
b) Run the simulation.
SystemVerilog Source File Extended VCD File
dump2.dump
initial
(Header
$dumpports(,"dump2.dump"); Information)
. User
. simulation
(Node
. Postprocessing
Information)
.
. (Value
. Changes)
Figure21-2—Creating the extended VCD file
The 4-state VCD file rules and syntax apply to the extended VCD file unless otherwise stated in this
subclause.
21.7.3.1 Specifying dump file name and ports to be dumped ($dumpports)
The $dumpports task shall be used to specify the name of the VCD file and the ports to be dumped. The
syntax for the task is given in Syntax21-21.

<!-- Page 695 -->

IEEE Std dumpports_task ::=
$dumpports ( scope_list , filename ) ;
scope_list ::=
module_identifier { , module_identfier }
Syntax21-21—Syntax for $dumpports task (not in AnnexA)
The arguments are optional and are defined as follows:
The scope_list is one or more module_identifiers. Only modules are allowed (not variables). If more than
one module_identifier is specified, they shall be separated by a comma. Path names to modules are allowed,
using the period hierarchy separator. String literals are not allowed for the module_identifier. If no
scope_list value is provided, the scope shall be the module from which $dumpports is called.
The filename is an expression that is a string literal, string data type, or an integral data type containing a
character string that denotes the file that shall contain the port VCD information. If no filename is provided,
the file shall be written to the current working directory with the name dumpports.vcd. If that file already
exists, it shall be silently overwritten. All file-writing checks shall be made by the simulator (e.g., write
rights, correct path name) and appropriate errors or warnings issued.
The following rules apply to the use of the $dumpports system task:
— All the ports in the model from the point of the $dumpports call are considered primary I/O pins
and shall be included in the VCD file. However, any ports that exist in instantiations below
scope_list are not dumped.
— If no arguments are specified for the task, $dumpports; and $dumpports(); are allowed. In both
of these cases, the default values for the arguments shall be used.
— If the first argument is null, a comma shall be used before specifying the second argument in the
argument list.
— Each scope specified in the scope_list shall be unique. If multiple calls to $dumpports are
specified, the scope_list values in these calls shall also be unique.
— The $dumpports task can be used in source code that also contains the $dumpvars task.
— When $dumpports executes, the associated value change dumping shall start at the end of the
current simulation time unit.
— The $dumpports task can be invoked multiple times throughout the model, but the execution of all
$dumpports tasks shall be at the same simulation time. Specifying the same filename multiple
times is not allowed.
21.7.3.2 Stopping and resuming the dump ($dumpportsoff/$dumpportson)
The $dumpportsoff and $dumpportson system tasks provide a means to control the simulation period
for dumping port values. The syntax for these system tasks is given in Syntax21-22.
dumpportsoff_task ::=
$dumpportsoff ( filename ) ;
dumpportson_task ::=
$dumpportson ( filename ) ;
Syntax21-22—Syntax for $dumpportsoff and $dumpportson system tasks (not in AnnexA)

<!-- Page 696 -->

IEEE Std The filename is an expression that is a string literal, string data type, or an integral data type containing a
character string that denotes the filename specified in the $dumpports system task.
When the $dumpportsoff task is executed, a checkpoint is made in the filename where each specified port
is dumped with an x value. Port values are no longer dumped from that simulation time forward. If filename
is not specified, all dumping to files opened by $dumpports calls shall be suspended.
When the $dumpportson task is executed, all ports specified by the associated $dumpports call shall have
their values dumped. This system task is typically used to resume dumping after the execution of
$dumpportsoff. If filename is not specified, dumping shall resume for all files specified by $dumpports
calls, if dumping to those files was stopped.
If $dumpportson is executed while ports are already being dumped to filename, the system task is ignored.
If $dumpportsoff is executed while port dumping is already suspended for filename, the system task is
ignored.
21.7.3.3 Generating a checkpoint ($dumpportsall)
The $dumpportsall system task creates a checkpoint in the VCD file that shows the value of all selected
ports at that time in the simulation, regardless of whether the port values have changed since the last time
step. The syntax for this system task is given in Syntax21-23.
dumpportsall_task ::=
$dumpportsall ( filename ) ;
Syntax21-23—Syntax for $dumpportsall system task (not in AnnexA)
The filename is an expression that is a string literal, string data type, or an integral data type containing a
character string that denotes the filename specified in the $dumpports system task.
If the filename is not specified, checkpointing occurs for all files opened by calls to $dumpports.
21.7.3.4 Limiting size of dump file ($dumpportslimit)
The $dumpportslimit system task allows control of the VCD file size. The syntax for this system task is
given in Syntax21-24.
dumpportslimit_task ::=
$dumpportslimit ( filesize , filename ) ;
Syntax21-24—Syntax for $dumpportslimit system task (not in AnnexA)
The filesize integer argument is required, and it specifies the maximum size in bytes for the associated
filename. When this filesize is reached, the dumping stops, and a comment is inserted into filename
indicating the size limit was attained.
The filename is an expression that is a string literal, string data type, or an integral data type containing a
character string that denotes the filename specified in the $dumpports system task.
If the filename is not specified, the filesize limit applies to all files opened for dumping due to calls to
$dumpports.

<!-- Page 697 -->

IEEE Std 21.7.3.5 Reading dump file during simulation ($dumpportsflush)
To facilitate performance, simulators often buffer VCD output and write to the file at intervals, instead of
line by line. The $dumpportsflush system task writes all port values to the associated file, clearing a
simulator’s VCD buffer.
The syntax for this system task is given in Syntax21-25.
dumpportsflush_task ::=
$dumpportsflush ( filename ) ;
Syntax21-25—Syntax for $dumpportsflush system task (not in AnnexA)
The filename is an expression that is a string literal, string data type, or an integral data type containing a
character string that denotes the filename specified in the $dumpports system task.
If the filename is not specified, the VCD buffers shall be flushed for all files opened by calls to
$dumpports.
21.7.3.6 Description of keyword commands
The general information in the extended VCD file is presented as a series of sections surrounded by
keywords. Keyword commands provide a means of inserting information in the extended VCD file.
Keyword commands can be inserted either by the dumper or manually. Extended VCD provides one
additional keyword command to that of the 4-state VCD.
21.7.3.6.1 $vcdclose
The $vcdclose keyword indicates the final simulation time at the time the extended VCD file is closed.
This allows accurate recording of the end simulation time, regardless of the state of signal changes, in order
to assist parsers that require this information. The syntax for the keyword is given in Syntax21-26.
vcdclose_task ::=
$vcdclose final_simulation_time $end
Syntax21-26—Syntax for $vcdclose keyword (not in AnnexA)
For example:
$vcdclose #13000 $end
21.7.3.7 General rules for extended VCD system tasks
For each extended VCD system task, the following rules apply:
— If a filename is specified that does not match a filename specified in a $dumpports call, the control
task shall be ignored.
— If no arguments are specified for the tasks that have only optional arguments, the system task name
can be used with no arguments or the name followed by ( ) can be specified, for example,
$dumpportsflush or $dumpportsflush(). In both of these cases, the default actions for the
arguments shall be executed.

<!-- Page 698 -->

IEEE Std #### 21.7.4 Format of extended VCD file
The format of the extended VCD file is similar to that of the 4-state VCD file, as it is also structured in a free
format. White space is used to separate commands and to make the file easily readable by a text editor.
21.7.4.1 Syntax of extended VCD file
The syntax of the extended VCD file is given in Syntax21-27. A 4-state VCD construct name that matches
an extended VCD construct shall be considered equivalent, except if preceded by an *.
value_change_dump_definitions ::= {declaration_command} {simulation_command}
declaration_command ::= declaration_keyword [command_text] $end
simulation_command ::=
simulation_keyword { value_change } $end
| $comment [comment_text] $end
| simulation_time
| value_change
declaration_keyword ::=
$comment | $date | $enddefinitions | $scope | $timescale | $upscope | $var | $vcdclose
| $version
command_text ::=
comment_text | close_text | date_section | scope_section | timescale_section | var_section
| version_section
simulation_keyword ::= $dumpports | $dumpportsoff | $dumpportson | $dumpportsall
simulation_time ::= #decimal_number
final_simulation_time ::= simulation_time
value_change ::= value identifier_code
value ::= pport_value 0_strength_component 1_strength_component
port_value ::= input_value | output_value | unknown_direction_value
input_value ::= D | U | N | Z | d | u
output_value ::= L | H | X | T | l | h
unknown_direction_value ::= 0 | 1 | ? | F | A | a | B | b | C | c | f
strength_component ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7
identifier_code ::= <{integer}
comment_text ::= {ASCII_character}
close_text ::= final_simulation_time
date_section ::= date_text
date_text :: = day month date time year
scope_section ::= scope_type scope_identifier
scope_type ::= module
timescale_section ::= number time_unit
number ::= 1 | 10 | 100
time_unit ::= fs | ps | ns | us | ms | s
var_section ::= var_type size identifier_code reference
var_type ::= port
size ::= 1 | vector_index

<!-- Page 699 -->

IEEE Std vector_index ::= [ msb_index : lsb_index ]
index ::= decimal_number
msb_index ::= index
lsb_index ::= index
reference ::= port_identifier
identifier ::= { any_printable_ASCII_character_except_white_space }
version_section ::= version_text
version_text ::= version_identifier {dumpports_command}
dumpports_command ::= $dumpports (scope_identifier , string_literal | variable | expression )
Syntax21-27—Syntax for output extended VCD file (not in AnnexA)
The extended VCD file starts with header information giving the date, the version number of the simulator
used for the simulation, and the timescale used. Next, the file contains definitions of the scope of the ports
being dumped, followed by the actual value changes at each simulation time increment. Only the ports that
change value during a time increment are listed.
The simulation time recorded in the extended VCD file is the absolute value of the simulation time for the
changes in port values that follow.
Value changes for all ports are specified in binary format by 0, 1, x, or z values and include strength
information.
A real number is dumped using a %.16g printf() format. This preserves the precision of that number by
outputting all 53 bits in the mantissa of a 64-bit IEEE Std 754 double-precision number. Application
programs can read a real number using a %g format to scanf().
The extended VCD format does not support a mechanism to dump part of a vector. For example, bits 8 to 15
([8:15]) of a 16-bit vector cannot be dumped in VCD file; instead, the entire vector ([0:15]) has to be
dumped. In addition, expressions, such as a + b, cannot be dumped in the VCD file.
Data in the extended VCD file are case sensitive.
21.7.4.2 Extended VCD node information
The node information section (also referred to as the variable definitions section) is affected by the
$dumpports task as Syntax21-28 shows.
$var var_type size < identifier_code reference $end
var_type ::= port
size ::= 1 | vector_index
vector_index ::= [ msb_index : lsb_index ]
index ::= decimal_number
identifier_code ::= integer
reference ::= port_identifier
Syntax21-28—Syntax for extended VCD node information (not in AnnexA)
The constructs are defined as follows:

<!-- Page 700 -->

IEEE Std var_type The keyword port. No other keyword is allowed.
size A decimal number indicating the number of bits in the port. If the port is a single
bit, the value shall be 1. If the port is a bus, the actual index is printed. The msb
indicates the most significant index; lsb, the least significant index.
identifier_code An integer preceded by <, which starts at zero and ascends in one-unit increments
for each port, in the order found in the module declaration.
reference Identifier indicating the port name.
For example:
module test_device(count_out, carry, data, reset)
output count_out, carry ;
input [0:3] data;
input reset;
. . .
initial begin
$dumpports(testbench.DUT, "testoutput.vcd");
. . .
end
endmodule
This example produces the following node information in the VCD file:
$scope module testbench.DUT $end
$var port 1 <0 count_out $end
$var port 1 <1 carry $end
$var port [0:3] <2 data $end
$var port 1 <3 reset $end
$upscope $end
At least one space shall separate each syntactical element. However, the formatting of the information is the
choice of the simulator vendor. All 4-state VCD syntax rules for the vector_index apply.
If the vector_index appears in the port declaration, this shall be the index dumped. If the vector_index is not
in the port declaration, the vector_index in the net or variable declaration matching the port name shall be
dumped. If no vector_index is found, the port is considered scalar (1-bit wide).
Concatenated ports shall appear in the extended VCD file as separate entries.
For example:
module addbit ({A, b}, ci, sum, co);
input A, b, ci;
output sum, co;
. . .
The VCD file output looks like the following:
$scope module addbit $end
$var port 1 <0 A $end
$var port 1 <1 b $end
$var port 1 <2 ci $end
$enddefinitions $end
. . .

<!-- Page 701 -->

IEEE Std 21.7.4.3 Value changes
The value change section of the VCD file is also affected by $dumpports, as Syntax21-29 shows.
value ::= pport_value 0_strength_component 1_strength_component
Syntax21-29—Syntax for value change section (not in AnnexA)
The constructs are defined as follows:
p Key character that indicates a port. There is no space between the p and the
port_value.
port_value State character (described as follows).
0_strength_component One of the eight SystemVerilog strengths that indicates the strength0
specification for the port.
1_strength_component One of the eight SystemVerilog strengths that indicates the strength1
specification for the port.
The SystemVerilog strength values are as follows (append keyword with 0 or 1 as appropriate for the
strength component):
0 highz
1 small
2 medium
3 weak
4 large
5 pull
6 strong
7 supply
identifier_code the integer preceded by the < character as defined in the $var construct for the
port.
21.7.4.3.1 State characters
The following state information is listed in terms of input values from a test fixture, the output values of the
device under test (DUT), and the states representing unknown direction:
INPUT (TESTFIXTURE):
D low
U high
N unknown
Z three-state
d low (two or more drivers active)
u high (two or more drivers active)
OUTPUT (DUT):
L low
H high
X unknown (do not care)

<!-- Page 702 -->

IEEE Std T three-state
l low (two or more drivers active)
h high (two or more drivers active)
UNKNOWN DIRECTION:
0 low (both input and output are active with 0 value)
1 high (both input and output are active with 1 value)
? unknown
F three-state (input and output unconnected)
A unknown (input 0 and output 1)
a unknown (input 0 and output X)
B unknown (input 1 and output 0)
b unknown (input 1 and output X)
C unknown (input X and output 0)
c unknown (input X and output 1)
f unknown (input and output three-stated)
21.7.4.3.2 Drivers
Drivers are considered only in terms of primitives, continuous assignments, and procedural continuous
assignments. Value 0/1 means both input and output are active with value 0/1. 0 and 1 are conflict states.
The following rules apply to conflicts:
— If both input and output are driving the same value with the same range of strength, then this is a
conflict. The resolved value is 0/1, and the strength is the stronger of the two.
— If the input is driving a strong strength (range) and the output is driving a weak strength (range), the
resolved value is d/u, and the strength is the strength of the input.
— If the input is driving a weak strength (range) and the output is driving a strong strength (range), then
the resolved value is l/h, and the strength is the strength of the output.
Range is as follows:
— Strength 7 to 5 : strong strength
— Strength 4 to 1: weak strength
21.7.4.4 Extended VCD file format example
The following example illustrates the format of the extended VCD file.
A module declaration:
module adder(data0, data1, data2, data3, carry, as, rdn, reset,
test, write);
inout data0, data1, data2, data3;
output carry;
input as, rdn, reset, test, write;
. . .
and the resulting VCD fragment:
$scope module testbench.adder_instance $end
$var port 1 <0 data0 $end

<!-- Page 703 -->

IEEE Std $var port 1 <1 data1 $end
$var port 1 <2 data2 $end
$var port 1 <3 data3 $end
$var port 1 <4 carry $end
$var port 1 <5 as $end
$var port 1 <6 rdn $end
$var port 1 <7 reset $end
$var port 1 <8 test $end
$var port 1 <9 write $end
$upscope $end
$enddefinitions $end
#0
$dumpports
pX 6 6 <0
pX 6 6 <1
pX 6 6 <2
pX 6 6 <3
pX 6 6 <4
pN 6 6 <5
pN 6 6 <6
pU 0 6 <7
pD 6 0 <8
pN 6 6 <9
$end
#180
pH 0 6 <4
#200000
pD 6 0 <5
pU 0 6 <6
pD 6 0 <9
#200500
pf 0 0 <0
pf 0 0 <1
pf 0 0 <2
pf 0 0 <3
#### 21.7.5 VCD SystemVerilog type mappings
SystemVerilog does not extend the IEEE Std 1364-2005 VCD format. Some SystemVerilog types can be
dumped into a standard VCD file by masquerading as an IEEE Std 1364-2005 type. Table21-11 lists the
basic SystemVerilog types and their mapping to an IEEE Std 1364-2005 type for VCD dumping.
Table21-11—VCD type mapping
IEEE Std 1364-2005
SystemVerilog Size
Verilog
bit reg Total size of packed dimension
logic reg Total size of packed dimension
int integer 32
shortint reg 16
longint reg 64
byte reg 8

<!-- Page 704 -->

IEEE Std Table21-11—VCD type mapping (continued)
IEEE Std 1364-2005
SystemVerilog Size
Verilog
enum integer 32
shortreal real —
Packed arrays and structures are dumped as a single vector of reg. Multiple packed array dimensions are
collapsed into a single dimension.
If an enum declaration specified a type, it is dumped as that type rather than the default shown previously.
Unpacked structures appear as named fork-join blocks, and their member elements of the structure appear as
the preceding types. Because named fork-join blocks with variable declarations are seldom used in
testbenches and hardware models, this makes structures easy to distinguish from variables declared in
begin-end blocks, which are more frequently used in testbenches and models.
Unpacked arrays and automatic variables are not dumped.
NOTE—The current VCD format does not indicate whether a variable has been declared as signed or unsigned.

<!-- Page 705 -->

IEEE Std 1800-2023