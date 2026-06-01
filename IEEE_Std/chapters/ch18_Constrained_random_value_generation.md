# Chapter 18: Constrained random value generation

## 18. Constrained random value generation
### 18.1 General
This clause describes the following:
— Random variables
— Constraint blocks
— Randomization methods
— Disabling randomization
— Controlling constraints
— Scope variable randomization
— Seeding the random number generator (RNG)
— Random weighted case statements
— Random sequence generation
### 18.2 Overview
Constraint-driven test generation allows users to automatically generate tests for functional verification.
Random testing can be more effective than a traditional, directed testing approach. By specifying
constraints, one can easily create tests that can find hard-to-reach corner cases. SystemVerilog allows users
to specify constraints in a compact, declarative way. The constraints are then processed by a solver that
generates random values that meet the constraints.
The random constraints are typically specified on top of an object-oriented data abstraction that models the
data to be randomized as objects that contain random variables and user-defined constraints. The constraints
determine the legal values that can be assigned to the random variables. Objects are ideal for representing
complex aggregate data types and protocols such as Ethernet packets.
Subclause 18.3 provides an overview of object-based randomization and constraint programming. The rest
of this clause provides detailed information on random variables, constraint blocks, and the mechanisms
used to manipulate them.
### 18.3 Concepts and usage
This subclause introduces the basic concepts and uses for generating random stimulus within objects.
SystemVerilog uses an object-oriented method for assigning random values to the member variables of an
object, subject to user-defined constraints. For example:
class Bus;
rand bit[15:0] addr;
rand bit[31:0] data;
constraint word_align {addr[1:0] == 2'b0;}
endclass
The Bus class models a simplified bus with two random variables, addr and data, representing the address
and data values on a bus. The word_align constraint declares that the random values for addr shall be
such that addr is word-aligned (the two low-order bits are 0).
The randomize() method (see 18.6.1) is called to generate new random values for a bus object:

<!-- Page 524 -->

IEEE Std Bus bus = new;
repeat (50) begin
if (bus.randomize() == 1)
$display ("addr = %4h data = %h\n", bus.addr, bus.data);
else
$display ("Randomization failed.\n");
end
Calling randomize() causes new values to be selected for all of the random variables in an object so that
all of the constraints are true (satisfied). In the preceding program test, a bus object is created and then
randomized 50 times. The result of each randomization is checked for success. If the randomization
succeeds, the new random values for addr and data are printed; if the randomization fails, an error
message is printed. In this example, only the addr value is constrained, while the data value is
unconstrained. Unconstrained variables are assigned any value in their declared range.
Constraint programming is a powerful method that lets users build generic, reusable objects that can later be
extended or constrained to perform specific functions. The approach differs from both traditional procedural
and object-oriented programming, as illustrated in this example that extends the Bus class:
typedef enum {low, mid, high} AddrType;
class MyBus extends Bus;
rand AddrType atype;
constraint addr_range
{
(atype == low ) -> addr inside { [0 : 15] };
(atype == mid ) -> addr inside { [16 : 127]};
(atype == high) -> addr inside {[128 : 255]};
}
endclass
The MyBus class inherits all of the random variables and constraints of the Bus class and adds a random
variable called atype that can be used to control the address range using another constraint. The
addr_range constraint uses implication to select one of three range constraints that corresponds to the
random value of atype. When a MyBus object is randomized, values for addr, data, and atype are
computed so that all of the constraints are satisfied. Using inheritance to build layered constraint systems
enables the development of general-purpose models that can be constrained to perform application-specific
functions.
Objects can be further constrained using the randomize() with construct (see 18.7), which declares
additional constraints inline with the call to randomize():
task exercise_bus (MyBus bus);
int res;
// EXAMPLE 1: restrict to low addresses
res = bus.randomize() with {atype == low;};
// EXAMPLE 2: restrict to address between 10 and 20
res = bus.randomize() with {10 <= addr && addr <= 20;};
// EXAMPLE 3: restrict data values to powers-of-two
res = bus.randomize() with {(data & (data - 1)) == 0;};
endtask

<!-- Page 525 -->

IEEE Std These examples illustrate several important properties of constraints, which are more thoroughly described
as follows:
— A constraint may be any expression containing operands of integral or real types only.
— Subexpressions of a constraint may include operands that are neither integral nor real if the subex-
pression contains no random variables and the result of the subexpression has an integral or real type
(e.g., an equality operator with both sides having string operands).
— The constraint solver shall be able to handle a wide spectrum of equations, such as algebraic
factoring, complex Boolean expressions, and mixed integer and bit expressions. In the previous
example, the power-of-two constraint was expressed arithmetically. It could have also been defined
with expressions using a shift operator; for example, 1 << n, where n is a 5-bit random variable.
— If a solution exists, the constraint solver shall find it. The solver can fail only when the problem is
over-constrained and there is no combination of random values that satisfy the constraints.
— The set of random values chosen shall satisfy all constraints and distributions (except as specified
otherwise). Thus, in this example, the value chosen for addr depends on atype and how it is con-
strained, and the value chosen for atype depends on addr and how it is constrained.This is true for
all constraint expression operators, including the set membership, distribution, and implication oper-
ators (inside, dist, and ->).
— Constraints support only 2-state values. The 4-state values (x or z) or 4-state operators (e.g., ===,
!== ) are illegal and shall result in an error.
— For each active random variable of enum type,the solver shall select a value from the set of named
constants defined by the corresponding enum.The solver shall not assign a value to a random vari-
able of enum type that lies outside its associated named constant set, even if the value can be suc-
cessfully cast to the enumerated type. Note that state variables ofenumtype may contain values
outside the set of namedenumconstants, which may still allow a valid solution.
Sometimes it is desirable to disable constraints on random variables. For example, to deliberately generate
an illegal address (nonword-aligned):
task exercise_illegal(MyBus bus, int cycles);
int res;
// Disable word alignment constraint.
bus.word_align.constraint_mode(0);
repeat (cycles) begin
// CASE 1: restrict to small addresses.
res = bus.randomize() with {addr[0] || addr[1];};
...
end
// Reenable word alignment constraint
bus.word_align.constraint_mode(1);
endtask
The constraint_mode() method can be used to enable or disable any named constraint block in an
object. In this example, the word-alignment constraint is disabled, and the object is then randomized with
additional constraints forcing the low-order address bits to be nonzero (and thus unaligned).
The ability to enable or disable constraints allows users to design constraint hierarchies. In these hierarchies,
the lowest level constraints can represent physical limits grouped by common properties into named
constraint blocks, which can be independently enabled or disabled.

<!-- Page 526 -->

IEEE Std Similarly, the rand_mode() method can be used to enable or disable any random variable. When a random
variable is disabled, it behaves in exactly the same way as other nonrandom variables.
Occasionally, it is desirable to perform operations immediately before or after randomization. That is
accomplished via two built-in methods, pre_randomize() and post_randomize(), which are
automatically called before and after randomization (see 18.6.2). These methods can be overridden with the
desired functionality:
class XYPair;
rand integer x, y;
endclass
class MyXYPair extends XYPair;
function void pre_randomize();
super.pre_randomize();
$display("Before randomize x=%0d, y=%0d", x, y);
endfunction
function void post_randomize();
super.post_randomize();
$display("After randomize x=%0d, y=%0d", x, y);
endfunction
endclass
By default, pre_randomize() and post_randomize() call their overridden base class methods. When
pre_randomize() or post_randomize() are overridden, care needs to be taken to invoke the base
class’s methods, unless the class is a base class (has no base class). Otherwise, the base class methods will
not be called.
The random stimulus generation capabilities and the object-oriented constraint-based verification
methodology enable users to quickly develop tests that cover complex functionality and better assure design
correctness.
### 18.4 Random variables
Class variables can be declared random using the rand and randc modifier keywords.
The syntax to declare a random variable in a class is as follows in Syntax18-1.
class_property ::= // from A.1.9
{ property_qualifier } data_declaration
| ...
property_qualifier10 ::=
random_qualifier
| class_item_qualifier
random_qualifier10 ::= rand | randc
10) In any one declaration, only one of protected or local is allowed, only one of rand or randc is allowed,
and static and/or virtual can appear only once.
Syntax18-1—Random variable declaration syntax (excerpt from AnnexA)

<!-- Page 527 -->

IEEE Std — The solver can randomize singular variables of any integral or real type.
— Real variables shall not be declared randc.
— Arrays can be declared rand or randc, in which case all of their member elements are treated as
rand or randc.
— Individual array elements can be constrained, in which case the index expression may include
iterative constraint loop variables, constants, and state variables.
— Dynamic arrays, associative arrays and queues can be declared rand or randc. All of the elements
in the array are randomized, overwriting any previous data.
— The size of a dynamic array or queue declared as rand or randc can also be constrained. In that
case, the array shall be resized according to the size constraint, and then all the array elements shall
be randomized. The array size constraint is declared using the size method. For example:
rand bit [7:0] len;
rand integer data[];
constraint db {data.size == len;}
The variable len is declared to be 8 bits wide. The randomizer computes a random value for the len
variable in the 8-bit range of 0 to 255 and then randomizes the first len elements of the data array.
When a dynamic array is resized by randomize(), the resized array is initialized (see 7.5.1) with
the original array. When a queue is resized by randomize(), elements are inserted or deleted (see
7.10.2.2 and 7.10.2.3) at the back (i.e., right side) of the queue as necessary to produce the new
queue size; any new elements inserted take on the default value of the element type. That is, the
resize grows or shrinks the array. This is significant for a dynamic array or queue of class handles.
randomize() does not allocate any class objects. Up to the new size, existing class objects are
retained and their content randomized. If the new size is greater than the original size, each of the
additional elements has a null value requiring no randomization.
In resizing a dynamic array or queue by randomize() or new, the rand_mode of each retained
element is preserved and the rand_mode of each new element is set to active.
The size of a dynamic array or queue may also appear in solve...before (see 18.5.9) and
disable soft (see 18.5.13.2) constraint constructs.
If the size of a dynamic array or queue is not constrained (or is constrained only by discarded soft
constraints, see 18.5.13), then the array shall not be resized and all the array elements shall be ran-
domized.
The size and index values of an associative array are not randomizable.
— An object handle can be declared rand, in which case all of that object’s variables and constraints
are solved concurrently with the variables and constraints of the object that contains the handle.
Randomization shall not modify the actual object handle. Object handles shall not be declared
randc.
— An unpacked structure can be declared rand, in which case all of that structure’s random members
are solved concurrently using one of the rules listed in this subclause. Unpacked structures shall not
be declared randc. A member of an unpacked structure can be made random by having a rand or
randc modifier in the declaration of its type.
For example:
class packet;
typedef struct {
randc int addr = 1 + constant;
int crc;
rand byte data [] = {1,2,3,4};
} header;
rand header h1;

<!-- Page 528 -->

IEEE Std endclass
packet p1 = new;
— Unpacked unions shall not be declared rand or randc.
— Packed tagged unions shall not be declared rand or randc.
— A packed untagged union can be declared rand or randc, in which case that union is treated as an
integral type. Members of packed untagged unions shall not have a rand or randc modifier.
— A packed structure can be declared rand or randc, in which case that structure is treated as an
integral type. Members of packed structures shall not have a rand or randc modifier.
— If a rand variable of packed structure or packed untagged union type has a member of enum type,
the rules in 18.3 restricting the random values of an enum variable shall not apply to that member.
For example:
typedef enum bit [1:0] {A=2'b00, B=2'b11} ab_e;
typedef struct packed {
ab_e ValidAB;
} VStructEnum;
typedef union packed {
ab_e ValidAB;
} VUnionEnum;
When randomizing a variable of type ab_e, the solver can only select a random value of 2'b00 or
2'b11 (A or B, respectively). However, when randomizing a variable of type VStructEnum or
VUnionEnum, the solver can select 2'b00, 2'b01, 2'b10 or 2'b11.
#### 18.4.1 Rand modifier
Variables declared with the rand keyword are standard random variables. Their values are uniformly
distributed over their range. For example:
rand bit [7:0] y;
This is an 8-bit unsigned integer with a range of 0 to 255. If unconstrained, this variable shall be assigned
any value in the range of 0 to 255 with equal probability. In this example, the probability of the same value
repeating on successive calls to randomize() is 1/256.
Likewise, random real values are uniformly distributed over their range. For example:
rand real v;
constraint c {v > 0.0 && v < 2.0;}
The real variable v is constrained to lie within the range 0.0 to 2.0. Hence, the probability of choosing a real
value in the range 0.0 to 1.0 shall be the same as the probability of choosing a real value in the range 1.0 to
2.0.
#### 18.4.2 Randc modifier
Variables declared with the randc keyword are random-cyclic variables that cycle through all the values in
a random permutation of their declared range.
To understand randc, consider a 2-bit random variable y:
randc bit [1:0] y;

<!-- Page 529 -->

IEEE Std The variable y can take on the values 0, 1, 2, and 3 (range of 0 to 3). randomize() computes an initial
random permutation of the range values of y and then returns those values in order on successive calls. After
it returns the last element of a permutation, it repeats the process by computing a new random permutation.
The basic idea is that randc randomly iterates over all the values in the range and that no value is repeated
within an iteration. When the iteration finishes, a new iteration automatically starts (see Figure18-1).
The permutation sequence for any given randc variable is recomputed whenever the constraints change on
that variable or when none of the remaining values in the permutation can satisfy the constraints. The
permutation sequence shall contain only 2-state values.
To reduce memory requirements, implementations may impose a limit on the maximum size of a randc
variable, but it shall be no less than 8 bits.
initial permutation: 0 3 2 1
next permutation: 2 1 3 0
...
next permutation: 2 0 1 3
Figure18-1—Example of randc
The semantics of random-cyclical variables requires that they be solved before other random variables. A set
of constraints that includes both rand and randc variables shall be solved so that the randc variables are
solved first, and this can sometimes cause randomize() to fail.
If a random variable is declared as static, the randc state of the variable shall also be static. Thus
randomize() chooses the next cyclic value (from a single sequence) when the variable is randomized
through any instance of the base class.
### 18.5 Constraint blocks
The values of random variables are determined using constraint expressions that are declared using
constraint blocks. Constraint blocks are class members, like tasks, functions, and variables. Constraint block
names shall be unique within a class.
The syntax to declare a constraint block is as follows in Syntax18-2.
constraint_declaration ::= // from A.1.10
[ static ] constraint [ dynamic_override_specifiers ]11 constraint_identifier constraint_block
constraint_block ::= { { constraint_block_item } }
constraint_block_item ::=
solve solve_before_list before solve_before_list ;
| constraint_expression
solve_before_list ::= constraint_primary { , constraint_primary }
constraint_primary ::= [ implicit_class_handle . | class_scope ] hierarchical_identifier select [ ( ) ]12
constraint_expression ::=
[ soft ] expression_or_dist ;

<!-- Page 530 -->

IEEE Std | uniqueness_constraint ;
| expression –> constraint_set
| if ( expression ) constraint_set [ else constraint_set ]
| foreach ( ps_or_hierarchical_array_identifier [ loop_variables ] ) constraint_set
| disable soft constraint_primary ;
constraint_set ::=
constraint_expression
| { { constraint_expression } }
expression_or_dist ::= expression [ dist { dist_list } ]
dist_list ::= dist_item { , dist_item }
dist_item ::=
value_range [ dist_weight ]
| default :/ expression
dist_weight ::=
:= expression
| :/ expression
constraint_prototype ::=
[ constraint_prototype_qualifier ] [ static ] constraint [ dynamic_override_specifiers ]8,11
constraint_identifier ;
constraint_prototype_qualifier ::= extern | pure
extern_constraint_declaration ::=
[ static ] constraint [ dynamic_override_specifiers ]11 class_scope constraint_identifier
constraint_block
loop_variables ::= [ index_variable_identifier ] { , [ index_variable_identifier ] } // from A.6.8
8) It shall be illegal to use the final_specifier when declaring a pure virtual method or pure constraint.
11) It shall be illegal to use the dynamic_override_specifiers with static constraints.
12) Parentheses are allowed only when the constraint_primary is an array built-in method, such as size().
Syntax18-2—Constraint syntax (excerpt from AnnexA)
The constraint_identifier is the name of the constraint block. This name can be used to enable or disable a
constraint using the constraint_mode() method (see 18.9).
The constraint_block is a list of expression statements that restrict the range of a variable or define relations
between variables. A constraint_expression is any SystemVerilog expression or one of the
constraint-specific operators, dist and -> (see 18.5.3 and 18.5.5, respectively).
The declarative nature of constraints imposes the following restrictions on constraint expressions:
— Functions are allowed with certain limitations (see 18.5.11).
— Operators with side effects, such as ++ and --, are not allowed.
— randc variables may not be specified in ordering constraints (see solve...before in 18.5.9).
— dist expressions may not appear in other expressions.
#### 18.5.1 External constraint blocks
Constraint blocks can be declared outside their enclosing class declaration if a constraint prototype appears
in the enclosing class declaration. A constraint prototype specifies that the class shall have a constraint of the

<!-- Page 531 -->

IEEE Std specified name, but does not specify a constraint block to implement that constraint. A constraint prototype
can take either of two forms, as shown in the following example:
class C;
rand int x;
constraint proto1; // implicit form
extern constraint proto2; // explicit form
endclass
For both forms the constraint can be completed by providing an external constraint block using the class
scope resolution operator, as in the following example:
constraint C::proto1 { x inside {-4, 5, 7}; }
constraint C::proto2 { x >= 0; }
An external constraint block shall appear in the same scope as the corresponding class declaration and shall
appear after the class declaration in that scope. If the explicit form of constraint prototype is used, it shall be
an error if no corresponding external constraint block is provided. If the implicit form of prototype is used
and there is no corresponding external constraint block, the constraint shall be treated as an empty constraint
and a warning may be issued. An empty constraint is one that has no effect on randomization, equivalent to
a constraint block containing the constant expression 1.
For either form, it shall be an error if more than one external constraint block is provided for any given
prototype, and it shall be an error if a constraint block of the same name as a prototype appears in the same
class declaration.
#### 18.5.2 Constraint inheritance
Constraints follow the same general rules for inheritance as other class members. The randomize()
method is virtual and therefore honors constraints of the object on which it was called, regardless of the data
type of the object handle through which the method was called.
A derived class shall inherit all constraints from its superclass. Any constraint in a derived class having the
same name as a constraint in its superclass shall replace the inherited constraint of that name. Any constraint
in a derived class that does not have the same name as a constraint in the superclass shall be an additional
constraint.
If a derived class has a constraint prototype with the same name as a constraint in its superclass, that
constraint prototype shall replace the inherited constraint. Completion of the derived class’s constraint
prototype shall then follow the rules described in 18.5.1.
An abstract class (i.e., a class declared using the syntax virtual class, as described in 8.21) may contain
pure constraints. A pure constraint is syntactically similar to a constraint prototype but uses the pure
keyword, as in the following example:
virtual class D;
pure constraint Test;
endclass
A pure constraint represents an obligation on any non-abstract derived class (i.e., a derived class that is not
virtual) to provide a constraint of the same name. It shall be an error if a non-abstract class does not have
an implementation of every pure constraint that it inherits. It shall be an error to declare a pure constraint in
a non-abstract class.
It shall be an error if a class containing a pure constraint also has a constraint block, constraint prototype or
external constraint block of the same name. However, any class (whether abstract or not) may contain a

<!-- Page 532 -->

IEEE Std constraint block or constraint prototype of the same name as a pure constraint that the class inherits; such a
constraint shall override the pure constraint, and shall be a non-pure constraint for the class and any class
derived from it.
An abstract class that inherits a constraint from its superclass may have a pure constraint of the same name.
In this case, the pure constraint in the derived virtual class shall replace the inherited constraint.
A constraint that overrides a pure constraint may be declared using a constraint block in the body of the
overriding class, or may be declared using a constraint prototype and external constraint as described in
18.5.1.
Like virtual methods in classes (see 8.20), the fact that a subclass constraint replaces a constraint of the same
name in a base class means that it is possible to accidentally override a base class constraint. The initial
specifier, preceded by a colon, when applied to constraints, specifies that the constraint shall not be an
override. Specifying initial when overriding a constraint within a subclass shall result in an error.
Similarly, a subclass may wish to explicitly state that a constraint is replacing a constraint in a base class.
The extends specifier specifies that the constraint shall be an override. Specifying extends when not
replacing a constraint from a base class shall result in an error.
initial and extends are mutually exclusive; specifying both in a constraint declaration shall result in an
error.
A subclass may also wish to prevent any further subclasses in the class hierarchy from replacing a
constraint. The final specifier, when applied to constraints, specifies that no further replacements of the
constraint shall be allowed. An attempt by a subclass to replace a constraint specified as final in a base
class shall result in an error. Additionally, final may be combined with either initial or extends.
When a constraint is declared using a constraint prototype and an external constraint block (see 18.5.1), the
initial, extends, and final specifiers shall be applied to both the constraint prototype and the external
constraint block, or to neither. It shall be an error if one but not the other uses the specifier.
#### 18.5.3 Distribution
In addition to the set membership operator (inside, see 11.4.13), constraints support sets of weighted
values called distributions. When used within a constraint, the set membership operator has a single
property: a relational test for set membership. All values that satisfy the set membership have an equal
probability. On the other hand, distributions have two properties: they are a relational test for set
membership, and they specify a statistical distribution function for the results.
The syntax to define a distribution expression is as follows in Syntax18-3.
constraint_expression ::= // from A.1.10
[ soft ] expression_or_dist ;
| ...
expression_or_dist ::= expression [ dist { dist_list } ]
dist_list ::= dist_item { , dist_item }
dist_item ::=
value_range [ dist_weight ]
| default :/ expression
dist_weight ::=
:= expression

<!-- Page 533 -->

IEEE Std | :/ expression
Syntax18-3—Constraint distribution syntax (excerpt from AnnexA)
The distribution operator dist evaluates to true if the value of its left-hand expression is contained in the
set; otherwise, it evaluates to false.
The distribution set is a comma-separated list of integral or real expressions and ranges. Optionally, each
term in the list can have a weight, which is specified using the := or :/ operator. However, a range of real
values shall use the :/ operator and shall specify a weight. If no weight is specified for an integral item, the
default weight is := 1. The weight can be any integral expression, with the result of the expression being
interpreted as an unsigned value. Absent any other constraints, the probability that the left-hand expression
matches any item in the list shall be proportional to the item’s weight.
Distribution weights can be used to cause selected corner cases to occur more frequently than they would
otherwise. However, nonzero distribution weights do not change the solution space and, therefore, cannot
cause the solver to fail. If there are constraints that cause the distribution weights on some expressions to be
unsatisfiable, implementations are only required to satisfy the constraints. The exceptions to this rule are
values not specified by the distribution and values specified with a total weight of zero, both of which shall
be treated as a constraint that the expression not have such values.
Both the := and :/ operators assign the specified weight to an individual item. For example:
x dist {100:=1, 200:=2, 300:=5};
x dist {100:/1, 200:/2, 300:/5};
In the example, both distributions are equivalent, and result in x being equal to 100, 200, or 300 with a
weight ratio of 1:2:5.
It is often easier to think about ratios, such as 1:2:5, rather than the actual probabilities (12.5%, 25%,
62.5%), because ratios do not have to be normalized to 100%. Converting probabilities to ratios is
straightforward.
If an additional constraint is added that specifies that x cannot be 200,
x != 200;
x dist {100:=1, 200:=2, 300:=5} ;
then x is equal to 100 or 300 with a weight ratio of 1:5.
The := and :/ operators behave differently when they are applied to ranges instead of to individual values.
When the := operator is applied to an integral range, the weight for the range is calculated as if each element
of the range had the specified weight, i.e., range_weight = range_size × specified_weight. Note
that the range_size in this calculation is the count of all values within the range, including values which
may be excluded from the solution space based on other constraints. When the :/ operator is applied to a
range, the weight for the entire range is the specified weight, i.e., range_weight = specified_weight.
For example:
x dist {[100:102]:=1, 103:=1};
x dist {[100:102]:/3, 103:=1};
In the example, both distributions are equivalent, with a result in the range of [100:102] being three times
more likely than a result of 103.

<!-- Page 534 -->

IEEE Std It is important to note that in both cases the range weight applies to the range as a whole. For example:
x > 101;
x dist {[100:102]:=1, 103:=1};
In the example, the weight of the range [100:102] is 3, even though the values 100 and 101 have been
excluded via another constraint. This will result in x being equal to 102 or 103 with a weight ratio of 3:1.
If a single value occurs in multiple items within a single distribution, then the weights allocated to that value
are additive. For example:
x dist {[100:102]:=1, 101:=1};
x dist {[100:101]:/1, [101:102]:/1};
x dist {100:/1, 101:/2, 102:/1};
x dist {100:/1, 101:/1, 101:/1, 102:/1};
Absent any other constraints, the four distributions in the example are equivalent. Each will result in x being
equal to 100, 101, or 102 with a weight ratio of 1:2:1.
This additive behavior of item weights includes items that have been explicitly weighted to zero:
x dist {100:/0, [100:102]:/1};
In the above example, x has an equal probability of being 100, 101, or 102. The 100:/0 weight does not act
as a constraint in this case because the value 100 appears in another item with a nonzero weight applied.
The default specification defines a value range that contains all possible values of the self-determined
type of the distribution expression that are not present in any of the other defined terms. For example:
x dist {[100:102]:/3, default:/1};
means that x can be any value. The range [100:102] has a weight of 3, whereas the range of all remaining
values has a weight of 1.
The default specification shall always use the :/ operator. It shall be an error if the :/ operator is omitted
or if the := operator is used. There shall be at most one default specification in a distribution.
Limitations are as follows:
— A dist operation shall not be applied to randc variables.
— A dist expression requires that the expression contain at least one rand variable.
The following example shows constraints on properties of real variables.
class real_constraint_c;
const int ZSTATE = –100;
const real VALUE_LOW = 0.70;
const real VALUE_MIN = 1.43;
const real VALUE_NOM = 3.30;
const real VALUE_MAX = 3.65;
rand real a;
rand real b;
constraint a_constraint {
a dist { ZSTATE := 5,

<!-- Page 535 -->

IEEE Std [VALUE_LOW:VALUE_MIN] :/ 1,
[VALUE_NOM +%- 1.0] :/ 13, // equivalent to 3.3 +/- 0.033
[VALUE_MIN:VALUE_MAX] :/ 1
};
}
constraint b_constraint {
(a inside [VALUE_LOW:VALUE_MIN])-> b == ZSTATE;
b dist { ZSTATE := 1,
[VALUE_MIN:VALUE_MAX] :/ 20
};
solve a before b;
}
endclass
This example illustrates the following features:
— A distribution may mix real and integral values.
— dist for a range of real values shall use the :/ operator and shall include a weight.
— The := operator may be used with a single real value.
— The inside operator may be used with a real value range.
— The [VALUE_NOM+%-1.0]:/13 statement assigns a weight of 13 to the relative tolerance range
VALUE_NOM+%-1.0.
#### 18.5.4 Uniqueness constraints
A group of variables can be constrained using the unique constraint so that no two members of the group
have the same value after randomization. The group of variables to be constrained shall be specified using a
restricted form of the range_list syntax in which each item in the comma-separated list shall be one of the
following:
— A singular variable of integral or real type
— An unpacked array variable whose leaf element type is integral or real, or a slice of such an
unpacked array variable
constraint_expression ::= // from A.1.10
...
| uniqueness_constraint ;
uniqueness_constraint ::= unique { range_list13 }
13) The range_list in a uniqueness_constraint shall contain only expressions that denote singular or array variables, as
described in 18.5.4.
Syntax18-4—Uniqueness constraint syntax (excerpt from AnnexA)
A leaf element of an unpacked array is found by descending through the array until an element is reached
that is not of unpacked array type.
All members of the group of variables so specified (that is, the singular variables, and the leaf elements of
the arrays and slices) shall be of equivalent type. No randc variable shall appear in the group.
If the group of variables so specified contains fewer than two members, the constraint shall have no effect
and shall not cause a constraint contradiction.

<!-- Page 536 -->

IEEE Std In the following example, variables a[2], a[3], b, and excluded will all contain different values after
randomization. Because of the constraint exclusion, none of the variables a[2], a[3], and b will contain
the value 5.
rand byte a[5];
rand byte b;
rand byte excluded;
constraint u { unique {b, a[2:3], excluded}; }
constraint exclusion { excluded == 5; }
#### 18.5.5 Implication
Constraints provide two constructs for declaring conditional (predicated) relations: implication and
if-else.
The implication operator ( –> ) can be used to declare an expression that implies a constraint.
The syntax to define an implication constraint is as follows in Syntax18-5.
constraint_expression ::= // from A.1.10
...
| expression –> constraint_set
Syntax18-5—Constraint implication syntax (excerpt from AnnexA)
The expression can be any integral or real expression. The constraint_set represents any valid constraint or
an unnamed constraint set.
The Boolean equivalent of the implication operator a -> b is (!a || b). This states that if the expression
is true, then all of the constraints in the constraint set shall also be satisfied. Otherwise, the random values
generated are unconstrained. Conversely, if the constraints in the constraint set are not satisfied, then the
expression shall be false.
For example:
rand enum {little, big, other} mode;
(mode == little) -> len < 10;
(mode == big) -> len > 100;
In this example, the value of mode implies that the value of len shall be constrained to less than 10 (if mode
== little), greater than 100 (if mode == big), or unconstrained (if mode != little and mode !=
big).
As with all constraints, the left-hand and right-hand sides of implication constraints are interdependent.
Thus, in the preceding declaration, just as the value of mode constrains the value of len, the value of len
also constrains the value of mode, since mode is also a random variable.
In the example
rand bit [3:0] a, b;
constraint c { (a == 0) -> (b == 1); }

<!-- Page 537 -->

IEEE Std both a and b are 4 bits; therefore, there are 256 combinations of a and b. Constraint c says that a == 0
implies that b == 1, thereby eliminating 15 combinations: {0,0}, {0,2}, … {0,15}. Therefore, the
probability that a == 0 is thus 1/(256-15) or 1/241.
#### 18.5.6 if–else constraints
The if–else style constraints are also supported.
The syntax to define an if–else constraint is as follows in Syntax18-6.
constraint_expression ::= // from A.1.10
...
| if ( expression ) constraint_set [ else constraint_set ]
Syntax18-6—If–else constraint syntax (excerpt from AnnexA)
The expression can be any integral or real expression.
The constraint_set represents any valid constraint or an unnamed constraint set. If the expression is true, all
of the constraints in the first constraint or constraint set shall be satisfied; otherwise, all of the constraints in
the optional else constraint or constraint set shall be satisfied. Constraint sets can be used to group multiple
constraints.
The if–else style constraint declarations are equivalent to implications
if (mode == little)
len < 10;
else if (mode == big)
len > 100;
which is equivalent to
mode == little -> len < 10 ;
mode == big -> len > 100 ;
In this example, the value of mode implies that the value of len is less than 10, greater than 100, or
unconstrained.
As with implication, the if condition and the constraint set are interdependent and constrain each other. In
the preceding declaration, the value of mode constrains the value of len, and the value of len constrains the
value of mode.
Because the else part of an if–else style constraint declaration is optional, there can be confusion when
an else is omitted from a nested if sequence. This is resolved by always associating the else with the
closest previous if that lacks an else. In the following example, the else goes with the inner if, as shown
by indentation:
if (mode != big)
if (mode == little)
len < 10;
else // the else applies to preceding if
len > 100;

<!-- Page 538 -->

IEEE Std #### 18.5.7 Iterative constraints
Iterative constraints allow arrayed variables to be constrained using loop variables and indexing expressions,
or by using array reduction methods.
18.5.7.1 foreach iterative constraints
The syntax to define a foreach iterative constraint is as follows in Syntax18-7.
constraint_expression ::= // from A.1.10
...
| foreach ( ps_or_hierarchical_array_identifier [ loop_variables ] ) constraint_set
loop_variables ::= [ index_variable_identifier ] { , [ index_variable_identifier ] } // from A.6.8
Syntax18-7—Foreach iterative constraint syntax (excerpt from AnnexA)
The foreach construct specifies iteration over the elements of an array. Its argument is an identifier that
designates any type of packed or unpacked array followed by a comma-separated list of loop variables
enclosed in square brackets. A string variable is treated as a dynamic array of bytes indexed from 0 to N-
1, where N is the number of characters in the string. Each loop variable corresponds to one of the dimensions
of the array. It shall be an error to include a function call as an implicit variable declaration in the identifier
(see 13.4.1).
For example:
class C;
rand byte A[];
constraint C1 { foreach ( A [ i ] ) A[i] inside {2,4,8,16}; }
constraint C2 { foreach ( A [ j ] ) A[j] > 2 * j; }
endclass
C1 constrains each element of the array A to be in the set [2,4,8,16]. C2 constrains each element of the array
A to be greater than twice its index.
The number of loop variables shall not exceed the number of dimensions of the array variable. The scope of
each loop variable is the foreach constraint construct, including its constraint_set. The type of each loop
variable is implicitly declared to be consistent with the type of array index. An empty loop variable indicates
no iteration over that dimension of the array. A list of commas at the end can be omitted; thus,
foreach(arr[ j ]) is a shorthand for foreach(arr[ j, , , , ]). It shall be an error for any loop
variable to have the same identifier as the array.
The mapping of loop variables to array indices is determined by the dimension cardinality, as described in
20.7.
// 1 2 3 3 4 1 2 -> Dimension numbers
int A [2][3][4]; bit [3:0][2:1] B [5:1][4];
foreach( A [ i, j, k ] ) ...
foreach( B [ q, r, , s ] ) ...
The first foreach causes i to iterate from 0 to 1, j from 0 to 2, and k from 0 to 3. The second foreach
causes q to iterate from 5 to 1, r from 0 to 3, and s from 2 to 1.

<!-- Page 539 -->

IEEE Std foreach iterative constraints can include predicates. For example:
class C;
rand int A[] ;
constraint c1 { A.size inside {[1:10]}; }
constraint c2 { foreach ( A[ k ] ) (k < A.size - 1) -> A[k + 1] > A[k]; }
endclass
The first constraint, c1, constrains the size of the array A to be between 1 and 10. The second constraint, c2,
constrains each array value to be greater than the preceding one, i.e., an array sorted in ascending order.
Within a foreach, predicate expressions involving only constants, state variables, object handle
comparisons, loop variables, or the size of the array being iterated behave as guards against the creation of
constraints, and not as logical relations. For example, the implication in constraint c2 above involves only a
loop variable and the size of the array being iterated; thus, it allows the creation of a constraint only when k
< A.size() - 1, which in this case prevents an out-of-bounds access in the constraint. Guards are
described in more detail in 18.5.12.
Index expressions can include loop variables, constants, and state variables. Invalid or out-of-bounds array
indices are not automatically eliminated; users have to explicitly exclude these indices using predicates.
The size method of a dynamic array or queue can be used to constrain the size of the array (see constraint
c1 above). If an array is constrained by both size constraints and iterative constraints, the size constraints are
solved first and the iterative constraints next. As a result of this implicit ordering between size constraints
and iterative constraints, the size method shall be treated as a state variable within the foreach block of
the corresponding array. For example, the expression A.size is treated as a random variable in constraint
c1 and as a state variable in constraint c2. This implicit ordering can cause the solver to fail in some
situations.
18.5.7.2 Array reduction iterative constraints
The array reduction methods can produce a single integral value from an unpacked array of integral values
(see 7.12.3). In the context of a constraint, an array reduction method is treated as an expression iterated over
each element of the array, joined by the relevant operand for each method. The result returns a single value
of the same type as the array element type or, if specified, the type of the expression in the with clause.
As with foreach iterative constraints, if an array is constrained by both size constraints and array reduction
iterative constraints, the size constraints are solved first and the iterative constraints next (see 18.5.7.1).
For example:
class C;
rand bit [7:0] A[];
constraint c1 {A.size == 5;}
constraint c2 {A.sum() with (int'(item)) < 1000;}
endclass
The constraint c2 will be interpreted as
(int'(A[0])+int'(A[1])+int'(A[2])+int'(A[3])+int'(A[4])) < 1000

<!-- Page 540 -->

IEEE Std #### 18.5.8 Global constraints
When an object member of a class is declared rand, all of its constraints and random variables are
randomized simultaneously along with the other class variables and constraints. Constraint expressions
involving random variables from other objects are called global constraints (see Figure18-2).
class A; // leaf node
rand bit [7:0] v;
endclass
class B extends A; // heap node
rand A left;
rand A right;
constraint heapcond {left.v <= v; right.v > v;}
endclass
.v
B
.left .right
.v .v
A A
Figure18-2—Global constraints
This example uses global constraints to define the legal values of an ordered binary tree. Class A represents a
leaf node with an 8-bit value v. Class B extends class A and represents a heap node with value v, a left
subtree, and a right subtree. Both subtrees are declared as rand in order to randomize them at the same time
as other class variables. The constraint block named heapcond has two global constraints relating the left
and right subtree values to the heap node value. When an instance of class B is randomized, the solver
simultaneously solves for B and its left and right children, which in turn can be leaf nodes or more heap
nodes.
The following rules determine which objects, variables, and constraints are to be randomized:
a) First, determine the set of objects that are to be randomized as a whole. Starting with the object that
invoked the randomize() method, add all objects that are contained within it, are declared rand,
and are active (see rand_mode in 18.8). The definition is recursive and includes all of the active
random objects that can be reached from the starting object. The objects selected in this step are
referred to as the active random objects.
b) Second, select all of the active constraints from the set of active random objects. These are the
constraints that are applied to the problem.
c) Third, select all of the active random variables from the set of active random objects. These are the
variables that are to be randomized. All other variable references are treated as state variables,
whose current value is used as a constant.
#### 18.5.9 Variable ordering
The solver shall assure that the random values are selected to give a uniform value distribution over legal
value combinations (that is, all combinations of legal values have the same probability of being the
solution). This important property guarantees that all legal value combinations are equally probable, which
allows randomization to better explore the whole design space. The total value distribution over all legal
value combinations is 100%. Illegal value combinations are ignored when generating the value distribution.

<!-- Page 541 -->

IEEE Std Sometimes, however, it is desirable to force certain combinations to occur more frequently. Consider the
case where a 1-bit control variable s constrains a 32-bit data value d:
class B;
rand bit s;
rand bit [31:0] d;
constraint c {s -> d == 0;}
endclass
The constraint c says “s implies that d equals zero.” Although this reads as if s determines d, in fact s and d
are determined together. There are 1 + 232 legal value combinations of {s,d}, but s is true only for one of
them, {s==1, d==0}. The 232 - 1 illegal value combinations {s==1, d!=0} play no role in determining
the probability of the legal value combinations. Table18-1 lists each of the legal value combinations and the
probability of occurrence of each:
Table18-1—Unordered constraint c legal value probability
s d Probability
1 'h00000000 1/(1 + 232)
0 'h00000000 1/(1 + 232)
0 'h00000001 1/(1 + 232)
0 'h00000002 1/(1 + 232)
0 ...
0 'hfffffffe 1/(1 + 232)
0 'hffffffff 1/(1 + 232)
The constraints provide a mechanism for ordering variables so that s can be chosen independently of d. This
mechanism defines a partial ordering on the evaluation of variables and is specified using the solve
keyword.
class B;
rand bit s;
rand bit [31:0] d;
constraint c { s -> d == 0; }
constraint order { solve s before d; }
endclass
In this case, the order constraint instructs the solver to solve for s before solving for d. The effect is that s is
now chosen 0 or 1 with 50%/50% probability, and then d is chosen subject to the value of s. Adding this
order constraint does not change the set of legal value combinations, but alters their probability of
occurrence, as shown in Table18-2:
Table18-2—Ordered constraint c legal value probability
s d Probability
1 'h00000000 1/2
0 'h00000000 1/2 × 1/232
0 'h00000001 1/2 × 1/232

<!-- Page 542 -->

IEEE Std Table18-2—Ordered constraint c legal value probability
s d Probability
0 'h00000002 1/2 × 1/232
0 ...
0 'hfffffffe 1/2 × 1/232
0 'hffffffff 1/2 × 1/232
Note that the probability of d==0 is 2/(1 + 232), near 0%, without the order constraint, and is
1/2 + (1/2 × 1/232), slightly over 50%, with the order constraint.
Variable ordering can be used to force selected corner cases to occur more frequently than they would
otherwise. However, a “solve...before...” constraint does not change the solution space and, therefore,
cannot cause the solver to fail.
The syntax to define variable order in a constraint block is as follows in Syntax18-8.
constraint_block_item ::= // from A.1.10
solve solve_before_list before solve_before_list ;
| constraint_expression
solve_before_list ::= constraint_primary { , constraint_primary }
constraint_primary ::= [ implicit_class_handle . | class_scope ] hierarchical_identifier select [ ( ) ]12
12) Parentheses are allowed only when the constraint_primary is an array built-in method, such as size().
Syntax18-8—Solve...before constraint ordering syntax (excerpt from AnnexA)
The following restrictions apply to variable ordering:
— Only random variables are allowed, that is, they shall be rand.
— randc variables are not allowed. randc variables are always solved before any other.
— The variables shall be integral or real values.
— array.size (with optional following parentheses) is allowed as a variable (see 18.4).
— A constraint block may contain both regular value constraints and ordering constraints.
— There shall be no circular dependencies in the ordering, such as “solve a before b” combined with
“solve b before a.”
— Variables that are not explicitly ordered shall be solved with the last set of ordered variables. These
values are deferred until as late as possible to assure a good distribution of values.
— Variables that are partially ordered shall be solved with the latest set of ordered variables so that all
ordering constraints are met. These values are deferred until as late as possible to assure a good
distribution of values.
— Variables may be solved in an order that is not consistent with the ordering constraints, provided that
the outcome is the same. An example situation where this might occur is as follows:
x == 0;
x < y;
solve y before x;

<!-- Page 543 -->

IEEE Std In this case, because x has only one possible assignment (0), x can be solved for before y. The
constraint solver can use this flexibility to speed up the solving process.
#### 18.5.10 Static constraint blocks
A constraint block can be defined as static by including the static keyword in its definition.
The syntax to declare a static constraint block is as follows in Syntax18-9.
constraint_declaration ::= // from A.1.10
[ static ] constraint [ dynamic_override_specifiers ]11 constraint_identifier constraint_block
11) It shall be illegal to use the dynamic_override_specifiers with static constraints.
Syntax18-9—Static constraint syntax (excerpt from AnnexA)
If a constraint block is declared as static, then calls to constraint_mode() shall affect all instances of
the specified constraint in all objects. Thus, if a static constraint is set to OFF, it is off for all instances of that
particular class.
When a constraint is declared using a constraint prototype and an external constraint block, the static
keyword shall be applied to both the constraint prototype and the external constraint block, or to neither. It
shall be an error if one but not the other is qualified static. Similarly, a pure constraint may be qualified
static but any overriding constraint shall match the pure constraint’s qualification or absence thereof.
#### 18.5.11 Functions in constraints
Some properties are unwieldy or impossible to express in a single expression. For example, the natural way
to compute the number of ones in a packed array uses a loop:
function int count_ones (bit [9:0] w);
for (count_ones = 0; w != 0; w = w >> 1)
count_ones += w & 1'b1;
endfunction
Such a function could be used to constrain other random variables to the number of 1 bits:
constraint C1 {length == count_ones( v ) ;}
Without the ability to call a function, this constraint requires the loop to be unrolled and expressed as a sum
of the individual bits:
constraint C2
{
length == ((v>>9)&1) + ((v>>8)&1) + ((v>>7)&1) + ((v>>6)&1) + ((v>>5)&1) +
((v>>4)&1) + ((v>>3)&1) + ((v>>2)&1) + ((v>>1)&1) + ((v>>0)&1);
}
Unlike the count_ones function, more complex properties, which require temporary states or unbounded
loops, may be impossible to convert into a single expression. The ability to call functions thus enhances the
expressive power of the constraint language and reduces the likelihood of errors. The two constraints, C1

<!-- Page 544 -->

IEEE Std and C2, from above are not completely equivalent; C2 is bidirectional (length can constrain v and vice
versa), whereas C1 is not.
To handle these common cases, SystemVerilog allows constraint expressions to include function calls, but it
imposes certain semantic restrictions, as follows:
— Functions that appear in constraint expressions shall not contain output, inout, or ref arguments
(const ref is allowed).
— Functions that appear in constraint expressions shall be automatic (or preserve no state information)
and have no side effects.
— Functions that appear in constraints cannot modify the constraints, for example, calling rand_mode
or constraint_mode methods.
— Functions shall be called before constraints are solved, and their return values shall be treated as
state variables.
— Random variables used as function arguments shall establish an implicit variable ordering or
priority. Constraints that include only variables with higher priority are solved before other, lower
priority constraints. Random variables solved as part of a higher priority set of constraints become
state variables to the remaining set of constraints. For example:
class B;
rand int x, y;
constraint C { x <= F(y); }
constraint D { y inside {2, 4, 8}; }
endclass
forces y to be solved before x. Thus, constraint D is solved separately before constraint C, which uses
the values of y and F(y) as state variables. In SystemVerilog, the behavior for variable ordering
implied by function arguments differs from the behavior for ordering specified using the
“solve...before...” constraint; function argument variable ordering subdivides the solution space
thereby changing it. Because constraints on higher priority variables are solved without considering
lower priority constraints at all, this subdivision can cause the overall constraints to fail. Within each
prioritized set of constraints, cyclical (randc) variables are solved first.
— Circular dependencies created by the implicit variable ordering shall result in an error.
— Function calls in active constraints are executed an unspecified number of times (at least once) in an
unspecified order.
#### 18.5.12 Constraint guards
Constraint guards are predicate expressions that function as guards against the creation of constraints and
not as logical relations to be satisfied by the solver. These predicate expressions are evaluated before the
constraints are solved and are characterized by involving only the following items:
— Constants
— State variables
— Object handle comparisons (comparisons between two handles or a handle and the constant null)
In addition to these, iterative constraints (see 18.5.7) also consider loop variables and the size of the array
being iterated as state variables.
Treating these predicate expressions as constraint guards prevents the solver from generating evaluation
errors, thereby failing on some seemingly correct constraints. This enables users to write constraints that
avoid errors due to nonexistent object handles or array indices out of bounds. For example, the sort
constraint of the singly linked list, SList, shown below is intended to assign a random sequence of numbers

<!-- Page 545 -->

IEEE Std that is sorted in ascending order. However, the constraint expression will fail on the last element when
next.n results in an evaluation error due to a nonexistent handle.
class SList;
rand int n;
rand Slist next;
constraint sort { n < next.n; }
endclass
This error condition can be avoided by writing a predicate expression to guard against that condition:
constraint sort { if( next != null ) n < next.n; }
In the preceding sort constraint, the if prevents the creation of a constraint when next == null, which in
this case avoids accessing a nonexistent object. Both implication ( –>) and if…else can be used as guards.
Guard expressions can themselves include subexpressions that result in evaluation errors (e.g., null
references), and they are also guarded from generating errors. This logical sifting is accomplished by
evaluating predicate subexpressions using the following 4-state representation:
— 0 FALSE Subexpression evaluates to FALSE.
— 1 TRUE Subexpression evaluates to TRUE.
— E ERROR Subexpression causes an evaluation error.
— R RANDOM Expression includes random variables and cannot be evaluated.
Every subexpression within a predicate expression is evaluated to yield one of the previous four values. The
subexpressions are evaluated in an arbitrary order, and the result of that evaluation plus the logical operation
define the outcome in the alternate 4-state representation. A conjunction ( && ), disjunction ( || ), or
negation ( ! ) of subexpressions can include some (perhaps all) guard subexpressions. The following rules
specify the resulting value for the guard:
— Conjunction ( && ): If any one of the subexpressions evaluates to FALSE, then the guard evaluates to
FALSE. If any one subexpression evaluates to ERROR, then the guard evaluates to ERROR. Otherwise,
the guard evaluates to TRUE.
• If the guard evaluates to FALSE, then the constraint is eliminated.
• If the guard evaluates to TRUE, then a (possibly conditional) constraint is generated.
• If the guard evaluates to ERROR, then an error is generated and randomize() fails.
— Disjunction ( || ): If any one of the subexpressions evaluates to TRUE, then the guard evaluates to
TRUE. If any one subexpression evaluates to ERROR, then the guard evaluates to ERROR. Otherwise,
the guard evaluates to FALSE.
• If the guard evaluates to FALSE, then a (possibly conditional) constraint is generated.
• If the guard evaluates to TRUE, then an unconditional constraint is generated.
• If the guard evaluates to ERROR, then an error is generated and randomize() fails.
— Negation ( ! ): If the subexpression evaluates to ERROR, then the guard evaluates to ERROR. Other-
wise, if the subexpression evaluates to TRUE or FALSE, then the guard evaluates to FALSE or TRUE,
respectively.
These rules are codified by the truth tables shown in Figure18-3.

<!-- Page 546 -->

IEEE Std && 0 1 E R || 0 1 E R !
0 0 0 0 0 0 0 1 E R 0 1
1 0 1 E R 1 1 1 1 1 1 0
E 0 E E E E E 1 E E E E
R 0 R E R R R 1 E R R R
Conjunction Disjunction Negation
Figure18-3—Truth tables for conjunction, disjunction, and negation rules
These rules are applied recursively until all subexpressions are evaluated. The final value of the evaluated
predicate expression determines the outcome as follows:
— If the result is TRUE, then an unconditional constraint is generated.
— If the result is FALSE, then the constraint is eliminated and can generate no error.
— If the result is ERROR, then an unconditional error is generated and the constraint fails.
— If the final result of the evaluation is RANDOM, then a conditional constraint is generated.
When the final value is RANDOM, a traversal of the predicate expression tree is needed to collect all
conditional guards that evaluate to RANDOM. When the final value is ERROR, a subsequent traversal of the
expression tree is not required, allowing implementations to issue only one error.
Example 1:
class D;
int x;
endclass
class C;
rand int x, y;
D a, b;
constraint c1 { (x < y || a.x > b.x || a.x == 5 ) -> x+y == 10; }
endclass
In Example 1, the predicate subexpressions are (x < y), (a.x > b.x), and (a.x == 5), which are all
connected by disjunction. Some possible cases are as follows:
— Case 1: a is non-null, b is null, a.x is 5.
Because (a.x==5) is true, the fact that b.x generates an error does not result in an error.
The unconditional constraint (x+y == 10) is generated.
— Case 2: a is null.
This always results in an error, irrespective of the other conditions.
— Case 3: a is non-null, b is non-null, a.x is 10, b.x is 20.
All the guard subexpressions evaluate to FALSE.
The conditional constraint (x<y) -> (x+y == 10) is generated.
Example 2:
class D;
int x;
endclass

<!-- Page 547 -->

IEEE Std class C;
rand int x, y;
D a, b;
constraint c1 { (x < y && a.x > b.x && a.x == 5 ) -> x+y == 10; }
endclass
In Example 2, the predicate subexpressions are (x < y), (a.x > b.x), and (a.x == 5), which are all
connected by conjunction. Some possible cases are as follows:
— Case 1: a is non-null, b is null, a.x is 6.
Because (a.x==5) is false, the fact that b.x generates an error does not result in an error.
The constraint is eliminated.
— Case 2: a is null
This always results in an error, irrespective of the other conditions.
— Case 3: a is non-null, b is non-null, a.x is 5, b.x is 2.
All the guard subexpressions evaluate to TRUE, producing constraint (x<y) -> (x+y == 10).
Example 3:
class D;
int x;
endclass
class C;
rand int x, y;
D a, b;
constraint c1 { (x < y && (a.x > b.x || a.x ==5)) -> x+y == 10; }
endclass
In Example 3, the predicate subexpressions are (x < y) and (a.x > b.x || a.x == 5), which are
connected by disjunction. Some possible cases are as follows:
— Case 1: a is non-null, b is null, a.x is 5.
The guard expression evaluates to (ERROR || a.x==5), which evaluates to (ERROR || TRUE)
The guard subexpression evaluates to TRUE.
The conditional constraint (x<y) -> (x+y == 10) is generated.
— Case 2: a is non-null, b is null, a.x is 8.
The guard expression evaluates to (ERROR || FALSE) and generates an error.
— Case 3: a is null
This always results in an error, irrespective of the other conditions.
— Case 4: a is non-null, b is non-null, a.x is 5, b.x is 2.
All the guard subexpressions evaluate to TRUE.
The conditional constraint (x<y) -> (x+y == 10) is generated.
#### 18.5.13 Soft constraints
The constraints described up to this point can be denoted as hard constraints because the solver shall always
satisfy them or result in a solver failure. A soft constraint is an expression_or_dist constraint expression
preceded by soft (see Syntax18-2). When there is no solution that satisfies all active hard constraints (if
any) simultaneously with a constraint defined as soft, the solver shall discard that soft constraint and find a
solution that satisfies the remaining constraints. If there are two or more soft constraints that cannot be
satisfied simultaneously, one or more of the soft constraints shall be discarded according to the priorities
specified in 18.5.13.1.

<!-- Page 548 -->

IEEE Std A discarded soft constraint is treated as if replaced by the value 1 (true). The discarded constraint is not
required to evaluate to true and has no effect on the solution distribution.
Soft constraints enable authors of generic verification blocks to provide complete working environments
that are more easily extended because the constraint solver automatically disregards generic soft constraints
overridden by subsequent, more specialized constraints. Soft constraints are often used to specify default
values and distributions for random variables. For example, the author of a generic packet class might add a
constraint to ensure packets of legal size are generated by default (absent any other constraints):
class Packet;
rand bit mode;
rand int length;
constraint deflt {
soft length inside {32,1024};
soft mode -> length == 1024;
// Note: soft mode -> {length == 1024;} is not legal syntax,
// as soft needs to be followed by an expression
}
endclass
Packet p = new();
p.randomize() with { length == 1512;} // mode will randomize to 0
p.randomize() with { length == 1512; mode == 1;} // mode will randomize to 1
If the constraint expression length inside {32,1024} is not defined as soft, the call to randomize()
will fail and require special attention. The failure might be resolved by explicitly turning off the constraint,
which requires additional procedural code, or by using a new class to extend the base class and override the
constraint with a new one, which significantly complicates the test. In contrast, a default value specified by a
soft constraint is automatically overridden, which results in a simpler layered test.
18.5.13.1 Soft constraint priorities
Soft constraints only express a preference for one solution over another; they are discarded when they are
contradicted by other more important constraints. Regular (hard) constraints shall always be satisfied; they
are never discarded and are thus considered to be of the same highest priority. Conversely, soft constraints
may be overridden by hard constraints or other higher-priority soft constraints, therefore, a specific priority
shall be associated with every soft constraint. The soft priorities are designed such that the last constraint
specified by the user will prevail. Hence, constraints specified in subsequent layers of the verification
environment are assigned higher priorities than those in the preceding layers.
The following rules determine the priorities of soft constraints:
— Constraints within the scope of the same construct—constraint block, class, or struct—are assigned
a priority relative to their syntactic declaration order. Constraint expressions that appear later in the
construct have higher priority.
— Constraints within external constraint blocks are assigned a priority relative to the declaration order
of the constraint prototype (extern declaration) in the class. The priority depends on the prototype
declaration and not on the out-of-body declaration. Constraint expressions in out-of-body constraint
blocks whose prototypes appear later in the class have higher priority.
— Constraints in contained objects (rand class handles) have lower priority than all constraints in the
container object (class or struct).
— Constraints in each contained object (rand class handle) are assigned a priority relative to the
declaration order of their class handle. Constraints in objects whose handles appear later in the
container object (class or struct) have higher priority. If the same object is contained multiple

<!-- Page 549 -->

IEEE Std times, the constraints in the contained object shall have the priority of the highest priority object—
the object whose handle is declared last.
— Constraints in a derived class shall have higher priority than all constraints in its superclasses.
— Constraints within inline constraint blocks shall have higher priority than constraints in the class
being randomized.
— Soft constraints within a foreach shall have a priority that is defined by iteration order; latter
iterations shall have higher priority. If the relational operator is not defined for an index type of an
associative array, the priority is implementation dependent.
The following example illustrates the preceding rules:
class B1;
rand int x;
constraint a { soft x > 10 ; soft x < 100 ; }
endclass /* a1 */ /* a2 */
class D1 extends B1;
constraint b { soft x inside {[5:9]} ; }
endclass /* b1 */
class B2;
rand int y;
constraint c { soft y > 10 ; }
endclass /* c1 */
class D2 extends B2;
constraint d { soft y inside {[5:9]} ; }
constraint e ; /* d1 */
rand D1 p1;
rand B1 p2;
rand D1 p3;
constraint f { soft p1.x < p2.x ; }
endclass /* f1 */
constraint D2::e { soft y > 100 ; }
/* e1 */
D2 d = new();
initial begin
d.randomize() with { soft y inside {10,20,30} ; soft y < p1.x ; };
end /* i1 */ /* i2 */
The relative priority of the soft constraints (highest to lowest) in the preceding example is:
i2 i1 f1 e1 d1 c1 p3.b1 p3.a2 p3.a1 p2.a2 p2.a1 p1.b1 p1.a2 p1.a1
If handles p1 and p3 refer to the same object, the relative priority is:
i2 i1 f1 e1 d1 c1 p3.b1 p3.a2 p3.a1 p2.a2 p2.a1
Soft constraints can only be specified on random variables; they may not be specified for randc variables.
The constraint solver implements the following conceptual model:
— Consider two soft constraints c1 and c2, such that c1 has higher priority than c2.
1) The constraint solver will first try to produce a solution satisfying both c1 and c2.

<!-- Page 550 -->

IEEE Std 2) If it fails in (1) then it will try to produce a solution satisfying only c1.
3) If it fails in (2) then it will try to produce a solution satisfying only c2.
4) If it fails in (3) then it will discard both c1 and c2.
— The constraint solver shall satisfy the following properties:
• If a call to randomize() only involves soft constraints, the call can never fail.
• If the soft constraints do not exhibit any contradictions, then the result is the same as if all
constraints were declared hard.
18.5.13.2 Disabling soft constraints
A disable soft constraint on a random variable (including array.size, see 18.4) specifies that all lower
priority soft constraints that reference the given random variable shall be discarded. For example:
class A;
rand int x;
constraint A1 {soft x == 3;}
constraint A2 {disable soft x;} // discard soft constraints
constraint A3 {soft x inside {1, 2};}
endclass
initial begin
A a = new();
a.randomize();
end
The constraint A2 instructs the solver to discard all soft constraints of lower priority on random variable x,
resulting in constraint A1 being discarded. Thus, the solver only has to satisfy the last constraint A3. As a
result the previous example will result in random variable x taking on the values 1 and 2. Note that if the
constraint A3 was omitted, then the variable would be unconstrained.
A disable soft constraint only disables soft constraints in which the referenced variable directly appears.
The following example clarifies this point:
class C {
bit p;
bit q;
constraint c_1 { p -> soft q; }
constraint c_2 { soft !p || q; }
constraint c_3 {
disable soft p; // disables soft constraint in c_2,
// but not constraint in c_1
disable soft q; // disables soft constraints in both c_1 and c_2,
// and then c_1 becomes equivalent to p -> 1;
}
}
A disable soft constraint causes lower priority soft constraints to be discarded regardless of whether
those constraints create a contradiction. This feature is very useful to extend the solution space beyond the
default values specified by any preceding soft constraints. The following example illustrates this behavior:
class B;
rand int x;
constraint B1 { soft x == 5; }
constraint B2 { disable soft x; soft x dist {5, 8}; }
endclass
initial begin

<!-- Page 551 -->

IEEE Std B b = new();
b.randomize();
end
In this example, the disable soft constraint preceding the soft distribution in block B2 causes the lower
priority constraint on variable x in block B1 to be discarded. Hence, the solver will assign to x the values
5and 8 with equal distribution—the result of solving constraint: x dist {5,8}.
Contrast the behavior of the previous example when the disable soft constraint is omitted:
class B;
rand int x;
constraint B1 { soft x == 5; }
constraint B3 { soft x dist {5, 8}; }
endclass
initial begin
B b = new();
b.randomize();
end
In this preceding, the soft distribution constraint in block B3 can be satisfied for the value of 5. Hence, the
solver will assign x the value 5. In general, if it is desired for the distribution weights of a soft dist
constraint to be satisfied regardless of the presence of lower priority soft constraints then those soft
constraints should be discarded first.
### 18.6 Randomization methods
#### 18.6.1 Randomize()
Variables in an object are randomized using the randomize() class method. Every class has a built-in
randomize() virtual method, declared as follows:
virtual function int randomize();
The randomize() method is a virtual function that generates random values for all the active random
variables in the object, subject to the active constraints.
The randomize() method returns 1 if it successfully sets all the random variables and objects to valid
values; otherwise, it returns 0.
Example:
class SimpleSum;
rand bit [7:0] x, y, z;
constraint c {z == x + y;}
endclass
This class definition declares three random variables, x, y, and z. Calling the randomize() method shall
randomize an instance of class SimpleSum:
SimpleSum p = new;
int success = p.randomize();
if (success == 1 ) ...

<!-- Page 552 -->

IEEE Std Checking the return status can be necessary because the actual value of state variables or addition of
constraints in derived classes can render seemingly simple constraints unsatisfiable.
The use of randomize() with an argument list is discussed in 18.11.
#### 18.6.2 Pre_randomize() and post_randomize()
Every class contains pre_randomize() and post_randomize() methods, which are automatically called
by randomize() before and after computing new random values.
The prototype for the pre_randomize() method is as follows:
function void pre_randomize();
The prototype for the post_randomize() method is as follows:
function void post_randomize();
When obj.randomize() is invoked, it first invokes pre_randomize() on obj and also all of its random
object members that are enabled. After the new random values are computed and assigned, randomize()
invokes post_randomize() on obj and also all of its random object members that are enabled.
Users can override the pre_randomize() in any class to perform initialization and set preconditions
before the object is randomized. If the class is a derived class and no user-defined implementation
of pre_randomize() exists, then pre_randomize() will automatically invoke
super.pre_randomize().
Users can override the post_randomize() in any class to perform cleanup, print diagnostics, and check
post-conditions after the object is randomized. If the class is a derived class and no user-defined
implementation of post_randomize() exists, then post_randomize() will automatically invoke
super.post_randomize().
If these methods are overridden, they shall call their associated base class methods; otherwise, their pre- and
post-randomization processing steps shall be skipped.
The pre_randomize() and post_randomize() methods are not virtual. However, because they are
automatically called by the randomize() method, which is virtual, they appear to behave as virtual
methods.
#### 18.6.3 Behavior of randomization methods
— Random variables declared as static are shared by all instances of the class in which they are
declared. Each time the randomize() method is called, the variable is changed in every class
instance.
— If randomize() fails, the constraints are infeasible, and the random variables retain their previous
values.
— If randomize() fails, post_randomize() is not called.
— The randomize() method is built-in and cannot be overridden.
— The randomize() method implements object random stability. An object’s RNG can be seeded by
calling its srandom() method (see 18.13.3).
— The built-in methods pre_randomize() and post_randomize() are functions and cannot block.

<!-- Page 553 -->

IEEE Std ### 18.7 Inline constraints—randomize() with
By using the randomize()with construct, users can declare inline constraints at the point where the
randomize() method is called. These additional constraints are applied along with the object constraints.
The syntax for randomize() with is as follows in Syntax18-10.
inline_constraint_declaration ::= // not in AnnexA
class_variable_identifier . randomize [ ( [ variable_identifier_list | null ] ) ]
with [ ( [ identifier_list ] ) ] constraint_block
randomize_call ::= // from A.8.2
randomize { attribute_instance }
[ ( [ variable_identifier_list | null ] ) ]
[ with [ ( [ identifier_list ] ) ] constraint_block ]43
variable_identifier_list ::= variable_identifier { , variable_identifier }
identifier_list ::= identifier { , identifier }
43) In a randomize_call that is not a method call of an object of class type (i.e., a scope randomize), the optional
parenthesized identifier_list after the keyword with shall be illegal, and the use of null shall be illegal.
Syntax18-10—Inline constraint syntax (excerpt from AnnexA)
The class_variable_identifier is the name of an instantiated object.
The unnamed constraint_block contains additional inline constraints to be applied along with the object
constraints declared in the class.
For example:
class SimpleSum;
rand bit [7:0] x, y, z;
constraint c {z == x + y;}
endclass
task InlineConstraintDemo(SimpleSum p);
int success;
success = p.randomize() with {x < y;};
endtask
This is the same example used before; however, randomize()with is used to introduce an additional
constraint that x < y.
The randomize()with construct can be used anywhere an expression can appear. The constraint block
following with can define all of the same constraint types and forms as would otherwise be declared in a
class.
The randomize()with constraint block can also reference local variables and subroutine arguments,
eliminating the need for mirroring a local state as member variables in the object class. When the constraint
block is not preceded by the optional parenthesized identifier_list, the constraint block is considered to be
unrestricted. The scope for resolution of variable names referenced in an unrestricted constraint block
begins with the randomize()with object class; that is, the class of the object handle used in the method
call to randomize(). Then, if a name fails to resolve within the randomize()with object class, the name

<!-- Page 554 -->

IEEE Std is resolved normally starting in the scope containing the inline constraint. Names qualified by this or
super shall bind to the class of the object handle used in the call to the randomize()with method. Hence,
it shall be an error if the qualified name fails to resolve within the randomize()with object class. Dotted
names other than those qualified by this or super shall first be resolved in a downwards manner (see 23.3)
starting in the scope of the randomize() with object class. If the dotted name does not resolve in the
scope of the randomize() with object class, it shall be resolved following normal resolution rules in the
scope containing the inline constraint.
The local:: qualifier (see 18.7.1) is used to bypass the scope of the (randomize() with object) class and
begin the name resolution procedure in the (local) scope that contains the randomize() method call.
When the constraint_block is preceded by the optional parenthesized identifier_list, the constraint block is
considered to be restricted. In a restricted constraint block, only variables whose name resolution begins
with identifiers in the identifier_list shall resolve into the randomize()with object class; all other names
shall resolve starting in the scope containing the randomize() method call. When the parenthesized
identifier_list is present and the local:: qualifier is used, the qualified name shall resolve starting in the
scope containing the randomize() method call independent of whether the name is present in the
identifier_list.
In the following example, the randomize()with class is C1.
class C1;
rand integer x;
endclass
class C2;
integer x;
integer y;
task doit(C1 f, integer x, integer z);
int result;
result = f.randomize() with {x < y + z;};
endtask
endclass
In the f.randomize() with constraint block, x is a member of class C1 and hides the x in class C2. It also
hides the x argument in the doit() task. y is a member of C2.z is a local argument.
A restricted constraint block can be used to guarantee that local variable references will resolve into a local
scope.
class C;
rand integer x;
endclass
function int F(C obj, integer y);
F = obj.randomize() with (x) { x < y; };
endfunction
In this example, only x is resolved into the object obj since only x is listed in the identifier_list. The
reference to y will never bind into obj even if a later change adds a property named y into class C.
#### 18.7.1 local:: scope resolution
The randomize()with constraint block can reference both class properties and variables local to the
method call. Unqualified names in an unrestricted inlined constraint block are then resolved by searching

<!-- Page 555 -->

IEEE Std first in the scope of the randomize()with object class followed by a search of the scope containing the
method call—the local scope. The local:: qualifier modifies the resolution search order. When applied to
an identifier within an inline constraint, the local:: qualifier bypasses the scope of the
[randomize()with object] class and resolves the identifier in the local scope.
In the following example, the randomize()with class is C, and the local scope is the function F():
class C;
rand integer x;
endclass
function int F(C obj, integer x);
F = obj.randomize() with { x < local::x; };
endfunction
In the unrestricted inline constraint block of the obj.randomize() call, the unqualified name, x, binds to
the property of class C (the scope of the object being randomized) while the qualified name local::x binds
to the argument of the function F() (the local scope).
As a result of the preceding rules, the following apply:
— Names qualified only by this or super shall bind to the class of the object handle used in the
randomize()with method call.
— Names qualified by local:: shall bind to the scope containing the randomize() method call,
including the special names this or super (i.e., local::this).
— The local:: prefix may be used to qualify class scopes and type names.
— As it pertains to wildcard package imports, the syntactic form local::a shall be semantically
identical to the unqualified name a declared in the local scope.
— Given a method call obj.randomize()with, the name local::obj shall bind to the scope of the
randomize()with object class.
### 18.8 Disabling random variables with rand_mode()
The rand_mode() method can be used to control whether a random variable is active or inactive. When a
random variable is inactive, it is treated the same as if it had not been declared rand or randc. Inactive
variables are not randomized by the randomize() method, and their values are treated as state variables by
the solver. All random variables are initially active.
The syntax for the rand_mode() method is as follows:
function void object[.random_variable].rand_mode(bit on_off);
or
function int object.random_variable.rand_mode();
The object is any expression that yields the object handle in which the random variable is defined.
The random_variable is the name of the random variable to which the operation is applied. If it is not
specified (only allowed when called as a void function), the action is applied to all random variables within
the specified object.

<!-- Page 556 -->

IEEE Std When rand_mode() is called as a void function, the argument to the method determines the operation to be
performed as shown in Table18-3.
Table18-3—rand_mode argument
Value Meaning Description
0 OFF Sets the specified variables to inactive so that they are not randomized
on subsequent calls to the randomize() method.
1 ON Sets the specified variables to active so that they are randomized on
subsequent calls to the randomize() method.
For unpacked array variables, random_variable can specify individual elements using the corresponding
index. Omitting the index results in all the elements of the array being affected by the call.
For unpacked structure variables, random_variable can specify individual members using the
corresponding member. Omitting the member results in all the members of the structure being affected by
the call.
If the random variable is an object handle, only the mode of the variable is changed, not the mode of random
variables within that object (see global constraints in 18.5.8).
A compiler error shall be issued if the specified variable does not exist within the class hierarchy or it exists
but is not declared as rand or randc.
When called as a nonvoid function (i.e., without an argument), rand_mode() returns the current active state
of the specified random variable. It returns 1 if the variable is active (ON) and 0 if the variable is inactive
(OFF). This form of rand_mode() only accepts singular variables; thus, if the specified variable is an
unpacked array, a single element shall be selected via its index.
Example:
class Packet;
rand integer source_value, dest_value;
... other declarations
endclass
int ret;
Packet packet_a = new;
// Turn off all variables in object
packet_a.rand_mode(0);
// ... other code
// Enable source_value
packet_a.source_value.rand_mode(1);
ret = packet_a.dest_value.rand_mode();
This example first disables all random variables in the object packet_a and then enables only the
source_value variable. Finally, it sets the ret variable to the active status of variable dest_value.
The rand_mode() method is built-in and cannot be overridden.

<!-- Page 557 -->

IEEE Std If a random variable is declared as static, the rand_mode state of the variable shall also be static. For
example, if rand_mode() is set to inactive, the random variable is inactive in all instances of the base class.
### 18.9 Controlling constraints with constraint_mode()
The constraint_mode() method can be used to control whether a constraint is active or inactive. When a
constraint is inactive, it is not considered by the randomize() method. All constraints are initially active.
The syntax for the constraint_mode() method is as follows:
function void object[.constraint_identifier].constraint_mode(bit on_off);
or
function int object.constraint_identifier.constraint_mode();
The object is any expression that yields the object handle in which the constraint is defined.
The constraint_identifier is the name of the constraint block to which the operation is applied. The
constraint name can be the name of any constraint block in the class hierarchy. If no constraint name is
specified (only allowed when called as a void function), the operation is applied to all constraints within the
specified object.
When constraint_mode()is called as a void function, the argument to the method determines the
operation to be performed as shown in Table18-4.
Table18-4—constraint_mode argument
Value Meaning Description
0 OFF Sets the specified constraint block to inactive so that it is not
enforced by subsequent calls to the randomize() method.
1 ON Sets the specified constraint block to active so that it is considered
on subsequent calls to the randomize() method.
A compiler error shall be issued if the specified constraint block does not exist within the class hierarchy.
When called as a nonvoid function (i.e., without an argument), constraint_mode() returns the current
active state of the specified constraint block. It returns 1 if the constraint is active (ON) and 0 if the constraint
is inactive (OFF).
Example:
class Packet;
rand integer source_value;
constraint filter1 {source_value > 2 * m;}
endclass
function integer toggle_rand(Packet p);
if (p.filter1.constraint_mode())
p.filter1.constraint_mode(0);
else
p.filter1.constraint_mode(1);

<!-- Page 558 -->

IEEE Std toggle_rand = p.randomize();
endfunction
In this example, the toggle_rand function first checks the current active state of the constraint filter1 in
the specified Packet object p. If the constraint is active, the function deactivates it; if it is inactive, the
function activates it. Finally, the function calls the randomize() method to generate a new random value
for variable source_value.
The constraint_mode() method is built-in and cannot be overridden.
### 18.10 Dynamic constraint modification
There are several ways to dynamically modify randomization constraints, as follows:
— Implication and if–else style constraints allow declaration of predicated constraints.
— Constraint blocks can be made active or inactive using the constraint_mode() built-in method.
Initially, all constraint blocks are active. Inactive constraints are ignored by the randomize()
function.
— Random variables can be made active or inactive using the rand_mode() built-in method. Initially,
all rand and randc variables are active. Inactive variables are not randomized by the
randomize() method, and their values are treated as state variables by the solver.
— The weights in a dist constraint can be changed, affecting the probability that particular values in
the set are chosen.
### 18.11 Inline random variable control
The randomize() method can be used to temporarily control the set of random and state variables within a
class instance or object. When the randomize() method is called with no arguments, it behaves as
described in the previous subclauses, that is, it assigns new values to all random variables in an object—
those declared as rand or randc—so that all of the constraints are satisfied. When randomize() is called
with arguments, those arguments designate the complete set of random variables within that object; all other
variables in the object are considered state variables. For example, consider the following class and calls to
randomize():
class CA;
rand byte x, y;
byte v, w;
constraint c1 { x < v && y > w );
endclass
CA a = new;
a.randomize(); // random variables: x, y state variables: v, w
a.randomize( x ); // random variables: x state variables: y, v, w
a.randomize( v, w ); // random variables: v, w state variables: x, y
a.randomize( w, x ); // random variables: w, x state variables: y, v
This mechanism controls the set of active random variables for the duration of the call to randomize(),
which is conceptually equivalent to making a set of calls to the rand_mode() method to disable or enable
the corresponding random variables. Calling randomize() with arguments allows changing the random
mode of any class property, even those not declared as rand or randc. This mechanism, however, does not
affect the cyclical random mode; it cannot change a nonrandom variable into a cyclical random variable

<!-- Page 559 -->

IEEE Std (randc) and cannot change a cyclical random variable into a noncyclical random variable (change from
randc to rand).
The scope of the arguments to the randomize() method is the object class. Arguments are limited to the
names of properties of the calling object; expressions are not allowed. The random mode of local class
members can only be changed when the call to randomize() has access to those properties, that is, within
the scope of the class in which the local members are declared.
#### 18.11.1 Inline constraint checker
Normally, calling the randomize() method of a class that has no random variables causes the method to
behave as a checker. In other words, it assigns no random values and only returns a status: 1 if all constraints
are satisfied and 0 otherwise. The inline random variable control mechanism can also be used to force the
randomize() method to behave as a checker.
The randomize() method accepts the special argument null to indicate no random variables for the
duration of the call. In other words, all class members behave as state variables. This causes the
randomize() method to behave as a checker instead of a generator. A checker evaluates all constraints and
simply returns 1 if all constraints are satisfied and 0 otherwise. For example, if class CA defined previously
executes the following call:
success = a.randomize( null ); // no random variables
then the solver considers all variables as state variables and only checks whether the constraint is satisfied,
namely, that the relation (x < v && y > w) is true using the current values of x, y, v, and w.
### 18.12 Randomization of scope variables—std::randomize()
The built-in class randomize() method operates exclusively on class member variables. Using classes to
model the data to be randomized is a powerful mechanism that enables the creation of generic, reusable
objects containing random variables and constraints that can be later extended, inherited, constrained,
overridden, enabled, disabled, and merged with or separated from other objects. The ease with which classes
and their associated random variables and constraints can be manipulated makes classes an ideal vehicle for
describing and manipulating random data and constraints. However, some less-demanding problems that do
not require the full flexibility of classes can use a simpler mechanism to randomize data that do not belong to
a class. The scope randomize function, std::randomize(), enables users to randomize data in the current
scope without the need to define a class or instantiate a class object.
The syntax of the scope randomize function is as follows in Syntax18-11.
scope_randomize ::=
[ std :: ] randomize ( [ variable_identifier_list ] ) [ with constraint_block ]
Syntax18-11—Scope randomize function syntax (not in AnnexA)
The scope randomize function behaves exactly the same as a class randomize method, except that it operates
on the variables of the current scope instead of class member variables. Arguments to this function specify
the variables that are to be assigned random values, i.e., the random variables.
For example:
module stim;

<!-- Page 560 -->

IEEE Std bit [15:0] addr;
bit [31:0] data;
function bit gen_stim();
bit success, rd_wr;
success = randomize( addr, data, rd_wr ); // call std::randomize
return rd_wr ;
endfunction
...
endmodule
The function gen_stim calls std::randomize() with three variables as arguments: addr, data, and
rd_wr. Thus, std::randomize() assigns new random variables to the variables that are visible in the
scope of the gen_stim function. In the preceding example, addr and data have module scope, whereas
rd_wr has scope local to the function. The preceding example can also be written using a class:
class stimc;
rand bit [15:0] addr;
rand bit [31:0] data;
rand bit rd_wr;
endclass
function bit gen_stim( stimc p );
bit [15:0] addr;
bit [31:0] data;
bit success;
success = p.randomize();
addr = p.addr;
data = p.data;
return p.rd_wr;
endfunction
However, for this simple application, the scope randomize function leads to a straightforward
implementation.
The scope randomize function returns 1 if it successfully sets all the random variables to valid values;
otherwise, it returns 0. If the scope randomize function is called with no argument, it shall not change the
value of any variable but instead it shall check its constraints. All constraint expressions in its
constraint_block shall be evaluated, and if one or more of those expressions evaluates to false (0) then the
randomize call shall return 0; otherwise it shall return 1.
#### 18.12.1 Adding constraints to scope variables—std::randomize() with
The std::randomize() with form of the scope randomize function allows users to specify random
constraints to be applied to the local scope variables. When specifying constraints, the arguments to the
scope randomize function become random variables; all other variables are considered state variables.
task stimulus( int length );
int a, b, c, success;
success = std::randomize( a, b, c ) with { a < b ; a + b < length ; };
...
success = std::randomize( a, b ) with { b - a > length ; };
...
endtask

<!-- Page 561 -->

IEEE Std The preceding task stimulus calls std::randomize twice resulting in two sets of random values for its
local variables a, b, and c. In the first call, variables a and b are constrained so that variable a is less than b
and their sum is less than the task argument length, which is designated as a state variable. In the second call,
variables a and b are constrained so that their difference is greater than the state variable length.
### 18.13 Random number system functions and methods
#### 18.13.1 $urandom
The system function $urandom provides a mechanism for generating pseudo-random numbers. The
function returns a new 32-bit random number each time it is called. The number shall be unsigned.
The syntax for $urandom is as follows:
function int unsigned $urandom [ (int seed ) ] ;
The seed is an optional argument that determines the sequence of random numbers generated. The seed can
be any integral expression. The random number generator (RNG) shall generate the same sequence of
random numbers every time the same seed is used.
The RNG is deterministic. Each time the program executes, it cycles through the same random sequence.
This sequence can be made nondeterministic by seeding the $urandom function with an extrinsic random
variable, such as the time of day.
For example:
bit [64:1] addr;
bit [ 3:0] number;
addr[32:1] = $urandom( 254 ); // Initialize the generator,
// get 32-bit random number
addr = {$urandom, $urandom }; // 64-bit random number
number = $urandom & 15; // 4-bit random number
#### 18.13.2 $urandom_range()
The $urandom_range() function returns an unsigned integer within a specified range.
The syntax for $urandom_range() is as follows:
function int unsigned $urandom_range( int unsigned maxval,
int unsigned minval = 0 );
The function shall return an unsigned integer in the range of maxval ... minval.
Example 1:
val = $urandom_range(7,0);
If minval is omitted, the function shall return a value in the range of maxval ... 0.
Example 2:
val = $urandom_range(7);

<!-- Page 562 -->

IEEE Std If maxval is less than minval, the arguments are automatically reversed so that the first argument is larger
than the second argument.
Example 3:
val = $urandom_range(0,7);
All of the three previous examples produce a value in the range of 0 to 7, inclusive.
$urandom_range() is automatically thread stable (see 18.14.2).
#### 18.13.3 srandom()
The srandom() method seeds an object’s RNG with the value of the given seed (see 18.14 and 18.15). The
RNG associated with a process can be seeded using the srandom() method of the process (see 9.7).
The prototype of the srandom() method is as follows:
function void srandom(int seed);
#### 18.13.4 get_randstate()
The get_randstate() method retrieves the current internal state of an object’s RNG (see 18.14 and
18.15). The state of the RNG associated with a process is retrieved using the get_randstate() method of
the process (see 9.7).
The prototype of the get_randstate() method is as follows:
function string get_randstate();
The RNG state is a string of unspecified length and format. The length and contents of the string are
implementation dependent.
#### 18.13.5 set_randstate()
The set_randstate() method sets the internal state of an object’s RNG with the given value (see 18.14
and 18.15). The state of the RNG associated with a process is set using the set_randstate() method of
the process (see 9.7).
The prototype of the set_randstate() method is as follows:
function void set_randstate(string state);
The RNG state is a string of unspecified length and format. The length and contents of the string are
implementation dependent. Calling set_randstate() with a string value that was not obtained from
get_randstate(), or from a different implementation of get_randstate(), is undefined.
### 18.14 Random stability
The RNG is localized to threads and objects. Because the sequence of random values returned by a thread or
object is independent of the RNG in other threads or objects, this property is called random stability.
Random stability applies to the following:
— The system randomization calls, $urandom() and $urandom_range()

<!-- Page 563 -->

IEEE Std — The shuffle() array manipulation method
— The procedural randcase and randsequence statements
— The object and process random seeding method, srandom()
— The object and scope randomization method, randomize()
Testbenches with this feature exhibit more stable RNG behavior in the face of small changes to the user
code. Additionally, it enables more precise control over the generation of random values by manually
seeding threads and objects.
#### 18.14.1 Random stability properties
Random stability encompasses the following properties:
— Initialization RNG. Each module instance, interface instance, program instance, and package has an
initialization RNG. Each initialization RNG is seeded with the default seed. The default seed is an
implementation-dependent value. An initialization RNG shall be used in the creation of static
processes and static initializers (see the following list items). Static processes are defined in
AnnexP.
— Thread stability. Each thread has an independent RNG for all randomization system calls invoked
from that thread. When a new dynamic thread is created, its RNG is seeded with the next random
value from its parent thread. This property is called hierarchical seeding. When a static process is
created, its RNG is seeded with the next value from the initialization RNG of the module instance,
interface instance, program instance, or package containing the thread declaration.
Program and thread stability can be achieved as long as thread creation and random number
generation are done in the same order as before. When adding new threads to an existing test, they
can be added at the end of a code block in order to maintain random number stability of previously
created work.
— Object stability. Each class instance (object) has an independent RNG for all randomization methods
in the class. When an object is created using new, its RNG is seeded with the next random value
from the thread that creates the object. When a class object is created by a static declaration
initializer, there is no active thread; thus, the RNG of the created object is seeded with the next
random value of the initialization RNG of the module instance, interface instance, program instance,
or package in which the declaration occurred.
Object stability shall be preserved when object and thread creation and random number generation
are done in the same order as before. In order to maintain random number stability, new objects,
threads, and random numbers can be created after existing objects are created.
— Manual seeding. All noninitialization RNGs can be manually seeded. Combined with hierarchical
seeding, this facility allows users to define the operation of a subsystem (hierarchy subtree)
completely with a single seed at the root thread of the subsystem.
#### 18.14.2 Thread stability
Random values returned from the $urandom and $urandom_range system calls, std::randomize()
scope randomization method, and shuffle() array manipulation method are independent of thread
execution order. Random values generated to select branches of the procedural randcase and
randsequence statements are also independent of thread execution order. For example:
integer x, y, z;
fork
begin //set a seed at the start of a thread
process pvar;
pvar = process::self;
pvar.srandom(100);

<!-- Page 564 -->

IEEE Std x = $urandom;
end
begin //set a seed during a thread
process pvar;
pvar = process::self;
y = $urandom;
pvar.srandom(200);
end
begin // draw 2 values from the thread RNG
z = $urandom + $urandom ;
end
join
The preceding program fragment illustrates the following properties:
— Thread locality. The values returned for x, y, and z are independent of the order of thread execution.
This is an important property because it allows development of subsystems that are independent,
controllable, and predictable.
— Hierarchical seeding. When a thread is created, its random state is initialized using the next random
value from the parent thread as a seed. The three forked threads are all seeded from the parent
thread.
Each thread is seeded with a unique value, determined solely by its parent. The root of a thread execution
subtree determines the random seeding of its children. This allows entire subtrees to be moved and preserves
their behavior by manually seeding their root thread.
#### 18.14.3 Object stability
The randomize() method built into every class exhibits object stability. This is the property that calls to
randomize() in one instance are independent of calls to randomize() in other instances and are
independent of calls to other randomize functions.
For example:
class C1;
rand integer x;
endclass
class C2;
rand integer y;
endclass
initial begin
C1 c1 = new();
C2 c2 = new();
integer z;
void'(c1.randomize());
// z = $random;
void'(c2.randomize());
end
— The values returned for c1.x and c2.y are independent of each other.
— The calls to randomize() are independent of the $random system call. If one uncomments the line
z = $random above, there is no change in the values assigned to c1.x and c2.y.
— Each instance has a unique source of random values that can be seeded independently. That random
seed is taken from the parent thread when the instance is created.

<!-- Page 565 -->

IEEE Std — Objects can be seeded at any time using the srandom() method.
class C3
function new (integer seed);
//set a new seed for this instance
this.srandom(seed);
endfunction
endclass
Once an object is created, there is no guarantee that the creating thread can change the object’s random state
before another thread accesses the object. Therefore, it is best that objects self-seed within their new method
rather than externally.
### 18.15 Manually seeding randomize
Each object maintains its own internal RNG, which is used exclusively by its randomize() method. This
allows objects to be randomized independently of each other and of calls to other system randomization
functions. When an object is created, its RNG is seeded using the next value from the RNG of the thread that
creates the object. This process is called hierarchical object seeding.
Sometimes it is desirable to manually seed an object’s RNG using the srandom() method. This can be done
either in a class method or external to the class definition:
An example of seeding the RNG internally, as a class method, is as follows:
class Packet;
rand bit[15:0] header;
...
function new (int seed);
this.srandom(seed);
...
endfunction
endclass
An example of seeding the RNG externally is as follows:
Packet p = new(200); // Create p with seed 200.
p.srandom(300); // Re-seed p with seed 300.
Calling srandom() in an object’s new() function assures the object’s RNG is set with the new seed before
any class member values are randomized.
### 18.16 Random weighted case—randcase
statement_item ::= // from A.6.4
...
| randcase_statement
randcase_statement ::= // from A.6.7
randcase randcase_item { randcase_item } endcase
randcase_item ::= expression : statement_or_null
Syntax18-12—Randcase syntax (excerpt from AnnexA)

<!-- Page 566 -->

IEEE Std The keyword randcase introduces a case statement that randomly selects one of its branches. The
randcase_item expressions are non-negative integral values that constitute the branch weights. An item’s
weight divided by the sum of all weights gives the probability of taking that branch. For example:
randcase
3 : x = 1;
1 : x = 2;
4 : x = 3;
endcase
The sum of all weights is 8; therefore, the probability of taking the first branch is 0.375, the probability of
taking the second is 0.125, and the probability of taking the third is 0.5.
If a branch specifies a zero weight, then that branch is not taken. If all randcase_items specify zero weights,
then no branch is taken and a warning can be issued.
The randcase weights can be arbitrary expressions, not just constants. For example:
byte a, b;
randcase
a + b : x = 1;
a - b : x = 2;
a ^ ~b : x = 3;
12'h800 : x = 4;
endcase
The precision of each weight expression is self-determined. The sum of the weights is computed using
standard addition semantics (maximum precision of all weights), where each summand is unsigned. Each
weight expression is evaluated at most once (implementations can cache identical expressions) in an
unspecified order. In the preceding example, the first three weight expressions are computed using 8-bit
precision, and the fourth expression is computed using 12-bit precision. The resulting weights are added as
unsigned values using 12-bit precision. The weight selection then uses unsigned 12-bit comparison.
Each call to randcase retrieves one random number in the range of 0 to the sum of the weights. The
weights are then selected in declaration order: smaller random numbers correspond to the first (top) weight
statements.
randcase statements exhibit thread stability. The random numbers are obtained from $urandom_
range(); thus, random values drawn are independent of thread execution order. This can result in multiple
calls to $urandom_range() to handle numbers greater than 32 bits.
### 18.17 Random sequence generation—randsequence
Parser generators, such as yacc, use a BNF or similar notation to describe the grammar of the language to be
parsed. The grammar is thus used to generate a program that is able to check whether a stream of tokens
represents a syntactically correct utterance in that language. SystemVerilog’s sequence generator reverses
this process. It uses the grammar to randomly create a correct utterance (i.e., a stream of tokens) of the
language described by the grammar. The random sequence generator is useful for randomly generating
structured sequences of stimulus such as instructions or network traffic patterns.
The sequence generator uses a set of rules and productions within a randsequence block. The syntax of the
randsequence block is as follows in Syntax18-13.

<!-- Page 567 -->

IEEE Std statement_item ::= // from A.6.4
...
| randsequence_statement
randsequence_statement ::= // from A.6.12
randsequence ( [ rs_production_ identifier ] )
rs_production { rs_production }
endsequence
rs_production ::= [ data_type_or_void ] rs_production_identifier [ ( tf_port_list ) ] : rs_rule { | rs_rule } ;
rs_rule ::= rs_production_list [ := rs_weight_specification [ rs_code_block ] ]
rs_production_list ::=
rs_prod { rs_prod }
| rand join [ ( expression ) ] rs_production_item rs_production_item { rs_production_item }
rs_weight_specification ::=
integral_number
| ps_identifier
| ( expression )
rs_code_block ::= { { data_declaration } { statement_or_null } }
rs_prod ::=
rs_production_item
| rs_code_block
| rs_if_else
| rs_repeat
| rs_case
rs_production_item ::= rs_production_identifier [ ( list_of_arguments ) ]
rs_if_else ::= if ( expression ) rs_production_item [ else rs_production_item ]
rs_repeat ::= repeat ( expression ) rs_production_item
rs_case ::= case ( case_expression ) rs_case_item { rs_case_item } endcase
rs_case_item ::=
case_item_expression { , case_item_expression } : rs_production_item ;
| default [ : ] rs_production_item ;
case_expression ::= expression // from A.6.7
case_item_expression ::= expression
Syntax18-13—Randsequence syntax (excerpt from AnnexA)
A randsequence grammar is composed of one or more productions. Each production contains a name and
a list of production items. Production items are further classified into terminals and nonterminals.
Nonterminals are defined in terms of terminals and other nonterminals. A terminal is an indivisible item that
needs no further definition than its associated code block. Ultimately, every nonterminal is decomposed into
its terminals. A production list contains a succession of production items, indicating that the items shall be
streamed in sequence. A single production can contain multiple production lists separated by the “|”
symbol. Production lists separated by a “|” imply a set of choices, which the generator will make at random.
A simple example illustrates the basic concepts:
randsequence( main )
main : first second done ;
first : add | dec ;
second : pop | push ;

<!-- Page 568 -->

IEEE Std done : { $display("done"); } ;
add : { $display("add"); } ;
dec : { $display("dec"); } ;
pop : { $display("pop"); } ;
push : { $display("push"); } ;
endsequence
The production main is defined in terms of three nonterminals: first, second, and done. When main is
chosen, it generates the sequence, first, second, and done. When the first production is generated, it is
decomposed into its productions, which specify a random choice between add and dec. Similarly, the
second production specifies a choice between pop and push. All other productions are terminals; they are
completely specified by their code block, which in the example displays the production name. Thus, the
grammar leads to the following possible outcomes:
add pop done
add push done
dec pop done
dec push done
When the randsequence statement is executed, it generates a grammar-driven stream of random
productions. As each production is generated, the side effects of executing its associated code blocks
produce the desired stimulus. In addition to the basic grammar, the sequence generator provides for random
weights, interleaving, and other control mechanisms. Although the randsequence statement does not
intrinsically create a loop, a recursive production will cause looping.
The randsequence statement creates an automatic scope. All production identifiers are local to the scope.
In addition, each code block within the randsequence block creates an anonymous automatic scope.
Hierarchical references to the variables declared within the code blocks are not allowed. To declare a static
variable, the static prefix shall be used. The randsequence keyword can be followed by an optional
production name (inside the parentheses) that designates the name of the top-level production. If
unspecified, the first production becomes the top-level production.
#### 18.17.1 Random production weights
The probability that a production list is generated can be changed by assigning weights to production lists.
The probability that a particular production list is generated is proportional to its specified weight.
rs_production ::= // from A.6.12
[ data_type_or_void ] rs_production_identifier [ ( tf_port_list ) ] : rs_rule { | rs_rule } ;
rs_rule ::= rs_production_list [ := rs_weight_specification [ rs_code_block ] ]
Syntax18-14—Random production weights syntax (excerpt from AnnexA)
The := operator assigns the weight specified by the rs_weight_specification to its production list. An
rs_weight_specification shall evaluate to an integral non-negative value. A weight is only meaningful when
assigned to alternative productions, that is, production lists separated by a |. Weight expressions are
evaluated when their enclosing production is selected, thus allowing weights to change dynamically. For
example, the first production of the previous example can be rewritten as follows:
first : add := 3
| dec := (1 + 1) // 2
;

<!-- Page 569 -->

IEEE Std This defines the production first in terms of two weighted production lists, add and dec. The production
add will be generated with 60% probability, and the production dec will be generated with 40% probability.
If no weight is specified, a production shall use a weight of 1. If only some weights are specified, the
unspecified weights shall use a weight of 1.
#### 18.17.2 if–else production statements
A production can be made conditionally by means of an if–else production statement. The syntax of the
if–else production statement is as follows in Syntax18-15.
rs_if_else ::= if ( expression ) rs_production_item [ else rs_production_item ] // from A.6.12
Syntax18-15—If–else conditional random production syntax (excerpt from AnnexA)
The expression can be any expression that evaluates to a Boolean value. If the expression evaluates to true,
the production following the expression is generated; otherwise, the production following the optional else
statement is generated. For example:
randsequence()
...
PP_OP : if ( depth < 2 ) PUSH else POP ;
PUSH : { ++depth; do_push(); };
POP : { --depth; do_pop(); };
endsequence
This example defines the production PP_OP. If the variable depth is less than 2, then production PUSH is
generated. Otherwise, production POP is generated. The variable depth is updated by the code blocks of both
the PUSH and POP productions.
#### 18.17.3 Case production statements
A production can be selected from a set of alternatives using a case production statement. The syntax of the
case production statement is as follows in Syntax18-16.
rs_case ::= case ( case_expression ) rs_case_item { rs_case_item } endcase // from A.6.12
rs_case_item ::=
case_item_expression { , case_item_expression } : rs_production_item ;
| default [ : ] rs_production_item ;
case_expression ::= expression // from A.6.7
case_item_expression ::= expression
Syntax18-16—Case random production syntax (excerpt from AnnexA)
The case production statement is analogous to the procedural case statement except as noted below. The
case expression is evaluated, and its value is compared against the value of each case_item expression, all of
which are evaluated and compared in the order in which they are given. The production generated is the one
associated with the first case_item expression matching the case expression. If no matching case_item
expression is found, then the production associated with the optional default item is generated, or nothing if
there is no default item. Multiple default statements in one case production statement shall be illegal. The

<!-- Page 570 -->

IEEE Std case_item expressions separated by commas allow multiple expressions to share the production. For
example:
randsequence()
SELECT : case ( device & 7 )
0 : NETWORK ;
1, 2 : DISK ;
default : MEMORY ;
endcase ;
...
endsequence
This example defines the production SELECT with a case statement. The case_expression (device & 7) is
evaluated and compared against the two case_item_expressions. If the expression matches 0, the production
NETWORK is generated; if it matches 1 or 2, the production DISK is generated. Otherwise, the production
MEMORY is generated.
#### 18.17.4 Repeat production statements
The repeat production statement is used to iterate over a production a specified number of times. The
syntax of the repeat production statement is as follows in Syntax18-17.
rs_repeat ::= repeat ( expression ) rs_production_item // from A.6.12
Syntax18-17—Repeat random production syntax (excerpt from AnnexA)
The repeat expression shall evaluate to a non-negative integral value. That value specifies the number of
times that the corresponding production is generated. For example:
randsequence()
...
PUSH_OPER : repeat($urandom_range(2, 6)) PUSH ;
PUSH : ...
endsequence
In this example, the PUSH_OPER production specifies that the PUSH production be repeated a random
number of times (between 2 and 6) depending on the value returned by $urandom_range().
The repeat production statement itself cannot be terminated prematurely. A break statement will
terminate the entire randsequence block (see 18.17.6).
#### 18.17.5 Interleaving productions—rand join
The rand join production control is used to randomly interleave two or more production sequences while
maintaining the relative order of each sequence. The syntax of the rand join production control is as
follows in Syntax18-18.
rs_production_list ::= // from A.6.12
rs_prod { rs_prod }
| rand join [ ( expression ) ] rs_production_item rs_production_item { rs_production_item }
Syntax18-18—Rand join random production syntax (excerpt from AnnexA)
For example:

<!-- Page 571 -->

IEEE Std randsequence( TOP )
TOP : rand join S1 S2 ;
S1 : A B ;
S2 : C D ;
endsequence
The generator will randomly produce the following sequences:
A B C D
A C B D
A C D B
C D A B
C A B D
C A D B
The optional expression following the rand join keywords shall be a real number in the range of 0.0 to 1.0.
The value of this expression represents the degree to which the length of the sequences to be interleaved
affects the probability of selecting a sequence. A sequence’s length is the number of productions not yet
interleaved at a given time. If the expression is 0.0, the shortest sequences are given higher priority. If the
expression is 1.0, the longest sequences are given priority. For instance, using the previous example,
TOP : rand join (0.0) S1 S2 ;
gives higher priority to the sequences: A B C D C D A B, and
TOP : rand join (1.0) S1 S2 ;
gives higher priority to the sequences: A C B D A C D B C A B D C A D B.
If unspecified, the generator used the default value of 0.5, which does not prioritize any sequence length.
At each step, the generator interleaves nonterminal symbols to depth of 1.
#### 18.17.6 Aborting productions—break and return
Two procedural statements can be used to terminate a production prematurely: break and return. These
two statements can appear in any code block; they differ in what they consider the scope from which to exit.
The break statement terminates the sequence generation. When a break statement is executed from within
a production code block, it forces a jump out of the randsequence block. For example:
randsequence()
WRITE : SETUP DATA ;
SETUP : { if( fifo_length >= max_length ) break; } COMMAND ;
DATA : ...
endsequence
next_statement : ...
When the preceding example executes the break statement within the SETUP production, the COMMAND
production is not generated, and execution continues on the line labeled next_statement. Use of the
break statement within a loop statement behaves as defined in 12.8. Thus, the break statement terminates
the smallest enclosing looping statement; otherwise, it terminates the randsequence block.
The return statement aborts the generation of the current production. When a return statement is
executed from within a production code block, the current production is aborted. Sequence generation
continues with the next production following the aborted production. For example:

<!-- Page 572 -->

IEEE Std randsequence()
TOP : P1 P2 ;
P1 : A B C ;
P2 : A { if( flag == 1 ) return; } B C ;
A : { $display( "A" ); } ;
B : { if( flag == 2 ) return; $display( "B" ); } ;
C : { $display( "C" ); } ;
endsequence
Depending on the value of variable flag, the preceding example displays the following:
flag == 0 ==> A B C A B C
flag == 1 ==> A B C A
flag == 2 ==> A C A C
When flag == 1, production P2 is aborted in the middle, after generating A. When flag == 2,
production B is aborted twice (once as part of P1 and once as part of P2); however, each time, generation
continues with the next production, C.
#### 18.17.7 Value passing between productions
Data can be passed down to a production about to be generated, and generated productions can return data to
the nonterminals that triggered their generation. Passing data to a production is similar to a task call and uses
the same syntax. Returning data from a production requires that a type be declared for the production, which
uses syntax similar to a function declaration.
Productions that accept data include a formal argument list. The syntax for declaring the arguments to a
production is similar to a task prototype; the syntax for passing data to the production is the same as a task
call (see Syntax18-19).
rs_production ::= // from A.6.12
[ data_type_or_void ] rs_production_identifier [ ( tf_port_list ) ] : rs_rule { | rs_rule } ;
Syntax18-19—Random production syntax (excerpt from AnnexA)
For example, the previous first example could be written as follows:
randsequence( main )
main : first second gen ;
first : add | dec ;
second : pop | push ;
add : gen("add") ;
dec : gen("dec") ;
pop : gen("pop") ;
push : gen("push") ;
gen( string s = "done" ) : { $display( s ); } ;
endsequence
In this example, the production gen accepts a string argument whose default is "done". Five other
productions generate this production, each with a different argument (the one in main uses the default).
A production creates a scope, which encompasses all its rules and code blocks. Thus, arguments passed
down to a production are available throughout the production.

<!-- Page 573 -->

IEEE Std Productions that return data require a type declaration. The optional return type precedes the production.
Productions that do not specify a return type shall assume a void return type.
A value is returned from a production by using the return with an expression. When the return statement
is used with a production that returns a value, it shall specify an expression of the correct type, just like
nonvoid functions. The return statement assigns the given expression to the corresponding production.
The return value can be read in the code blocks of the production that triggered the generation of the
production returning a value. Within these code blocks, return values are accessed using the production
name plus an optional indexing expression.
Within a rule, a variable is implicitly declared for each production (of the rule) that returns a value. The type
of the variable is determined by the return type of the production and the number of times the production
syntactically appears with the rule. If a production appears only once in a rule, the type of the implicit
variable is the return type of the production. If a production appears multiple times, the type is an array
where the element type is the return type of the production. The array is indexed from 1 to the number of
times the production appears within the rule. The elements of the array are assigned the values returned by
the instances of the production according to the syntactic order of appearance.
Example 1:
randsequence( bin_op )
void bin_op : value operator value // void type is optional
{ $display("%s %b %b", operator, value[1], value[2]); }
;
bit [7:0] value : { return $urandom; } ;
string operator : { return "+" ; }
| { return "-" ; }
| { return "*" ; }
;
endsequence
In the preceding example, the operator and value productions return a string and an 8-bit value,
respectively. The production bin_op includes these two value-returning productions. Therefore, the code
block associated with production bin_op has access to the following implicit variable declarations:
bit [7:0] value [1:2];
string operator;
Example 2:
int cnt;
...
randsequence( A )
void A : A1 A2;
void A1 : { cnt = 1; } B repeat(5) C B
{ $display("c=%d, b1=%d, b2=%d", C, B[1], B[2]); }
;
void A2 : if (cond) D(5) else D(20)
{ $display("d1=%d, d2=%d", D[1], D[2]); }
;
int B : C { return C;}
| C C { return C[2]; }
| C C C { return C[3]; }
;
int C : { cnt = cnt + 1; return cnt; };
int D (int prm) : { return prm; };
endsequence

<!-- Page 574 -->

IEEE Std In Example 2, the code block in production A1 has access to the implicit variable declarations:
int B[1:2];
int C;
The code block in production A2 has access to the implicit variable declaration:
int D[1:2];
If cond is true, the first element is assigned the value returned by D(5). If cond is false, the second element
is assigned the value returned by D(20).
The code block in the first rule of production B has access to the implicit variable declaration:
int C;
The code block in the third rule of production B has access to the implicit variable declaration:
int C[1:3];
Accessing these implicit variables yields the values returned from the corresponding productions. When
executed, Example 1, above, displays a simple three-item random sequence: an operator followed by two
8-bit values. The operators +, -, and * are chosen with a distribution of 5/8, 2/8, and 1/8, respectively.
Only the return values of productions already generated (i.e., to the left of the code block accessing them)
can be retrieved. Attempting to read the return value of a production that has not been generated results in an
undefined value. For example:
X : A {int y = B;} B ; // invalid use of B
X : A {int y = A[2];} B A ; // invalid use of A[2]
X : A {int y = A;} B {int j = A + B;} ; // valid
The sequences produced by randsequence can be driven directly into a system, as a side effect of
production generation, or the entire sequence can be generated for future processing. For example, the
following function generates and returns a queue of random numbers in the range given by its arguments.
The first and last queue items correspond to the lower and upper bounds, respectively. Also, the size of the
queue is randomly selected based on the production weights.
function int[$] GenQueue(int low, int high);
int[$] q;
randsequence()
TOP : BOUND(low) LIST BOUND(high) ;
LIST : LIST ITEM := 8 { q = { q, ITEM }; }
| ITEM := 2 { q = { q, ITEM }; }
;
int ITEM : { return $urandom_range( low, high ); } ;
BOUND(int b) : { q = { q, b }; } ;
endsequence
GenQueue = q;
endfunction
When the randsequence in function GenQueue executes, it generates the TOP production, which causes
three productions to be generated: BOUND with argument low, LIST, and BOUND with argument high. The
BOUND production simply appends its argument to the queue. The LIST production consists of a weighted
LIST ITEM production and an ITEM production. The LIST ITEM production is generated with 80%

<!-- Page 575 -->

IEEE Std probability, which causes the LIST production to be generated recursively, thereby postponing the
generation of the ITEM production. The selection between LIST ITEM and ITEM is repeated until the ITEM
production is selected, which terminates the LIST production. Each time the ITEM production is generated,
it produces a random number in the indicated range, which is later appended to the queue.
The following example uses a randsequence block to produce random traffic for a DSL packet network:
class DSL; ... endclass // class that creates valid DSL packets
randsequence (STREAM)
STREAM : GAP DATA := 80
| DATA := 20 ;
DATA : PACKET(0) := 94 { transmit( PACKET ); }
| PACKET(1) := 6 { transmit( PACKET ); } ;
DSL PACKET (bit bad) : { DSL d = new;
if( bad ) d.crc ^= 23; // mangle crc
return d;
};
GAP: { ## {$urandom_range( 1, 20 )}; };
endsequence
In this example, the traffic consists of a stream of (good and bad) data packets and gaps. The first
production, STREAM, specifies that 80% of the time the traffic consists of a GAP followed by some DATA and
20% of the time it consists of just DATA (no GAP). The second production, DATA, specifies that 94% of all
data packets are good packets and the remaining 6% are bad packets. The PACKET production implements
the DSL packet creation; if the production argument is 1, then a bad packet is produced by mangling the crc
of a valid DSL packet. Finally, the GAP production implements the transmission gaps by waiting a random
number of cycles between 1 and 20.

<!-- Page 576 -->

IEEE Std 1800-2023