# Chapter 19: Functional coverage

## 19. Functional coverage
### 19.1 General
This clause describes the following:
— Defining coverage groups
— Defining coverage points
— Defining cross coverage
— Coverage options
— Coverage system tasks and system functions
— Coverage computation
### 19.2 Overview
Functional verification comprises a large portion of the resources required to design and validate a complex
system. Often, the validation needs to be comprehensive without redundant effort. To minimize wasted
effort, coverage is used as a guide for directing verification resources by identifying tested and untested
portions of the design.
Coverage is defined as the percentage of verification objectives that have been met. It is used as a metric for
evaluating the progress of a verification project in order to reduce the number of simulation cycles spent in
verifying a design.
Broadly speaking, there are two types of coverage metrics: those that can be automatically extracted from
the design code, such as code coverage, and those that are user-specified in order to tie the verification
environment to the design intent or functionality. The latter form is referred to as functional coverage and is
the topic of this clause.
Functional coverage is a user-defined metric that measures how much of the design specification, as
enumerated by features in the test plan, has been exercised. It can be used to measure whether interesting
scenarios, corner cases, specification invariants, or other applicable design conditions—captured as features
of the test plan—have been observed, validated, and tested.
The key aspects of functional coverage are as follows:
— It is user-specified and is not automatically inferred from the design.
— It is based on the design specification (i.e., its intent) and is thus independent of the actual design
code or its structure.
Because it is fully specified by the user, functional coverage requires more up-front effort (someone has to
write the coverage model). Functional coverage also requires a more structured approach to verification.
Although functional coverage can shorten the overall verification effort and yield higher quality designs, its
shortcomings can impede its adoption.
The SystemVerilog functional coverage extensions address these shortcomings by providing language
constructs for easy specification of functional coverage models. This specification can be efficiently
executed by the SystemVerilog simulation engine, thus enabling coverage data manipulation and analysis
tools that speed up the development of high-quality tests. The improved set of tests can exercise more corner
cases and required scenarios, without redundant work.
The SystemVerilog functional coverage constructs enable the following:
— Coverage of variables and expressions, as well as cross coverage between them
— Automatic as well as user-defined coverage bins

<!-- Page 577 -->

IEEE Std — Associate bins with sets of values, transitions, or cross products
— Filtering conditions at multiple levels
— Events and sequences to automatically trigger coverage sampling
— Procedural activation and query of coverage
— Optional directives to control and regulate coverage
### 19.3 Defining the coverage model: covergroup
The covergroup construct encapsulates the specification of a coverage model. Each covergroup
specification can include the following components:
— A coverage event that synchronizes the sampling of coverage points
— A set of coverage points
— Cross coverage between coverage points
— Optional formal arguments
— Coverage options
The covergroup construct is a user-defined type. The type definition is written once, and multiple
instances of that type can be created in different contexts. Similar to a class, once defined, a covergroup
instance can be created via the new() operator. A covergroup can be defined in a package, module,
program, interface, checker, or class (see Syntax19-1).
covergroup_declaration ::= // from A.2.11
covergroup covergroup_identifier [ ( [ tf_port_list ] ) ] [ coverage_event ] ;
{ coverage_spec_or_option }
endgroup [ : covergroup_identifier ]
| covergroup extends covergroup_identifier ; 29
{ coverage_spec_or_option }
endgroup [ : covergroup_identifier ]
coverage_spec_or_option ::=
{ attribute_instance } coverage_spec
| { attribute_instance } coverage_option ;
coverage_option ::=
option . member_identifier = expression
| type_option . member_identifier = constant_expression
coverage_spec ::=
cover_point
| cover_cross
coverage_event ::=
clocking_event
| with function sample ( [ tf_port_list ] )
| @@ ( block_event_expression )
block_event_expression ::=
block_event_expression or block_event_expression
| begin hierarchical_btf_identifier
| end hierarchical_btf_identifier
hierarchical_btf_identifier ::=
hierarchical_tf_identifier
| hierarchical_block_identifier
| [ hierarchical_identifier . | class_scope ] method_identifier

<!-- Page 578 -->

IEEE Std 29) The extends specification of covergroup is allowed only within a class.
Syntax19-1—Covergroup syntax (excerpt from AnnexA)
The identifier associated with the covergroup declaration defines the name of the coverage model. Using
this name, an arbitrary number of coverage model instances can be created. For example:
covergroup cg; ... endgroup
cg cg_inst = new;
The preceding example defines a covergroup named cg. An instance of cg is declared as cg_inst and
created using the new operator.
A covergroup can specify an optional list of arguments as described in 13.5. When the covergroup specifies
a list of formal arguments, its instances shall provide to the new operator all the actual arguments that are not
defaulted. Actual arguments are evaluated when the new operator is executed. A ref argument tracks the
value of the actual argument as it changes. In contrast, an input argument only has the value passed to the
new operator at the time the constructor is called.
An output or inout shall be illegal as a formal argument. Since a covergroup cannot modify any argument
to the new operator, a ref argument will be treated the same as a read-only const ref argument. The
formal arguments of a covergroup cannot be accessed using a hierarchical name (the formals cannot be
accessed outside the covergroup declaration).
The optional coverage_event defines the event at which coverage points are sampled. This coverage_event
may be a clocking event, a block event expression, or a sample() method with customized arguments (see
19.8.1). If no coverage_event is specified, the covergroup will use a default sample() method with no
arguments (see 19.8). If the covergroup uses a sample() method, coverage sampling is triggered by calling
sample() and providing any actual arguments that are not defaulted.
A clocking event may refer to arguments of the covergroup. The sampling behavior of a clocking event may
be modified with the strobe option (see 19.7.1). When the strobe option is not set (the default), a
coverage point is sampled the instant the clocking event takes place, as if the process triggering the event
were to call the built-in sample() method. If the clocking event occurs multiple times in a time step, the
coverage point will also be sampled multiple times. The strobe option can be used to specify that coverage
points are sampled in the Postponed region, thereby filtering multiple clocking events so that only one
sample per time slot is taken. The strobe option only applies to the scheduling of samples triggered by a
clocking event. It shall have no effect on sampling due to a block event expression or procedural calls to the
sample() method.
As an alternative to a clocking event or a sample method, a coverage group mayspecify a block event
expression to indicate that the coverage sample is to be triggered by the start or the end of execution of a
given named block, task, function, or class method. Block event expressions that specify the begin
keyword followed by a hierarchical identifier denoting a named block, task, function, or class method shall
be triggered immediately before the corresponding block, task, function, or method begins executing its first
statement. Block event expressions that specify the end keyword followed by a hierarchical identifier
denoting a named block, task, function, or class method shall be triggered immediately after the
corresponding block, task, function, or method executes its last statement. Block event expressions that
specify the end of execution shall not be triggered if the block, task, function, or method is disabled.
A covergroup can contain one or more coverage points. A coverage point can cover a variable or an
expression.
Each coverage point includes a set of bins associated with its sampled values or its value transitions. Bins
associated with value sets are referred to as state bins while bins associated with value transitions are

<!-- Page 579 -->

IEEE Std referred to as transition bins. The bins can be explicitly defined by the user or automatically created by the
tool. Coverage points are discussed in detail in 19.5.
enum {red, green, blue} color;
covergroup g1 @(posedge clk);
c: coverpoint color;
endgroup
The preceding example defines coverage group g1 with a single coverage point associated with variable
color. The value of the variable color is sampled at the indicated clocking event: the positive edge of
signal clk. Because the coverage point does not explicitly define any bins, the tool automatically creates
three bins, one for each possible value of the enumerated type. Automatic bins are described in 19.5.3.
A coverage group can also specify cross coverage between any combination of two or more variables or
coverage points previously declared. For example:
enum {red, green, blue} color;
bit [3:0] pixel_adr, pixel_offset, pixel_hue;
covergroup g2 @(posedge clk);
Offset: coverpoint pixel_offset;
Hue: coverpoint pixel_hue;
AxC: cross color, pixel_adr; // cross 2 variables
// (implicitly declared coverpoints)
all: cross color, Hue, Offset; // cross 1 variable and 2 coverpoints
endgroup
This example creates coverage group g2 that includes two coverage points and two cross coverage items.
Explicit coverage points labeled Offset and Hue are defined for variables pixel_offset and
pixel_hue. SystemVerilog implicitly declares coverage points for variables color and pixel_adr in
order to track their cross coverage. Implicitly declared coverage points are described in 19.6.
A coverage group can also specify one or more options to control and regulate how coverage data are
structured and collected. Coverage options can be specified for the coverage group as a whole or for specific
items within the coverage group, that is, any of its coverage points or crosses. In general, a coverage option
specified at the covergroup level applies to all of its items unless overridden by them. Coverage options are
described in 19.7.
The extends variation of covergroup declaration is allowed only for a covergroup defined within a class (see
19.4.1).
### 19.4 Using covergroups in classes
By embedding a coverage group within a class definition, the covergroup provides a simple way to cover a
subset of the class properties. This integration of coverage with classes provides an intuitive and expressive
mechanism for defining the coverage model associated with a class. For example:
In class xyz, defined as follows, members m_x and m_y are covered using an embedded covergroup:
class xyz;
bit [3:0] m_x;
int m_y;
bit m_z;

<!-- Page 580 -->

IEEE Std covergroup cov1 @m_z; // embedded covergroup
coverpoint m_x;
coverpoint m_y;
endgroup
function new(); cov1 = new; endfunction
endclass
In this example, data members m_x and m_y of class xyz are sampled on every change of data member m_z.
A covergroup declaration within a class is an embedded covergroup declaration. An embedded
covergroup declaration declares an anonymous covergroup type and an instance variable of the
anonymous type. The covergroup_identifier defines the name of the instance variable. In the preceding
example, a variable cov1 (of the anonymous coverage group) is implicitly declared.
An embedded covergroup can define a coverage model for protected and local class properties without any
changes to the class data encapsulation. A derived covergroup (see 19.4.1) in a derived class may inherit a
coverage model for local class properties from the base covergroup, but it shall not define a new coverage
model for class properties that are local to the base class. Class members can be used in coverpoint
expressions or can be used in other coverage constructs, such as conditional guards or option initialization.
A class can have more than one covergroup. The following example shows two coverage groups in class MC:
class MC;
logic [3:0] m_x;
local logic m_z;
bit clk;
bit m_e;
covergroup cv1 @(posedge clk); coverpoint m_x; endgroup
covergroup cv2 @m_e ; coverpoint m_z; endgroup
endclass
In covergroup cv1, public class member variable m_x is sampled at every positive edge of signal clk.
Local class member m_z is covered by another covergroup cv2. Each coverage group is sampled by a
different clocking event.
An embedded coverage group may be instantiated in the new() method of the covergroup’s parent class by
assigning the result of the new() operator to the covergroup variable. If this assignment is not present, then
the coverage group is not created and no data will be sampled. The covergroup variable shall not be assigned
outside the new() method of the parent class.
Following is an example of an embedded coverage group that does not have any passed-in arguments and
uses explicit instantiation to synchronize with another object:
class Helper;
int m_ev;
endclass
class MyClass;
Helper m_obj;
int m_a;
covergroup Cov @(m_obj.m_ev);
coverpoint m_a;
endgroup
function new();
m_obj = new;

<!-- Page 581 -->

IEEE Std Cov = new; // Create embedded covergroup after creating m_obj
endfunction
endclass
In this example, covergroup Cov is embedded within class MyClass, which contains an object of type
Helper class, called m_obj. The clocking event for the embedded coverage group refers to data member
m_ev of m_obj. Because the coverage group Cov uses m_obj, m_obj has to be instantiated before Cov.
Therefore, the coverage group Cov is instantiated after instantiating m_obj in the class constructor. As
shown previously, the instantiation of an embedded coverage group is done by assigning the result of the
new operator to the coverage group identifier.
The following example shows how arguments passed in to an embedded coverage group can be used to set a
coverage option of the coverage group:
class C1;
bit [7:0] x;
covergroup cv (int arg, ref bit clk) @(posedge clk);
option.at_least = arg;
coverpoint x;
endgroup
function new(int p1);
cv = new(p1);
endfunction
endclass
initial begin
C1 obj = new(4);
end
#### 19.4.1 Embedded covergroup inheritance
Embedded covergroups support a kind of single inheritance, using the declaration syntax:
covergroup extends covergroup_identifier ;
{ coverage_spec_or_option }
endgroup [: covergroup_identifier ]
When this extends declaration is used, a derived covergroup with name covergroup_identifier is defined that
may provide additions and/or overrides to the base covergroup that has the name covergroup_identifier. It
shall be an error to use the extends declaration if the covergroup_identifier has not previously been defined
in a base class of the enclosing class. A derived covergroup in one class may act as the base covergroup for
covergroup inheritance in another class.
When a derived covergroup extends a base covergroup, it replaces the instance of the base covergroup with
an instance of the derived covergroup. This means that all references to the name provided by the
covergroup_identifier for the base covergroup in this class and in all its base classes will resolve to an
instance of the derived covergroup.
A derived covergroup may refer to any of the components in its base covergroup (see 19.3). Unless
overridden as described below, all components belonging to the base covergroup are considered to also
belong to the derived covergroup.

<!-- Page 582 -->

IEEE Std If the base covergroup has a list of arguments specified, the derived covergroup implicitly has the same list
of arguments. If the base covergroup has a coverage event specified, the derived covergroup shall use that
coverage event.
If a derived covergroup has a coverage point with a new name not found in the base covergroup, it will be
considered an additional coverage point to be sampled. If a derived covergroup has a coverage point with a
name that is identical to the name of a coverage point found in its base covergroup, then that coverage point
in its base covergroup will not contribute to the coverage computation (see 19.11). Even if a coverage point
in the base covergroup does not contribute to the computation, a cross (see 19.6) in the base covergroup that
includes that base coverpoint still contributes to the computation unless there is a cross with the same name
in the derived covergroup.
If a derived covergroup specifies a coverage option that is also specified in its base covergroup, the option in
the base covergroup is overridden. If there are coverage options in the base covergroup that are not
overridden, they will apply to the derived covergroup.
For the purposes of get_coverage(), a derived covergroup and its base covergroup are separate types. If
there are instances of the base class with the base covergroup and instances of a derived class with a derived
covergroup, no aggregation occurs across the base and derived covergroups.
The following example shows a derived class that has a derived covergroup:
class base;
enum {red, green, blue} color;
covergroup g1 (bit [3:0] a) with function sample(bit b);
option.weight = 10;
option.per_instance = 1;
coverpoint a;
coverpoint b;
c: coverpoint color;
endgroup
function new();
g1 = new;
endfunction
endclass
class derived extends base;
bit d;
covergroup extends g1;
option.weight = 1; // overrides the weight from base g1
// uses per_instance=1 from base g1
c: coverpoint color // overrides the c coverpoint in base g1
{
ignore_bins ignore = {blue};
}
coverpoint d; // adds new coverpoint
cross a, d; // crosses new coverpoint with inherited one
endgroup :g1
function new();
super.new();
endfunction
endclass

<!-- Page 583 -->

IEEE Std ### 19.5 Defining coverage points
A covergroup can contain one or more coverage points. A coverage point specifies an integral or real
expression that is to be covered. Each coverage point includes a set of bins associated with the sampled
values or value transitions of the covered expression. Coverage of real expressions is limited to sampled
value bins (and their crosses). The bins can be explicitly defined by the user or automatically created by
SystemVerilog. The syntax for specifying coverage points is given in Syntax19-2. Evaluation of the
coverage point expression (and of its enabling iff condition, if any) takes place when the covergroup is
sampled. The expression shall be evaluated in a procedural context, and therefore it shall be legal for the
expression to make access through a virtual interface (see 25.9).
cover_point ::= // from A.2.11
[ [ data_type_or_implicit ] cover_point_identifier : ] coverpoint expression [ iff ( expression ) ]
bins_or_empty
bins_or_empty ::=
{ { attribute_instance } { bins_or_options ; } }
| ;
bins_or_options ::=
coverage_option
| [ wildcard ] bins_keyword bin_identifier [ [ [ covergroup_expression ] ] ] =
{ covergroup_range_list } [ with ( with_covergroup_expression ) ]
[ iff ( expression ) ]
| [ wildcard ] bins_keyword bin_identifier [ [ [ covergroup_expression ] ] ] =
cover_point_identifier with ( with_covergroup_expression ) [ iff ( expression ) ]
| [ wildcard ] bins_keyword bin_identifier [ [ [ covergroup_expression ] ] ] =
set_covergroup_expression [ iff ( expression ) ]
| [ wildcard] bins_keyword bin_identifier [ [ ] ] = trans_list [ iff ( expression ) ]
| bins_keyword bin_identifier [ [ [ covergroup_expression ] ] ] = default [ iff ( expression ) ]
| bins_keyword bin_identifier = default sequence [ iff ( expression ) ]
bins_keyword::= bins | illegal_bins | ignore_bins
covergroup_range_list ::= covergroup_value_range { , covergroup_value_range }
covergroup_value_range ::=
covergroup_expression
| [ covergroup_expression : covergroup_expression ]
| [ $ : covergroup_expression ]
| [ covergroup_expression : $ ]
| [ covergroup_expression +/- covergroup_expression ]
| [ covergroup_expression +%- covergroup_expression ]
with_covergroup_expression ::= covergroup_expression31
set_covergroup_expression ::= covergroup_expression32
covergroup_expression ::= expression33
31) The result of this expression shall be assignment compatible with an integral type as described in 19.5.1.1.
32) This expression is restricted as described in 19.5.1.2.
33) This expression is restricted as described in 19.5.
Syntax19-2—Coverage point syntax (excerpt from AnnexA)

<!-- Page 584 -->

IEEE Std A coverpoint coverage point creates a hierarchical scope and can be optionally labeled. If the label is
specified, then it designates the name of the coverage point. This name can be used to add this coverage
point to a cross coverage specification or to access the methods of the coverage point. If the label is omitted
and the coverage point is associated with a single variable, then the variable name becomes the name of the
coverage point. Otherwise, an implementation can generate a name for the coverage point only for the
purposes of coverage reporting; that is, generated names cannot be used within the language.
A data type for the coverpoint may be specified explicitly or implicitly in data_type_or_implicit. In either
case, it shall be understood that a data type is specified for the coverpoint. The data type shall be an integral
or real type. If a data type is specified, then a cover_point_identifier shall also be specified.
If a data type is specified, then the coverpoint expression shall be assignment compatible with the data type.
Values for the coverpoint shall be of the specified data type and shall be determined as though the
coverpoint expression were assigned to a variable of the specified data type.
If no data type is specified, then the inferred data type for the coverpoint shall be the self-determined type of
the coverpoint expression.
A coverpoint name has limited visibility. An identifier can only refer to a coverpoint in the following
contexts:
— In the coverpoint list of a cross declaration (see 19.6),
— In a hierarchical name where the prefix specifies the name of a covergroup variable. For example,
cov1.cp.option.weight where cov1 is the name of a covergroup variable and cp is the name of
a coverpoint declared within the covergroup.
— Following ::, where the left operand of the scope resolution operator refers to a covergroup. For
example, covtype :: cp :: type_option.weight.
Only constant expressions (see 11.2.1), global and instance constants (for an embedded covergroup, see 19.4
and 8.19), or non-ref arguments to the covergroup are allowed to be used as variables in a
covergroup_expression.
Global and instance constants referenced from a covergroup_expression shall be members of the enclosing
class. The initializers for such instance constants shall appear before the referring covergroup constructor
call in the class constructor. These initializers shall not appear with the covergroup constructor call in the
body of any looping statement (see 12.7) or fork-join_none, either before or after.
Function calls may participate in a covergroup_expression, but the following semantic restrictions are
imposed:
— Functions shall not contain output, inout, or ref arguments (const ref is allowed).
— Functions shall be automatic (or preserve no state information) and have no side effects.
— Functions shall not reference non-constant variables outside the local scope of the function.
— System function calls are restricted to constant system function calls (see 11.2.1).
For example:
covergroup cg (ref int x , ref int y, input int c);
coverpoint x; // creates coverpoint "x" covering the formal "x"
x: coverpoint y; // INVALID: coverpoint label "x" already exists
b: coverpoint y; // creates coverpoint "b" covering the formal "y"
cx: coverpoint x; // creates coverpoint "cx" covering the formal "x"
option.weight = c; // set weight of "cg" to value of formal "c"

<!-- Page 585 -->

IEEE Std bit [7:0] d: coverpoint y[31:24]; // creates coverpoint "d" covering the
// high order 8 bits of the formal "y"
e: coverpoint x {
option.weight = 2; // set the weight of coverpoint "e"
}
e.option.weight = 2; // INVALID use of "e", also syntax error
cross x, y { // Creates implicit coverpoint "y" covering
// the formal "y". Then creates a cross of
// coverpoints "x", "y"
option.weight = c; // set weight of cross to value of formal "c"
}
b: cross y, x; // INVALID: coverpoint label "b" already exists
endgroup
A coverage point can sample the values that correspond to a particular scheduling region (see Clause4) by
specifying a clocking block signal. Thus, a coverage point that denotes a clocking block signal will sample
the values made available by the clocking block. If the clocking block specifies a skew of #1step, the
coverage point will sample the signal values from the Preponed region. If the clocking block specifies a
skew of #0, the coverage point will sample the signal values from the Observed region.
The expression within the iff construct specifies an optional condition that disables coverage for that
coverpoint. If the guard expression evaluates to false at a sampling point, the coverage point is ignored.
For example:
covergroup g4;
coverpoint s0 iff(!reset);
endgroup
In the preceding example, coverage point s0 is covered only if the value reset is false.
A coverage point bin associates a name and a count with a set of values or a sequence of value transitions. If
the bin designates a set of values, the count is incremented every time the coverage point matches one of the
values in the set. If the bin designates a sequence of value transitions, the count is incremented every time
the coverage point matches the entire sequence of value transitions.
The bins for an integral type coverage point can be automatically created by SystemVerilog or explicitly
defined using the bins construct to name each bin. If the bins are not explicitly defined, they are
automatically created. The number of automatically created bins can be controlled using the auto_bin_max
coverage option. Coverage options are described in 19.7. Bins shall not be automatically created for
coverpoints of real expressions; therefore, coverpoints of real expressions shall specify at least one explicit
bins construct.
The default specification defines a bin that catches the values of the coverage point that do not lie within
any of the defined bins. The default is useful for catching unplanned or invalid values. The default
sequence form can be used to catch all transitions (or sequences) that do not lie within any of the defined
transition bins (see 19.5.2). The coverage calculation for a coverage point shall not take into account the
coverage captured by default bins. Default bins are also excluded from cross coverage (see 19.6). A default
bin cannot be explicitly ignored (see 19.5.5).
#### 19.5.1 Specifying bins for values
The bins construct associates a single named bin or a named array of bins with a set of values of a
coverpoint, specified by a covergroup_range_list. For a coverpoint of an integral expression, the bins

<!-- Page 586 -->

IEEE Std construct allows creating a single bin for all the values in the covergroup_range_list, a separate bin for each
value, or a fixed number of bins.
To create a single bin for the entire set of values, the bins keyword is followed by the bin name without
square brackets. To create a separate bin for each value (an open array of bins), empty square brackets, [],
shall follow the bin name. To create a fixed number of bins for a set of values, a single positive integral
expression is specified inside the square brackets. The bin name and optional square brackets are followed
by a covergroup_range_list that specifies the set of values associated with the bin. See 11.4.13 for an
explanation of range list syntax. It shall be legal to use the $ primary in a covergroup_value_range, as
shown in Syntax19-2.
If a fixed number of bins is specified and that number is smaller than the specified number of values, then
the bin values are uniformly distributed among the specified bins, as described here. Let B represent the
number of values assigned to each bin. B is defined as the total number of values divided by the number of
bins, rounded down, but not less than 1. The first B specified values are assigned to the first bin, the next B
specified values are assigned to the next bin, etc. If the number of values is not evenly divisible by the
number of bins, then the last bin will include the remaining values. Duplicate values are retained; thus the
same value can be assigned to multiple bins. For example:
bins fixed [4] = {[1:10], 1, 4, 7};
In the above example, number of values = 13 and number of bins = 4; therefore B = 13/4 = 3. The 13 values
are distributed as follows: <1,2,3>, <4,5,6>, <7,8,9>, <10,1,4,7>.
If the number of bins exceeds the number of values, then some of the bins will be empty. For example:
bins fixed [5] = {1, 4, 7};
In the above example, number of values = 3 and number of bins = 5; therefore B = 1. The 3 values are
distributed as follows: <1>, <4>, <7>, < >, < >. Note the empty fourth and fifth bins.
For integral state bins declared as “binname[],” bin names are “binname[value].” For state bins declared as
“binname[N],” bin names range from “binname[0]” through “binname[N-1].”
The following code example shows various bin groupings.
bit [9:0] v_a;
covergroup cg @(posedge clk);
coverpoint v_a
{
bins a = { [0:63],65 };
bins b[] = { [127:150],[148:191] }; // note overlapping values
bins c[] = { 200,201,202 };
bins d = { [1000:$] };
bins others[] = default;
}
endgroup
In this example, the first bins construct associates bin a with the values of variable v_a between 0 and 63
and the value 65. The second bins construct creates a set of 65 bins b[127] . . . b[191]. Note that when
empty square brackets are specified, each value is assigned one bin, including values that are specified more
than once. The third bins construct creates 3 bins: c[200], c[201], and c[202]. The fourth bins
construct associates bin d with the values between 1000 and 1023 ($ represents the maximum value of v_a).
Every value that does not match bins a, b[], c[], or d is added into its own distinct bin in the others[] bin
array.

<!-- Page 587 -->

IEEE Std Before discussing coverpoints of real expressions, it is useful to discuss how a range
[covergroup_expression : covergroup_expression] of integral values differs from a range of real values. A
range of integral values defines a discrete set of values, e.g., [1:4] is the set of values, 1,2,3,4. For a
range of real values, [1.0:4.0] defines all values within the specified range, i.e. all values that are greater
than or equal to 1.0 and less than or equal to 4.0. A coverpoint bin may partition a range of real values into
intervals, as described below.
For a coverpoint of a real expression, the bins construct allows creating a single bin for all the values and
ranges in the covergroup_range_list, a separate bin for each individually-specified value and for each
interval of each specified range, or a fixed number of bins.
If a real range is wider than the size of a single interval (which is defined by
type_option.real_interval, see 19.7.1), then the range will be divided into interval-size partitions.
The last partition may be smaller than the interval size if the range size is not evenly divisible by the interval
size. Each interval will be inclusive of its low value and exclusive of its high value, except for the last
interval, which will be inclusive of its high value also.
To create a single bin, the bins construct is followed by the bin name without square brackets. To create a
separate bin for each individually-specified value and for each range interval (an open array of bins), empty
square brackets, [], shall follow the bin name. To create a fixed number of bins, a single positive integral
expression is specified inside the square brackets. The bin name and optional square brackets are followed
by a covergroup_range_list that specifies the set of real values and ranges associated with the bin. See
#### 11.4.13 for an explanation of range list syntax.
Given an open array of bins (e.g., bins a1[]), each bin will cover any value that falls within its interval.
The first interval for a real range starts at the minimum value of the covergroup_value_range. Each
subsequent real interval starts at the ending value of the preceding real interval. The last real interval ends at
the maximum value of the covergroup_value_range. If the size of a range is smaller than the interval size,
only one bin will be used.
For real open bin arrays declared as “binname[]”, the bin names of intervals shall use “[” and “]” to denote
inclusivity and “)” to denote exclusivity of the end points: “binname[low_value:high_value)” or
“binname[low_value:high_value]”. A bin for an individual real value shall be named “binname[value]”.
For example, if a real number coverpoint open bin array is defined as bins a2[] = {[1.0:3.0], 7.5,
[8.5+/-0.1]}; using the default real interval size of 1.0, the resultant bins will be named a2[1.0:2.0),
a2[2.0:3.0], a2[7.5], and a2[8.4:8.6].
If an open bin array has multiple real ranges, identical intervals from the multiple ranges will be merged.
For example (given a real interval of 1.0):
bins a3[] = {[1.0:4.0], [2.0:5.0], [1.1:3.1], 3.3, [1.5+/-0.5]};
In this example, the first range has intervals of [1.0:2.0), [2.0:3.0), and [3.0:4.0]. The second
range has intervals of [2.0:3.0), [3.0:4.0), and [4.0:5.0]. The third range has intervals of
[1.1:2.1)and [2.1:3.1]. Finally, a there is a single value and a value with a tolerance range. The
identical intervals [2.0:3.0) will be merged, and the final set of bins will be: a3[1.0:2.0),
a3[2.0:3.0), a3[3.0:4.0], a3[3.0:4.0), a3[4.0:5.0], a3[1.1:2.1), a3[2.1:3.1], a3[3.3],
and a3[1.0:2.0].
For a fixed number of bins, declared as “binname[N]”, bin names range from “binname[0]” through
“binname[N-1].” The number of items to be distributed is the sum of the number of intervals of the specified
ranges and the number of individual values. If the number of bins is smaller than the specified number of
items, then the items are uniformly distributed among the specified bins, as described here. Let B represent
the number of items assigned to each bin. B is defined as the total number of items divided by the number of
bins, rounded down, but not less than 1. The first B specified items are assigned to the first bin, the next B

<!-- Page 588 -->

IEEE Std specified items are assigned to the next bin, etc. If the number of items is not evenly divisible by the number
of bins, then the last bin will include the remaining items. Duplicate items are retained; thus, the same value
or interval can be assigned to multiple bins. Consider the following code, using the default real interval of
1.0:
bins fixed[4] = {[1.0:4.0], [2.0:5.0], 1.0, 4.0, 7.0};
This example has six intervals: [1.0:2.0), [2.0:3.0), [3.0:4.0], [2.0:3.0), [3.0:4.0),
[4.0:5.0], and three values. number of items = 9, number of bins = 4; therefore B = 9/4 = 2. The items are
distributed as follows: <[1.0:2.0),[2.0:3.0)>, <[3.0:4.0],[2.0:3.0)>, <[3.0:4.0),
[4.0:5.0]>, <1.0,4.0,7.0>.
If the number of bins exceeds the number of items, then some of the bins will be empty. For example:
bins fixed[5] = {[1.0:3.0], 7.0};
In the above example, number of items = 3 and number of bins = 5; therefore B = 1. The 3 items are
distributed as follows: <[1.0:2.0)>, <[2.0:3.0]>, <7.0>, < >, < >. Note the empty fourth and fifth
bins.
It shall be legal to use the $ primary in a covergroup_value_range as shown in Syntax19-2. However, real
value ranges using the $ primary shall be assigned to a single bin and shall not be divided into intervals. For
example, bins a[]={[$:0.75]}; will be a single bin. The statement bins a[]={3.0, [0.75:$]};
will create two bins. And finally, bins a[4]={3.0, [0.75:$]}; will create four bins, one for 3.0, one
for the range, and two empty bins.
A default bin for real expressions shall not have an array of bins (i.e., the [] or [N] notation is not
allowed).
A single-value real bin can fail to cover numbers very close to the bin value, because a real number
representation has limited precision (see IEEE Std 754). For example, 0.1 + 0.2 = 0.30000000000000004,
and so may fail to be covered by a bin covering {0.3}. This is problematic for use with analog
specifications that include tolerance ranges. The +/- and +%- tokens (see 11.4.13) may be specified to
designate a tolerance around explicit single real values to designate an implicit range. See bins highz and
x_st in the example below. A single real value with a tolerance specification shall define a range and
therefore the rules for ranges and intervals apply. As with all ranges, a tolerance range that is wider than the
size of the real interval shall be divided into multiple bins.
The following example shows various binnings of real expressions:
real a, b;
parameter real ZSTATE = -100.0; // some real number to represent HiZ state
parameter int XSTATE = 100; // some integer number to represent X state
covergroup cg_real_value;
//default for type_option.real_interval is 1.0
cvp_a: coverpoint a {
bins highz = {[ZSTATE+/-0.1]}; // covers values in range -100.1..-99.9
bins x_st = {[XSTATE+%-1.0]}; // covers values in range 99.0..101.0
bins a1[] = {[0.75:0.85]}; // 1 bin:
// "a1[0.75:0.85]" covers 0.75..0.85
bins a2[3] = {[0.75:0.85]}; // 3 bins:
// "a2[0]" covers 0.75..0.85
// "a2[1]" and "a2[2]" will be empty
bins a3[] = {[1.0:5.0]}; // 4 bins:
// "a3[1.0:2.0)" - 1.0 to less than 2.0

<!-- Page 589 -->

IEEE Std // "a3[2.0:3.0)" - 2.0 to less than 3.0
// "a3[3.0:4.0)" - 3.0 to less than 4.0
// "a3[4.0:5.0]" - 4.0 to 5.0 inclusive
bins a4[] = {[1.0:2.0]}; // 1 bin:
// "a4[1.0:2.0]" covers 1.0 to 2.0
bins a5 = {[0.1:$]}; // 1 bin - covers all values >= 0.1
bins a6 = default; // 1 bin - covers all values < 0.1
// except -100.0 plus/minus 0.1
}
cvp_b: coverpoint b {
type_option.real_interval = 0.01; // override default real_interval size
bins xstate = {XSTATE}; // XSTATE value 100 will be cast to real
// (exact match)
bins b1[] = {[0.75:0.85]}; // 10 bins:
// "b1[0.75:0.76)" - 0.75 to less than 0.76
// "b1[0.76:0.77)" - 0.76 to less than 0.77
// . . .
// "b1[0.84:0.85]" - 0.84 to 0.85 inclusive
bins b2[] = {[0.75:0.85], [0.90:0.92]}; // 12 bins covering each
// range interval separately
bins b3[] = {[0.75:0.80], 0.902}; // 6 bins: 5 for range intervals
// + 1 for the individual value
bins b4[] = {[0.75:0.80], 0.752}; // 6 bins: 5 for range intervals
// + 1 for the individual value
bins b5[] = {[0.75:0.85], [0.753+/-0.01], 0.902};
// 10 bins for range intervals of [0.75:0.85]
// 2 bins for tolerance range intervals:
// "b5[0.743:0.753)" covers lower interval
// "b5[0.753:0.763]" covers upper interval
// 1 bin for the single real number:
// "b5[0.902]"
}
ab_cross: cross cvp_a, cvp_b {
bins az_and_bx = binsof(cvp_a.highz) && binsof(cvp_b.xstate);
ignore_bins others = !binsof(cvp_a.highz) || !binsof(cvp_b.xstate);
}
endgroup
Features of the bins in this example are as follows (for these notes, “interval size” refers to the value of
type_option.real_interval for the coverpoint):
— Bin highz covers an absolute ±0.1 tolerance range around the real value ZSTATE inclusively.
— In x_st, the integer is cast to real. The bin covers a range of a real value with a relative tolerance of
±1.0% inclusively.
— In a1, the range is smaller than the interval size, so only one bin will be created and will cover the
specified range inclusively.
— In a2, the range is smaller than the interval size, so the first bin will cover the range, and the remain-
ing bins will be empty.
— In a3, the range is greater than the interval size. Four bins will be created, one for each interval.
— In a4, the range size equals the interval size. Only one bin will be created, covering the whole range
inclusively.
— a5 is a single bin covering all values equal to or greater than 0.1.

<!-- Page 590 -->

IEEE Std — a6 is a single default bin forall values not specified in other bins.
— Bin xstate shows an integer that is cast to real and covers a single value of 100.0.
— In b1, the range is greater than the interval, so 10 bins are created.
— In b2, bins are created for each range separately.The gap between the ranges is not covered.
— In b3, there are five bins for the range and one separate bin for the real value outside of the range.
— In b4, there are five bins for the range and one separate bin for the real value inside of the range.
— b5 will have 13 bins: ten bins for the range intervals of [0.75:0.85], two bins for the tolerance
range intervals of [0.753+/-0.01], and one bin for the single real number 0.902.
— ab_cross is crossing bins of real values.
An iff construct at the end of a bin definition provides a per-bin guard condition. If the expression within
the iff construct is false at a sampling point, the count for the bin is not incremented.
Generic coverage groups are written by passing arguments to the constructor. For example:
covergroup cg (ref int ra, input int low, int high) @(posedge clk);
coverpoint ra // sample variable passed by reference
{
bins good = {[low:high]};
bins bad[] = default;
}
endgroup
...
int va, vb;
cg c1 = new(va, 0, 50); // cover variable va in the range 0 to 50
cg c2 = new(vb, 120, 600); // cover variable vb in the range 120 to 600
This example defines a coverage group, cg, in which the signal to be sampled and the extent of the coverage
bins are specified as arguments. Later, two instances of the coverage group are created; each instance
samples a different signal and covers a different range of values.
19.5.1.1 Coverpoint bin with covergroup expressions
The with clause specifies that only those values in the covergroup_range_list that satisfy the given
expression (i.e., for which the expression evaluates to true, as described in 12.4) are included in the bin. In
the expression, the name item shall be used to represent the candidate value. The candidate value is of the
same type as the coverpoint. with expressions are not allowed for bins of a real type.
The name of the coverpoint itself may be used in place of the covergroup_range_list to denote all values of
the coverpoint. Only the name of the coverpoint containing the bin being defined shall be allowed; no other
coverpoint names shall be permitted.
Consider the following example:
a: coverpoint x
{
bins mod3[] = {[0:255]} with (item % 3 == 0);
}
This bin definition selects all values from 0 to 255 that are evenly divisible by 3.
coverpoint b

<!-- Page 591 -->

IEEE Std {
bins func[] = b with (myfunc(item));
}
Note the use of the coverpoint name b to denote that the with_covergroup_expression will be applied to all
values of the coverpoint.
As with array manipulation methods involving with (see 7.12), if the expression has side effects, the results
are unpredictable.
The with clause behaves as if the expression were evaluated for every value in the covergroup_range_list at
the time the covergroup instance is constructed. By default, the with_covergroup_expression is applied to
the set of values in the covergroup_range_list prior to distribution of values to the bins. If the distribution of
values is desired before with_covergroup_expression application, the distribute_first covergroup
option (see 19.7.1) can be used to achieve this ordering. The result of applying a
with_covergroup_expression shall preserve multiple, equivalent bin items as well as bin order. The intent of
these rules is to allow the use of non-simulation analysis techniques to calculate the bin (for example, formal
symbolic analysis) or for caching of previously calculated results.
19.5.1.2 Coverpoint bin set covergroup expressions
The set_covergroup_expression syntax allows specifying an expression yielding an array of values that
define the bin. Any array whose element type is assignment compatible with the coverpoint type is
permitted, with the exception that associative arrays are not permitted. Identifiers declared within the
covergroup (such as coverpoint identifiers and bin identifiers) are not visible. The expression is evaluated
when the covergroup instance is constructed.
#### 19.5.2 Specifying bins for transitions
The syntax for specifying transition bins (Syntax19-3) accepts a subset of the sequence syntax described in
16.9:
bins_or_options ::= // from A.2.11
coverage_option
| [ wildcard ] bins_keyword bin_identifier [ [ [ covergroup_expression ] ] ] =
{ covergroup_range_list } [ with ( with_covergroup_expression ) ]
[ iff ( expression ) ]
| [ wildcard ] bins_keyword bin_identifier [ [ [ covergroup_expression ] ] ] =
cover_point_identifier with ( with_covergroup_expression ) [ iff ( expression ) ]
| [ wildcard ] bins_keyword bin_identifier [ [ [ covergroup_expression ] ] ] =
set_covergroup_expression [ iff ( expression ) ]
| [ wildcard] bins_keyword bin_identifier [ [ ] ] = trans_list [ iff ( expression ) ]
...
bins_keyword::= bins | illegal_bins | ignore_bins
covergroup_range_list ::= covergroup_value_range { , covergroup_value_range }
trans_list ::= ( trans_set ) { , ( trans_set ) }
trans_set ::= trans_range_list { => trans_range_list }
trans_range_list ::=
trans_item
| trans_item [* repeat_range ]
| trans_item [–> repeat_range ]
| trans_item [= repeat_range ]

<!-- Page 592 -->

IEEE Std trans_item ::= covergroup_range_list
covergroup_value_range ::=
covergroup_expression
| [ covergroup_expression : covergroup_expression ]
| [ $ : covergroup_expression ]
| [ covergroup_expression : $ ]
| [ covergroup_expression +/- covergroup_expression ]
| [ covergroup_expression +%- covergroup_expression ]
repeat_range ::=
covergroup_expression
| covergroup_expression : covergroup_expression
Syntax19-3—Transition bin syntax (excerpt from AnnexA)
A trans_list specifies one or more sets of ordered integral value transitions of the coverage point. Transition
bins of real values are not allowed. A single integral value transition is thus specified as follows:
value1 => value2
This represents the value of coverage point at two successive sample points, that is, value1 followed by
value2 at the next sample point.
A sequence of transitions is represented as follows:
value1 => value3 => value4 => value5
In this case, value1 is followed by value3, followed by value4, and followed by value5. A sequence
can be of any arbitrary length.
A set of transitions can be specified as follows:
range_list1 => range_list2
This specification expands to transitions between each value in range_list1 and each value in
range_list2. For example:
1,5 => 6, 7
specifies the following four transitions:
(1=>6), (1=>7), (5=>6), (5=>7)
Consecutive repetitions of transitions are specified using:
trans_item [* repeat_range ]
Here, trans_item is repeated for repeat_range times. For example:
3 [* 5]
is the same as
3=>3=>3=>3=>3
An example of a range of repetitions is as follows:

<!-- Page 593 -->

IEEE Std 3 [* 3:5]
which is the same as
(3=>3=>3), (3=>3=>3=>3), (3=>3=>3=>3=>3)
The goto repetition and the nonconsecutive repetition are similar to those in assertions (see 16.9.2).
The goto repetition is specified using: trans_item [-> repeat_range]. The required number of occurrences
of a particular value is specified by the repeat_range. Any number of sample points,including none, can
occur before the first occurrence of the specified value and any number of additional sample
points,including none, can occur between occurrences of the specified value. The transition following the
goto repetition shall immediately follow the last occurrence of the repetition. For example:
3 [-> 3]
is the same as
...=>3...=>3...=>3
where the dots (...) represent any transition that does not contain the value 3.
A goto repetition followed by an additional value is represented as follows:
1 => 3 [-> 3] => 5
is the same as
1...=>3...=>3...=>3 =>5
The nonconsecutive repetition is specified using: trans_item [= repeat_range]. The nonconsecutive
repetition is like the goto repetition except that the transition following the nonconsecutive repetition may
occur after any number of sample points so long as the repetition value does not occur again.
For example:
3 [= 2]
is the same as
...=>3...=>3...
A nonconsecutive repetition followed by an additional value is represented as follows:
1 => 3 [=2] => 6
is the same as
1...=>3...=>3...=>6
A trans_list specifies one or more sets of ordered value transitions of the coverage point. If the sequence of
value transitions of the coverage point matches any complete sequence in the trans_list, the coverage count
of the corresponding bin is incremented. For transition bins declared as “binname[],” bin names are
“binname[transition]” for some bounded transition. For example:
bit [4:1] v_a;
covergroup cg @(posedge clk);

<!-- Page 594 -->

IEEE Std coverpoint v_a
{
bins sa = (4 => 5 => 6), ([7:9],10 => 11,12);
bins sb[] = (4 => 5 => 6), ([7:9],10 => 11,12);
bins sc = (12 => 3 [-> 1]);
bins allother = default sequence ;
}
endgroup
The preceding example defines three transition coverage bins. The first bins construct associates the
following sequences with bin sa: 4=>5=>6, or 7=>11, 8=>11, 9=>11, 10=>11, 7=>12, 8=>12, 9=>12,
10=>12. The second bins construct associates an individual bin with each of the above sequences:
sb[4=>5=>6], ..., sb[10=>12]. The third bins construct associates the unbounded sequence
12=>...=>3 with bin sc. The bin allother is incremented when none of the coverpoint’s other
nondefault sequence transition bins increments, and none of the coverpoint’s previously pending transition
bins remains pending. For example, consider the following sequence of sampled values:
4 5 7 11 8 12 2 2 3
The bin allother increments twice. The bin allother increments on the sample of 7 because 5=>7
causes the matching of the pending sequence 4=>5=>6 to fail for bins sa and sb[4=>5=>6], and there were
no other previously pending sequences or incremented bins. The bin allother increments on the sample of
8 since no other bin increments on the transition 11=>8 and no other sequences were previously pending.
The bin allother does not increment during the transitions 12=>2=>2 because the bin sc remains pending
throughout.
Transitions that specify sequences of unbounded or undetermined varying length cannot be used with the
multiple bins construct (the [] notation). For example, the length of the transition 3[=2], which uses
nonconsecutive repetition, is unbounded and can vary during simulation. An attempt to specify multiple bins
with such sequences shall result in an error. A default sequence specification shall not accept multiple
transition bins (i.e., the [] notation is not allowed).
A transition bin is incremented every time the sequence of value transitions of its corresponding coverage
point matches a complete sequence, even when the sequences overlap. For example, given the definition
covergroup sg @(posedge clk);
coverpoint v
{
bins b2 = (2 [-> 3:5]); // 3 to 5 nonconsecutive 2's
bins b3 = (3 [-> 3:5]); // 3 to 5 nonconsecutive 3's
bins b5 = (5 [* 3]); // 3 consecutive 5's
bins b6 = (1 => 3 [-> 4:6] => 1); // 1 followed by
// 4 to 6 goto nonconsecutive 3's
// followed immediately by a 1
bins b7 = (1 => 2 [= 3:6] => 5); // 1 followed by
// 3 to 6 nonconsecutive 2's
// followed sometime later by a 5
}
endgroup
and the sequence of sampled values for coverpoint variable v
1st Sample
| 5th 10th 15th
| | | |
1 4 3 2 3 3 2 2 3 2 3 1 5 5 5 5 5 5

<!-- Page 595 -->

IEEE Std the preceding sequence causes transition bin b2 to be incremented on the 8th sample (three nonconsecutive
2’s), and transition bin b3 to be incremented on the 6th sample (three nonconsecutive 3’s). Likewise,
transition bin b2 is incremented on the 10th sample, and transition bin b3 is incremented on the 9th and 11th
samples. Transition bin b5 is incremented on the 15th, 16th, 17th, and 18th samples. Transition bin b6 is
incremented on the 12th sample. Transition bin b7 is incremented on the 13th sample.
A transition bin is incremented at most once per sample. In the preceding example, on the 10th sample, the
transition bin b2 is incremented only once (1 is added to the bin count).
Transition bin specifications of length 0 shall be illegal. These are transition bin specifications containing a
trans_set production of a single covergroup_value_range, e.g., (0) or ([0:1]), or a single
covergroup_value_range with a repeat_range evaluating to 1, e.g., (0[*1]) or ([0:1][*1]).
#### 19.5.3 Automatic bin creation for coverage points of integral expressions
If a coverage point of an integral expression does not define any bins except ignored or illegal bins (see
#### 19.5.5 and 19.5.6), SystemVerilog automatically creates state bins. Bins shall not be automatically created
for a coverpoint of real expressions.
When the automatic bin creation mechanism is used, SystemVerilog creates N bins to collect the sampled
values of a coverage point. The value N is determined as follows:
— For an enum coverage point, N is the cardinality of the enumeration.
— For any other integral coverage point, N is the minimum of 2M and the value of the auto_bin_max
option, where M is the number of bits needed to represent the coverage point.
If the number of automatic bins is smaller than the number of possible values (N < 2M), then the 2M values
are uniformly distributed in the N bins. If the number of values, 2M, is not divisible by N, then the last bin
will include the additional remaining items. For example, if M is 3 and N is 3, then the eight possible values
are distributed as follows: <0,1>, <2,3>, <4,5,6,7>.
Automatically created bins only consider 2-state values; sampled values containing x or z are excluded.
SystemVerilog implementations can impose a limit on the number of automatic bins. See 19.7 for the default
value of auto_bin_max.
Each automatically created bin will have a name of the form of auto[value] where value is either a
single coverage point value or the range of coverage point values included in the bin, in the form low:high.
For enumerated types, value is the named constant associated with a particular enumerated value.
See 19.11.1 for more details on automatically created bins in the presence of ignored or illegal bins.
#### 19.5.4 Wildcard specification of coverage point bins
By default, an integral value or transition bin definition can specify 4-state values. When a bin definition of
an integral type includes an x or z, it indicates that the bin count should only be incremented when the
sampled value has an x or z in the same bit positions, i.e., the comparison is done using ===. A wildcard
bin definition causes all x, z, or ? to be treated as wildcards for 0 or 1. For example:
wildcard bins g12_15 = { 4'b11?? };
The count of bin g12_15 is incremented when the sampled variable is between 12 and 15:
1100 1101 1110 1111
In the following example a separate bin is created for each of the four values shown above:

<!-- Page 596 -->

IEEE Std wildcard bins g12_15_array[] = { 4'b11?? };
Similarly, transition bins can define wildcard bins. For example:
wildcard bins T0_3 = (2'b0x => 2'b1x);
The count of transition bin T0_3 is incremented for the following transitions (as if by (0,1=>2,3)):
00 => 10 00 => 11 01 => 10 01 => 11
In the following example a separate bin is created for each of the four transition sequences previously
shown:
wildcard bins T0_3_array[] = (2'b0x => 2'b1x);
A wildcard bin definition only considers 2-state values; sampled values containing x or z are excluded.
Wildcard specification of coverpoint bins of real types is not allowed.
#### 19.5.5 Excluding coverage point values or transitions
A set of values or transitions associated with a coverage point can be explicitly excluded from coverage by
specifying them as ignore_bins. For example:
covergroup cg23;
coverpoint a
{
ignore_bins ignore_vals = {7,8};
ignore_bins ignore_trans = (1=>3=>5);
}
endgroup
All values or transitions associated with ignored bins are excluded from coverage. For state bins, each
ignored value is removed from the set of values associated with any coverage bin. For transition bins, any
covered sequence is removed when it cannot be matched without also matching an ignored sequence. (For
example, the ignored sequence 2=>3 would remove the covered sequence 1=>2=>3=>4.) The removal of
ignored values shall occur after the distribution of values to the specified bins. An ignored value has no
effect on a transition that includes the value. Ignored transition bins cannot specify a sequence of unbounded
or undetermined varying length.
The above may result in a bin that is associated with no values or sequences. Such empty bins are excluded
from coverage (see 19.11).
#### 19.5.6 Specifying Illegal coverage point values or transitions
A set of values or transitions associated with a coverage point can be marked as illegal by specifying them as
illegal_bins. For example:
covergroup cg3;
coverpoint b
{
illegal_bins bad_vals = {1,2,3};
illegal_bins bad_trans = (4=>5=>6);
}
endgroup

<!-- Page 597 -->

IEEE Std All values or transitions associated with illegal bins are excluded from coverage. For state bins, each illegal
value is removed from the set of values associated with any coverage bin. For transition bins, any covered
sequence is removed when it cannot be matched without also matching an illegal sequence. (For example,
the illegal sequence 2=>3 would remove the covered sequence 1=>2=>3=>4.) The removal of illegal values
shall occur after the distribution of values to the specified bins. If an illegal value or transition occurs, a run-
time error is issued. Illegal bins take precedence over any other bins; that is, they will result in a run-time
error even if they are also included in another bin. Specifying an illegal value has no effect on a transition
that includes the value. Illegal transition bins cannot specify a sequence of unbounded or undetermined
varying length.
The above may result in a bin that is associated with no values or sequences. Such empty bins are excluded
from coverage (see 19.11).
#### 19.5.7 Value resolution
A coverpoint expression, the expressions in a bins construct, and the coverpoint type, if present, are
involved in comparison operations in order to determine into which bins a particular value falls. Let e be the
coverpoint expression and b be an expression in a bins covergroup_range_list or
set_covergroup_expression. The following rules shall apply when evaluating e and b: For wildcard bins, x
and z values in b shall be treated as all possible 0 and 1 values prior to applying these rules.
a) If there is no coverpoint type, the effective type of e shall be self-determined. In the presence of a
coverpoint type, the effective type of e shall be the coverpoint type.
b) b shall be statically cast to the effective type of e. Enumeration values in expressions b and e shall
first be treated as being in an expression context. This implies that the type of an enumeration value
is the base type of the enumeration and not the enumeration type itself. An implementation shall
issue a warning under the following conditions:
1) If the effective type of e is unsigned and b is signed with a negative value.
2) If assigning b to a variable of the effective type of e would yield a value that is not equal to b
under normal comparison rules for ==.
3) If b yields a value with any x or z bits. This rule does not apply to wildcard bins because x
and z shall be treated as 0 and 1 as described above.
If a warning is issued for a bins element, the following rules shall apply:
— If an element of a bins covergroup_range_list or set_covergroup_expression is a singleton value b,
the element shall not participate in the bins values.
— If an element of a bins covergroup_range_list is a range [b1:b2] and either b1 or b2 contains any
x or z bits or every value in the range would generate a warning, then the element shall not
participate in the bins values.
— If an element of a bins covergroup_range_list is a range [b1:b2] and there exists at least one
value in the range for which a warning would not be issued, then the range shall be treated as
containing the intersection of the values in the range and the values expressible by the effective type
of e.
Examples:
bit [2:0] p1; // type expresses values in the range 0 to 7
bit signed [2:0] p2; // type expresses values in the range –4 to 3
covergroup g1 @(posedge clk);
coverpoint p1 {
bins b1 = { 1, [2:5], [6:10]};
bins b2 = {-1, [1:10], 15};
}
coverpoint p2 {

<!-- Page 598 -->

IEEE Std bins b3 = { 1, [2:5], [6:10]};
bins b4 = {-1, [1:10], 15};
}
endgroup
— For b1, a warning is issued for the range [6:10]. b1 is treated as though it had the specification
{1, [2:5], [6:7]}.
— For b2, a warning is issued for the range [1:10] and for the values –1 and 15. b2 is treated as
though it had the specification {[1:7]}.
— For b3, a warning is issued for the ranges [2:5] and [6:10]. b3 is treated as though it had the
specification {1, [2:3]}.
— For b4, a warning is issued for the range [1:10] and for the value 15. b2 is treated as though it had
the specification {-1, [1:3]}.
### 19.6 Defining cross coverage
A coverage group can specify cross coverage between two or more coverage points or variables. Cross
coverage is specified using the cross construct. When a variable V is part of a cross coverage,
SystemVerilog implicitly creates a coverage point for the variable, as if it had been created by the statement
coverpoint V;. Thus, a cross involves only coverage points. Expressions cannot be used directly in a
cross; a coverage point shall be explicitly defined first. A cross shall not directly include a real variable but
may include coverpoints of real expressions. (A cross_item may be a variable_identifier only for integral
variables, not for real variables.)
The syntax for specifying cross coverage is given in Syntax19-4.
cover_cross ::= // from A.2.11
[ cross_identifier : ] cross list_of_cross_items [ iff ( expression ) ] cross_body
list_of_cross_items ::= cross_item , cross_item { , cross_item }
cross_item ::=
cover_point_identifier
| variable_identifier
cross_body ::=
{ { cross_body_item } }
| ;
cross_body_item ::=
function_declaration
| bins_selection_or_option ;
bins_selection_or_option ::=
{ attribute_instance } coverage_option
| { attribute_instance } bins_selection
bins_selection ::= bins_keyword bin_identifier = select_expression [ iff ( expression ) ]
select_expression30 ::=
select_condition
| ! select_condition
| select_expression && select_expression
| select_expression || select_expression
| ( select_expression )
| select_expression with ( with_covergroup_expression ) [ matches integer_covergroup_expression ]
| cross_identifier

<!-- Page 599 -->

IEEE Std | cross_set_expression [ matches integer_covergroup_expression ]
select_condition ::= binsof ( bins_expression ) [ intersect { covergroup_range_list } ]
bins_expression ::=
variable_identifier
| cover_point_identifier [ . bin_identifier ]
covergroup_range_list ::= covergroup_value_range { , covergroup_value_range }
covergroup_value_range ::=
covergroup_expression
| [ covergroup_expression : covergroup_expression ]
| [ $ : covergroup_expression ]
| [ covergroup_expression : $ ]
| [ covergroup_expression +/- covergroup_expression ]
| [ covergroup_expression +%- covergroup_expression ]
with_covergroup_expression ::= covergroup_expression31
integer_covergroup_expression ::= covergroup_expression | $
cross_set_expression ::= covergroup_expression
30) The matches operator shall have higher precedence than the && and || operators.
31) The result of this expression shall be assignment compatible with an integral type as described in 19.5.1.1.
Syntax19-4—Cross coverage syntax (excerpt from AnnexA)
The label for a cross declaration provides an optional name. The label also creates a hierarchical scope for
the bins defined within the cross.
A cross name has limited visibility. An identifier can only refer to a cross in the following contexts:
— In a hierarchical name where the prefix specifies the name of a covergroup variable. For example,
cov1.crs.option.weight where cov1 is the name of a covergroup variable and crs is the name
of a cross declared within the covergroup.
— Following :: where the left operand of the scope resolution operator refers to a covergroup. For
example, covtype :: crs :: type_option.weight.
Identifiers and function calls within the cross are restricted in the same way as identifiers and function calls
within coverpoints (see 19.5). Functions declared within the cross shall not be visible outside that scope.
The expression within the optional iff provides a conditional guard for the cross coverage. If at any sample
point, the condition evaluates to false, the cross coverage is ignored. The expression within the optional iff
construct at the end of a cross bin definition provides a per-bin guard condition. If the expression is false, the
cross bin is ignored.
Cross coverage of a set of N coverage points is defined as the coverage of all combinations of all bins
associated with the N coverage points, that is, the Cartesian product of the N sets of coverage point bins. For
example:
bit [3:0] a, b;
covergroup cov @(posedge clk);
aXb : cross a, b;
endgroup

<!-- Page 600 -->

IEEE Std The coverage group cov in the preceding example specifies the cross coverage of two 4-bit variables, a and
b. SystemVerilog implicitly creates a coverage point for each variable. Each coverage point has 16 bins,
namely auto[0]...auto[15]. The cross of a and b (labeled aXb), therefore, has 256 cross products, and
each cross product is a bin of aXb.
Cross coverage between expressions previously defined as coverage points is also allowed. For example:
bit [3:0] a, b, c;
covergroup cov2 @(posedge clk);
BC: coverpoint b+c;
aXb : cross a, BC;
endgroup
The coverage group cov2 has the same number of cross products as the previous example, but in this case,
one of the coverage points is the expression b+c, which is labeled BC.
bit [31:0] a_var;
bit [3:0] b_var;
covergroup cov3 @(posedge clk);
A: coverpoint a_var { bins yy[] = {[0:9]}; }
CC: cross b_var, A;
endgroup
The coverage group cov3 crosses variable b_var with coverage point A (labeled CC). Variable b_var
automatically creates 16 bins (auto[0]...auto[15]). Coverage point A explicitly creates 10 bins
(yy[0]...yy[9]). The cross of two coverage points creates 16 × 10 = 160 cross product bins, namely the
following pairs:
<auto[0], yy[0]>
<auto[0], yy[1]>
...
<auto[0], yy[9]>
<auto[1], yy[0]>
...
<auto[15], yy[9]>
No cross coverage bins shall be created for coverpoint bins that are specified as default, ignored, or illegal
bins.
Cross coverage is allowed only between coverage points defined within the same coverage group. Coverage
points defined in a coverage group other than the one enclosing the cross cannot participate in a cross.
Attempts to cross items from different coverage groups shall result in a compiler error.
#### 19.6.1 Defining cross coverage bins
In addition to specifying the coverage points that are crossed, SystemVerilog includes a powerful set of
operators that allow defining cross coverage bins. Cross coverage bins can be specified in order to group
together a set of cross products. A cross coverage bin associates a name and a count with a set of cross
products. The count of the bin is incremented every time any of the cross products match, i.e., every
coverage point in the cross matches its corresponding bin in the cross product.
User-defined bins for cross coverage are defined using bin select expressions. The syntax for defining these
bin select expressions is given in Syntax19-4.

<!-- Page 601 -->

IEEE Std User-defined cross bins and automatically generated bins may coexist in the same cross. By default,
automatically generated bins are retained for those cross products that do not intersect cross products
specified by any user-defined cross bin. This may be controlled using the cross_retain_auto_bins
option.
Consider the following example code:
int i,j;
covergroup ct;
coverpoint i { bins i[] = { [0:1] }; }
coverpoint j { bins j[] = { [0:1] }; }
x1: cross i,j;
x2: cross i,j {
bins i_zero = binsof(i) intersect { 0 };
}
endgroup
Cross x1 has the following bins:
<i[0],j[0]>
<i[1],j[0]>
<i[0],j[1]>
<i[1],j[1]>
Cross x2 has the following bins:
i_zero // user-specified bin for <i[0],j[0]> and <i[0],j[1]>
<i[1],j[0]> // an automatically generated bin that is retained
<i[1],j[1]> // an automatically generated bin that is retained
The automatically generated cross bins (which are the same as the set given above for cross x1) are retained
for those bins that do not overlap the explicitly declared cross bins. In this particular case, since the
explicitly declared bin covers all cases for which i == 0, the cross will have the explicitly declared bin
(i_zero) plus automatically generated bins for cases where i != 0.
The binsof construct yields the bins of its expression, which can be either a coverage point (explicitly
defined or implicitly defined for a single variable) or a coverage point bin. The resulting bins can be further
selected by including (or excluding) only the bins whose associated values intersect a desired set of values.
The desired set of values can be specified using a comma-separated list of covergroup_value_range as
shown in Syntax19-4. For example, the select expression
binsof( x ) intersect { y }
denotes the bins of coverage point x whose values intersect the range given by y. Its negated form
! binsof( x ) intersect { y }
denotes the bins of coverage point x whose values do not intersect the range given by y.
The bins selected can be combined with other selected bins using the logical operators && and ||.
19.6.1.1 Example of user-defined cross coverage and select expressions
bit [7:0] v_a, v_b;
covergroup cg @(posedge clk);

<!-- Page 602 -->

IEEE Std a: coverpoint v_a
{
bins a1 = { [0:63] };
bins a2 = { [64:127] };
bins a3 = { [128:191] };
bins a4 = { [192:255] };
}
b: coverpoint v_b
{
bins b1 = {0};
bins b2 = { [1:84] };
bins b3 = { [85:169] };
bins b4 = { [170:255] };
}
c : cross a, b
{
bins c1 = ! binsof(a) intersect {[100:200]}; // 4 cross products
bins c2 = binsof(a.a2) || binsof(b.b2); // 7 cross products
bins c3 = binsof(a.a1) && binsof(b.b4); // 1 cross product
}
endgroup
The preceding example defines a coverage group named cg that samples its coverage points on the positive
edge of signal clk (not shown). The coverage group includes two coverage points, one for each of the two
8-bit variables, v_a and v_b. Coverage point a associated with variable v_a defines four equal-sized bins
for each possible value of variable v_a. Likewise, coverage point b associated with variable v_b defines
four bins for each possible value of variable v_b. Cross definition c specifies the cross coverage of the two
coverage points a and b. If the cross coverage of coverage points a and b were defined without any
additional cross bins (select expressions), then cross coverage of a and b would include 16 cross products
corresponding to all combinations of bins a1 through a4 with bins b1 through b4, that is, cross products
<a1,b1>, <a1,b2>, <a1,b3>, <a1,b4>...<a4,b1>, <a4,b2>, <a4,b3>, <a4,b4>.
The first user-defined cross bin, c1, specifies that c1 should include only cross products of coverage point a
that do not intersect the value range of 100 to 200. This select expression excludes bins a2, a3, and a4.
Thus, c1 will cover only four cross products of <a1,b1>, <a1,b2>, <a1,b3>, and <a1,b4>.
The second user-defined cross bin, c2, specifies that bin c2 should include only cross products whose values
are covered by bin a2 of coverage point a or cross products whose values are covered by bin b2 of coverage
point b. This select expression includes the following seven cross products: <a2,b1>, <a2,b2>,
<a2,b3>, <a2,b4>, <a1,b2>, <a3,b2>, and <a4,b2>.
The final user-defined cross bin, c3, specifies that c3 should include only cross products whose values are
covered by bin a1 of coverage point a and cross products whose values are covered by bin b4 of coverage
point b. This select expression includes only one cross product: <a1,b4>.
Additionally, the cross retains those automatically generated bins that represent cross products not
intersecting any of the user-defined bins. There are 6 of these: <a3,b1>, <a4,b1>, <a3,b3>, <a4,b3>,
<a3,b4>, and <a4,b4>.
When select expressions are specified on transition bins, the binsof operator uses the last value of the
transition.

<!-- Page 603 -->

IEEE Std 19.6.1.2 Cross bin with covergroup expressions
The with clause in a select_expression specifies that only those bin tuples in the subordinate
select_expression for which sufficiently many value tuples satisfy the given with_covergroup_expression
(i.e., for which the expression evaluates to true, as described in 12.4) are selected.
In the with_covergroup_expression, occurrences of cross_items (i.e., those cover_point_identifiers or
variable_identifiers occurring in the list_of_cross_items for the cross) represent corresponding values in the
value tuples of the candidate bin tuples.
When a cross_identifier is used as a select_expression, it selects all possible bin tuples. When used with a
with clause, the cross bin can be completely described using a with_covergroup_expression. Only the
cross_identifier of the enclosing cross may be used; other cross_identifiers shall be disallowed.
The optional matches clause specifies the selection policy. The integer_covergroup_expression shall
evaluate to a positive integer or $, representing the minimum number of satisfying value tuples required to
select the candidate bin tuple. The $ symbol specifies that all value tuples are required to satisfy the
expression in order to select the candidate bin tuple. When the matches clause is omitted, the selection
policy defaults to one.
Consider the following example:
logic [0:7] a, b;
parameter [0:7] mask;
...
covergroup cg;
coverpoint a
{
bins low[] = {[0:127]};
bins high = {[128:255]};
}
coverpoint b
{
bins two[] = b with (item % 2 == 0);
bins three[] = b with (item % 3 == 0);
}
X: cross a,b
{
bins apple = X with (a+b < 257) matches 127;
bins cherry = ( binsof(b) intersect {[0:50]}
&& binsof(a.low) intersect {[0:50]}) with (a==b);
bins plum = binsof(b.two) with (b > 12)
|| binsof(a.low) with (a & b & mask);
}
endgroup
The bin structure for coverpoint a is straightforward—bin array low contains 128 single-element bins for
each value between 0 and 127, and bin high contains all values from 128 to 255. The bins of coverpoint b
are specified using the with clause; bin array two contains a bin for each even value, and three contains a
bin for each value divisible by 3.
The cross X crosses coverpoints a and b. Three cross bins are defined, apple, cherry, and plum. apple
consists of all bin tuples for which a+b < 257 for at least 127 value tuples. In this example, apple would
consist of three coverpoint bin tuples: <high, two[0]>, <high, two[2]>, and <high, three[0]>.
The cross bin cherry demonstrates using the with clause on a complex select_expression. First, those bin
tuples consisting of a bin from b containing a value between 0 and 50 are selected; then, the && operator

<!-- Page 604 -->

IEEE Std selects from those bin tuples the bin tuples with a bin from a.low containing a value between 0 and 50. The
with clause then selects from those only the bin tuples containing at least one value tuple where a==b.
The cross bin plum demonstrates a select_expression composed of with covergroup expressions. The first
with covergroup expression selects those bin tuples containing bins in the b.two bin array whose values
are greater than 12. The || operator then adds the bin tuples selected by the second with covergroup
expression—namely those containing a bin from a.low and for which the bitwise-AND of the a-value,
b-value and a mask is nonzero for some values a and b in the bins of the bin tuple.
As with array manipulation methods involving with (see 7.12), if the expression has side effects, the results
are unpredictable.
The with clause behaves as if the expression were evaluated for every value tuple of every bin tuple
selected by the subordinate select_expression at the time the covergroup instance is constructed. However,
implementations are not required to schedule the evaluation events when calculating the bin tuples in the
cross bin; all, some, or none of the events may be scheduled. The intent of these rules is to allow the use of
non-simulation analysis techniques to calculate the cross bin (for example, formal symbolic analysis), or for
caching of previously calculated results.
19.6.1.3 Cross bin automatically defined types
A cross defines a coverage space composed of tuples of values. To aid in describing the structure of that
space, SystemVerilog provides automatically defined types for these tuples and queues of tuples in each
cross. The types are named CrossValType and CrossQueueType. The scope of the type names is the
cross itself and the types are not accessible outside this scope.
The definition of CrossValType is a SystemVerilog struct consisting of one member for each coverpoint in
the cross. The name and type of each field are the name and type of the corresponding coverpoint. If range
bounds for the coverpoint type are not evident (e.g., the coverpoint expression is a concatenation and no
other type is specified), the bounds are assumed to be [$bits(coverpoint_expression)-1:0]. The
definition of CrossQueueType is an unbounded queue of CrossValType elements.
The cross types shall be considered as implicit typedefs in the body of the cross, even though the syntax does
not allow typedefs to appear there explicitly. Consider the following example:
covergroup cg (ref logic [0:3] x, ref logic [0:7] y, ref logic [0:2] a);
xy: coverpoint {x,y};
coverpoint y;
XYA: cross xy, a
{
// the cross types are as if defined here as follows:
// typedef struct {logic [11:0] xy;logic [0:2] a;} CrossValType;
// typedef CrossValType CrossQueueType[$];
};
endgroup
Subclause 19.6.1.4 shows how CrossValType and CrossQueueType can be used to compute explicit
enumerations of cross bins.
19.6.1.4 Cross bin set expression
The cross_set_expression syntax allows specifying an expression yielding a queue of elements that define
the cross bin, similarly to the set_covergroup_expression for coverpoint bins. However, for cross bins the
type of the queue shall be the cross’s CrossQueueType, whose elements are of type CrossValType (see
19.6.1.3).

<!-- Page 605 -->

IEEE Std The selection of bin tuples for the cross bin by the elements of the cross_set_expression is subject to the
same policy specification as the cross bin with covergroup expression (see 19.6.1.2). The optional
matches expression specifies the number of value tuples in a bin tuple that shall be present in the
cross_set_expression for that bin tuple to be selected. The default policy is one, denoting the policy where a
single value tuple from a bin tuple shall exist in the cross_set_expression to select the bin tuple.
For example:
int a;
logic [7:0] b;
covergroup cg;
coverpoint a { bins x[] = {[0:10]}; }
coverpoint b { bins y[] = {[0:20]}; }
aXb : cross a, b
{
bins one = '{ '{1,2}, '{3,4}, '{5,6} };
}
endgroup
The cross bin definition uses an array literal to define the bin tuples in cross bin one as <a.x[1],
b.y[2]>, <a.x[3], b.y[4]>, and <a.x[5], b.y[6]>. Here, the cross bin provides the context
required to determine the type of the literal (in this case, the cross's CrossQueueType). In general, literal
arrays are not required; any expression may be used as long as it evaluates to the cross’s CrossQueueType.
A cast is required if the type is assignment-incompatible with the cross type.
Following is a more involved example:
module mod_m;
logic [31:0] a, b;
covergroup cg(int cg_lim);
coverpoint a;
coverpoint b;
aXb : cross a, b
{
function CrossQueueType myFunc1(int f_lim);
for (int i = 0; i < f_lim; ++i)
myFunc1.push_back('{i,i});
endfunction
bins one = myFunc1(cg_lim);
bins two = myFunc2(cg_lim);
function CrossQueueType myFunc2(logic [31:0] f_lim);
for (logic [31:0] i = 0; i < f_lim; ++i)
myFunc2.push_back('{2*i,2*i});
endfunction
}
endgroup
cg cg_inst = new(3);
endmodule
Here functions are used to create the queues that define the cross bins. Note that the coverpoints a and b are
32 bits wide; iterating over all value tuples using a with expression would be computationally expensive.

<!-- Page 606 -->

IEEE Std By using functions, the user is able to restrict the bin computation to a reasonable subset of value tuples; the
entire cross space need not be considered.
As shown, the bins for cg_inst are as follows:
cg_inst.aXb.one = <0,0>, <1,1>, <2,2>
cg_inst.aXb.two = <0,0>, <2,2>, <4,4>
#### 19.6.2 Excluding cross products
A group of bins can be excluded from coverage by specifying a select expression using ignore_bins. For
example:
covergroup yy;
cross a, b
{
ignore_bins ignore = binsof(a) intersect { 5, [1:3] };
}
endgroup
All cross products that satisfy the select expression are excluded from coverage. Ignored cross products are
excluded even if they are included in other cross coverage bins of the enclosing cross.
#### 19.6.3 Specifying illegal cross products
A group of bins can be marked as illegal by specifying a select expression using illegal_bins. For
example:
covergroup zz(int bad);
cross x, y
{
illegal_bins illegal = binsof(y) intersect {bad};
}
endgroup
All cross products that satisfy the select expression are excluded from coverage, and a run-time error is
issued. Illegal cross products take precedence over any other cross products; that is, they will result in a run-
time error even if they are also explicitly ignored (using an ignore_bins) or included in another cross bin.
### 19.7 Specifying coverage options
Options control the behavior of the covergroup, coverpoint, and cross. There are two types of options:
those that are specific to an instance of a covergroup and those that specify an option for the covergroup type
as a whole.
Specifying a value for the same option more than once within the same covergroup definition shall be an
error.

<!-- Page 607 -->

IEEE Std Table19-1 lists instance-specific covergroup options and their description. Each instance of a covergroup
can initialize an instance-specific option to a different value. The initialized option value affects only that
instance.
Table19-1—Instance-specific coverage options
Option name Default Description
name=string unique Specifies a name for the covergroup instance. If unspecified, a
name unique name for each instance is automatically generated by the
tool.
weight= number 1 If set at the covergroup syntactic level, it specifies the weight
of this covergroup instance for computing the overall instance
coverage of the simulation. If set at the coverpoint (or
cross) syntactic level, it specifies the weight of a coverpoint
(or cross) for computing the instance coverage of the enclosing
covergroup. The specified weight shall be a non-negative
integral value.
goal=number 100 Specifies the target goal for a covergroup instance or for a
coverpoint or a cross of an instance.
comment=string "" A comment that appears with a covergroup instance or with a
coverpoint or cross of the covergroup instance. The comment is
saved in the coverage database and included in the coverage
report.
at_least=number 1 Minimum number of hits for each bin. A bin with a hit count that
is less than number is not considered covered.
auto_bin_max=number 64 Maximum number of automatically created bins when no bins are
explicitly defined for a coverpoint.
cross_num_print_missing= 0 Number of missing (not covered) cross product bins that shall be
number saved to the coverage database and printed in the coverage report.
cross_retain_auto_bins= 1 When true, automatically generated cross bins are retained for
boolean those cross products that do not intersect cross products specified
by any user-defined cross bin. When false, all automatically gen-
erated bins are removed by the presence of any explicit cross bin.
detect_overlap=boolean 0 When true, a warning is issued if there is an overlap between the
range list (or transition list) of two bins of a coverpoint.
per_instance=boolean 0 Each instance contributes to the overall coverage information for
the covergroup type. When true, coverage information for this
covergroup instance shall be saved in the coverage database and
included in the coverage report. When false, implementations are
not required to save instance-specific information.
get_inst_coverage=boolean 0 Only applies when the merge_instances type option is set.
Enables the tracking of per instance coverage with the
get_inst_coverage built-in method. When false, the
value returned by get_inst_coverage shall equal the
value returned by get_coverage.
The instance-specific options mentioned in Table19-1 can be set in the covergroup definition. The syntax
for setting these options in the covergroup definition is as follows:
option . member_name = expression ;

<!-- Page 608 -->

IEEE Std The identifier option is a built-in member of every covergroup, coverpoint and cross (see 19.10 for a
description).
For example:
covergroup g1 (int w, string instComment) @(posedge clk) ;
// track coverage information for each instance of g1 in addition
// to the cumulative coverage information for covergroup type g1
option.per_instance = 1;
// comment for each instance of this covergroup
option.comment = instComment;
a : coverpoint a_var
{
// Create 128 automatic bins for coverpoint "a" of each instance of g1
option.auto_bin_max = 128;
}
b : coverpoint b_var
{
// This coverpoint contributes w times as much to the coverage of an
// instance of g1 as coverpoints "a" and "c1"
option.weight = w;
}
c1 : cross a_var, b_var ;
endgroup
Option assignment statements in the covergroup definition are evaluated at the time that the covergroup is
instantiated. The per_instance and get_inst_coverage options can only be set in the covergroup
definition. The auto_bin_max, detect_overlap, and cross_retain_auto_bins options can only be
set in the covergroup or coverpoint definition. Other instance-specific options can be assigned
procedurally after a covergroup has been instantiated.
For example:
covergroup gc (int maxA, int maxB) @(posedge clk) ;
a : coverpoint a_var;
b : coverpoint b_var;
endgroup
...
gc g1 = new (10,20);
g1.option.comment = "Here is a comment set for the instance g1";
g1.a.option.weight = 3; // Set weight for coverpoint "a" of instance g1
Table19-2 summarizes the syntactical level (covergroup, coverpoint, or cross) at which instance
options can be specified. All instance options can be specified at the covergroup level. Except for the
weight, goal, comment, and per_instance options, all other options set at the covergroup syntactic
level act as a default value for the corresponding option of all coverpoints and crosses in the covergroup.
Individual coverpoints or crosses can overwrite these default values. When set at the covergroup level, the
weight, goal, comment, and per_instance options do not act as default values to the lower syntactic
levels.

<!-- Page 609 -->

IEEE Std Table19-2—Coverage options per syntactic level
Allowed in syntactic level
Option name
covergroup coverpoint cross
name Yes No No
weight Yes Yes Yes
goal Yes Yes Yes
comment Yes Yes Yes
at_least Yes (default for coverpoints & crosses) Yes Yes
auto_bin_max Yes (default for coverpoints) Yes No
cross_num_print_missing Yes (default for crosses) No Yes
cross_retain_auto_bins Yes (default for crosses) No Yes
detect_overlap Yes (default for coverpoints) Yes No
per_instance Yes No No
get_inst_coverage Yes No No
#### 19.7.1 Covergroup type options
Table19-3 lists options that describe particular features (or properties) of the covergroup type as a whole.
They are analogous to static data members of classes.
The covergroup type options mentioned in Table19-3 can be set in the covergroup definition. The
syntax for setting these options in the covergroup definition is as follows:
type_option . member_name = constant_expression ;
The identifier type_option is a built-in static member of every covergroup, coverpoint, and cross (see
### 19.10 for a description).
Table19-3—Coverage group type (static) options
Option name Default Description
weight=constant_number 1 If set at the covergroup syntactic level, it specifies the weight of
this covergroup for computing the overall cumulative (or type)
coverage of the saved database. If set at the coverpoint (or
cross) syntactic level, it specifies the weight of a coverpoint (or
cross) for computing the cumulative (or type) coverage of the
enclosing covergroup. The specified weight shall be a
non-negative integral value.
goal=constant_number 100 Specifies the target goal for a covergroup type or for a coverpoint
or cross of a covergroup type.
comment=string_literal "" A comment that appears with the covergroup type or with a
coverpoint or cross of the covergroup type. The comment is saved in
the coverage database and included in the coverage report.

<!-- Page 610 -->

IEEE Std Table19-3—Coverage group type (static) options (continued)
Option name Default Description
strobe=boolean 0 When true, all samples happen at the end of the time slot, like the
$strobe system task.
merge_instances=boolean 0 When true, cumulative (or type) coverage is computed by merging
instances together as the union of coverage of all instances. When
false, type coverage is computed as the weighted average of
instances.
distribute_first=boolean 0 When true, instructs the tool to perform value distribution to the bins
prior to application of the with_covergroup_expression.
real_interval=real 1.0 Specifies the width of real value range intervals distributed among
multiple bins
Different instances of a covergroup cannot assign different values to type options. This is syntactically
disallowed because these options can only be initialized via constant expressions (see 11.2.1). For example:
covergroup g1 (int w, string instComment) @(posedge clk) ;
// track coverage information for each instance of g1 in addition
// to the cumulative coverage information for covergroup type g1
option.per_instance = 1;
type_option.comment = "Coverage model for features x and y";
type_option.strobe = 1; // sample at the end of the time slot
// compute type coverage as the merge of all instances
type_option.merge_instances = 1;
// comment for each instance of this covergroup
option.comment = instComment;
a : coverpoint a_var
{
// Use weight 2 to compute the coverage of each instance
option.weight = 2;
// Use weight 3 to compute the cumulative (type) coverage for g1
type_option.weight = 3;
// NOTE: type_option.weight = w would cause syntax error.
}
b : coverpoint b_var
{
// Use weight w to compute the coverage of each instance
option.weight = w;
// Use weight 5 to compute the cumulative (type) coverage of g1
type_option.weight = 5;
}
endgroup
In the preceding example, the coverage for each instance of g1 is computed as follows:
(((instance coverage of a) × 2) + ((instance coverage of b) × w)) / ( 2 + w)
On the other hand, the coverage for covergroup type g1 is computed as follows:

<!-- Page 611 -->

IEEE Std ( ((merge of coverage of a from all instances) × 3)
+ ((merge of coverage of b from all instances) × 5) ) / (3 + 5)
The strobe and real_interval type options can only be set in the covergroup definition. Other type
options can be assigned procedurally at any time during simulation.
For example:
covergroup gc @(posedge clk) ;
a : coverpoint a_var;
b : coverpoint b_var;
endgroup
...
gc::type_option.comment = "Here is a comment for all covergroups of type gc";
// Set the weight for coverpoint "a" of all covergroups of type gc
gc::a::type_option.weight = 3;
gc g1 = new;
Table19-4 summarizes the syntactical level (covergroup, coverpoint, or cross) in which type options
can be specified. When set at the covergroup level, the type options do not act as defaults for lower syntactic
levels, except for real_interval.
Table19-4—Coverage type options
Allowed syntactic level
Option name
covergroup coverpoint cross
weight Yes Yes Yes
goal Yes Yes Yes
comment Yes Yes Yes
strobe Yes No No
merge_instances Yes No No
distribute_first Yes No No
real_interval Yes Yes No
### 19.8 Predefined coverage methods
The coverage methods in Table19-5 are provided for the covergroup. These methods can be invoked
procedurally at any time.
Table19-5—Predefined coverage methods
Can be called on
Method
Description
(function)
covergroup coverpoint cross
void sample() Yes No No Triggers sampling of the
covergroup

<!-- Page 612 -->

IEEE Std Table19-5—Predefined coverage methods (continued)
real get_coverage() Yes Yes Yes Calculates type coverage number
real get_coverage(ref int, ref int) (0...100)
real get_inst_coverage() Yes Yes Yes Calculates the coverage number
real get_inst_coverage(ref int, ref int) (0...100)
void set_inst_name(string) Yes No No Sets the instance name to the
given string
void start() Yes Yes Yes Starts collecting coverage
information
void stop() Yes Yes Yes Stops collecting coverage
information
The get_coverage() method returns the cumulative (or type) coverage, which considers the contribution
of all instances of a particular coverage item; and it is a static method that is available on both types (via the
:: operator) and instances (using the . operator). In contrast, the get_inst_coverage() method returns
the coverage of the specific instance on which it is invoked; thus, it can only be invoked via the . operator.
The get_coverage() and get_inst_coverage() methods both accept an optional set of arguments, a
pair of int values passed by reference. When the optional arguments are specified, the get_coverage()
and get_inst_coverage() methods assign to the first argument the number of covered bins and to the
second argument the number of coverage bins defined for the given coverage item. When
get_inst_coverage() is called on a coverpoint or cross, these two values correspond to the
numerator and the denominator used for calculating the particular coverage number (i.e., the return value
before scaling by 100); in other cases, these two values do not necessarily correspond to the numerator and
denominator. When get_inst_coverage() is called on a covergroup, these values are aggregated
numbers of bins from all constituent coverpoints and crosses of the same instance. When get_coverage()
is called on a coverpoint or cross, these values are aggregated numbers of bins from the same
coverpoint or cross in all instances. When get_coverage() is called on a covergroup, these values
are aggregated numbers of bins from all coverpoints and crosses in all instances.
For example:
covergroup cg (int xb, yb, ref int x, y) ;
coverpoint x {bins xbins[] = { [0:xb] }; }
coverpoint y {bins ybins[] = { [0:yb] }; }
endgroup
cg cv1 = new (1,2,a,b); // cv1.x has 2 bins, cv1.y has 3 bins
cg cv2 = new (3,6,c,d); // cv2.x has 4 bins, cv2.y has 7 bins
initial begin
cv1.x.get_inst_coverage(covered,total); // total = 2
cv1.get_inst_coverage(covered,total); // total = 5
cg::x::get_coverage(covered,total); // total = 6
cg::get_coverage(covered,total); // total = 16
end
#### 19.8.1 Overriding the built-in sample method
Overriding the predefined sample() method with a triggering function that accepts arguments facilitates
sampling coverage data from contexts other than the scope enclosing the covergroup declaration. For
example, an overridden sample method can be called with different arguments to pass directly to a
covergroup the data to be sampled from within an automatic task or function, or from within a particular
instance of a process, or from within a sequence or property of a concurrent assertion. Since concurrent

<!-- Page 613 -->

IEEE Std assertions have special sampling semantics (values are sampled in the Preponed region), passing their values
as arguments to an overridden sample method facilitates managing various aspects of assertion coverage,
such as sampling of multiple covergroups by one property, sampling of multiple properties by the same
covergroup, or sampling different branches of a sequence or property (including local variables) by any
arbitrary covergroup.
For example:
covergroup p_cg with function sample(bit a, int x);
coverpoint x;
cross x, a;
endgroup : p_cg
p_cg cg1 = new;
property p1;
int x;
@(posedge clk)(a, x = b) ##1 (c, cg1.sample(a, x));
endproperty : p1
c1: cover property (p1);
function automatic void F(int j);
bit d;
...
cg1.sample( d, j );
endfunction
The preceding example declares covergroup p_cg whose sample method is overridden to accept two
arguments: a and x. The sample method of an instance of this covergroup (cg1) is then called directly from
within property p1 and from the automatic function F().
The formal arguments of an overridden sample method shall be searched before the enclosing scope; each
such argument may only designate a coverpoint or conditional guard expression. It shall be an error to use a
sample formal argument in any context other than a coverpoint or conditional guard expression. Formal
sample arguments shall not designate an output direction. The formal arguments of an overridden sample
method belong to the same lexical scope as the formal arguments to the covergroup (consumed by the
covergroup new operator). Hence, it shall be an error for the same argument name to be specified in both
argument lists.
For example:
covergroup C1 (int v) with function sample (int v, bit b); // error (v)
coverpoint v;
option.per_instance = b;// error: b may only designate a coverpoint
option.weight = v; // error: v is ambiguous
endgroup
### 19.9 Predefined coverage system tasks and system functions
SystemVerilog provides the following system tasks and system functions to help manage coverage data
collection.
— $set_coverage_db_name(filename) sets the file name of the coverage database into which
coverage information is saved at the end of a simulation run.
— $load_coverage_db(filename) loads from the given file name the cumulative coverage
information for all coverage group types.

<!-- Page 614 -->

IEEE Std — $get_coverage() returns as a real number in the range of 0 to 100 the overall coverage of all
coverage group types. This number is computed as previously described.
### 19.10 Organization of option and type_option members
The option and type_option members of a covergroup, coverpoint, and cross are implicitly declared
structures with the following composition:
struct // covergroup option declaration
{
string name ;
int weight ;
int goal ;
string comment ;
int at_least ;
int auto_bin_max ;
int cross_num_print_missing ;
bit cross_retain_auto_bins ;
bit detect_overlap ;
bit per_instance ;
bit get_inst_coverage ;
} option;
struct // coverpoint option declaration
{
int weight ;
int goal ;
string comment ;
int at_least ;
int auto_bin_max ;
bit detect_overlap ;
} option;
struct // cross option declaration
{
int weight ;
int goal ;
string comment ;
int at_least ;
int cross_num_print_missing ;
bit cross_retain_auto_bins ;
} option;
struct // covergroup type_option declaration
{
int weight ;
int goal ;
string comment ;
bit strobe ;
bit merge_instances ;
bit distribute_first ;
real real_interval;
} type_option;
struct // coverpoint type_option declaration
{
int weight ;

<!-- Page 615 -->

IEEE Std int goal ;
string comment ;
real real_interval;
} type_option;
struct // cross type_option declaration
{
int weight ;
int goal ;
string comment ;
} type_option;
### 19.11 Coverage computation
This subclause describes how SystemVerilog computes functional coverage numbers. The cumulative (or
type) coverage considers the contribution of all instances of a particular coverage item, and it is the value
returned by the get_coverage() method. Thus, when applied to a covergroup, the get_coverage()
method returns the contribution of all instances of that particular covergroup. In contrast, the
get_inst_coverage() method returns the coverage of the specific coverage instance on which it is
invoked. Because get_coverage() is a static method, it is available for both types (via the :: operator)
and instances (using the . operator). There are two different ways in which type coverage can be computed,
selected with type_option.merge_instances. See 19.11.3.
The coverage of a coverage group, C , is the weighted average of the coverage of all items defined in the
g
coverage group, and it is computed according to the following equation:
W ×C
i i
C = ---i-------------------------
g
W
i
i
where
i ∈ set of coverage items (coverage points and crosses) defined in the coverage group
W is the weight associated with item i
i
C is the coverage of item i
i
The coverage of each item, C, is a measure of how much the item has been covered, and its computation
i
depends on the type of coverage item: coverpoint or cross. Each of these is described in 19.11.1 and
19.11.2, respectively.
The rules for computation of the coverage C of an item may indicate that the item is to be excluded from the
i
coverage computation. In this case, the contribution of the item is excluded from both the numerator and the
denominator.
There are several circumstances that can result in the denominator of the covergroup calculation equation
being zero, as follows:
— All items in a covergroup are excluded from coverage due to the rules for computation of C
i
— All weights W are zero
i
— A covergroup contains no coverpoints or crosses
Any zero denominator in the coverage calculation shall result in the following:
a) The covergroup does not contribute to the overall coverage score.

<!-- Page 616 -->

IEEE Std b) If the covergroup’s weight is nonzero, a value of 0.0 is returned by get_coverage and
get_inst_coverage.
c) If the covergroup’s weight is zero, a value of 100.0 is returned by get_coverage and
get_inst_coverage.
d) If get_coverage or get_inst_coverage is called with two arguments, zero is assigned to both
arguments—the numerator and denominator.
Consistent with the above behavior, $get_coverage shall return a value of 100.0 if called on a design that
has no covergroup instances, or if called on a design in which all covergroups have a weight of 0.
#### 19.11.1 Coverpoint coverage computation
Coverage of a coverpoint item is computed differently depending on whether the bins of the coverage point
are explicitly defined by the user (see 19.5.1, 19.5.2) or automatically created by the tool (see 19.5.3). For
user-defined bins, the coverage of a coverpoint is computed as follows:
bins
C = -------------c--o--v--e--r--e-d---
i bins
where
|bins| is the cardinality of the set of bins defined
|bins | is the cardinality of the covered bins—the subset of all (defined) bins that are covered
covered
For automatically generated bins, the coverage of a coverpoint is computed as follows:
bins
C = ------------------------------c-o---v--e-r--e--d-------------------
i M
MIN(auto_bin_max, 2 )
where
|bins | is the cardinality of the covered bins—the subset of all (auto-defined) bins that are
covered
covered
M is the minimum number of bits needed to represent the coverpoint
auto_bin_max is the value of the auto_bin_max option in effect (see 19.7)
If there is no value or transition associated with a bin, the bin is ignored and shall not contribute to the
coverage computation; that is, the bin is excluded from both the numerator and the denominator of the
coverage equation.
If none of the bins have an associated value or transition, the denominator of the coverage calculation is
zero. In this case:
a) The coverpoint does not contribute to the coverage computation (of the parent covergroup).
b) If the coverpoint’s weight is nonzero, a value of 0.0 is returned by get_coverage and
get_inst_coverage.
c) If the coverpoint’s weight is zero, a value of 100.0 is returned by get_coverage and
get_inst_coverage.
d) If get_coverage or get_inst_coverage is called with two arguments, zero is assigned to both
arguments—the numerator and denominator.
For example:
bit [2:0] a, b;
covergroup ct;

<!-- Page 617 -->

IEEE Std coverpoint b {
option.auto_bin_max = 4;
ignore_bins ig = {[0:1], [5:6]};
}
endgroup
In this case, coverpoint b will have four auto bins: auto[0,1], auto[2,3], auto[4,5], auto[6,7]. The
ignore_bins declaration specifies that the values 0,1,5,6 are ignored. After applying the ignore_bins,
the bins are: auto[], auto[2,3], auto[4], auto[7]. Since it is no longer is associated with any value,
auto[] does not contribute to coverage.
To determine whether a particular bin of a coverage group is covered, the cumulative coverage computation
considers the value of the at_least option of all instances being accumulated. Consequently, a bin is not
considered covered unless its hit count equals or exceeds the maximum of all the at_least values of all
instances. Use of the maximum represents the more conservative choice.
#### 19.11.2 Cross coverage computation
The coverage of a cross item is computed according to the following equation:
bins
C = -------------c--o--v--e--r--e-d---
i B +B
c u
B =
∏B 
–B
c  j b
j
where
j ∈ set of coverpoints being crossed
B is the cardinality (number of bins) of the jth coverpoint being crossed
j
B is the number of auto-cross bins
c
B u is the number of significant user-defined cross bins—excluding ignore_bins and
illegal_bins
B is the number of cross products that comprise all user-defined cross bins
b
The term B represents user-defined bins that contribute towards coverage.
u
If the denominator of the cross coverage calculation equation has a value of zero:
a) The cross does not contribute to the coverage computation (of the parent covergroup).
b) If the cross’s weight is nonzero, a value of 0.0 is returned by get_coverage and
get_inst_coverage.
c) If the cross’s weight is zero, a value of 100.0 is returned by get_coverage and
get_inst_coverage.
d) If get_coverage or get_inst_coverage is called with two arguments, zero is assigned to both
arguments—the numerator and denominator.
#### 19.11.3 Type coverage computation
Cumulative (or type) coverage is computed in two ways. When type_option.merge_instances is false,
type coverage is computed as the weighted average of all instances. When
type_option.merge_instances is true, type coverage is computed as if instances were merged together
into the type, as a union of coverage of all instances.

<!-- Page 618 -->

IEEE Std When type coverage is computed as the weighted average of all instances, the covergroup type coverage
depends on the instances only, not its coverpoints or crosses, as follows:
W ×I
i i
W
i
where
W is the option.weight of a covergroup instance
i
I is the coverage of a covergroup instance
i
Likewise, the type coverage of a coverpoint or cross is computed from the coverage of that coverpoint or
cross in each instance, weighted by option.weight in the coverpoint or cross scope for each instance.
The values returned by get_coverage(ref int, ref int) are consistent with the weighted sum above
when type_option.merge_instances is false.
When type coverage is computed as the merge of coverage from all instances, the union of all bins from all
instances shall be computed. To determine when bins overlap among instances, the bin name is used as
described in detail as follows. When bins overlap among instances, the cumulative coverage count of an
overlapping bin is the sum of counts of that bin in all instances containing it. For example:
covergroup gt (int l, h);
coverpoint a {bins b[] = { [l:h] };}
endgroup
gt gv1 = new (0,1);
gt gv2 = new (1,2);
In this case, bin b[1] overlaps between instances referenced by gv1 and gv2. The covergroup gt has bins
b[0], b[1], and b[2]. If a==0 were sampled by gv1 and a==1 sampled by both gv1 and gv2, the
gt::get_coverage() value would be 66.6667 because 2 out of 3 type bins were covered; the cumulative
count for the bin b[1] would be 2. If instance coverage were enabled with option.get_inst_coverage
equal to 1 for both instances, gv1.get_inst_coverage() would return 100.0 and
gv2.get_inst_coverage() would return 50.0.
To compute the union of all bins in all instances, bins are compared by name, so that bins with the same
name are overlapping among instances. For state or transition bins declared as “binname,” all instances
share the same name, so the bin overlaps in all instances. For state bins declared as “binname[],” bin names
are “binname[value]” (as specified in 19.5.1) for a set of scalar values. Instances sharing the same value
have overlapping bins. For state bins declared as “binname[N],” bin names range from “binname[0]”
through “binname[N-1].” Instances sharing the same indices have overlapping bins. For automatically
created bins, bin names are of the form “auto[value]” or “auto[low:high]” (as specified in 19.5.3), and these
names are unaffected by ignored or illegal values in the coverpoint except inasmuch as they may empty an
automatically created bin. Instances sharing the same value or low:high range have overlapping bins. For
transition bins declared as “binname[],” bin names are “binname[transition]” for some bounded transition
(as specified in 19.5.2). Instances sharing the same transition have overlapping bins. For automatically
created cross bins, bin names are of the form “<binname1,...,binnameN>” where the bin names are derived
from the crossed coverpoint bins (as specified in 19.6). Instances sharing exactly the same cross product bin
name have overlapping bins.
The following example shows automatically created bins:
bit [7:0] a;
covergroup ga (int abm);
option.auto_bin_max = abm;

<!-- Page 619 -->

IEEE Std coverpoint a {ignore_bins i = {3};}
endgroup
ga gv1 = new (64);
ga gv2 = new (32);
In this case, the bins of the instance referenced by gv1 are auto[0:3] through auto[252:255], while the
bins of the instance referenced by gv2 are auto[0:7] through auto[248:255]. Note how the ignored
value 3 does not have an effect on the bin names. Because none of the bin names overlap between the two
instances, the covergroup type ga has 96 cumulative bins.

<!-- Page 620 -->

IEEE Std 1800-2023