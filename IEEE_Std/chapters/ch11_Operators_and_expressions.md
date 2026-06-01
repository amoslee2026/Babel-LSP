# Chapter 11: Operators and expressions

## 11. Operators and expressions
### 11.1 General
This clause describes the following:
— Expression semantics
— Operations on expressions
— Operator precedence
— Operand size extension rules
— Signed and unsigned operation rules
— Bit-select and part-select operations and longest static prefix
— Bit-stream operations
### 11.2 Overview
This clause describes the operators and operands available in SystemVerilog and how to use them to form
expressions.
An expression is a construct that combines operands with operators to produce a result that is a function of
the values of the operands and the semantic meaning of the operator. Any legal operand, such as a net bit-
select, without any operator is considered an expression. Wherever a value is needed in a SystemVerilog
statement, an expression can be used.
An operand can be one of the following:
— Constant literal number, including real literals
— String literal
— Parameter, including local and specify parameters
— Parameter bit-select or part-select, including local and specify parameters
— Net (see 6.7)
— Net bit-select or part-select
— Variable (see 6.8)
— Variable bit-select or part-select
— Structure, either packed or unpacked
— Structure member
— Packed structure bit-select or part-select
— Union, packed, unpacked, or tagged
— Union member
— Packed union bit-select or part-select
— Array, either packed or unpacked
— Packed array bit-select, part-select, element, or slice
— Unpacked array element bit-select or part-select, element, or slice
— A call to a user-defined function, system function, or method that returns any of the above

<!-- Page 271 -->

IEEE Std #### 11.2.1 Constant expressions
Some statement constructs require an expression to be a constant expression. The operands of a constant
expression consist of constant numbers, strings, parameters, constant bit-selects and part-selects of
parameters, non-void constant function calls (see 13.4.3), constant system function calls, and constant built-
in method calls only. Constant expressions can use any of the operators defined in Table11-1.
Constant system function calls are calls to certain built-in system functions where the arguments meet
conditions outlined in this subclause. When used in constant expressions, these function calls shall be
evaluated at elaboration time. The system functions that may be used in constant system function calls are
pure functions, i.e., those whose value depends only on their input arguments and that have no side effects.
Calls to the following system functions are constant system function calls if the arguments are constant
expressions: the timescale system functions listed in 20.4 ($timescale, $timeprecision), the
conversion system functions listed in 20.5, the mathematical system functions listed in 20.8, the bit vector
system functions listed in 20.9, and the $sformatf system function listed in 21.3.3.
Calls to the data query system functions listed in 20.6 and the array query system functions listed in 20.7
may also be constant system function calls even when their arguments are not constant. See those subclauses
for the conditions under which these query system function calls are considered to be constant expressions.
Constant built-in method calls are calls to built-in methods (see 5.13) wherein the conditions outlined in this
subclause are met. When used in constant expressions, these function calls shall be evaluated at elaboration
time. The methods that may be used in constant built-in method calls shall be functions whose value
depends only on their input arguments or the current value of the identifier on which they are called.
Constant built-in method calls shall have no side effects, including changing the current value of the
identifier on which they are called.
Built-in method calls that meet the above conditions are constant built-in method calls if the identifier and
input arguments are constant expressions. In addition, built-in methods that meet the above conditions and
whose value does not depend on the current value of the identifier are constant built-in method calls if the
input arguments are constant expressions even when the identifier is not constant.
#### 11.2.2 Aggregate expressions
Unpacked structure and array data objects, as well as unpacked structure and array constructors, can all be
used as aggregate expressions. A multi-element slice of an unpacked array can also be used as an aggregate
expression.
Aggregate expressions can be copied in an assignment, through a port, or as an argument to a subroutine.
Aggregate expressions can also be compared with equality or inequality operators.
If the two operands of a comparison operator are aggregate expressions, they shall be of equivalent type as
defined in 6.22.2. Assignment compatibility of aggregate expressions is defined in 6.22.3 and, for arrays, in
7.6.
### 11.3 Operators
The symbols for the SystemVerilog operators are similar to those in the C programming language.
Syntax11-1 and Table11-1 list these operators.
assignment_operator ::= = | += | -= | *= | /= | %= | &= | |= | ^= | <<= | >>= | <<<= | >>>= // from A.6.2
conditional_expression ::= // from A.8.3
cond_predicate ? { attribute_instance } expression : expression
unary_operator ::= + | - | ! | ~ | & | ~& | | | ~| | ^ | ~^ | ^~ // from A.8.6

<!-- Page 272 -->

IEEE Std binary_operator ::=
+ | - | * | / | % | == | != | === | !== | ==? | !=? | && | || | **
| < | <= | > | >= | & | | | ^ | ^~ | ~^ | >> | << | >>> | <<< |-> | <->
inc_or_dec_operator ::= ++ | --
stream_operator ::= >> | << // from A.8.1
Syntax11-1—Operator syntax (excerpt from AnnexA)
Table11-1—Operators and data types
Operator token Name Operand data types
= Binary assignment operator Any
+= -= /= *= Binary arithmetic assignment operators Integral, real, shortreal
%= Binary arithmetic modulus assignment operator Integral
&= |= ^= Binary bitwise assignment operators Integral
>>= <<= Binary logical shift assignment operators Integral
>>>= <<<= Binary arithmetic shift assignment operators Integral
?: Conditional operator Any
+ - Unary arithmetic operators Integral, real, shortreal
~ Unary bitwise negation operator Integral
! Unary logical negation operator Integral, real, shortreal
& ~& | ~| ^ ~^ ^~ Unary reduction operators Integral
+ - * / ** Binary arithmetic operators Integral, real, shortreal
% Binary arithmetic modulus operator Integral
& | ^ ^~ ~^ Binary bitwise operators Integral
>> << Binary logical shift operators Integral
>>> <<< Binary arithmetic shift operators Integral
&& || –> <–> Binary logical operators Integral, real, shortreal
< <= > >= Binary relational operators Integral, real, shortreal
=== !== Binary case equality operators Any except real and
shortreal
== != Binary logical equality operators Any
==? !=? Binary wildcard equality operators Integral
++ -- Unary increment, decrement operators Integral, real, shortreal
inside Binary set membership operator Singular for the left operand
dista Binary distribution operator Integral
{} {{}} Concatenation, replication operators Integral
{<<{}} {>>{}} Stream operators Integral
aThe dist operator is described in 16.14.2 and 18.5.3.

<!-- Page 273 -->

IEEE Std #### 11.3.1 Operators with real operands
Table11-1 shows what operators may be applied to real operands.
The result of using logical or relational operators or the inside operator on real operands shall be a
single-bit value.
For other operators, if any operand, except before the ? in the conditional operator, is real, the result is
real. Otherwise, if any operand, except before the ? in the conditional operator, is shortreal, the result is
shortreal.
Real operands can also be used in the following expressions:
str.realval // structure or union member
realarray[intval] // array element
See 6.12 for more information on use of real numbers.
#### 11.3.2 Operator precedence
Operator precedence and associativity are listed in Table11-2. The highest precedence is listed first.
Table11-2—Operator precedence and associativity
Operator Associativity Precedence
() [] :: . Left Highest
+ - ! ~ & ~& | ~| ^ ~^ ^~ ++ -- (unary)
** Left
* / % Left
+ - (binary) Left
<< >> <<< >>> Left
< <= > >= inside dist Left
== != === !== ==? !=? Left
& (binary) Left
^ ~^ ^~ (binary) Left
| (binary) Left
&& Left
|| Left
?: (conditional operator) Right
–> <–> Right
= += -= *= /= %= &= ^= |= None
<<= >>= <<<= >>>= := :/ <=
{} {{}} Concatenation Lowest

<!-- Page 274 -->

IEEE Std Operators shown on the same row in Table11-2 shall have the same precedence. Rows are arranged in order
of decreasing precedence for the operators. For example, *, /, and % all have the same precedence, which is
higher than that of the binary + and – operators.
All operators shall associate left to right with the exception of the conditional (?:), implication (->), and
equivalence (<->) operators, which shall associate right to left. Associativity refers to the order in which the
operators having the same precedence are evaluated. Thus, in the following example, B is added to A, and
then C is subtracted from the result of A+B.
A + B - C
When operators differ in precedence, the operators with higher precedence shall associate first. In the
following example, B is divided by C (division has higher precedence than addition), and then the result is
added to A.
A + B / C
Parentheses can be used to change the operator precedence.
(A + B) / C // not the same as A + B / C
#### 11.3.3 Using integer literals in expressions
Integer literals can be used as operands in expressions. An integer literal can be expressed as the following:
— An unsized, unbased integer (e.g., 12)
— An unsized, based integer (e.g., 'd12, 'sd12)
— A sized, based integer (e.g., 16'd12, 16'sd12)
See 5.7.1 for integer literal syntax.
A negative value for an integer with no base specifier shall be interpreted differently from an integer with a
base specifier. An integer with no base specifier shall be interpreted as a signed value in two’s-complement
form. An integer with an unsigned base specifier shall be interpreted as an unsigned value.
The following example shows four ways to write the expression “minus 12 divided by 3.” Note that -12 and
-'d12 both evaluate to the same two’s-complement bit pattern, but, in an expression, the -'d12 loses its
identity as a signed negative number.
int IntA;
IntA = -12 / 3; // The result is -4
IntA = -'d 12 / 3; // The result is 1431655761
IntA = -'sd 12 / 3; // The result is -4
IntA = -4'sd 12 / 3; // -4'sd12 is the negative of the 4-bit
// quantity 1100, which is -4. -(-4) = 4
// The result is 1
#### 11.3.4 Operations on logic (4-state) and bit (2-state) types
Operators may be applied to 2-state values or to a mixture of 2-state and 4-state values. The result is the
same as if all values were treated as 4-state values. In most cases, if all operands are 2-state, the result is in

<!-- Page 275 -->

IEEE Std the 2-state value set. The only exceptions involve operators that produce an x result for operands in the
2-state value set (e.g., division by zero).
int n = 8, zero = 0;
int res = 'b01xz | n; // res gets 'b11xz coerced to int, or 'b1100
int sum = n + n; // sum gets 16 coerced to int, or 16
int sumx = 'x + n; // sumx gets 'x coerced to int, or 0
int div2 = n/zero + n; // div2 gets 'x coerced to int, or 0
integer div4 = n/zero + n; // div4 gets 'x
#### 11.3.5 Operator expression short circuiting
The operators shall follow the associativity rules while evaluating an expression as described in 11.3.2.
Some operators (&&, ||, ->, and ?:) shall use short-circuit evaluation; in other words, some of their
operand expressions shall not be evaluated if their value is not required to determine the final value of the
operation. The detailed short-circuiting behavior of each of these operators is described in its corresponding
subclause (11.4.7 and 11.4.11). All other operators shall not use short-circuit evaluation—all of their
operand expressions are always evaluated. When short circuiting occurs, any side effects or run-time errors
that would have occurred due to evaluation of the short-circuited operand expression shall not occur.
For example:
logic regA, regB, regC, result1, result2;
function logic myFunc(logic x);
...
endfunction
result1 = regA & (regB | myFunc(regC)) ;
result2 = regA && (regB || myFunc(regC)) ;
For result1, even if regA is zero, the subexpression (regB | myFunc(regC)) will be evaluated and any
side effects caused by calling myFunc(regC) will occur. For result2, if regA is zero, the subexpression
(regB || myFunc(regC)) will not be evaluated and thus any side effects caused by calling
myFunc(regC) will not occur.
Note that implementations are free to optimize by omitting evaluation of subexpressions as long as the
simulation behavior (including side effects) is as if the standard rules were followed.
#### 11.3.6 Assignment within an expression
An expression can include a blocking assignment, provided it does not have a timing control. These
blocking assignments shall be enclosed in parentheses to avoid common mistakes such as using a=b for
a==b or using a|=b for a!=b.
if ((a=b)) b = (a+=1);
a = (b = (c = 5));
The semantics of such an assignment expression is that of a function that evaluates the right-hand side, casts
the right-hand side to the left-hand data type, stacks it, updates the left-hand side, and returns the stacked
value. The data type of the value that is returned is the data type of the left-hand side. If the left-hand side is
a concatenation, then the data type of the value that is returned shall be an unsigned integral data type whose
bit length is the sum of the length of its operands.

<!-- Page 276 -->

IEEE Std It shall be illegal to include an assignment operator in an event expression, in an expression within a
procedural continuous assignment, or in an expression that is not within a procedural statement.
### 11.4 Operator descriptions
#### 11.4.1 Assignment operators
In addition to the simple assignment operator, =, SystemVerilog includes the C assignment operators and
special bitwise assignment operators: +=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=, <<<=, and >>>=. An
assignment operator is semantically equivalent to a blocking assignment, with the exception that any
left-hand index expression is only evaluated once. For example:
a[i]+=2; // same as a[i] = a[i] +2;
#### 11.4.2 Increment and decrement operators
SystemVerilog includes the C increment and decrement assignment operators ++i, --i, i++, and i--.
These do not need parentheses when used in expressions. These increment and decrement assignment
operators behave as blocking assignments.
The ordering of assignment operations relative to any other operation within an expression is undefined. An
implementation can warn whenever a variable is both written and read-or-written within an integral
expression or in other contexts where an implementation cannot guarantee order of evaluation. For example:
i = 10;
j = i++ + (i = i - 1);
After execution, the value of j can be 18, 19, or 20 depending upon the relative ordering of the increment
and the assignment statements.
The increment and decrement operators, when applied to real operands, increment or decrement the operand
by 1.0.
#### 11.4.3 Arithmetic operators
The binary arithmetic operators are given in Table11-3.
Table11-3—Arithmetic operators defined
a + b a plus b
a - b a minus b
a * b a multiplied by b (or a times b)
a / b a divided by b
a % b a modulo b
a ** b a to the power of b
The integer division shall truncate any fractional part toward zero. For the division or modulus operators, if
the second operand is a zero, then the entire result value shall be x. The modulus operator (for example,
a % b) gives the remainder when the first operand is divided by the second and thus is zero when b divides
a exactly. The result of a modulus operation shall take the sign of the first operand.

<!-- Page 277 -->

IEEE Std If either operand of the power operator is real, then the result type shall be real (see 11.3.1). The result of the
power operator is unspecified if the first operand is zero and the second operand is negative or if the first
operand is negative and the second operand is not an integral value.
If neither operand of the power operator is real, then the result type shall be determined as outlined in 11.6.1
and 11.8.1. The entire result value shall be x if the first operand is zero and the second operand is negative.
The result value is 1 if the second operand is zero.
In all cases, the second operand of the power operator shall be treated as self-determined.
These statements are illustrated in Table11-4.
Table11-4—Integral power operator rules
op1 < -1 op1 == -1 op1 == 0 op1 == 1 op1 > 1
op2 is odd -> -1
op2 is positive op1 ** op2 0 1 op1 ** op2
op2 is even -> 1
op2 is zero 1 1 1 1 1
op2 is odd -> -1
op2 is negative 0 'x 1 0
op2 is even -> 1
Table11-5 gives examples of some modulus and power operations.
Table11-5—Examples of modulus and power operators
Expression Result Comments
10 % 3 1 10/3 yields a remainder of 1.
11 % 3 2 11/3 yields a remainder of 2.
12 % 3 0 12/3 yields no remainder.
–10 % 3 –1 The result takes the sign of the first operand.
11 % –3 2 The result takes the sign of the first operand.
–4'd12 % 3 1 –4'd12 is seen as a large positive number that leaves a remainder of 1 when divided by 3.
3 ** 2 9 3 × 3
2 ** 3 8 2 × 2 × 2
2 ** 0 1 Anything to the zero exponent is 1.
0 ** 0 1 Zero to the zero exponent is also 1.
### 2.0 ** –3'sb1 0.5 2.0 is real, giving real reciprocal.
2 ** –3 'sb1 0 2 ** –1 = 1/2. Integer division truncates to zero.
0 ** –1 'x 0 ** –1 = 1/0. Integer division by zero is 'x.
9 ** 0.5 3.0 Real square root.

<!-- Page 278 -->

IEEE Std Table11-5—Examples of modulus and power operators (continued)
Expression Result Comments
### 9.0 ** (1/2) 1.0 Integer division truncates exponent to zero.
–3.0 ** 2.0 9.0 Defined because real 2.0 is still integral value.
The unary arithmetic operators shall take precedence over the binary operators. The unary operators are
given in Table11-6.
Table11-6—Unary operators defined
+m Unary plus m (same as m)
-m Unary minus m
For the arithmetic operators, if any operand bit value is the unknown value x or the high-impedance value z,
then the entire result value shall be x.
11.4.3.1 Arithmetic expressions with unsigned and signed types
Nets and variables can be explicitly declared as unsigned or signed. The byte, shortint, int, integer,
and longint data types are signed by default. Other data types are unsigned by default.
A value assigned to an unsigned variable or net shall be treated as an unsigned value. A value assigned to a
signed variable or net shall be treated as signed. Signed values, except for those assigned to real variables,
shall use a two’s-complement representation. Values assigned to real variables shall use a floating-point
representation. Conversions between signed and unsigned values shall keep the same bit representation;
only the interpretation changes.
Table11-7 lists how arithmetic operators interpret each data type.
Table11-7—Data type interpretation by arithmetic operators
Data type Interpretation
Unsigned net Unsigned
Signed net Signed, two’s-complement
Unsigned variable Unsigned
Signed variable Signed, two’s-complement
Real variable Signed, floating point
The following example shows various ways to divide “minus twelve by three”—using integer and logic
variables in expressions.
integer intS;
var logic [15:0] U;
var logic signed [15:0] S;
intS = -4'd12;
U = intS / 3; // expression result is -4,

<!-- Page 279 -->

IEEE Std // intS is an integer data type, U is 65532
U = -4'd12; // U is 65524
intS = U / 3; // expression result is 21841,
// U is a logic data type
intS = -4'd12 / 3; // expression result is 1431655761.
// -4'd12 is effectively a 32-bit logic data type
U = -12 / 3; // expression result is -4, -12 is effectively
// an integer data type. U is 65532
S = -12 / 3; // expression result is -4. S is a signed logic
S = -4'sd12 / 3; // expression result is 1. -4'sd12 is actually 4.
// The rules for integer division yield 4/3==1
#### 11.4.4 Relational operators
Table11-8 lists and defines the relational operators.
Table11-8—Definitions of relational operators
a<b alessthanb
a>b agreaterthanb
a<=b alessthanorequaltob
a>=b agreaterthanorequaltob
An expression using these relational operators shall yield 1'b0 if the specified relation is false or 1'b1 if it
is true. If either operand of a relational operator contains an unknown (x) or high-impedance (z) value, then
the result shall be 1'bx.
When one or both operands of a relational expression are unsigned, the expression shall be interpreted as a
comparison between unsigned values. If the operands are of unequal bit lengths, the smaller operand shall be
zero-extended to the size of the larger operand.
When both operands are signed, the expression shall be interpreted as a comparison between signed values.
If the operands are of unequal bit lengths, the smaller operand shall be sign-extended to the size of the larger
operand. See 11.8.2 for more information.
If either operand is a real operand, then the other operand shall be converted to an equivalent real value and
the expression shall be interpreted as a comparison between real values.
All the relational operators shall have the same precedence. Relational operators shall have lower
precedence than arithmetic operators.
The following examples illustrate the implications of this precedence rule:
a < b - 1 // this expression is the same as
a < (b - 1) // this expression, but . . .
b - (1 < a) // this one is not the same as
b - 1 < a // this expression

<!-- Page 280 -->

IEEE Std When b - (1 < a) evaluates, the relational expression evaluates first, and then either zero or one is
subtracted from b. When b - 1 < a evaluates, the value of b operand is reduced by one and then
compared with a.
#### 11.4.5 Equality operators
The equality operators shall rank lower in precedence than the relational operators. Table11-9 lists and
defines the equality operators.
Table11-9—Definitions of equality operators
a===b aequaltob,includingxandz
a!==b anotequaltob,includingxandz
a ==b aequaltob,resultcan beunknown
a !=b anotequaltob,resultcan beunknown
All four equality operators shall have the same precedence. These four operators compare operands bit for
bit. As with the relational operators, the result shall be 1'b0 if the comparison fails and 1'b1 if it succeeds.
When one or both operands are unsigned, the expression shall be interpreted as a comparison between
unsigned values. If the operands are of unequal bit lengths, the smaller operand shall be zero-extended to the
size of the larger operand.
When both operands are signed, the expression shall be interpreted as a comparison between signed values.
If the operands are of unequal bit lengths, the smaller operand shall be sign-extended to the size of the larger
operand. See 11.8.2 for more information.
If either operand is a real operand, then the other operand shall be converted to an equivalent real value, and
the expression shall be interpreted as a comparison between real values.
The logical equality (or case equality) operator is a legal operation if either operand is a class handle or the
literal null, and one of the operands is assignment compatible with the other. The logical equality (or case
equality) operator is a legal operation if either operand is a chandle or the literal null. In both cases, the
operator compares the values of the class handles, interface class handles, or chandles.
For the logical equality and logical inequality operators (== and !=), if, due to unknown or high-impedance
bits in the operands, the relation is ambiguous, then the result shall be 1'bx.
For the case equality and case inequality operators (=== and !==), the comparison shall be done just as it is
in the procedural case statement (see 12.5). Bits that are x or z shall be included in the comparison and shall
match for the result to be considered equal. The result of these operators shall always be a known value,
either 1'b1 or 1'b0.

<!-- Page 281 -->

IEEE Std #### 11.4.6 Wildcard equality operators
The wildcard equality operators shall have the same precedence as the equality operators. Table11-10 lists
and defines the wildcard equality operators.
Table11-10—Wildcard equality and wildcard inequality operators
Operator Usage Description
==? a ==? b a equals b, x and z values in b act as wildcards
!=? a !=? b a does not equal b, x and z values in b act as wildcards
The wildcard equality operator (==?) and inequality operator (!=?) treat x and z values in a given bit
position of their right operand as a wildcard. x and z values in the left operand are not treated as wildcards.
A wildcard bit matches any bit value (0, 1, z, or x) in the corresponding bit of the left operand being
compared against it. Any other bits are compared as for the logical equality and logical inequality operators.
These operators compare operands bit for bit and return a 1-bit self-determined result. If the operands to the
wildcard equality/inequality are of unequal bit length, the operands are extended in the same manner as for
the logical equality/inequality operators. If the relation is true, the operator yields a 1'b1. If the relation is
false, it yields a 1'b0. If the relation is unknown, it yields 1'bx.
The different types of equality and inequality operators behave differently when their operands contain
unknown values (x or z). The == and != operators may result in 1'bx if any of their operands contains an x
or z. The === and !== operators explicitly check for 4-state values; therefore, x and z values shall either
match or mismatch, never resulting in 1'bx. The ==? and !=? operators may result in 1'bx if the left
operand contains an x or z that is not being compared with a wildcard in the right operand.
The wildcard equality operator is equivalent to the logical equality operator if its operands are class handles,
interface class handles, chandles or the literal null.
#### 11.4.7 Logical operators
The operators logical AND (&&), logical OR (||), logical implication (->), and logical equivalence (<->)
are logical connectives. The result of the evaluation of a logical operation shall be 1'b1 (defined as true),
1'b0(defined as false), or, if the result is ambiguous, the unknown value (1'bx). The precedence of && is
greater than that of ||, and both are lower than relational and equality operators. The precedence of -> and
<-> is at the same level, the binding of operands between the two operations is governed by associativity
(right), both are lower than other logical operators and the conditional operator.
The logical implication expression1 –> expression2 is logically equivalent to
(!expression1 || expression2), and the logical equivalence expression1 <–> expression2 is
logically equivalent to ((expression1 –> expression2) && (expression2 –> expression1)).
Each of the two operands of the logical equivalence operator shall be evaluated exactly once.
The unary logical negation operator (!) converts a nonzero or true operand into 1'b0 and a zero or false
operand into 1'b1. An ambiguous truth value results in 1'bx.
Example 1: If variable alpha holds the integer value 237 and beta holds the value 0, then the following
examples perform as described:
regA = alpha && beta; // regA is set to 0
regB = alpha || beta; // regB is set to 1

<!-- Page 282 -->

IEEE Std Example 2: The following expression performs a logical AND of three subexpressions without needing any
parentheses:
a < size-1 && b != c && index != lastone
However, it is recommended for readability purposes that parentheses be used to show very clearly the
precedence intended, as in the following rewrite of this example:
(a < size-1) && (b != c) && (index != lastone)
Example 3: A common use of ! is in constructions like the following:
if (!inword)
In some cases, the preceding construct makes more sense to someone reading the code than this equivalent
construct:
if (inword == 0)
The &&, ||, and -> operators shall use short-circuit evaluation as follows:
— The first operand expression shall always be evaluated.
— For && and ->, if the first operand value is logically false then the second operand shall not be
evaluated.
— For ||, if the first operand value is logically true then the second operand shall not be evaluated.
#### 11.4.8 Bitwise operators
The binary bitwise operators perform bitwise manipulations on integral operands. The binary bitwise AND
(&), OR (|), exclusive OR (^), and exclusive NOR (^~, ~^) operators combine a bit in one operand with the
corresponding bit in the other operand, resulting in one bit for each bit of the operands. The unary bitwise
negation operator (~) negates each bit of a single integral operand.
For the binary bitwise operators, if one or both operands are unsigned, the result is unsigned. If the operands
are of unequal bit lengths, the smaller operand shall be zero-extended to the size of the larger operand.
If both operands are signed, the result is signed. If the operands are of unequal bit lengths, the smaller
operand shall be sign-extended to the size of the larger operand. See 11.8.2 for more information.
For the unary bitwise negation operator, if the operand is unsigned, the result is unsigned. If the operand is
signed, the result is signed.
The following truth tables show the result for each operator and input operands.

<!-- Page 283 -->

IEEE Std Table11-11—Bitwise binary AND operator
& 0 1 x z
0 0 0 0 0
1 0 1 x x
x 0 x x x
z 0 x x x
Table11-12—Bitwise binary OR operator
| 0 1 x z
0 0 1 x x
1 1 1 1 1
x x 1 x x
z x 1 x x
Table11-13—Bitwise binary exclusive OR operator
^ 0 1 x z
0 0 1 x x
1 1 0 x x
x x x x x
z x x x x
Table11-14—Bitwise binary exclusive NOR operator
^~
0 1 x z
~^
0 1 0 x x
1 0 1 x x
x x x x x
z x x x x

<!-- Page 284 -->

IEEE Std Table11-15—Bitwise unary negation operator
~
0 1
1 0
x x
z x
#### 11.4.9 Reduction operators
The unary reduction operators shall perform a bitwise operation on a single operand to produce a single-bit
result. For reduction AND, reduction OR, and reduction XOR operators, the first step of the operation shall
apply the operator between the first bit of the operand and the second using Table11-16 through
Table11-18. The second and subsequent steps shall apply the operator between the 1-bit result of the prior
step and the next bit of the operand using the same logic table. For reduction NAND, reduction NOR, and
reduction XNOR operators, the result shall be computed by inverting the result of the reduction AND,
reduction OR, and reduction XOR operation, respectively.
Table11-16—Reduction unary AND operator
& 0 1 x z
0 0 0 0 0
1 0 1 x x
x 0 x x x
z 0 x x x
Table11-17—Reduction unary OR operator
| 0 1 x z
0 0 1 x x
1 1 1 1 1
x x 1 x x
z x 1 x x

<!-- Page 285 -->

IEEE Std Table11-18—Reduction unary exclusive OR operator
^ 0 1 x z
0 0 1 x x
1 1 0 x x
x x x x x
z x x x x
For example, Table11-19 shows the results of applying reduction operators on different operands.
Table11-19—Results of unary reduction operations
Operand & ~& | ~| ^ ~^ Comments
4'b0000 0 1 0 1 0 1 No bits set
4'b1111 1 0 1 0 0 1 All bits set
4'b0110 0 1 1 0 0 1 Even number of bits set
4'b1000 0 1 1 0 1 0 Odd number of bits set
#### 11.4.10 Shift operators
There are two types of shift operators: the logical shift operators, << and >>, and the arithmetic shift
operators, <<< and >>>. The left shift operators, << and <<<, shall shift their left operand to the left by the
number of bit positions given by the right operand. In both cases, the vacated bit positions shall be filled
with zeros. The right shift operators, >> and >>>, shall shift their left operand to the right by the number of
bit positions given by the right operand. The logical right shift shall fill the vacated bit positions with zeros.
The arithmetic right shift shall fill the vacated bit positions with zeros if the result type is unsigned. It shall
fill the vacated bit positions with the value of the most significant (i.e., sign) bit of the left operand if the
result type is signed. If the right operand has an x or z value, then the result shall be unknown. The right
operand is always treated as an unsigned number and has no effect on the signedness of the result. The result
signedness is determined by the left-hand operand and the remainder of the expression, as outlined in 11.8.1.
Example 1: In this example, the variable result is assigned the binary value 0100, which is 0001 shifted to
the left two positions and zero-filled.
module shift;
logic [3:0] start, result;
initial begin
start = 1;
result = (start << 2);
end
endmodule
Example 2: In this example, the variable result is assigned the binary value 1110, which is 1000 shifted to
the right two positions and sign-filled.
module ashift;
logic signed [3:0] start, result;
initial begin

<!-- Page 286 -->

IEEE Std start = 4'b1000;
result = (start >>> 2);
end
endmodule
#### 11.4.11 Conditional operator
The conditional operator shall be right associative and shall be constructed using three operands separated
by two operators in the format given in Syntax11-2.
conditional_expression ::= // from A.8.3
cond_predicate ? { attribute_instance } expression : expression
cond_predicate ::= expression_or_cond_pattern { &&& expression_or_cond_pattern } // from A.6.6
expression_or_cond_pattern ::= expression | cond_pattern
cond_pattern ::= expression matches pattern
Syntax11-2—Conditional operator syntax (excerpt from AnnexA)
This subclause describes the traditional notation where cond_predicate is just a single expression.
SystemVerilog also allows cond_predicate to perform pattern matching, which is described in 12.6.
If cond_predicate is true, the operator returns the value of the first expression without evaluating the second
expression; if false, it returns the value of the second expression without evaluating the first expression. If
cond_predicate evaluates to an ambiguous value (x or z), then both the first expression and the second
expression shall be evaluated, and compared for logical equivalence as described in 11.4.5. If that
comparison is true (1), the operator shall return either the first or second expression. Otherwise the operator
returns a result based on the data types of the expressions.
When both the first and second expressions are of integral types, if the cond_predicate evaluates to an
ambiguous value and the expressions are not logically equivalent, their results shall be combined bit by bit
using Table11-20 to calculate the final result. The first and second expressions are extended to the same
width, as described in 11.6.1 and 11.8.2.
Table11-20—Ambiguous condition results for conditional operator
?: 0 1 x z
0 0 x x x
1 x 1 x x
x x x x x
z x x x x
The following example of a three-state output bus illustrates a common use of the conditional operator:
wire [15:0] busa = drive_busa ? data : 16'bz;
The bus called data is driven onto busa when drive_busa is 1. If drive_busa is unknown, then an
unknown value is driven onto busa. Otherwise, busa is not driven.

<!-- Page 287 -->

IEEE Std The conditional operator can be used with nonintegral types (see 6.11.1) and aggregate expressions (see
11.2.2) using the following rules:
— If both the first expression and second expression are of integral types, the operation proceeds as
defined.
— If both expressions are real, then the resulting type is real. If one expression is real and the other
expression is shortreal or integral, the other expression is cast to real, and the resulting type is
real. If one expression is shortreal and the other expression is integral, the integral expression is
cast to shortreal, and the resulting type is shortreal.
— Otherwise, if the first expression or second expression is an integral type and the opposing
expression can be implicitly cast to an integral type, the cast is made and proceeds as defined.
— If the first expression or second expression is a class or interface class data type, the condition
expression is legal in the following cases:
a) If both first expression and second expression are the literal value null, the result of the entire
conditional expression is as if the expression were the literal null.
b) Else, if either first expression or second expression is the literal null, the resulting type is the
type of the non-null expression.
c) Else, if the first expression is assignment compatible with the second expression, the resulting
type is the type of the second expression,
d) Else, if the second expression is assignment compatible with the first expression, the resulting
type is the type of the first expression,
e) Else, if the first expression and second expression are of a class type deriving from a common
base class type, the resulting type is the closest common inherited class type.
In the preceding cases, the resulting value is the value of the first expression if the condition
evaluates to true or the value of the second expression if the condition evaluates to false.
— For all other cases, the type of the first expression and second expression shall be equivalent (see
6.22.2).
For nonintegral and aggregate expressions, if cond_predicate evaluates to an ambiguous value and the
expressions are not logically equivalent, then:
— For aggregate array data types, except associative arrays, where both expressions contain the same
number of elements their results shall be combined element by element. If the elements match, the
element shall be returned. If they do not match, then the value specified in Table7-1 for that
element’s type shall be returned.
— For all other data types, the value specified in Table7-1 for the resulting type as defined previously
shall be returned.
#### 11.4.12 Concatenation operators
A concatenation is the result of the joining together of bits resulting from one or more expressions. The
concatenation shall be expressed using the brace characters { and }, with commas separating the expressions
within.
Unsized constant numbers shall not be allowed in concatenations. This is because the size of each operand in
the concatenation is needed to calculate the complete size of the concatenation.
The following example concatenates four expressions:
{a, b[3:0], w, 3'b101}
The preceding example is equivalent to the following example:

<!-- Page 288 -->

IEEE Std {a, b[3], b[2], b[1], b[0], w, 1'b1, 1'b0, 1'b1}
The concatenation is treated as a packed vector of bits. It can be used on the left-hand side of an assignment
or in an expression.
logic log1, log2, log3;
{log1, log2, log3} = 3'b111;
{log1, log2, log3} = {1'b1, 1'b1, 1'b1}; // same effect as 3'b111
One or more bits of a concatenation can be selected as if the concatenation were a packed array with the
range [n-1:0], where n is the number of bits in the concatenation. Such a select shall not be legal as a
net_lvalue, variable_lvalue, or in any equivalent use, such as on the left-hand side of an assignment. For
example:
byte a, b ;
bit [1:0] c ;
c = {a + b}[1:0]; // 2 LSBs of sum of a and b
A concatenation is not the same as a structure literal (see 5.10) or an array literal (see 5.11). Concatenations
are enclosed in just braces ( { } ), whereas structure and array literals are enclosed in braces that begin with
an apostrophe ( '{ } ).
11.4.12.1 Replication operator
A replication operator (also called a multiple concatenation) is expressed by a concatenation preceded by a
non-negative, non-x, and non-z constant expression, called a multiplier, enclosed together within brace
characters. A replication indicates a joining together of that many copies of the concatenation. Unlike
regular concatenations, expressions containing replications shall not appear on the left-hand side of an
assignment and shall not be connected to output or inout ports.
This example replicates w four times.
{4{w}} // yields the same value as {w, w, w, w}
The following examples show illegal replications:
{1'bz{1'b0}} // illegal
{1'bx{1'b0}} // illegal
The next example illustrates a replication nested within a concatenation:
{b, {3{a, b}}} // yields the same value as {b, a, b, a, b, a, b}
A replication operation may have a multiplier with a value of zero. This is useful in parameterized code. A
replication with a zero multiplier is considered to have a size of zero and is ignored. Such a replication shall
appear only within a concatenation in which at least one of the operands of the concatenation has a positive
size.
For example:
parameter P = 32;
// The following is legal for all P from 1 to 32
assign b[31:0] = { {32-P{1'b1}}, a[P-1:0] } ;
// The following is illegal for P=32 because the zero

<!-- Page 289 -->

IEEE Std // replication appears alone within a concatenation
assign c[31:0] = { {{32-P{1'b1}}}, a[P-1:0] }
// The following is illegal for P=32
initial
$displayb({32-P{1'b1}}, a[P-1:0]);
When a replication expression is evaluated, the operands shall be evaluated exactly once, even if the
multiplier is zero. For example:
result = {4{func(w)}} ;
would be computed as
y = func(w) ;
result = {y, y, y, y} ;
11.4.12.2 String concatenation and replication
A concatenation of data objects of type string is allowed. In general, if any of the operands is of the data
type string, the concatenation is treated as a string, and all other arguments are implicitly converted to the
string data type (as described in 6.16). String concatenation is not allowed on the left-hand side of an
assignment, only as an expression.
string hello = "hello";
string s;
s = {hello, " ", "world"};
$display("%s\n", s); // displays 'hello world'
s = {s, " and goodbye"};
$display("%s\n", s); // displays 'hello world and goodbye'
The replication operator form of braces can also be used with data objects or concatenations of type string.
In the case of string replication, a non-constant multiplier is allowed.
int n = 3;
string s = {n {"boo "}};
$display("%s\n", s); // displays 'boo boo boo '
Unlike bit concatenation, the result of a string concatenation or replication is not truncated. Instead, the
destination variable (of type string) is resized to accommodate the resulting string.
See 6.16 for more information on string concatenation and replication.
#### 11.4.13 Set membership operator
SystemVerilog supports singular value sets and a set membership operator.
The syntax for the set membership operator is as follows:

<!-- Page 290 -->

IEEE Std inside_expression ::= expression inside { range_list } // from A.8.3
range_list ::= value_range { , value_range } // from A.6.7
value_range ::=
expression
| [ expression : expression ]
| [ $ : expression ]
| [ expression : $ ]
| [ expression +/- expression ]
| [ expression +%- expression ]
Syntax11-3—Inside expression syntax (excerpt from AnnexA)
The expression on the left-hand side of the inside operator is any singular expression.
The range_list on the right-hand side of the inside operator is a comma-separated list of expressions or
ranges. The members of the set are scanned until a match is found and the operation returns 1'b1. If no
match is found, the inside operator returns 1'b0. If an expression in the list is an unpacked array, its
elements are traversed by descending into the array until reaching a singular value. Values can be repeated;
therefore, values and value ranges can overlap. The order of evaluation of the expressions and ranges is
nondeterministic.
int a, b, c;
if (a inside {b, c}) ...
int array [$] = '{3,4,5};
if (ex inside {1, 2, array}) ... // same as {1, 2, 3, 4, 5}
The inside operator uses the equality (==) operator on nonintegral expressions to perform the comparison.
Integral expressions use the wildcard equality (==?) operator so that an x or z bit in a value in the set is
treated as a do-not-care in that bit position (see 11.4.6). As with wildcard equality, an x or z in the
expression on the left-hand side of the inside operator is not treated as a do-not-care.
logic [2:0] val;
while (val inside {3'b1?1}) ... // matches 3'b101, 3'b111, 3'b1x1, 3'b1z1
If no match is found, but some of the comparisons result in x, the inside operator shall return 1'bx. The
return value is effectively the OR reduction of all the comparisons in the set with the expression on the
left-hand side.
wire r;
assign r = 3'bz11 inside {3'b1?1, 3'b011}; // r = 1'bx
A range can be specified with a low and high bound enclosed by square braces [ ] and separated by a colon
( : ), as in [low_bound:high_bound]. A bound specified by $ shall represent the lowest or highest value
for the type of the expression on the left-hand side. A match is found if the expression on the left-hand side
is inclusively within the range. When specifying a range, the expressions shall be of a singular type for
which the relation operators ( <=, >= ) are defined. If the bound to the left of the colon is greater than the
bound to the right, the range is empty and contains no values.
For example:
bit ba = a inside {[16:23], [32:47]};
string I;
if (I inside {["a rock":"hard place"]}) ...

<!-- Page 291 -->

IEEE Std // I between "a rock" and a "hard place"
A range may also be specified by two expressions separated by the absolute tolerance token ( +/- ) or the
relative tolerance token ( +%- ).
[A +/- B] designates an absolute tolerance range, defined as
[A-type(A)’(B) : A+type(A)’(B)].
[A +%- B] designates a relative tolerance range, defined as
[A-type(A)’(A*B/100.0) : A+type(A)’(A*B/100.0)].
Regardless of the signedness of A and B, the bounds are always considered as [low_bound:high_bound],
swapping the two resulting values as necessary. The resulting type of the range is always the same type as A.
For real to integral conversions of the tolerance, truncation occurs instead of rounding.
For example, let A = 7.0. A 25% tolerance of A would be expressed as; [A+%-25] and the result would be
the range [5.25:8.75]. If A is integral (7 or -7), the resultant range is converted to integral type by truncation
of the tolerance from 1.75 (25% of 7) to 1; thus, [5.25:8.75] is converted to [6:8] and [-8.75:-5.25] is
converted to [-8:-6].
#### 11.4.14 Streaming operators (pack/unpack)
The bit-stream casting described in 6.24.3 is most useful when the conversion operation can be easily
expressed using only a type cast and the specific ordering of the bit stream is not important. Sometimes,
however, a stream that matches a particular machine organization is required. The streaming operators
perform packing of bit-stream types (see 6.24.3) into a sequence of bits in a user-specified order. When used
in the left-hand side, the streaming operators perform the reverse operation, i.e., unpack a stream of bits into
one or more variables.
If the data being packed contains any 4-state types, the result of a pack operation is a 4-state stream;
otherwise, the result of a pack is a 2-state stream. In the remainder of this subclause, the word bit, without
other qualification, denotes either a 2-state or a 4-state bit as required by this paragraph.
The syntax of the bit-stream concatenation is as follows:
streaming_concatenation ::= { stream_operator [ slice_size ] stream_concatenation } // from A.8.1
stream_operator ::= >> | <<
slice_size ::= simple_type | constant_expression
stream_concatenation ::= { stream_expression { , stream_expression } }
stream_expression ::= expression [ with [ array_range_expression ] ]
array_range_expression ::=
expression
| expression : expression
| expression +: expression
| expression -: expression
primary ::= // from A.8.4
...
| streaming_concatenation
Syntax11-4—Streaming concatenation syntax (excerpt from AnnexA)

<!-- Page 292 -->

IEEE Std A streaming_concatenation (as specified in Syntax11-4) shall be used either as the target of an assignment,
or as the source of an assignment, or as the operand of a bit-stream cast, or as a stream_expression in another
streaming_concatenation. Use of streaming_concatenation as the target of an assignment, and the
associated unpack operation, is described in 11.4.14.3.
It shall be an error to use a streaming_concatenation as an operand in an expression without first casting it to
a bit-stream type. When a streaming_concatenation is used as the source of an assignment, the target of that
assignment shall be either a data object of bit-stream type or a streaming_concatenation.
If the target is a data object of bit-stream type, the stream created by the source streaming_concatenation
shall first be widened if necessary to left-align it in the target, as follows:
— If the target represents a fixed-size variable that is narrower (has fewer bits) than the stream, an error
shall be generated.
— If the target represents a fixed-size variable that is wider than the stream, the stream shall be
widened to match it by filling with zero bits on the right.
— If the target represents a dynamically sized variable, such as a queue or dynamic array, the variable
shall first be resized so that it has the smallest number of elements that make it as wide as or wider
than the stream. If the resized variable is wider than the stream, the stream shall then be widened to
match it by filling with zero bits on the right.
The stream, widened if necessary as described previously, shall then be implicitly cast to the type of the
target.
The pack operation performed by a streaming_concatenation is described in two steps for convenience, but
the intermediate result between the two steps is never visible and therefore tools are free to implement it in
any way that yields the same overall result. First, all integral data in the stream_expressions are
concatenated into a single stream of bits, similarly to bit-stream casting (as described in 6.24.3) but with
fewer restrictions. Second, the resulting stream may be re-ordered in a manner specified by the
stream_operator and slice_size. These two steps are described in more detail in 11.4.14.1 and 11.4.14.2.
11.4.14.1 Concatenation of stream_expressions
Each stream_expression within the stream_concatenation, starting with the leftmost and proceeding from
left to right through the comma-separated list of stream_expressions, is converted to a bit-stream and
appended to a packed array (stream) of bits, the generic stream, by recursively applying the following
procedure:
if the expression is a streaming_concatenation or it is of any bit-stream type,
it shall be cast to a packed array of bit using a bit-stream cast, including casting 2-state to
4-state if necessary, and that packed array shall then be appended to the right-hand end of the
generic stream;
elseif the expression is an unpacked array (i.e., a queue, dynamic array, associative array, or fixed-size
unpacked array)
this procedure shall be applied in turn to each element of the array. An associative array is
processed in index-sorted order. Other unpacked arrays are processed in the order in which
they would be traversed by a foreach loop (see 12.7.3) having exactly one index variable;
elseif the expression is of a struct type
this procedure shall be applied in turn to each element of the struct, in declaration order;
elseif the expression is of an untagged union type
this procedure shall be applied to the first-declared member of the union;

<!-- Page 293 -->

IEEE Std elseif the expression is a null class handle
the expression shall be skipped (not streamed), and a warning may be issued;
elseif the expression is a non-null class handle
this procedure shall be applied in turn to each data member of the referenced object, and not
the handle itself. Class members shall be streamed in declaration order. Extended class
members shall be streamed after members of their base class. The result of streaming an
object hierarchy that contains cycles shall be undefined, and an error may be issued. It shall
be illegal to stream a class handle with local or protected members if those members would
not be accessible at the point of the streaming operator;
else
the expression shall be skipped (not streamed), and an error shall be issued.
In the preceding description, the phrase skipped (not streamed) means that the expression in question is not
appended to the stream, and operation of the procedure then proceeds with the next item in turn.
Implementations are not required to continue the procedure after issuing an error.
11.4.14.2 Re-ordering of the generic stream
The stream resulting from the operation described in 11.4.14.1 is then re-ordered by slicing it into blocks
and then re-ordering those blocks.
The slice_size determines the size of each block, measured in bits. If a slice_size is not specified, the default
is 1. If specified, it may be a constant integral expression or a simple type. If a type is used, the block size
shall be the number of bits in that type. If a constant integral expression is used, it shall be an error for the
value of the expression to be zero or negative.
The stream_operator << or >> determines the order in which blocks of data are streamed: >> causes blocks
of data to be streamed in left-to-right order, while << causes blocks of data to be streamed in right-to-left
order. Left-to-right streaming using >> shall cause the slice_size to be ignored and no re-ordering performed.
Right-to-left streaming using << shall reverse the order of blocks in the stream, preserving the order of bits
within each block. For right-to-left streaming using <<, the stream is sliced into blocks with the specified
number of bits, starting with the right-most bit. If as a result of slicing the last (left-most) block has fewer
bits than the block size, the last block has the size of the remaining bits; there is no padding or truncation.
For example:
int j = { "A", "B", "C", "D" };
{ >> {j}} // generates stream "A" "B" "C" "D"
{ << byte {j}} // generates stream "D" "C" "B" "A" (little endian)
{ << 16 {j}} // generates stream "C" "D" "A" "B"
{ << { 8'b0011_0101 }} // generates stream 'b1010_1100 (bit reverse)
{ << 4 { 6'b11_0101 }} // generates stream 'b0101_11
{ >> 4 { 6'b11_0101 }} // generates stream 'b1101_01 (same)
{ << 2 { { << { 4'b1101 }} }} // generates stream 'b1110
11.4.14.3 Streaming concatenation as an assignment target (unpack)
When a streaming_concatenation appears as the target of an assignment, the streaming operators perform
the reverse operation; i.e., to unpack a stream of bits into one or more variables. The source expression shall
be of bit-stream type, or the result of another streaming_concatenation. If the source expression contains
more bits than are needed, the appropriate number of bits shall be consumed from its left (most significant)
end. However, if more bits are needed than are provided by the source expression, an error shall be
generated.
Unpacking a 4-state stream into a 2-state target is done by casting to a 2-state type, and vice versa. Null
handles are skipped by both the pack and unpack operations; therefore, the unpack operation shall not create

<!-- Page 294 -->

IEEE Std class objects. If a particular object hierarchy is to be reconstructed from a stream, the object hierarchy into
which the stream is to be unpacked shall be created before the streaming operator is applied. The unpack
operation shall only modify explicitly declared properties; it will not modify implicitly declared properties
such as random modes (see Clause18).
For example:
int a, b, c;
logic [10:0] up [3:0];
logic [11:1] p1, p2, p3, p4;
bit [96:1] y = {>>{ a, b, c }}; // OK: pack a, b, c
int j = {>>{ a, b, c }}; // error: j is 32 bits < 96 bits
bit [99:0] d = {>>{ a, b, c }}; // OK: d is padded with 4 bits
{>>{ a, b, c }} = 23'b1; // error: too few bits in stream
{>>{ a, b, c }} = 96'b1; // OK: unpack a = 0, b = 0, c = 1
{>>{ a, b, c }} = 100'b11111; // OK: unpack a = 0, b = 0, c = 1
// 96 MSBs unpacked, 4 LSBs truncated
{ >> {p1, p2, p3, p4}} = up; // OK: unpack p1 = up[3], p2 = up[2],
// p3 = up[1], p4 = up[0]
11.4.14.4 Streaming dynamically sized data
If the unpack operation includes unbounded dynamically sized types, the process is greedy (as in a cast): the
first dynamically sized item is resized to accept all the available data (excluding subsequent fixed-size
items) in the stream; any remaining dynamically sized items are left empty. This mechanism is sufficient to
unpack a packet-sized stream that contains only one dynamically sized data item. However, when the stream
contains multiple variable-size data packets, or each data packet contains more than one variable-size data
item, or the size of the data to be unpacked is stored in the middle of the stream, this mechanism can become
cumbersome and error-prone. To overcome these problems, the unpack operation allows a with expression
to explicitly specify the extent of a variable-size field within the unpack operation.
The syntax of the with expression is as follows:
stream_expression ::= expression [ with [ array_range_expression ] ] // from A.8.1
array_range_expression ::=
expression
| expression : expression
| expression +: expression
| expression -: expression
Syntax11-5—With expression syntax (excerpt from AnnexA)
The array range expression within the with construct shall be of integral type and evaluate to values that lie
within the bounds of a fixed-size array or to positive values for dynamic arrays or queues. The expression
before the with can be any one-dimensional unpacked array (including a queue). The expression within the
with is evaluated immediately before its corresponding array is streamed (i.e., packed or unpacked). Thus,
the expression can refer to data that are unpacked by the same operator but before the array. If the expression
refers to variables that are unpacked after the corresponding array (to the right of the array), then the
expression is evaluated using the previous values of the variables.
When used within the context of an unpack operation and the array is a variable-size array, it shall be resized
to accommodate the range expression. If the array is a fixed-size array and the range expression evaluates to
a range outside the extent of the array, only the range that lies within the array is unpacked and an error is
generated. If the range expression evaluates to a range smaller than the extent of the array (fixed or variable

<!-- Page 295 -->

IEEE Std size), only the specified items are unpacked into the designated array locations; the remainder of the array is
unmodified.
When used within the context of a pack (on the right-hand side), it behaves the same as an array slice. The
specified number of array items are packed into the stream. If the range expression evaluates to a range
smaller than the extent of the array, only the specified array items are streamed. If the range expression
evaluates to a range greater than the extent of the array size, the entire array is streamed, and the remaining
items are generated using the nonexistent array entry value (as described in Table7-1 in 7.4.5) for the given
array.
For example, the following code uses streaming operators to model a packet transfer over a byte stream that
uses little-endian encoding:
byte stream[$]; // byte stream
class Packet;
rand int header;
rand int len;
rand byte payload[];
int crc;
constraint G { len > 1; payload.size == len ; }
function void post_randomize; crc = payload.sum; endfunction
endclass
...
send: begin // Create random packet and transmit
byte q[$];
Packet p = new;
void'(p.randomize());
q = {<< byte{p.header, p.len, p.payload, p.crc}}; // pack
stream = {stream, q}; // append to stream
end
...
receive: begin // Receive packet, unpack, and remove
byte q[$];
Packet p = new;
{<< byte{ p.header, p.len, p.payload with [0 +: p.len], p.crc }} = stream;
stream = stream[ $bits(p) / 8 : $ ]; // remove packet
end
In the preceding example, the pack operation could have been written as either:
q = {<<byte{p.header, p.len, p.payload with [0 +: p.len], p.crc}};
or
q = {<<byte{p.header, p.len, p.payload with [0 : p.len-1], p.crc}};
or
q = {<<byte{p}};
The result in this case would be the same because p.len is the size of p.payload as specified by the
constraint.

<!-- Page 296 -->

IEEE Std ### 11.5 Operands
There are several types of operands that can be specified in expressions. The simplest type is a reference to a
net, variable, or parameter in its complete form; that is, just the name of the net, variable, or parameter is
given. In this case, all of the bits making up the net, variable, or parameter value shall be used as the
operand.
If a single bit of a vector net, vector variable, packed array, packed structure or parameter is required, then a
bit-select operand shall be used. A part-select operand shall be used to reference a group of adjacent bits in a
vector net, vector variable, packed array, packed structure, or parameter.
An unpacked array element can be referenced as an operand.
A concatenation of other operands (including nested concatenations) can be specified as an operand.
A function call is an operand.
Each of the types of operands mentioned previously is an example of a simple operand. An operand is
simple if it is not parenthesized and is a primary as defined in A.8.4. In the following example, the
expressions 1'b1 - 2'b00 and (1'b1 + 1'b1) are operands, but are not simple operands.
1'b1 - 2'b00 + (1'b1 + 1'b1)
#### 11.5.1 Vector bit-select and part-select addressing
Bit-selects extract a particular bit from a vector, packed array, packed structure, parameter, or concatenation.
The bit can be addressed using an expression that shall be evaluated in a self-determined context. If the bit-
select address is invalid (it is out of bounds or has one or more x or z bits), then the value returned by the
reference shall be x for 4-state and 0 for 2-state values. A bit-select or part-select of a scalar, or of a real
variable or real parameter, shall be illegal.
Several contiguous bits can be addressed and are known as part-selects. There are two types of part-selects:
a non-indexed part-select and an indexed part-select. A non-indexed part-select is given with the following
syntax:
vect[msb_expr:lsb_expr]
Both msb_expr and lsb_expr shall be constant integer expressions. Each of these expressions shall be
evaluated in a self-determined context. The first expression shall address a more significant bit than the
second expression.
An indexed part-select is given with the following syntax:
logic [15:0] down_vect;
logic [0:15] up_vect;
down_vect[lsb_base_expr +: width_expr]
up_vect[msb_base_expr +: width_expr]
down_vect[msb_base_expr -: width_expr]
up_vect[lsb_base_expr -: width_expr]
The msb_base_expr and lsb_base_expr shall be integer expressions, and the width_expr shall be a positive
constant integer expression. Each of these expressions shall be evaluated in a self-determined context. The
lsb_base_expr and msb_base_expr can vary at run time. The first two examples select bits starting at the

<!-- Page 297 -->

IEEE Std base and ascending the bit range. The number of bits selected is equal to the width expression. The second
two examples select bits starting at the base and descending the bit range.
A constant bit-select is a bit-select whose position is constant. A constant part-select is a part-select whose
position and width are both constant. The width of a part-select is always constant. Thus, a non-indexed
part-select is always a constant part-select, and an indexed part-select is a constant part-select if its base is a
constant value as well as its width.
A part-select that addresses a range of bits that are completely out of the address bounds of the vector,
packed array, packed structure, parameter or concatenation, or a part-select that is x or z shall yield the value
x when read and shall have no effect on the data stored when written. Part-selects that are partially out of
range shall, when read, return x for the bits that are out of range and shall, when written, only affect the bits
that are in range.
For example:
logic [31: 0] a_vect;
logic [0 :31] b_vect;
logic [63: 0] dword;
integer sel;
a_vect[ 0 +: 8] // == a_vect[ 7 : 0]
a_vect[15 -: 8] // == a_vect[15 : 8]
b_vect[ 0 +: 8] // == b_vect[0 : 7]
b_vect[15 -: 8] // == b_vect[8 :15]
dword[8*sel +: 8] // variable part-select with fixed width
The following example specifies the single bit of vector acc that is addressed by the operand index:
acc[index]
The actual bit that is accessed by an address is, in part, determined by the declaration of acc. For instance,
each of the declarations of acc shown in the next example causes a particular value of index to access a
different bit:
logic [15:0] acc;
logic [2:17] acc;
The next example and the bullet items that follow it illustrate the principles of bit addressing. The code
declares an 8-bit variable called vect and initializes it to a value of 4. The list describes how the separate
bits of that vector can be addressed.
logic [7:0] vect;
vect = 4; // fills vect with the pattern 00000100
// msb is bit 7, lsb is bit 0
— If the value of addr is 2, then vect[addr] returns 1.
— If the value of addr is out of bounds, then vect[addr] returns x.
— If addr is 0, 1, or 3 through 7, vect[addr] returns 0.
— vect[3:0] returns the bits 0100.
— vect[5:1] returns the bits 00010.
— vect[expression that returns x] returns x.

<!-- Page 298 -->

IEEE Std — vect[expression that returns z] returns x.
— If any bit of addr is x or z, then the value of addr is x.
NOTE 1—Part-select indices that evaluate to x or z may be flagged as a compile-time error.
NOTE 2—Bit-select or part-select indices that are outside the declared range may be flagged as a compile-time error.
#### 11.5.2 Array and memory addressing
Declaration of arrays and memories (one-dimensional arrays of reg, logic, or bit) are discussed in 7.4.
This subclause discusses array addressing.
The following example declares a memory of 1024 8-bit words:
logic [7:0] mem_name[0:1023];
The syntax for a memory address shall consist of the name of the memory and an expression for the address,
specified with the following format:
mem_name[addr_expr]
The addr_expr can be any integer expression; therefore, memory indirections can be specified in a single
expression. The next example illustrates memory indirection:
mem_name[mem_name[3]]
In this example, mem_name[3] addresses word three of the memory called mem_name. The value at word
three is the index into mem_name that is used by the memory address mem_name[mem_name[3]]. As with
bit-selects, the address bounds given in the declaration of the memory determine the effect of the address
expression. If the address is invalid (it is out of bounds or has one or more x or z bits), then the value of the
reference shall be as described in 7.4.5.
The next example declares an array of 256-by-256 8-bit elements and an array 256-by-256-by-8 1-bit
elements:
logic [7:0] twod_array[0:255][0:255];
wire threed_array[0:255][0:255][0:7];
The syntax for access to the array shall consist of the name of the memory or array and an integer expression
for each addressed dimension:
twod_array[addr_expr][addr_expr]
threed_array[addr_expr][addr_expr][addr_expr]
As before, the addr_expr can be any integer expression. The array twod_array accesses a whole 8-bit
vector, while the array threed_array accesses a single bit of the three-dimensional array.
To express bit-selects or part-selects of array elements, the desired word shall first be selected by supplying
an address for each dimension. Once selected, bit-selects and part-selects shall be addressed in the same
manner as net and variable bit-selects and part-selects (see 11.5.1).
For example:
twod_array[14][1][3:0] // access lower 4 bits of word
twod_array[1][3][6] // access bit 6 of word
twod_array[1][3][sel] // use variable bit-select
threed_array[14][1][3:0] // Illegal

<!-- Page 299 -->

IEEE Std #### 11.5.3 Longest static prefix
Informally, the longest static prefix of a select is the longest part of the select for which an analysis tool has
known values following elaboration. This concept is used when describing implicit sensitivity lists (see
9.2.2.2) and when describing error conditions for drivers of logic ports (see 6.5). The remainder of this
subclause defines what constitutes the “longest static prefix” of a select.
A field select is defined as a hierarchical name where the right-hand side of the last “.” hierarchy separator
identifies a field of a variable whose type is a struct or union declaration. The field select prefix is
defined to be the left-hand side of the final “.” hierarchy separator in a field select.
An indexing select is a single indexing operation. The indexing select prefix is either an identifier or, in the
case of a multidimensional select, another indexing select. Array selects, bit-selects, part-selects, and
indexed part-selects are examples of indexing selects.
The definition of a static prefix is recursive and is defined as follows:
— An identifier is a static prefix.
— A hierarchical reference to an object is a static prefix.
— A package reference to net or variable is a static prefix.
— A field select is a static prefix if the field select prefix is a static prefix.
— An indexing select is a static prefix if the indexing select prefix is a static prefix and the select
expression is a constant expression.
The definition of the longest static prefix is defined as follows:
— An identifier that is not the field select prefix or indexing select prefix of an expression that is a
static prefix.
— A field select that is not the field select prefix or indexing select prefix of an expression that is a
static prefix.
— An indexing select that is not the field select prefix or indexing select prefix of an expression that is
a static prefix.
Examples:
localparam p = 7;
reg [7:0] m [5:1][5:1];
integer i;
m[1][i] // longest static prefix is m[1]
m[p][1] // longest static prefix is m[p][1]
m[i][1] // longest static prefix is m
### 11.6 Expression bit lengths
The number of bits of an expression is determined by the operands and the context. Casting can be used to
set the size context of an intermediate value (see 6.24).
Controlling the number of bits that are used in expression evaluations is important if consistent results are to
be achieved. Some situations have a simple solution; for example, if a bitwise AND operation is specified on
two 16-bit variables, then the result is a 16-bit value. However, in some situations, it is not obvious how
many bits are used to evaluate an expression or what size the result should be.

<!-- Page 300 -->

IEEE Std For example, should an arithmetic add of two 16-bit values perform the evaluation using 16 bits, or should
the evaluation use 17 bits in order to allow for a possible carry overflow? The answer depends on the type of
device being modeled and whether that device handles carry overflow. SystemVerilog uses the bit length of
the operands to determine how many bits to use while evaluating an expression. The bit length rules are
given in 11.6.1. In the case of the addition operator, the bit length of the largest operand, including the left-
hand side of an assignment, shall be used.
For example:
logic [15:0] a, b; // 16-bit variables
logic [15:0] sumA; // 16-bit variable
logic [16:0] sumB; // 17-bit variable
sumA = a + b; // expression evaluates using 16 bits
sumB = a + b; // expression evaluates using 17 bits
#### 11.6.1 Rules for expression bit lengths
The rules governing the expression bit lengths have been formulated so that most practical situations have a
natural solution.
The number of bits of an expression (known as the size of the expression) shall be determined by the
operands involved in the expression and the context in which the expression is given.
A self-determined expression is one where the bit length of the expression is solely determined by the
expression itself—for example, an expression representing a delay value.
A context-determined expression is one where the bit length of the expression is determined by the bit length
of the expression and by the fact that it is part of another expression. For example, the bit size of the
right-hand expression of an assignment depends on itself and the size of the left-hand side.
Table11-21 shows how the form of an expression shall determine the bit lengths of the results of the
expression. In Table11-21, i, j, and k represent expressions of an operand, and L(i) represents the bit
length of the operand represented by i.
Table11-21—Bit lengths resulting from self-determined expressions
Expression Bitlength Comments
Unsized constant number At least 32 bits
Sized constant number As given
i op j, where op is: max(L(i),L(j))
+ - * / % & | ^ ^~ ~^
op i, where op is: L(i)
+ - ~
i op j, where op is: 1 bit Operands are sized to max(L(i),L(j))
=== !== == != > >= < <=
i op j, where op is: 1 bit All operands are self-determined
&& || –> <->
op i, where op is: 1 bit All operands are self-determined
& ~& | ~| ^ ~^ ^~ !

<!-- Page 301 -->

IEEE Std Table11-21—Bit lengths resulting from self-determined expressions (continued)
Expression Bitlength Comments
i op j, where op is: L(i) j is self-determined
>> << ** >>> <<<
i ? j : k max(L(j),L(k)) i is self-determined
{i,...,j} L(i)+...+L(j) All operands are self-determined
{i{j,...,k}} i × (L(j)+...+L(k)) All operands are self-determined
Multiplication may be performed without losing any overflow bits by assigning the result to something wide
enough to hold it.
#### 11.6.2 Example of expression bit-length problem
During the evaluation of an expression, interim results shall take the size of the largest operand (in case of an
assignment, this also includes the left-hand side). Care has to be taken to prevent loss of a significant bit
during expression evaluation. The following example describes how the bit lengths of the operands could
result in the loss of a significant bit.
Given the following declarations:
logic [15:0] a, b, answer; // 16-bit variables
the intent is to evaluate the expression
answer = (a + b) >> 1; // will not work properly
where a and b are to be added, which can result in an overflow, and then shifted right by 1 bit to preserve
the carry bit in the 16-bit answer.
A problem arises, however, because all operands in the expression are of a 16-bit width. Therefore, the
expression (a + b) produces an interim result that is only 16 bits wide, thus losing the carry bit before the
evaluation performs the 1-bit right shift operation.
The solution is to force the expression (a + b) to evaluate using at least 17 bits. For example, adding an
integer value of 0 to the expression will cause the evaluation to be performed using the bit size of integers.
The following example will produce the intended result:
answer = (a + b + 0) >> 1; // will work correctly
In the following example:
module bitlength();
logic [3:0] a, b, c;
logic [4:0] d;
initial begin
a = 9;
b = 8;
c = 1;
$display("answer = %b", c ? (a&b) : d);
end
endmodule

<!-- Page 302 -->

IEEE Std the $display statement will display
answer = 01000
By itself, the expression a&b would have the bit length 4, but because it is in the context of the conditional
expression, which uses the maximum bit length, the expression a&b actually has length 5, the length of d.
#### 11.6.3 Example of self-determined expressions
logic [3:0] a;
logic [5:0] b;
logic [15:0] c;
initial begin
a = 4'hF;
b = 6'hA;
$display("a*b=%h", a*b); // expression size is self-determined
c = {a**b}; // expression a**b is self-determined
// due to concatenation operator {}
$display("a**b=%h", c);
c = a**b; // expression size is determined by c
$display("c=%h", c);
end
Simulator output for this example:
a*b=16 // 'h96 was truncated to 'h16 since expression size is 6
a**b=1 // expression size is 4 bits (size of a)
c=ac61 // expression size is 16 bits (size of c)
### 11.7 Signed expressions
Controlling the sign of an expression is important if consistent results are to be achieved. 11.8.1 outlines the
rules that determine if an expression is signed or unsigned.
The cast operator can be used to change either the signedness or type of an expression (see 6.24.1). In
addition to the cast operator, the $signed and $unsigned system functions are available for casting the
signedness of expressions. These functions shall evaluate the input expression and return a one-dimensional
packed array with the same number of bits and value of the input expression and the signedness defined by
the function.
$signed—returned value is signed
$unsigned—returned value is unsigned
For example:
logic [7:0] regA, regB;
logic signed [7:0] regS;
regA = $unsigned(-4); // regA = 8'b11111100
regB = $unsigned(-4'sd4); // regB = 8'b00001100
regS = $signed (4'b1100); // regS = -4
regA = unsigned'(-4); // regA = 8'b11111100
regS = signed'(4'b1100); // regS = -4

<!-- Page 303 -->

IEEE Std regS = regA + regB; // will do unsigned addition
regS = byte'(regA) + byte'(regB); // will do signed addition
regS = signed'(regA) + signed'(regB); // will do signed addition
regS = $signed(regA) + $signed(regB); // will do signed addition
### 11.8 Expression evaluation rules
#### 11.8.1 Rules for expression types
The following are the rules for determining the resulting type of an expression:
— Expression type depends only on the operands. It does not depend on the left-hand side (if any).
— Decimal numbers are signed.
— Based numbers are unsigned, except where the s notation is used in the base specifier (as in
4'sd12).
— Bit-select results are unsigned, regardless of the operands.
— Part-select results are unsigned, regardless of the operands even if the part-select specifies the entire
vector.
logic [15:0] a;
logic signed [7:0] b;
initial
a = b[7:0]; // b[7:0] is unsigned and therefore zero-extended
— Concatenation results are unsigned, regardless of the operands.
— Comparison and reduction operator results are unsigned, regardless of the operands.
— Reals converted to integers by type coercion are signed.
— The sign and size of any self-determined operand are determined by the operand itself and
independent of the remainder of the expression.
— For non-self-determined operands, the following rules apply:
• If any operand is real, the result is real.
• If any operand is unsigned, the result is unsigned, regardless of the operator.
• If all operands are signed, the result will be signed, regardless of operator, except when specified
otherwise.
#### 11.8.2 Steps for evaluating an expression
The following are the steps for evaluating an expression:
— Determine the expression size based upon the standard rules of expression size determination (see
11.6).
— Determine the sign of the expression using the rules outlined in 11.8.1.
— Propagate the type and size of the expression (or self-determined subexpression) back down to the
context-determined operands of the expression. In general, any context-determined operand of an
operator shall be the same type and size as the result of the operator. However, there are two
exceptions:
• If the result type of the operator is real and if it has a context-determined operand that is not real,
that operand shall be treated as if it were self-determined and then converted to real just before
the operator is applied.

<!-- Page 304 -->

IEEE Std • The relational and equality operators have operands that are neither fully self-determined nor
fully context-determined. The operands shall affect each other as if they were context-determined
operands with a result type and size (maximum of the two operand sizes) determined from them.
However, the actual result type shall always be 1 bit unsigned. The type and size of the operand
shall be independent of the rest of the expression and vice versa.
— When propagation reaches a simple operand as defined in 11.5, then that operand shall be converted
to the propagated type and size. If the operand shall be extended, then it shall be sign-extended only
if the propagated type is signed.
#### 11.8.3 Steps for evaluating an assignment
The following are the steps for evaluating an assignment:
— Determine the size of the right-hand side by the standard assignment size determination rules (see
11.6).
— If needed, extend the size of the right-hand side, performing sign extension if, and only if, the type
of the right-hand side is signed.
#### 11.8.4 Handling x and z in signed expressions
If a signed operand is to be resized to a larger signed width and the value of the sign bit is x, the resulting
value shall be bit-filled with x. If the sign bit of the value is z, then the resulting value shall be bit-filled with
z. If any bit of a signed value is x or z, then any nonlogical operation involving the value shall result in the
entire resultant value being an x and the type consistent with the expression’s type.
### 11.9 Tagged union expressions and member access
expression ::= // from A.8.3
...
| tagged_union_expression
tagged_union_expression ::=
tagged member_identifier [ primary ]
Syntax11-6—Tagged union syntax (excerpt from AnnexA)
A tagged union expression (packed or unpacked) is expressed using the keyword tagged followed by a
tagged union member identifier, followed by a primary representing the corresponding member value. For
void members, the member value primary is omitted.
Example:
typedef union tagged {
void Invalid;
int Valid;
} VInt;
VInt vi1, vi2;
vi1 = tagged Valid (23+34); // Create Valid int
vi2 = tagged Invalid; // Create an Invalid value

<!-- Page 305 -->

IEEE Std In the following tagged union expressions, the expressions in braces are structure assignment patterns (see
10.9.2).
typedef union tagged {
struct {
bit [4:0] reg1, reg2, regd;
} Add;
union tagged {
bit [9:0] JmpU;
struct {
bit [1:0] cc;
bit [9:0] addr;
} JmpC;
} Jmp;
} Instr;
Instr i1, i2;
// Create an Add instruction with its 3 register fields
i1 = ( e
? tagged Add '{ e1, 4, ed }; // struct members by position
: tagged Add '{ reg2:e2, regd:3, reg1:19 });// by name (order irrelevant)
// Create a Jump instruction, with "unconditional" sub-opcode
i1 = tagged Jmp (tagged JmpU 239);
// Create a Jump instruction, with "conditional" sub-opcode
i2 = tagged Jmp (tagged JmpC '{ 2, 83 }); // inner struct by position
i2 = tagged Jmp (tagged JmpC '{ cc:2, addr:83 }); // by name
The type of a tagged union expression shall be known from its context (e.g., it is used in the right-hand side
of an assignment to a variable whose type is known, or it has a cast, or it is used inside another expression
from which its type is known). The expression evaluates to a tagged union value of that type. The tagged
union expression can be completely type-checked statically; the only member names allowed after the
tagged keyword are the member names for the expression type, and the member expression shall have the
corresponding member type.
An uninitialized variable of tagged union type shall be undefined. This includes the tag bits. A variable of
tagged union type can be initialized with a tagged union expression provided the member value expression is
a legal initializer for the member type.
Members of tagged unions can be read or assigned using the usual dot notation. Such accesses are
completely type-checked, i.e., the value read or assigned shall be consistent with the current tag. In general,
this can require a run-time check. An attempt to read or assign a value whose type is inconsistent with the
tag results in a run-time error.
All of the following examples are legal only if the instruction variable i1 currently has tag Add:
x = i1.Add.reg1;
i1.Add = '{19, 4, 3};
i1.Add.reg2 = 4;
### 11.10 String literal expressions
This subclause discusses operations on string literals (see 5.9) and string literals stored in bit vectors and
other packed types. SystemVerilog also has string variables, which store strings differently than vectors. The

<!-- Page 306 -->

IEEE Std string data type has several special built-in methods for manipulating strings. See 6.16 for a discussion of
the string data type and associated methods.
String literal operands shall be treated as constant numbers consisting of a sequence of 8-bit ASCII codes,
one per character. Any SystemVerilog operator can manipulate string literal operands. The operator shall
behave as though the entire string were a single numeric value.
When a vector is larger than required to hold the string literal value being assigned, the contents after the
assignment shall be padded on the left with zeros. This is consistent with the padding that occurs during
assignment of nonstring unsigned values.
The following example declares a vector variable large enough to hold 14 characters and assigns a value to
it. The example then manipulates the stored value using the concatenation operator.
module string_test;
bit [8*14:1] stringvar;
initial begin
stringvar = "Hello world";
$display("%s is stored as %h", stringvar, stringvar);
stringvar = {stringvar,"!!!"};
$display("%s is stored as %h", stringvar, stringvar);
end
endmodule
The result of simulating the preceding description is as follows:
Hello world is stored as 00000048656c6c6f20776f726c64
Hello world!!! is stored as 48656c6c6f20776f726c64212121
#### 11.10.1 String literal operations
SystemVerilog operators support the common string operations copy, concatenate, and compare for string
literals and string literals stored in vectors. Copy is provided by simple assignment. Concatenation is
provided by the concatenation operator. Comparison is provided by the equality operators.
When manipulating string literal values in vectors, the vectors should be at least 8*n bits (where n is the
number of ASCII characters) in order to preserve the 8-bit ASCII code.
#### 11.10.2 String literal value padding and potential problems
When string literals are assigned to vectors, the values stored shall be padded on the left with zeros. Padding
can affect the results of comparison and concatenation operations. The comparison and concatenation
operators shall not distinguish between zeros resulting from padding and the original string characters (\0,
ASCII NUL).
The following example illustrates the potential problem:
bit [8*10:1] s1, s2;
initial begin
s1 = "Hello";
s2 = " world!";
if ({s1,s2} == "Hello world!")
$display("strings are equal");
end

<!-- Page 307 -->

IEEE Std The comparison in this example fails because during the assignment the variables s1 and s2 are padded as
illustrated in the next example:
s1 = 000000000048656c6c6f
s2 = 00000020776f726c6421
The concatenation of s1 and s2 includes the zero padding, resulting in the following value:
000000000048656c6c6f00000020776f726c6421
Because the string literal “Hello world!” contains no zero padding, the comparison fails, as shown in the
following example:
s1 s2
000000000048656c6c6f00000020776f726c6421
48656c6c6f20776f726c6421
"Hello" " world!"
This comparison yields a result of zero, which represents false.
#### 11.10.3 Empty string literal handling
The empty string literal ("") shall be considered equivalent to the ASCII NUL ("\0"), which has a value
zero (0), which is different from a string "0".
### 11.11 Minimum, typical, and maximum delay expressions
SystemVerilog delay expressions can be specified as three expressions separated by colons and enclosed by
parentheses. This is intended to represent minimum, typical, and maximum values—in that order. The
syntax is given in Syntax11-7.
mintypmax_expression ::= // from A.8.3
expression
| expression : expression : expression
constant_mintypmax_expression ::=
constant_expression
| constant_expression : constant_expression : constant_expression
expression ::=
primary
| unary_operator { attribute_instance } primary
| inc_or_dec_expression
| ( operator_assignment )
| expression binary_operator { attribute_instance } expression
| conditional_expression
| inside_expression
| tagged_union_expression
constant_expression ::=
constant_primary

<!-- Page 308 -->

IEEE Std | unary_operator { attribute_instance } constant_primary
| constant_expression binary_operator { attribute_instance } constant_expression
| constant_expression ? { attribute_instance } constant_expression : constant_expression
constant_primary ::= // from A.8.4
primary_literal
| ps_parameter_identifier constant_select
| specparam_identifier [ [ constant_range_expression ] ]
| genvar_identifier44
| formal_port_identifier constant_select
| [ package_scope | class_scope ] enum_identifier
| empty_unpacked_array_concatenation
| constant_concatenation [ [ constant_range_expression ] ]
| constant_multiple_concatenation [ [ constant_range_expression ] ]
| constant_function_call [ [ constant_range_expression ] ]
| constant_let_expression
| ( constant_mintypmax_expression )
| constant_cast
| constant_assignment_pattern_expression
| type_reference45
| null
primary_literal ::= number | time_literal | unbased_unsized_literal | string_literal
44) A genvar_identifier shall be legal in a constant_primary only within a loop generate construct.
45) It shall be legal to use a type_reference constant_primary as the casting_type in a static cast. It shall be illegal for a
type_reference constant_primary to be used with any operators except the equality/inequality and case equality/
inequality operators.
Syntax11-7—Syntax for min:typ:max expression (excerpt from AnnexA)
SystemVerilog models typically specify three values for delay expressions. The three values allow a design
to be tested with minimum, typical, or maximum delay values, known as a min:typ:max expression.
Values expressed in min:typ:max format can be used in expressions. The min:typ:max format can be used
wherever expressions can appear.
Example 1: This example shows an expression that defines a single triplet of delay values. The minimum
value is the sum of a+d; the typical value is b+e; the maximum value is c+f, as follows:
(a:b:c) + (d:e:f)
Example 2: The next example shows a typical expression that is used to specify min:typ:max format values:
val - (32'd 50: 32'd 75: 32'd 100)
### 11.12 Let construct
let_declaration ::= let let_identifier [ ( [ let_port_list ] ) ] = expression ; // from A.2.12
let_identifier ::= identifier
let_port_list ::= let_port_item { , let_port_item }
let_port_item ::=

<!-- Page 309 -->

IEEE Std { attribute_instance } let_formal_type formal_port_identifier { variable_dimension } [ = expression ]
let_formal_type ::=
data_type_or_implicit
| untyped
let_expression ::= [ package_scope ] let_identifier [ ( [ let_list_of_arguments ] ) ]
let_list_of_arguments ::=
[ let_actual_arg ] { , [ let_actual_arg ] } { , . identifier ( [ let_actual_arg ] ) }
| . identifier ( [ let_actual_arg ] ) { , . identifier ( [ let_actual_arg ] ) }
let_actual_arg ::= expression
Syntax11-8—Let syntax (excerpt from AnnexA)
A let declaration defines a template expression (a let body), customized by its ports. A let construct may
be instantiated in other expressions.
let declarations can be used for customization and can replace the text macros in many cases. The let
construct is safer because it has a local scope, while the scope of compiler directives is global within the
compilation unit. Including let declarations in packages (see Clause26) is a natural way to implement a
well-structured customization for the design code.
Example 1:
package pex_gen9_common_expressions;
let valid_arb(request, valid, override) = |(request & valid) || override;
...
endpackage
module my_checker;
import pex_gen9_common_expressions::*;
logic a, b;
wire [1:0] req;
wire [1:0] vld;
logic ovr;
...
if (valid_arb(.request(req), .valid(vld), .override(ovr))) begin
...
end
...
endmodule
Example 2:
let mult(x, y) = ($bits(x) + $bits(y))'(x * y);
Just as properties and sequences serve as templates for concurrent assertions (see 16.5), the let construct
can serve this purpose for immediate assertions. For example:
let at_least_two(sig, rst = 1'b0) = rst || ($countones(sig) >= 2);
logic [15:0] sig1;
logic [3:0] sig2;
always_comb begin
q1: assert (at_least_two(sig1));
q2: assert (at_least_two(~sig2));
end

<!-- Page 310 -->

IEEE Std Another intended use of let is to provide shortcuts for identifiers or subexpressions. For example:
task write_value;
input logic [31:0] addr;
input logic [31:0] value;
...
endtask
...
let addr = top.block1.unit1.base + top.block1.unit2.displ;
...
write_value(addr, 0);
The formal arguments may optionally be typed and also may have optional default values. If a formal
argument of a let is typed, then the type shall be event or one of the types allowed in 16.6. The following
rules apply to typed formal arguments and their corresponding actual arguments, including default actual
arguments declared in a let:
1) If the formal argument is of type event, then the actual argument shall be an event_expression and
each reference to the formal argument shall be in a place where an event_expression may be written.
2) Otherwise, the self-determined result type of the actual argument shall be cast compatible (see
6.22.4) with the type of the formal argument. The actual argument shall be cast to the type of the
formal argument before being substituted for a reference to the formal argument in the rewriting
algorithm (see F.4).
Variables used in a let that are not formal arguments to the let are resolved according to the scoping rules
from the scope in which the let is declared. In the scope of declaration, a let body shall be defined before
it is used. No hierarchical references to let declarations are allowed.
The let body gets expanded with the actual arguments by replacing the formal arguments with the actual
arguments. Semantic checks are performed to verify that the expanded let body with the actual arguments
is legal. The result of the substitution is enclosed in parentheses (...) so as to preserve the priority of
evaluation of the let body. Recursive let instantiations are not permitted.
A let body may contain sampled value function calls (see 16.9.3 and 16.9.4). Their clock, if not explicitly
specified, is inferred in the instantiation context in the same way as if the functions were used directly in the
instantiation context. It shall be an error if the clock is required, but cannot be inferred in the instantiation
context.
A let may be declared in any of the following:
— A module
— An interface
— A program
— A checker
— A clocking block
— A package
— A compilation-unit scope
— A generate block
— A sequential or parallel block
— A subroutine
Examples:
a) let with arguments and without arguments

<!-- Page 311 -->

IEEE Std module m;
logic clk, a, b;
logic p, q, r;
// let with formal arguments and default value on y
let eq(x, y = b) = x == y;
// without parameters, binds to a, b above
let tmp = a && b;
...
a1: assert property (@(posedge clk) eq(p,q));
always_comb begin
a2: assert (eq(r)); // use default for y
a3: assert (tmp);
end
endmodule : m
The effective code after expanding let expressions:
module m;
bit clk, a, b;
logic p, q, r;
// let eq(x, y = b) = x == y;
// let tmp = a && b;
...
a1: assert property (@(posedge clk) (m.p == m.q));
always_comb begin
a2: assert ((m.r == m.b)); // use default for y
a3: assert ((m.a && m.b));
end
endmodule : m
b) Declarative context binding of let arguments
module top;
logic x = 1'b1;
logic a, b;
let y = x;
...
always_comb begin
// y binds to preceding definition of x
// in the declarative context of let
bit x = 1'b0;
b = a | y;
end
endmodule : top
The effective code after expanding let expressions:
module top;
bit x = 1'b1;
bit a;
// let y = x;
...
always_comb begin
// y binds to preceding definition of x
// in the declarative context of let
bit x = 1'b0;
b = a | (top.x);

<!-- Page 312 -->

IEEE Std end
endmodule : top
c) Sequences (and properties) with let in structural context (see 16.8)
module top;
logic a, b;
let x = a || b;
sequence s;
x ##1 b;
endsequence : s
...
endmodule : top
The effective code after expanding let expressions:
module top;
logic a, b;
// let x = a || b;
sequence s;
(top.a || top.b) ##1 b;
endsequence : s
...
endmodule : top
d) let declared in a generate block
module m(...);
wire a, b;
wire [2:0] c;
wire [2:0] d;
wire e;
...
for (genvar i = 0; i < 3; i++) begin : L0
if (i !=1) begin : L1
let my_let(x) = !x || b && c[i];
s1: assign d[2 – i] = my_let(a)); // OK
end : L1
end : L0
s2: assign e = L0[0].L1.my_let(a)); // Illegal
endmodule : m
Statement s1 becomes two statements L0[0].L1.s1 and L0[2].L1.s1, the first of them being
assign d[2] = (!m.a || m.b && m.c[0]);
and the second one being
assign d[0] = (!m.a || m.b && m.c[2]);
Statement s2 is illegal since it references the let expression hierarchically, while hierarchical
references to let expressions are not allowed.
e) let with typed arguments
module m(input clock);
logic [15:0] a, b;
logic c, d;
typedef bit [15:0] bits;
...
let ones_match(bits x, y) = x == y;
let same(logic x, y) = x === y;

<!-- Page 313 -->

IEEE Std always_comb
a1: assert(ones_match(a, b));
property toggles(bit x, y);
same(x, y) |=> !same(x, y);
endproperty
a2: assert property (@(posedge clock) toggles(c, d));
endmodule : m
In this example the let expression ones_match checks that both arguments have bits set to 1 at the
same position. Because of the explicit specification of the formal arguments to be of the 2-state type bit
in the let declaration, all argument bits having unknown logic value or a high-impedance value become
0, and therefore the comparison captures the match of the bits set to 1. The let expression same tests for
the case equality (see 11.4.5) of its operands. When instantiated in the property toggles its actual
arguments will be of type bit. The effective code after expanding let expressions:
module m(input clock);
logic [15:0] a, b;
logic c, d;
typedef bit [15:0] bits;
...
// let ones_match(bits x, y) = x == y;
// let same(logic x, y) = x === y;
always_comb
a1:assert((bits'(a) == bits'(b)));
property toggles(bit x, y);
(logic'(x) === logic'(y)) |=> ! (logic'(x) === logic'(y));
endproperty
a2: assert property (@(posedge clock) toggles(c, d));
endmodule : m
f) Sampled value functions in let
module m(input clock);
logic a;
let p1(x) = $past(x);
let p2(x) = $past(x,,,@(posedge clock));
let s(x) = $sampled(x);
always_comb begin
a1: assert(p1(a));
a2: assert(p2(a));
a3: assert(s(a));
end
a4: assert property(@(posedge clock) p1(a));
...
endmodule : m
The effective code after expanding let expressions:
module m(input clock);
logic a;
// let p1(x) = $past(x);
// let p2(x) = $past(x,,,@(posedge clock));

<!-- Page 314 -->

IEEE Std // let s(x) = $sampled(x);
always_comb begin
a1: assert(($past(a))); // Illegal: no clock can be inferred
a2: assert(($past(a,,,@(posedge clock))));
a3: assert(($sampled (a)));
end
a4: assert property(@(posedge clock)($past(a))); // @(posedge clock)
// is inferred
...
endmodule : m

<!-- Page 315 -->

1800-2023