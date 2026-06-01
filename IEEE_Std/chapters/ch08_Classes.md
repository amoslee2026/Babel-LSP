# Chapter 8: Classes

## 8. Classes
### 8.1 General
This clause describes the following:
— Class definitions
— Virtual classes and methods
— Polymorphism
— Parameterized classes
— Interface classes
— Memory management
### 8.2 Overview
A class is a type that includes data and subroutines (functions and tasks) that operate on those data. A class’s
data are referred to as class properties, and its subroutines are called methods; both are members of the class.
The class properties and methods, taken together, define the contents and capabilities of some kind of object.
For example, a packet might be an object. It might have a command field, an address, a sequence number, a
time stamp, and a packet payload. In addition, there are various things that can be done with a packet:
initialize the packet, set the command, read the packet’s status, or check the sequence number. Each packet
is different, but as a class, packets have certain intrinsic properties that can be captured in a definition.
class Packet ;
//data or class properties
bit [3:0] command;
bit [40:0] address;
bit [4:0] initiator_id;
integer time_requested;
integer time_issued;
integer status;
typedef enum {ERR_OVERFLOW = 10, ERR_UNDERFLOW = 1123} PCKT_TYPE;
const integer buffer_size = 100;
const integer header_size;
// initialization
function new();
command = 4'd0;
address = 41'b0;
initiator_id = 5'bx;
header_size = 10;
endfunction
// methods
// public access entry points
task clean();
command = 0; address = 0; initiator_id = 5'bx;
endtask
task issue_request( int delay );
// send request to bus
endtask
function integer current_status();

<!-- Page 181 -->

IEEE Std current_status = status;
endfunction
endclass
The object-oriented class extension allows objects to be created and destroyed dynamically. Class instances,
or objects, can be passed around via object handles, which provides a safe-pointer capability. An object can
be declared as an argument with direction input, output, inout, or ref. In each case, the argument
copied is the object handle, not the contents of the object.
### 8.3 Syntax
class_declaration ::= // from A.1.2
[ virtual ] class [ final_specifier ] class_identifier [ parameter_port_list ]
[ extends class_type [ ( [ list_of_arguments | default ] ) ] ]
[ implements interface_class_type { , interface_class_type } ] ;
{ class_item }
endclass [ : class_identifier ]
class_item ::= // from A.1.9
{ attribute_instance } class_property
| { attribute_instance } class_method
| { attribute_instance } class_constraint
| { attribute_instance } class_declaration
| { attribute_instance } interface_class_declaration
| { attribute_instance } covergroup_declaration
| local_parameter_declaration ;
| parameter_declaration7 ;
| ;
class_property ::=
{ property_qualifier } data_declaration
| const { class_item_qualifier } data_type const_identifier [ = constant_expression ] ;
class_method ::=
{ method_qualifier } task_declaration
| { method_qualifier } function_declaration
| pure virtual { class_item_qualifier } method_prototype ;
| extern { method_qualifier } method_prototype8 ;
| { method_qualifier } class_constructor_declaration
| extern { method_qualifier } class_constructor_prototype
class_constructor_declaration ::=
function [ class_scope ] new [ ( [ class_constructor_arg_list ] ) ] ;
{ block_item_declaration }
[ super . new [ ( [ list_of_arguments | default ] ) ] ; ]
{ function_statement_or_null }
endfunction [ : new ]
class_constructor_prototype ::= function new [ ( [ class_constructor_arg_list ] )] ;
class_constructor_arg_list ::= class_constructor_arg { , class_constructor_arg }9
class_constructor_arg ::= tf_port_item | default
class_constraint ::=
constraint_prototype
| constraint_declaration

<!-- Page 182 -->

IEEE Std class_item_qualifier10 ::= static | protected | local
property_qualifier10 ::=
random_qualifier
| class_item_qualifier
random_qualifier10 ::= rand | randc
method_qualifier10 ::=
[ pure ] virtual
| class_item_qualifier
method_prototype ::=
task_prototype
| function_prototype
function_declaration ::= // from A.2.6
function [ dynamic_override_specifiers ]25 [ lifetime ] function_body_declaration
function_prototype ::=
function [ dynamic_override_specifiers ]25 data_type_or_void function_identifier
[ ( [ tf_port_list ] ) ]
task_declaration ::= // from A.2.7
task [ dynamic_override_specifiers ]25 [ lifetime ] task_body_declaration
task_prototype ::=
task [ dynamic_override_specifiers ]25 task_identifier [ ( [ tf_port_list ] ) ]
dynamic_override_specifiers ::= [ initial_or_extends_specifier ] [ final_specifier ]
initial_or_extends_specifier ::=
: initial
| : extends
final_specifier ::= : final
7) In a parameter_declaration that is a class_item, the parameter keyword shall be a synonym for the
localparam keyword.
8) It shall be illegal to use the final_specifier when declaring a pure virtual method or pure constraint.
9) The default keyword shall appear at most once in a class constructor argument list.
10) In any one declaration, only one of protected or local is allowed, only one of rand or randc is allowed,
and static and/or virtual can appear only once.
25) The dynamic_override_specifiers shall only be legal on method declarations inside a non-interface class scope.
Syntax8-1—Class syntax (excerpt from AnnexA)
### 8.4 Objects (class instance)
A class defines a data type. An object is an instance of that class. An object is used by first declaring a
variable of that class type (that holds an object handle) and then creating an object of that class (using the
new function) and assigning it to the variable.
Packet p; // declare a variable of class Packet
p = new; // initialize variable to a new allocated object
// of the class Packet
SystemVerilog objects are referenced using an object handle. The variable p is said to hold an object handle
to an object of class Packet.

<!-- Page 183 -->

IEEE Std Uninitialized object handles are set by default to the special value null. An uninitialized object handle can
be detected by comparing its value with null.
For example: The following task task1 checks whether the object handle is initialized. If it is not, it creates
a new object via the new function.
class obj_example;
...
endclass
task task1(integer a, obj_example myexample);
if (myexample == null) myexample = new;
endtask
Accessing non-static members (see 8.9) or virtual methods (see 8.20) via a null object handle is illegal. The
result of an illegal access via a null object handle is indeterminate, and implementations may issue an error.
There are some differences between a C pointer and a SystemVerilog object handle (see Table8-1). C
pointers give programmers a lot of latitude in how a pointer can be used. The rules governing the usage of
SystemVerilog object handles are much more restrictive. A C pointer can be incremented, for example, but a
SystemVerilog object handle cannot. In addition to object handles, 6.14 introduces the chandle data type
for use with the DPI (see Clause35).
Table8-1—Comparison of pointer and handle types
SV object
Operation C pointer SV chandle
handle
Arithmetic operations (such as incrementing) Allowed Not allowed Not allowed
For arbitrary data types Allowed Not allowed Not allowed
Dereference when null Error Error, see Not allowed
text above
Casting Allowed Limited Not allowed
Assignment to an address of a data type Allowed Not allowed Not allowed
Unreferenced objects are garbage collected No Yes No
Default value Undefined null null
For classes (C++) Allowed Not allowed
Only the following operators are valid on object handles:
— Equality (==), inequality (!=) with another class object or with null. One of the objects being
compared shall be assignment compatible with the other.
— Case equality (===), case inequality (!==) with another class object or with null (same semantics
as == and !=).
— Conditional operator (see 11.4.11).
— Assignment of a class object whose class data type is assignment compatible with the target class
object.
— Assignment of null.

<!-- Page 184 -->

IEEE Std ### 8.5 Object properties and object parameter data
There are no restrictions on the data type of a class property. The class properties of an object can be used by
qualifying class property names with an instance name. Using the earlier example (see 8.2), the properties
for the Packet object p can be used as follows:
Packet p = new;
int var1;
p.command = INIT;
p.address = $random;
packet_time = p.time_requested;
var1 = p.buffer_size;
Class enum names, in addition to being accessed using a class scope resolution operator, can also be
accessed by qualifying the class enum name with an instance name.
initial $display (p.ERR_OVERFLOW);
The parameter data values of an object can also be accessed by qualifying the class value parameter or local
value parameter name with an instance name. Such an expression is not a constant expression. Accessing
data types using a class handle is not allowed. For example:
class vector #(parameter width = 7, type T = int);
endclass
vector #(3) v = new;
initial $display (vector #(3)::T'(3.45)); // Typecasting
initial $display ((v.T)'(3.45)); //ILLEGAL
initial $display (v.width);
### 8.6 Object methods
An object’s methods can be accessed using the same syntax used to access class properties:
Packet p = new;
status = p.current_status();
The preceding assignment to status cannot be written as follows:
status = current_status(p);
The focus in object-oriented programming is the object, in this case the packet, not the function call. Also,
objects are self-contained, with their own methods for manipulating their own properties. Therefore, the
object does not have to be passed as an argument to current_status(). A class’s properties are freely
and broadly available to the methods of the class, but each method only accesses the properties associated
with its object, i.e., its instance.
The lifetime of methods declared as part of a class type shall be automatic. It shall be illegal to declare a
class method with a static lifetime.

<!-- Page 185 -->

IEEE Std ### 8.7 Constructors
SystemVerilog does not require the complex memory allocation and deallocation of C++. Construction of an
object is straightforward; and garbage collection, as in Java, is implicit and automatic. There can be no
memory leaks or other subtle behaviors, which are so often the bane of C++ programmers.
SystemVerilog provides a mechanism for initializing an instance at the time the object is created. When an
object is created, for example,
Packet p = new;
the system executes the new function associated with the class:
class Packet;
integer command;
function new();
command = IDLE;
endfunction
endclass
As shown previously, new is now being used in two very different contexts with very different semantics.
The variable declaration creates an object of class Packet. In the course of creating this instance, the new
function is invoked, in which any specialized initialization required can be done. The new function is also
called the class constructor.
The new operation is defined as a function with no return type, and like any other function, it shall be
nonblocking. Even though new does not specify a return type, the left-hand side of the assignment
determines the return type.
If a class does not provide an explicit user-defined new method, an implicit new method shall be provided
automatically. The new method of a derived class (see 8.13) shall first call its base class constructor
[super.new() as described in 8.15]. After the base class constructor call (if any) has completed, each
property defined in the class shall be initialized to its explicit default value or its uninitialized value if no
default is provided. After the properties are initialized, the remaining code in a user-defined constructor shall
be evaluated. The default constructor has no additional effect after the property initialization. The value of a
property prior to its initialization shall be undefined.
Example:
class C;
int c1 = 1;
int c2 = 1;
int c3 = 1;
function new(int a);
c2 = 2;
c3 = a;
endfunction
endclass
class D extends C;
int d1 = 4;
int d2 = c2;
int d3 = 6;
function new;
super.new(d3);

<!-- Page 186 -->

IEEE Std endfunction
endclass
After the construction of an object of type D is complete, the properties are as follows:
— c1 has the value 1
— c2 has the value 2 since the constructor assignment happens after the property initialization
— c3 has an undefined value since the constructor call from D passes in the value of d3, which is
undefined when the super.new(d3) call is made
— d1 has the value 4
— d2 has the value 2 since the super.new call is complete when d2 is initialized
— d3 has the value 6
It is also possible to pass arguments to the constructor, which allows run-time customization of an object:
Packet p = new(STARTUP, $random, $time);
where the new initialization task in Packet might now look like the following:
function new(int cmd = IDLE, bit[12:0] adrs = 0, int cmd_time );
command = cmd;
address = adrs;
time_requested = cmd_time;
endfunction
The conventions for arguments are the same as for any other procedural subroutine calls, such as the use of
default argument values.
A constructor may be declared as a local or protected method (see 8.18). A constructor shall not be
declared as a static (see 8.10) or virtual method (see 8.20).
### 8.8 Typed constructor calls
class_new23 ::= // from A.2.4
[ class_scope ] new [ ( list_of_arguments ) ]
| new expression
23) In a shallow copy, the expression shall evaluate to an object handle.
Syntax8-2—Calling a constructor (excerpt from AnnexA)
Uses of new described in earlier parts of this clause require that the type of the object to be constructed
matches the assignment target’s type. An alternative form of constructor invocation, the typed constructor
call, adds class_scope immediately before the new keyword, specifying the constructed object’s type
independently of the assignment target. The specified type shall be assignment compatible with the target.
The following example illustrates a typed constructor call. The extends keyword and the concept of a
superclass type are described in 8.13.
class C; . . . endclass
class D extends C; . . . endclass

<!-- Page 187 -->

IEEE Std C c = D::new; // variable c of superclass type C now references
// a newly constructed object of type D
NOTE—The effect of this typed constructor call is as if a temporary variable of type D had been declared, constructed,
and then copied to variable c, as in this example fragment:
D d = new;
C c = d;
A typed constructor call shall create and initialize a new object of the specified type. Creation and
initialization of the new object shall proceed exactly as it would for an ordinary constructor as described in
8.7. Arguments may be passed to a typed constructor call if appropriate, just as for an ordinary constructor.
If the type of object to be constructed is a parameterized class, as described in 8.25, the specified type may
have parameter specializations. The following example, continuing the previous example, illustrates a typed
constructor call for a parameterized class and also illustrates how arguments may be passed to the
constructor as described in 8.7.
class E #(type T = int) extends C;
T x;
function new(T x_init);
super.new();
x = x_init;
endfunction
endclass
initial begin
c = E #(.T(byte))::new(.x_init(5));
end
### 8.9 Static class properties
The previous examples have only declared instance class properties. Each instance of the class (i.e., each
object of type Packet) has its own copy of each of its eight variables. Sometimes only one version of a
variable is required to be shared by all instances. These class properties are created using the keyword
static. Thus, for example, in the following case, all instances of a class need access to a common file
descriptor:
class Packet ;
static integer fileID = $fopen( "data", "r" );
Now, fileID shall be created and initialized once. Thereafter, every Packet object can access the file
descriptor in the usual way:
Packet p;
c = $fgetc( p.fileID );
The static class properties can be used without creating an object of that type.
### 8.10 Static methods
Methods can be declared as static. A static method is subject to all the class scoping and access rules, but
behaves like a regular subroutine that can be called outside the class, even with no class instantiation. A
static method has no access to non-static members (class properties or methods), but it can directly access
static class properties or call static methods of the same class. Access to non-static members or to the special

<!-- Page 188 -->

IEEE Std this handle within the body of a static method is illegal and results in a compiler error. Static methods
cannot be virtual.
class id;
static int current = 0;
static function int next_id();
next_id = ++current; // OK to access static class property
endfunction
endclass
A static method is different from a task with static lifetime. The former refers to the lifetime of the method
within the class, while the latter refers to the lifetime of the arguments and variables within the task.
class TwoTasks;
static task t1(); ... endtask // static class method with
// automatic variable lifetime
task static t2(); ... endtask // ILLEGAL: non-static class method with
// static variable lifetime
endclass
### 8.11 This
The this keyword is used to unambiguously refer to class properties, value parameters, local value
parameters, or methods of the current instance. The this keyword denotes a predefined object handle that
refers to the object that was used to invoke the subroutine that this is used within. The this keyword shall
only be usedas type(this) (see 6.23) or within non-static class methods, constraints, inlined constraint
methods, or covergroups embedded within classes (see 19.4); otherwise, an error shall be issued. For
example, the following declaration is a common way to write an initialization task:
class Demo;
integer x;
function new (integer x);
this.x = x;
endfunction
endclass
The x is now both a property of the class and an argument to the function new. In the function new, an
unqualified reference to x shall be resolved by looking at the innermost scope, in this case, the subroutine
argument declaration. To access the instance class property, it is qualified with the this keyword, to refer to
the current instance.
NOTE—In writing methods, members can be qualified with this to refer to the current instance, but it is usually
unnecessary.
### 8.12 Assignment, renaming, and copying
Declaring a class variable only creates the name by which the object is known. Thus,
Packet p1;
creates a variable, p1, that can hold the handle of an object of class Packet, but the initial value of p1 is
null. The object does not exist, and p1 does not contain an actual handle, until an instance of type Packet
is created:

<!-- Page 189 -->

IEEE Std p1 = new;
Thus, if another variable is declared and assigned the old handle, p1, to the new one, as in
Packet p2;
p2 = p1;
then there is still only one object, which can be referred to with either the name p1 or p2. In this example,
new was executed only once; therefore, only one object has been created.
If, however, the preceding example is rewritten as follows, a copy of p1 shall be made:
Packet p1;
Packet p2;
p1 = new;
p2 = new p1;
The last statement has new executing a second time, thus creating a new object p2, whose class properties
are copied from p1. This is known as a shallow copy. All of the variables are copied: integers, strings,
instance handles, etc. Objects, however, are not copied, only their handles; as before, two names for the
same object have been created. This is true even if the class declaration includes the instantiation operator
new.
It shall be illegal to use a typed constructor call for a shallow copy (see 8.8).
A shallow copy executes in the following manner:
1) An object of the class type being copied is allocated. This allocation shall not call the object’s con-
structor or execute any variable declaration initialization assignments.
2) All class properties, including the internal states used for randomization and coverage, are copied to
the new object. Object handles are copied; this includes the object handles for covergroup objects
(see Clause19). An exception is made for embedded covergroups (see 19.4). The object handle of
an embedded covergroup shall be set to null in the new object. The internal states for
randomization include the random number generator (RNG) state, the constraint_mode status of
constraints, the rand_mode status of random variables, and the cyclic state of randc variables (see
Clause18).
3) A handle to the newly created object is assigned to the variable on the left-hand side.
NOTE—A shallow copy does not create new coverage objects (covergroup instances). As a result, the properties of the
new object are not covered.
class baseA ;
integer j = 5;
endclass
class B ;
integer i = 1;
baseA a = new;
endclass
class xtndA extends baseA;
rand int x;
constraint cst1 { x < 10; }
endclass
function integer test;
xtndA xtnd1;
baseA base2, base3;

<!-- Page 190 -->

IEEE Std B b1 = new; // Create an object of class B
B b2 = new b1; // Create an object that is a copy of b1
b2.i = 10; // i is changed in b2, but not in b1
b2.a.j = 50; // change a.j, shared by both b1 and b2
test = b1.i; // test is set to 1 (b1.i has not changed)
test = b1.a.j; // test is set to 50 (a.j has changed)
xtnd1 = new; // create a new instance of class xtndA
xtnd1.x = 3;
base2 = xtnd1; // base2 refers to the same object as xtnd1
base3 = new base2; // Creates a shallow copy of xtnd1
endfunction
In the last statement base3 is assigned a shallow copy of base2. The type of the variable base3 is a handle
to the base class baseA. When the shallow copy is invoked, this variable contains a handle to an instance of
the extended class xtndA. The shallow copy creates a duplicate of the referenced object, resulting in a
duplicate instance of the extended class xntdA. The handle to this instance is then assigned to the variable
base3.
Several things are noteworthy. First, class properties and instantiated objects can be initialized directly in a
class declaration. Second, the shallow copy does not copy objects. Third, instance qualifications can be
chained as needed to reach into objects or to reach through objects:
b1.a.j // reaches into a, which is a property of b1
p.next.next.next.val // chain through a sequence of handles to get to val
To do a full (deep) copy, where everything (including nested objects) is copied, custom code is typically
needed. For example:
Packet p1 = new;
Packet p2 = new;
p2.copy(p1);
where copy(Packet p) is a custom method written to copy the object specified as its argument into its
instance.
### 8.13 Inheritance and subclasses
The previous subclauses defined a class called Packet. This class can be extended so that the packets can be
chained together into a list. One solution would be to create a new class called LinkedPacket that contains
a variable of type Packet called packet_c.
To refer to a class property of Packet, the variable packet_c needs to be referenced.
class LinkedPacket;
Packet packet_c;
LinkedPacket next;
function LinkedPacket get_next();
get_next = next;
endfunction
endclass
Because LinkedPacket is a special form of Packet, a more elegant solution is to extend the class, creating
a subclass that inherits the members of the base class.

<!-- Page 191 -->

IEEE Std Subclasses (or derived classes) are classes that are extensions of the current class whereas superclasses (or
base classes) are classes from which the current class is extended, beginning with the original base class.
Thus, for example:
class LinkedPacket extends Packet;
LinkedPacket next;
function LinkedPacket get_next();
get_next = next;
endfunction
endclass
Now, all of the methods and class properties of Packet are part of LinkedPacket (as if they were defined
in LinkedPacket), and LinkedPacket has additional class properties and methods.
The methods of the base class can also be overridden to change their definitions.
The mechanism provided by SystemVerilog is called single inheritance, that is, each class is derived from a
single base class.
The final specifier, preceded by a colon, when applied to classes, specifies that a class shall not be extended:
class :final TopPacket extends LinkedPacket;
...
endclass
class BrokenPacket extends TopPacket; // ILLEGAL: TopPacket specified final!
An attempt to extend a class that was specified as final shall result in an error.
### 8.14 Overridden members
Subclass objects are also legal representative objects of their base classes. For example, every
LinkedPacket object is a perfectly legal Packet object.
The handle of a LinkedPacket object can be assigned to a Packet variable:
LinkedPacket lp = new;
Packet p = lp;
In this case, references to p access the methods and class properties of the Packet class. So, for example, if
class properties and methods in LinkedPacket are overridden, these overridden members referred to
through p get the original members in the Packet class. From p, new and all overridden members in
LinkedPacket are now hidden.
class Packet;
integer i = 1;
function integer get();
get = i;
endfunction
endclass
class LinkedPacket extends Packet;
integer i = 2;

<!-- Page 192 -->

IEEE Std function integer get();
get = -i;
endfunction
endclass
LinkedPacket lp = new;
Packet p = lp;
j = p.i; // j = 1, not 2
j = p.get(); // j = 1, not -1 or –2
To call the overridden method via a base class object (p in the example), the method needs to be declared
virtual (see 8.20).
### 8.15 Super
The super keyword is used from within a derived class to refer to members, class value parameters, or local
value parameters of the base class. It is necessary to use super to access members, value parameters, or
local value parameters of a base class when those are overridden by the derived class. An expression using
super to access the value parameter or local value parameter is not a constant expression.
class Packet; // base class
integer value;
function integer delay();
delay = value * value;
endfunction
endclass
class LinkedPacket extends Packet; // derived class
integer value;
function integer delay();
delay = super.delay()+ value * super.value;
endfunction
endclass
The member, value parameter, or local value parameter can be declared a level up or be inherited by the
class one level up. There is no way to reach higher (for example, super.super.count is not allowed).
A super.new call shall be the first statement executed in the constructor. This is because the superclass
shall be initialized before the current class and, if the user code does not provide an initialization, the
compiler shall insert a call to super.new automatically.
### 8.16 Casting
It is always legal to assign an expression of a subclass type to a variable of a class type higher in the
inheritance tree (a superclass or ancestor of the expression type). It shall be illegal to directly assign a
variable of a superclass type to a variable of one of its subclass types. However, $cast may be used to
assign a superclass handle to a variable of a subclass type provided the superclass handle refers to an object
that is assignment compatible with the subclass variable.
To check whether the assignment is legal, the dynamic cast function $cast is used (see 6.24.2).
The prototype for $cast is as follows:
function int $cast(data_type dest_variable, data_type source_expression);

<!-- Page 193 -->

IEEE Std or
task $cast(data_type dest_variable, data_type source_expression);
When $cast is applied to class handles, it succeeds in only three cases:
1) The source expression and the destination type are assignment compatible, that is, the destination is
the same type or a superclass of the source expression.
2) The type of the source expression is cast compatible with the destination type, that is, either:
— the type of the source expression is a superclass of the destination type, or
— the type of the source expression is an interface class (see 8.26)
and the source is an object that is assignment compatible with the destination type. This type of
assignment requires a run-time check as provided by $cast.
3) The source expression is the literal constant null.
In all other situations $cast shall fail, particularly when the source and destination types are not cast
compatible, even if the source expression evaluates to null.
When $cast succeeds, it performs the assignment. Otherwise, the error handling is as described in 6.24.2.
### 8.17 Chaining constructors
When a subclass is instantiated, the class method new() is invoked. The first action that new() takes, before
any code defined in the function is evaluated, is to invoke the new() method of its superclass and so on up
the inheritance hierarchy. Thus, all the constructors are called, in the proper order, beginning with the root
base class and ending with the current class. Class property initialization occurs during this sequence as
described in 8.7.
If the new() method of the superclass requires arguments, there are two choices: to call the superclass
constructor explicitly from the subclass constructor with super.new() (see 8.15) or to specify the
arguments in the extends specifier (see 8.13), as described below.
The more general approach is to call the superclass constructor explicitly from the subclass constructor:
function new();
super.new(5);
endfunction
To use this approach, super.new(...) shall be the first executable statement in the function new.
The argument list of a subclass constructor may include all the arguments of the superclass constructor by
specifying the default keyword as an argument in the list. The default keyword shall expand to the
argument list of the superclass constructor, with the same declaration order, direction, type, name, and
default values as in the superclass constructor argument list.
The following additional rules apply to the default keyword in a class constructor:
— The class shall be a subclass (i.e., it shall be extending another class).
— The default keyword shall appear no more than once in the constructor argument list.
— No argument in an argument list with the default keyword shall have the same name as an argu-
ment in the superclass constructor.
— If an argument directly follows the default keyword in the argument list, then it shall have the
default direction of input and the default data type of logic (see 13.4).

<!-- Page 194 -->

IEEE Std — It shall be an error to use the default keyword if the default value of a superclass constructor argu-
ment refers to a local member (see 8.18).
The default keyword may also be passed as the sole argument to super.new() inside a class constructor.
When this occurs, all arguments expanded by the default keyword in the class constructor argument list
shall be implicitly passed to the superclass constructor. It shall be an error to use the default keyword for
super.new() if the default keyword was not used in the constructor argument list. If the user code does
not provide a call to super.new() inside a subclass constructor that specifies the default argument, the
compiler shall insert a call to super.new(default) automatically (see 8.15).
The following example illustrates using the default keyword as a constructor argument:
class Base;
string name;
local int m_id;
function new(string name, output int id);
this.name = name;
id = m_id++;
endfunction : new
endclass : Base
// Class A does not add any additional arguments before the default keyword
class A extends Base;
function new(default);
// Compiler automatically calls super.new(default)
endfunction : new
endclass : A
// Class B adds additional arguments before the default keyword
class B extends Base;
int size;
function new(int size, default);
super.new(default); // Optional explicit use of super.new
this.size = size;
endfunction : new
endclass : B
// Class C adds additional arguments after the default keyword
class C extends B;
bit enable;
function new(default, bit enable);
super.new(default); // Optional explicit use of super.new
this.enable = enable; // enable is an input by default
endfunction : new
endclass : C
If the arguments are always the same, then they can be specified in the extends specifier:
class EtherPacket extends Packet(5);
This passes 5 to the new routine associated with Packet.
If the arguments are specified in the extends specifier, the subclass constructor shall not contain a
super.new() call. The compiler shall insert a call to super.new() automatically, as it does whenever the
subclass constructor does not contain a super.new() call.

<!-- Page 195 -->

IEEE Std The default keyword may also be specified as the sole argument in the extends specifier. If no user-
defined new() method is provided for the subclass, then the implicit new() method shall have all of the
arguments of the superclass, as if default were the sole argument. If a user-defined new() method is
provided for the subclass, then the constructor argument list shall contain the default keyword.
As such, classes A and B in the previous example could be rewritten as follows, with the same result:
// Class A does not require an explicit constructor;
// the implicit new() method has the argument list of Base::new.
class A extends Base(default);
endclass : A
// Class B still requires an explicit constructor,
// as additional arguments are provided.
class B extends Base(default);
int size;
function new(int size, default);
// Implicit call to super.new(default) from Base(default)
this.size = size;
endfunction : new
endclass : B
NOTE 1—Declaring a class constructor as a local method makes that class inextensible since the reference to
super.new() in a subclass would be illegal.
NOTE 2—When calling a virtual method from a constructor new(), the constructor calls the method as described in
8.20. However, users need to be aware of the class property initialization sequence as described in 8.7, as properties the
method refers to may not have been initialized, depending on where in the chain of constructors the method was called
from.
### 8.18 Data hiding and encapsulation
In SystemVerilog, unqualified class properties and methods are public, available to anyone who has access
to the object’s name. Often, it is desirable to restrict access to class properties and methods from outside the
class by hiding their names. This keeps other programmers from relying on a specific implementation, and it
also protects against accidental modifications to class properties that are internal to the class. When all data
become hidden (i.e., being accessed only by public methods), testing and maintenance of the code become
much easier.
Class parameters and class local parameters are also public.
A member identified as local is available only to methods inside the class. Further, these local members
are not visible within subclasses. Of course, nonlocal methods that access local class properties or methods
can be inherited and work properly as methods of the subclass.
A protected class property or method has all of the characteristics of a local member, except that it can
be inherited; it is visible to subclasses.
Within a class, a local method or class property of the same class can be referenced, even if it is in a different
instance of the same class. For example:
class Packet;
local integer i;
function integer compare (Packet other);
compare = (this.i == other.i);
endfunction
endclass

<!-- Page 196 -->

IEEE Std A strict interpretation of encapsulation might say that other.i should not be visible inside this packet
because it is a local class property being referenced from outside its instance. Within the same class,
however, these references are allowed. In this case, this.i shall be compared to other.i, and the result of
the logical comparison returned.
Class members can be identified as either local or protected; class properties can be further defined as
const, and methods can be defined as virtual. There is no predefined ordering for specifying these
modifiers; however, they can only appear once per member. It shall be an error to define members to be both
local and protected or to duplicate any of the other modifiers.
### 8.19 Constant class properties
Class properties can be made read-only by a const declaration like any other SystemVerilog variable.
However, because class objects are dynamic objects, class properties allow two forms of read-only
variables: global constants and instance constants.
Global constant class properties include an initial value as part of their declaration. They are similar to other
const variables in that they cannot be assigned a value anywhere other than in the declaration.
class Jumbo_Packet;
const int max_size = 9 * 1024; // global constant
byte payload [];
function new( int size );
payload = new[ size > max_size ? max_size : size ];
endfunction
endclass
Instance constants do not include an initial value in their declaration, only the const qualifier. This type of
constant can be assigned a value at run time, but the assignment can only be done once in the corresponding
class constructor.
class Big_Packet;
const int size; // instance constant
byte payload [];
function new();
size = $urandom % 4096; // one assignment in new -> OK
payload = new[ size ];
endfunction
endclass
Typically, global constants are also declared static because they are the same for all instances of the class.
However, an instance constant cannot be declared static because doing so would disallow all assignments
in the constructor.
### 8.20 Virtual methods
A method of a class may be identified with the keyword virtual. Virtual methods are a basic polymorphic
construct. A virtual method shall override a method in all of its base classes, whereas a non-virtual method
shall only override a method in that class and its descendants. One way to view this is that there is only one
implementation of a virtual method per class hierarchy, and it is always the one in the latest derived class.
Virtual methods provide prototypes for the methods that later override them, i.e., all of the information
generally found on the first line of a method declaration: the encapsulation criteria, the type and number of
arguments, and the return type if it is needed.

<!-- Page 197 -->

IEEE Std Virtual method overrides in subclasses shall have matching argument types, identical argument names,
identical qualifiers, and identical directions to the prototype. The virtual qualifier is optional in the
derived class method declarations. The return type of a virtual function shall be either:
— a matching type (see 6.22.1)
— or a derived class type
of the return type of the virtual function in the superclass. It is not necessary to have matching default
expressions, but the presence of a default shall match.
Example 1 illustrates virtual method override.
Example 1:
class BasePacket;
int A = 1;
int B = 2;
function void printA;
$display("BasePacket::A is %d", A);
endfunction : printA
virtual function void printB;
$display("BasePacket::B is %d", B);
endfunction : printB
endclass : BasePacket
class My_Packet extends BasePacket;
int A = 3;
int B = 4;
function void printA;
$display("My_Packet::A is %d", A);
endfunction: printA
virtual function void printB;
$display("My_Packet::B is %d", B);
endfunction : printB
endclass : My_Packet
BasePacket P1 = new;
My_Packet P2 = new;
initial begin
P1.printA; // displays 'BasePacket::A is 1'
P1.printB; // displays 'BasePacket::B is 2'
P1 = P2; // P1 has a handle to a My_packet object
P1.printA; // displays 'BasePacket::A is 1'
P1.printB; // displays 'My_Packet::B is 4' – latest derived method
P2.printA; // displays 'My_Packet::A is 3'
P2.printB; // displays 'My_Packet::B is 4'
end
Example 2 illustrates the use of a derived class type for a virtual function return type and of matching formal
argument types. In the derived class D, the virtual function return type is D, a derived class type of C. The
formal argument data type is T, which is a matching data type of the predefined type int.
Example 2:
typedef int T; // T and int are matching data types.

<!-- Page 198 -->

IEEE Std class C;
virtual function C some_method(int a); endfunction
endclass
class D extends C;
virtual function D some_method(T a); endfunction
endclass
class E #(type Y = logic) extends C;
virtual function D some_method(Y a); endfunction
endclass
E #() v1; // Illegal: type parameter Y resolves to logic, which is not
// a matching type for argument a
E #(int) v2; // Legal: type parameter Y resolves to int
A virtual method may override a non-virtual method, but once a method has been identified as virtual, it
shall remain virtual in any subclass that overrides it. In that case, the virtual keyword may be used in later
declarations, but is not required.
Since a subclass may override a virtual method without declaring the override virtual, it is possible to
accidentally override a base class virtual method. The initial specifier, preceded by a colon, when applied to
methods, specifies that the method shall not be a virtual override. Specifying initial when overriding a
virtual method within a subclass, including overriding methods declared pure virtual (see 8.21) in the
base class, shall result in an error.
Usage of initial does not cause a method to be virtual. A method that is not declared virtual may use
initial to ensure that it is not accidentally overriding a virtual base class method, whereas a method that is
declared virtual may use initial to ensure that it is the initial declaration of a virtual method in the class
hierarchy.
NOTE—initial only prevents overriding virtual base class methods; overriding a non-virtual base class method does
not result in an error.
Similarly, a subclass may wish to explicitly state that a method is an override of a virtual method in a base
class. The extends specifier, when applied to methods, specifies that the method shall be a virtual override.
Specifying extends when not extending a virtual method from a base class shall result in an error. Unlike
initial, the extends specifier does imply that the method is virtual. The virtual keyword is optional
when using the extends specifier.
A subclass may also wish to prevent any further subclasses in the class hierarchy from overriding a method.
The final specifier, when applied to methods, specifies that no further overrides of the method shall be
allowed. Similar to initial, final does not cause a method to be virtual. An attempt by a subclass to
override a method specified as final in a base class shall result in an error, regardless of whether the
method is declared virtual in either the subclass or the base. It shall be illegal to specify final in a pure
virtual method declaration and shall result in an error.
initial and extends are mutually exclusive; specifying both in a method declaration shall result in an
error. final may be combined with either initial or extends.
Example 3:
class base;
function void f1(); ... endfunction // non-virtual
virtual function void f2(); ... endfunction // virtual
pure virtual function void f3(); // pure virtual

<!-- Page 199 -->

IEEE Std endclass : base
class A extends base;
function :initial void f1(); ... endfunction
// OK: base::f1 is not a virtual method
virtual function :final :extends void f2(); ... endfunction
// OK: f2 shall not be overridden in subclasses of A
function :final void f4();…endfunction
// OK: f4 shall not be overridden in subclasses of A
virtual function :extends void f5(); ... endfunction
// NOT OK: f5 is not a virtual override
endclass : A
class B extends A;
virtual function :initial void f1(); ... endfunction
// OK: A::f1 is not a virtual method
virtual function void f2(); ... endfunction
// NOT OK: f2 is specified final in A
function void f4(); ... endfunction
// NOT OK: A::f4 is specified final
endclass : B
class C extends base;
function :initial void f2(); ... endfunction
// NOT OK: f2 is a virtual override from base::f2
function :initial void f3(); ... endfunction
// NOT OK: f3 is a virtual override from pure virtual base::f3
extern function :initial void f5();
// OK: f5 is not a virtual override
endclass : C
function void C::f5(); ... endfunction
// OK: external method definitions do not declare override specifiers
// (see 8.24)
### 8.21 Abstract classes and pure virtual methods
A set of classes may be created that can be viewed as all being derived from a common base class. For
example, a common base class of type BasePacket that sets out the structure of packets, but is incomplete,
would never be constructed. This is characterized as an abstract class. From this abstract base class,
however, a number of useful subclasses may be derived, such as Ethernet packets, token ring packets, GPS
packets, and satellite packets. Each of these packets might look very similar, all needing the same set of
methods, but they could vary significantly in terms of their internal details.
A base class may be characterized as being abstract by identifying it with the keyword virtual:
virtual class BasePacket;
...
endclass

<!-- Page 200 -->

IEEE Std An object of an abstract class shall not be constructed directly. Its constructor may only be called indirectly
through the chaining of constructor calls originating in an extended non-abstract object.
A virtual method in an abstract class may be declared as a prototype without providing an implementation.
This is called a pure virtual method and shall be indicated with the keyword pure together with not
providing a method body. An extended subclass may provide an implementation by overriding the pure
virtual method with a virtual method having a method body.
Abstract classes may be extended to further abstract classes, but all pure virtual methods shall have
overridden implementations in order to be extended by a non-abstract class. By having implementations for
all its methods, the class is complete and may now be constructed. Any class may be extended into an
abstract class, and may provide additional or overridden pure virtual methods.
virtual class BasePacket;
pure virtual function integer send(bit[31:0] data); // No implementation
endclass
class EtherPacket extends BasePacket;
virtual function integer send(bit[31:0] data);
// body of the function
...
endfunction
endclass
EtherPacket is now a class that can have an object of its type constructed.
NOTE—A method without a statement body is still a legal, callable method. For example, if the function send was
declared as follows, it would have an implementation:
virtual function integer send(bit[31:0] data); // Will return 'x
endfunction
### 8.22 Polymorphism: dynamic method lookup
Polymorphism allows the use of a variable of the superclass type to hold subclass objects and to reference
the methods of those subclasses directly from the superclass variable. As an example, assume the base class
for the Packet objects, BasePacket, defines, as virtual functions, all of the public methods that are to be
generally used by its subclasses. Such methods include send, receive, and print. Even though
BasePacket is abstract, it can still be used to declare a variable:
BasePacket packets[100];
Now, instances of various packet objects can be created and put into the array:
EtherPacket ep = new; // extends BasePacket
TokenPacket tp = new; // extends BasePacket
GPSPacket gp = new; // extends EtherPacket
packets[0] = ep;
packets[1] = tp;
packets[2] = gp;
If the data types were, for example, integers, bits, and strings, all of these types could not be stored into a
single array, but with polymorphism, it can be done. In this example, because the methods were declared as
virtual, the appropriate subclass methods can be accessed from the superclass variable, even though the
compiler did not know—at compile time—what was going to be loaded into it.

<!-- Page 201 -->

IEEE Std For example, packets[1]
packets[1].send();
shall invoke the send method associated with the TokenPacket class. At run time, the system correctly
binds the method from the appropriate class.
This is a typical example of polymorphism at work, providing capabilities that are far more powerful than
what is found in a nonobject-oriented framework.
### 8.23 Class scope resolution operator ::
The class scope resolution operator :: is used to specify an identifier defined within the scope of a class. It
has the following form:
class_type :: { class_type :: } identifier
The left operand of the scope resolution operator :: shall be a class type name, package name (see 26.2),
covergroup type name, coverpoint name, cross name (see 19.5, 19.6), typedef name, or type
parameter name. When a type name is used, the name shall resolve to a class or covergroup type after
elaboration. While incomplete forward types, types defined by an interface-based typedef (see 6.18), and
type parameters (see 6.20.3) may resolve to class types, use of the class scope resolution operator to select a
type with such a prefix shall be restricted to typedef declarations (see 6.18), the type operator (see 6.23),
and type parameter assignments.
Because classes and other scopes can have the same identifiers, the class scope resolution operator uniquely
identifies a member, a parameter or local parameter of a particular class. In addition to disambiguating class
scope identifiers, the :: operator also allows access to static members (class properties and methods), class
parameters, and class local parameters from outside the class, as well as access to public or protected
elements of a superclass from within the derived classes. A class parameter or local parameter is a public
element of a class. A class scoped parameter or local parameter is a constant expression.
class Base;
typedef enum {bin,oct,dec,hex} radix;
static task print( radix r, integer n ); ... endtask
endclass
...
Base b = new;
int bin = 123;
b.print( Base::bin, bin ); // Base::bin and bin are different
Base::print( Base::hex, 66 );
In SystemVerilog, the class scope resolution operator applies to all static elements of a class: static class
properties, static methods, typedefs, enumerations, parameters, local parameters, constraints, structures,
unions, and nested class declarations. Class scope resolved expressions can be read (in expressions), written
(in assignments or subroutines calls), or triggered off (in event expressions). A class scope can also be used
as the prefix of a type or a method call.
Like modules, classes are scopes and can nest. Nesting allows hiding of local names and local allocation of
resources. This is often desirable when a new type is needed as part of the implementation of a class.
Declaring types within a class helps prevent name collisions and the cluttering of the outer scope with
symbols that are used only by that class. Type declarations nested inside a class scope are public and can be
accessed outside the class.

<!-- Page 202 -->

IEEE Std class StringList;
class Node; // Nested class for a node in a linked list.
string name;
Node link;
endclass
endclass
class StringTree;
class Node; // Nested class for a node in a binary tree.
string name;
Node left, right;
endclass
endclass
// StringList::Node is different from StringTree::Node
The class scope resolution operator enables the following:
— Access to static public members (methods and class properties) from outside the class hierarchy.
— Access to public or protected class members of a superclass from within the derived classes.
— Access to constraints, type declarations, and enumeration named constants declared inside the class
from outside the class hierarchy or from within derived classes.
— Access to parameters and local parameters declared inside the class from outside the class hierarchy
or from within derived classes.
Nested classes shall have the same access rights as methods do in the containing class. They have full access
rights to local and protected methods and properties of the containing class. Nested classes have
lexically scoped, unqualified access to the static properties and methods, parameters, and local
parameters of the containing class. They shall not have implicit access to non-static properties and methods
except through a handle either passed to it or otherwise accessible by it. There is no implicit this handle to
the outer class. For example:
class Outer;
int outerProp;
local int outerLocalProp;
static int outerStaticProp;
static local int outerLocalStaticProp;
class Inner;
function void innerMethod(Outer h);
outerStaticProp = 0;
// Legal, same as Outer::outerStaticProp
outerLocalStaticProp = 0;
// Legal, nested classes may access local's in outer class
outerProp = 0;
// Illegal, unqualified access to non-static outer
h.outerProp = 0;
// Legal, qualified access.
h.outerLocalProp = 0;
// Legal, qualified access and locals to outer class allowed.
endfunction
endclass
endclass
The class scope resolution operator has special rules when used with a prefix that is the name of a
parameterized class; see 8.25.1 for details.

<!-- Page 203 -->

IEEE Std ### 8.24 Out-of-block declarations
It is convenient to be able to move method definitions out of the body of the class declaration. This is done
in two steps. First, within the class body, declare the method prototypes, i.e., whether it is a function or task,
any qualifiers (local, protected, or virtual), any specifiers (initial, extends, or final), and the
full argument specification plus the extern qualifier. The extern qualifier indicates that the body of the
method (its implementation) is to be found outside the declaration. Second, outside the class declaration,
declare the full method (e.g., the prototype but without the qualifiers and specifiers), and, to tie the method
back to its class, qualify the method name with the class name and a pair of colons, as follows:
class Packet;
Packet next;
function Packet get_next();// single line
get_next = next;
endfunction
// out-of-body (extern) declaration
extern protected virtual function int send(int value);
endclass
function int Packet::send(int value);
// dropped protected virtual, added Packet::
// body of method
...
endfunction
The out-of-block method declaration shall match the prototype declaration exactly, with the following
exceptions:
— The method name is preceded by the class name and the class scope resolution operator.
— A function return type may also require the addition of a class scope in the out-of-block declaration,
as described below.
— A default argument value specified in the prototype may be omitted in the out-of-block declaration.
If a default argument value is specified in the out-of-block declaration, then there shall be a
syntactically identical default argument value specified in the prototype.
An out-of-block declaration shall be declared in the same scope as the class declaration and shall follow the
class declaration. It shall be an error if more than one out-of-block declaration is provided for a particular
extern method.
The class scope resolution operator is required in some situations in order to name the return type of a
method with an out-of-block declaration. When the return type of the out-of-block declaration is defined
within the class, the class scope resolution operator shall be used to indicate the internal return type.
Example:
typedef real T;
class C;
typedef int T;
extern function T f();
extern function real f2();
endclass
function C::T C::f();// the return needs to use the class scope resolution
// operator, since the type is defined within the class

<!-- Page 204 -->

IEEE Std return 1;
endfunction
function real C::f2();
return 1.0;
endfunction
An out-of-block method declaration shall be able to access all declarations of the class in which the
corresponding prototype is declared. Following normal resolution rules, the prototype has access to class
types only if they are declared prior to the prototype. It shall be an error if an identifier referenced in the
prototype does not resolve to the same declaration as the declaration resolved for the corresponding
identifier in the out-of-block method declaration’s header.
Example:
typedef int T;
class C;
extern function void f(T x);
typedef real T;
endclass
function void C::f(T x);
endfunction
In this example, identifier T in the prototype for method f resolves to the declaration of T in the outer scope.
In the out-of-block declaration for method f the identifier T resolves to C::T since the out-of-block
declaration has visibility to all types in class C. Since the resolution of T in the out-of-block declaration does
not match the resolution in the prototype, an error shall be reported.
### 8.25 Parameterized classes
It is often useful to define a generic class whose objects can be instantiated to have different array sizes or
data types. This avoids writing similar code for each size or type and allows a single specification to be used
for objects that are fundamentally different and (like a templated class in C++) not interchangeable.
The SystemVerilog parameter mechanism is used to parameterize a class:
class vector #(int size = 1);
bit [size-1:0] a;
endclass
Instances of this class can then be instantiated like modules or interfaces, using the same parameter override
rules (see 23.10):
vector #(10) vten; // object with vector of size 10
vector #(.size(2)) vtwo; // object with vector of size 2
typedef vector#(4) Vfour; // Class with vector of size 4
This feature is particularly useful when using types as parameters:
class stack #(type T = int);
local T items[];
task push(T a); ... endtask
task pop(ref T a); ... endtask
endclass

<!-- Page 205 -->

IEEE Std The preceding class defines a generic stack class, which can be instantiated with any arbitrary type:
stack is; // default: a stack of ints
stack#(bit[1:10]) bs; // a stack of 10-bit vectors
stack#(real) rs; // a stack of real numbers
Any type can be supplied as a parameter, including a user-defined type such as a class or struct.
The combination of a generic class and the actual parameter values is called a specialization. Each
specialization of a class has a separate set of static member variables (this is consistent with C++
templated classes). To share static member variables among several class specializations, they need to be
placed in a nonparameterized base class.
class vector #(int size = 1);
bit [size-1:0] a;
static int count = 0;
function void disp_count();
$display("count: %d of size %d", count, size);
endfunction
endclass
Each specialization of the class vector in the preceding example has its own unique copy of count. The
method disp_count can only access the variable count from the same specialization.
A specialization is the combination of a specific generic class with a unique set of parameters. Two sets of
parameters shall be unique unless all parameters are the same, as defined by the following rules:
a) A parameter is a type parameter and the two types are matching types.
b) A parameter is a value parameter, their value typesare matching types, and their values are the
same.
All matching specializations of a particular generic class shall represent the same type. The set of matching
specializations of a generic class is defined by the context of the class declaration. Because generic classes
in a package are visible throughout the system, all matching specializations of a package generic class are
the same type. In other contexts, such as modules or programs, each instance of the scope containing the
generic class declaration creates a unique generic class, thus defining a new set of matching specializations.
A generic class is not a type; only a concrete specialization represents a type. In the preceding example, the
class vector becomes a concrete type only when it has had parameters applied to it, for example:
typedef vector my_vector; // use default size of 1
vector#(6) vx; // use size 6
To avoid having to repeat the specialization either in the declaration or to create parameters of that type, a
typedef should be used:
typedef vector#(4) Vfour;
typedef stack#(Vfour) Stack4;
Stack4 s1, s2; // declare objects of type Stack4
A parameterized class can extend another parameterized class. For example:
class C #(type T = bit); ... endclass // base class
class D1 #(type P = real) extends C; // T is bit (the default)
class D2 #(type P = real) extends C #(integer); // T is integer
class D3 #(type P = real) extends C #(P); // T is P

<!-- Page 206 -->

IEEE Std class D4 #(type P = C#(real)) extends P; // for default, T is real
Class D1 extends the base class C using the base class’s default type (bit) parameter. Class D2 extends the
base class C using an integer parameter. Class D3 extends the base class C using the parameterized type (P)
with which the extended class is parameterized. Class D4 extends the base class specified by the type
parameter P.
When a type parameter or typedef name is used as a base class, as in class D4 above, the name shall resolve
to a class type after elaboration.
The default specialization of a parameterized class is the specialization of the parameterized class with an
empty parameter override list. For a parameterized class C, the default specialization is C#(). Other than as
the prefix of the scope resolution operator, use of the unadorned name of a parameterized class shall denote
the default specialization of the class. Not all parameterized classes have a default specialization since it is
legal for a class to not provide parameter defaults. In that case all specializations shall override at least those
parameters with no defaults.
Example:
class C #(int p = 1);
...
endclass
class D #(int p);
...
endclass
C obj; // legal; equivalent to "C#() obj";
D obj; // illegal; D has no default specialization
#### 8.25.1 Class scope resolution operator for parameterized classes
Use of the class scope resolution operator with a prefix that is the unadorned name of a parameterized class
(see 8.25) shall be restricted to use within the scope of the named parameterized class and within its out-of-
block declarations (see 8.24). In such cases, the unadorned name of the parameterized class does not denote
the default specialization but is used to unambiguously refer to members of the parameterized class. When
referring to the default specialization as the prefix to the class scope resolution operator, the explicit default
specialization form of #() shall be used.
Outside the context of a parameterized class or its out-of-block declarations, the class scope resolution
operator may be used to access any of the class parameters. In such a context, the explicit specialization
form shall be used; the unadorned name of the parameterized class shall be illegal. The explicit
specialization form may denote a specific parameter or the default specialization form. The class scope
resolution operator may access value as well as type parameters that are either local or parameters to the
class.
Example:
class C #(int p = 1);
parameter int q = 5; // local parameter
static task t;
int p;
int x = C::p; // C::p disambiguates p
// C::p is not p in the default specialization
endtask
endclass

<!-- Page 207 -->

IEEE Std int x = C::p; // illegal; C:: is not permitted in this context
int y = C#()::p; // legal; refers to parameter p in the default
// specialization of C
typedef C T; // T is a default specialization, not an alias to
// the name "C"
int z = T::p; // legal; T::p refers to p in the default specialization
int v = C#(3)::p; // legal; parameter p in the specialization of C#(3)
int w = C#()::q; // legal; refers to the local parameter
T obj = new();
int u = obj.q; // legal; refers to the local parameter
bit arr[obj.q]; // illegal: local parameter is not a constant expression
In the context of a parameterized class method out-of-block declaration, use of the class scope resolution
operator shall be a reference to the name as though it was made inside the parameterized class; no
specialization is implied.
Example:
class C #(int p = 1, type T = int);
extern static function T f();
endclass
function C::T C::f();
return p + C::p;
endfunction
initial $display("%0d %0d", C#()::f(),C#(5)::f());// output is "2 10"
### 8.26 Interface classes
A set of classes may be created that can be viewed as all having a common set of behaviors. Such a common
set of behaviors may be created using interface classes. An interface class makes it unnecessary for related
classes to share a common abstract superclass or for that superclass to contain all method definitions needed
by all subclasses. A non-interface class can be declared as implementing one or more interface classes. This
creates a requirement for the non-interface class to provide implementations for a set of methods that shall
satisfy the requirements of a virtual method override (see 8.20).
An interface class shall only contain pure virtual methods (see 8.21), type declarations (see 6.18), and
parameter declarations (see 6.20, 8.25). Constraint blocks, covergroups, and nested classes (see 8.23) shall
not be allowed in an interface class. An interface class shall not be nested within another interface class. An
interface class can inherit from one or more interface classes through the extends keyword, meaning that it
inherits all the member types, pure virtual methods and parameters of the interface classes it extends, except
for any member types and parameters that it may hide. In the case of multiple inheritance, name conflicts
may occur that have to be resolved (see 8.26.6).
Classes can implement one or more interface classes through the implements keyword. No member types
or parameters are inherited through the implements keyword. A subclass implicitly implements all of the
interface classes implemented by its superclass. In the following example, class C implicitly implements
interface class A and has all of the requirements and capabilities as if it explicitly implemented interface
class A:
interface class A;
endclass

<!-- Page 208 -->

IEEE Std class B implements A;
endclass
class C extends B;
endclass
Each pure virtual method from an interface class shall have a virtual method implementation in order to be
implemented by a non-abstract class. When an interface class is implemented by a class, the required
implementations of interface class methods may be provided by inherited virtual method implementations.
A virtual class shall define or inherit a pure virtual method prototype or virtual method
implementation for each pure virtual method prototype in each implemented interface class. The
keyword virtual shall be used unless the virtual method is inherited.
A variable whose declared type is an interface class type may have as its value a reference to any instance of
a class that implements the specified interface class (see 8.22). It is not sufficient that a class provides
implementations for all the pure virtual methods of an interface class; the class or one of its superclasses
shall be declared to implement the interface class through the implements keyword, or else the class does
not implement the interface class.
The following is a simple example of interface classes.
interface class PutImp#(type PUT_T = logic);
pure virtual function void put(PUT_T a);
endclass
interface class GetImp#(type GET_T = logic);
pure virtual function GET_T get();
endclass
class Fifo#(type T = logic, int DEPTH=1) implements PutImp#(T), GetImp#(T);
T myFifo [$:DEPTH-1];
virtual function void put(T a);
myFifo.push_back(a);
endfunction
virtual function T get();
get = myFifo.pop_front();
endfunction
endclass
class Stack#(type T = logic, int DEPTH=1) implements PutImp#(T), GetImp#(T);
T myFifo [$:DEPTH-1];
virtual function void put(T a);
myFifo.push_front(a);
endfunction
virtual function T get();
get = myFifo.pop_front();
endfunction
endclass
The example has two interface classes, PutImp and GetImp, which contain prototype pure virtual methods
put and get. The Fifo and Stack classes use the keyword implements to implement the PutImp and
GetImp interface classes and they provide implementations for put and get. These classes therefore share
common behaviors without sharing a common implementation.

<!-- Page 209 -->

IEEE Std #### 8.26.1 Interface class syntax
interface_class_declaration ::= // from A.1.2
interface class class_identifier [ parameter_port_list ]
[ extends interface_class_type { , interface_class_type } ] ;
{ interface_class_item }
endclass [ : class_identifier ]
interface_class_item ::= // from A.1.9
type_declaration
| { attribute_instance } interface_class_method
| local_parameter_declaration ;
| parameter_declaration7 ;
| ;
interface_class_method ::= pure virtual method_prototype ;
7) In a parameter_declaration that is a class_item, the parameter keyword shall be a synonym for the
localparam keyword.
Syntax8-3—Interface class syntax (excerpt from AnnexA)
#### 8.26.2 Extends versus implements
Conceptually extends is a mechanism to add to or modify the behavior of a superclass while implements
is a requirement to provide implementations for the pure virtual methods in an interface class. When a class
is extended, all members of the class are inherited into the subclass. When an interface class is implemented,
nothing is inherited.
An interface class may extend, but not implement, one or more interface classes, meaning that the interface
subclass inherits members from multiple interface classes and may add additional member types, pure
virtual method prototypes, and parameters. A class or virtual class may implement, but not extend, one or
more interface classes. Because virtual classes are abstract, they are not required to fully define the methods
from their implemented classes (see 8.26.7). The following highlights these differences:
— An interface class
• may extend zero or more interface classes
• may not implement an interface class
• may not extend a class or virtual class
• may not implement a class or virtual class
— A class or virtual class
• may not extend an interface class
• may implement zero or more interface classes
• may extend at most one other class or virtual class
• may not implement a class or virtual class
• may simultaneously extend a class and implement interface classes
In the following example, a class is both extending a base class and implementing two interface classes:
interface class PutImp#(type PUT_T = logic);
pure virtual function void put(PUT_T a);
endclass

<!-- Page 210 -->

IEEE Std interface class GetImp#(type GET_T = logic);
pure virtual function GET_T get();
endclass
class MyQueue #(type T = logic, int DEPTH = 1);
T PipeQueue[$:DEPTH-1];
virtual function void deleteQ();
PipeQueue.delete();
endfunction
endclass
class Fifo #(type T = logic, int DEPTH = 1)
extends MyQueue#(T, DEPTH)
implements PutImp#(T), GetImp#(T);
virtual function void put(T a);
PipeQueue.push_back(a);
endfunction
virtual function T get();
get = PipeQueue.pop_front();
endfunction
endclass
In this example, the PipeQueue property and deleteQ method are inherited in the Fifo class. In addition
the Fifo class is also implementing the PutImp and GetImp interface classes so it shall provide
implementations for the put and get methods, respectively.
The following example demonstrates that multiple types can be parameterized in the class definition and the
resolved types used in the implemented classes PutImp and GetImp.
virtual class XFifo#(type T_in = logic, type T_out = logic, int DEPTH = 1)
extends MyQueue#(T_out)
implements PutImp#(T_in), GetImp#(T_out);
pure virtual function T_out translate(T_in a);
virtual function void put(T_in a);
PipeQueue.push_back(translate(a));
endfunction
virtual function T_out get();
get = PipeQueue.pop_front();
endfunction
endclass
An inherited virtual method can provide the implementation for a method of an implemented interface class.
Here is an example:
interface class IntfClass;
pure virtual function bit funcBase();
pure virtual function bit funcExt();
endclass
class BaseClass;
virtual function bit funcBase();
return (1);
endfunction
endclass
class ExtClass extends BaseClass implements IntfClass;
virtual function bit funcExt();
return (0);

<!-- Page 211 -->

IEEE Std endfunction
endclass
ExtClass fulfills its requirement to implement IntfClass by providing an implementation of funcExt
and by inheriting an implementation of funcBase from BaseClass.
An inherited non-virtual method does not provide an implementation for a method of an implemented
interface class.
interface class IntfClass;
pure virtual function void f();
endclass
class BaseClass;
function void f();
$display("Called BaseClass::f()");
endfunction
endclass
class ExtClass extends BaseClass implements IntfClass;
virtual function void f();
$display("Called ExtClass::f()");
endfunction
endclass
The non-virtual function f() in BaseClass does not fulfill the requirement to implement IntfClass. The
implementation of f() in ExtClass simultaneously hides the f() of BaseClass and fulfills the
requirement to implement IntfClass.
#### 8.26.3 Type access
Parameters and typedefs within an interface class are inherited by extending interface classes, but are not
inherited by implementing interface classes. All parameters and typedefs within an interface class are static
and can be accessed through the class scope resolution operator :: (see 8.23). Accessing parameters through
an interface class handle has the same restrictions as accessing parameters through a class handle (see 8.5).
Example 1: Types and parameter declarations are inherited by extends.
interface class IntfA #(type T1 = logic);
typedef T1[1:0] T2;
pure virtual function T2 funcA();
endclass : IntfA
interface class IntfB #(type T = bit) extends IntfA #(T);
pure virtual function T2 funcB(); // legal, type T2 is inherited
endclass : IntfB
Example 2: Type and parameter declarations are not inherited by implements and need to be specified with
the class scope resolution operator.
interface class IntfC;
typedef enum {ONE, TWO, THREE} t1_t;
pure virtual function t1_t funcC();
endclass : IntfC
class ClassA implements IntfC;
t1_t t1_i; // error, t1_t is not inherited from IntfC

<!-- Page 212 -->

IEEE Std virtual function IntfC::t1_t funcC(); // correct
return (IntfC::ONE); // correct
endfunction : funcC
endclass : ClassA
#### 8.26.4 Type usage restrictions
A class shall not implement a type parameter, nor shall an interface class extend a type parameter, even if the
type parameter resolves to an interface class. The following examples illustrate this restriction and are
illegal:
class Fifo #(type T = PutImp) implements T;
virtual class Fifo #(type T = PutImp) implements T;
interface class Fifo #(type T = PutImp) extends T;
A class shall not implement a forward typedef for an interface class. An interface class shall not extend from
a forward typedef of an interface class. An interface class shall be declared before it is implemented or
extended.
typedef interface class IntfD;
class ClassB implements IntfD #(bit); // illegal
virtual function bit[1:0] funcD();
endclass : ClassB
// This interface class declaration needs to be declared before ClassB
interface class IntfD #(type T1 = logic);
typedef T1[1:0] T2;
pure virtual function T2 funcD();
endclass : IntfD
#### 8.26.5 Casting and object reference assignment
It shall be legal to assign an object handle to a variable of an interface class type that the object implements.
class Fifo #(type T = int) implements PutImp#(T), GetImp#(T);
endclass
Fifo#(int) fifo_obj = new;
PutImp#(int) put_ref = fifo_obj;
It shall be legal to dynamically cast between interface class variables if the actual class handle is valid to
assign to the destination.
GetImp#(int) get_ref;
Fifo#(int) fifo_obj = new;
PutImp#(int) put_ref = fifo_obj;
$cast(get_ref, put_ref);
In the preceding, put_ref is an instance of Fifo#(int) that implements GetImp#(int). It shall also be
legal to cast from an object handle to an interface class type handle if the actual object implements the
interface class type.
$cast(fifo_obj, put_ref); // legal
$cast(put_ref, fifo_obj); // legal, but casting is not required
Like abstract classes, an object of an interface class type shall not be constructed.

<!-- Page 213 -->

IEEE Std put_ref = new(); // illegal
Casting from a source interface class handle that is null is handled in the same manner as casting from a
source class handle that is null (see 8.16).
#### 8.26.6 Name conflicts and resolution
When a class implements multiple interface classes, or when an interface class extends multiple interface
classes, identifiers are merged from different name spaces into a single name space. When this occurs, it is
possible that the same identifier name from multiple name spaces may be simultaneously visible in a single
name space creating a name conflict that has to be resolved.
8.26.6.1 Method name conflict resolution
It is possible that an interface class may inherit multiple methods, or a class may be required through
implements to provide an implementation of multiple methods, where these methods have the same name.
This is a method name conflict. A method name conflict shall be resolved with a single method prototype or
implementation that simultaneously provides an implementation for all pure virtual methods of the same
name of any implemented interface class. That method prototype or implementation shall also be a valid
virtual method override (see 8.20) for any inherited method of the same name.
Example:
interface class IntfBase1;
pure virtual function bit funcBase();
endclass
interface class IntfBase2;
pure virtual function bit funcBase();
endclass
virtual class ClassBase;
pure virtual function bit funcBase();
endclass
class ClassExt extends ClassBase implements IntfBase1, IntfBase2;
virtual function bit funcBase();
return (0);
endfunction
endclass
Class ClassExt provides an implementation of funcBase that overrides the pure virtual method prototype
from ClassBase and simultaneously provides an implementation for funcBase from both IntfBase1 and
IntfBase2.
There are cases in which a method name conflict cannot be resolved.
Example:
interface class IntfBaseA;
pure virtual function bit funcBase();
endclass
interface class IntfBaseB;
pure virtual function string funcBase();
endclass

<!-- Page 214 -->

IEEE Std class ClassA implements IntfBaseA, IntfBaseB;
virtual function bit funcBase();
return (0);
endfunction
endclass
In this case, funcBase is prototyped in both IntfBaseA and IntfBaseB but with different return types,
bit and string respectively. Although the implementation of funcBase is a valid override of
IntfBaseA::funcBase, it is not simultaneously a valid override of the prototype of IntfBaseB::
funcBase, so an error shall occur.
8.26.6.2 Parameter and type declaration inheritance conflicts and resolution
Interface classes may inherit parameters and type declarations from multiple interface classes. A name
collision will occur if the same name is inherited from different interface classes. The subclass shall provide
parameter and/or type declarations that override all such name collisions.
Example:
interface class PutImp#(type T = logic);
pure virtual function void put(T a);
endclass
interface class GetImp#(type T = logic);
pure virtual function T get();
endclass
interface class PutGetIntf#(type TYPE = logic)
extends PutImp#(TYPE), GetImp#(TYPE);
typedef TYPE T;
endclass
In the preceding example, the parameter T is inherited from both PutImp and GetImp. A conflict occurs
despite the fact that PutImp::T matches GetImp::T and is never used by PutGetIntf. PutGetIntf
overrides T with a type definition to resolve the conflict.
8.26.6.3 Diamond relationship
A diamond relationship occurs if an interface class is implemented by the same class or inherited by the
same interface class in multiple ways. In the case of a diamond relationship, only one copy of the symbols
from any single interface class will be merged so as to avoid a name conflict. For example:
interface class IntfBase;
parameter SIZE = 64;
endclass
interface class IntfExt1 extends IntfBase;
pure virtual function bit funcExt1();
endclass
interface class IntfExt2 extends IntfBase;
pure virtual function bit funcExt2();
endclass
interface class IntfExt3 extends IntfExt1, IntfExt2;
endclass

<!-- Page 215 -->

IEEE Std In the preceding example, the class IntfExt3 inherits the parameter SIZE from IntfExt1 and IntfExt2.
Since these parameters originate from the same interface class, IntfBase, only one copy of SIZE shall be
inherited into IntfExt3 so it shall not be considered a conflict.
Each unique parameterization of a parameterized interface class is an interface class specialization. Each
interface class specialization is considered as though it is a unique interface class type. Therefore, there is no
diamond relationship if different specializations of the same parameterized interface class are inherited by
the same interface class or implemented by the same class. As a result, method name conflicts as described
in 8.26.6.1 and parameter and type declaration name conflicts as described in 8.26.6.2 may occur. For
example:
interface class IntfBase #(type T = int);
pure virtual function bit funcBase();
endclass
interface class IntfExt1 extends IntfBase#(bit);
pure virtual function bit funcExt1();
endclass
interface class IntfExt2 extends IntfBase#(logic);
pure virtual function bit funcExt2();
endclass
interface class IntfFinal extends IntfExt1, IntfExt2;
typedef bit T; // Override the conflicting identifier name
pure virtual function bit funcBase();
endclass
In the preceding example, there are two different parameterizations of the interface class IntfBase. Each of
these parameterizations of IntfBase is a specialization; therefore there is no diamond relationship and
there are conflicts of the parameter T and method funcBase that have to be resolved.
#### 8.26.7 Partial implementation
It is possible to create classes that are not fully defined and that take advantage of interface classes through
the use of virtual classes (see 8.21). Because virtual classes do not have to fully define their implementation,
they are free to partially define their methods. The following is an example of a partially implemented
virtual class.
interface class IntfClass;
pure virtual function bit funcA();
pure virtual function bit funcB();
endclass
// Partial implementation of IntfClass
virtual class ClassA implements IntfClass;
virtual function bit funcA();
return (1);
endfunction
pure virtual function bit funcB();
endclass
// Complete implementation of IntfClass
class ClassB extends ClassA;
virtual function bit funcB();
return (1);
endfunction
endclass

<!-- Page 216 -->

IEEE Std It shall be illegal to use an interface class to partially define a virtual class without fulfilling the interface
class prototype requirements. In other words, when an interface class is implemented by a virtual class, the
virtual class shall do one of the following for each interface class method prototype:
— Provide a method implementation
— Re-declare the method prototype with the pure qualifier
In the preceding example ClassA fully defines funcA, but re-declares the prototype funcB.
#### 8.26.8 Method default argument values
Method declarations within interface classes may have default argument values. The default expression shall
be a constant expression and is evaluated in the scope containing the subroutine declaration. The value of the
constant expression shall be the same for all the classes that implement the method. See 13.5.3 for more
information.
#### 8.26.9 Constraint blocks, covergroups, and randomization
Constraint blocks and covergroups shall not be declared in interface classes.
A randomize() method call shall be legal with interface class handles. While inline constraints shall also
be legal, interface classes cannot contain any data meaning that inline constraints will only be able to
express conditions related to state variables and are therefore of very limited utility. Use of rand_mode and
constraint_mode shall not be legal as a consequence of the name resolution rules and the fact that
interface classes are not permitted to contain data members.
Interface classes contain two built-in empty virtual methods pre_randomize() and post_randomize()
that are automatically called before and after randomization. These methods can be overridden. As a special
case, pre_randomize() and post_randomize() shall not cause method name conflicts.
### 8.27 Typedef class
Sometimes a class variable needs to be declared before the class itself has been declared; for example, if two
classes each need a handle to the other. When, in the course of processing the declaration for the first class,
the compiler encounters the reference to the second class, that reference is undefined and the compiler flags
it as an error.
This is resolved using typedef to provide a forward declaration for the second class:
typedef class C2; // C2 is declared to be of type class
class C1;
C2 c;
endclass
class C2;
C1 c;
endclass
In this example, C2 is declared to be of type class, a fact that is reinforced later in the source code. The
class construct always creates a type and does not require a typedef declaration for that purpose (as in
typedef class …).
In the preceding example, the class keyword in the statement typedef class C2; is not necessary and
is used only for documentation purposes. The statement typedef C2; is equivalent and shall work the
same way.

<!-- Page 217 -->

IEEE Std As with other forward typedefs as described in 6.18, the actual class definition of a forward class declaration
shall be resolved within the same local scope or generate block.
A forward typedef to a class may refer to a class with a parameter port list.
Example:
typedef class C ;
module top ;
C#(1, real) v2 ; // positional parameter override
C#(.p(2), .T(real)) v3 ; // named parameter override
endmodule
class C #(parameter p = 2, type T = int);
endclass
### 8.28 Classes and structures
On the surface, it might appear that class and struct provide equivalent functionality, and only one of
them is needed. However, that is not true; class differs from struct in the following three fundamental
ways:
a) SystemVerilog structs are strictly static objects; they are created either in a static memory location
(global or module scope) or on the stack of an automatic task. Conversely, SystemVerilog objects
(i.e., class instances) are exclusively dynamic; their declaration does not create the object. Creating
an object is done by calling new.
b) SystemVerilog objects are implemented using handles, thereby providing C-like pointer
functionality. But SystemVerilog disallows casting handles onto other data types; thus,
SystemVerilog handles do not have the risks associated with C pointers.
c) SystemVerilog objects form the basis of an object-oriented data abstraction that provides true
polymorphism. Class inheritance, abstract classes, and dynamic casting are powerful mechanisms,
which go way beyond the mere encapsulation mechanism provided by structs.
### 8.29 Memory management
Memory for objects, strings, and dynamic and associative arrays is allocated dynamically. When objects are
created, SystemVerilog allocates more memory. When an object is no longer needed, SystemVerilog
automatically reclaims the memory, making it available for reuse. The automatic memory management
system is an integral part of SystemVerilog. Without automatic memory management, SystemVerilog’s
multithreaded, reentrant environment creates many opportunities for users to run into problems. A manual
memory management system, such as the one provided by C’s malloc and free, would not be sufficient.
Consider the following example:
myClass obj = new;
fork
task1(obj);
task2(obj);
join_none
In this example, the main process (the one that forks off the two tasks) does not know when the two
processes might be done using the object obj. Similarly, neither task1 nor task2 knows when any of the
other two processes will no longer be using the object obj. It is evident from this simple example that no

<!-- Page 218 -->

IEEE Std single process has enough information to determine when it is safe to free the object. The only two options
available to the user are as follows:
— Play it safe and never reclaim the object, or
— Add some form of reference count that can be used to determine when it might be safe to reclaim the
object.
Adopting the first option can cause the system to quickly run out of memory. The second option places a
large burden on users, who, in addition to managing their testbench, need also to manage the memory using
less than ideal schemes. To avoid these shortcomings, SystemVerilog manages all dynamic memory
automatically.
Users do not need to worry about dangling references, premature deallocation, or memory leaks for objects
within SystemVerilog. In the preceding example, users assign null to all variables referencing handle obj
when they no longer need it. The system shall automatically reclaim any object that is no longer being used.
As in Java, SystemVerilog uses different levels of reachability to reflect the life cycle of an object:
— An object is strongly reachable if there are class handles referring to that object in any active scope,
or if there are pending nonblocking assignments to non-static members of that object. A newly cre-
ated object is strongly reachable by the process that created it.
— An object is weakly reachable if it is not strongly reachable, but can be reached by traversing a weak
reference (see 8.30).
— An object is unreachable when it is not reachable in any of the above ways.
The term garbage collection refers to the automatic memory management system transitioning an object’s
state from strongly reachable to unreachable. Depending on the presence of weak references, an object’s
state may transition to weakly reachable during garbage collection. The memory associated with an object is
eligible for reclamation once the object becomes unreachable.
Under certain circumstances, such as when crossing the language boundary, a user may wish to maintain a
reference to an object without preventing the object from being reclaimed, or to perform actions after
reclamation has occurred. To accomplish this, weak references can be used.
### 8.30 Weak references
#### 8.30.1 Overview
weak_reference is a built-in parameterized class that allows access to an object while not preventing
garbage collection. Users can declare variables of type weak_reference#(T) and safely pass them
through functions and tasks, or incorporate them into other objects. The parameter type T shall be a class
type; all other types shall result in a compiler error.
If the memory management system determines that a referent is weakly reachable (see 8.29), then it shall
clear all weak references to that referent. That is, the referent can be garbage collected. The clearing
operation shall be performed atomically.
An example of creating a weak reference is as follows:
typedef class my_obj;
weak_reference#(my_obj) weak_obj;
The weak reference class provides the following methods:
— Create a new weak reference: new()

<!-- Page 219 -->

IEEE Std — Query the referent of the weak reference: get()
— Clear the referent of the weak reference: clear()
— Query the identification value for an object: get_id()
Instances of the weak_reference class are treated the same as instances of all other classes with regards to
garbage collection. When a weak_reference instance is no longer strongly reachable, the instance is
eligible for garbage collection, even if the referent has yet to be garbage collected.
The weak_reference class resides in the built-in std package (see 26.7); thus, it can be redefined by user
code in any other scope.
#### 8.30.2 New()
Weak references are created with the new() method.
The prototype for new() is as follows:
function new(T referent);
If the referent value passed to the constructor is null, the method issues no warning.
Each weak_reference instance is a unique object. As such, two weak_reference instances pointing to
the same referent cannot be directly compared:
obj strong_obj = new();
weak_reference#(obj) weak1 = new(strong_obj);
weak_reference#(obj) weak2 = new(strong_obj);
a1: assert(weak1 != weak2); // weak1 is a different object from weak2
a2: assert(weak1.get() == weak2.get()) // ... but the referent is the same
#### 8.30.3 Get()
The weak reference get() method is used to query the referent object.
The prototype for get() is as follows:
function T get();
If this reference has not yet been cleared, then get() shall return the referent value used to initialize the
weak reference. Otherwise, it shall return null.
NOTE—Querying a weak reference via get() and then storing the result in a strongly reachable class handle makes
the referent strongly reachable. For example:
obj strong_obj = new(); // strong_obj is a strong reference
weak_reference#(obj) weak_obj = new(strong_obj);
// weak_obj weakly references strong_obj
obj get_obj = weak_obj.get(); // get_obj is a strong reference!
The get() function may be used in the context of an event control expression (see 9.4.2) to detect that a
weak reference has been cleared, either by the user (see 8.30.4) or by the memory management system:
wait (weak_obj.get() == null);
NOTE—As the memory management system is implementation dependent, the user should not rely on specific timing
for it to clear weak references.

<!-- Page 220 -->

IEEE Std #### 8.30.4 Clear()
The weak reference clear() function is used to clear the weak reference.
The prototype for clear() is as follows:
function void clear();
Clearing a weak reference sets the return value of get() to null (see 8.30.3).
#### 8.30.5 Get_id()
The static get_id() function returns an identification value for the referent object.
The prototype for get_id() is as follows:
static function longint get_id(T obj);
If the obj is null, then get_id() shall return 0; otherwise it shall return a nonzero value. The value shall
be deterministic, in that subsequent calls to get_id() for the same object instance shall return the same
value, regardless of the class handle’s location in the inheritance tree (see 8.16). For example:
typedef weak_reference#(obj) obj_ref;
typedef weak_reference#(ex_obj) ex_obj_ref; // ex_obj extends obj
ex_obj my_obj = new();
a1: assert(obj_ref::get_id(my_obj) == ex_obj_ref::get_id(my_obj));
// referent ID is the same
The identification value returned by get_id() shall be unique with respect to all other objects, regardless
of type, for the lifetime of the object instance. After an object has been garbage collected, its identification
value may be reused.
This identification value may be used as an index to an associative array (see 7.8) to prevent the indexes of
the associative array from acting as strong references (see 8.29). However, care needs to be taken to ensure
that the value is removed from the associative array when the object is garbage collected, as the
identification value may be reused for new objects.

<!-- Page 221 -->

IEEE Std 1800-2023