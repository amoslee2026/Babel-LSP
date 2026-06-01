# Chapter 12: Procedural programming statements

## 12. Procedural programming statements
### 12.1 General
This clause describes the following:
— Selection statements (if–else, case, casez, casex, unique, unique0, priority)
— Loop statements (for, repeat, foreach, while, do...while, forever)
— Jump statements (break, continue, return)
### 12.2 Overview
Procedural programming statements shall be contained within any of the following constructs:
— Procedural blocks that automatically activate, introduced with one of the keywords:
• initial
• always
• always_comb
• always_latch
• always_ff
• final
See Clause9 for a description of each type of procedural block.
— Procedural blocks that activate when called, introduced with one of the keywords:
• task
• function
See Clause13 for a description of tasks and functions.
Procedural programming statements include the following:
— Selection statements (see 12.4 and 12.5)
— Loop statements (see 12.7)
— Jump statements (see 12.8)
— Sequential and parallel blocks (see 9.3)
— Timing controls (see 9.4)
— Process control (see 9.5 through 9.7)
— Procedural assignments (see 10.4 and 10.6)
— Subroutine calls (see Clause13)
### 12.3 Syntax
The syntax for procedural statements is as follows in Syntax12-1:
statement_or_null ::= // from A.6.4
statement
| { attribute_instance } ;
statement ::= [ block_identifier : ] { attribute_instance } statement_item
statement_item ::=
blocking_assignment ;

<!-- Page 316 -->

| nonblocking_assignment ;
| procedural_continuous_assignment ;
| case_statement
| conditional_statement
| subroutine_call_statement
| disable_statement
| event_trigger
| loop_statement
| jump_statement
| par_block
| procedural_timing_control_statement
| seq_block
| wait_statement
| procedural_assertion_statement
| clocking_drive ;
| randsequence_statement
| randcase_statement
| expect_property_statement
Syntax12-1—Procedural statement syntax (excerpt from AnnexA)
### 12.4 Conditional if–else statement
The conditional statement (or if–else statement) is used to make a decision about whether a statement is
executed. Formally, the syntax is given in Syntax12-2.
conditional_statement ::= // from A.6.6
[ unique_priority ] if ( cond_predicate ) statement_or_null
{ else if ( cond_predicate ) statement_or_null }
[ else statement_or_null ]
unique_priority ::= unique | unique0 | priority
cond_predicate ::= expression_or_cond_pattern { &&& expression_or_cond_pattern }
expression_or_cond_pattern ::= expression | cond_pattern
cond_pattern ::= expression matches pattern
Syntax12-2—Syntax for if–else statement (excerpt from AnnexA)
If the cond_predicate expression evaluates to true (that is, has a nonzero known value), the first statement
shall be executed. If it evaluates to false (that is, has a zero value or the value is x or z), the first
statementshall not execute. If there is an else statement and the cond_predicate expression is false, the
else statement shall be executed.
Because the numeric value of the if expression is tested for being zero, certain shortcuts are possible. For
example, the following two statements express the same logic:
if (expression)
if (expression != 0)
Because the else part of an if–else is optional, there can be confusion when an else is omitted from a
nested if sequence. This is resolved by always associating the else with the closest previous if that lacks an
else. In the following example, the else goes with the inner if, as shown by indentation.

<!-- Page 317 -->

if (index > 0)
if (rega > regb)
result = rega;
else // else applies to preceding if
result = regb;
If that association is not desired, a begin-end block statement shall be used to force the proper association, as
in the following example:
if (index > 0)
begin
if (rega > regb)
result = rega;
end
else result = regb;
#### 12.4.1 if–else–if construct
The if–else construct can be chained.
if (expression) statement;
else if (expression) statement;
else if (expression) statement;
else statement;
This sequence of if–else statements (known as an if–else–if construct) is the most general way of writing a
multiway decision. The expressions shall be evaluated in order. If any expression is true, the statement
associated with it shall be executed, and this shall terminate the whole chain. Each statement is either a
single statement or a block of statements.
The last else of the if–else–if construct handles the none-of-the-above or default case where none of the
other conditions were satisfied. Sometimes there is no explicit action for the default. In that case, the trailing
else statement can be omitted, or it can be used for error checking to catch an unexpected condition.
The following module fragment uses the if–else statement to test the variable index to decide whether one
of three modify_segn variables has to be added to the memory address and which increment is to be added
to the index variable.
// declare variables and parameters
logic [31:0] instruction,
segment_area[255:0];
logic [7:0] index;
logic [5:0] modify_seg1,
modify_seg2,
modify_seg3;
parameter
segment1 = 0, inc_seg1 = 1,
segment2 = 20, inc_seg2 = 2,
segment3 = 64, inc_seg3 = 4,
data = 128;
// test the index variable
if (index < segment2) begin
instruction = segment_area [index + modify_seg1];
index = index + inc_seg1;
end

<!-- Page 318 -->

else if (index < segment3) begin
instruction = segment_area [index + modify_seg2];
index = index + inc_seg2;
end
else if (index < data) begin
instruction = segment_area [index + modify_seg3];
index = index + inc_seg3;
end
else
instruction = segment_area [index];
#### 12.4.2 unique-if, unique0-if, and priority-if
The keywords unique, unique0, and priority can be used before an if to perform certain violation
checks.
If the keywords unique or priority are used, a violation report shall be issued if no condition matches
unless there is an explicit else. For example:
unique if ((a==0) || (a==1)) $display("0 or 1");
else if (a == 2) $display("2");
else if (a == 4) $display("4"); // values 3,5,6,7 cause a violation report
priority if (a[2:1]==0) $display("0 or 1");
else if (a[2] == 0) $display("2 or 3");
else $display("4 to 7"); // covers all other possible values,
// so no violation report
If the keyword unique0 is used, there shall be no violation if no condition is matched. For example:
unique0 if ((a==0) || (a==1)) $display("0 or 1");
else if (a == 2) $display("2");
else if (a == 4) $display("4"); // values 3,5,6,7
// cause no violation report
Unique-if and unique0-if assert that there is no overlap in a series of if–else–if conditions, i.e., they are
mutually exclusive and hence it is safe for the conditions to be evaluated in parallel.
In unique-if and unique0-if, the conditions may be evaluated and compared in any order. The
implementation shall continue the evaluations and comparisons after finding a true condition. A unique-if or
unique0-if is violated if more than one condition is found true. The implementation shall issue a violation
report and execute the statement associated with the true condition that appears first in the if statement, but
not the statements associated with other true conditions.
After finding a uniqueness violation, the implementation is not required to continue evaluating and
comparing additional conditions. The implementation is not required to try more than one order of
evaluations and comparisons of conditions. The presence of side effects in conditions may cause
nondeterministic results.
A priority-if indicates that a series of if–else–if conditions shall be evaluated in the order listed. In the
preceding example, if the variable a had a value of 0, it would satisfy both the first and second conditions,
requiring priority logic.

<!-- Page 319 -->

The unique, unique0, and priority keywords apply to the entire series of if–else–if conditions. In the
preceding examples, it would have been illegal to insert any of these keywords after any of the occurrences
of else. To nest another if statement within such a series of conditions, a begin-end block should be used.
12.4.2.1 Violation reports generated by unique-if, unique0-if, and priority-if constructs
The descriptions in 12.4.2 mention several cases in which a violation report shall be generated by unique-if,
unique0-if, or priority-if statements. These violation checks shall be immune to false violation reports due to
zero-delay glitches in the active region set (see 4.4.1).
A unique, unique0, or priority violation check is evaluated at the time the statement is executed, but
violation reporting is deferred until the Observed region of the current time step (see 4.4). The violation
reporting can be controlled by using assertion control system tasks (see 20.11).
Once a violation is detected, a pending violation report is scheduled in the Observed region of the current
time step. It is scheduled on a violation report queue associated with the currently executing process. A
violation report flush point is said to be reached if any of the following conditions are met:
— The procedure, having been suspended earlier due to reaching an event control or wait statement,
resumes execution.
— The procedure was declared by an always_comb or always_latch statement, and its execution is
resumed due to a transition on one of its dependent signals.
If a violation report flush point is reached in a process, its violation report queue is cleared. Any pending
violation reports are discarded.
In the Observed region of each simulation time step, each pending violation report shall mature or be
confirmed for reporting. Once a report matures, it shall no longer be flushed. A tool-specific violation report
mechanism is then used to report each violation, and the pending violation report is cleared from the
appropriate process violation report queue.
The following is an example of a unique-if that is immune to zero-delay glitches in the active region set:
always_comb begin
not_a = !a;
end
always_comb begin : a1
u1: unique if (a)
z = a | b;
else if (not_a)
z = a | c;
end
In this example, uniqueif u1 is checking for overlap in the two conditional expressions. When a and
not_a are in a state of 0 and 1, respectively, and a transitions to 1, this uniqueif could be executed while
a and not_a are both true, so the violation check for uniqueness will fail. Since this check is in the active
region set, the failure is not immediately reported. After the update to not_a, process a1 will be
rescheduled, which results in a flush of the original violation report. The violation check will now pass, and
no violation will be reported.
Another example shows how looping constructs are likewise immune to zero-delay glitches in the active
region set:
always_comb begin
for (int j = 0; j < 3; j++)

<!-- Page 320 -->

not_a[j] = !a[j];
end
always_comb begin : a1
for (int j = 0; j < 3; j++)
unique if (a[j])
z[j] = a[j] | b[j];
else if (not_a[j])
z[j] = a[j] | c[j];
end
This example is identical to the previous example but adds loop statements. Each loop iteration
independently checks for a uniqueness violation in the exact same manner as the previous example. Any
iteration in the loop can report a uniqueness violation. If the process a1 is rescheduled, all violations in the
loop are flushed and the entire loop is reevaluated.
12.4.2.2 If statement violation reports and multiple processes
As described in the previous subclauses (see 12.4.2 and 12.4.2.1), violation reports are inherently associated
with the process in which they are executed. This means that a violation check within a task or function may
be executed several times due to the task or function being called by several different processes, and each of
these different process executions is independent. The following example illustrates this situation:
module fsm(...);
function bit f1(bit a, bit not_a, ...)
...
a1: unique if (a)
...
else if (not_a)
...
endfunction
...
always_comb begin : b1
some_stuff = f1(c, d, ...);
...
end
always_comb begin : b2
other_stuff = f1(e, f, ...);
...
end
endmodule
In this case, there are two different processes that may call process a1: b1 and b2. Suppose simulation
executes the following scenario in the first passage through the Active region of each time step. Note that
this example refers to three distinct points in simulation time and how glitch resolution is handled for each
specific time step:
a) In time step 1, b1 executes with c=1 and d=1, and b2 executes with e=1 and f=1.
In this first time step, since a1 fails independently for processes b1 and b2, its failure is reported
twice.
b) In time step 2, b1 executes with c=1 and d=1, then again with c=1 and d=0.
In this second time step, the failure of a1 in process b1 is flushed when the process is re-triggered,
and since the final execution passes, no failure is reported.

<!-- Page 321 -->

c) In time step 3, b1 executes with c=1 and d=1, then b2 executes with e=1 and f=0.
In this third time step, the failure in process b1 does not see a flush point, so that failure is reported.
In process b2, the violation check passes, so no failure is reported from that process.
### 12.5 Case statement
The case statement is a multiway decision statement that tests whether an expression matches one of a
number of other expressions and branches accordingly. The case statement has the syntax shown in
Syntax12-3.
case_statement ::= // from A.6.7
[ unique_priority ] case_keyword ( case_expression )
case_item { case_item } endcase
| [ unique_priority ] case_keyword ( case_expression ) matches
case_pattern_item { case_pattern_item } endcase
| [ unique_priority ] case ( case_expression ) inside
case_inside_item { case_inside_item } endcase
case_keyword ::= case | casez | casex
case_expression ::= expression
case_item ::=
case_item_expression { , case_item_expression } : statement_or_null
| default [ : ] statement_or_null
case_pattern_item ::=
pattern [ &&& expression ] : statement_or_null
| default [ : ] statement_or_null
case_inside_item ::=
range_list : statement_or_null
| default [ : ] statement_or_null
case_item_expression ::= expression
Syntax12-3—Syntax for case statements (excerpt from AnnexA)
The default statement shall be optional. Use of multiple default statements in one case statement shall
be illegal.
The case_expression and case_item_expressions are not required to be constant expressions.
A simple example of the use of the case statement is the decoding of variable data to produce a value for
result as follows:
logic [15:0] data;
logic [9:0] result;
case (data)
16'd0: result = 10'b0111111111;
16'd1: result = 10'b1011111111;
16'd2: result = 10'b1101111111;
16'd3: result = 10'b1110111111;
16'd4: result = 10'b1111011111;
16'd5: result = 10'b1111101111;

<!-- Page 322 -->

16'd6: result = 10'b1111110111;
16'd7: result = 10'b1111111011;
16'd8: result = 10'b1111111101;
16'd9: result = 10'b1111111110;
default result = 'x;
endcase
The case_expression shall be evaluated exactly once and before any of the case_item_expressions. The
case_item_expressions shall be evaluated and then compared in the exact order in which they appear. If
there is a default case_item, it is ignored during this linear search. During the linear search, if one of the
case_item_expressions matches the case_expression, then the statement associated with that case_item shall
be executed, and the linear search shall terminate. If all comparisons fail and the default item is given,
then the default item statement shall be executed. If the default statement is not given and all of the
comparisons fail, then none of the case_item statements shall be executed.
Apart from syntax, the case statement differs from the multiway if–else–if construct in two important ways:
a) The conditional expressions in the if–else–if construct are more general than comparing one
expression with several others, as in the case statement.
b) The case statement provides a definitive result when there are x and z values in an expression.
In a case_expression comparison, the comparison only succeeds when each bit matches exactly with respect
to the values 0, 1, x, and z. As a consequence, care is needed in specifying the expressions in the case
statement. The bit length of all the expressions needs to be equal, so that exact bitwise matching can be
performed. Therefore, the length of all the case_item_expressions, as well as the case_expression, shall be
made equal to the length of the longest case_expression and case_item_expressions. If any of these
expressions is unsigned, then all of them shall be treated as unsigned. If all of these expressions are signed,
then they shall be treated as signed.
The reason for providing a case_expression comparison that handles the x and z values is that it provides
a mechanism for detecting such values and reducing the pessimism that can be generated by their presence.
Example 1: The following example illustrates the use of a case statement to handle x and z values
properly:
case (select[1:2])
2'b00: result = 0;
2'b01: result = flaga;
2'b0x,
2'b0z: result = flaga ? 'x : 0;
2'b10: result = flagb;
2'bx0,
2'bz0: result = flagb ? 'x : 0;
default result = 'x;
endcase
In this example, if select[1] is 0 and flaga is 0, then even if the value of select[2] is x or z, result
should be 0—which is resolved by the third case_item.
Example 2: The following example shows another way to use a case statement to detect x and z values:
case (sig)
1'bz: $display("signal is floating");
1'bx: $display("signal is unknown");
default: $display("signal is %b", sig);
endcase

<!-- Page 323 -->

#### 12.5.1 Case statement with do-not-cares
Two other types of case statements are provided to allow handling of do-not-care conditions in the case
comparisons. One of these treats high-impedance values (z) as do-not-cares, and the other treats both
high-impedance and unknown (x) values as do-not-cares. These case statements can be used in the same
way as the traditional case statement, but they begin with keywords casez and casex, respectively.
Do-not-care values (z values for casez, z and x values for casex) in any bit of either the case_expression
or the case_items shall be treated as do-not-care conditions during the comparison, and that bit position shall
not be considered.
The syntax of literal numbers allows the use of the question mark (?) in place of z in these case
statements. This provides a convenient format for specification of do-not-care bits in case statements.
Example 1: The following is an example of the casez statement. It demonstrates an instruction decode,
where values of the MSBs select which task should be called. If the MSB of ir is a 1, then the task
instruction1 is called, regardless of the values of the other bits of ir.
logic [7:0] ir;
casez (ir)
8'b1???????: instruction1(ir);
8'b01??????: instruction2(ir);
8'b00010???: instruction3(ir);
8'b000001??: instruction4(ir);
endcase
Example 2: The following is an example of the casex statement. It demonstrates an extreme case of how do-
not-care conditions can be dynamically controlled during simulation. In this example, if
r = 8'b01100110, then the task stat2 is called.
logic [7:0] r, mask;
mask = 8'bx0x0x0x0;
casex (r ^ mask)
8'b001100xx: stat1;
8'b1100xx00: stat2;
8'b00xx0011: stat3;
8'bxx010100: stat4;
endcase
#### 12.5.2 Constant expression in case statement
A constant expression can be used for the case_expression. The value of the constant expression shall be
compared against the case_item_expressions.
The following example demonstrates the usage by modeling a 3-bit priority encoder:
logic [2:0] encode ;
case (1)
encode[2] : $display("Select Line 2") ;
encode[1] : $display("Select Line 1") ;
encode[0] : $display("Select Line 0") ;
default $display("Error: One of the bits expected ON");
endcase

<!-- Page 324 -->

In this example, the case_expression is a constant expression (1). The case_items are expressions (bit-
selects) and are compared against the constant expression for a match.
#### 12.5.3 unique-case, unique0-case, and priority-case
The case, casez, and casex keywords can be qualified by priority, unique, or unique0 keywords to
perform certain violation checks. These are collectively referred to as a priority-case, unique-case, or
unique0-case. A priority-case shall act on the first match only. Unique-case and unique0-case assert that
there are no overlapping case_items and hence that it is safe for the case_items to be evaluated in parallel.
In unique-case and unique0-case, the case_expression shall be evaluated exactly once and before any of the
case_item_expressions. The case_item_expressions may be evaluated in any order and compared in any
order. The implementation shall continue the evaluations and comparisons after finding a matching
case_item. Unique-case and unique0-case are violated if more than one case_item is found to match the
case_expression. The implementation shall issue a violation report and execute the statement associated
with the matching case_item that appears first in the case statement, but not the statements associated with
other matching case_items.
After finding a uniqueness violation, the implementation is not required to continue evaluating and
comparing additional case_items. It is not a violation of uniqueness for a single case_item to contain more
than one case_item_expression that matches the case_expression. If a case_item_expression matches the
case_expression, the implementation is not required to evaluate additional case_item_expressions in the
same case_item. The implementation is not required to try more than one order of evaluations and
comparisons of case_item_expressions. The presence of side-effects in case_item_expressions may cause
nondeterministic results.
If the case is qualified as priority or unique, the simulator shall issue a violation report if no case_item
matches. A violation report may be issued at compile time if it is possible then to determine the violation. If
it is not possible to determine the violation at compile time, a violation report shall be issued during run
time. If the case is qualified as unique0, the implementation shall not issue a violation report if no
case_item matches.
NOTE—By specifying unique or priority, it is not necessary to code a default case to trap unexpected case
values.
Consider the following example:
bit [2:0] a;
unique case(a) // values 3,5,6,7 cause a violation report
0,1: $display("0 or 1");
2: $display("2");
4: $display("4");
endcase
priority casez(a) // values 4,5,6,7 cause a violation report
3'b00?: $display("0 or 1");
3'b0??: $display("2 or 3");
endcase
unique0 case(a) // values 3,5,6,7 do not cause a violation report
0,1: $display("0 or 1");
2: $display("2");
4: $display("4");
endcase

<!-- Page 325 -->

12.5.3.1 Violation reports generated by unique-case, unique0-case, and priority-case
constructs
The descriptions in 12.5.3 mention several cases in which a violation report shall be generated by unique-
case, unique0-case, or priority-case statements. These violation checks shall be immune to false violation
reports due to zero-delay glitches in the active region set (see 4.4.1). The violation reporting can be
controlled by using assertion control system tasks (see 20.11).
The mechanics of handling zero-delay glitches shall be identical to those used when processing zero-delay
glitches for unique-if, unique0-if, and priority-if constructs (see 12.4.2.1).
The following is an example of a unique-case that is immune to zero-delay glitches in the active region set:
always_comb begin
not_a = !a;
end
always_comb begin : a1
unique case (1'b1)
a : z = b;
not_a : z = c;
endcase
end
In this example the uniquecase is checking for overlap in the two case_item selects. When a and not_a
are in state 0 and 1, respectively, and a transitions to 1, this uniquecase could be executed while a and
not_a are both true, so the violation check for uniqueness will fail. But since this violation check is in the
active region set, the failure is not reported immediately. After the update to not_a, process a1 will be
rescheduled, which results in a flush of the original violation report. The violation check will now pass, and
no violation will be reported.
12.5.3.2 Case statement violation reports and multiple processes
Case violation reports shall behave in the same manner as if violation reports when dealing with multiple
processes (see 12.4.2.2).
#### 12.5.4 Set membership case statement
The keyword inside can be used after the parenthesized expression to indicate a set membership (see
11.4.13). In a case-inside statement, the case_expression shall be compared with each case_item_expression
(range_list) using the set membership inside operator. The inside operator uses asymmetric wildcard
matching (see 11.4.6). Accordingly, the case_expression shall be the left operand, and each
case_item_expression shall be the right operand. The case_expression and each case_item_expression in
braces shall be evaluated in the order specified by a normal case, unique-case, or priority-case statement. A
case_item shall be matched when the inside operation compares the case_expression to the
case_item_expressions and returns 1'b1 and no match when the operation returns 1'b0 or 1'bx. If all
comparisons do not match and the default item is given, the default item statement shall be executed.
For example:
logic [2:0] status;
always @(posedge clock)
priority case (status) inside
1, 3 : task1; // matches 'b001 'b011
3'b0?0, [4:7]: task2; // matches 'b000 'b010 'b0x0 'b0z0

<!-- Page 326 -->

// 'b100 'b101 'b110 'b111
endcase // priority case fails all other values including
// 'b00x 'b01x 'bxxx
### 12.6 Pattern matching conditional statements
Pattern matching provides a visual and succinct notation to compare a value against structures, tagged
unions, and constants and to access their members. Pattern matching can be used with case and if–else
statements and with conditional expressions. Before describing pattern matching in those contexts, the
general concepts are described first.
A pattern is a nesting of tagged union and structure expressions with identifiers, constant expressions (see
11.2.1), and the wildcard pattern “.*” at the leaves. For tagged union patterns, the identifier following the
tagged keyword is a union member name. For void members, the nested member pattern is omitted.
pattern ::= // from A.6.7.1
( pattern )
| . variable_identifier
| . *
| constant_expression
| tagged member_identifier [ pattern ]
| ' { pattern { , pattern } }
| ' { member_identifier : pattern { , member_identifier : pattern } }
Syntax12-4—Pattern syntax (excerpt from AnnexA)
A pattern always occurs in a context of known type because it is matched against an expression of known
type. Recursively, its nested patterns also have known type. A constant expression pattern shall be of
integral type. Thus a pattern can always be statically type-checked.
Each pattern introduces a new scope; the extent of this scope is described separately below for case
statements, if–else statements, and conditional expressions. Each pattern identifier is implicitly declared as a
new variable within the pattern’s scope, with the unique type that it shall have based on its position in the
pattern. Pattern identifiers shall be unique in the pattern, i.e., the same identifier cannot be used in more than
one position in a single pattern.
In pattern-matching, the value V of an expression is always matched against a pattern, and static type
checking verifies that V and the pattern have the same type. The result of a pattern match is as follows:
— A 1-bit determined value: 0 (false, or fail) or 1 (true, or succeed). The result cannot be x or z even if
the value and pattern contain such bits.
— If the match succeeds, the pattern identifiers are bound to the corresponding members from V, using
ordinary procedural assignment.
— Each pattern is matched using the following simple recursive rule:
• An identifier pattern always succeeds (matches any value), and the identifier is bound to that
value (using ordinary procedural assignment).
• The wildcard pattern “.*” always succeeds.
• A constant expression pattern succeeds if V is equal to its value.
• A tagged union pattern succeeds if the value has the same tag and, recursively, if the nested
pattern matches the member value of the tagged union.
• A structure pattern succeeds if, recursively, each of the nested member patterns matches the
corresponding member values in V. In structure patterns with named members, the textual order
of members does not matter, and members can be omitted. Omitted members are ignored.

<!-- Page 327 -->

Conceptually, if the value V is seen as a flattened vector of bits, the pattern specifies which bits to match,
with what values they should be matched, and, if the match is successful, which bits to extract and bind to
the pattern identifiers. Matching can be insensitive to x and z values, as described in the following
individual constructs.
#### 12.6.1 Pattern matching in case statements
In a pattern-matching case statement, the expression in parentheses is followed by the keyword matches,
and the statement contains a series of case_pattern_items. The left-hand side of a case item, before the colon
( : ), consists of a pattern and, optionally, the operator &&& followed by a Boolean filter expression. The
right-hand side of a case item is a statement. Each pattern introduces a new scope, in which its pattern
identifiers are implicitly declared; this scope extends to the optional filter expression and the statement in the
right-hand side of the same case item. Thus different case items can reuse pattern identifiers.
All the patterns are completely statically type-checked. The expression being tested in the pattern-matching
case statement shall have a known type, which is the same as the type of the pattern in each case item.
The expression in parentheses in a pattern-matching case statement shall be evaluated exactly once. Its value
V shall be matched against the left-hand sides of the case items, one at a time, in the exact order given,
ignoring the default case item if any. During this linear search, if a case item is selected, its statement is
executed and the linear search is terminated. If no case item is selected and a default case item is given,
then its statement is executed. If no case item is selected and no default case item is given, no statement is
executed.
For a case item to be selected, the value V shall match the pattern (and the pattern identifiers are assigned the
corresponding member values in V), and then the Boolean filter expression shall evaluate to true (a
determined value other than 0).
Example 1:
typedef union tagged {
void Invalid;
int Valid;
} VInt;
...
VInt v;
...
case (v) matches
tagged Invalid : $display ("v is Invalid");
tagged Valid .n : $display ("v is Valid with value %d", n);
endcase
In the case statement, if v currently has the Invalid tag, the first pattern is matched. Otherwise, it has the
Valid tag, and the second pattern is matched. The identifier n is bound to the value of the Valid member,
and this value is displayed. It is impossible to access the integer member value (n) when the tag is Invalid.
Example 2:
typedef union tagged {
struct {
bit [4:0] reg1, reg2, regd;
} Add;
union tagged {
bit [9:0] JmpU;
struct {

<!-- Page 328 -->

bit [1:0] cc;
bit [9:0] addr;
} JmpC;
} Jmp;
} Instr;
...
Instr instr;
...
case (instr) matches
tagged Add '{.r1, .r2, .rd} &&& (rd != 0) : rf[rd] = rf[r1] + rf[r2];
tagged Jmp .j : case (j) matches
tagged JmpU .a : pc = pc + a;
tagged JmpC '{.c, .a}: if (rf[c]) pc = a;
endcase
endcase
If instr holds an Add instruction, the first pattern is matched, and the identifiers r1, r2, and rd are bound
to the three register fields in the nested structure value. The right-hand statement executes the instruction on
the register file rf. It is impossible to access these register fields if the tag is Jmp. If instr holds a Jmp
instruction, the second pattern is matched, and the identifier j is bound to the nested tagged union value. The
inner case statement, in turn, matches this value against JmpU and JmpC patterns and so on.
Example 3 (same as previous example, but using wildcard and constant patterns to eliminate the rd = 0
case; in some processors, register 0 is a special “discard” register):
case (instr) matches
tagged Add '{.*, .*, 0} : ; // no op
tagged Add '{.r1, .r2, .rd} : rf[rd] = rf[r1] + rf[r2];
tagged Jmp .j : case (j) matches
tagged JmpU .a : pc = pc + a;
tagged JmpC '{.c, .a} : if (rf[c]) pc = a;
endcase
endcase
Example 4 (same as previous example except that the first inner case statement involves only structures and
constants but no tagged unions):
case (instr) matches
tagged Add .s: case (s) matches
'{.*, .*, 0} : ; // no op
'{.r1, .r2, .rd} : rf[rd] = rf[r1] + rf[r2];
endcase
tagged Jmp .j: case (j) matches
tagged JmpU .a : pc = pc + a;
tagged JmpC '{.c, .a} : if (rf[c]) pc = a;
endcase
endcase
Example 5 (same as previous example, but using nested tagged union patterns):
case (instr) matches
tagged Add '{.r1, .r2, .rd} &&& (rd != 0) : rf[rd] = rf[r1] + rf[r2];
tagged Jmp (tagged JmpU .a) : pc = pc + a;
tagged Jmp (tagged JmpC '{.c, .a}) : if (rf[c]) pc = a;
endcase
Example 6 (same as previous example, with named structure components):

<!-- Page 329 -->

case (instr) matches
tagged Add '{reg2:.r2,regd:.rd,reg1:.r1} &&& (rd != 0):
rf[rd] = rf[r1] + rf[r2];
tagged Jmp (tagged JmpU .a) : pc = pc + a;
tagged Jmp (tagged JmpC '{addr:.a,cc:.c}) : if (rf[c]) pc = a;
endcase
The casez and casex keywords can be used instead of case, with the same semantics. In other words,
during pattern matching, wherever 2 bits are compared (whether they are tag bits or members), the casez
form ignores z bits, and the casex form ignores both z and x bits.
The priority and unique qualifiers can also be used. priority implies that some case item will be
selected. unique implies that some case item will be selected and also implies that exactly one case item
will be selected so that they can be evaluated in parallel.
#### 12.6.2 Pattern matching in if statements
The predicate in an if–else statement can be a series of clauses separated with the &&& operator. Each clause
is either an expression (used as a Boolean filter) or has the form: expression matches pattern. The clauses
represent a sequential conjunction from left to right (i.e., if any clause fails, the remaining clauses are not
evaluated) and all of them shall succeed for the predicate to be true. Boolean expression clauses are
evaluated as usual. Each pattern introduces a new scope, in which its pattern identifiers are implicitly
declared; this scope extends to the remaining clauses in the predicate and to the corresponding “true” arm of
the if–else statement.
In each e matches p clause, e and p shall have the same known statically known type. The value of e is
matched against the pattern p as previously described.
Even though the pattern matching clauses always return a defined 1-bit result, the overall result can be
ambiguous because of the Boolean filter expressions in the predicate. The standard semantics of if–else
statements holds, i.e., the first statement is executed if, and only if, the result is a determined value other
than0.
Example 1:
if (e matches (tagged Jmp (tagged JmpC '{cc:.c,addr:.a})))
... // c and a can be used here
else
...
Example 2 (same as previous example, illustrating a sequence of two pattern matches with identifiers bound
in the first pattern used in the second pattern):
if (e matches (tagged Jmp .j) &&&
j matches (tagged JmpC '{cc:.c,addr:.a}))
... // c and a can be used here
else
...
Example 3 (same as first example, but adding a Boolean expression to the sequence of clauses). The idea
expressed is “if e is a conditional jump instruction and the condition register is not equal to zero ...”.

<!-- Page 330 -->

if (e matches (tagged Jmp (tagged JmpC '{cc:.c,addr:.a}))
&&& (rf[c] != 0))
... // c and a can be used here
else
...
The priority and unique qualifiers can be used even if they use pattern matching.
#### 12.6.3 Pattern matching in conditional expressions
A conditional expression (e1 ? e2 : e3) can also use pattern matching, i.e., the predicate e1 can be a
sequence of expressions and “expression matches pattern” clauses separated with the &&& operator, just
like the predicate of an if–else statement. The clauses represent a sequential conjunction from left to right,
(i.e., if any clause fails, the remaining clauses are not evaluated) and all of them shall succeed for the
predicate to be true. Boolean expression clauses are evaluated as usual. Each pattern introduces a new scope,
in which its pattern identifiers are implicitly declared; this scope extends to the remaining clauses in the
predicate and to the consequent expression e2.
As described in the previous subclause, e1 can evaluate to true, false, or an ambiguous value. The semantics
of the overall conditional expression is described in 11.4.11, based on these three possible outcomes for e1.
### 12.7 Loop statements
SystemVerilog provides six types of looping constructs, as shown in Syntax12-5.
loop_statement ::= // from A.6.8
forever statement_or_null
| repeat ( expression ) statement_or_null
| while ( expression ) statement_or_null
| for ( [ for_initialization ] ; [ expression ] ; [ for_step ] )
statement_or_null
| do statement_or_null while ( expression ) ;
| foreach ( ps_or_hierarchical_array_identifier [ loop_variables ] ) statement
for_initialization ::=
list_of_variable_assignments
| for_variable_declaration { , for_variable_declaration }
for_variable_declaration ::=
[ var ] data_type variable_identifier = expression { , variable_identifier = expression }18
for_step ::= for_step_assignment { , for_step_assignment }
for_step_assignment ::=
operator_assignment
| inc_or_dec_expression
| function_subroutine_call
loop_variables ::= [ index_variable_identifier ] { , [ index_variable_identifier ] }
18) When a type_reference is used in a net declaration, it shall be preceded by a net type keyword; and when it is used
in a variable declaration, it shall be preceded by the var keyword.
Syntax12-5—Loop statement syntax (excerpt from AnnexA)

<!-- Page 331 -->

#### 12.7.1 The for-loop
The for-loop controls execution of its associated statement(s) by a three-step process:
a) Unless the optional for_initialization is omitted, executes one or more for_initialization
assignments, which are normally used to initialize a variable that controls the number of times the
loop is executed.
b) Unless the optional expression is omitted, evaluates the expression. If the result is false (as defined
in 12.4), the for-loop shall exit. Otherwise, or if the expression is omitted, the for-loop shall execute
its associated statement(s) and then perform step c). If the expression evaluates to an unknown or
high-impedance value, it shall be treated as zero.
c) Unless the optional for_step is omitted, executes one or more for_step assignments, normally used
to modify the value of the loop-control variable, then repeats step b).
The variables used to control a for-loop can be declared prior to the loop. If loops in two or more parallel
processes use the same loop control variable, there is a potential of one loop modifying the variable while
other loops are still using it.
The variables used to control a for-loop can also be declared within the loop, as part of the for_initialization
assignments. This creates an implicit begin-end block around the loop, containing declarations of the loop
variables with automatic lifetime. This block creates a new hierarchical scope, making the variables local to
the loop scope. The block is unnamed by default, but can be named by adding a statement label (9.3.5) to the
for-loop statement. Thus, other parallel loops cannot inadvertently affect the loop control variable. For
example:
module m;
initial begin
for (int i = 0; i <= 255; i++)
...
end
initial begin
loop2: for (int i = 15; i >= 0; i--)
...
end
endmodule
This is equivalent to the following:
module m;
initial begin
begin
automatic int i;
for (i = 0; i <= 255; i++)
...
end
end
initial begin
begin : loop2
automatic int i;
for (i = 15; i >= 0; i--)
...
end
end
endmodule

<!-- Page 332 -->

Only for-loop statements containing variable declarations as part of the for-initialization assignments create
implicit begin-end blocks around them.
The initial declaration or assignment statement can be one or more comma-separated statements. The step
assignment can also be one or more comma-separated assignment statements, increment or decrement
expressions, or function calls.
for ( int count = 0; count < 3; count++ )
value = value +((a[count]) * (count+1));
for ( int count = 0, done = 0, j = 0; j * count < 125; j++, count++)
$display("Value j = %d\n", j );
In a for-loop initialization, either all or none of the control variables shall be locally declared. In the second
loop of the example above, count, done, and j are all locally declared. The following would be illegal
because it attempts to locally declare y whereas x was not locally declared:
for (x = 0, int y = 0; ...)
...
In a for-loop initialization that declares multiple local variables, the initialization expression of a local
variable can use earlier local variables.
for (int i = 0, j = i + offset; i < N; i++,j++)
...
#### 12.7.2 The repeat loop
The repeat-loop executes a statement a fixed number of times. The loop evaluates its expression once before
the loop starts—changing any part of the expression once in the loop has no effect. If the expression
evaluates to a negative, unknown or high-impedance value, it shall be treated as zero, and no statement shall
be executed.
In the following example of a repeat-loop, add and shift operators implement a multiplier:
parameter size = 8, longsize = 16;
logic [size:1] opa, opb;
logic [longsize:1] result;
begin : mult
logic [longsize:1] shift_opa, shift_opb;
shift_opa = opa;
shift_opb = opb;
result = 0;
repeat (size) begin
if (shift_opb[1])
result = result + shift_opa;
shift_opa = shift_opa << 1;
shift_opb = shift_opb >> 1;
end
end
#### 12.7.3 The foreach-loop
The foreach-loop construct specifies iteration over the elements of an array. Its argument is an identifier that
designates any type of packed or unpacked array followed by a comma-separated list of loop variables

<!-- Page 333 -->

enclosed in square brackets. A string variable is treated as a dynamic array of bytes indexed from 0 to N-
1, where N is the number of characters in the string. Each loop variable corresponds to one of the dimensions
of the array. The foreach-loop is similar to a repeat-loop that uses the array bounds to specify the repeat
count instead of an expression. It shall be an error to include a function call as an implicit variable
declaration in the identifier (see 13.4.1).
Examples:
string words [2] = '{ "hello", "world" };
int prod [1:8] [1:3];
foreach( words[j] )
$display( j , words[j] ); // print each index and value
foreach( prod[k, m] )
prod[k][m] = k * m; // initialize
The number of loop variables shall not be greater than the number of dimensions of the array variable. Loop
variables may be omitted to indicate no iteration over that dimension of the array, and trailing commas in the
list may also be omitted. As in a for-loop (12.7.1), a foreach-loop creates an implicit begin-end block around
the loop statement, containing declarations of the loop variables with automatic lifetime. This block creates
a new hierarchical scope, making the variables local to the loop scope. The block is unnamed by default, but
can be named by adding a statement label (9.3.5) to the foreach statement. foreach-loop variables are read-
only. The type of each loop variable is implicitly declared to be consistent with the type of array index. It
shall be an error for any loop variable to have the same identifier as the array.
The mapping of loop variables to array indices is determined by the dimension cardinality, as described in
20.7. The foreach-loop arranges for higher cardinality indices to change more rapidly.
// 1 2 3 3 4 1 2 -> Dimension numbers
int A [2][3][4]; bit [3:0][2:1] B [5:1][4];
foreach( A [ i, j, k ] ) ...
foreach( B [ q, r, , s ] ) ...
The first foreach-loop causes i to iterate from 0 to 1, j from 0 to 2, and k from 0 to 3. The second foreach-
loop causes q to iterate from 5 to 1, r from 0 to 3, and s from 2 to 1 (iteration over the third index is
skipped).
If the dimensions of a dynamically sized array are changed while iterating over a foreach-loop construct, the
results are undefined and may cause invalid index values to be generated.
Multiple loop variables correspond to nested loops that iterate over the given indices. The nesting of the
loops is determined by the dimension cardinality; outer loops correspond to lower cardinality indices. In the
preceding first example, the outermost loop iterates over i, and the innermost loop iterates over k.
When loop variables are used in expressions other than as indices to the designated array, they are auto-cast
into a type consistent with the type of index. For fixed-size and dynamic arrays, the auto-cast type is int.
For associative arrays indexed by a specific index type, the auto-cast type is the same as the index type. To
use different types, an explicit cast can be used.

<!-- Page 334 -->

#### 12.7.4 The while-loop
The while-loop repeatedly executes a statement as long as a control expression is true (as defined in 12.4). If
the expression is not true at the beginning of the execution of the while-loop, the statement shall not be
executed at all.
The following example counts the number of logic 1 values in data:
begin : count1s
logic [7:0] tempreg;
count = 0;
tempreg = data;
while (tempreg) begin
if (tempreg[0])
count++;
tempreg >>= 1;
end
end
#### 12.7.5 The do...while-loop
The do...while-loop differs from the while-loop in that a do...while-loop tests its control expression at the
end of the loop. Loops with a test at the end are sometimes useful to save duplication of the loop body.
string s;
if ( map.first( s ) )
do
$display( "%s : %d\n", s, map[ s ] );
while ( map.next( s ) );
The condition can be any expression that can be treated as a Boolean. It is evaluated after the statement.
#### 12.7.6 The forever-loop
The forever-loop repeatedly executes a statement. To avoid a zero-delay infinite loop, which could hang the
simulation event scheduler, the forever loop should only be used in conjunction with the timing controls or
the disable statement. For example:
initial begin
clock1 <= 0;
clock2 <= 0;
fork
forever #10 clock1 = ~clock1;
#5 forever #10 clock2 = ~clock2;
join
end

<!-- Page 335 -->

### 12.8 Jump statements
jump_statement ::= // from A.6.5
return [ expression ] ;
| break ;
| continue ;
Syntax12-6—Jump statement syntax (excerpt from AnnexA)
SystemVerilog provides the C-like jump statements break, continue, and return.
break // break out of loop, as in C
continue // skip to end of loop, as in C
return expression // exit from a function
return // exit from a task or void function
The continue and break statements can only be used in a loop. The continue statement jumps to the end
of the loop and executes the loop control if present. The break statement jumps out of the loop. In the case
of a foreach loop with multiple dimensions, the continue statement jumps to the end of the loop for the
current set of loop variable values, and the break statement jumps out of the entire loop, not just out of the
current dimension.
The continue and break statements cannot be used inside a fork-join block to control a loop outside the
fork-join block.
The return statement can only be used in a subroutine. In a function returning a value, the return statement
shall have an expression of the correct type.
NOTE—SystemVerilog does not include the C goto statement.

<!-- Page 336 -->

IEEE Std 1800-2023