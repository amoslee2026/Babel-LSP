# Chapter 5: Lexical conventions

## 5. Lexical conventions
### 5.1 General
This clause describes the following:
— Lexical tokens (white space, comments, operators)
— Integer, real, string, array, structure, and time literals
— Built-in method calls
— Attributes
### 5.2 Lexical tokens
SystemVerilog source text files shall be a stream of lexical tokens. A lexical token shall consist of one or
more characters. The layout of tokens in a source file shall be free format; that is, spaces and newline
characters shall not be syntactically significant other than being token separators, except for escaped
identifiers (see 5.6.1).
The types of lexical tokens in the language are as follows:
— White space
— Comment
— Operator
— Number
— String literal
— Identifier
— Keyword
### 5.3 White space
White space shall contain the characters for spaces, tabs, newlines, formfeeds, and end of file. These
characters shall be ignored except when they serve to separate other lexical tokens. However, blanks and
tabs shall be considered significant characters in string literals (see 5.9).
### 5.4 Comments
SystemVerilog has two forms to introduce comments. A one-line comment shall start with the two
characters // and end with a newline character. A block comment shall start with /* and end with */. Block
comments shall not be nested. The one-line comment token // shall not have any special meaning inside a
block comment, and the block comment tokens /* and */ shall not have any special meaning inside a one-
line comment.
### 5.5 Operators
Operators are single-, double-, or triple-character sequences and are used in expressions. Clause11
discusses the use of operators in expressions.
Unary operators shall appear to the left of their operand. Binary operators shall appear between their
operands. A conditional operator shall have two operator characters that separate three operands.

<!-- Page 75 -->

### 5.6 Identifiers, keywords, and system names
An identifier is used to give an object a unique name sothat it can be referenced. An identifier is either a
simple identifier or an escaped identifier (see 5.6.1). A simple identifier shall be any sequence of letters,
digits, dollar signs ($), and underscore characters (_). A keyword (see 5.6.2) may not be used as a user-
defined identifier.
The first character of a simple identifier shall not be a digit or $; it can be a letter or an underscore.
Identifiers shall be case sensitive.
For example:
shiftreg_a
busa_index
error_condition
merge_ab
_bus3
n$657
Implementations may set a limit on the maximum length of identifiers, but the limit shall be at least
1024characters. If an identifier exceeds the implementation-specific length limit, an error shall be reported.
#### 5.6.1 Escaped identifiers
Escaped identifiers shall start with the backslash character (\) and end with white space. They provide a
means of including any of the printable ASCII characters except white space in an identifier (the decimal
values 33 through 126, or 21 through 7E in hexadecimal).
Neither the leading backslash character nor the terminating white space is considered to be part of the
identifier. Therefore, an escaped identifier \cpu3 is treated the same as a nonescaped identifier cpu3. An
escaped keyword is an exception. An escaped keyword is treated as a user-defined identifier.
For example:
\busa+index
\-clock
\***error-condition***
\net1/\net2
\{a,b}
\a*(b+c)
\net // "net" is a keyword. "\net " is a user-defined identifier.
#### 5.6.2 Keywords
Keywords are predefined nonescaped identifiers that are used to define the language constructs. A
SystemVerilog keyword preceded by an escape (backslash) character is not interpreted as a keyword.
All keywords are defined in lowercase only. AnnexB gives a list of all defined keywords. Subclause 22.14
discusses compatibility of reserved keywords with previous versions of IEEE Std 1364 and IEEE Std 1800.
#### 5.6.3 System tasks and system functions
The dollar sign ($) introduces a language construct that enables development of user-defined system tasks
and system functions. System constructs are not design semantics, but refer to simulator functionality. A
name following the $ is interpreted as a system task or a system function.

<!-- Page 76 -->

The syntax for system tasks and system functions is given in Syntax5-1.
system_tf_call ::= // from A.8.2
system_tf_identifier [ ( list_of_arguments ) ]
| system_tf_identifier ( data_type [ , expression ] )
| system_tf_identifier ( expression { , [ expression ] } [ , [ clocking_event ] ] )
system_tf_identifier55 ::= $[ a-zA-Z0-9_$ ]{ [ a-zA-Z0-9_$ ] } // from A.9.3
55) The $ character in a system_tf_identifier shall not be followed by white_space. A system_tf_identifier shall not be
escaped.
Syntax5-1—Syntax for system tasks and system functions (excerpt from AnnexA)
SystemVerilog defines a standard set of system tasks and system functions in this document (see Clause20
and Clause21). Unlike SystemVerilog tasks (see 13.3), these standard system tasks do not consume time and
can be used in the same places void functions (see 13.4) can be used.
Additional user-defined system tasks and system functions can be defined using the PLI, as described in
Clause36. Software implementations can also specify additional system tasks and system functions, which
may be tool-specific (see AnnexD for some common additional system tasks and system functions).
Additional system tasks and system functions are not part of this standard.
For example:
$display ("display a message");
$finish;
#### 5.6.4 Compiler directives
The ` character (the ASCII value 0x60, called grave accent) introduces a language construct used to
implement compiler directives. The compiler behavior dictated by a compiler directive shall take effect as
soon as the compiler reads the directive. The directive shall remain in effect for the rest of the compilation
unit (see 3.12.1) unless a different compiler directive specifies otherwise. A compiler directive in one
description file can, therefore, control compilation behavior in multiple description files. A compiler
directive shall not affect other compilation units.
For example:
`define wordsize
SystemVerilog defines a standard set of compiler directives in this document (see Clause22). Software
implementations can also specify additional compiler directives, which may be tool-specific (see AnnexE
for some common additional compiler directives). Additional compiler directives are not part of this
standard.
### 5.7 Numbers
Constant numbers can be specified as integer constants (see 5.7.1) or real constants (see 5.7.2). The formal
syntax for numbers is listed in Syntax5-2.

<!-- Page 77 -->

primary_literal ::= number | time_literal | unbased_unsized_literal | string_literal // from A.8.4
time_literal49 ::=
unsigned_number time_unit
| fixed_point_number time_unit
time_unit ::= s | ms | us | ns | ps | fs
number ::= // from A.8.7
integral_number
| real_number
integral_number ::=
decimal_number
| octal_number
| binary_number
| hex_number
decimal_number ::=
unsigned_number
| [ size ] decimal_base unsigned_number
| [ size ] decimal_base x_digit { _ }
| [ size ] decimal_base z_digit { _ }
binary_number ::= [ size ] binary_base binary_value
octal_number ::= [ size ] octal_base octal_value
hex_number ::= [ size ] hex_base hex_value
sign ::= + | -
size ::= unsigned_number
real_number38 ::=
fixed_point_number
| unsigned_number [ . unsigned_number ] exp [ sign ] unsigned_number
fixed_point_number38 ::= unsigned_number . unsigned_number
exp ::= e | E
unsigned_number38 ::= decimal_digit { _ | decimal_digit }
binary_value38 ::= binary_digit { _ | binary_digit }
octal_value38 ::= octal_digit { _ | octal_digit }
hex_value38 ::= hex_digit { _ | hex_digit }
decimal_base38 ::= '[s|S]d | '[s|S]D
binary_base38 ::= '[s|S]b | '[s|S]B
octal_base38 ::= '[s|S]o | '[s|S]O
hex_base38 ::= '[s|S]h | '[s|S]H
decimal_digit ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
binary_digit ::= x_digit | z_digit | 0 | 1
octal_digit ::= x_digit | z_digit | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7
hex_digit ::= x_digit | z_digit | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | a | b | c | d | e | f | A | B | C | D | E | F
x_digit ::= x | X
z_digit ::= z | Z | ?
unbased_unsized_literal ::= '0 | '1 | 'z_or_x 53
38) Embedded spaces are illegal.
49) The unsigned number or fixed-point number in time_literal shall not be followed by white_space.
53) The apostrophe ( ' ) in unbased_unsized_literal shall not be followed by white_space.
Syntax5-2—Syntax for integer and real numbers (excerpt from AnnexA)

<!-- Page 78 -->

#### 5.7.1 Integer literal constants
Integer literal constants can be specified in decimal, hexadecimal, octal, or binary format.
There are two forms to express integer literal constants. The first form is a simple decimal number, which
shall be specified as a sequence of digits 0 through 9, optionally starting with a plus or minus unary
operator. The second form specifies a based literal constant, which shall be composed of up to three
tokens—an optional size constant, an apostrophe character (', ASCII 0x27) followed by a base format
character, and the digits representing the value of the number. It shall be legal to macro-substitute these three
tokens.
The first token, a size constant, shall specify the size of the integer literal constant in terms of its exact
number of bits. It shall be specified as a nonzero unsigned decimal number. For example, the size
specification for two hexadecimal digits is eight because one hexadecimal digit requires 4 bits.
The second token, a base format, shall consist of a case insensitive letter specifying the base for the number,
optionally preceded by the single character s (or S) to indicate a signed quantity, preceded by the apostrophe
character. Legal base specifications are d, D, h, H, o, O, b, or B for the bases decimal, hexadecimal, octal, and
binary, respectively.
The apostrophe character and the base format character shall not be separated by any white space.
The third token, an unsigned number, shall consist of digits that are legal for the specified base format. The
unsigned number token shall immediately follow the base format, optionally preceded by white space. The
hexadecimal digits a to f shall be case insensitive.
Simple decimal numbers without the size and the base format shall be treated as signed integers, whereas the
numbers specified with the base format shall be treated as signed integers if the s designator is included or
as unsigned integers if the base format only is used. The s designator does not affect the bit pattern
specified, only its interpretation.
A plus or minus operator preceding the size constant is a unary plus or minus operator. A plus or minus
operator between the base format and the number is an illegal syntax.
Negative numbers shall be represented in two’s-complement form.
An x represents the unknown value in hexadecimal, octal, and binary literal constants. A z represents the
high-impedance value. See 6.3 for a discussion of the SystemVerilog value set. An x shall set 4 bits to
unknown in the hexadecimal base, 3 bits in the octal base, and 1 bit in the binary base. Similarly, a z shall set
4 bits, 3bits, and 1 bit, respectively, to the high-impedance value.
If the size of the unsigned number is smaller than the size specified for the literal constant, the unsigned
number shall be padded to the left with zeros. If the leftmost bit in the unsigned number is an x or a z, then
an x or a z shall be used to pad to the left, respectively. If the size of the unsigned number is larger than the
size specified for the literal constant, the unsigned number shall be truncated from the left.
The number of bits that make up an unsized number (which is a simple decimal number or a number with a
base specifier but no size specification) shall be at least 32. An unsized number that requires more than 32
bits shall have at least the minimum width needed to properly represent the value, including a sign bit if the
number is signed. For example, 'h7_0000_0000, an unsigned hexadecimal number, shall have at least 35
bits. 4294967296 (232), a positive signed integer, shall be represented by at least 34 bits.
Unsized unsigned literal constants where the high-order bit is unknown (X or x) or high-impedance (Z or z)
shall be extended to the size of the expression containing the literal constant.

<!-- Page 79 -->

An unsized unsigned single-bit value can be specified by preceding the single-bit value with an apostrophe
( ' ), but without the base specifier. All bits of the unsized value shall be set to the value of the specified bit.
In a self-determined context, it shall have a width of 1 bit.
'0, '1, 'X, 'x, 'Z, 'z // sets all bits to specified value
The use of x and z in defining the value of a number is case insensitive.
When used in a number, the question mark (?) character is a SystemVerilog alternative for the z character.
It sets 4 bits to the high-impedance value in hexadecimal numbers, 3 bits in octal, and 1 bit in binary. The
question mark can be used to enhance readability in cases where the high-impedance value is a do-not-care
condition. See the discussion of casez and casex in 12.5.1. The question mark character is also used in
UDP state tables. See Table29-1 in 29.3.6.
In a decimal literal constant, the unsigned number token shall not contain any x, z, or ? digits, unless there is
exactly one digit in the token, indicating that every bit in the decimal literal constant is x or z.
The underscore character (_) shall be legal anywhere in a number except as the first character. The
underscore character is ignored. This feature can be used to break up long numbers for readability purposes.
Several examples of specifying literal integer numbers are as follows:
Example 1: Unsized literal constant numbers
659 // is a decimal number
'h 837FF // is a hexadecimal number
'o7460 // is an octal number
4af // is illegal (hexadecimal format requires 'h)
Example 2: Sized literal constant numbers
4'b1001 // is a 4-bit binary number
5 'D 3 // is a 5-bit decimal number
3'b01x // is a 3-bit number with the least
// significant bit unknown
12'hx // is a 12-bit unknown number
16'hz // is a 16-bit high-impedance number
Example 3: Using sign with literal constant numbers
8 'd -6 // this is illegal syntax
-8 'd 6 // this defines the two's-complement of 6,
// held in 8 bits—equivalent to -(8'd 6)
4 'shf // this denotes the 4-bit number '1111', to
// be interpreted as a two's-complement number,
// or '-1'. This is equivalent to -4'h 1
-4 'sd15 // this is equivalent to -(-4'd 1), or '0001'
16'sd? // the same as 16'sbz
Example 4: Automatic left padding of literal constant numbers
logic [11:0] a, b, c, d;
logic [84:0] e, f, g;
initial begin
a = 'h x; // yields xxx
b = 'h 3x; // yields 03x

<!-- Page 80 -->

c = 'h z3; // yields zz3
d = 'h 0z3; // yields 0z3
e = 'h5; // yields {82{1'b0},3'b101}
f = 'hx; // yields {85{1'hx}}
g = 'hz; // yields {85{1'hz}}
end
Example 5: Automatic left padding of constant literal numbers using a single-bit value
logic [15:0] a, b, c, d;
a = '0; // sets all 16 bits to 0
b = '1; // sets all 16 bits to 1
c = 'x; // sets all 16 bits to x
d = 'z; // sets all 16 bits to z
Example 6: Underscores in literal constant numbers
27_195_000 // unsized decimal 27195000
16'b0011_0101_0001_1111 // 16-bit binary number
32 'h 12ab_f001 // 32-bit hexadecimal number
An integer literal constant is a vector (see 7.4) of type logic with range [n-1:0], where n is the number of
bits in the constant, as specified above. The vector is signed if the constant is signed, and unsigned
otherwise. Sized negative literal constant numbers and sized signed literal constant numbers are sign-
extended when assigned to a data object of type logic, regardless of whether the type itself is signed.
#### 5.7.2 Real literal constants
The real literal constant numbers shall be represented as described by IEEE Std 754, an IEEE standard for
double-precision floating-point numbers.
Real numbers can be specified in either decimal notation (for example, 14.72) or in scientific notation (for
example, 39e8, which indicates 39 multiplied by 10 to the eighth power). Real numbers expressed with a
decimal point shall have at least one digit on each side of the decimal point.
For example:
1.2
0.1
2394.26331
1.2E12 (the exponent symbol can be e or E)
1.30e-2
0.1e-0
23E10
29E-2
236.123_763_e-12 (underscores are ignored)
The following are invalid forms of real numbers because they do not have at least one digit on each side of
the decimal point:
.12
9.
4.E3
.2e-7

<!-- Page 81 -->

The default type for fixed-point format (e.g., 1.2), and exponent format (e.g., 2.0e10) shall be real.
A cast can be used to convert real literal values to the shortreal type (e.g., shortreal'(1.2)). Casting
is described in 6.24.
### 5.8 Time literals
Time is written in integer or fixed-point format, followed without a space by a time unit (fs ps ns us ms s).
For example:
2.1ns
40ps
The time literal is interpreted as a realtime value scaled to the current time unit.
### 5.9 String literals
A string literal is a sequence of characters enclosed by a single pair of double quotes (""), called a quoted
string, or a triple pair of double quotes ("""..."""), called a triple-quoted string. There is no predefined
limit to the length of a string literal.
The syntax for string literals in shown in Syntax5-3.
string_literal ::= // from A.8.8
quoted_string
| triple_quoted_string
quoted_string ::= " { quoted_string_item | string_escape_seq } "
triple_quoted_string ::= """ { triple_quoted_string_item | string_escape_seq } """
quoted_string_item ::= any_ASCII_character except \ or newline or "
triple_quoted_string_item ::= any_ASCII_character except \
string_escape_seq ::=
\any_ASCII_character
| \one_to_three_digit_octal_number
| \x one_to_two_digit_hex_number
Syntax5-3—Syntax for string literals (excerpt from AnnexA)
Within a string literal, nonprintable and other special characters can be represented by a string escape
sequence as described in 5.9.1. Support for nonescaped nonprintable ASCII characters is implementation
dependent.
A quoted string shall be contained in a single line unless the newline character is immediately preceded by a
\ (backslash). In this case, the backslash and the newline character are ignored.
Example 1:
$display("Humpty Dumpty sat on a wall. \
Humpty Dumpty had a great fall.");
prints

<!-- Page 82 -->

Humpty Dumpty sat on a wall. Humpty Dumpty had a great fall.
Example 2:
$display("Humpty Dumpty sat on a wall.\n\
Humpty Dumpty had a great fall.");
prints
Humpty Dumpty sat on a wall.
Humpty Dumpty had a great fall.
Triple-quoted string literals differ from quoted string literals in two ways:
— Triple-quoted string literals allow for a newline character to be inserted directly without using the \n
escape sequence.
— Triple-quoted string literals allow for a " character to be inserted directly without using the \"
escape sequence.
In all other ways, the two constructs are identical. This means that an escaped newline in a triple-quoted
string literal is treated exactly like an escaped newline in a single-quoted string literal.
Example 3:
$display("""Humpty Dumpty sat on a "wall".
Humpty Dumpty had a great fall. """);
prints
Humpty Dumpty sat on a "wall".
Humpty Dumpty had a great fall.
Example 4:
$display("""Humpty Dumpty sat on a wall. \
Humpty Dumpty had a great fall. """);
prints
Humpty Dumpty sat on a wall. Humpty Dumpty had a great fall.
Example 5:
$display("""Humpty Dumpty \n sat on a wall. \n
Humpty Dumpty had a great fall. """);
prints
Humpty Dumpty
sat on a wall.
Humpty Dumpty had a great fall.
String literals used as operands in expressions and assignments shall be treated as unsigned integer constants
represented by a sequence of 8-bit ASCII values, with one 8-bit ASCII value representing one character. An
escaped character sequence in a string literal is also represented by a single 8-bit ASCII value.

<!-- Page 83 -->

A string literal can be assigned to an integral type, such as a packed array. If the size differs, it is right
justified. To fully store a string literal, the integral type should be declared with a width equal to the number
of characters in the string multiplied by 8. For example:
byte c1 = "A" ;
bit [7:0] d = "\n" ;
The rules of SystemVerilog assignments shall be followed if the packed array width does not match the
number of characters multiplied by 8. When an integral type is larger than required to hold the string literal
value being assigned, the value is right-justified, and the leftmost bits are padded with zeros, as is done with
nonstring values. If a string literal is larger than the destination integral type, the string is right-justified, and
the leftmost characters are truncated.
For example, to store the 12-character string "Hello world\n" requires a variable 8 × 12, or 96 bits wide.
bit [8*12:1] stringvar = "Hello world\n";
Alternatively, a multidimensional packed array can be used, with 8-bit subfields, as in:
bit [0:11] [7:0] stringvar = "Hello world\n" ;
A string literal can be assigned to an unpacked array of bytes. If the size differs, it is left justified.
byte c3 [0:12] = "hello world\n" ;
Packed and unpacked arrays are discussed in 7.4.
String literals can also be cast to a packed or unpacked array type, which shall follow the same rules as
assigning a string literal to a packed or unpacked array. Casting is discussed in 6.24.
SystemVerilog also includes a string data type to which a string literal can be assigned. Variables of type
string have arbitrary length; they are dynamically resized to hold any string. String literals are packed
arrays (of a width that is a multiple of 8 bits), and they are implicitly converted to the string type when
assigned to a string type or used in an expression involving string type operands (see 6.16).
The following example shows an assignment to a string variable using multi-line triple quotes to start and
end the string value.
Example 6:
string foo;
foo = """
This is one continuous string.
Single ' and double " can
be placed throughout, and
only a triple quote will end it.
""";
NOTE—In the preceding example, the newline preceding the ending triple quote is included in the string.
String literals stored in vectors can be manipulated using the SystemVerilog operators. The value being
manipulated by the operator is the sequence of 8-bit ASCII values. See 11.10 for operations on string
literals.

<!-- Page 84 -->

#### 5.9.1 Special characters in strings
Certain ASCII characters can only be represented in string literals using an escape sequence. Table5-1 lists
these characters in the right-hand column, with the escape sequence that represents the character in the left-
hand column. While triple-quoted string literals support unescaped " and newline characters, the escape
sequences associated with those characters are also supported.
Table5-1—Specifying special characters in string literals
Escape sequence Character produced by escape sequence
\n Newline character
\t Tab character
\\ \ character
\" " character
\v Vertical tab
\f Form feed
\a Bell
\ddd A character specified in 1 to 3 octal_digits (see Syntax5-2). If fewer than three
digits are used, the following character shall not be an octal_digit. Implementations may
issue an error if the character represented is greater than \377. It shall be illegal for an
octal_digit in an escape sequence to be an x_digit or a z_digit (see Syntax5-2).
\xdd A character specified in 1 to 2 hex_digits (see Syntax5-2). If only one digit is used, the
following character shall not be a hex_digit. It shall be illegal for a hex_digit in an
escape sequence to be an x_digit or a z_digit (see Syntax5-2).
An escaped character not appearing in Table5-1 is treated the same as if the character was not escaped. For
example, "\b" is treated the same as "b". Both literals are considered to contain a single character, the letter
"b".
If a newline character is immediately preceded by \\ (double backslash), the double backslash is interpreted
as an escape sequence representing a single backslash character in the string and not as the first part of a line
continuation sequence. Therefore, a line continuation sequence requires a third backslash.
Example:
$display("Humpty Dumpty sat on a wall. \\\
Humpty Dumpty had a great fall.");
prints
Humpty Dumpty sat on a wall. \Humpty Dumpty had a great fall.
### 5.10 Structure literals
Structure literals are structure assignment patterns or pattern expressions with constant member expressions
(see 10.9.2). A structure literal shall have a type, which may be either explicitly indicated with a prefix or
implicitly indicated by an assignment-like context (see 10.8).
typedef struct {int a; shortreal b;} ab;

<!-- Page 85 -->

ab c;
c = '{0, 0.0}; // structure literal type determined from
// the left-hand context (c)
Nested braces shall reflect the structure. For example:
ab abarr[1:0] = '{'{1, 1.0}, '{2, 2.0}};
The C-like alternative '{1, 1.0, 2, 2.0} for the preceding example is not allowed.
Structure literals can also use member name and value or use data type and default value (see 10.9.2):
c = '{a:0, b:0.0}; // member name and value for that member
c = '{default:0}; // all elements of structure c are set to 0
d = ab'{int:1, shortreal:1.0}; // data type and default value for all
// members of that type
When an array of structures is initialized, the nested braces shall reflect the array and the structure. For
example:
ab abarr[1:0] = '{'{1, 1.0}, '{2, 2.0}};
Replication operators can be used to set the values for the exact number of members. The inner pair of
braces in a replication is removed.
struct {int X,Y,Z;} XYZ = '{3{1}};
typedef struct {int a,b[4];} ab_t;
int a,b,c;
ab_t v1[1:0] [2:0];
v1 = '{2{'{3{'{a,'{2{b,c}}}}}}};
/* expands to '{ '{3{ '{ a, '{2{ b, c }} } }},
'{3{ '{ a, '{2{ b, c }} } }}
} */
/* expands to '{ '{ '{ a, '{2{ b, c }} },
'{ a, '{2{ b, c }} },
'{ a, '{2{ b, c }} }
},
'{ '{ a, '{2{ b, c }} },
'{ a, '{2{ b, c }} },
'{ a, '{2{ b, c }} }
}
} */
/* expands to '{ '{ '{ a, '{ b, c, b, c } },
'{ a, '{ b, c, b, c } },
'{ a, '{ b, c, b, c } }
},
'{ '{ a, '{ b, c, b, c } },
'{ a, '{ b, c, b, c } },
'{ a, '{ b, c, b, c } }
}
} */

<!-- Page 86 -->

### 5.11 Array literals
Array literals are syntactically similar to C initializers, but with the replication operator ( {{}} ) allowed.
int n[1:2][1:3] = '{'{0,1,2},'{3{4}}};
The nesting of braces shall follow the number of dimensions, unlike in C. However, replication operators
can be nested. The inner pair of braces in a replication is removed. A replication expression only operates
within one dimension.
int n[1:2][1:6] = '{2{'{3{4, 5}}}}; // same as
'{'{4,5,4,5,4,5},'{4,5,4,5,4,5}}
Array literals are array assignment patterns or pattern expressions with constant member expressions (see
10.9.1). An array literal shall have a type, which may be either explicitly indicated with a prefix or implicitly
indicated by an assignment-like context (see 10.8).
typedef int triple [1:3];
$mydisplay(triple'{0,1,2});
Array literals can also use their index or type as a key and use a default key value (see 10.9.1).
triple b = '{1:1, default:0}; // indices 2 and 3 assigned 0
### 5.12 Attributes
A mechanism is included for specifying properties about objects, statements, and groups of statements in the
SystemVerilog source that can be used by various tools, including simulators, to control the operation or
behavior of the tool. These properties are referred to as attributes. This subclause specifies the syntactic
mechanism used for specifying attributes, without standardizing on any particular attributes.
The syntax for specifying an attribute is shown in Syntax5-4.
attribute_instance ::= (* attr_spec { , attr_spec } *) // from A.9.1
attr_spec ::= attr_name [ = constant_expression ]
attr_name ::= identifier
Syntax5-4—Syntax for attributes (excerpt from AnnexA)
An attribute_instance can appear in the SystemVerilog description as a prefix attached to a declaration, a
module item, a statement, or a port connection. It can appear as a suffix to an operator or a function name in
an expression.
The default type of an attribute with no value is bit, with a value of 1. Otherwise, the attribute takes the
type of the expression.
If the same attribute name is defined more than once for the same language element, the last attribute value
shall be used, and a tool can issue a warning that a duplicate attribute specification has occurred.
Nesting of attribute instances is disallowed. It shall be illegal to specify the value of an attribute with a
constant expression that contains an attribute instance.

<!-- Page 87 -->

Refer to AnnexA for the syntax of specifying an attribute instance on specific language elements. Several
examples are illustrated below.
Example 1: The following example shows how to attach attributes to a case statement:
(* full_case, parallel_case *)
case (a)
<rest_of_case_statement>
or
(* full_case=1 *)
(* parallel_case=1 *) // Multiple attribute instances also OK
case (a)
<rest_of_case_statement>
or
(* full_case, // no value assigned
parallel_case=1 *)
case (a)
<rest_of_case_statement>
Example 2: To attach the full_case attribute, but not the parallel_case attribute:
(* full_case *) // parallel_case not specified
case (a)
<rest_of_case_statement>
or
(* full_case=1, parallel_case = 0 *)
case (a)
<rest_of_case_statement>
Example 3: To attach an attribute to a module definition:
(* optimize_power *)
module mod1 (<port_list>);
or
(* optimize_power=1 *)
module mod1 (<port_list>);
Example 4: To attach an attribute to a module instantiation:
(* optimize_power=0 *)
mod1 synth1 (<port_list>);
Example 5: To attach an attribute to a variable declaration:
(* fsm_state *) logic [7:0] state1;
(* fsm_state=1 *) logic [3:0] state2, state3;
logic [3:0] reg1; // reg1 does NOT have fsm_state set
(* fsm_state=0 *) logic [3:0] reg2; // nor does reg2

<!-- Page 88 -->

Example 6: To attach an attribute to an operator:
a = b + (* mode = "cla" *) c; // sets the value for the attribute 'mode'
// to be the string cla.
Example 7: To attach an attribute to a function call:
a = add (* mode = "cla" *) (b, c);
Example 8: To attach an attribute to a conditional operator:
a = b ? (* no_glitch *) c : d;
### 5.13 Built-in methods
SystemVerilog uses a C++-like class method calling syntax, in which a subroutine is called using the dot
notation (.):
object.task_or_function()
The object uniquely identifies the data on which the subroutine operates. Hence, the method concept is
naturally extended to built-in types in order to add functionality, which traditionally was done via system
tasks or system functions. Unlike system tasks, built-in methods are not prefixed with a $ because they
require no special prefix to avoid collisions with user-defined identifiers. Thus, the method syntax allows
extending the language without the addition of new keywords or the cluttering of the global name space with
system tasks.
Built-in methods, unlike system tasks, cannot be redefined by users via PLI tasks. Thus, only functions that
users should not be allowed to redefine are good candidates for built-in method calls.
In general, a built-in method is preferred over a system task when a particular functionality applies to all
data types or when it applies to a specific data type. For example:
dynamic_array.size, associative_array.num, and string.len
These are all similar concepts, but they represent different things. A dynamic array has a size, an associative
array contains a given number of items, and a string has a given length. Using the same system task, such as
$size, for all of them would be less clear and intuitive.
A built-in method can only be associated with a particular data type. Therefore, if some functionality is a
simple side effect (i.e., $stop or $reset) or it operates on no specific data (i.e., $random), then a system
task shall be used.
When a subroutine built-in method call specifies no arguments, the empty parentheses, (), following the
subroutine name are optional. This is also true for subroutines that require arguments, when all arguments
have defaults specified. For a method, this rule allows simple calls to appear as properties of the object or
built-in type. Note the exception to this rule when using a built-in method call as an implicit variable,
wherein the parentheses are always required even when empty (see 13.4.1). Similar rules are defined for
subroutines in 13.5.5.

<!-- Page 89 -->

IEEE Std 1800-2023