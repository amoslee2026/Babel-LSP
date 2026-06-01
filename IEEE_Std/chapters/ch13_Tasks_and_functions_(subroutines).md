# Chapter 13: Tasks and functions (subroutines)

## 13. Tasks and functions (subroutines)
### 13.1 General
This clause describes the following:
— Task declarations
— Function declarations
— Calling tasks and functions
### 13.2 Overview
Tasks and functions provide the ability to execute common procedures from several different places in a
description. They also provide a means of breaking up large procedures into smaller ones to make it easier to
read and debug the source descriptions. This clause discusses the differences between tasks and functions,
describes how to define and invoke tasks and functions, and presents examples of each.
Tasks and functions are collectively referred to as subroutines.
The following rules distinguish tasks from functions, with exceptions noted in 13.4.4:
— The statements in the body of a function shall execute in one simulation time unit; a task may
contain time-controlling statements.
— A function cannot enable a task; a task can enable other tasks and functions.
— A nonvoid function shall return a single value; a task or void function shall not return a value.
— A nonvoid function can be used as an operand in an expression; the value of that operand is the
value returned by the function.
For example:
Either a task or a function can be defined to switch bytes in a 16-bit word. The task would return the
switched word in an output argument; therefore, the source code to enable a task called switch_bytes
could look like the following example:
switch_bytes (old_word, new_word);
The task switch_bytes would take the bytes in old_word, reverse their order, and place the reversed
bytes in new_word.
A word-switching function would return the switched word as the return value of the function. Thus, the
function call for the function switch_bytes could look like the following example:
new_word = switch_bytes (old_word);
### 13.3 Tasks
A task shall be enabled from a statement that defines the argument values to be passed to the task and the
variables that receive the results. Control shall be passed back to the enabling process after the task has
completed. Thus, if a task has timing controls inside it, then the time of enabling a task can be different from
the time at which the control is returned. A task can enable other tasks, which in turn can enable still other
tasks—with no limit on the number of tasks enabled. Regardless of how many tasks have been enabled,
control shall not return until all enabled tasks have completed.

<!-- Page 337 -->

IEEE Std The syntax for task declarations is as follows in Syntax13-1.
task_declaration ::= task [ dynamic_override_specifiers ]25 [ lifetime ] task_body_declaration // from A.2.7
task_body_declaration ::=
[ interface_identifier . | class_scope ] task_identifier ;
{ tf_item_declaration }
{ statement_or_null }
endtask [ : task_identifier ]
| [ interface_identifier . | class_scope ] task_identifier ( [ tf_port_list ] ) ;
{ block_item_declaration }
{ statement_or_null }
endtask [ : task_identifier ]
tf_item_declaration ::=
block_item_declaration
| tf_port_declaration
tf_port_list ::= tf_port_item { , tf_port_item }
tf_port_item28 ::=
{ attribute_instance }
[ tf_port_direction ] [ var ] data_type_or_implicit
[ port_identifier { variable_dimension } [ = expression ] ]
tf_port_direction ::=
port_direction
| [ const ] ref [ static ]
tf_port_declaration ::=
{ attribute_instance } tf_port_direction [ var ] data_type_or_implicit list_of_tf_variable_identifiers ;
lifetime ::= static | automatic // from A.2.1.3
signing ::= signed | unsigned // from A.2.2.1
data_type_or_implicit ::=
data_type
| implicit_data_type
implicit_data_type ::= [ signing ] { packed_dimension }
25) The dynamic_override_specifiers shall only be legal on method declarations inside a non-interface class scope.
28) In a tf_port_item, it shall be illegal to omit the explicit port_identifier except within a function_prototype or
task_prototype.
Syntax13-1—Task syntax (excerpt from AnnexA)
A task declaration has the formal arguments either in parentheses (like ANSI C) or in declarations and
directions.
task mytask1 (output int x, input logic y);
...
endtask
task mytask2;
output x;
input y;
int x;
logic y;

<!-- Page 338 -->

IEEE Std ...
endtask
Each formal argument has one of the following directions:
input // copy value in at beginning
output // copy value out at end
inout // copy in at beginning and out at end
ref // pass reference (see 13.5.2)
There is a default direction of input if no direction has been specified. Once a direction is given,
subsequent formals default to the same direction. The const and static qualifiers on the ref direction are
included in this default. In the following example, the formal arguments a and b default to inputs, and u and
v are both outputs:
task mytask3(a, b, output logic [15:0] u, v);
...
endtask
Each formal argument has a data type that can be explicitly declared or inherited from the previous
argument. If the data type is not explicitly declared, then the default data type is logic if it is the first
argument or if the argument direction is explicitly specified. Otherwise, the data type is inherited from the
previous argument.
An array can be specified as a formal argument to a task. For example:
// the resultant declaration of b is input [3:0][7:0] b[3:0]
task mytask4(input [3:0][7:0] a, b[3:0], output [3:0][7:0] y[1:0]);
...
endtask
Multiple statements can be written between the task declaration and endtask. Statements are executed
sequentially, the same as if they were enclosed in a begin...end group. It shall also be legal to have no
statements at all.
A task exits when the endtask is reached. The return statement can be used to exit the task before the
endtask keyword.
A call to a task is also referred to as a task enable (see 13.5 for more details on calling tasks).
Example 1: The following example illustrates the basic structure of a task definition with five arguments:
task my_task;
input a, b;
inout c;
output d, e;
. . . // statements that perform the work of the task
. . .
c = a; // the assignments that initialize result outputs
d = b;
e = c;
endtask
Or using the second form of a task declaration, the task could be defined as follows:
task my_task (input a, b, inout c, output d, e);

<!-- Page 339 -->

IEEE Std . . . // statements that perform the work of the task
. . .
c = a; // the assignments that initialize result variables
d = b;
e = c;
endtask
The following statement calls the task:
initial
my_task (v, w, x, y, z);
The task call arguments (v, w, x, y, and z) correspond to the arguments (a, b, c, d, and e) defined by the
task. At the time of the call, the input and inout type arguments (a, b, and c) receive the values passed in
v, w, and x. Thus, execution of the call effectively causes the following assignments:
a = v;
b = w;
c = x;
As part of the processing of the task, the task definition for my_task places the computed result values into
c, d, and e. When the task completes, the following assignments to return the computed values to the calling
process are performed:
x = c;
y = d;
z = e;
Example 2: The following example illustrates the use of tasks by describing a traffic light sequencer:
module traffic_lights;
logic clock, red, amber, green;
parameter on = 1, off = 0, red_tics = 350,
amber_tics = 30, green_tics = 200;
// initialize colors
initial red = off;
initial amber = off;
initial green = off;
always begin // sequence to control the lights
red = on; // turn red light on
light(red, red_tics); // and wait.
green = on; // turn green light on
light(green, green_tics); // and wait.
amber = on; // turn amber light on
light(amber, amber_tics); // and wait.
end
// task to wait for 'tics' positive edge clocks
// before turning 'color' light off
task light (output color, input [31:0] tics);
repeat (tics) @ (posedge clock);
color = off; // turn light off.
endtask: light
always begin // waveform for the clock
#100 clock = 0;

<!-- Page 340 -->

IEEE Std #100 clock = 1;
end
endmodule: traffic_lights
#### 13.3.1 Static and automatic tasks
Tasks defined within a module, interface, program, or package default to being static, with all declared items
being statically allocated. These items shall be shared across all uses of the task executing concurrently.
Tasks can be defined to use automatic storage in the following two ways:
— Explicitly declared using the optional automatic keyword as part of the task declaration.
— Implicitly declared by defining the task within a module, interface, program, or package that is
defined as automatic.
Tasks defined within a class are always automatic (see 8.6).
All items declared inside automatic tasks are allocated dynamically for each invocation. All formal
arguments and local variables are stored on the stack.
Automatic task items cannot be accessed by hierarchical references. Automatic tasks can be invoked through
use of their hierarchical name.
Specific local variables can be declared as automatic within a static task or as static within an automatic
task.
#### 13.3.2 Task memory usage and concurrent activation
A task may be enabled more than once concurrently. All variables of an automatic task shall be replicated on
each concurrent task invocation to store state specific to that invocation. All variables of a static task shall be
static in that there shall be a single variable corresponding to each declared local variable in a module
instance, regardless of the number of concurrent activations of the task. However, static tasks in different
instances of a module shall have separate storage from each other.
Variables declared in static tasks, including input, output, and inout type arguments, shall retain their
values between invocations. They shall be initialized to the default initialization value as described in 6.8.
Variables declared in automatic tasks, including output type arguments, shall be initialized to the default
initialization value whenever execution enters their scope. input and inout type arguments shall be
initialized to the values passed from the expressions corresponding to these arguments listed in the
task-enabling statements.
Because variables declared in automatic tasks are deallocated at the end of the task invocation, they shall not
be used in certain constructs that might refer to them after that point:
— They shall not be assigned values using nonblocking assignments or procedural continuous
assignments.
— They shall not be referenced by assign or force procedural continuous assignments.
— They shall not be referenced in intra-assignment event controls of nonblocking assignments.
— They shall not be traced with system tasks such as $monitor and $dumpvars.

<!-- Page 341 -->

IEEE Std ### 13.4 Functions
The primary purpose of a function is to return a value that is to be used in an expression. A void function can
also be used instead of a task to define a subroutine that executes and returns within a single time step. The
rest of this clause explains how to define and use functions.
Functions have restrictions that make certain they return without suspending the process that enables them.
The following rules shall govern their usage, with exceptions noted in 13.4.4:
a) A function shall not contain any time-controlling statements. That is, any statements containing #,
##, @, fork-join, fork-join_any, wait, wait fork, wait_order, or expect.
b) A function shall not enable tasks regardless of whether those tasks contain time-controlling
statements.
c) A function may call fine-grain process control methods to kill, suspend,or resume another
process,or kill the current process (see 9.7).
The syntax for defining a function is given in Syntax13-2.
function_declaration ::= // from A.2.6
function [ dynamic_override_specifiers ]25 [ lifetime ] function_body_declaration
function_body_declaration ::=
function_data_type_or_implicit
[ interface_identifier . | class_scope ] function_identifier ;
{ tf_item_declaration }
{ function_statement_or_null }
endfunction [ : function_identifier ]
| function_data_type_or_implicit
[ interface_identifier . | class_scope ] function_identifier ( [ tf_port_list ] ) ;
{ block_item_declaration }
{ function_statement_or_null }
endfunction [ : function_identifier ]
function_data_type_or_implicit ::=
data_type_or_void
| implicit_data_type
data_type ::= // from A.2.2.1
integer_vector_type [ signing ] { packed_dimension }
| integer_atom_type [ signing ]
| non_integer_type
| struct_union [ packed [ signing ] ] { struct_union_member { struct_union_member } }
{ packed_dimension }17
| enum [ enum_base_type ] { enum_name_declaration { , enum_name_declaration } }
{ packed_dimension }
| string
| chandle
| virtual [ interface ] interface_identifier [ parameter_value_assignment ] [ . modport_identifier ]
| [ class_scope | package_scope ] type_identifier { packed_dimension }
| class_type
| event
| ps_covergroup_identifier
| type_reference18
signing ::= signed | unsigned

<!-- Page 342 -->

IEEE Std lifetime ::= static | automatic // from A.2.1.3
17) When a packed dimension is used with the struct or union keyword, the packed keyword shall also be used.
18) When a type_reference is used in a net declaration, it shall be preceded by a net type keyword; and when it is used
in a variable declaration, it shall be preceded by the var keyword.
25) The dynamic_override_specifiers shall only be legal on method declarations inside a non-interface class scope.
Syntax13-2—Function syntax (excerpt from AnnexA)
To indicate the return type of a function, its declaration can either include an explicit data_type_or_void or
use an implicit syntax that indicates only the ranges of the packed dimensions and, optionally, the
signedness. When the implicit syntax is used, the return type is the same as if the implicit syntax had been
immediately preceded by the logic keyword. In particular, the implicit syntax can be empty, in which case
the return type is a logic scalar. A function can also be void, without a return value (see 13.4.1).
A function declaration has the formal arguments either in parentheses (like ANSI C) or in declarations and
directions, as follows:
function logic [15:0] myfunc1(int x, int y);
...
endfunction
function logic [15:0] myfunc2;
input int x;
input int y;
...
endfunction
Functions can have the same formal arguments as tasks. Function argument directions are as follows:
input // copy value in at beginning
output // copy value out at end
inout // copy in at beginning and out at end
ref // pass reference (see 13.5.2)
Function declarations default to the formal direction input if no direction has been specified. Once a
direction is given, subsequent formals default to the same direction. The const and static qualifiers on
the ref direction are included in this default. In the following example, the formal arguments a and b
default to inputs, and u and v are both outputs:
function logic [15:0] myfunc3(int a, int b, output logic [15:0] u, v);
...
endfunction
Each formal argument has a data type that can be explicitly declared or inherited from the previous
argument. If the data type is not explicitly declared, then the default data type is logic if it is the first
argument or if the argument direction is explicitly specified. Otherwise the data type is inherited from the
previous argument. An array can be specified as a formal argument to a function, for example:
function [3:0][7:0] myfunc4(input [3:0][7:0] a, b[3:0]);
...
endfunction

<!-- Page 343 -->

IEEE Std It shall be illegal to call a function with output, inout, or ref arguments in an event expression, in an
expression within a procedural continuous assignment, or in an expression that is not within a procedural
statement. However, a const ref function argument shall be legal in this context (see 13.5.2).
Multiple statements can be written between the function header and endfunction. Statements are executed
sequentially, as if they were enclosed in a begin-end group. It is also legal to have no statements at all, in
which case the function returns the current value of the implicit variable that has the same name as the
function.
#### 13.4.1 Return values and void functions
The function definition shall implicitly declare a variable, internal to the function, with the same name as the
function. This variable has the same type as the function return value. Function return values can be
specified in two ways, either by using a return statement or by assigning a value to the internal variable
with the same name as the function. For example:
function [15:0] myfunc1 (input [7:0] x,y);
myfunc1 = x * y - 1; // return value assigned to function name
endfunction
function [15:0] myfunc2 (input [7:0] x,y);
return x * y - 1; //return value is specified using return statement
endfunction
The return statement shall override any value assigned to the function name. When the return statement is
used, nonvoid functions shall specify an expression with the return.
A function return can be a structure or union. In this case, a hierarchical name used inside the function and
beginning with the function name is interpreted as a member of the return value. If the function name is used
outside the function, the name indicates the scope of the whole function. If the function name is used within
a hierarchical name, it also indicates the scope of the whole function.
Functions can be declared as type void, which do not have a return value. Function calls may be used as
expressions unless of type void, which are statements:
a = b + myfunc1(c, d); // call myfunc1 (defined above) as an expression
myprint(a); // call myprint (defined below) as a statement
function void myprint (int a);
...
endfunction
Functions that return a value may be used in an assignment or an expression. Calling a nonvoid function as
if it has no return value shall be legal, but shall issue a warning. The function can be used as a statement and
the return value discarded without a warning by casting the function call to the void type.
void'(some_function());
It shall be illegal to declare another object with the same name as the function in the scope where the
function is declared or explicitly imported. It shall also be illegal to declare another object with the same
name as the function inside the function scope.
To simplify usage of function return values, a nonvoid function call may be used as an implicit variable
within an expression. This implicit variable shall be of the return type of the function. For example:

<!-- Page 344 -->

IEEE Std class Node;
typedef bit [15:10] value_t;
protected Node m_next;
protected value_t m_val;
function new(value_t v); m_val = v; endfunction
function set_next(Node n); m_next = n; endfunction
function Node get_next() return m_next; endfunction
function value_t get_val(); return m_val; endfunction
endclass
function Node get_first_node();
Node n1, n2;
n1 = new(6'h00);
n2 = new(6'h3F);
n1.set_next(n2);
return n1;
endfunction
begin
bit [3:0] my_bits;
Node first_node, next_node;
Node::value_t next_value;
first_node = get_first_node();
next_node = first_node.get_next();
next_value = next_node.get_val();
my_bits = next_value[13:10];
end
In the example, the variables my_node, next_node, and next_value are only used to traverse the
hierarchy of the function return values. This can be simplified using the implicitly declared variables:
begin
bit [3:0] my_bits;
my_bits = get_first_node().get_next().get_val()[13:10];
end
Note that care needs to be taken when using implicitly declared variables with functions that may return
null, such as get_first_node() and get_next() in the example. If either method returned null, then
the result would be a dereference error. Also note that the return type of get_val() is of type
Node::value_t, i.e., bit[15:10], and that the implicit variable shall be indexed accordingly.
To avoid confusion with hierarchical references into function scopes, parentheses shall always be included
when using a function call as an implicit variable, even if no arguments are being supplied. For example:
class A;
int member=1;
endclass
module top;
A a;
function A F;
int member;

<!-- Page 345 -->

IEEE Std a = new();
return a;
endfunction
initial begin
$display(F.member); // "0". No "()", hierarchical reference,
// a is still null
$display(F().member); // "1". With "()", implicit variable,
// a is initialized
end
endmodule
Unless otherwise specified, all nonvoid function calls, including built-in methods (see 5.13), system
functions (see Clause20 and Clause21), and DPI imported functions (see 35.5). shall be allowed as implicit
variables within an expression.
#### 13.4.2 Static and automatic functions
Functions defined within a module, interface, program, or package default to being static, with all declared
items being statically allocated. These items shall be shared across all uses of the function executing
concurrently.
Functions can be defined to use automatic storage in the following two ways:
— Explicitly declared using the optional automatic keyword as part of the function declaration.
— Implicitly declared by defining the function within a module, interface, program, or package that is
defined as automatic.
Functions defined within a class are always automatic (see 8.6).
An automatic function is reentrant, with all the function declarations allocated dynamically for each
concurrent function call. Automatic function items cannot be accessed by hierarchical references. Automatic
functions can be invoked through the use of their hierarchical name.
Specific local variables can be declared as automatic within a static function or as static within an
automatic function.
The following example defines a function called factorial that returns an integer value. The factorial
function is called iteratively and the results are printed.
module tryfact;
// define the function
function automatic integer factorial (input [31:0] operand);
if (operand >= 2)
factorial = factorial (operand - 1) * operand;
else
factorial = 1;
endfunction: factorial
// test the function
integer result;
initial begin
for (int n = 0; n <= 7; n++) begin
result = factorial(n);
$display("%0d factorial=%0d", n, result);
end

<!-- Page 346 -->

IEEE Std end
endmodule: tryfact
The simulation results are as follows:
0 factorial=1
1 factorial=1
2 factorial=2
3 factorial=6
4 factorial=24
5 factorial=120
6 factorial=720
7 factorial=5040
#### 13.4.3 Constant functions
Constant functions are a subset of normal functions that shall meet the following constraints:
— The function shall not have output, inout, or ref arguments.
— The function shall not be a DPI import function (see 35.2.1).
— The function shall not contain a statement that directly schedules an event to execute after the func-
tion has returned.
— The function shall not contain any fork constructs.
— The function shall not contain any hierarchical references.
— The function shall not contain function invocations that are not constant function calls. An exception
to this constraint is that it shall be legal to call all built-in methods on variables local to the function,
including implicit variables as described in 13.4.1. This includes calls to built-in methods that would
not otherwise qualify as constant built-in method calls (see 11.2.1).
— The function shall only call system functions that are allowed in a constant_expression (see 11.2.1).
— The function shall not reference any identifiers that are not either parameter or function names, or
declared locally to the current function.
— The function shall not be declared inside a generate block (see Clause27).
— The function shall not call constant functions in any context requiring a constant expression.
— The function may have default argument values (see 13.5.3), but any such default argument value
shall be a constant expression.
A constant function call is a function call of a constant function that is evaluated at elaboration time,
wherein the constant function’s declaration is local to the calling designelement or is in a package or
$unit, and where the arguments to the function, if any, are all constant expressions. Calls to static functions
declared within classes (see 8.10), including parameterized functions (see 13.8), that are local to the calling
design element or in a package or $unit are also constant function calls when the arguments to the function
are all constant expressions.
During evaluation of a constant function call, the following is true:
— All value and type parameters used within the function shall be defined before the use of the
invoking constant function call (i.e., any parameter use in the evaluation of a constant function call
constitutes a use of that parameter at the site of the original constant function call). A constant
function may reference parameters defined in packages or $unit.
— If the function uses any parameter value that is affected directly or indirectly by a defparam
statement (see 23.10.1), the result shall be undefined. This can produce an error or the constant
function can return an indeterminate value.

<!-- Page 347 -->

IEEE Std — All system task calls within a constant function shall be ignored except for the elaboration severity
system tasks (see 20.10.1).
The execution of a constant function call has no effect on the initial values of the variables used either at
simulation time or among multiple invocations of a function at elaboration time. In each of these cases, the
variables are initialized as they would be for normal simulation.
The following example defines a function called clogb2 that returns an integer with the value of the ceiling
of the log base 2.
NOTE—In practice, the built-in system function $clog2 (see 20.8.1) would normally be used. A user-defined function
is written here in order to show an example of a user-defined constant function.
module ram_model (address, write, chip_select, data);
parameter data_width = 8;
parameter ram_depth = 256;
localparam addr_width = clogb2(ram_depth);
input [addr_width - 1:0] address;
input write, chip_select;
inout [data_width - 1:0] data;
//define the clogb2 function
function integer clogb2 (input [31:0] value);
value = value - 1;
for (clogb2 = 0; value > 0; clogb2 = clogb2 + 1)
value = value >> 1;
endfunction
logic [data_width - 1:0] data_store[0:ram_depth - 1];
//the rest of the ram model
endmodule: ram_model
An instance of this ram_model with parameters assigned is as follows:
ram_model #(32,421) ram_a0(a_addr,a_wr,a_cs,a_data);
#### 13.4.4 Background processes spawned by function calls
Functions shall execute with no delay. Thus, a process calling a function shall return immediately.
Statements that do not block shall be allowed inside a function; specifically, nonblocking assignments, event
triggers, clocking drives, and fork-join_none constructs shall be allowed inside a function.
Calling a function that tries to schedule an event that cannot become active until after that function returns
shall be allowed provided that the thread calling the function is created by an initial procedure, always
procedure, or fork block from one of those procedures and in a context in which a side effect is allowed.
Implementations shall issue an error either at compile time or run time when these provisions have not been
met.
Within a function, a fork-join_none construct may contain any statements that are legal within a task.
Examples of a legal and illegal usage of fork-join_none in a function are shown as follows:
class IntClass;
int a;
endclass
IntClass address=new(), stack=new();

<!-- Page 348 -->

IEEE Std function automatic bit watch_for_zero(IntClass p);
fork
forever @p.a begin
if (p.a == 0) $display ("Unexpected zero");
end
join_none
return (p.a == 0);
endfunction
function bit start_check();
return (watch_for_zero(address) | watch_for_zero(stack));
endfunction
bit y = watch_for_zero(stack); // illegal
initial if (start_check()) $display ("OK"); // legal
initial fork
if (start_check()) $display("OK too"); // legal
join_none
### 13.5 Subroutine calls and argument passing
Tasks and void functions are called as statements within procedural blocks (see 9.2). A nonvoid function call
may be an operand within an expression.
The syntax for calling a subroutine as a statement is shown in Syntax13-3:
subroutine_call_statement ::= // from A.6.9
subroutine_call ;
| void ' ( function_subroutine_call ) ;
subroutine_call ::= // from A.8.2
tf_call
| system_tf_call
| method_call
| [ std :: ] randomize_call
tf_call42 ::= ps_or_hierarchical_tf_identifier { attribute_instance } [ ( list_of_arguments ) ]
list_of_arguments ::=
[ expression ] { , [ expression ] } { , . identifier ( [ expression ] ) }
| . identifier ( [ expression ] ) { , . identifier ( [ expression ] ) }
ps_or_hierarchical_tf_identifier ::= // from A.9.3
[ package_scope ] tf_identifier
| hierarchical_tf_identifier
42) It shall be illegal to omit the parentheses in a tf_call unless the subroutine is a task, void function, or class method.
If the subroutine is a nonvoid class function method, it shall be illegal to omit the parentheses if the call is directly
recursive.
Syntax13-3—Task or function call syntax (excerpt from AnnexA)

<!-- Page 349 -->

IEEE Std If an argument in the subroutine is declared as an input, then the corresponding expression in the
subroutine call can be any expression. The order of evaluation of the expressions in the argument list is
undefined.
If the argument in the subroutine is declared as an output or an inout, then the corresponding expression
in the subroutine call shall be restricted to an expression that is valid on the left-hand side of a procedural
assignment (see 10.4).
The execution of the subroutine call shall pass input values from the expressions listed in the arguments of
the call. Execution of the return from the subroutine shall pass values from the output and inout type
arguments to the corresponding variables in the subroutine call.
SystemVerilog provides two means for passing arguments to tasks and functions: by value and by reference.
Arguments can also be bound by name as well as by position. Subroutine arguments can also be given
default values, allowing the call to the subroutine to not pass arguments.
#### 13.5.1 Pass by value
Pass by value is the default mechanism for passing arguments to subroutines. This argument passing
mechanism works by copying each argument into the subroutine area. If the subroutine is automatic, then
the subroutine retains a local copy of the arguments in its stack. If the arguments are changed within the
subroutine, the changes are not visible outside the subroutine. When the arguments are large, it can be
undesirable to copy the arguments. Also, programs sometimes need to share a common piece of data that is
not declared global.
For example, calling the following function copies 1000 bytes each time the call is made.
function automatic int crc( byte packet [1000:1] );
for( int j= 1; j <= 1000; j++ ) begin
crc ^= packet[j];
end
endfunction
#### 13.5.2 Pass by reference
Arguments passed by reference are not copied into the subroutine area, rather, a reference to the original
argument is passed to the subroutine. The subroutine can then access the argument data via the reference.
Arguments passed by reference shall be matched with equivalent data types (see 6.22.2). No casting shall be
permitted. To indicate argument passing by reference, the argument declaration is preceded by the ref
keyword. It shall be illegal to use argument passing by reference for subroutines with a lifetime of static.
The general syntax is as follows:
subroutine(ref type argument);
For example, the preceding example can be written as follows:
function automatic int crc(ref byte packet [1000:1]);
for(int j= 1; j <= 1000; j++) begin
crc ^= packet[j];
end
endfunction
As shown in the preceding example, no change other than addition of the ref keyword is needed. The
compiler knows that packet is now addressed via a reference, but users do not need to make these

<!-- Page 350 -->

IEEE Std references explicit either in the callee or at the point of the call. In other words, the call to either version of
the crc function remains the same:
byte packet1[1000:1];
int k = crc(packet1); // pass by value or by reference: call is the same
When the argument is passed by reference, both the caller and the subroutine share the same representation
of the argument; therefore, any changes made to the argument, within either the caller or the subroutine,
shall be visible to each other. The semantics of assignments to variables passed by reference is that changes
are seen outside the subroutine immediately (before the subroutine returns).
Only the following shall be legal to pass by reference:
— A variable,
— A class property,
— A member of an unpacked structure, or
— An element of an unpacked array.
Nets and selects into nets shall not be passed by reference.
Because a variable passed by reference may be an automatic variable, a ref argument shall not be used in
any context forbidden for automatic variables, unless it is ref static. A ref static argument shall only
be passed variables, properties, members or elements with static lifetime, or another ref static argument.
They shall not be passed automatic variables, elements of dynamically sized array variables, non-static class
properties, or ref arguments without the static qualifier.
Elements of dynamic arrays, queues, and associative arrays that are passed by reference may get removed
from the array or the array may get resized before the called subroutine completes. The specific array
element passed by reference shall continue to exist within the scope of the called subroutines until they
complete. Changes made to the values of array elements by the called subroutine shall not be visible outside
the scope of those subroutines if those array elements were removed from the array before the changes were
made. These references shall be called outdated references.
The following operations on a variable-size array shall cause existing references to elements of that array to
become outdated references:
— A dynamic array is resized with an implicit or explicit new[].
— A dynamic array is deleted with the delete() method.
— The element of an associative array being referenced is deleted with the delete() method.
— The queue or dynamic array containing the referenced element is updated by assignment.
— The element of a queue being referenced is deleted by a queue method.
Passing an argument by reference is a unique argument-passing qualifier, different from input, output, or
inout. Combining ref with any other directional qualifier shall be illegal. For example, the following
declaration results in a compiler error:
task automatic incr( ref input int a );// incorrect: ref cannot be qualified
A ref argument is similar to an inout argument except that an inout argument is copied twice: once from
the actual into the argument when the subroutine is called and once from the argument into the actual when
the subroutine returns. Passing object handles is no exception and has similar semantics when passed as ref
or inout arguments. Thus, a ref of an object handle allows changes to the object handle (for example,
assigning a new object) in addition to modification of the contents of the object.

<!-- Page 351 -->

IEEE Std To protect arguments passed by reference from being modified by a subroutine, the const qualifier can be
used together with ref to indicate that the argument, although passed by reference, is a read-only variable.
task automatic show ( const ref byte data [] );
for ( int j = 0; j < data.size ; j++ )
$display( data[j] ); // data can be read but not written
endtask
When the formal argument is declared as a const ref, the subroutine cannot alter the variable, and an
attempt to do so shall generate a compiler error.
#### 13.5.3 Default argument values
To handle common cases or allow for unused arguments, SystemVerilog allows a subroutine declaration to
specify a default value for each singular argument.
The syntax to declare a default argument value in a subroutine is as follows:
subroutine( [ direction ] [ type ] argument = default_expression);
The default_expression is evaluated in the scope containing the subroutine declaration each time a call using
the default is made. If the default is not used, the default expression is not evaluated. The use of defaults
shall only be allowed with the ANSI style declarations.
When the subroutine is called, arguments with defaults can be omitted from the call, and the compiler shall
insert their corresponding values. Unspecified (or empty) arguments can be used as placeholders for
arguments with default values. If an unspecified argument is used for an argument that does not have a
default, a compiler error shall be issued.
task read(int j = 0, int k, int data = 1 );
...
endtask
This example declares a task read() with default values for two arguments, j and data. The task can then
be called using various values for those two arguments:
read( , 5 ); // is equivalent to read( 0, 5, 1 );
read( 2, 5 ); // is equivalent to read( 2, 5, 1 );
read( , 5, ); // is equivalent to read( 0, 5, 1 );
read( , 5, 7 ); // is equivalent to read( 0, 5, 7 );
read( 1, 5, 2 ); // is equivalent to read( 1, 5, 2 );
read( ); // error; k has no default value
read( 1, , 7 ); // error; k has no default value
The following example shows an output argument with a default expression:
module m;
logic a, w;
task t1 (output o = a) ; // default binds to m.a
...
endtask :t1
task t2 (output o = b) ; // illegal, b cannot be resolved
...
endtask :t2

<!-- Page 352 -->

IEEE Std task t3 (inout io = w) ; // default binds to m.w
...
endtask :t1
endmodule :m
module n;
logic a;
initial begin
m.t1(); // same as m.t1(m.a), not m.t1(n.a);
// at end of task, value of t1.o is copied to m.a
m.t3(); // same as m.t3(m.w)
// value of m.w is copied to t3.io at start of task and
// value of t3.io is copied to m.w at end of task
end
endmodule :n
#### 13.5.4 Argument binding by name
SystemVerilog allows arguments to tasks and functions to be bound by name as well as by position. This
allows omitting arguments with default values without using empty placeholders. For example:
function int fun( int j = 1, string s = "no" );
...
endfunction
The fun function can be called as follows:
fun( .j(2), .s("yes") ); // fun( 2, "yes" );
fun( .s("yes") ); // fun( 1, "yes" );
fun( , "yes" ); // fun( 1, "yes" );
fun( .j(2) ); // fun( 2, "no" );
fun( .s("yes"), .j(2) ); // fun( 2, "yes" );
fun( .s(), .j() ); // fun( 1, "no" );
fun( 2 ); // fun( 2, "no" );
fun( ); // fun( 1, "no" );
If the arguments have default values, they are treated like parameters to module instances. If the arguments
do not have a default, then they shall be given, or the compiler shall issue an error.
If both positional and named arguments are specified in a single subroutine call, then all the positional
arguments shall come before the named arguments. Then, using the same example as above:
fun(.s("yes"), 2); // illegal
fun(2, .s("yes")); // OK
#### 13.5.5 Optional argument list
When a void function or class function method specifies no arguments, the empty parentheses, (), following
the subroutine name shall be optional. This is also true for tasks, void functions, and class methods that
require arguments, when all arguments have defaults specified. It shall be illegal to omit the parentheses in a
directly recursive nonvoid class function method call that is not hierarchically qualified.

<!-- Page 353 -->

IEEE Std ### 13.6 Import and export functions
SystemVerilog provides a direct programming interface (DPI) that allows importing foreign language
subroutines, such as C functions, into SystemVerilog. An imported subroutine is called in the same way as a
SystemVerilog subroutine. SystemVerilog tasks and functions can also be exported to a foreign language.
See Clause35 for details on the DPI.
### 13.7 Task and function names
Task and function names are resolved following slightly different rules than other references. Even when
used as a simple name, a task or function name follows a modified form of the upwards hierarchical name
resolution rules. This means that “forward” references to a task or function defined later in the same or an
enclosing scope can be resolved. See 23.8.1 for the rules that govern task and function name resolution.
### 13.8 Parameterized tasks and functions
SystemVerilog provides a way to create parameterized tasks and functions, also known as parameterized
subroutines. A parameterized subroutine allows the user to generically specify or define an implementation.
When using that subroutine one may provide the parameters that fully define its behavior. This allows for
only one definition to be written and maintained instead of multiple subroutines with different array sizes,
data types, and variable widths.
The way to implement parameterized subroutines is through the use of static methods in parameterized
classes (see 8.10 and 8.25). The following generic encoder and decoder example shows how to use static
class methods along with class parameterization to implement parameterized subroutines. The example has
one class with two subroutines that, in this case, share parameterization. The class may be declared virtual in
order to prevent object construction and enforce the strict static usage of the method.
virtual class C#(parameter DECODE_W, parameter ENCODE_W = $clog2(DECODE_W));
static function logic [ENCODE_W-1:0] ENCODER_f
(input logic [DECODE_W-1:0] DecodeIn);
ENCODER_f = '0;
for (int i=0; i<DECODE_W; i++) begin
if(DecodeIn[i]) begin
ENCODER_f = i[ENCODE_W-1:0];
break;
end
end
endfunction
static function logic [DECODE_W-1:0] DECODER_f
(input logic [ENCODE_W-1:0] EncodeIn);
DECODER_f = '0;
DECODER_f[EncodeIn] = 1'b1;
endfunction
endclass
Class C contains two static subroutines, ENCODER_f and DECODER_f. Each subroutine is parameterized by
reusing the class parameters DECODE_W and ENCODE_W. The default value of parameter ENCODE_W is
determined by using the system function $clog2 (see 20.8.1). These parameters are used within the
subroutines to specify the size of the encoder and the size of the decoder.
module top ();
logic [7:0] encoder_in;

<!-- Page 354 -->

IEEE Std logic [2:0] encoder_out;
logic [1:0] decoder_in;
logic [3:0] decoder_out;
// Encoder and Decoder Input Assignments
assign encoder_in = 8'b0100_0000;
assign decoder_in = 2'b11;
// Encoder and Decoder Function calls
assign encoder_out = C#(8)::ENCODER_f(encoder_in);
assign decoder_out = C#(4)::DECODER_f(decoder_in);
initial begin
#50;
$display("Encoder input = %b Encoder output = %b\n",
encoder_in, encoder_out );
$display("Decoder input = %b Decoder output = %b\n",
decoder_in, decoder_out );
end
endmodule
The top level module first defines some intermediate variables used in this example, and then assigns
constant values to the encoder and decoder inputs. The subroutine call of the generic encoder, ENCODER_f,
uses the specialized class parameter value of 8 that represents the decoder width value for that specific
instance of the encoder while at the same time passing the input encoded value, encoder_in. This
expression uses the static class scope resolution operator :: (see 8.23) to access the encoder subroutine. The
expression is assigned to an output variable to hold the result of the operation. The subroutine call for the
generic decoder, DECODER_f, is similar, using the parameter value of 4.

<!-- Page 355 -->

1800-2023