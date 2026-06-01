# Chapter 32: Backannotation using the standard delay format

## 32. Backannotation using the standard delay format
### 32.1 General
This clause describes the following:
— Standard delay format (SDF) annotator
— Mapping SDF constructs to SystemVerilog
— Multiple annotations
— Multiple SDF files
— Pulse limit annotation
— SDF to SystemVerilog value mapping
— The $sdf_annotate SDF file reader
### 32.2 Overview
SDF files contain timing values for specify path delays, specparam values, timing check constraints, and
interconnect delays. SDF files can also contain other information in addition to simulation timing, but these
need not concern SystemVerilog simulation. The timing values in SDF files usually come from application-
specific integrated circuit (ASIC) delay calculation tools that take advantage of connectivity, technology,
and layout geometry information.
SystemVerilog backannotation is the process by which timing values from the SDF file update specify path
delays, specparam values, timing constraint values, and interconnect delays.
All this information is covered further in IEEE Std 1497™-2001 [B1].
### 32.3 The SDF annotator
The term SDF annotator refers to any tool capable of backannotating SDF data to a SystemVerilog
simulator. It shall issue a warning for any data it is unable to annotate.
An SDF file can contain many constructs that are not related to specify path delays, specparam values,
timing check constraint values, or interconnect delays. An example is any construct in the TIMINGENV
section of the SDF file. All constructs unrelated to SystemVerilog timing shall be ignored without any
warnings issued.
Any SystemVerilog timing value for which the SDF file does not provide a value shall not be modified
during the backannotation process, and its prebackannotation value shall be unchanged.
### 32.4 Mapping of SDF constructs to SystemVerilog
SDF timing values appear within a CELL declaration, which can contain one or more of DELAY,
TIMINGCHECK, and LABEL sections. The DELAY section contains propagation delay values for specify paths
and interconnect delays. The TIMINGCHECK section contains timing check constraint values. The LABEL
section contains new values for specparams. Backannotation into SystemVerilog is done by matching SDF
constructs to the corresponding SystemVerilog declarations and then replacing the existing SystemVerilog
timing values with those from the SDF file.

<!-- Page 926 -->

IEEE Std #### 32.4.1 Mapping of SDF delay constructs to SystemVerilog declarations
When annotating DELAY constructs that are not interconnect delays (covered in 32.4.4), the SDF annotator
looks for specify paths where the names and conditions match. When annotating TIMINGCHECK constructs,
the SDF annotator looks for timing checks of the same type where the names and conditions match.
Table32-1 shows which SystemVerilog structures can be annotated by each SDF construct in the DELAY
section.
Table32-1—Mapping of SDF delay constructs to SystemVerilog declarations
SDF construct SystemVerilog annotated structure
(PATHPULSE... Conditional and nonconditional specify path pulse limits
(PATHPULSEPERCENT... Conditional and nonconditional specify path pulse limits
(IOPATH... Conditional and nonconditional specify path delays/pulse limits
(IOPATH (RETAIN... Conditional and nonconditional specify path delays/pulse limits,
RETAIN may be ignored
(COND (IOPATH... Conditional specify path delays/pulse limits
(COND (IOPATH (RETAIN... Conditional specify path delays/pulse limits, RETAIN may be ignored
(CONDELSE (IOPATH... ifnone
(CONDELSE (IOPATH (RETAIN... ifnone, RETAIN may be ignored
(DEVICE... All specify paths to module outputs. If no specify paths, all primitives
driving module outputs.
(DEVICE port_instance... If port_instance is a module instance, all specify paths to module
outputs. If no specify paths, all primitives driving module outputs. If
port_instance is a module instance output, all specify paths to that
module output. If no specify path, all primitives driving that module
output.
In the following example, the source SDF signal sel matches the source SystemVerilog signal, and the
destination SDF signal zout also matches the destination SystemVerilog signal. Therefore, the rise/fall
times of 1.3 and 1.7 are annotated to the specify path.
SDF file:
(IOPATH sel zout (1.3) (1.7))
SystemVerilog specify path:
(sel => zout) = 0;
A conditional IOPATH delay between two ports shall annotate only to SystemVerilog specify paths between
those same two ports with the same condition. In the following example, the rise/fall times of 1.3 and 1.7
are annotated only to the second specify path:
SDF file:
(COND mode (IOPATH sel zout (1.3) (1.7)))

<!-- Page 927 -->

IEEE Std SystemVerilog specify paths:
if (!mode) (sel => zout) = 0;
if (mode) (sel => zout) = 0;
A nonconditional IOPATH delay between two ports shall annotate to all SystemVerilog specify paths
between those same two ports. In the following example, the rise/fall times of 1.3 and 1.7 are annotated
to both specify paths:
SDF file:
(IOPATH sel zout (1.3) (1.7))
SystemVerilog specify paths:
if (!mode) (sel => zout) = 0;
if (mode) (sel => zout) = 0;
#### 32.4.2 Mapping of SDF timing check constructs to SystemVerilog
Table32-2 shows which SystemVerilog timing checks are annotated to by each type of SDF timing check.
v1 is the first value of a timing check, v2 is the second value, while x indicates no value is annotated.
Table32-2—Mapping of SDF timing check constructs to SystemVerilog
SDF timing check Annotated SystemVerilog timing checks
(SETUP v1... $setup(v1), $setuphold(v1,x)
(HOLD v1... $hold(v1), $setuphold(x,v1)
(SETUPHOLD v1 v2... $setup(v1), $hold(v2), $setuphold(v1,v2)
(RECOVERY v1... $recovery(v1), $recrem(v1,x)
(REMOVAL v1... $removal(v1), $recrem(x,v1)
(RECREM v1 v2... $recovery(v1), $removal(v2), $recrem(v1,v2)
(SKEW v1... $skew(v1), $timeskew(v1)
(BIDIRECTSKEW v1 v2... $fullskew(v1,v2)
(WIDTH v1... $width(v1,x)
(PERIOD v1... $period(v1)
(NOCHANGE v1 v2... $nochange(v1,v2)
The reference and data signals of timing checks can have logical condition expressions and edges associated
with them. An SDF timing check with no conditions or edges on any of its signals shall match all
corresponding SystemVerilog timing checks regardless of whether conditions are present. In the following
example, the SDF timing check shall annotate to all the SystemVerilog timing checks:
SDF file:
(SETUPHOLD data clk (3) (4))

<!-- Page 928 -->

IEEE Std SystemVerilog timing checks:
$setuphold (posedge clk &&& mode, data, 1, 1, ntfr);
$setuphold (negedge clk &&& !mode, data, 1, 1, ntfr);
$setuphold (edge clk, data, 1, 1, ntfr);
When conditions and/or edges are associated with the signals in an SDF timing check, then they shall match
those in any corresponding SystemVerilog timing check before annotation shall happen. In the following
example, the SDF timing check shall annotate to the first SystemVerilog timing check, but not the second
and the third:
SDF file:
(SETUPHOLD data (posedge clk) (3) (4))
SystemVerilog timing checks:
$setuphold (posedge clk &&& mode, data, 1, 1, ntfr);// Annotated
$setuphold (negedge clk &&& !mode, data, 1, 1, ntfr);// Not annotated
$setuphold (edge clk, data, 1, 1, ntfr); // Not annotated
Here, the SDF timing check shall not annotate to any of the SystemVerilog timing checks.
SDF file:
(SETUPHOLD data (COND !mode (posedge clk)) (3) (4))
SystemVerilog timing checks:
$setuphold (posedge clk &&& mode, data, 1, 1, ntfr);// Not annotated
$setuphold (negedge clk &&& !mode, data, 1, 1, ntfr);// Not annotated
$setuphold (edge clk, data, 1, 1, ntfr); // Not annotated
#### 32.4.3 SDF annotation of specparams
The SDF LABEL construct annotates to specparams. Any expression containing one or more specparams is
reevaluated when annotated to from an SDF file.
The following example shows SDF LABEL constructs annotating to specparams in a SystemVerilog module.
The specparams are used in procedural delays to control when the clock transitions. The SDF LABEL
construct annotates the values of dhigh and dlow, thereby setting the period and duty cycle of the clock.
SDF file:
(LABEL
(ABSOLUTE
(dhigh 60)
(dlow 40)))
SystemVerilog file:
module clock(clk);
output clk;
reg clk;
specparam dhigh=0, dlow=0;
initial clk = 0;

<!-- Page 929 -->

IEEE Std always
begin
#dhigh clk = 1; // Clock remains low for time dlow
// before transitioning to 1
#dlow clk = 0; // Clock remains high for time dhigh
// before transitioning to 0
end
endmodule
The following example shows a specparam in an expression of a specify path. The SDF LABEL construct can
be used to change the value of the specparam and cause reevaluation of the expression.
specparam cap = 0;
...
specify
(A => Z) = 1.4 * cap + 0.7;
endspecify
#### 32.4.4 SDF annotation of interconnect delays
SDF interconnect delay annotation differs from annotation of other constructs previously described in that
there exists no corresponding SystemVerilog declaration to which to annotate. In SystemVerilog simulation,
interconnect delays are an abstraction that represents the signal propagation delay from an output or inout
module port to an input or inout module port. The INTERCONNECT construct includes a source, a load, and
delay values, while the PORT and NETDELAY constructs include only a load and delay values. Interconnect
delays can only be annotated between module ports, never between primitive pins. Table32-3 shows how
the SDF interconnect constructs in the DELAY section are annotated.
Table32-3—SDF annotation of interconnect delays
SDF construct SystemVerilog annotated structure
(PORT... Interconnect delay
(NETDELAY a Interconnect delay
(INTERCONNECT... Interconnect delay
aOnly OVI SDF version 1.0, 2.0, and 2.1 and IEEE SDF version 4.0.
Interconnect delays can be annotated to both single source and multisource nets.
When annotating a PORT construct, the SDF annotator shall search for the port and, if it exists, shall annotate
an interconnect delay to that port that shall represent the delay from all sources on the net to that port.
When annotating a NETDELAY construct, the SDF annotator shall check to see if it is annotating to a port or a
net. If it is a port, then the SDF annotator shall annotate an interconnect delay to that port. If it is a net, then
it shall annotate an interconnect delay to all load ports connected to that net. If the port or net has more than
one source, then the delay shall represent the delay from all sources. NETDELAY delays can only be
annotated to input or inout module ports or to nets.
In the case of multisource nets, unique delays can be annotated between each source/load pair using the
INTERCONNECT construct. When annotating this construct, the SDF annotator shall find the source port and
the load port; and if both exist, it shall annotate an interconnect delay between the two. If the source port is
not found or if the source port and the load port are not actually on the same net, then a warning is issued,
but the delay to the load port is annotated anyway. If this happens for a load port that is part of a multisource

<!-- Page 930 -->

IEEE Std net, then the delay is treated as if it were the delay from all source ports, which is the same as the annotation
behavior for a PORT delay. Source ports shall be output or inout ports, while load ports shall be input or inout
ports.
Interconnect delays share many of the characteristics of specify path delays. The same rules for specify path
delays for filling in missing delays and pulse limits also apply for interconnect delays. Interconnect delays
have 12 transition delays, and unique reject and error pulse limits are associated with each of the 12. An
unlimited number of future schedules are permitted.
In a SystemVerilog module, a reference to an annotated port, wherever it occurs, whether in $monitor and
$display statements or in expressions, shall provide the delayed signal value. A reference to the source
shall yield the undelayed signal value, while a reference to the load shall yield the delayed signal value. In
general, references to the signal value hierarchically before the load shall yield the undelayed signal value,
while references to the signal at or hierarchically after the load shall yield the delayed signal value. An
annotation to a hierarchical port shall affect all connected ports at higher or lower hierarchical levels,
depending on the direction of annotation. An annotation from a source port shall be interpreted as being
from all sources hierarchically higher or lower than that source port.
Up-hierarchy annotations shall be properly handled. This situation arises when the load is hierarchically
above the source. The delay to all ports that are hierarchically above the load or that connect to the net at
points hierarchically above the load is the same as the delay to that load.
Down-hierarchy annotation shall also be properly handled. This situation arises when the source is
hierarchically above the load. The delay to the load is interpreted as being from all ports that are at or above
the source or that connect to the net at points hierarchically above the source.
Hierarchically overlapping annotations are permitted. This occurs when annotations to or from the same port
take place at different hierarchical levels and, therefore, do not correspond to the same hierarchical subset of
ports. In the following example, the first INTERCONNECT statement annotates to all ports of the net that are
at or hierarchically within i53/selmode, while the second annotates to a smaller subset of ports, only those
at or hierarchically within i53/u21/in:
(INTERCONNECT i14/u5/out i53/selmode (1.43) (2.17))
(INTERCONNECT i14/u5/out i53/u21/in (1.58) (1.92))
Overlapping annotations can occur in many different ways, particularly on multisource/multiload nets, and
SDF annotation shall properly resolve all the interactions.
### 32.5 Multiple annotations
SDF annotation is an ordered process. The constructs from the SDF file are annotated in their order of
occurrence. In other words, annotation of an SDF construct can be changed by annotation of a subsequent
construct that either modifies (INCREMENT) or overwrites (ABSOLUTE) it. These do not have to be the
same construct. The following example first annotates pulse limits to an IOPATH and then annotates the
entire IOPATH, thereby overwriting the pulse limits that were just annotated:
(DELAY
(ABSOLUTE
(PATHPULSE A Z (2.1) (3.4))
(IOPATH A Z (3.5) (6.1))
Overwriting the pulse limits can be avoided by using empty parentheses to hold the current values of the
pulse limits:

<!-- Page 931 -->

IEEE Std (DELAY
(ABSOLUTE
(PATHPULSE A Z (2.1) (3.4))
(IOPATH A Z ((3.5) () ()) ((6.1) () ()) )
The preceding annotation can be simplified into a single statement as follows:
(DELAY
(ABSOLUTE
(IOPATH A Z ((3.5) (2.1) (3.4)) ((6.1) (2.1) (3.4)) )
A PORT annotation followed by an INTERCONNECT annotation to the same load shall cause only the delay
from the INTERCONNECT source to be affected. For the following net with three sources and a single load,
the delay from all sources except i13/out remains 6:
(DELAY
(ABSOLUTE
(PORT i15/in (6))
(INTERCONNECT i13/out i15/in (5))
An INTERCONNECT annotation followed by a PORT annotation shall cause the INTERCONNECT annotation to
be overwritten. Here, the delays from all sources to the load shall become 6:
(DELAY
(ABSOLUTE
(INTERCONNECT i13/out i15/in (5))
(PORT i15/in (6))
### 32.6 Multiple SDF files
More than one SDF file can be annotated. Each call to the $sdf_annotate task annotates the design with
timing information from an SDF file. Annotated values either modify (INCREMENT) or overwrite
(ABSOLUTE) values from earlier SDF files. Different regions of a design can be annotated from different
SDF files by specifying the region’s hierarchy scope as the second argument to $sdf_annotate.
### 32.7 Pulse limit annotation
For SDF annotation of delays (not timing constraints), the default values annotated for pulse limits shall be
calculated using the percentage settings for the reject and error limits. By default, these limits are 100%, but
they can be modified through invocation options. For example, assuming invocation options have set the
reject limit to 40% and the error limit to 80%, the following SDF construct shall annotate a delay of 5, a reject
limit of 2, and an error limit of 4:
(DELAY
(ABSOLUTE
(IOPATH A Z (5))
Given that the specify path delay was originally 0, the following annotation results in a delay of 5 and pulse
limits of 0:
(DELAY
(ABSOLUTE
(IOPATH A Z ((5) () ()) )

<!-- Page 932 -->

IEEE Std Annotations in INCREMENT mode can result in pulse limits less than 0, in which case they shall be adjusted
to 0. For example, if the specify path pulse limits were both 3, the following annotation results in a 0 value
for both pulse limits:
(DELAY
(INCREMENT
(IOPATH A Z (() (-4) (-5)) )
There are two SDF constructs that annotate only to pulse limits, PATHPULSE and PATHPULSEPERCENT.
They do not affect the delay. When PATHPULSE sets the pulse limits to values greater than the delay,
SystemVerilog shall exhibit the same behavior as if the pulse limits had been set equal to the delay.
### 32.8 SDF to SystemVerilog delay value mapping
SystemVerilog specify paths and interconnects can have unique delays for up to 12 state transitions (see
30.5.1). All other constructs, such as gate primitives and continuous assignments, can have only three state
transition delays (see 28.16).
For SystemVerilog specify path and interconnect delays, the number of transition delay values provided by
SDF might be less than 12.
Table32-4 shows how fewer than 12 SDF delays are extended to be 12 delays. The SystemVerilog
transition types are shown down the left-hand side, while the number of SDF delays provided is shown
across the top. The SDF values are given the names v1 through v12.
Table32-4—SDF to SystemVerilog delay value mapping
Number of SDF delay values provided
SystemVerilog
transition
1 value 2 values 3 values 6 values 12 values
0 –> 1 v1 v1 v1 v1 v1
1 –> 0 v1 v2 v2 v2 v2
0 –> z v1 v1 v3 v3 v3
z –> 1 v1 v1 v1 v4 v4
1 –> z v1 v2 v3 v5 v5
z –> 0 v1 v2 v2 v6 v6
0 –> x v1 v1 min(v1,v3) min(v1,v3) v7
x –> 1 v1 v1 v1 max(v1,v4) v8
1 –> x v1 v2 min(v2,v3) min(v2,v5) v9
x –> 0 v1 v2 v2 max(v2,v6) v10
x –> z v1 max(v1,v2) v3 max(v3,v5) v11
z –> x v1 min(v1,v2) min(v1,v2) min(v4,v6) v12
For other delays that can have at most three values, the expansion of less than three SDF delays into three
SystemVerilog delays is covered in Table28-9. More than three SDF delays are reduced to three
SystemVerilog delays by simply ignoring the extra delays. The delay to the X-state is created from the
minimum of the other three delays.

<!-- Page 933 -->

IEEE Std ### 32.9 Loading timing data from an SDF file
The syntax for the $sdf_annotate system task is shown in Syntax32-1.
sdf_annotate_task ::=
$sdf_annotate ( sdf_file [ , [ module_instance ] [ , [ "config_file" ]
[ , [ "log_file" ] [ , [ "mtm_spec" ] [ , [ "scale_factors" ] [ , [ "scale_type" ] ] ] ] ] ] ] ) ;
Syntax32-1—Syntax for $sdf_annotate system task (not in AnnexA)
The $sdf_annotate system task reads timing data from an SDF file into a specified region of the design.
sdf_file—is an expression that is a string literal, string data type, or an integral data type containing a
character string that names the file to be opened.
module_instance—is an optional argument specifying the scope to which to annotate the information in the
SDF file. The SDF annotator uses the hierarchy level of the specified instance for running the annotation.
Array indices are permitted. If the module_instance is not specified, the SDF annotator uses the module
containing the call to the $sdf_annotate system task as the module_instance for annotation.
config_file—is an optional character string argument providing the name of a configuration file. Information
in this file can be used to provide detailed control over many aspects of annotation.
log_file—is an optional character string argument providing the name of the log file used during SDF
annotation. Each individual annotation of timing data from the SDF file results in an entry in the log file.
mtm_spec—is an optional character string argument specifying which member of the min/typ/max triples
shall be annotated. The legal values for this string are described in Table32-5. This overrides any MTM_SPEC
keywords in the configuration file.
Table32-5—mtm_spec argument
Keyword Description
MAXIMUM Annotates the maximum value
MINIMUM Annotates the minimum value
TOOL_CONTROL (default) Annotates the value as selected by the simulator
TYPICAL Annotates the typical value
scale_factors—is an optional character string argument specifying the scale factors to be used while
annotating timing values. For example, "1.6:1.4:1.2" causes minimum values to be multiplied by 1.6,
typical values by 1.4, and maximum values by 1.2. The default values are 1.0:1.0:1.0. The
scale_factors argument overrides any SCALE_FACTORS keywords in the configuration file.
scale_type—is an optional character string argument specifying how the scale factors should be applied to
the min/typ/max triples. The legal values for this string are shown in Table32-6. This overrides any
SCALE_TYPE keywords in the configuration file.

<!-- Page 934 -->

IEEE Std Table32-6—scale_type argument
Keyword Description
FROM_MAXIMUM Applies scale factors to maximum value
FROM_MINIMUM Applies scale factors to minimum value
FROM_MTM (default) Applies scale factors to min/typ/max values
FROM_TYPICAL Applies scale factors to typical value

<!-- Page 935 -->

IEEE Std 1800-2023