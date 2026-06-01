# Chapter 24: Programs

## 24. Programs
### 24.1 General
This clause describes the following:
— Program declarations
— Program scheduling semantics
— Programs in conjunction with clocking blocks
— Anonymous programs
### 24.2 Overview
The module is the basic building block for designs. Modules can contain hierarchies of other modules, nets,
variables, subroutine declarations, and procedural statements within always and initial procedures. This
construct works extremely well for the description of hardware. However, for the testbench, the emphasis is
not in the hardware-level details such as wires, structural hierarchy, and interconnects, but in modeling the
complete environment in which a design is verified. The environment needs to be properly initialized and
synchronized, avoiding races between the design and the testbench, automating the generation of input
stimuli, and reusing existing models and other infrastructure.
The program block serves the following three basic purposes:
— It provides an entry point to the execution of testbenches.
— It creates a scope that encapsulates program-wide data, tasks, and functions.
— It provides a syntactic context that specifies scheduling in the reactive region set.
The program construct serves as a clear separator between design and testbench, and, more importantly, it
specifies specialized execution semantics in the reactive region set for all elements declared within the
program. Together with clocking blocks, the program construct provides for race-free interaction between
the design and the testbench and enables cycle- and transaction-level abstractions.
The abstraction and modeling constructs of SystemVerilog simplify the creation and maintenance of
testbenches. The ability to instantiate and individually connect each program instance enables their use as
generalized models.
### 24.3 The program construct
A typical program contains type and data declarations, subroutines, connections to the design, and one or
more procedural code streams. The connection between design and testbench uses the same interconnect
mechanism used to specify port connections, including interfaces. Program port declaration syntax and
semantics are the same as those of modules (see 23.2.2).
The syntax for the program block is as follows:
program_nonansi_header ::= // from A.1.2
{ attribute_instance } program [ lifetime ] program_identifier
{ package_import_declaration } [ parameter_port_list ] list_of_ports ;
program_ansi_header ::=
{ attribute_instance } program [ lifetime ] program_identifier
{ package_import_declaration }1 [ parameter_port_list ] [ list_of_port_declarations ] ;

<!-- Page 776 -->

IEEE Std program_declaration ::=
program_nonansi_header [ timeunits_declaration ] { program_item }
endprogram [ : program_identifier ]
| program_ansi_header [ timeunits_declaration ] { non_port_program_item }
endprogram [ : program_identifier ]
| { attribute_instance } program program_identifier ( . * ) ;
[ timeunits_declaration ] { program_item }
endprogram [ : program_identifier ]
| extern program_nonansi_header
| extern program_ansi_header
program_item ::= // from A.1.7
port_declaration ;
| non_port_program_item
non_port_program_item ::=
{ attribute_instance } continuous_assign
| { attribute_instance } module_or_generate_item_declaration
| { attribute_instance } initial_construct
| { attribute_instance } final_construct
| { attribute_instance } concurrent_assertion_item
| timeunits_declaration3
| program_generate_item
program_generate_item5 ::=
loop_generate_construct
| conditional_generate_construct
| generate_region
| elaboration_severity_system_task
lifetime ::= static | automatic // from A.2.1.3
anonymous_program ::= program ; { anonymous_program_item } endprogram // from A.1.11
anonymous_program_item ::=
task_declaration
| function_declaration
| class_declaration
| interface_class_declaration
| covergroup_declaration
| class_constructor_declaration
| ;
1) A package_import_declaration in a module_ansi_header, interface_ansi_header, or program_ansi_header shall
be followed by a parameter_port_list or list_of_port_declarations, or both.
3) A timeunits_declaration shall be legal as a non_port_module_item, non_port_interface_item,
non_port_program_item, or package_item only if it repeats and matches a previous timeunits_declaration within
the same time scope.
5) It shall be illegal for a program_generate_item to include any item that would be illegal in a program_declaration
outside a program_generate_item.
Syntax24-1—Program declaration syntax (excerpt from AnnexA)
For example:
program test (input clk, input [16:1] addr, inout [7:0] data);
initial ...

<!-- Page 777 -->

IEEE Std endprogram
or
program test ( interface device_ifc );
initial ...
endprogram
A more complete example is included in 14.8 and 14.9.
The program construct can be considered a leaf module with special execution semantics. Once declared, a
program block can be instantiated in the required hierarchical location (typically at the top level), and its
ports can be connected in the same manner as any other module.
Program blocks can be nested within modules or interfaces. This allows multiple cooperating programs to
share variables local to the scope. Nested programs with no ports or top-level programs that are not
explicitly instantiated are implicitly instantiated once. Implicitly instantiated programs have the same
instance and declaration name. For example:
module test(...);
int shared; // variable shared by programs p1 and p1
program p1;
...
endprogram
program p2;
...
endprogram // p1 and p2 are implicitly instantiated once in module test
endmodule
A program block may contain data declarations, class definitions, subroutine definitions, object instances,
and one or more initial or final procedures. It shall not contain always procedures, primitives, UDPs, or
declarations or instances of modules, interfaces, or other programs.
When all initial procedures within a program have reached their end, that program shall immediately
terminate all descendant threads of initial procedures within that program. If there is at least one initial
procedure within at least one program block, the entire simulation shall terminate by means of an implicit
call to the $finish system task immediately after all the threads and all their descendant threads originating
from all initial procedures within all programs have ended.
Type and data declarations within the program are local to the program scope and have static lifetime.
Variables declared within the scope of a program, including variables declared as ports, are called program
variables. Similarly, nets declared within the scope of a program are called program nets. Program variables
and nets are collectively termed program signals.
The dual of a program signal is a design signal. Any net or variable declared within a module, interface,
package, or $unit is considered to be a design signal.
References to program signals from outside any program block shall be an error. It shall be legal for
hierarchical references to extend from one program scope to another program scope. However, anonymous
programs shall not contain hierarchical references to other program scopes.

<!-- Page 778 -->

IEEE Std #### 24.3.1 Scheduling semantics of code in program constructs
Statements and constructs within a program block that are sensitive to changes (e.g., update events) on
design signals are scheduled in the Reactive region. Consider a program that contains the statement
@(clk) S1; where clk is a design signal. Every transition of signal clk will cause the statement S1 to be
scheduled into the Reactive region. The continuous assignment assign tclk = clk; would also be
scheduled in the Reactive region. Likewise, initial procedures within program blocks are scheduled in the
Reactive region. The standard # delay operator within program blocks schedules process resumption in the
Reactive region.
Nonblocking assignments in program code schedule their updates in the Re-NBA region. The Re-NBA
region is processed after the Reactive and Re-Inactive regions have been emptied of events. See 4.2.
Concurrent assertions are allowed in program blocks. Concurrent assertions have invariant scheduling
semantics—whether present in program code or design code. Assertions always sample the values available
while processing the Preponed region, and they are always evaluated when processing the Observed region.
If an assertion is clocked by activity on a program object (not recommended), the scheduler will iterate from
the reactive region set back around the outer loop in Figure4-1, through the Observed region, where the
assertion is evaluated.
Once a program process starts a thread of execution, all subsequent blocking statements in that thread are
scheduled in the Reactive region. This includes subroutine code called by the thread, even if the subroutine
code is declared in a module, package, or interface. Effectively, a section of sequential code anywhere in the
design or testbench inherits the scheduling region of the thread that calls it. Since program code can never be
called by module code, program code always executes as part of the reactive region set processing. Code in
a module, interface, or package scope may execute as part of either the active region set or the reactive
region set processing.
#### 24.3.2 Operation of program port connections in the absence of clocking blocks
The interaction of clocking blocks with program ports is described in Clause14. Clocking blocks are an
important component in establishing race-free behavior between designs and testbenches. However, it is
possible to construct a program that contains no clocking blocks. Such programs are more prone to races
when interacting with design code. This subclause defines the interaction of program ports with design code
in the absence of clocking blocks.
Program ports are program-scope objects. They are always connected to design objects (nets and variables),
since programs can only be instantiated in design scopes.
Sequential code declared in programs always executes in the reactive region set. Thus, variables on the other
side of a program port connection are updated in the reactive region set. Similarly, the driving and resolution
of nets on the other side of a program port connection also occurs in the reactive region set. Such driving and
resolution occurs immediately after an event causes a change to a driver on a program net. Design processes
sensitive to those cross-region variables and nets are scheduled for wake up in the active region set.
Consider the following example design, which contains both design constructs and program constructs:
module m;
logic r;
wire dw1, dw2;
initial begin
r = 0;
#10 r = 1;

<!-- Page 779 -->

IEEE Std end
assign dw1 = r;
p p_i(dw2, dw1);
always @(dw2)
$display("dw2 is %b", dw2);
endmodule
program p(output pw2, input pw1);
assign pw2 = pw1;
endprogram
In this design, the flow of data originates in logic r and terminates in the execution of the always
procedure. Due to the presence of program p, it is necessary for simulators to perform multiple iterations
over the entire loop in Figure4-1. This is because the assign statement in program p shall not be
executed until the Reactive region. And when it executes and triggers activity on the always procedure in
module m, that always procedure is not executed until the Active region in the next iteration of the overall
scheduling loop.
### 24.4 Eliminating testbench races
There are two major sources of nondeterminism in SystemVerilog. The first one is that active events are
processed in an arbitrary order. The second one is that statements without time control constructs in
behavioral blocks do not execute as one event. However, from the testbench perspective, these effects are all
unimportant details. The primary task of a testbench is to generate valid input stimulus for the design under
test and to verify that the device operates correctly. Furthermore, testbenches that use cycle abstractions are
only concerned with the stable or steady state of the system for both checking the current outputs and for
computing stimuli for the next cycle. Formal tools also work in this fashion.
Because the program schedules events in the reactive region set, the clocking block construct is very useful
to automatically sample the steady-state values of previous time steps or clock cycles. Programs that read
design values exclusively through clocking blocks with clocks that are design signals are insensitive to read-
write races. It is important to understand that simply sampling input signals (or setting nonzero skews on
clocking block inputs) does not eliminate the potential for races. Proper input sampling only addresses a
single clocking block. With multiple clocks, the arbitrary order in which overlapping or simultaneous clocks
are processed is still a potential source for races. The program construct addresses this issue by scheduling
its execution in the Reactive region, after all design events have been processed, including clocks driven by
nonblocking assignments.
### 24.5 Blocking tasks in cycle/event mode
Calling program subroutines from within design modules is illegal and shall result in an error. This is
because the design should not be aware of the testbench. Programs are allowed to call subroutines in other
programs or within design modules. Functions within design modules can be called from a program and
require no special handling. When a task within a design module is called from a program, it shall use the
reactive region set for its scheduling activities. See 24.3.1.
module ...
task T;
S1: a = b; // executes in reactive region set if called from a program
#5;
S2: b <= 1'b1; // executes in reactive region set if called from a program

<!-- Page 780 -->

IEEE Std endtask
endmodule
If task T, above, is called from within a module, then the statement S1 can execute immediately when the
Active region is processed, before variable b is updated by the nonblocking assignment. If the same task is
called from within a program, then the statement S1 shall execute when the Reactive region is processed.
Statement S2 shall also execute in the Reactive region, and variable b’s update shall be scheduled in the
Re-NBA region.
### 24.6 Program-wide space and anonymous programs
The set of program definitions and instances define a space of program-wide data, tasks, and functions that
is accessible only to programs.
Anonymous programs can be used inside packages (see Clause26) or compilation-unit scopes (see 3.12.1)
to declare items that are part of the program-wide space without declaring a new scope. Items declared in an
anonymous program share the same name space as the package or compilation-unit scope in which they are
declared.
NOTE—Although identifiers declared inside an anonymous program cannot be referenced outside any program block,
attempting to declare another identifier with the same name outside the anonymous program block will generate an error.
This occurs because the identifier shares the same name space within the scope of the surrounding package or
compilation unit.
### 24.7 Program control tasks
In addition to the normal simulation control tasks ($stop and $finish), a program can use the $exit
control task.
A program block may terminate the threads of all its initial procedures as well as all of their descendants
explicitly by calling the $exit system task. The syntax for the $exit system task is as follows:
$exit();
Calling $exit from a thread or its descendant thread originating in an initial procedure of a program
block shall terminate all initial procedures and their descendant threads within that originating program
block. Calling $exit from a thread or its descendant thread that does not originate in an initial procedure
in a program shall be ignored, and a warning may be issued to indicate that the call to $exit has been
ignored.

<!-- Page 781 -->

IEEE Std 1800/-2023