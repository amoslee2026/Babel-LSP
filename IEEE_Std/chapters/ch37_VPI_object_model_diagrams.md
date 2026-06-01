# Chapter 37: VPI object model diagrams

## 37. VPI object model diagrams
### 37.1 General
This clause describes the following:
— Using VPI data models
— VPI data model diagrams
### 37.2 VPI handles
A handle is an opaque reference to an object in the VPI information model. It is represented as a value of the
data type vpiHandle (see AnnexK); however, the interpretation of the representation is implementation
defined. A handle allows a VPI program to refer to an object without assuming details of the representation
of the object. The VPI provides functions that operate on objects referred to by handles. The particular
operations that are legal for an object referred to by a handle depend on the type of the object.
#### 37.2.1 Handle creation
A handle is created by a tool as the result of one of the following functions called by a VPI application
program:
a) vpi_handle(), which returns a handle that refers to an object in a one-to-one relationship
b) vpi_handle_by_index(), which returns a handle that refers to an object in an ordered, one-to-many
relationship using an index
c) vpi_handle_by_multi_index(), which returns a handle that refers to an indexed subobject of a
multidimensional parent object using an array of indices
d) vpi_handle_by_name(), which returns a handle that refers to an object identified by a specific name
e) vpi_handle_multi(), which returns a handle to an object in a many-to-one relationship
f) vpi_iterate(), which returns a handle to an iterator object for scanning a one-to-many relationship
g) vpi_put_value(), which returns a handle to a scheduled event object
h) vpi_register_cb(), which returns a handle to the callback object being registered.
i) vpi_register_systf(), which returns a handle to the callback object for a user-defined system task or
function
j) vpi_scan(), which returns a handle to objects in a one-to-many relationship, using their iterator
object
A tool shall support multiple VPI programs, each of which acquires handles. The way in which a tool
implements handles shall allow a VPI program to function correctly independently of other VPI programs
executing concurrently. A tool may share between VPI programs resources associated with the
implementation of handles and the objects to which they refer. However, the occurrence of such sharing
shall not alter the effect of the VPI programs. If a tool creates two handles that refer to the same object, the
tool may create two distinct handles or may provide the same handle in both cases. Two distinct handles that
refer to the same object are equivalent.
NOTE—The number of handles that an implementation can create may be constrained by the capacity of the host
system.
#### 37.2.2 Handle release
The function vpi_release_handle() called by a VPI program causes a tool to release a handle. If a tool
shares resources associated with handles and one VPI program releases a handle, other VPI programs shall

<!-- Page 1000 -->

IEEE Std be able to continue to refer to objects using handles that they have not released. The tool may reclaim
resources associated with the representation of a released handle.Handles may also be released as part of the
action of other VPI function calls, in particular:
a) vpi_remove_callback() releases the associated callback handle.
b) vpi_scan() releases the iterator handle after its last object has been scanned.
Simulation events or actions may also cause certain handles to be released, in particular:
1) A simulation restart shall release all handles except for cbStartOfRestart and cbEndOfRestart
callback handles.
2) Whenever the simulator frees objects belonging to a frame or thread, it shall release all handles to
those objects, and to any subelement of these objects. Handles to callbacks placed on these objects
will also be released.
3) Whenever the simulator reclaims the memory of a class object, it shall release all handles to the class
object, to any of its automatic data members, and to any subelement of its automatic data members.
Handles to callbacks placed on these objects will also be released.
NOTE 1—It is recommended that a VPI program release handles when they are no longer needed.
NOTE 2—A tool may reclaim resources associated with a handle when the handle is released by a VPI program,
provided the requirements of 37.2 are met. As a consequence, resources might not be reclaimed immediately upon
release of a handle by a VPI program, as the resources may be associated with handles in use by other VPI programs.
NOTE 3—A static local variable declared in a task/function does not belong to a frame or thread, and handles to such a
variable or callbacks associated with the variable are not released automatically when the frame or thread ends.
#### 37.2.3 Handle comparison
Handle equivalence cannot be determined with a C “==” comparison. The function vpi_compare_objects()
compares the objects they refer to. It returns the value 1 if the objects they refer to are the same object);
otherwise it returns the value 0. See 38.3.
#### 37.2.4 Validity of handles
The lifetime of an object is the duration of existence of the object in the VPI information model. Lifetime of
objects is discussed in 37.3.7. A tool can create a handle that refers to an object only during the lifetime of
the object. A handle is said to be valid from the time of its creation until the time at which it is released, or
until the object that it refers to ceases to exist, or until termination of the tool; at other times it is invalid. A
VPI program shall not refer to an object using an invalid handle, nor shall a VPI program attempt to release
an invalid handle.
### 37.3 VPI object classifications
VPI objects are classified using data model diagrams. These diagrams provide a graphical representation of
those objects within a SystemVerilog design to which the VPI routines shall provide access. The diagrams
shall show the relationships between objects and the properties of each object. Objects with sufficient
commonality are placed in groups. Group relationships and properties apply to all the objects in the group.
As an example, the simplified diagram in Figure37-1 shows that there is a one-to-many relationship from
objects of type module to objects of type net and a one-to-one relationship from objects of type net
toobjects of type module. Objects of type net have properties vpiName, vpiVector, and vpiSize with data
types string, Boolean, and integer, respectively.

<!-- Page 1001 -->

IEEE Std module net
-> name
str: vpiName
str: vpiFullName
-> vector
bool: vpiVector
-> size
int: vpiSize
Figure37-1—Example of object relationships diagram
For object relationships (unless a special tag is shown in the diagram), the type used for access is determined
by adding “vpi” to the beginning of the word within the enclosure with each word’s first letter being a
capital. Using the above example, if an application has a handle to a net and wants to go to the module
instance where the net is defined, the call would be as follows:
modH = vpi_handle(vpiModule,netH);
where netH is a handle to the net. As another example, to access a “named event” object, use the type
vpiNamedEvent.
#### 37.3.1 Accessing object relationships and properties
VPI defines the C data type of vpiHandle. All objects are manipulated via a vpiHandle variable. Object
handles can be accessed from a relationship with another object or from a hierarchical name as the following
example demonstrates:
vpiHandle net;
net = vpi_handle_by_name("top.m1.w1", NULL);
This example call retrieves a handle to wire top.m1.w1 and assigns it to the vpiHandle variable net. The
NULL second argument directs the routine to search for the name from the top level of the design.
VPI provides generic functions for tasks, such as traversing relationships and determining property values.
One-to-one relationships are traversed with routine vpi_handle(). In the following example, the module that
contains net is derived from a handle to that net:
vpiHandle net, mod;
net = vpi_handle_by_name("top.m1.w1", NULL);
mod = vpi_handle(vpiModule, net);
The call to vpi_handle() in the preceding example shall return a handle to module top.m1.
Sometimes it is necessary to access a class of objects that do not have a name or whose name is ambiguous
with another class of objects that can be accessed from the reference handle. Tags are used in this situation,
as shown in Figure37-2.

<!-- Page 1002 -->

IEEE Std expr
vpiLeftRange
part select
expr
vpiRightRange
Figure37-2—Accessing a class of objects using tags
In this example, the tags vpiLeftRange and vpiRightRange are used to access the expressions that make up
the range of the part-select. These tags are used instead of vpiExpr to get to the expressions. Without the
tags, VPI would not know which expression should be accessed. For example:
vpi_handle(vpiExpr, part_select_handle)
would be illegal when the reference handle (part_select_handle) is a handle to a part-select because the part-
select can refer to two expressions, a left-range and a right-range.
Properties of objects shall be derived with routines in the vpi_get family. The routine vpi_get() returns
integer and Boolean properties. Integer and Boolean properties shall be defined to be of type PLI_INT32.
For Boolean properties, a value of 1 shall represent TRUE and a value of 0 shall represent FALSE. The
routine vpi_get64() returns 64-bit integer properties as type PLI_INT64. The routine vpi_get_str() accesses
string properties. String properties shall be defined to be of type PLI_BYTE8 *. For example, to retrieve a
pointer to the full hierarchical name of the object referenced by handle mod, the following call would be
made:
PLI_BYTE8 *name = vpi_get_str(vpiFullName, mod);
In the preceding example, the pointer name shall now point to the string “top.m1”.
One-to-many relationships are traversed with an iteration mechanism. The routine vpi_iterate() creates an
object of type vpiIterator, which is then passed to the routine vpi_scan() to traverse the desired objects. In
the following example, each net in module top.m1 is displayed:
vpiHandle itr;
itr = vpi_iterate(vpiNet,mod);
while (net = vpi_scan(itr))
vpi_printf("\t%s\n", vpi_get_str(vpiFullName, net));
As the preceding examples illustrate, the routine naming convention is a “vpi” prefix with “_” word
delimiters (with the exception of callback-related defined values, which use the “cb” prefix). Macro-defined
types and properties have the “vpi” prefix, and they use capitalization for word delimiters.
The routines for traversing SystemVerilog structures and accessing objects are described in Clause38.
#### 37.3.2 Object type properties
All objects have a vpiType property, which is not shown in the data model diagrams.
-> type
int: vpiType
Using vpi_get(vpiType, <object_handle>) returns an integer constant that represents the type of the
object.

<!-- Page 1003 -->

IEEE Std Using vpi_get_str(vpiType, <object_handle>) returns a pointer to a string containing the name of
the type constant. The name of the type constant is derived from the name of the object as it is shown in the
data model diagram (see 37.3 for a description of how type constant names are derived from object names).
Some objects have additional type properties that are shown in the data model diagrams vpiDelayType,
vpiNetType, vpiOpType, vpiPrimType, vpiResolvedNetType, and vpiTchkType. Using
vpi_get(<type_property>, <object_handle>) returns an integer constant that represents the
additional type of the object. See vpi_user.h in AnnexK and sv_vpi_user.h in AnnexM for the types
that can be returned for these additional type properties. The constant names of the types returned for these
additional type properties can be accessed using vpi_get_str().
#### 37.3.3 Object file and line properties
Most objects have the following two location properties, which are not shown in the data model diagrams:
-> location
int: vpiLineNo
str: vpiFile
The properties vpiLineNo and vpiFile can be affected by the `line compiler directive. See 22.12 for more
details on the `line compiler directive. These properties are applicable to every object that corresponds to
some object within the source code. The exceptions are objects of the following types:
— vpiCallback
— vpiDelayTerm
— vpiDelayDevice
— vpiInterModPath
— vpiIterator
— vpiTimeQueue
— vpiGenScopeArray
— vpiGenScope
#### 37.3.4 Delays and values
Most properties are of type integer, Boolean, or string. Delay and logic value properties, however, are more
complex and require specialized routines and associated structures. The routines vpi_get_delays() and
vpi_put_delays() use structure pointers, where the structure contains the pertinent information about delays.
Similarly, simulation values are also handled with the routines vpi_get_value() and vpi_put_value(), along
with an associated set of structures.
The routines, C structures, and some examples for handling delays and logic values are presented in
Clause38. See 38.15 for vpi_get_value(), 38.34 for vpi_put_value(), 38.10 for vpi_get_delays(), and
### 38.32 for vpi_put_delays().
Nets, primitives, module paths, timing checks, and continuous assignments can have delays specified within
the SystemVerilog source code. Additional delays may exist, such as module input port delays or inter-
module path delays, that do not appear within the SystemVerilog source code. To access the delay
expressions that are specified within the SystemVerilog source code, use the method vpiDelay. These
expressions shall be either an expression that evaluates to a constant if there is only one delay specified or an
operation if there are more than one delay specified. If multiple delays are specified, then the operation’s
vpiOpType shall be vpiListOp. To access the actual delays being used by the tool, use the routine
vpi_get_delays() on any of these objects.

<!-- Page 1004 -->

IEEE Std #### 37.3.5 Expressions with side effects
VPI gives applications access to arbitrarily complex expressions from the SystemVerilog source, either as
arguments to system tasks or functions (see 36.4) or by traversing the design hierarchy. Expressions may
have side effects when evaluated; such expressions include the following:
— Assignment operators (11.4.1)
— Increment and decrement operators (11.4.2)
— Function calls, including built-in methods and system function calls, that change the state of the
simulation other than via their return values
— Expressions in which other expressions with side effects appear as operands, arguments, or index
expressions
Applying the function vpi_get_value() (38.15) to an expression with side effects shall fully evaluate the
expression together with its side effects. However, it shall be an error for an application to ask for a VPI
property or relation of an expression if the VPI implementation cannot determine the value or handle
without also evaluating an expression with side effects. Since implementations may differ in their ability to
determine whether an expression has side effects, this result may result in an error with some
implementations but not with others. It shall be an error for an application to apply vpi_put_value() (38.34)
to an object if any of its index expressions is an expression with side effects.
To provide the greatest flexibility for VPI applications, it is recommended that expressions with side effects
not be used as index expressions or as arguments to system tasks or functions or to SystemVerilog function
calls.
Example 1:
function string ename(my_enum_type e);
static first_time = 1;
begin
if (first_time == 1) first_time = 0;
ename = e.name();
end
endfunction
...
foo = ename(e);
For most implementations, asking for the vpiSize property of the function call ename(e) shall be an error
because the implementation cannot determine the size of the function call without evaluating it, and
evaluating it may have the side effect of changing the value of first_time.
In the unusual case in which all the names of the enumeration type have the same length, an implementation
could in principle determine the vpiSize by analyzing the function without evaluating it. However, this is not
required by the standard, and an implementation may issue an error in this case as well.
Example 2:
j = my_array[i++];
k = my_array[--i];
It shall be an error for a VPI application to apply vpi_put_value() to either my_array[i++] or my_array
[--i], since both expressions have side effects.

<!-- Page 1005 -->

IEEE Std #### 37.3.6 Object protection properties
All objects have a vpiIsProtected property, which is not shown in the data model diagrams.
-> IsProtected
bool: vpiIsProtected
Using vpi_get(vpiIsProtected, object_handle) returns a Boolean constant that indicates whether
the object represents code contained in a decryption envelope. The vpiIsProtected property shall be TRUE
if the object_handle represents code that is protected; otherwise, it shall be FALSE. Unless otherwise
specified, access to relationships and properties of a protected object shall be an error. Restrictions on access
to complex properties are specified in the function reference descriptions for the corresponding VPI
functions. Access to the vpiType property and the vpiIsProtected property of a protected object shall be
permitted for all objects.
NOTE—Handles to protected objects can be returned through object relationships or by direct lookup using VPI
functions that return handles.
#### 37.3.7 Lifetimes of objects
The lifetime of an object is the duration of existence of the object in the VPI information model. A source
code object comes into existence during analysis and persists, independent of elaboration and run time, until
the tool terminates. It has a lifetime that is independent of simulation. Static objects rooted in the static
design hierarchy are alive from the point at which they are created during elaboration and for the entire
simulation. Objects that may have a lifetime shorter than the duration of the simulation are called transient
objects. Class objects and automatic variables are transient objects.
A class object (see 37.32) is alive from the time it is created by a call to new() until the time its memory is
reclaimed by the simulator’s automatic memory management (see 8.29); data members and methods that
belong to the class object have the same object lifetime as the class object. An automatic variable that
belongs to a frame (see 37.43) has the same object lifetime as that of the frame, which is alive from the point
of the call that establishes the stack frame until the stack frame is destroyed.
Other transient objects include the following:
a) Threads (see 37.44)
b) Outdated and out-of-scope references made within a thread
c) Iterators (objects of type vpiIterator), which are created by calls to vpi_iterate() (see 38.23)
d) A vpiSchedEvent created by vpi_put_value() (see 38.34)
e) Callbacks (see 38.36)
There are two properties relevant to understanding the lifetimes of objects. As a property of an object,
vpiAutomatic is a Boolean property that, when false, means the object is static. When true, it means the
object is non-static and may be an automatic variable or dynamic object. The property name vpiAutomatic
and its interpretation reflect the keywords in the language, static and automatic, used to declare the object.
Those keywords may be applied to the object declaration or to the scope of the object, the latter indicating
the default for all objects of that scope. vpiAutomatic is also a property of an instance of a module,
program, interface, or package, indicating the default lifetime for variables of any of its declared tasks/
functions. vpiAutomatic is also a property of a class defn or class typespec, indicating the default lifetime
for variables of any of its declared tasks/functions. Other exceptions to this general description of
vpiAutomatic are noted in the object diagram details.
The property vpiAllocScheme indicates how an object’s memory was allocated and thus supports
understanding its lifetime. It is useful for determining whether and how to manage a transient object. It is an
enumeration of three possible values: vpiAutomaticScheme, vpiDynamicScheme, and vpiOtherScheme.

<!-- Page 1006 -->

IEEE Std vpiAutomaticScheme indicates the object is allocated as part of a frame or thread and has the lifetime of
that frame or thread. vpiDynamicScheme indicates the object was allocated in dynamic memory and may
be a class object or part thereof. For all other objects, vpiAllocScheme shall return vpiOtherScheme.
#### 37.3.8 Managing transient objects
One may obtain a handle to an object during its lifetime, and it remains valid only as long as the object
exists. For a static object, one may therefore keep its handle indefinitely. For a transient object, one may
release its handle after use or expect that handle to be released and become invalid when the object ceases to
exist.
The life of a transient object may be tracked through various callbacks, depending on the specific type of
object. The callbacks are described on the object model diagrams and/or the function reference for
vpi_register_cb(), as appropriate. The relevant callbacks are as follows:
cbCreateObj, cbReclaimObj, cbStartofFrame, cbEndOfFrame, cbStartOfThread, cbEndOfThread,
and cbEndOfObject.
### 37.4 Key to data model diagrams
This subclause contains the keys to the symbols used in the data model diagrams. Keys are provided for
objects and classes, traversing relationships, and accessing properties.

<!-- Page 1007 -->

IEEE Std #### 37.4.1 Diagram key for objects and classes
Object definition:
obj defn
Bold letters in a solid enclosure indicate an object definition. The
properties of the object are defined in this location.
Object reference:
object
Normal letters in a solid enclosure indicate an object reference.
Class definition:
class defn
class Bold italic letters in a dotted enclosure indicate a class definition,
where the class groups other objects and classes. Properties of the
obj defn class are defined in this location. The class definition can contain an
object definition.
object
Class reference:
class
Italic letters in a dotted enclosure indicate a class reference.
Unnamed class:
obj1
A dotted enclosure with no name is an unnamed class. It is sometimes
obj2 convenient to group objects although they shall not be referenced as a
group elsewhere; therefore, a name is not indicated.
#### 37.4.2 Diagram key for accessing properties
obj Integer and Boolean properties are accessed with the routine vpi_get().
These properties are of type PLI_INT32.
-> vector
bool: vpiVector For example: Given handle obj_h to an object of type vpiObj, test if
-> size the object is a vector, and get the size of the object.
int: vpiSize PLI_INT32 vect_flag = vpi_get(vpiVector, obj_h);
PLI_INT32 size = vpi_get(vpiSize, obj_h);
String properties are accessed with routine vpi_get_str(). String
obj
properties are of type PLI_BYTE8 *.
-> name
str: vpiName For example:
str: vpiFullName
PLI_BYTE8 *name = vpi_get_str(vpiName, obj_h);
object Complex properties for time and logic value are accessed with the
indicated routines. See the descriptions of the routines for usage.
-> complex
func1()
func2()

<!-- Page 1008 -->

IEEE Std #### 37.4.3 Diagram key for traversing relationships
A single arrow indicates a one-to-one relationship accessed
ref with the routine vpi_handle().
For example: Given vpiHandle variable ref_h of type ref,
access obj_h of type Obj:
obj
obj_h = vpi_handle(Obj, ref_h);
ref A tagged one-to-one relationship is traversed similarly, using
Tag instead of Obj.
Tag
For example:
obj obj_h = vpi_handle(Tag, ref_h);
A one-to-one relationship that originates from a circle is
traversed using NULL for the ref_h.
For example:
obj
obj_h = vpi_handle(Obj, NULL);
A double arrow indicates a one-to-many relationship accessed
ref
with the routine vpi_scan().
For example: Given vpiHandle variable ref_h of type ref,
scan objects of type Obj:
obj
itr = vpi_iterate(Obj, ref_h);
while (obj_h = vpi_scan(itr) )
/* process 'obj_h' */
A tagged one-to-many relationship is traversed similarly, using
ref Tag instead of Obj.
For example:
Tag
itr = vpi_iterate(Tag, ref_h);
obj while (obj_h = vpi_scan(itr) )
/* process 'obj_h' */
A one-to-many relationship that originates from a circle is
traversed using NULL for the ref_h.
For example:
obj itr = vpi_iterate(Obj, NULL);
while (obj_h = vpi_scan(itr) )
/* process 'obj_h' */
For relationships that do not have a tag, the type used for access is determined by adding “vpi” to the
beginning of the word within the enclosure, with each word’s first letter being a capital. See 37.3 for more
details on VPI access to constant names.

<!-- Page 1009 -->

IEEE Std ### 37.5 Module
expr
vpiIndex
vpiGlobalClocking
clocking block vpiInternalScope
scope
vpiDefaultClocking
clocking block
port
expr vpiDefaultDisableIff interface
distribution
interface array
process
instance array module
cont assign
-> top module
bool: vpiTopModule
module array
module
-> decay time
int: vpiDefDecayTime
module array
primitive
primitive array
mod path
tchk
def param
io decl
alias stmt
clocking block
Details:
1) Top-level modules shall be accessed using vpi_iterate() with a NULL reference object.
2) If a module is an element within a module array, the vpiIndex transition is used to access the index within the array.
If a module is not part of a module array, this transition shall return NULL.

<!-- Page 1010 -->

IEEE Std ### 37.6 Interface
expr
vpiIndex
vpiGlobalClocking
clocking block
vpiDefaultClocking
clocking block
interface tf decl
expr vpiDefaultDisableIff modport
distribution
mod path
instance array interface cont assign
vpiInstance
clocking block
interface
interface array
process
Details:
1) If an interface is an element within an instance array, the vpiIndex transition is used to access the index within the
array. If an interface is not part of an instance array, this transition shall return NULL.
### 37.7 Modport
interface modport io decl
-> name
str: vpiName
### 37.8 Interface task or function declaration
task
interface tf decl
-> access type
int: vpiAccessType function
Details:
1) vpi_iterate() can return more than one task or function declaration for modport tasks or functions with an access
type of vpiForkJoinAcc, because the task or function can be imported from multiple module instances.
2) Possible return values for the vpiAccessType property for an interface tf decl are vpiForkJoinAcc and
vpiExternAcc.

<!-- Page 1011 -->

IEEE Std ### 37.9 Program
expr
vpiIndex
vpiDefaultClocking
clocking block
expr vpiDefaultDisableIff
cont assign
distribution
clocking block
instance array program
vpiInstance
interface
interface array
process
Details:
1) If a program is an element within an instance array, the vpiIndex transition is used to access the index within the
array. If a program is not part of an instance array, this transition shall return NULL.

<!-- Page 1012 -->

IEEE Std ### 37.10 Instance
instance item
program
instance
program array
package
task func
-> compile unit
bool: vpiUnit
net
interface
array net
program variables
vpiReg
logic var
module
vpiRegArray
array var
-> array member -> protected
vpiMemory
bool: vpiArray (deprecated) bool: vpiProtected array var
bool: vpiArrayMember -> timeprecision
-> cell int: vpiTimePrecision
named event
bool: vpiCellInstance -> timeunit
-> default net type int: vpiTimeUnit
int: vpiDefNetType -> unconnected drive named event array
-> definition location int: vpiUnconnDrive
vpiParameter
int: vpiDefLineNo -> configuration parameters
str: vpiDefFile
str: vpiLibrary
-> definition name str: vpiCell
spec param
str: vpiDefName str: vpiConfig
-> delay mode -> default lifetime
int: vpiDefDelayMode bool: vpiAutomatic assertion
-> name -> top
vpiTypedef
str: vpiName bool: vpiTop typespec
str: vpiFullName
class defn
vpiNetTypedef
nettype decl
Details:
1) The vpiTypedef iteration shall return the user-defined typespecs that have typedefs explicitly declared in the
instance.
2) vpiModule shall return a module if the object is inside a module instance, otherwise it shall return NULL.
3) vpiInstance shall always return the immediate instance (package, module, interface, or program) in which the
object is instantiated.

<!-- Page 1013 -->

IEEE Std 4) vpiMemory shall return array variable objects rather than vpiMemory objects.
5) vpiFullName for objects that exist within a compilation unit shall begin with “$unit::”. As a result, the full
name for objects within a compilation unit may be ambiguous. vpiFullName for a package shall be the name of the
package and should end with “::”; this syntax disambiguates between a module and a package of the same name.
vpiFullName for objects that exist in a package shall begin with the name of the package followed by “::”. The
separator :: shall appear between the package name and the immediately following name component. The
“.”separator shall be used in all cases except package and class defn.
6) The following items shall not be accessible via vpi_handle_by_name():
— Imported items
— Objects that exist within a compilation unit
7) Passing a NULL handle to vpi_get() with properties vpiTimePrecision or vpiTimeUnit shall return the smallest
time precision of all modules in the instantiated design.
8) The properties vpiDefLineNo and vpiDefFile can be affected by the `line compiler directive. See 22.12 for more
details on the `line directive.
9) For details on lifetime and memory allocation properties, see 37.3.7.
10) The vpiNetTypedef iteration shall return the handles to the user-defined nettypes that are explicitly declared in the
instance.

<!-- Page 1014 -->

IEEE Std ### 37.11 Instance arrays
vpiLeftRange
expr
expr instance array
expr
range primitive array vpiRightRange
instance
interface array
program array param assign
module array
module
-> access by index
vpi_handle_by_index()
vpi_handle_by_multi_index()
-> name
str: vpiName
str: vpiFullName
->size
int: vpiSize
primitive array
primitive
gate array
expr
switch array
vpiDelay
udp array
Details:
1) Traversing from the instance array to expr shall return a simple expression object of type vpiOperation with a
vpiOpType of vpiListOp. This expression can be used to access the actual list of connections to the instance array
in the SystemVerilog source code
2) vpi_iterate(vpiRange, instance_array_handle) shall return the set of instance array ranges beginning with the
leftmost range of the array declaration and iterating through the rightmost range. Using the vpiLeftRange/
vpiRightRange properties returns the bounds of the leftmost dimension of a multidimensional array.

<!-- Page 1015 -->

IEEE Std ### 37.12 Scope
scope property decl
instance sequence decl
task func concurrent assertion
named begin
named event
begin
named event array
named fork
variables
fork
virtual interface var
-> join type vpiReg
logic var
int: vpiJoinType
vpiRegArray
array var
class defn
stmt vpiMemory
class typespec array var
vpiParameter
class obj parameters
clocking block vpiInternalScope
scope
gen scope vpiImport
instance item
for
vpiTypedef
typespec
foreach stmt
let decl
-> name
str: vpiName
str: vpiFullName
Details:
1) An unnamed begin or unnamed fork shall be a scope if, and only if, it directly contains a block item declaration
such as a variable declaration or type declaration. A named begin or named fork shall always be a scope.
Example:
begin
begin : BLK
var logic v; // This declaration is not local to the unnamed begin
v = 1'b1;
end
end
In this example, the block BLK is a scope, but the unnamed begin is not a scope because it does not directly contain
a block item declaration.
2) A for statement shall be a scope if, and only if, the vpiLocalVarDecls property returns TRUE. In this case, the
scope of each loop control variable shall be the for statement.

<!-- Page 1016 -->

IEEE Std 3) The scope of each loop control variable in a foreach stmt shall be the foreach stmt.
4) The vpiImport iterator shall return all objects imported into the current scope via import declarations. Only objects
actually referenced through the import shall be returned, rather than items potentially made visible as a result of the
import. Refer to 26.3 for more details.
5) A task func can have zero or more statements (see 13.3, 13.4). If the number of statements is greater than 1, the
vpiStmt relation shall return an unnamed begin that contains the statements of the task or function. If the number
of statements is zero, the vpiStmt relation shall return NULL.
6) The vpiJoinType property indicates what type of join statement terminates the fork-join block. It shall return one
of the values vpiJoin, vpiJoinNone, or vpiJoinAny.
7) The vpiVirtualInterfaceVar iteration is supported only within elaborated contexts and is not supported within
lexical contexts such as class defns (see 37.31). If the scope declares an array of virtual interfaces, the
vpiVirtualInterfaceVar iteration shall return each element of the array separately. However, the vpiVariables
iteration shall return the array declaration as a single vpiArrayVar.
### 37.13 IO declaration
ref obj
instance
vpiExpr interface tf decl
udp defn io decl
nets
-> direction
task func int: vpiDirection
variables
-> name
module str: vpiName
vpiLeftRange
-> scalar expr
bool: vpiScalar
vpiRightRange
-> sign
expr
bool: vpiSigned
-> size
range
int: vpiSize
-> vector
typespec
bool: vpiVector
Details:
1) vpiDirection returns vpiRef for pass by ref ports or arguments.
2) A ref obj type handle shall be returned for the vpiExpr of an io decl if it is passed by reference or if the io decl is an
interface or a modport. If the io decl is a virtual interface, vpiExpr shall return a vpiVirtualInterfaceVar.
3) If the vpiExpr of an io decl is a ref obj and if the vpiActual of the ref obj is an interface or modport declaration,
then the vpiDirection of the io decl shall be undefined. The vpiDirection shall also be undefined if the vpiExpr is
a virtual interface var.
4) The vpiRange, vpiLeftRange, and vpiRightRange relations for an io decl shall be the same as for the
corresponding typespec (see 37.25).

<!-- Page 1017 -->

IEEE Std ### 37.14 Ports
expr
vpiHighConn
ports
instance port vpiLowConn
ref obj
vpiParent
module
typespec
vpiBit
port bit
-> access by index -> index
vpi_handle_by_index() int: vpiPortIndex
vpi_handle_by_multi_index() -> name
->connected by name str: vpiName
bool: vpiConnByName -> port type
-> delay (mipd) int: vpiPortType
vpi_get_delays() -> scalar
vpi_put_delays()
bool: vpiScalar
-> direction
-> size
int: vpiDirection
int: vpiSize
-> explicitly named
-> vector
bool: vpiExplicitName
bool: vpiVector
Details:
1) vpiPortType shall be one of the following three types: vpiPort, vpiInterfacePort, or vpiModportPort. Port type
depends on the formal, not on the actual.
2) vpi_get_delays() and vpi_put_delays() shall not be applicable for vpiInterfacePort.
3) vpiHighConn shall indicate the hierarchically higher (closer to the top module) port connection.
4) vpiLowConn shall indicate the lower (further from the top module) port connection.
5) vpiLowConn of a vpiInterfacePort shall always be vpiRefObj.
6) Properties vpiScalar and vpiVector shall indicate whether the port is 1 bit or more than 1 bit. They shall not
indicate anything about what is connected to the port.
7) Properties vpiPortIndex and vpiName shall not apply for port bits.
8) If a port is explicitly named, then the explicit name shall be returned. If not, and a name exists, then that name shall
be returned. Otherwise, NULL shall be returned.
9) vpiPortIndex can be used to determine the port order. The first port has a port index of zero.
10) vpiLowConn shall return NULL if the module or interface or program port is a null port (e.g., “module M();”).
vpiHighConn shall return NULL if the instance of the module, interface, or program does not have a connection to
the port.
11) vpiSize for a null port shall return 0.

<!-- Page 1018 -->

IEEE Std ### 37.15 Reference objects
vpiPortInst ports
ports
vpiLowConn vpiHighConn
instance ref obj ref obj
vpiParent
-> name
typespec
str: vpiName
task func
str: vpiFullName
-> generic
bool: vpiGeneric vpiActual interface
-> definition name
str: vpiDefName interface array
modport
nets
variables
named event
named event array
part select
Details:
1) A ref obj represents a declared object or subelement of that object that is a reference to an actual instantiated object.
A ref obj exists for ports with ref direction, for an interface port, a modport port, or for formal task function ref
arguments. The specific cases for a ref obj are as follows:
— A variable, named event, named event array that is the lowconn of a ref port
— Any subelement expression of the above
— A local declaration of an interface or modport passed through a port or any net, variable, named event, named
event array of those
— A ref formal argument of a task or function, or subelement expression of it
2) A ref obj may be obtained when walking port connections (lowConn, highConn), when traversing an expression
that is a use of such ref obj, or when accessing the io decl of an instance or task or function.
3) The name of ref obj can be different at every instance level it is being declared. The vpiActual relationship always
returns the actual instantiated object if the ref obj is bound to an actual object at the time of the query.
4) The vpiParent relationship allows the traversal of a ref obj that is a subelement of a ref obj. In the following
example, r[0] is a ref obj whose parent is the ref obj r. The vpiActual for the ref obj r[0] would return the var
bit a[0], and the vpiActual of the ref obj r would return the variable a.
module top;
logic [2:0] a;
m u1 (a);
endmodule
module m (ref [2:0] r);
initial
r[0] = 1'b0;
endmodule

<!-- Page 1019 -->

IEEE Std 5) The vpiGeneric property shall return TRUE if the ref obj is a reference to a generic interface and FALSE if the ref
obj is a reference to an interface that is not a generic interface. The vpiGeneric property shall return vpiUndefined
for all other kinds of ref obj.
6) The vpiDefName property when applied to a ref obj that is an actual of an interface or modport shall return the
interface definition name or modport name.
7) The vpiTypespec relation returns NULL for a ref obj that vpiActual is a not a net, variable, or part select.
Example: Passing an interface or modport through a port:
interface simple ();
logic req, gnt;
modport target (input req, output gnt);
modport initiator (input gnt, output req);
endinterface
module top();
simple i();
child1 i1(i);
child2 i2(i.initiator);
endmodule
/***********************************
for the port of i1,
the vpiHighConn relationship returns a handle of type vpiRefObj. The
vpiActual relationship applied to the ref obj returns a handle of type
vpiInterface.
for the port of i2,
the vpiHighConn relationship returns a handle of type vpiRefObj. The
vpiActual relationship applied to the ref obj returns a handle of type
vpiModport.
****************************************/
module child1(simple s);
c1 c_1(s);
c1 c_2(s.initiator);
endmodule
/****************************
for the port of module child1,
the vpiLowConn relationship returns a handle of type vpiRefObj. The
vpiActual relationship applied to the ref obj returns a handle of type
vpiInterface.
for that refObj,
the vpiPort relationship returns the port of child1.
the vpiPortInst iteration returns handles to s, s.initiator.
the vpiActual relationship returns a handle to i.
for the port of instance c_1,
vpiHighConn returns a handle of type vpiRefObj. The vpiActual relationship
applied to the ref obj handle returns a handle of type vpiInterface.
for the port of instance c_2,
vpiHighConn returns a handle of type vpiRefObj. The vpiActual relationship
applied to the ref obj handle returns a handle of type vpiModport.
****************************************/

<!-- Page 1020 -->

IEEE Std ### 37.16 Nets
vpiPortInst vpiDriver
ports net drivers
vpiLoad
ports net loads
vpiLocalDriver
vpiLowConn vpiHighConn
net drivers
vpiLocalLoad
net loads
module nets
prim term
net
cont assign
vpiParent
vpiIndex path term
expr vpiBit
tchk term
net bit
vpiSimNet
expr nets
vpiIndex
interconnect array
typespec
vpiParent
array net net
range vpiIndex
expr
-> access by index -> member -> sign
vpi_handle_by_index() bool: vpiStructUnionMember bool: vpiSigned
vpi_handle_by_multi_index() -> name -> size
-> array member str: vpiName int: vpiSize
bool: vpiArray (deprecated) str: vpiFullName -> strength
bool: vpiArrayMember -> net decl assign int: vpiStrength0
-> constant selection bool: vpiNetDeclAssign int: vpiStrength1
bool: vpiConstantSelect -> net type int: vpiChargeStrength
-> delay int: vpiNetType -> value
vpi_get_delays() int: vpiResolvedNetType vpi_get_value()
-> expanded -> scalar vpi_put_value()
bool: vpiExpanded bool: vpiScalar -> vector
-> implicitly declared -> scalared declaration bool: vpiVector
bool: vpiImplicitDecl bool: vpiExplicitScalared -> vectored declaration
bool: vpiExplicitVectored

<!-- Page 1021 -->

IEEE Std vpiMember
nets
net
vpiParent enum net
struct net
vpiParent
struct net
union net
union net
enum net
vpiElement
packed array net
integer net
range time net -> packed array member
bool:
vpiRightRange logic net vpiPackedArrayMember
expr
vpiIndex
packed array net
expr
vpiLeftRange
expr interconnect net
user defined net
user defined net
short real net
real net
byte net
short int net
int net
long int net
integer net
time net
unpacked array net
range packed array net
vpiRightRange bit net
expr
logic net
vpiLeftRange
struct net
expr
union net
enum net
Details:
1) Any net declared as an array with one or more unpacked ranges is an array net. Any packed struct net,packed union
net, or enum net declared with one or more explicit packed ranges is a packed array net. The range iterator for a

<!-- Page 1022 -->

IEEE Std packed array net returns only the explicit packed ranges for such a net. It shall not return the implicit range of
packed struct net or packed union net elements themselves, nor shall it return the range (explicit or implicit) for the
base type of enum net elements. For example:
// a 34-bit-wide struct net (range iteration not allowed)
wire struct packed {logic [1:0]vec1; integer i1;} psnet;
// a packed array net (ranges [3:0] and [2:1] returned by range iteration)
wire struct packed {logic [1:0]vec1; integer i1;} [3:0][2:1] panet;
// an array net (ranges [5:4] and [6:8] returned by range iteration)
wire struct packed {logic [1:0]vec1; integer i1;} [3:0][2:1] anet [5:4][6:8];
2) The Boolean property vpiArray is deprecated in this standard. The vpiArrayMember property shall be TRUE for
a net that is an element of an array net. It shall be FALSE otherwise. The vpiPackedArrayMember property shall
be TRUE for a packed struct net, a packed union net, an enum net, or a packed array net that is an element of a
packed array net.
3) For logic nets or bit nets, net bits shall be available regardless of vector expansion.
4) Continuous assignments and primitive terminals shall be accessed regardless of hierarchical boundaries.
5) Continuous assignments and primitive terminals shall only be accessed from scalar nets or bit-selects.
6) For vpiPorts, if the reference handle is a net bit, then port bits shall be returned. If it is an entire net or array net,
then a handle to the entire port shall be returned.
7) For vpiPortInst, if the reference handle is a bit or scalar, then port bits or scalar ports shall be returned, unless the
highconn for the port is a complex expression where the bit index cannot be determined. If this is the case, then the
entire port shall be returned. If the reference handle is an entire net or array net, then the entire port shall be
returned.
8) For vpiPortInst, it is possible for the reference handle to be part of the highconn expression, but not connected to
any of the bits of the port. This may occur if there is a size mismatch. In this situation, the port shall not qualify as a
member for that iteration.
9) For implicit nets, vpiLineNo shall return 0, and vpiFile shall return the file name where the implicit net is first
referenced.
10) vpi_handle(vpiIndex, net_bit_handle) shall return the bit index for the net bit. vpi_iterate(vpiIndex,
net_bit_handle) shall return the set of indices for a multidimensional net array bit-select, starting with the index for
the net bit and working outward.
11) The vpiNetType for a net declared with a nettype shall be vpiNettypeNet. The vpiNetType for any part of a net
declared with a nettype shall be vpiNettypeNetSelect. The vpiDriver and vpiLocalDriver iterations shall not be
supported for a net with a vpiNetType value of vpiNettypeNetSelect.
12) The vpiNetType for an interconnect net or interconnect array shall be vpiInterconnect. The vpiResolvedNetType
for an interconnect net that is a simulated net (see 23.3.3.7) shall be the resolved type of the simulated net.
13) The vpiTypespec relation shall return NULL for an interconnect array.
14) Only active forces and assign statements shall be returned for vpiLoad.
15) Only active forces shall be returned for vpiDriver.
16) vpiDriver shall also return ports that are driven by objects other than nets and net bits.
17) vpiLocalLoad and vpiLocalDriver return only the loads or drivers that are local, i.e., contained by the module
instance that contains the net, including any ports connected to the net (output and inout ports are loads, input and
inout ports are drivers).
18) For vpiLoad, vpiLocalLoad, vpiDriver, and vpiLocalDriver iterators, if the object is a vector net (a net of an
integral data type (see 6.11.1) and for which vpiVector is TRUE), then all loads or drivers are returned exactly
once as the loading or driving object. That is, if a part-select loads or drives only some bits, the load or driver
returned is the part-select. If a driver is repeated, it is only returned once. To trace exact bit-by-bit connectivity, pass
a vpiNetBit object to vpi_iterate.
19) An iteration on loads or drivers for a variable bit-select shall return the set of loads or drivers for whatever bit to
which the bit-select is referring to at the beginning of the iteration.

<!-- Page 1023 -->

IEEE Std 20) vpiSimNet shall return a unique net if an implementation collapses nets across hierarchy (refer to 23.3.3.7 for the
definition of simulated net and collapsed net).
21) The property vpiExpanded on an object of type vpiNetBit shall return the property’s value for the parent.
22) The loads and drivers returned from (vpiLoad, obj_handle) and vpi_iterate(vpiDriver, obj_handle) may not be
the same in different implementations, due to allowable net collapsing (see 23.3.3.7). The loads and drivers
returned from vpi_iterate(vpiLocalLoad, obj_handle) and vpi_iterate(vpiLocalDriver, obj_handle) shall be the
same for all implementations.
23) The Boolean property vpiConstantSelect shall return TRUE for a net or net bit if it has no parent (the vpiParent
relation returns NULL) or if both of the following are true of the “select” part of the equivalent primary expression
(see A.8.4):
— Every index expression in the select is an elaboration-time constant expression.
— Every element within the select denotes either a member of a struct netor a union net or a member of a packed
or unpacked array with static bounds.
Otherwise, vpiConstantSelect shall return FALSE.
NOTE—If vpiConstantSelect is TRUE, then if the handle refers to a valid underlying simulation object at the
beginning of simulation (or at any point in the simulation), it refers to the same object at all points in the simulation.
Moreover, if any index expression is in or out of bounds at the beginning of simulation, it is in or out of bounds at
all subsequent simulation times as well.
24) For an interconnect array, vpiSize shall return the number of elements in the array. For an interconnect net that is
not an array, the vpiSize is the same as the vpiSize of the net to which it is connected. For an array net, vpiSize
shall return the number of nets in the array. For a net of an integral data type (see 6.11.1), vpiSize shall return the
size of the net in bits. For a net bit, vpiSize shall return 1. For unpacked structures or unions, vpiSize shall return
the number of members in the structure or union.
25) vpi_iterate(vpiIndex, net_handle) shall return the set of indices for a net within an array net, starting with the
index for the net and working outward. If the net is not part of an array (the vpiArrayMember property is FALSE),
a NULL shall be returned. The vpiIndex iterator shall work similarly for packed array net elements (packed struct
nets, packed union nets, enum nets, or packed array nets whose vpiPackedArrayMember property is TRUE). The
indices returned shall start with the index of the element and work outward until the vpiParent packed array net is
reached (see detail 31). The indices retrieved for packed array net elements shall be the same as those shown in the
example for detail 32 for each of the subelements returned by vpiElement. The indices will be retrieved in right-to-
left order as they appear in the text.
26) For an array net, vpi_iterate(vpiRange, handle) shall return the set of array range declarations beginning with the
leftmost unpacked range of the array declaration and iterating through the rightmost unpacked range. For a packed
array (bit net, logic net, or packed array net), the iteration shall return the set of ranges beginning with the leftmost
packed range and iterating through the rightmost packed range. For a bit net, logic net, or packed array net, the
vpiLeftRange and vpiRightRange relations shall return the bounds of the leftmost packed dimension.
27) vpiArrayNet is #defined the same as vpiNetArray for backward compatibility. A call to vpi_get_str(vpiType,
<array_net_handle>) may return either “vpiArrayNet” or “vpiNetArray”.
28) A bit net or logic net without a packed dimension defined is a scalar; and for that object, the property vpiScalar
shall return TRUE and the property vpiVector shall return FALSE. A net bit is a scalar, and the property vpiScalar
shall return TRUE (vpiVector shall return FALSE). The properties vpiScalar and vpiVector when queried on a
handle to an enum net shall return the value of the respective property for an object for which the typespec is the
same as the base typespec of the typespec of the enum net. For any other net of an integral data type (see 6.11.1),
the property vpiVector shall return TRUE (vpiScalar shall return FALSE). For an array net, the vpiScalar and
vpiVector properties shall return the values of the respective properties for an array element. The vpiScalar and
vpiVector properties shall return FALSE for all other net objects.
29) vpiLogicNet is #defined the same as vpiNet for backward compatibility. A call to vpi_get_str(vpiType,
<logic_net_handle>) may return either “vpiLogicNet” or “vpiNet”.
30) Array nets, unpacked struct nets, unpacked union nets, and interconnect arrays do not have a value property.
31) The vpiParent transition shall be allowed on all net objects. It shall return one of the following types of objects
listed, representing one of its prefix objects (field select prefix or indexing select prefix as described in 11.5.3), or
NULL, depending on whether certain criteria are met. For purposes of defining vpiParent, a prefix object is the
object obtained from successively removing the rightmost index or identifier from a compound or indexed/
multidimensional object name.
Consider the following vpiArrayNet objects:

<!-- Page 1024 -->

IEEE Std wire logic [1:0][2:3] mda [4:6][7:8];
wire struct { int i1; logic[1:0][2:3]bvec[4:5]; } spa [9:11][12:13];
mda[6][8][1][3] is a vpiNetBit, mda[6][8][1] is its first prefix object (a 2-bit vpiLogicNet vector), and
×
mda[6][8] is its second prefix object (a 2 2 packed array vpiLogicNet), etc. The spa[9][12].bvec[4]
×
object is a vpiLogicNet (a 2 2 packed array vpiLogicNet), and spa[9][12].bvec is its first prefix object (a
vpiArrayNet struct member), and spa[9][12] is the second prefix object (the vpiStructNet containing the
bvec member), etc.
For a net object with prefix objects, the vpiParent transition shall return one of the following prefix objects,
whichever comes first in prefix order (rightmost to leftmost):
— Struct or union net
— Struct or union member net
— The largest containing packed array net object
— The largest containing unpacked array net object
If there is no prefix object, or no prefix object meets at least one of the above criteria, vpiParent shall return NULL.
Using the preceding declarations, the vpiParent of mda[6][8][1][3] is mda[6][8], the vpiLogicNet
representing the largest containing packed array prefix; the vpiParent of mda[6][8] is mda, the vpiArrayNet
representing the largest containing unpacked array net prefix. Likewise, the vpiParent of
spa[9][12].bvec[4][0] is spa[9][12].bvec[4] (the largest containing packed array net); the
vpiParent of spa[9][12].bvec[4] is spa[9][12].bvec (struct member), and applying vpiParent again
yields spa[9][12], the struct net for member bvec. The vpiParent of spa[9][12] is spa, the largest
containing unpacked array of the struct net; vpiParent of spa (or mda) would return NULL.
32) The vpiElement transition shall be used to iterate over the subelements of packed array nets. Unlike vpiNet
iterations for vpiArrayNet objects, vpiElement shall retrieve elements for only one dimension level at a time. This
means that for multidimensioned packed array nets, vpiElement shall retrieve elements that are themselves also
vpiPackedArrayNet objects. vpiElement can then be used to iterate over the subelements of these objects and so
on, until the leaf level struct nets, union nets, or enum nets are returned. In other words, the data type of each
element retrieved by vpiElement is equivalent to the original vpiPackedArrayNet object’s data type with one
leftmost packed range removed. For example, consider the following vpiPackedArrayNet object:
typedef struct packed { integer i1; logic [1:0][2:3] bvec; } pavartype;
wire pavartype [0:2][6:3] panet1;
The vpiElement transition applied to panet1 shall return three vpiPackedArrayNet objects: panet1[0],
panet1[1], and panet1[2]. The vpiElement transition applied to vpiPackedArrayNet panet1[0] in turn
shall retrieve vpiStructNet objects panet1[0][6], panet1[0][5], panet1[0][4], and
panet1[0][3], respectively. Also, the vpiParent transition for all the above-mentioned subelements of
panet1 shall return panet1 (as per detail 31), since panet1 is “the largest containing packed array net object.”
33) The vpiStructUnionMember property shall be TRUE for any net or array net that is a direct member of a struct net
or a union net, i.e., whose vpiParent is a struct net or a union net (see detail 31). This property shall be FALSE for
any net or array net whose vpiParent is not a struct net or a union net. The vpiParent of a net bit is vpiNet, not a
struct net or a union net, so the vpiStructUnionMember property is not defined for net bits.
34) The vpiDecompile and vpiFullName properties for net objects that are members of structs or unions shall include
their struct name prefix. Such prefixes shall include all nested levels of vpiParent objects sufficient to identify the
respective member element in an expression. The vpiName property for these objects shall not include such
prefixes. The vpi_handle_by_name function shall require the vpiDecompile form of the name to properly resolve
it for any non-top-level scope context, and the vpiFullName form shall be required for the top level. If the object is
an indexed element or indexed subarray (slice) of another net object, those indices shall be included in
vpiDecompile, vpiName, and vpiFullName properties for the object in order to distinguish it from its vpiParent
object. For example:
module top;
wire [7:0] warr1 [1:4][9:15];
wire struct {
integer i1;
logic [1:4] vec [5:8];
struct {
time t1;

<!-- Page 1025 -->

IEEE Std integer j1;
} inner1;
} str1;
endmodule
// Objects from above declarations
vpiFullName: top.warr1[1][9]
vpiDecompile: warr1[1][9]
vpiName: warr1[1][9]
vpiFullName: top.str1.i1
vpiDecompile: str1.i1
vpiName: i1
vpiFullName: top.str1.inner1.j1
vpiDecompile: str1.inner1.j1
vpiName: j1
vpiFullName: top.str1.vec[5]
vpiDecompile: str1.vec[5]
vpiName: vec[5]

<!-- Page 1026 -->

IEEE Std ### 37.17 Variables
vpiPortInst
ports ports
vpiLowConn vpiHighConn
expr vpiDriver
variables variable drivers
short real var vpiLoad
variable loads
real var
module
prim term
byte var
instance
short int var cont assign
scope int var path term
long int var
tchk term
expr
integer var
vpiIndex typespec
time var
vpiParent
vpiParent
variables array var var select
vpiReg
-> array type
int: vpiArrayType
vpiRightRange
vpiParent
expr
packed array var
vpiParent
bit var vpiLeftRange
vpiParent expr
logic var
vpiParent range
struct var
expr vpiParent
vpiParent
union var
vpiIndex vpiParent
enum var
vpiBit
vpiMember
string var
var bit
variables
vpiIndex -> constant selection chandle var
bool: vpiConstantSelect
class var
expr virtual interface var
var bit
-> access by index -> member
vpi_handle_by_index() bool: vpiStructUnionMember
vpi_handle_by_multi_index()
-> lifetime ->value
-> array member bool: vpiAutomatic vpi_get_value()
bool: vpiArray (deprecated) -> memory allocation vpi_put_value()
bool: vpiArrayMember
int: vpiAllocScheme -> scalar
-> name
-> constant variable bool: vpiScalar
str: vpiName
bool: vpiConstantVariable -> visibility
str: vpiFullName
-> determine random availability int: vpiVisibility
-> sign
bool: vpiIsRandomized -> vector
bool: vpiSigned
-> randomization type bool: vpiVector
-> size
int: vpiRandType
int: vpiSize

<!-- Page 1027 -->

IEEE Std Details:
1) Any variable declared as an array with one or more unpacked ranges is an array var.
2) The Boolean property vpiArray is deprecated in this standard. The Boolean property vpiArrayMember shall be
TRUE if the referenced variable is a member of an array variable. It shall be FALSE otherwise.
3) To obtain the members of a union and structure, see the relations in 37.26.
4) For an array var, vpi_iterate(vpiRange, handle) shall return the set of array range declarations beginning with the
leftmost unpacked range and iterating through the rightmost unpacked range. If any dimension of the unpacked
array other than the first dimension is a dynamic array or queue dimension, the iteration shall return an empty range
(see 37.22) for that dimension. The iteration shall also return an empty range for any dimension that is an associa-
tive array dimension. For a packed array, the iteration shall return the set of ranges beginning with the leftmost
packed range and iterating through the rightmost packed range. The ranges returned for a packed array shall not
include the implicit range for packed struct or union var elements themselves, or the range (explicit or implicit) for
the base type of enum var elements.
5) vpi_handle (vpiIndex, var_select_handle) shall return the index of a var select in a one-dimensional array.
vpi_iterate (vpiIndex, var_select_handle) shall return the set of indices for a var select in a multidimensional
array, starting with the index for the var select and working outward.
6) The vpiLeftRange and vpiRightRange relations shall return the bounds of the leftmost packed dimension for a
packed array and of the leftmost unpacked dimension for an unpacked array. If the unpacked array has no mem-
bers,or the leftmost range corresponds to an empty range (see 37.22), vpiLeftRange and vpiRightRange shall
return NULL.
7) A var select is an element selected from an array var.
8) If the variable has an initialization expression, the expression can be obtained from vpi_handle(vpiExpr,
var_handle).
9) vpiSize for a variable array shall return the number of variables in the array. For variables belonging to an integer
data type (see 6.11), for enum vars, and for packed struct and union variables, vpiSize shall return the size of the
variable in bits. For a string var, it shall return the number of characters that the variable currently contains. For
unpacked structures and unions, the size returned indicates the number of fields in the structure or union. For a var
bit, vpiSize shall return 1. For all other variables, the behavior of the vpiSize property is not defined.
10) vpiSize for a var select shall return the number of bits in the var select. This applies only for packed var select.
11) Variables of type vpiArrayVar, vpiClassVar or vpiVirtualInterfaceVar do not have a value property. Struct var
and union var variables for which the vpiVector property is FALSE do not have a value property.
12) vpiBit iterator applies only for logic, bit, packed struct, packed union, and packed array variables.
13) vpi_handle(vpiIndex, var_bit_handle) shall return the bit index for the variable bit. vpi_iterate(vpiIndex,
var_bit_handle) shall return the set of indices for a multidimensional variable bit select, starting with the index for
the bit and working outwards.
14) cbSizeChange shall be applicable only for dynamic and associative arrays, for queues, and for string vars. If both
value and size change, the size change callback shall be invoked first. This callback fires after the size change
occurs and before any value changes for that variable. The value in the callback is the new size of the array.
15) The property vpiRandType returns the current randomization type for the variable, which can be one of vpiRand,
vpiRandC, or vpiNotRand.
16) vpiIsRandomized is a property to determine whether a random variable is currently active for randomization.
17) When the vpiStructUnionMember property is TRUE, it indicates that the variable is a member of a parent struct or
union variable. See also the relations in 37.26 and 37.18 detail 5.
18) If a variable is an element of an array (the vpiArrayMember property is TRUE), the vpiIndex iterator shall return
the indexing expressions that select that specific variable out of the array. See 37.18 (and detail 6) for similar
functionality available for elements of packed array vars.
19) In the preceding diagram:
logic var == reg
var bit == reg bit
array var == reg array
vpiVarBit is #defined the same as vpiRegBit for backward compatibility. However, a vpiVarBit can be an

<!-- Page 1028 -->

IEEE Std element of a vpiBitVar (2-state) or a vpiLogicVar (4-state), whereas vpiRegBit could only be an element of a
vpiReg (4-state).
SystemVerilog treats reg and logic variables as equivalent in all respects. To allow for backward compatibility,
a call to vpi_get_str(vpiType,<logic_var_handle>) may return either “vpiLogicVar” or “vpiReg”. Similarly,
vpi_get_str(vpiType,<var_bit_handle>) may return either “vpiVarBit” or “vpiRegBit”, while
vpi_get_str(vpiType,<array_var_handle>) may return either “vpiArrayVar” or “vpiRegArray”.
20) A bit var or logic var, without a packed dimension defined, is a scalar and for those objects, the property vpiScalar
shall return TRUE, and the property vpiVector shall return FALSE. A bit var or logic var, with one or more packed
dimensions defined, is a vector, and the property vpiVector shall return TRUE (vpiScalar shall return FALSE). A
packed struct var, a packed union var, and packed array var are vectors, and the property vpiVector shall return
TRUE (vpiScalar shall return FALSE). A var bit is a scalar, and the property vpiScalar shall return TRUE
(vpiVector shall return FALSE). The properties vpiScalar and vpiVector when queried on a handle to an enum var
shall return the value of the respective property for an object for which the typespec is the same as the base typespec
of the typespec of the enum var. For an integer var, time var, short int var, int var, long int var, and byte var, the
property vpiVector shall return TRUE (vpiScalar shall return FALSE). For an array var, the vpiScalar and
vpiVector properties shall return the values of the respective properties for an array element. The vpiScalar and
vpiVector properties shall return FALSE for all other var objects.
21) vpiArrayType can be one of vpiStaticArray, vpiDynamicArray, vpiAssocArray, or vpiQueueArray.
22) vpiRandType can be one of vpiRand, vpiRandC, or vpiNotRand.
23) For details on lifetime and memory allocation properties, see 37.3.7.
24) vpiVisibility denotes the visibility (local, protected, or default) of a variable that is a class member.
vpiVisibility shall return vpiPublicVis for a class member that is not local or protected, or for a variable that
is not a class member.
25) A non-static data member of a class var does not have a vpiFullName property. The static data member of a class,
referenced either via a class var or a class defn, has the vpiFullName property. It shall return a full name string
representing the hierarchical path of the static variable through “class defn”. For example:
module top;
class Packet ;
static integer Id ;
...
endclass
Packet p;
c = p.Id;
...
The vpiFullName for p.Id is “top.Packet::Id”.
26) The vpiParent transition shall be allowed on all variable objects. It shall return one of the following types of
objects, representing one of its prefix objects (similar to the field select prefix or indexing select prefix as described
in 11.5.3), or NULL, depending on whether certain criteria are met. For purposes of defining vpiParent, a prefix
object is the object obtained from successively removing the rightmost index or identifier from a compound or
indexed/multidimensional object name (excluding scope identifiers).
Consider the following vpiArrayVar objects:
logic [1:0][2:3] mda [4:6][7:8];
struct { int i1; bit [1:0][2:3]bvec[4:5]; } spa [9:11][12:13];
mda[6][8][1][3] is a vpiVarBit, mda[6][8][1] is its first prefix object (a 2-bit vpiLogicVar vector), and
mda[6][8] is its second prefix object (a 2 x 2 vpiLogicVar packed array), etc. The spa[9][12].bvec[4]
object is a vpiBitVar (a 2 x 2 vpiBitVar packed array), and spa[9][12].bvec is its first prefix object (a
vpiArrayVar struct member), and spa[9][12] is the second prefix object (the vpiStructVar containing the
bvec member). etc.
For a variable object with prefix objects, the vpiParent transition shall return one of the following prefix objects,
whichever comes first in prefix order (rightmost to leftmost):
— Struct, union, or class variable
— Struct or union member variable, or class variable data member
— The largest containing packed array object

<!-- Page 1029 -->

IEEE Std — The largest containing unpacked array object
If there is no prefix object, or no prefix object meets at least one of the above criteria, vpiParent shall return
NULL.
Using the preceding declarations, the vpiParent of mda[6][8][1][3] is mda[6][8], the vpiLogicVar
representing the largest containing packed array prefix; the vpiParent of mda[6][8] is mda, the vpiArrayVar
representing the largest containing unpacked array prefix. Likewise, the vpiParent of
spa[9][12].bvec[4][0] is spa[9][12].bvec[4] (the largest containing packed array); the vpiParent
of spa[9][12].bvec[4] is spa[9][12].bvec (struct member), and applying vpiParent again yields
spa[9][12], the struct variable for member bvec. The vpiParent of spa[9][12] is spa, the largest
containing unpacked array of the struct variable; vpiParent of spa (or mda) would return NULL.
Class variables (as previously mentioned in the prefix object types) shall be returned as parent objects only when
they are explicitly used to reference corresponding class data members in the design. A VPI handle to a data
member that does not correspond to such an explicit reference in the design (e.g., a VPI handle to a data member
derived from iterations on its vpiClassObj or vpiClassDefn) shall have a NULL parent.
27) The property vpiConstantSelect shall return TRUE for a var bit or other variable if it has a static lifetime and has
no parent (the vpiParent relation returns NULL) or if both of the following are true of the “select” part of the
equivalent primary expression (see A.8.4):
— Every index expression in the select is an elaboration-time constant expression.
— Every element within the select denotes either a member of a struct or union variable or a member of a packed
or unpacked array with static bounds.
Otherwise, vpiConstantSelect shall return FALSE.
NOTE 1—The final (non-prefix) element of the select may be an unindexed member identifier belonging to any
VPI variable type. It may, for example, be the name of a class variable or dynamic array. However, it may not be a
member of a class variable if the member has an automatic lifetime, and it may not be an element of a dynamically
allocated array.
NOTE 2—If vpiConstantSelect is TRUE, then if the handle refers to a valid underlying simulation object at the
beginning of simulation (or at any point in the simulation), it refers to the same object at all points in the simulation.
Moreover, if any index expression is in or out of bounds at the beginning of simulation, it is in or out of bounds at
all subsequent simulation times as well.
28) The vpiDecompile and vpiFullName properties for variable objects that are members of structs, unions, or class
vars shall include their struct, union, or class var name prefixes. Such prefixes shall include all nested levels of
vpiParent objects sufficient to identify the respective member element in an expression. The vpiName property for
these objects shall not include such prefixes. The vpi_handle_by_name function shall require the vpiDecompile
form of the name to properly resolve it for any non-top-level scope context, and the vpiFullName form shall be
required for the top level. If the object is an indexed element or indexed subarray (slice) of another object, those
indices shall be included in vpiDecompile, vpiName, and vpiFullName properties for the object in order to
distinguish it from its vpiParent object. For example:
module top;
bit [7:0] arr1 [1:4][9:15];
struct {
integer i1;
logic [1:4] vec [5:8];
struct {
shortint j1;
byte b1;
} inner1;
} str1;
class cdef;
int cvInt;
endclass
cdef cv = new;
endmodule

<!-- Page 1030 -->

IEEE Std // Objects from above declarations
vpiFullName: top.arr1[1][9]
vpiDecompile: arr1[1][9]
vpiName: arr1[1][9]
vpiFullName: top.str1.i1
vpiDecompile: str1.i1
vpiName: i1
vpiFullName: top.str1.inner1.j1
vpiDecompile: str1.inner1.j1
vpiName: j1
vpiFullName: top.str1.vec[5]
vpiDecompile: str1.vec[5]
vpiName: vec[5]
vpiFullName: top.cv.cvInt
vpiDecompile: cv.cvInt
vpiName: cvInt
### 37.18 Packed array variables
enum var
vpiParent struct var vpiIndex
packed array var expr
vpiElement union var
-> packed
packed array var
bool: vpiPacked
-> packed array member
bool: vpiPackedArrayMember
-> constant selection
bool: vpiConstantSelect
Details:
1) vpiPackedArrayVar objects shall represent packed arrays of packed struct var, union var, or enum var objects.
The properties vpiVector and vpiPacked for these objects and their underlying struct var, union var, or enum var
elements shall always be TRUE (see 37.17).
2) For consistency with other variable-width vector objects, the vpiSize property for vpiPackedArrayVar objects
shall be the number of bits in the packed array, not the number of struct var, union var, or enum var elements. The
total number of struct var, union var, or enum var elements for a packed array var can be obtained by computing the
product of the vpiSize property for all of its packed ranges.
3) The vpiElement transition shall be used to iterate over the subelements of packed array variables. Unlike
vpiVarSelect or vpiReg transitions for vpiArrayVar objects, vpiElement shall retrieve elements for only one
dimension level at a time. This means that for multidimensioned packed arrays, vpiElement shall retrieve elements
that are themselves also vpiPackedArrayVar objects. vpiElement can then be used to iterate over the subelements
of these objects and so on, until the leaf level struct, enum, or union vars are returned. In other words, the data type
of each element retrieved by vpiElement is equivalent to the original vpiPackedArrayVar object’s data type with
the leftmost packed range removed. For example, consider the following vpiPackedArrayVar object:
typedef struct packed { int i1; bit [1:0][2:3] bvec; } pavartype;
pavartype [0:2][6:3] pavar1;

<!-- Page 1031 -->

IEEE Std The vpiElement transition applied to pavar1 shall return 3 vpiPackedArrayVar objects: pavar1[0],
pavar1[1], and pavar1[2]. The vpiElement transition applied to vpiPackedArrayVar pavar1[0] in turn
shall retrieve vpiStructVar objects pavar1[0][6], pavar1[0][5], pavar1[0][4], and
pavar1[0][3], respectively. Also, the vpiParent transition for all the above-mentioned subelements of
pavar1 shall return pavar1 (as per detail 26 of 37.17, since pavar1 is “the largest containing packed array
object”).
4) The vpiPackedArrayMember property shall be TRUE for any struct var, union var, enum var, or packed array var
whose vpiParent is a packed array var (see detail 26 of 37.17).
5) The vpiStructUnionMember property shall be TRUE only for packed array vars that are direct members of struct
or union vars, i.e., whose vpiParent is a struct or union var (see detail 26 of 37.17). This property shall be FALSE
for all subelements (as returned by the vpiElement iterator) of such packed array vars.
6) vpi_iterate(vpiIndex, packed_array_var_handle) shall return the set of indices for a subelement of a packed
array variable (relative to its vpiParent), starting with the index for the subelement and working outwards. The
indices retrieved shall be the same as those shown in the example for detail 3 for each of the subelements returned
by vpiElement. The indices will be retrieved in right-to-left order as they appear in the text.
### 37.19 Variable select
vpiIndex
expr
vpiParent vpiIndex
array var var select expr
-> constant selection
bool: vpiConstantSelect
-> name typespec
str: vpiName
str: vpiFullName
-> size
int: vpiSize
-> value
vpi_get_value()
vpi_put_value()
Details:
1) The property vpiConstantSelect shall return TRUE for a var select if
— every associated index expression is an elaboration-time constant expression, and
— the parent of the var select is an unpacked array with static bounds, and
— vpiConstantSelect returns TRUE for the parent of the var select.
Otherwise, vpiConstantSelect shall return FALSE.
NOTE—If vpiConstantSelect is TRUE, then if the handle refers to a valid underlying simulation object at the
beginning of simulation (or at any point in the simulation), it refers to the same object at all points in the simulation.
Moreover, if an index expression of the var select or of any of its parents is in or out of bounds at the beginning of
simulation, it is in or out of bounds at all subsequent simulation times as well.

<!-- Page 1032 -->

IEEE Std ### 37.20 Memory
scope
vpiLeftRange
expr
module
vpiRightRange
expr
vpiMemory
vpiParent
reg array
-> access by index
vpi_handle_by_index()
vpi_handle_by_multi_index()
-> is a memory
bool: vpiIsMemory
vpiLeftRange
vpiMemoryWord expr
expr reg
vpiIndex
vpiRightRange
expr
Details:
1) The objects vpiMemory and vpiMemoryWord have been generalized with the addition of arrays of variables. To
preserve backwards compatibility, they have been converted into methods that will return objects of type
vpiRegArray and vpiReg, respectively. See 37.17 for the definitions of variables and variable arrays.
### 37.21 Variable drivers and loads
vpiDriver vpiLoad
variable drivers variables variable loads
ports assign stmt
force force
cont assign cont assign
cont assign bit cont assign bit
assign stmt
Details:
1) vpiDrivers/Loads for a structure, union, or class variable shall include the following:
— Driver/Load for the whole variable
— Driver/Load for any bit-select or part-select of that variable
— Driver/Load of any member nested inside that variable
2) vpiDrivers/Loads for any variable array should include driver/load for entire array/vector or any portion of an
array/vector to which a handle can be obtained.

<!-- Page 1033 -->

IEEE Std ### 37.22 Object range
vpiLeftRange
expr
range
vpiRightRange
-> size expr
int: vpiSize
Details:
1) An empty range is a range that has no elements. An empty range shall be used to represent:
— any range corresponding to an associative array dimension (see 37.17, detail 4)
— a range corresponding to an empty dynamic array or queue
— any range obtained from a typespec corresponding to a dynamic array, queue, or associative array dimension
For example:
int arr1 [][string];
initial
begin
#1 arr1 = new[2];
#1 arr1[0]["hello"] = 5;
end
All ranges obtained from the typespec handle of arr1 are empty. Also, ranges obtained from the arr1 object
itself at simulation time 0 are all empty, since the array is not sized yet. At times 1 and 2, the first range of arr1 is
[0:1] and the second is empty since it corresponds to an associative array dimension.
2) For an empty range, vpiSize shall return 0, while the vpiLeftRange and vpiRightRange relations shall each return
NULL.
### 37.23 Nettype declaration
vpiNetTypedefAlias
nettype decl nettype decl
-> name
typespec
str: vpiName
vpiWith
function
Details:
1) If the nettype declaration has no associated resolution function, the vpiWith relation shall return NULL.
2) If the nettype declaration is an alias of another nettype declaration, the vpiNetTypedefAlias relation shall return a
non-null handle that represents the handle to the aliased nettype.

<!-- Page 1034 -->

IEEE Std ### 37.24 Generic interconnect
interconnect array range
-> packed
vpiLeftRange
bool: vpiPacked expr
vpiRightRange
vpiElement expr
typespec
vpiElement
interconnect array nets
vpiMember
interconnect net nets
Details:
1) The typespec for an interconnect net shall be the typespec of the net or nets it is connected to. If the data type of that
typespec is a packed or unpacked array, the vpiElement iteration applied to the interconnect net shall retrieve
corresponding elements of the interconnect net. If the data type of the typespec is a packed or unpacked struct, the
vpiMember iteration applied to the interconnect net shall retrieve corresponding struct members of the
interconnect net.
2) The vpiElement transition shall be used to iterate over the subelements of interconnect arrays. The vpiElement
iteration shall retrieve elements for only one dimension level at a time. This means that for multidimensional inter-
connect arrays, vpiElement shall retrieve elements that are themselves also interconnect arrays. vpiElement can
then be used to iterate over the subelements of these objects and so on, until the leaf-level interconnect nets are
returned.

<!-- Page 1035 -->

IEEE Std ### 37.25 Typespec
vpiTypedefAlias
typespec
instance typespec
short real typespec
real typespec
byte typespec
short int typespec
int typespec
long int typespec
integer typespec
class typespec
typespec vpiBaseTypespec
time typespec typespec
expr enum typespec
enum const
string typespec -> name
str: vpiName
struct typespec
-> value
typespec member
union typespec vpi_get_value()
-> name
-> tagged
str: vpiName
bool: vpiTagged
-> randomization type
-> soft
int: vpiRandType
bool: vpiSoft
-> packed bit typespec
bool: vpiPacked vpiElemTypespec
range
logic typespec
bit typespec
vpiElemTypespec
expr logic typespec enum typespec
vpiLeftRange vpiElemTypespec
packed array typespec struct typespec
expr
vpiRightRange
-> vector union typespec
vpiIndexTypespec bool: vpiVector
packed array typespec
typespec array typespec
-> array type
vpiElemTypespec
range int: vpiArrayType
typespec
void typespec
sequence typespec
property typespec
event typespec
interface typespec
type parameter
-> name
str: vpiName

<!-- Page 1036 -->

IEEE Std Details:
1) If a typespec denotes a type that has a user-defined typedef, the vpiName property shall return the name of that
type; otherwise, except in the case of a class typespec (see 37.32), the vpiName property shall return NULL.
Consequently the vpiName property returns NULL for any SystemVerilog built-in type. If the typespec denotes a
type with a typedef that creates an alias of another typedef, then the vpiTypedefAlias of the typespec shall return a
non-null handle, which represents the handle to the aliased typedef. For example:
typedef enum bit [0:2] {red, yellow, blue} primary_colors;
typedef primary_colors colors;
If “h1” is a handle to the typespec colors, its vpiType shall return vpiEnumTypespec, the vpiName property
shall return “colors,” vpiTypedefAlias shall return a handle “h2” to the typespec “primary_colors” of
vpiType vpiEnumTypespec. The vpiName property for “h2” shall return “primary_colors”, and its
vpiTypedefAlias shall return NULL.
2) vpiIndexTypespec relation is present only on associative array typespecs and returns the type that is used as the
key into the associative array. For the wildcard index type (see 7.8.1), vpiIndexTypespec shall return NULL.
3) If the value of the property vpiType of a typespec is vpiStructTypesec or vpiUnionTypespec, then it is possible to
iterate over vpiTypespecMember to obtain the structure of the user-defined type. For each typespec member, the
typespec relation indicates the type of the member.
4) The property vpiName of a typespec member returns the name of the corresponding member, rather than the name
(if any) of the associated typespec.
5) The name of a typedef may be the empty string if the typespec denotes a typedef field defined inline rather than via
a typedef declaration. For example:
typedef struct {
struct
int a;
} B
} C;
The typespec representing the typedef C is a struct typespec; it has a single typespec member named B. The
typespec relation for B returns another struct typespec that has no name and has a single typespec member named
“a”. The typespec relation for “a” returns an int typespec.
6) If a type is defined as an alias of another type, it inherits the vpiType of this other type. For example:
typedef time my_time;
my_time t;
The vpiTypespec of the variable named “t” shall return a handle h1 to the typespec “my_time” whose vpiType
shall be a vpiTimeTypespec. The vpiTypedefAlias applied to handle h1 shall return a typespec handle h2 to the
predefined type “time”.
7) The expr associated with a typespec member shall represent the explicit default member value, if any, of the
corresponding member of an unpacked structure data type (See 7.2).
8) The vpiElemTypespec transition shall be used to unwind the typespec of an unpacked array (array typespec) or a
packed array (packed array typespec, or a bit or logic typespec with one or more dimensions), one dimension level
at a time. This means that for a multidimensional array typespec (a typespec with more than one unpacked range),
vpi_handle(vpiElemTypespec, array_typespec_handle) shall initially retrieve a vpiArrayTypespec equivalent
to the original typespec with its leftmost unpacked range removed. Subsequent calls to the vpiElemTypespec
method continue the unwinding until a typespec object is retrieved that has no unpacked ranges remaining.
Similarly, when the vpiElemTypespec is applied to a typespec of a multidimensional packed array object, a
vpiPackedArrayTypespec (or vpiBitTypespec or vpiLogicTypespec) is retrieved that is equivalent to the
original typespec with its leftmost packed range removed, and so on, until a typespec without an explicit packed
range is retrieved. When the vpiElemTypespec relation is applied to a vpiStructTypespec, vpiUnionTypespec,
vpiEnumTypespec, or a vpiBitTypespec or vpiLogicTypespec with no ranges present, it shall return NULL. This
allows packed or unpacked array typespecs constructed with multiple typedefs to be unwound without losing name
information. Consider the complex array typespec defined below for arr:
typedef struct packed { int i1; bit bvec; } [1:3] parrtype;
typedef parrtype [2:1] parrtype2;

<!-- Page 1037 -->

IEEE Std typedef parrtype2 unparrtype [6:4];
unparrtype arr [3:0];
×
The typespec of the object arr is an unpacked 4 3 array typespec with a NULL vpiName property. The typespec
retrieved by applying vpiElemTypespec to this is a 3-element unpacked array typespec with a vpiName property
×
of “unparrtype”. The typespec retrieved by using vpiElemTypespec on this in turn yields a 2 3 packed array
typespec (of packed struct objects) with a vpiName property of “parrtype2”. Using vpiElemTypespec again in
turn yields another packed array typespec (of 3 packed struct objects) with a vpiName property of “parrtype”.
One more application of vpiElemTypespec to this result yields a struct typespec, a non-array typespec for which
no further array subelements exist (the unwinding is done).
9) If a logic typespec, bit typespec, or packed array typespec has more than one packed dimension, vpiLeftRange and
vpiRightRange shall return the bounds of the leftmost packed dimension. If an array typespec has more than one
unpacked dimension, vpiLeftRange and vpiRightRange shall return the bounds of the leftmost unpacked
dimension, unless that dimension corresponds to an empty range (see 37.22), in which case they shall return NULL.
10) For an array typespec, vpi_iterate(vpiRange, handle) shall return the set of array range declarations beginning
with the leftmost unpacked range and iterating through the rightmost unpacked range. If any dimension of the array
typespec corresponds to a dynamic array, associative array, or queue, the iteration shall return an empty range (see
37.22) for that dimension. For a logic typespec or bit typespec that has an associated range, the iteration shall return
the set of ranges beginning with the leftmost packed range and iterating through the rightmost packed range.
11) In a context (such as a class defn) in which a type parameter has not been resolved, the type parameter itself shall
act as a typespec.
### 37.26 Structures and unions
struct var vpiParent
variables
union var vpiMember
-> tagged
bool: vpiTagged
-> soft
bool: vpiSoft
-> packed
bool: vpiPacked
struct net vpiParent
nets
union net vpiMember
-> tagged
bool: vpiTagged
-> soft
bool: vpiSoft
-> packed
bool: vpiPacked
Details:
1) vpi_get_value()/vpi_put_value() cannot be used to access values of entire unpacked structures and unpacked
unions.

<!-- Page 1038 -->

IEEE Std ### 37.27 Named events
vpiTypespec
event typespec
instance
vpiWaitingProcesses
named event thread
-> array member
scope
bool: vpiArray (deprecated)
bool: vpiArrayMember
-> name
module
str: vpiName
str: vpiFullName
-> value
vpi_put_value()
-> lifetime
bool: vpiAutomatic
-> memory allocation
int: vpiAllocScheme
vpiTypespec
array typespec
instance
vpiParent
module named event array named event
-> name
range
str: vpiName
vpiIndex
str: vpiFullName
-> access by index expr
vpi_handle_by_index()
vpi_handle_by_multi_index()
-> lifetime
bool: vpiAutomatic
-> memory allocation
int: vpiAllocScheme
Details:
1) The vpiWaitingProcesses iterator returns all waiting processes, static or dynamic, identified by their threads, for
that named event.
2) vpi_iterate(vpiIndex, named_event_handle) shall return the set of indices for a named event within an array,
starting with the index for the named event and working outward. If the named event is not part of an array, a NULL
shall be returned.
3) vpi_iterate(vpiRange, named_event_array_handle) shall return the set of array range declarations beginning
with the leftmost unpacked range and iterating through the rightmost unpacked range.
4) For details on lifetime and memory allocation properties, see 37.3.7.

<!-- Page 1039 -->

IEEE Std ### 37.28 Parameter, spec param, def param, param assign
module
vpiParameter
parameters
scope
parameter
type parameter
-> local
bool: vpiLocalParam
-> name
str: vpiName
str: vpiFullName
parameter typespec
-> constant type
expr
int: vpiConstType
-> sign vpiLeftRange
bool: vpiSigned expr
-> size vpiRightRange
int: vpiSize expr
-> value
vpi_get_value()
type parameter typespec
vpiExpr
typespec
vpiLhs
parameter
module def param
vpiRhs
expr
vpiLhs
parameters
module
param assign
expr
scope -> connection by name vpiRhs
bool: vpiConnByName
typespec
Details:
1) For a value parameter, vpi_get_value() shall return the value that the parameter has at the end of elaboration.
2) The vpiTypespec of a type parameter shall return the typespec that the type parameter has at the end of elaboration,
but without resolving typedef aliases.

<!-- Page 1040 -->

IEEE Std 3) The vpiExpr relation of a value parameter shall return the default expr, while the vpiExpr relation of a type
parameter shall return the default typespec.
4) vpiLhs from a param assign object shall return a handle to the overridden value parameter or type parameter.
5) If a value parameter does not have an explicitly defined range, vpiLeftRange and vpiRightRange shall return a
NULL handle.
### 37.29 Virtual interface
vpiTypespec
virtual interface var interface typespec
-> name
str: vpiName interface expr
str: vpiFullName
-> is modport interface
bool: vpiIsModPort
modport
virtual interface var
vpiExpr
ref obj
constant
interface
vpiActual modport
Details:
1) The vpiExpr relation shall return the interface instance assigned to the virtual interface in its declaration, if any;
otherwise, vpiExpr shall return NULL.
2) A ref obj may be an interface expr only if it is a local declaration of an interface or modport passed through a port.
A constant may be an interface expr only if it has a vpiConstType of vpiNullConst.
Example 1: Passing an interface or modport through a port:
interface SBus #(parameter WIDTH=8);
logic req, grant;
logic [WIDTH-1:0] addr, data;
modport phy(input addr, inout data);
endinterface
module top;
parameter SIZE = 4;
virtual SBus#(16) V16;
virtual SBus#(32).phy V32_Array [1:SIZE];
...
endmodule

<!-- Page 1041 -->

IEEE Std In this example, V16 is a virtual interface, while V32_Array is an array var. The vpiVariables iteration from
module top includes both V16 and V32_Array, while the vpiVirtualInterfaceVar iteration returns V16
together with the individual elements of V32_Array, that is, V32_Array[1] through V32_Array[4].
Example 2: Virtual interface declaration in a class definition:
interface SBus; // A Simple bus interface
logic req, grant;
logic [7:0] addr, data;
endinterface
class SBusTransactor; // SBus transactor class
virtual SBus bus; // virtual interface of type SBus
function new( virtual SBus s );
bus = s; // initialize the virtual interface
endfunction
task request(); // request the bus
bus.req <= 1'b1;
endtask
task wait_for_bus(); // wait for the bus to be granted
@(posedge bus.grant);
endtask
endclass
module devA( SBus s ); ... endmodule // devices that use SBus
module devB( SBus s ); ... endmodule
module top;
SBus s[1:4] (); // instantiate 4 interfaces
devA a1( s[1] ); // instantiate 4 devices
devB b1( s[2] );
devA a2( s[3] );
devB b2( s[4] );
initial begin
SbusTransactor t[1:4]; // create 4 bus-transactors and bind
t[1] = new( s[1] );
t[2] = new( s[2] );
t[3] = new( s[3] );
t[4] = new( s[4] );
end
endmodule
A virtual interface var is returned for the left-hand side expression of the statement “bus = s” in the constructor
of the class definition SBusTransactor. The vpiName of the virtual interface var is “bus”, and it has a
vpiInterfaceTypespec for which the vpiDefName is “SBus”. The vpiActual relationship returns the interface
instance associated with that particular call to new after the assignment has executed. For example, if it was
“new(s[1])”, vpiActual would return the interface s[1]. If vpiActual is queried before the assignment is
executed, the method shall return NULL if the virtual interface is uninitialized. In addition, the right-hand side
expression of “bus = s” returns a virtual interface var for which vpiActual is the interface instance passed to the
call to new.

<!-- Page 1042 -->

IEEE Std ### 37.30 Interface typespec
interface typespec
vpiParent
interface typespec param assign
-> name
str: vpiName
-> def name
str: vpiDefName
-> is modport
bool: vpiIsModPort
Details:
1) The vpiDefName of an interface typespec that represents a modport shall be the modport identifier. The
vpiDefName of an interface typespec that represents an interface shall be the identifier of the interface declaration.
2) For an interface typespec that represents a modport, vpiParent shall return an interface typespec of the
corresponding interface. For an interface typespec that represents an interface, vpiParent shall return NULL.
3) In the following example, the first typedef defines an interface typespec corresponding to “virtual
SBus#(16)” whose vpiName is SB16. The vpiDefname of this typespec shall be SBus, and the assigned
parameter value of 16 shall be derived by iterating on vpiParamAssign. The typedef SBphy, however, is an array
typespec for which the vpiElemTypespec returns an interface typespec corresponding to “virtual
SBus#(32).phy”.
The vpiTypedef iteration from the module top returns handles to both SB16 and SBphy interface typespecs.
interface SBus #(parameter WIDTH=8);
logic req, grant;
logic [WIDTH-1:0] addr, data;
modport phy(input addr, inout data);
endinterface
module top;
parameter SIZE = 4;
typedef virtual SBus#(16) SB16;
typedef virtual SBus#(32).phy SBphy [1:SIZE];
...
endmodule

<!-- Page 1043 -->

IEEE Std ### 37.31 Class definition
class typespec
instance
class defn
vpiDerivedClasses scope
variables
extends class defn
vpiMethods
task func
-> name
str: vpiName
constraint
-> virtual
vpiArgument
bool: vpiVirtual
vpiParameter
expr -> declared lifetime parameters
bool: vpiAutomatic
named event
named event array
vpiTypedef
typespec
scope
vpiInternalScope
Details:
1) The iterations over vpiVariables, vpiMethods, vpiNamedEvent, and vpiNamedEventArray shall return both
static and automatic properties or methods. However, the iteration over vpiMethods shall not include built-in
methods for which there is no explicit declaration.
2) vpi_get_value() and vpi_put_value() are not allowed for variable and event handles obtained from class defn
handles.
3) The iterator to constraints returns only normal constraints and not inline constraints.
4) The vpiConstraint iteration shall return the constraints in syntactic declaration order. The position within this
order of a constraint declared as extern shall be determined by the position of its prototype. To get constraints
inherited from base classes, it is necessary to traverse the extends relation to obtain the base class typespec.
5) The vpiDerivedClasses iterator shall return all the class defns derived from the given class defn.
6) The relation to vpiExtends exists whenever one class is derived from another class (refer to 8.13). The relation
from extends to class typespec provides the base class. The vpiArgument iterator from extends shall provide the
arguments used in constructor chaining (refer to 8.17).
7) The vpiParameter iteration shall return both the parameters declared in the parameter port list of the class
declaration and the parameters declared within the body of the class declaration as class items. The property
vpiLocalParam (see 37.28) shall return TRUE for parameters declared within the body.
8) For details on lifetime and memory allocation properties, see 37.3.7.

<!-- Page 1044 -->

IEEE Std ### 37.32 Class typespec
param assign
class typespec
vpiExtends
variables
class defn class typespec
vpiMethods
-> name task func
str: vpiName
-> class type constraint
int: vpiClassType
-> declared lifetime vpiParameter
parameters
bool: vpiAutomatic
virtual interface var
named event
named event array
scope
vpiInternalScope
Details:
1) According to how it is obtained, a class typespec may represent either a lexical construct or a class specialization.
If the class typespec is obtained as part of a class defn, it represents a lexical construct from the SystemVerilog
source code. In particular, it shall represent a lexical construct under the following conditions:
— It is obtained from a class defn via the vpiTypedef iteration. In this case it represents a user-defined typedef.
— It is part of the declaration of a class item (variable or method) obtained from the class defn.
— It is obtained from the extends object associated with the class defn.
A class typespec object that has all parameter values resolved shall represent a class specialization. In particular, it
shall represent a class specialization under the following conditions:
— It is obtained from a class defn by iterating over vpiClassTypespec.
— It is the type of a variable or method for which no containing scope is a class defn. If the variable or method is
declared using the name of a typedef, the class typespec shall be the corresponding class instantiation rather
than the class typespec for the typedef itself.
A class typespec derived from a class defn for which the parameter port list is empty may represent both a lexical
construct and a class specialization.
2) For a class typespec that represents only a lexical construct, the one-to-many relations vpiVariables, vpiMethods,
vpiConstraint, vpiNamedEvent, vpiNamedEventArray, vpiTypedef, and vpiInternalScope are not supported.
3) In the case of a class typespec that represents a lexical construct, if the class type construct includes an explicit
parameter expression or type, the object for that parameter or type shall constitute the vpiRhs part of the
corresponding param assign (see 37.28); otherwise the vpiRhs part shall reference the default expression or type
with which the parameter was declared. However, if the class typespec represents a class specialization, the vpiRhs
of each param assignment may be any object that has the correct value (in the case of a non-type parameter) or type
(in the case of a type parameter).

<!-- Page 1045 -->

IEEE Std 4) A class typespec that represents a class specialization shall have a valid, though tool-dependent, name.
5) From a class typespec that represents a class specialization, the iterations over vpiVariables, vpiMethods,
vpiNamedEvent, and vpiNamedEventArray shall return both static and automatic properties or methods.
However, the iteration over vpiMethods shall not include built-in methods for which there is no explicit
declaration.
6) vpi_get_value() and vpi_put_value() are not allowed for non-static variable and event handles obtained from class
typespec handles.
7) The iterator to constraints returns only normal constraints and not inline constraints.
8) The vpiConstraint iteration shall return the constraints in syntactic declaration order. The position within this
order of a constraint declared as extern shall be determined by the position of its prototype. To get constraints
inherited from a base class typespec, it is necessary to traverse the extends relation to obtain the base class typespec.
9) The vpiExtends relation shall return the base class typespec, if any, from which a given class typespec is derived.
The base class typespec of a class specialization shall also be a specialization.
10) The vpiClassTypespec iteration from a class defn shall return the class specializations derived directly (and not by
inheritance) from that class defn.
11) The vpiVirtualInterfaceVar iteration (formerly vpiInterfaceDecl—now deprecated in this standard—see C.4.3,
item 5) shall return the virtual interface var declarations in the class specialization (see 37.12 detail 7). If an array of
virtual interfaces is declared, the vpiVirtualInterfaceVar iteration shall return each element of the array
separately. However, the vpiVariables iteration shall return the array declaration as a single vpiArrayVar.
12) The vpiParameter iteration shall return parameters corresponding both to those declared in the parameter port list
of the class declaration and to those declared within the body of the class declaration as class items. The property
vpiLocalParam (see 37.28) shall return TRUE for parameters declared within the body.
13) The vpiClassDefn relation shall return NULL for built-in classes.
14) For details on lifetime and memory allocation properties, see 37.3.7.

<!-- Page 1046 -->

IEEE Std ### 37.33 Class variables and class objects
class var class obj variables
-> referenced identity -> my identity
class typespec
int64: vpiObjId int64: vpiObjId
vpiWaitingProcesses
thread
class typespec
vpiMessages
expr
vpiMethods
task func
constraint
vpiParameter
parameters
virtual interface var
named event
named event array
scope
vpiInternalScope
Details:
1) The property vpiObjId is a class object’s identifier. It is a property of a live object and guaranteed to be unique
with respect to all other dynamic objects that support this property for as long as the object is alive. After the object
is destroyed by garbage collection, its particular vpiObjId value may be reused.
2) For a class var, its vpiObjId is the identifier of the object it references or 0, indicating it is not referencing any
object.
3) The vpiWaitingProcesses iterator on a mailbox or semaphore shall return the threads waiting on the class object or
object resource. A waiting process is a static or dynamic process represented by its suspended thread. A process
may be waiting to retrieve a message from a mailbox or waiting for a semaphore resource key.
4) A vpiMessages iteration shall return all the messages in a mailbox.
5) For a class var, vpiClassTypespec shall return the class typespec with which the class var was declared in the
SystemVerilog source text. If the class var has the value of NULL, the vpiClassObj relationship applied to the class
var shall return a null handle. vpiClassTypespec when applied to a class obj handle shall return the class typespec
with which the class obj was created. The difference between the two usages of vpiClassTypespec can be seen in
the following example:
class Packet;
...
endclass : Packet
class LinkedPacket extends Packet;
...
endclass : LinkedPacket
LinkedPacket l = new;
Packet p = l;
In this example, the vpiClassTypespec of variable p is Packet, but the vpiClassTypespec of the class obj
associated with variable p is “LinkedPacket”.
NOTE—When a class var is obtained as a data member of a class typespec, the application shall use vpiScope (see
37.12) rather than vpiClassTypespec to obtain the enclosing scope.

<!-- Page 1047 -->

IEEE Std 6) From a class obj, the iterations over vpiVariables, vpiMethods, vpiNamedEvent, and vpiNamedEventArray
shall return both static and automatic properties or methods. However, the iteration over vpiMethods shall not
include built-in methods for which there is no explicit declaration.
7) The vpiVirtualInterfaceVar iteration (formerly vpiInterfaceDecl—now deprecated in this standard—see C.4.3,
item 5) shall return the virtual interface var declarations in the class object. If an array of virtual interfaces is
declared, the vpiVirtualInterfaceVar iteration shall return each element of the array separately. However, the
vpiVariables iteration shall return the array declaration as a single vpiArrayVar.
8) The vpiParameter iteration shall return parameters corresponding both to those declared in the parameter port list
of the class declaration and to those declared within the body of the class declaration as class items. The property
vpiLocalParam (see 37.28) shall return TRUE for parameters declared with the body. The value of a parameter
derived from a class obj shall be the same as that of the same parameter derived from the corresponding class
typespec.
9) vpi_handle_by_name() shall accept a full name to a non-static data member, even though it does not have a
vpiFullName property. For example:
module top;
class Packet;
integer Id;
...
endclass
Packet p;
c = p.Id;
...
vpi_handle_by_name() accepts “top.p.Id”.
10) For details on class object specific callbacks, see 38.36.1.

<!-- Page 1048 -->

IEEE Std ### 37.34 Constraint, constraint ordering, distribution
vpiParent
class obj constraint constraint item
-> virtual
constraint ordering
bool: vpiVirtual
-> lifetime (static/automatic)
bool: vpiAutomatic constraint expr
-> memory allocation
int: vpiAllocScheme
-> access
int: vpiAccessType
-> name
str: vpiName
str: vpiFullName
-> active
bool: vpiIsConstraintEnabled
vpiSolveBefore
expr
constraint ordering
vpiSolveAfter
expr
expr
vpiValueRange
distribution dist item
range
-> distribution type
int: vpiDistType
vpiWeight
expr expr
Details:
1) For a constraint, vpiAutomatic property does not mean lifetime, but reflects the keyword used in the constraint
declaration. vpiAutomatic == 0 implies the constraint was declared static. See 18.5.10 for meaning.
2) For details on memory allocation property, see 37.3.7.
3) Possible return values for the vpiAccessType property for a constraint are vpiExternAcc or zero, indicating
whether it was declared outside its enclosing class declaration or not (see 18.5.1).
4) The vpiConstraint iteration shall return the constraints in syntactic declaration order. The position within this
order of a constraint declared as extern shall be determined by the position of its prototype.
5) The vpiConstraintItem iteration shall return the constraint items in the order in which they occur within the
constraint.

<!-- Page 1049 -->

IEEE Std ### 37.35 Primitive, prim term
module
expr
primitive array
vpiDelay
expr primitive
prim term
gate -> direction
expr
int: vpiDirection
vpiIndex
-> index
switch
vpiTermIndex
-> value
udp defn udp vpi_get_value()
-> array member
bool: vpiArray (deprecated)
bool: vpiArrayMember
-> definition name
str: vpiDefName
-> delay
vpi_get_delays()
vpi_put_delays()
-> name
str: vpiName
str: vpiFullName
-> primitive type
int: vpiPrimType
-> number of inputs
int: vpiSize
-> strength
int: vpiStrength0
int: vpiStrength1
-> value
vpi_get_value()
vpi_put_value()
Details:
1) vpiSize shall return the number of inputs.
2) For primitives, vpi_put_value() shall only be used with sequential UDP primitives.
3) vpiTermIndex can be used to determine the terminal order. The first terminal has a term index of zero.
4) If a primitive is an element within a primitive array, the vpiIndex transition is used to access the index within the
array. If a primitive is not part of a primitive array, this transition shall return NULL.

<!-- Page 1050 -->

IEEE Std ### 37.36 UDP
udp
udp defn io decl
-> definition name
str: vpiDefName
-> number of inputs
int: vpiSize table entry
-> protected -> number of symbol entries
bool: vpiProtected int: vpiSize
-> type -> value
int: vpiPrimType vpi_get_value()
initial
Details:
1) Only string (decompilation) and vector (ASCII values) shall be obtained for table entry objects using
vpi_get_value(). Refer to the definition of vpi_get_value() for additional details.
2) vpiPrimType returns vpiSeqPrim for sequential UDPs and vpiCombPrim for combinational UDPs.
### 37.37 Intermodule path
inter mod path ports
-> delay
vpi_get_delays()
vpi_put_delays()
Details:
1) To get to an intermodule path, vpi_handle_multi(vpiInterModPath, port1, port2) can be used.

<!-- Page 1051 -->

IEEE Std ### 37.38 Constraint expression
constraint expr
implication
vpiCondition
expr constr if constraint expr
vpiElseConst
constr if else
constraint expr
constr foreach
distribution
expr
-> soft constraint
bool: vpiSoft
soft disable expr
variables
vpiLoopVars variables
constr foreach
operation
constraint expr
Details:
1) The variable obtained via the vpiVariables relation from a vpiConstrForeach shall represent the array being
indexed.
2) The vpiLoopVars iteration shall return the index variables of the foreach constraint in left-to-right order. If an
index variable is skipped, its place shall be represented as a vpiOperation for which the vpiOpType is vpiNullOp.
3) Each vpiConstraintExpr iteration shall return the expressions in the order in which they occur in the containing
implication, if, if-else, or foreach constraint.

<!-- Page 1052 -->

IEEE Std ### 37.39 Module path, path term
vpiCondition
module expr expr
vpiDelay
expr
vpiModDataPathIn
path term
vpiModPathOut
path term
mod path
-> delay
path term
vpi_get_delays() vpiModPathIn
vpi_put_delays() -> direction
int: vpiDirection
-> path type
int: vpiPathType -> edge
int: vpiEdge
-> polarity
int: vpiPolarity
int: vpiDataPolarity
-> hasIfNone interface
bool: vpiModPathHasIfNone
vpiInstance module
Details:
1) Specify blocks can occur in both modules and interfaces. For backwards compatibility the vpiModule relation has
been preserved; however this relation shall return NULL for specify blocks in interfaces. For new code, it is
recommended that the vpiInstance relation be used instead.

<!-- Page 1053 -->

IEEE Std ### 37.40 Timing check
module
vpiExpr expr
tchk term
vpiTchkRefTerm
tchk term
vpiTchkDataTerm
expr tchk tchk term
vpiDelay
-> limit
vpi_get_delays()
vpi_put_delays()
regs
-> tchk type
vpiTchkNotifier
int: vpiTchkType
expr
tchk term
expr
-> edge
vpiCondition
int: vpiEdge
Details:
1) For the timing checks in 31.2 the relationship vpiTchkRefTerm shall denote the reference_event or
controlled_reference_event, while vpiTchkDataTerm shall denote the data_event, if any.
2) When iterating over vpiExpr from a tchk, the handles returned for a reference_event, a controlled_reference_event,
or a data_event shall have the type vpiTchkTerm. All other arguments shall have types matching the expression.

<!-- Page 1054 -->

IEEE Std ### 37.41 Task and function declaration
vpiLeftRange
expr
io decl task func
expr
vpiRightRange
func call function
-> sign vpiReturn
variables
bool: vpiSigned
-> size
class defn
int: vpiSize
-> type vpiParent
ref obj
int: vpiFuncType
task call task
-> method -> pure DPI
bool: vpiMethod bool: vpiDPIPure
-> access -> context
int: vpiAccessType bool: vpiDPIContext
-> visibility -> DPI qualifier
int: vpiVisibility int: vpiDPICStr
-> virtual -> identifier
bool: vpiVirtual str: vpiDPICIdentifier
-> default lifetime
bool: vpiAutomatic
Details:
1) A SystemVerilog function shall contain an object with the same name, size, and type as the function. This object
shall be used to capture the return value for this function.
2) For a function where the return type is a user-defined type, vpi_handle(vpiReturn, function_handle) shall return
the implicit variable handle representing the return of the function from which the user can get the details of that
user-defined type.
3) vpiReturn shall always return a var object, even for simple returns.
4) vpiVisibility denotes the visibility (local, protected, or default) of a task or function that is a class member (a
method). vpiVisibility shall return vpiPublicVis for a class member that is not local or protected, or for a task or
function that is not a class member.
5) vpiFullName of a task or function declared inside a package or class defn shall begin with the full name of the
package or class defn followed by “::” and immediately followed with the name of the task or function.
6) vpiAccessType shall return vpiDPIExportAcc for "DPI" and "DPI-C" export functions/tasks, and shall return
vpiDPIImportAcc for "DPI" and "DPI-C" import functions/tasks.
7) vpiDPIPure shall return TRUE for pure "DPI" and "DPI-C" import functions.
8) vpiDPIContext shall return TRUE for context import "DPI" and "DPI-C", functions/tasks.
9) vpiDPICStr shall return vpiDPI for a "DPI" function/task, and vpiDPIC for a "DPI-C" function/task.
10) vpiDPICIdentifier shall return a string corresponding to the C linkage name for the "DPI"/"DPI-C" function/task.
11) For details on lifetime and memory allocation properties, see 37.3.7.
12) If the vpiSize of the vpiReturn variable is defined (see 37.17, detail 9) and can be determined without evaluating
the function, vpiSize for the function shall return the same value as vpiSize for the vpiReturn variable. For a void
function, vpiSize shall return 0. For all other cases the behavior of vpiSize is undefined.

<!-- Page 1055 -->

IEEE Std ### 37.42 Task and function call
vpiArgument expr
scope tf call
interface expr
task call
scope
func call
primitive
-> type
named event
int: vpiFuncType
-> value
function named event array
vpi_get_value()
vpiPrefix
task expr
method func call
-> value vpiWith expr
vpi_get_value()
constraint
method task call
-> user-defined
bool: vpiUserDefn
sys func call
-> type
int: vpiFuncType
vpiSysTfCall
-> value user systf
vpi_get_value()
-> systf info
vpi_put_value()
p_vpi_systf_data:
vpi_get_systf_info()
sys task call
-> user-defined
bool: vpiUserDefn
-> decompile
str: vpiDecompile
-> name
str: vpiName
Details:
1) The vpiWith relation is only available for randomize methods (see 18.7) and for array locator methods (see 7.12.1).
2) For methods (method func call, method task call), the vpiPrefix relation shall return the object to which the method
is being applied. For example, for the class method invocation
packet.send();
the prefix for the “send” method is the class var “packet”.
3) The system task or function that invoked an application shall be accessed with vpi_handle(vpiSysTfCall, NULL).
4) vpi_get_value() shall return the current value of the system function.

<!-- Page 1056 -->

IEEE Std 5) If the vpiUserDefn property of a system task or function call is true, then the properties of the corresponding systf
object shall be obtained via vpi_get_systf_info().
6) All user-defined system tasks or functions shall be retrieved using vpi_iterate(), with vpiUserSystf as the type
argument, and a NULL reference argument.
7) The simulator shall not evaluate arguments to system tasks or functions when calling those tasks or functions
(36.4). Effectively, the value of any argument expression, or of any operand or argument of the expression, is not
known until an application asks for it using vpi_get_value() (38.15), a cbValueChange callback (38.36.1), or other
equivalent operation. If no application asks for the value of the argument, it is never evaluated.
8) An empty (omitted) argument (see 21.2.1) shall be represented as an expression with a vpiType of vpiOperation
and a vpiOpType of vpiNullOp. An argument consisting of the special value null shall be represented as an
expression with a vpiType of vpiConstant and a vpiConstType of vpiNullConst.
Example:
logic my_var;
$my_task(my_var, "", , null, );
In the call to the user-defined system task $my_task(), my_var is an ordinary argument of type vpiLogicVar.
The second argument, an empty string (but not an empty argument), is a vpiConstant for which the vpiConstType
is vpiStringConst. The third and fifth arguments are empty arguments, while the fourth argument is a vpiConstant
with a vpiConstType of vpiNullConst. VPI shall represent the third and fifth arguments as vpiOperations with a
vpiOpType of vpiNullOp.
9) The property vpiDecompile shall return a string with a functionally equivalent system task or function call to what
was in the original source code. The arguments shall be decompiled using the same manner as any expression is
decompiled. See 37.59 for a description of expression decompilation.
10) System task and function calls that are protected shall allow iteration over the vpiArgument relationship.
11) For a built-in method func call, vpiFunction shall return NULL, while vpiTask shall return NULL for a built-in
method task call.

<!-- Page 1057 -->

IEEE Std ### 37.43 Frames
scope
task call
func call
method task call
method func call
nets
vpiOrigin
stmt
vpiParent
frame frame thread
-> active
bool: vpiActive vpiAutomatics
variables
named event
named event array
Details:
1) A frame shall represent any dynamically activated procedural scope, together with its locally declared automatic
variables, events, and event arrays, if any.
2) It shall be illegal to place value change callbacks on automatic variables.
3) It shall be illegal to put a value with a delay on automatic variables.
4) There is at most only one active frame at any time in a given thread. To get a handle to the currently active frame,
use vpi_handle(vpiFrame, NULL). The frame to stmt transition shall return the currently active statement within
the frame.
5) The vpiParent relation shall indicate the frame from which the child frame was activated. For the parent frame, the
frame to stmt transition shall indicate the atomic statement or scope whose execution activated the child frame.
6) The vpiOrigin relation shall indicate the point in the elaboration hierarchy from which the frame was activated.
The vpiOrigin may be a net or net array that belongs to a nettype with a user-defined resolution function; in this
case the frame shall correspond to the currently active resolution function.
7) The frame object model is not backwards compatible with IEEE Std 1800-2017 and all preceding versions of IEEE
Std 1800.
8) For details on frame specific callbacks, see 38.36.1.

<!-- Page 1058 -->

IEEE Std ### 37.44 Threads
thread
vpiParent
vpiOrigin
stmt
frame thread
-> active
bool: vpiActive
thread
Details:
1) A thread is a SystemVerilog process such as an always procedure or a branch of a fork construct. As a thread
works its way down a call chain of tasks and/or functions, a new frame is activated as each new task or function is
entered.
2) For details on thread specific callbacks, see 38.36.1.
### 37.45 Delay terminals
module
vpiInTerm vpiDriver
delay term net drivers
delay device
-> delay type
delay term net loads
int: vpiDelayType vpiOutTerm
vpiLoad
-> delay type
int: vpiDelayType
-> value
vpi_get_value()
Details:
1) The value of the input delay term shall change before the delay associated with the delay device.
2) The value of the output delay term shall not change until after the delay has occurred.

<!-- Page 1059 -->

IEEE Std ### 37.46 Net drivers and loads
vpiDriver vpiLoad
net drivers nets net loads
ports delay term
force assign stmt
delay term force
cont assign cont assign
cont assign bit cont assign bit
prim term prim term
ports
Details:
1) Complex expressions on input ports that are not concatenations shall be considered a load for a net. Iterating on
loads for trinet in the following example will cause the fourth port of ram to be a load:
module my_module;
tri trinet;
ram r0 (a, write, read, !trinet);
endmodule
Access to the complex expression shall be available using vpi_handle(vpiHighConn, portH) where portH is the
handle to the port returned when iterating on loads.

<!-- Page 1060 -->

IEEE Std ### 37.47 Continuous assignment
vpiDelay expr
expr vpiRhs
expr
vpiLhs
module cont assign
vpiParent
vpiBit
cont assign bit
-> offset from LSB
int: vpiOffset
-> delay
vpi_get_delays()
-> net decl assign
bool: vpiNetDeclAssign
-> strength
int: vpiStrength0
int: vpiStrength1
-> value
vpi_get_value()
Details:
1) The size of a cont assign bit is always scalar.
2) Callbacks for value changes can be placed onto cont assign or a cont assign bit.
3) vpiOffset shall return zero for the LSB.

<!-- Page 1061 -->

IEEE Std ### 37.48 Clocking block
vpiInputSkew vpiOutputSkew
delay control delay control
vpiClockingEvent instance
event control clocking block
-> name clocking io decl
str: vpiName
str: vpiFullName property decl
-> edge
int: vpiInputEdge sequence decl
int: vpiOutputEdge
vpiPrefix
virtual interface var
vpiActual
clocking block
vpiInputSkew vpiOutputSkew
delay control expr
nets
vpiExpr
clocking io decl
-> direction variables
int: vpiDirection
-> name ref obj
str: vpiName
-> edge
int: vpiInputEdge
int: vpiOutputEdge
Details:
1) The methods, vpiInputSkew and vpiOutputSkew, and properties vpiInputEdge and vpiOutputEdge, on the
clocking block apply to the default constructs. The same methods and properties on the clocking io decl apply to
the clocking io decl itself.
2) The vpiPrefix relation shall be non-NULL when the clocking block represents an expression in the SystemVerilog
source code immediately prefixed by a virtual interface.
3) If a prefix of a clocking block is a virtual interface that has no value at the current simulation time, the vpiActual
relation shall return NULL.
4) vpiExpr shall return the expression or ref obj referenced by the clocking io decl. Consider input
enable = top.mem1.enable. Here, “enable” is represented by a clocking io decl, and the vpiExpr
relation returns a handle to “top.mem1.enable”.

<!-- Page 1062 -->

IEEE Std ### 37.49 Assertion
assertion
instance clocking block
sequence inst
assert
assume
cover
restrict
property inst
immediate assert
immediate assume
immediate cover
-> location
str: vpiFile
int: vpiStartLine
int: vpiColumn
int: vpiEndLine
int: vpiEndColumn
-> assertion name
str: vpiName
Details:
1) For details on using VPI to obtain static and dynamic assertion information as well as assertion callbacks and
control, see Clause39.
2) For details on using VPI to obtain assertion coverage, see 40.5.2.

<!-- Page 1063 -->

IEEE Std ### 37.50 Concurrent assertion
vpiClockingEvent
expr concurrent assertion stmt
cover
-> is cover sequence
bool:vpiIsCoverSequence
vpiElseStmt
assert stmt
assume
restrict vpiProperty
property inst
-> name
str: vpiName
property spec
str: vpiFullName
-> is clock inferred
bool:vpiIsClockInferred
vpiDisableCondition
expr
distribution
Details:
1) Clocking event is always the actual clocking event on which the assertion is being evaluated, regardless of whether
this is explicit or implicit (inferred).
2) The restrict property statement has no pass and no fail action statement. Also, it is not simulated and hence
generates no run-time information.

<!-- Page 1064 -->

IEEE Std ### 37.51 Property declaration
property inst property decl variables
-> name
prop formal decl
str: vpiName property spec
str: vpiFullName
vpiDisableCondition
expr
vpiArgument property expr
property inst
named event
property decl
vpiExpr named event
prop formal decl
property expr
-> name
str: vpiName
-> direction
typespec
int: vpiDirection
Details:
1) The vpiPropFormalDecl iterator shall return the property declaration arguments in the order that the formals for
the property are declared.
2) The vpiArgument iterator shall return the property instance arguments in the order that the formals for the property
are declared, so that the correspondence between each argument and its respective formal can be made. If a formal
has a default value, that value shall appear as the argument should the instantiation not provide a value for that
argument.
3) The vpiTypespec relation shall return NULL if the formal is untyped.
4) If the formal has an initialization expression, the expression can be obtained using the vpiExpr relation.
5) vpiDirection returns vpiNoDirection if the formal argument is not a local variable argument. Otherwise,
vpiDirection returns vpiInput.

<!-- Page 1065 -->

IEEE Std ### 37.52 Property specification
expr property spec property expr
vpiClockingEvent
expr
distribution vpiDisableCondition
property expr
vpiOperand
operation property expr
-> operation type
int: vpiOpType
-> operator strength
bool: vpiOpStrong
sequence expr
multiclock
sequence expr
property inst vpiClockingEvent
expr
clocked property
property expr
vpiCondition
expr
case property
case property item expr
property expr
Details:
1) Variables are declarations of property variables. The value of these variables cannot be accessed.
2) Within the context of a property expr, vpiOpType can be any one of vpiAcceptOnOp, vpiAlwaysOp,
vpiCompAndOp, vpiCompOrOp, vpiEventuallyOp, vpiIfElseOp, vpiIfOp, vpiIffOp, vpiImpliesOp,
vpiNexttimeOp, vpiNonOverlapFollowedByOp, vpiNonOverlapImplyOp, vpiNotOp,
vpiOverlapFollowedByOp, vpiOverlapImplyOp, vpiRejectOnOp, vpiSyncAcceptOnOp,
vpiSyncRejectOnOp, vpiUntilOp, or vpiUntilWithOp.
Operands to these operations shall be provided in the same order as shown in the BNF, with the following
exceptions:
— vpiNexttimeOp: Arguments shall be: property, constant. constant shall only be given if different from 1.
— vpiAlwaysOp and vpiEventuallyOp: Arguments shall be: property, left range, right range.
3) vpiOpStrong is valid only for operations vpiNexttimeOp, vpiAlwaysOp, vpiEventuallyOp, vpiUntilOp,
vpiUntilWithOp, and for sequence expression. vpiOpStrong shall return TRUE to indicate the strong version of
the corresponding operator.
4) The case property item shall group all case conditions that branch to the same property statement.
5) vpi_iterate() shall return NULL for the default case item because there is no expression with the default case.

<!-- Page 1066 -->

IEEE Std ### 37.53 Sequence declaration
variables
sequence inst
sequence decl
-> name
str: vpiName
str: vpiFullName vpiExpr sequence expr
seq formal decl
multiclock
sequence expr
vpiExpr named event
seq formal decl
sequence expr
-> name
str: vpiName
-> direction
typespec
int: vpiDirection
Details:
1) The vpiSeqFormalDecl iterator shall return the sequence declaration arguments in the order that the formals for the
sequence are declared.
2) The vpiTypespec relation shall return NULL if the formal is untyped.
3) If the formal has an initialization expression, the expression can be obtained using the vpiExpr relation.
4) vpiDirection returns vpiNoDirection if the formal argument is not a local variable argument. Otherwise,
vpiDirection returns either vpiInput, vpiOutput, or vpiInout.

<!-- Page 1067 -->

IEEE Std ### 37.54 Sequence expression
sequence expr
vpiOperand
operation sequence expr
-> operation type
int: vpiOpType
vpiArgument named event
sequence decl sequence inst
sequence expr
distribution
vpiMatchItem assignment
expr
tf call
Details:
1) The vpiArgument iterator shall return the sequence instance arguments in the order that the formals for the
sequence are declared, so that the correspondence between each argument and its respective formal can be made. If
a formal has a default value, that value shall appear as the argument should the instantiation not provide a value for
that argument.
2) Within a sequence expression, vpiOpType can be any one of vpiCompAndOp, vpiIntersectOp, vpiCompOrOp,
vpiFirstMatchOp, vpiThroughoutOp, vpiWithinOp, vpiUnaryCycleDelayOp, vpiCycleDelayOp,
vpiRepeatOp, vpiConsecutiveRepeatOp, or vpiGotoRepeatOp.
3) For operations, the operands shall be provided in the same order as the operands appear in BNF, with the following
exceptions:
— vpiUnaryCycleDelayOp: Arguments shall be: sequence, left range, right range. Right range shall only be
given if different from left range.
— vpiCycleDelayOp: Arguments shall be: left-hand side sequence, right-hand side sequence, left range, right
range. Right range shall only be provided if different than left range.
— All the repeat operators: The first argument shall be the sequence being repeated, and the next argument shall be
the left repeat bound, followed by the right repeat bound. The right repeat bound shall only be provided if
different from left repeat bound.
and, intersect, or,
first_match,
throughout, within,
##,
[*], [=], [->]

<!-- Page 1068 -->

IEEE Std ### 37.55 Immediate assertions
stmt
expr immediate assert
vpiElseStmt
-> is deferred stmt
int: vpiIsDeferred
-> is final
int: vpiIsFinal
stmt
expr immediate assume
vpiElseStmt
-> is deferred
stmt
int: vpiIsDeferred
-> is final
int: vpiIsFinal
expr immediate cover stmt
-> is deferred
int: vpiIsDeferred
-> is final
int: vpiIsFinal

<!-- Page 1069 -->

IEEE Std ### 37.56 Multiclock sequence expression
multiclock
clocked seq
sequence expr
vpiClockingEvent
expr
clocked seq
sequence expr
### 37.57 Let
vpiArgument
let expr expr
let decl expr
-> name
str: vpiName
seq formal decl
Details:
1) The vpiArgument iterator shall return the let expression arguments in the order that the formals for the let are
declared, so that the correspondence between each argument and its respective formal can be made. If a formal has
a default value, that value shall appear as the argument should the instantiation not provide a value for that
argument.

<!-- Page 1070 -->

IEEE Std ### 37.58 Simple expressions
vpiUse
simple expr
prim term
nets
path term
tchk term
variables
delay term
ref obj
ports
parameter
stmt
spec param cont assign
cont assign bit
var select
vpiParent vpiIndex
bit select expr
var select
-> name
integer var str: vpiName
str: vpiFullName
time var -> constant select
bool:
parameter vpiConstantSelect
spec param
Details:
1) For vectors, the vpiUse relationship shall access any use of the vector or of the part-selects or bit-selects of the
vector.
2) For bit-selects, the vpiUse relationship shall access any specific use of that bit, any use of the parent vector, and any
part-select that contains that bit.
3) The property vpiConstantSelect shall return TRUE for a bit-select if
— every associated index expression is an elaboration-time constant expression, and
— vpiConstantSelect returns TRUE for the parent of the bit-select.
Otherwise, vpiConstantSelect shall return FALSE.
NOTE—If vpiConstantSelect is TRUE, then if the handle refers to a valid underlying simulation object at the
beginning of simulation (or at any point in the simulation), it refers to the same object at all points in the simulation.
Moreover, if an index expression of the bit-select or of any of its parents is in or out of bounds at the beginning of
simulation, it is in or out of bounds at all subsequent simulation times as well.

<!-- Page 1071 -->

IEEE Std ### 37.59 Expressions
expr typespec
vpiBaseExpr
expr
indexed part select
-> constant selection expr
vpiWidthExpr
bool: vpiConstantSelect
-> index part select type
int: vpiIndexedPartSelectType
simple expr
vpiParent
vpiParent
vpiLeftRange
expr
part select vpiRightRange
-> constant selection expr
bool: vpiConstantSelect
vpiOperand
operation
-> operation type expr
int: vpiOpType
interface expr
constant
-> constant type range
int: vpiConstType
pattern
func call
sequence inst
method func call property inst
sys func call
let expr
-> decompile
str: vpiDecompile
-> size
int: vpiSize
-> value
vpi_get_value()
Details:
1) For an operator whose type is vpiMultiConcatOp, the first operand shall be the multiplier expression. The
remaining operands shall be the expressions within the concatenation.
2) The property vpiDecompile shall return a string with a functionally equivalent expression to the original expression
within the source code. Parentheses shall be added only to preserve precedence. Each operand and operator shall be
separated by a single space character. No additional white space shall be added due to parentheses.

<!-- Page 1072 -->

IEEE Std 3) The cast operation, for which vpiOpType returns vpiCastOp, is represented as a unary operation, with its sole
argument being the expression being cast, and the typespec of the cast expression being the type to which the
argument is being cast.
4) The constant type vpiUnboundedConst represents the $ value used in assertion ranges.
5) The one-to-one relation to typespec shall always be available for vpiCastOp operations, for simple expressions,
and for vpiAssignmentPatternOp and vpiMultiAssignmentPatternOp expressions when the curly braces of the
assignment pattern are prefixed by a data type name to form an assignment pattern expression. For other
expressions, it is implementation dependent as to whether or not there is any associated typespec.
6) For an operation of type vpiAssignmentPatternOp, the operand iteration shall return the expressions as if the
assignment pattern were written with the positional notation. Nesting of assignment patterns shall be preserved.
Example 1:
struct {
int A;
struct {
logic B;
real C;
} BC1, BC2;
} ABC = '{BC1: '{1'b1, 1.0}, int: 0, BC2: '{default: 0}};
The assignment pattern that initializes the struct variable ABC uses member, type, and default keys. The
vpiOperand traversal would represent this assignment pattern expression as:
'{0, '{1'b1, 1.0}, '{0, 0}}
or some other equivalent positional assignment pattern.
Example 2:
logic [2:0] varr [0:3] = '{3: 3'b1, default: 3'b0};
The assignment pattern that initializes the array variable varr uses index and default keys. The vpiOperand
traversal would represent this assignment pattern as:
'{3'b0, 3'b0, 3'b0, 3'b1}
7) For an operator whose type is vpiMultiAssignmentPatternOp, the first operand shall be the multiplier expression.
The remaining operands shall be the expressions within the assignment pattern.
Example:
bit unpackedbits [1:0];
initial unpackedbits = '{2 {y}} ; // same as '{y, y}
For the assignment pattern '{2{y}}, the vpiOpType property shall return vpiMultiAssignmentPatternOp, and
the first operand shall be the constant 2. The next operand shall represent the expression y.
8) Expressions that are protected shall permit access to the vpiSize property.
9) The property vpiConstantSelect shall return TRUE for a part-select or indexed part-select if
— vpiConstantSelect returns TRUE for its parent, and
— the parent is a packed or unpacked array with static bounds, and
— each range expression in the part-select or indexed part-select is an elaboration-time constant expression.
Otherwise, vpiConstantSelect shall return FALSE.
NOTE—If vpiConstantSelect is TRUE, then if the handle refers to a valid underlying simulation object at the
beginning of simulation (or at any point in the simulation), it refers to the same object at all points in the simulation.
Moreover, if any index expression of the part-select or indexed part-select or of any of its parents is in or out of
bounds at the beginning of simulation, it is in or out of bounds at all subsequent simulation times as well.

<!-- Page 1073 -->

IEEE Std 10) For a part-select or indexed part-select, the vpiParent object shall correspond to the expression formed by
removing the part-select range from the expression represented by the part-select or indexed part-select itself. For
example, given the declaration
logic [0:3][7:0] r [1:4];
then the parents of various part-selects or indexed part-selects shall be as shown in Table37-1:
Table37-1—Part-select parent expressions
Part-select or indexed
Parent expression
part-select expression
r[4][3][1:0] r[4][3]
r[i+1][3][j+:2] r[i+1][3]
r[0][j-:4] r[0]
r[0:2] r

<!-- Page 1074 -->

IEEE Std ### 37.60 Atomic statement
atomic stmt
if
if else
while
repeat
waits
case
for
delay control
event control
event stmt
assignment
assign stmt
deassign
disables
tf call
forever
force
release
do while
expect stmt
foreach stmt
return stmt
break
continue
immediate assert
immediate assume
immediate cover
null stmt
-> label
str: vpiName

<!-- Page 1075 -->

IEEE Std Details:
1) The vpiName property shall provide the statement label if one was given; otherwise, the name is NULL.
### 37.61 Dynamic prefixing
simple expr
indexed part select
class var
part select
virtual interface var
named event vpiPrefix
clocking block
named event array
tf call
-> has actual
bool: vpiHasActual
Details:
1) The vpiPrefix relation shall be non-NULL when the object represents an expression or task call in the
SystemVerilog source code prefixed by a virtual interface or a clocking block, or when the object is all or part of a
non-static class property prefixed by a class var.
2) The memory allocation scheme value for an object for which a class var or virtual interface var vpiPrefix is
non-NULL shall be the same as for the prefix.
3) The property vpiHasActual shall return TRUE:
— whenever the prefix object has a corresponding actual at the current simulation time.
— if the object is all or part of a statically declared object in an elaborated context.
— if the object is part or all of an automatically allocated variable obtained from a frame (see 37.43).
The property vpiHasActual shall return FALSE:
— whenever the prefix object has no corresponding actual at the current simulation time.
— if the object is obtained from a lexical context, such as from a class defn (see 37.31).
— if the object is part or all of a non-static class property variable referenced relative to its class
typespec (see 37.32).
— if the object is part or all of an automatically allocated variable obtained from a task or function
declaration (see 37.41).

<!-- Page 1076 -->

IEEE Std ### 37.62 Event statement
event stmt named event
-> blocking
bool: vpiBlocking
### 37.63 Process
module scope
process stmt
initial scope
final atomic stmt
always
-> always type
int: vpiAlwaysType
Details:
1) vpiAlwaysType can be one of vpiAlways, vpiAlwaysComb, vpiAlwaysFF, or vpiAlwaysLatch.

<!-- Page 1077 -->

IEEE Std ### 37.64 Assignment
vpiLhs
delay control
expr
vpiRhs assignment event control
expr
-> operator
int: vpiOpType repeat control
interface expr
-> blocking
bool: vpiBlocking
Details:
1) vpiOpType shall return vpiAssignmentOp for normal assignments (both blocking “=” and nonblocking “<=”).
For assignment operators, vpiOpType shall return a value that corresponds to the operator that is combined with
the assignment as described in 11.4.1.
For example, the assignment
a += 2;
shall return vpiAddOp for the vpiOpType property.
### 37.65 Event control
vpiCondition
event control “@”
expr
sequence inst
named event
stmt
Details:
1) For event control associated with assignment, the statement shall always be NULL.

<!-- Page 1078 -->

IEEE Std ### 37.66 While, repeat
while
vpiCondition
expr
repeat
stmt
### 37.67 Waits
waits stmt
vpiCondition
wait
sequence inst
vpiCondition
ordered wait
expr
wait fork vpiElseStmt
stmt
### 37.68 Delay control
delay control “#” stmt
-> delay
vpi_get_delays() vpiDelay
expr
Details:
1) For delay control associated with assignment, the statement shall always be NULL.

<!-- Page 1079 -->

IEEE Std ### 37.69 Repeat control
repeat control expr
event control
### 37.70 Forever
forever stmt
### 37.71 If, if–else
if
vpiCondition
expr
stmt
vpiElseStmt
if else stmt
-> qualifier
int: vpiQualifier

<!-- Page 1080 -->

IEEE Std ### 37.72 Case, pattern
vpiCondition
case expr
-> type
int: vpiCaseType pattern
vpiExpr
-> qualifier case item
int: vpiQualifier expr
pattern stmt
any pattern
-> name pattern
str: vpiName
tagged pattern typespec
-> name
str: vpiName
struct pattern pattern
-> name
str: vpiName
expr
Details:
1) The case item shall group all case conditions that branch to the same statement.
2) vpi_iterate() shall return NULL for the default case item because there is no expression with the default case.

<!-- Page 1081 -->

IEEE Std ### 37.73 Expect
property spec
expect stmt stmt
vpiElseStmt
stmt
### 37.74 For
vpiForInitStmt
stmt
for vpiForIncStmt
stmt
-> has local variables
int: vpiLocalVarDecls vpiForInitStmt
stmt
vpiCondition
expr
vpiForIncStmt
stmt
stmt
### 37.75 Do-while, foreach
vpiCondition
expr
do while
stmt
variables
vpiLoopVars variables
foreach stmt
operation
stmt
Details:
1) The variable obtained via the vpiVariables relation from a foreach stmt shall represent the packed array, unpacked
array, or string var being indexed.
2) The vpiLoopVars iteration shall return the index variables of the foreach statement in left-to-right order. If an
index variable is skipped, its place shall be represented as a vpiOperation for which the vpiOpType is vpiNullOp.

<!-- Page 1082 -->

IEEE Std ### 37.76 Alias statement
vpiLhs
expr
instance alias stmt
vpiRhs
expr
Example:
alias a=b=c=d;
results in 3 aliases:
alias a=d;
alias b=d;
alias c=d;
d is the right-hand side for all.
### 37.77 Disables
disables
vpiExpr
disable
task
disable fork
function
named begin
named fork
### 37.78 Return statement
vpiCondition
return stmt expr

<!-- Page 1083 -->

IEEE Std ### 37.79 Assign statement, deassign, force, release
vpiRhs
force expr
vpiLhs
assign stmt expr
deassign
vpiLhs
expr
release
### 37.80 Callback
prim term callback
-> cb info
expr p_cb_data: vpi_get_cb_info()
time queue
stmt
Details:
1) To get information about the callback object, the routine vpi_get_cb_info() can be used..
2) To get callback objects not related to the above objects, the second argument to vpi_iterate() shall be NULL.

<!-- Page 1084 -->

IEEE Std ### 37.81 Time queue
time queue
-> time
vpi_get_time()
Details:
1) The time queue objects shall be returned in increasing order of simulation time.
2) vpi_iterate() shall return NULL if there is nothing left in the simulation time queue.
3) The current time queue shall only be returned as part of the iteration if there are events that precede read only sync.
### 37.82 Active time format
vpiActiveTimeFormat
tf call
Details:
1) If $timeformat() has not been called, vpi_handle(vpiActiveFormat, NULL) shall return NULL.

<!-- Page 1085 -->

IEEE Std ### 37.83 Attribute
attribute
vpiParent
instances -> name
str: vpiName
port
-> On definition
bool: vpiDefAttribute
net
-> value:
array net vpi_get_value()
-> definition location
variables str: vpiDefFile
int: vpiDefLineNo
named event
prim term
path term
mod path
tchk
param assign
spec param
task func
primitive
table entry
stmt
process
operation
concurrent assertion
sequence decl
property decl
clocking block
class defn
constraint

<!-- Page 1086 -->

IEEE Std ### 37.84 Iterator
vpiUse
iterator
instance array
-> type
int: vpiIteratorType scope
udp defn
ports
nets
net array
regs
reg array
variables
named event array
primitive
prim term
mod path
param assign
inter mod path
path term
delay term
tchk
tf call
process
expr
stmt
case item
frame
time queue
Details:
1) vpi_handle(vpiUse, iterator_handle) shall return the reference handle used to create the iterator.
2) It is possible to have a NULL reference handle, in which case vpi_handle(vpiUse, iterator_handle) shall return
NULL.

<!-- Page 1087 -->

IEEE Std ### 37.85 Generates
vpiInternalScope
scope
module
interface
vpiInstance net
program
array net
module
logic var
array var
gen var vpiMemory
array var
-> name
str: vpiName
variables
str: vpiFullName
named event
gen scope array named event array
-> size
process
int: vpiSize
-> name cont assign
str: vpiName
str: vpiFullName module
-> access by index
vpi_handle_by_index() module array
vpi_handle_by_multi_index()
primitive
primitive array
vpiIndex
expr gen scope
def param
-> array member
bool: vpiArray (deprecated) vpiParameter
parameters
bool: vpiArrayMember
-> name
gen scope array
str: vpiName
str: vpiFullName
program
-> protected
bool: vpiProtected
program array
-> is implicitly declared
bool: vpiImplicitDecl assertion
interface
interface array
alias stmt
clocking block
vpiTypedef
typespec
vpiNetTypedef
nettype decl

<!-- Page 1088 -->

IEEE Std Details:
1) The size for a gen scope array shall be the number of elements in the array.
2) For an unnamed generate, an implicit scope shall be created. Its vpiImplicitDecl property shall return TRUE.
3) References to gen vars within the gen scope shall be treated as local parameters.
4) Parameters within the gen scope shall be treated as local parameters.
5) The vpiTypedef iteration shall return the user-defined typespecs that have typedefs explicitly declared in the
instance.
6) The vpiNetTypedef iteration shall return the handles to the user-defined nettypes that are explicitly declared in the
instance.

<!-- Page 1089 -->

IEEE Std 1800-2023