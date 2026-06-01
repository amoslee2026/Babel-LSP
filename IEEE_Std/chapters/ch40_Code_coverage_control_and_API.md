# Chapter 40: Code coverage control and API

## 40. Code coverage control and API
### 40.1 General
This clause describes the following:
— SystemVerilog coverage API
— Coverage constants
— Coverage VPI routines
— FSM recognition
— Coverage VPI extensions
### 40.2 Overview
This clause defines the coverage API in SystemVerilog.
#### 40.2.1 SystemVerilog coverage API
The following criteria are used within this API:
a) This API shall be similar for all coverages. A wide number of coverage types are available, with
possibly different sets offered by different vendors. Maintaining a common interface across all the
different types enhances portability and ease of use.
b) At a minimum, the following types of coverage shall be supported:
1) Statement coverage
2) Toggle coverage
3) Finite state machine (FSM) coverage
i) FSM states
ii) FSM transitions
3) Assertion coverage
c) Coverage APIs shall be extensible in a transparent manner, i.e., adding a new coverage type shall
not break any existing coverage usage.
d) This API shall provide means to obtain coverage information from specific subhierarchies of the
design without requiring the user to enumerate all instances in those hierarchies.
#### 40.2.2 Nomenclature
The following terms are used in this standard:
— assertion coverage: For each assertion, whether it has had at least one success. Implementations
permit querying for further details, such as attempt counts, success counts, failure counts, and failure
coverage.
— finite state machine (FSM) coverage: The number of states in an FSM that this simulation reached.
This standard does not require FSM automatic extraction, but a standard mechanism to force
specific extraction is available via pragmas.
— statement coverage: Whether a statement has been executed. Statement is anything defined as a
statement in the LRM. Covered means it executed at least once. Some implementations also permit
querying the execution count. The granularity of statement coverage can be per-statement or
per-statement block depending on the query (see 40.5.2 for details).

<!-- Page 1158 -->

IEEE Std — toggle coverage: For each bit of every signal (wire and register), whether that bit has both a 0 value
and a 1 value. Full coverage means both are seen; otherwise, some implementations can query for
partial coverage. Some implementations also permit querying the toggle count of each bit.
These terms define the primitives for each coverage type. Over instances or blocks, the coverage number is
merely the sum of all contained primitives in that instance or block.
### 40.3 SystemVerilog real-time coverage access
This subclause describes the mechanisms in SystemVerilog through which SystemVerilog code can query
and control coverage information. Coverage information is provided to SystemVerilog by means of a
number of built-in system functions (described in 40.3.2) using a number of predefined constants (described
in 40.3.1) to describe the types of coverage and the control actions to be performed.
#### 40.3.1 Predefined coverage constants in SystemVerilog
The following predefined `define macros represent basic real-time coverage capabilities accessible
directly from SystemVerilog:
— Coverage control
`define SV_COV_START 0
`define SV_COV_STOP 1
`define SV_COV_RESET 2
`define SV_COV_CHECK 3
— Scope definition (hierarchy traversal/accumulation type)
`define SV_COV_MODULE 10
`define SV_COV_HIER 11
— Coverage type identification
`define SV_COV_ASSERTION 20
`define SV_COV_FSM_STATE 21
`define SV_COV_STATEMENT 22
`define SV_COV_TOGGLE 23
— Status results
`define SV_COV_OVERFLOW -2
`define SV_COV_ERROR -1
`define SV_COV_NOCOV 0
`define SV_COV_OK 1
`define SV_COV_PARTIAL 2
#### 40.3.2 Built-in coverage access system functions
40.3.2.1 $coverage_control
$coverage_control(control_constant,
coverage_type,
scope_def,
modules_or_instance)

<!-- Page 1159 -->

IEEE Std This function is used to control or query coverage availability in the specified portion of the hierarchy. The
following control options are available:
— `SV_COV_START, if possible, starts collecting coverage information in the specified hierarchy.
There is no effect if coverage is already being collected. Coverage is automatically started at the
beginning of simulation for all portions of the hierarchy enabled for coverage.
— `SV_COV_STOP stops collecting coverage information in the specified hierarchy. There is no effect
if coverage is not being collected.
— `SV_COV_RESET resets all available coverage information in the specified hierarchy. There is no
effect if coverage not available.
— `SV_COV_CHECK checks whether coverage information can be obtained from the specified
hierarchy. The existence of coverage information does not imply that coverage is being collected, as
the coverage could have been stopped.
The return value is a `define name, with the value indicating the success of the action.
— `SV_COV_OK, on a check operation, denotes that coverage is fully available in the specified
hierarchy. For all other operations, it represents successful and complete execution of the desired
operation.
— `SV_COV_ERROR, on all operations, means that the control operation failed without any effect,
typically due to errors in arguments, such as a nonexisting module.
— `SV_COV_NOCOV, on a check or start operation, denotes that coverage is not available at any point in
the specified hierarchy.
— `SV_COV_PARTIAL, on a check or start operation, denotes that coverage is only partially available
in the specified hierarchy.
Table40-1 describes the possible return values for each of the coverage control options.
Table40-1—Coverage control return values
`SV_COV_OK `SV_COV_ERROR `SV_COV_NOCOV `SV_COV_PARTIAL
`SV_COV_START Success Bad args No coverage Partial coverage
`SV_COV_STOP Success Bad args — —
`SV_COV_RESET Success Bad args — —
`SV_COV_CHECK Full coverage Bad args No coverage Partial coverage
Starting coverage on an instance that has already had coverage started via a prior call to
$coverage_control() shall have no effect. Similarly, repeated calls to stop or reset coverage shall have
no effect.
The hierarchy(ies) being controlled or queried are specified as follows:
— `SV_MODULE_COV, "unique module def name" provides coverage of all instances of the
given module (the unique module name is a string), excluding any child instances in the instances of
the given module. The module definition name can use special notation to describe nested module
definitions.
— `SV_COV_HIER, "module name" provides coverage of all instances of the given module,
including all the hierarchy below.

<!-- Page 1160 -->

IEEE Std — `SV_MODULE_COV, instance_name provides coverage of the one named instance. The instance
is specified as a normal SystemVerilog hierarchical path.
— `SV_COV_HIER, instance_name provides coverage of the named instance, plus all the hierarchy
below it.
All the permutations are summarized in Table40-2.
Table40-2—Instance coverage permutations
Control/query Definition name instance.name
`SV_COV_MODULE The sum of coverage for all Coverage for just the named
instances of the named module, instance, excluding any
excluding any hierarchy below hierarchy in instances below that
those instances. instance.
`SV_COV_HIER The sum of coverage for all Coverage for the named instance
instances of the named module, and any hierarchy below it.
including all coverage for all
hierarchy below those instances.
NOTE—Definition names are represented as strings, whereas instance names are referenced by
hierarchical paths. A hierarchical path need not include any . if the path refers to an instance in the
current context (i.e., normal SystemVerilog hierarchical path rules apply).
See Figure40-1 for an example of hierarchical instances.

<!-- Page 1161 -->

IEEE Std $root
module TestBench
instance tb
module DUT
instance unit1
module component
instance comp
module control
instance ctrl
module DUT
instance unit2
module component
instance comp
module control
instance ctrl
module BusWatcher
instance watch
Figure40-1—Hierarchical instance example
If coverage is enabled on all instances shown in Figure40-1, then
— $coverage_control(`SV_COV_CHECK, `SV_COV_TOGGLE, `SV_COV_HIER, $root)
checks all instances to verify they have coverage and, in this case, returns `SV_COV_OK.
— $coverage_control(`SV_COV_RESET, `SV_COV_TOGGLE, `SV_COV_MODULE, "DUT")
resets coverage collection on both instances of the DUT, specifically, $root.tb.unit1 and
$root.tb.unit2, but leaves coverage unaffected in all other instances.
— $coverage_control(`SV_COV_RESET, `SV_COV_TOGGLE, `SV_COV_MODULE,
$root.tb.unit1)
resets coverage of only the instance $root.tb.unit1, leaving all other instances unaffected.
— $coverage_control(`SV_COV_STOP, `SV_COV_TOGGLE, `SV_COV_HIER,
$root.tb. unit1)
resets coverage of the instance $root.tb.unit1 and also resets coverage for all instances below it,
specifically $root.tb.unit1.comp and $root.tb.unit1.ctrl.
— $coverage_control(`SV_COV_START, `SV_COV_TOGGLE, `SV_COV_HIER, "DUT")
starts coverage on all instances of the module DUT and of all hierarchy(ies) below those instances.
In thisdesign, coverage is started for the instances $root.tb.unit1, $root.tb.unit1.comp,
$root.tb.unit1.ctrl, $root.tb.unit2, $root.tb.unit2.comp,
and $root.tb.unit2.ctrl.

<!-- Page 1162 -->

IEEE Std 40.3.2.2 $coverage_get_max
$coverage_get_max(coverage_type, scope_def, modules_or_instance)
This function obtains the value representing 100% coverage for the specified coverage type over the
specified portion of the hierarchy. This value shall remain constant across the duration of the simulation.
NOTE—This value is proportional to the design size and structure; therefore, it also needs to be constant through
multiple independent simulations and compilations of the same design, assuming any compilation options do not modify
the coverage support or design structure.
The return value is an integer, with the following meanings:
— -2 (`SV_COV_OVERFLOW). The value exceeds a number that can be represented as an integer.
— -1 (`SV_COV_ERROR). An error occurred (typically due to using incorrect arguments).
— 0 (`SV_COV_NOCOV). No coverage is available for that coverage type on that/those
hierarchy(ies).
— +pos_num. This value is the maximum coverage number (where pos_num > 0), which is the
sum of all coverable items of that type over the given hierarchy(ies).
The scope of this function is specified per $coverage_control() (see 40.3.2.1).
40.3.2.3 $coverage_get
$coverage_get(coverage_type, scope_def, modules_or_instance)
This function obtains the current coverage value for the given coverage type over the given portion of the
hierarchy. This number can be converted to a coverage percentage by use of the following equation:
coverage_get()
×
coverage% = 100
coverage_get_max()
The return value follows the same pattern as $coverage_get_max() (see 40.3.2.2), but with pos_num
representing the current coverage level, i.e., the number of the coverable items that have been covered in
this/these hierarchy(ies).
The scope of this function is specified per $coverage_control() (see 40.3.2.1).
The return value is an integer, with the following meanings:
— -2 (`SV_COV_OVERFLOW). The value exceeds a number that can be represented as an integer.
— -1 (`SV_COV_ERROR). An error occurred (typically due to using incorrect arguments).
— 0 (`SV_COV_NOCOV).No coverage is available for that coverage type on that/those
hierarchy(ies).
— +pos_num. This value is the maximum coverage number (where pos_num > 0), which is the
sum of all coverable items of that type over the given hierarchy(ies).
40.3.2.4 $coverage_merge
$coverage_merge(coverage_type, "name")
This function loads and merges coverage data for the specified coverage into the simulator. name is an
arbitrary string used by the tool, in an implementation-specific way, to locate the appropriate coverage
database, i.e., tools are allowed to store coverage files any place they want with any extension they want as

<!-- Page 1163 -->

IEEE Std long as the user can retrieve the information by asking for a specific saved name from that coverage
database. If name does not exist or does not correspond to a coverage database from the same design, an
error shall occur. If an error occurs during loading, the coverage numbers generated by this simulation might
not be meaningful.
The return values from this function are as follows:
— `SV_COV_OK. The coverage data have been found and merged.
— `SV_COV_NOCOV. The coverage data have been found, but did not contain the coverage type
requested.
— `SV_COV_ERROR. The coverage data were not found, or they did not correspond to this design, or
another error occurred.
40.3.2.5 $coverage_save
$coverage_save(coverage_type, "name")
This function saves the current state of coverage to the tool’s coverage database and associates it with the
given name. This name will be mapped in an implementation-specific way into some file or set of files in the
coverage database. Data saved to the database shall be retrieved later by using $coverage_merge() and
supplying the same name. Saving coverage shall not have any effect on the state of coverage in this
simulation.
The return values from this function are as follows:
— `SV_COV_OK. The coverage data were successfully saved.
— `SV_COV_NOCOV. No such coverage is available in this design (nothing was saved).
— `SV_COV_ERROR. Some error occurred during the save. If an error occurs, the tool shall
automatically remove the coverage database entry for name to preserve the coverage database
integrity. It is not an error to overwrite a previously existing name.
Details:
1) The coverage database format is implementation dependent.
2) Mapping of names to actual directories or files is implementation dependent. There is no
requirement that a coverage name map to any specific set of files or directories.
### 40.4 FSM recognition
Coverage tools need to have automatic recognition of many of the common FSM coding idioms in
SystemVerilog. This standard does not attempt to describe or require any specific automatic FSM
recognition mechanisms. However, this standard does prescribe a means by which nonautomatic FSM
extraction occurs. The presence of any of these standard FSM description additions shall override the tool’s
default extraction mechanism.
Identification of an FSM consists of identifying the following items:
— The state register (or expression)
— The next state register (this is optional)
— The legal states
#### 40.4.1 Specifying signal that holds current state
Use the following pragma to identify the vector signal that holds the current state of the FSM:

<!-- Page 1164 -->

IEEE Std /* tool state_vector signal_name */
where tool and state_vector are required keywords. This pragma needs to be specified inside the module
definition where the signal is declared.
Another pragma is also required that specifies an enumeration name for the FSM. This enumeration name is
also specified for the next state and any possible states, associating them with each other as part of the same
FSM. There are two ways to do this, as follows:
— Use the same pragma as above:
/* tool state_vector signal_name enum enumeration_name */
— Use a separate pragma in the signal’s declaration:
/* tool state_vector signal_name */
logic [7:0] /* tool enum enumeration_name */ signal_name;
In either case, enum is a required keyword. If using a separate pragma, tool is also a required keyword, and
the pragma needs to be specified immediately after the bit range of the signal.
#### 40.4.2 Specifying part-select that holds current state
A part-select of a vector signal can be used to hold the current state of the FSM. When a coverage tool
displays or reports FSM coverage data, it names the FSM after the signal that holds the current state. If a
part-select holds the current state in the user’s FSM, the user needs to also specify a name for the FSM for
the coverage tool to use. The FSM name is not the same as the enumeration name.
Specify the part-select by using the following pragma:
/* tool state_vector signal_name[n:n] FSM_name enum enumeration_name */
#### 40.4.3 Specifying concatenation that holds current state
Like specifying a part-select, a concatenation of signals can be specified to hold the current state (when
including an FSM name and an enumeration name):
/* tool state_vector {signal_name , signal_name, ...} FSM_name enum
enumeration_name */
The concatenation is composed of all the signals specified. Bit-selects or part-selects of signals cannot be
used in the concatenation.
#### 40.4.4 Specifying signal that holds next state
The signal that holds the next state of the FSM can also be specified with the pragma that specifies the
enumeration name:
logic [7:0] /* tool enum enumeration_name */ signal_name
This pragma can be omitted if, and only if, the FSM does not have a signal for the next state.

<!-- Page 1165 -->

IEEE Std #### 40.4.5 Specifying current and next state signals in same declaration
The tool assumes the first signal following the pragma holds the current state and the next signal holds the
next state when a pragma is used for specifying the enumeration name in a declaration of multiple signals.
For example:
/* tool state_vector cs */
logic [1:0] /* tool enum myFSM */ cs, ns, nonstate;
In this example, the tool assumes signal cs holds the current state and signal ns holds the next state. It
assumes nothing about signal nonstate.
#### 40.4.6 Specifying possible states of FSM
The possible states of the FSM can also be specified with a pragma that includes the following enumeration
name:
parameter /* tool enum enumeration_name */
S0 = 0,
s1 = 1,
s2 = 2,
s3 = 3;
Put this pragma immediately after the keyword parameter, unless a bit width for the parameters is used, in
which case, specify the pragma immediately after the bit width:
parameter [1:0] /* tool enum enumeration_name */
S0 = 0,
s1 = 1,
s2 = 2,
s3 = 3;
#### 40.4.7 Pragmas in one-line comments
These pragmas work in both block comments, between the /* and */ character strings, and one-line
comments, following the // character string. For example:
parameter [1:0] // tool enum enumeration_name
S0 = 0,
s1 = 1,
s2 = 2,
s3 = 3;
#### 40.4.8 Example
See Figure40-2 for an example of FSM specified with pragmas.

<!-- Page 1166 -->

IEEE Std module m3;
Signal ns holds the next state
reg[31:0] cs;
reg[31:0] /* tool enum MY_FSM */ ns;
reg[31:0] clk;
reg[31:0] rst; Signal cs holds the current state
// tool state_vector cs enum MY_FSM
parameter // tool enum MY_FSM
p1=10,
p2=11,
p3=12; p1, p2, and p3 are possible states of
the FSM
endmodule // m3
Figure40-2—FSM specified with pragmas
### 40.5 VPI coverage extensions
#### 40.5.1 Extensions to VPI enumerations
— Coverage control
vpiCoverageStart
vpiCoverageStop
vpiCoverageReset
vpiCoverageCheck
vpiCoverageMerge
vpiCoverageSave
— VPI properties
• Coverage type properties
vpiAssertCoverage
vpiFsmStateCoverage
vpiStatementCoverage
vpiToggleCoverage
• Coverage status properties
vpiCovered
vpiCoveredMax
vpiCoveredCount
• Assertion-specific coverage status properties
vpiAssertAttemptCovered
vpiAssertSuccessCovered
vpiAssertFailureCovered
vpiAssertVacuousSuccessCovered
vpiAssertDisableCovered
vpiAssertKillCovered
• FSM-specific methods
vpiFsmStates
vpiFsmStateExpression
— FSM handle types (vpi types)
vpiFsm

<!-- Page 1167 -->

IEEE Std vpiFsmHandle
#### 40.5.2 Obtaining coverage information
To obtain coverage information, the vpi_get() function is extended with additional VPI properties that can
be obtained from the following existing handles:
vpi_get(<coverageType>, instance_handle)
returns the number of covered items of the given coverage type in the given instance. Coverage type is one
of the coverage type properties described in 40.5.1. For example, given coverage type
vpiStatementCoverage, this call would return the number of covered statements in the instance pointed
to by instance_handle.
vpi_get(vpiCovered, assertion_handle)
vpi_get(vpiCovered, statement_handle)
vpi_get(vpiCovered, signal_handle)
vpi_get(vpiCovered, fsm_handle)
vpi_get(vpiCovered, fsm_state_handle)
returns whether the item referenced by the handle has been covered. For handles that can contain multiple
coverable entities, such as statement, FSM, and signal handles, the return value indicates how many of the
entities have been covered.
— For assertion handle, the coverable entities are assertions.
— For statement handle, the entities are statements.
— For signal handle, the entities are individual signal bits.
— For FSM handle, the entities are FSM states.
For assertions, vpiCovered implies that the assertion has been attempted, has succeeded at least once, and
has never failed. More detailed coverage information can be obtained for assertions by the following
queries:
vpi_get(vpiAssertAttemptCovered, assertion_handle)
returns the number of times the assertion has been attempted.
vpi_get(vpiAssertSuccessCovered, assertion_handle)
returns the number of times the assertion has succeeded nonvacuously or, if the assertion handle corresponds
to a cover sequence, the number of times the sequence has been matched. Refer to 16.12.7 and 16.14.8 for
the definition of vacuity.
vpi_get(vpiAssertVacuousSuccessCovered, assertion_handle)
returns the number of times the assertion has succeeded vacuously.
vpi_get(vpiAssertDisableCovered, assertion_handle)
returns the number of times the assertion has reached the disabled state (e.g., as a result of disable iff
condition becoming true or if an attempt starts when the disable iff is true). Refer to 16.12 for the
definition of disabled evaluation.
vpi_get(vpiAssertKillCovered, assertion_handle)

<!-- Page 1168 -->

IEEE Std returns the number of times the assertion has been killed (e.g., as a result of a control action, see 39.5.2).
vpi_get(vpiAssertFailureCovered, assertion_handle)
returns the number of times the assertion has failed. For any assertion, the number of attempts that have not
yet reached any conclusion (success, failure, disabled, or killed) can be derived from the equation:
in progress = attempts - (successes + vacuous success + disabled + killed +
failures)
The equation does not apply to cover sequence statements as there can be multiple matches
corresponding to a single attempt. The following example illustrates some of these queries:
module covtest;
bit on = 1, off = 0;
logic clk;
initial begin
clk = 0;
forever begin
#10;
clk = ~clk;
end
end
always @(false) begin
anvr: assert(on ##1 on); // assertion will not be attempted
end
always @(posedge clk) begin
aundf: assert (on ##[1:$] off); // assertion will not pass or fail
afail: assert (on ##1 off); // assertion will always fail on 2nd tick
apass: assert (on ##1 on); // assertion will succeed on each attempt
end
endmodule
For this example, the assertions will have the coverage results shown in Table40-3.
Table40-3—Assertion coverage results
vpiAssertAttempt- vpiAssertSuccess- vpiAssertFailure-
vpiCovered
Covered Covered Covered
anvr False False False False
aundf False True False False
afail False True False True
apass True True True False
The number of times an item has been covered can be obtained by the vpiCoveredCount property.
vpi_get(vpiCoveredCount, assertion_handle)
vpi_get(vpiCoveredCount, statement_handle)
vpi_get(vpiCoveredCount, signal_handle)
vpi_get(vpiCoveredCount, fsm_handle)
vpi_get(vpiCoveredCount, fsm_state_handle)

<!-- Page 1169 -->

IEEE Std returns the number of times each coverable entity referred by the handle has been covered. The handle
coverage information is only easily interpretable when the handle points to a unique coverable item (such as
an individual statement). When the handle points to an item containing multiple coverable entities (such as a
handle to a block statement containing a number of statements), the result is the sum of coverage counts for
each of the constituent entities.
vpi_get(vpiCoveredMax, assertion_handle)
vpi_get(vpiCoveredMax, statement_handle)
vpi_get(vpiCoveredMax, signal_handle)
vpi_get(vpiCoveredMax, fsm_handle)
vpi_get(vpiCoveredMax, fsm_state_handle)
returns the number of coverable entities pointed by the handle. The number returned shall always be 1 when
applied to an assertion or FSM state handle.
vpi_iterate(vpiFsm, instance-handle)
returns an iterator to all FSMs in an instance.
vpi_handle(vpiFsmStateExpression, fsm-handle)
returns the handle to the signal or expression encoding the FSM state.
vpi_iterate(vpiFsmStates, fsm-handle)
returns an iterator to all states of an FSM.
vpi_get_value(fsm_state_handle, state-handle)
returns the value of an FSM state.
#### 40.5.3 Controlling coverage
Control of the collection of coverage shall be through the vpi_control() routine:
vpi_control(<coverageControl>, <coverageType>, instance_handle)
vpi_control(<coverageControl>, <coverageType>, assertion_handle)
Statement, toggle, and FSM coverage are not individually controllable (i.e., they are controllable only at the
instance level and not on a per-statement, signal, or FSM basis). The semantics and behavior are per the
$coverage_control() system function (see 40.3.2.1). coverageControl shall be vpiCoverageStart,
vpiCoverageStop, vpiCoverageReset, or vpiCoverageCheck, as defined in 40.5.1. coverageType is any
one of the VPI coverage type properties (see 40.5.1)
To save coverage for the current simulation use the following:
vpi_control(coverageSave, <coverageType>, name)
as defined in 40.5.1. The semantics and behavior are specified per the equivalent system function
$coverage_save() (see 40.3.2.5).

<!-- Page 1170 -->

IEEE Std To merge coverage for the current simulation use the following:
vpi_control(vpiCoverageMerge, <coverageType>, name)
as defined in 40.5.1. The semantics and behavior are specified per the equivalent system function
$coverage_merge() (see 40.3.2.4).

<!-- Page 1171 -->

IEEE Std 1800-2023