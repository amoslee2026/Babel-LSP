# Chapter 39: Assertion API

## 39. Assertion API
### 39.1 General
This clause describes the following:
— SystemVerilog assertion API
— Obtaining assertion handles
— Assertions system callbacks
— Assertion control API functions
### 39.2 Overview
SystemVerilog provides assertion capabilities to enable the following:
— A user’s C code to react to assertion events
— Third-party assertion “waveform” dumping tools to be written
— Third-party assertion coverage tools to be written
— Third-party assertion debug tools to be written
### 39.3 Static information
This subclause defines how to obtain assertion handles and other static assertion information.
#### 39.3.1 Obtaining assertion handles
SystemVerilog extends the VPI navigation model to encompass assertions, properties, and sequences. It also
enhances the instance iterator model with direct access to assertions, properties, and sequences.
The following steps highlight how to obtain the assertion handles for named assertions through direct
access:
a) Iterate all assertions in the design: use a NULL reference handle (ref) to vpi_iterate(). For example:
itr = vpi_iterate(vpiAssertion, NULL);
while (assertion = vpi_scan(itr)) {
/* process assertion */
}
b) Iterate all assertions in an instance: pass the appropriate instance handle as a reference handle to
vpi_iterate(). For example:
itr = vpi_iterate(vpiAssertion, instanceHandle);
while (assertion = vpi_scan(itr)) {
/* process assertion */
}
c) Obtain the assertion by name: extend vpi_handle_by_name() to also search for assertion names in
the appropriate scope(s). For example:
vpiHandle = vpi_handle_by_name(assertName, scope)

<!-- Page 1149 -->

IEEE Std d) To obtain an assertion of a specific type, e.g., concurrent cover property statements, the
following approach should be used:
vpiHandle assertion;
itr = vpi_iterate(vpiAssertion, NULL);
while (assertion = vpi_scan(itr)) {
if (vpi_get(vpiType, assertion) == vpiCover) {
/* process cover type assertion */
}
}
Details:
— As with all VPI handles, assertion handles are handles to a specific instance of a specific assertion.
— Unnamed assertions cannot be found by name.
#### 39.3.2 Obtaining static assertion information
The following information about an assertion is considered to be static:
— Assertion name
— Instance in which the assertion occurs
— Module definition containing the assertion
— Assertion type
• Sequence instance
• Assert
• Assume
• Cover
• Restrict
• Property instance
• Immediate assert
• Immediate assume
• Immediate cover
— Assertion source information: the file, line, and column where the assertion is defined
— Assertion clocking block/expression
### 39.4 Dynamic information
This subclause defines how to place assertion system and assertion callbacks.
#### 39.4.1 Placing assertion system callbacks
To place an assertion system callback, use vpi_register_cb(), setting the cb_rtn element to the function to
be invoked and the reason element of the s_cb_data structure to one of the following values:
— cbAssertionSysInitialized. This callback occurs after the system has initialized. No
assertion-specific actions can be performed until this callback completes. The assertion system can
initialize before cbStartOfSimulation does or afterwards.
— cbAssertionSysLock. This callback occurs when the assertion system is locked, e.g., due to a
system control action.
— cbAssertionSysUnlock. This callback occurs when the assertion system is unlocked, e.g., due to a
system control action.

<!-- Page 1150 -->

IEEE Std — cbAssertionSysOn. The assertion system has become active and starts processing assertion
attempts. This always occurs after cbAssertionSysInitialized. By default, the assertion system is
“started” on simulation startup, but the user can delay this by using assertion system control actions.
— cbAssertionSysOff. The assertion system has been temporarily suspended. While stopped, no new
assertion attempts are processed and no new assertion-related callbacks occur. Assertions already
executing are not affected. The assertion system can be stopped and resumed an arbitrary number of
times during a single simulation run.
— cbAssertionSysKill. The assertion system has been temporarily suspended. While suspended, no
assertion attempts are processed, and no assertion-related callbacks occur. The assertion system can
be suspended and resumed an arbitrary number of times during a single simulation run.
— cbAssertionSysEnd. This callback occurs when all assertions have completed and no new attempts
shall start. Once this callback occurs, no more assertion-related callbacks shall occur, and assertion-
related actions shall have no further effect. This typically occurs after the end of simulation.
— cbAssertionSysReset. This callback occurs when the assertion system is reset, e.g., due to a system
control action.
— cbAssertionSysEnablePassAction. The pass action is enabled for vacuous and nonvacuous success
for the assertion (e.g., as a result of a system control action).
— cbAssertionSysEnableFailAction. The fail action is enabled for vacuous and nonvacuous success
for the assertion (e.g., as a result of a system control action).
— cbAssertionSysDisablePassAction. The pass action is disabled for vacuous and nonvacuous
success for the assertion (e.g., as a result of a system control action).
— cbAssertionSysDisableFailAction. The fail action is disabled for vacuous and nonvacuous success
for the assertion (e.g., as a result of a system control action).
— cbAssertionSysEnableNonvacuousAction. The pass action is enabled for nonvacuous success of
the assertion (e.g., as a result of a system control action).
— cbAssertionSysDisableVacuousAction. The pass action is disabled for vacuous success of the
assertion (e.g., as a result of a system control action).
The callback routine invoked follows the normal VPI callback prototype and is passed an s_cb_data
containing the callback reason and any user data provided to the vpi_register_cb() call.
#### 39.4.2 Placing assertions callbacks
To place an assertion callback, use vpi_register_assertion_cb(). The prototype is as follows:
/* typedef for vpi_register_assertion_cb callback function */
typedef PLI_INT32 (vpi_assertion_callback_func)(
PLI_INT32 reason, /* callback reason */
p_vpi_time cb_time, /* callback time */
vpiHandle assertion, /* handle to assertion */
p_vpi_attempt_info info, /* attempt related information */
PLI_BYTE8 *user_data /* user data entered upon registration */
);
vpiHandle vpi_register_assertion_cb(
vpiHandle assertion, /* handle to assertion */
PLI_INT32 reason, /* reason for which callbacks needed */
vpi_assertion_callback_func *cb_rtn,
PLI_BYTE8 *user_data /* user data to be supplied to cb */
);
typedef struct t_vpi_assertion_step_info {
PLI_INT32 matched_expression_count;

<!-- Page 1151 -->

IEEE Std vpiHandle *matched_exprs; /* array of expressions */
PLI_INT32 stateFrom, stateTo;/* identify transition */
} s_vpi_assertion_step_info, *p_vpi_assertion_step_info;
typedef struct t_vpi_attempt_info {
union {
vpiHandle failExpr;
p_vpi_assertion_step_info step;
} detail;
s_vpi_time attemptStartTime; /* Time attempt triggered */
} s_vpi_attempt_info, *p_vpi_attempt_info;
where reason is any of the following.
— cbAssertionStart. An assertion attempt has started. For most assertions, one attempt starts each and
every clock tick. For property and sequence instances the start is the start of evaluation of the
property or sequence instance. A property or sequence instance that is not instantiated in a
verification statement will never start.
— cbAssertionSuccess. An assertion attempt or property instance reaches a success state. For
sequence instances, success is a match.
— cbAssertionVacuousSuccess. An assertion attempt reaches a vacuous success state.
— cbAssertionDisabledEvaluation. An assertion attempt reaches the disabled state (e.g., as a result of
disable iff condition becoming true or if an attempt starts when the disable iff is true).
— cbAssertionFailure. An assertion attempt or a property fails to reach a success state or a sequence
instance fails to match.
— cbAssertionStepSuccess. Progress one step along an attempt. A step is defined as progress along
the flattened assertion (e.g., rewriting algorithm defined in F.4.1). By default, step callbacks are not
enabled on any assertions; they are enabled on a per-assertion/per-attempt basis (see 39.5.2), rather
than on a per-assertion basis.
— cbAssertionStepFailure. Fail to progress by one step along an attempt. A step is defined as
progress along the flattened assertion (e.g., rewriting algorithm defined in F.4.1). By default, step
callbacks are not enabled on any assertions; they are enabled on a per-assertion/per-attempt basis
(see 39.5.2), rather than on a per-assertion basis.
— cbAssertionLock. The assertion is locked (e.g., as a result of a control action, see 39.5.2).
— cbAssertionUnlock. The assertion is unlocked (e.g., as a result of a control action, see 39.5.2).
— cbAssertionDisable. The assertion is disabled (e.g., as a result of a control action, see 39.5.2).
— cbAssertionEnable. The assertion is enabled (e.g., as a result of a control action, see 39.5.2).
— cbAssertionReset. The assertion is reset (e.g., as a result of a control action, see 39.5.2).
— cbAssertionKill. An attempt is killed (e.g., as a result of a control action, see 39.5.2).
— cbAssertionDisablePassAction. The pass action is disabled for vacuous and nonvacuous success
for the assertion (e.g., as a result of control action, see 39.5.2).
— cbAssertionEnablePassAction. The pass action is enabled for vacuous and nonvacuous success for
the assertion (e.g., as a result of control action, see 39.5.2).
— cbAssertionDisableFailAction. The fail action is disabled for the assertion (e.g., as a result of
control action, see 39.5.2).
— cbAssertionDisableVacuousAction. The pass action is disabled for vacuous success of the
assertion (e.g., as a result of control action, see 39.5.2).
— cbAssertionEnableNonvacuousAction. The pass action is enabled for nonvacuous success of the
assertion (e.g., as a result of control action, see 39.5.2).
— cbAssertionEnableFailAction. The fail action is enabled for the assertion (e.g., as a result of
control action, see 39.5.2).

<!-- Page 1152 -->

IEEE Std Each of these callbacks may be registered on any concurrent or immediate assertion. The cbAssertionStart,
cbAssertionSuccess, and cbAssertionFailure callbacks may also be registered on a sequence instance or a
property instance.
These callbacks are specific to a given assertion; placing such a callback on one assertion does not cause the
callback to trigger on an event occurring on a different assertion. If the callback is successfully placed, a
handle to the callback is returned. This handle can be used to remove the callback via vpi_remove_cb(). If
there were errors on placing the callback, a NULL handle is returned.
Once the callback is placed, the user-supplied function shall be called each time the specified event occurs
on the given assertion. The callback shall continue to be called whenever the event occurs until the callback
is removed.
The callback function shall be supplied the following arguments:
— The reason for the callback
— A pointer to the time of the callback
— The handle for the assertion
— A pointer to an attempt information structure
— A reference to the user data supplied when the callback was registered
The t_vpi_attempt_info attempt information structure contains details relevant to the specific event that
occurred.
— On lock, unlock, disable, enable, reset, kill, pass action, fail action, vacuous action, and nonvacuous
action callbacks, the returned p_vpi_attempt_info info pointer is NULL, and no attempt
information is available.
— On start and success callbacks, only the attemptStartTime field is valid.
— On a cbAssertionFailure callback, the attemptStartTime and detail.failExpr fields are valid.
— On a step callback, the attemptStartTime and detail.step fields are valid.
On a step callback, the detail describes the set of expressions matched in satisfying a step along the
assertion, along with the corresponding source references. In addition, the step also identifies the source
and destination “states” needed to uniquely identify the path being taken through the assertion. State ids are
just integers, with 0 identifying the origin state, 1 identifying an accepting state, and any other number
representing some intermediate point in the assertion. It is possible for the number of expressions in a step to
be 0, which represents an unconditional transition. In the case of a failing transition, the information
provided is just as that for a successful one, but the last expression in the array represents the expression
where the transition failed.
Details:
a) In a failing transition, there shall always be at least one element in the expression array.
b) Placing a step callback results in the same callback function being invoked for both success and
failure steps.
c) The content of the cb_time field depends on the reason identified by the reason field, as follows:
— cbAssertionStart: cb_time is the time when the assertion attempt has been started.
— cbAssertionSuccess, cbAssertionFailure: cb_time is the time when the assertion succeeded
nonvacuously or failed.
— cbAssertionVacuousSuccess: cb_time is the time when the assertion succeeded vacuously.
— cbAssertionDisabledEvaluation: cb_time is the time when the assertion reached the disabled
state.

<!-- Page 1153 -->

IEEE Std — cbAssertionStepSuccess, cbAssertionStepFailure: cb_time is the time when the assertion
attempt step succeeded or failed.
— cbAssertionDisable, cbAssertionEnable, cbAssertionReset, cbAssertionKill: cb_time is
the time when the assertion attempt was disabled, enabled, reset, or killed.
d) In contrast to cb_time, the content of attemptStartTime is always the start time of the actual attempt
of an assertion. It can be used as a unique identifier that distinguishes the attempts of any given
assertion.
e) See 39.4.2.1 for callbacks for assertions containing global clocking future sampled value functions.
39.4.2.1 Placing callbacks for assertions with global clocking future sampled value
functions
Callback execution for assertions referring to global clocking future sampled value functions (see 16.9.4)
has the following peculiarities:
— The callback is executed at the nearest tick of the global clock strictly following the callback event.
— cb_time contains the time of the callback event.
For example:
a1: assert property(@(posedge clk) $falling_gclk(a) |=> b);
a2: assert property(@(posedge clk) a |=> $falling_gclk(b));
For both assertions a1 and a2 the callback executes at time 12, and not at time 11 when $assertkill
directive was issued (see Figure39-1). cb_time has the time value of 11—the time when the callback event
actually happened—and attemptStartTime has the time value of 10.
10 1112
$global_clock
clk
a
b
$assertkill
Figure39-1—Assertions with global clocking future sampled value functions
### 39.5 Control functions
This subclause defines how to obtain assertion system control and assertion control information.
#### 39.5.1 Assertion system control
To control the assertion system, use vpi_control() with one of the following constants and a second handle
argument that is a vpiHandle for a scope. A NULL handle signifies that the control applies to all assertions
regardless of scope.

<!-- Page 1154 -->

IEEE Std — Usage example: vpi_control(vpiAssertionSysReset, handle)
• vpiAssertionSysReset discards all attempts in progress for all assertions and restores the entire
assertion system to its initial state. Any preexisting vpiAssertionStepSuccess and
vpiAssertionStepFailure callbacks shall be removed; all other assertion callbacks shall
remain.
— Usage example: vpi_control(vpiAssertionSysOff, handle)
• vpiAssertionSysOff disables any further assertions from being started. Assertions already
executing are not affected. This control has no effect on preexisting assertion callbacks.
— Usage example: vpi_control(vpiAssertionSysKill, handle)
• vpiAssertionSysKill discards all attempts in progress and disables any further assertions from
being started. This control has no effect on preexisting assertion callbacks.
— Usage example: vpi_control(vpiAssertionSysLock, handle)
• vpiAssertionSysLock locks the assertions from being changed. The status of the assertions can
not be changed without unlocking it.
— Usage example: vpi_control(vpiAssertionSysUnlock, handle)
• vpiAssertionSysUnlock unlocks the assertions. Now, the status of the assertion can be changed.
— Usage example: vpi_control(vpiAssertionSysOn, handle)
• vpiAssertionSysOn restarts the assertion system after it was stopped or suspended (e.g., due to
vpiAssertionSysOff or vpiAssertionSysKill). Once started, attempts shall resume on all
assertions. This control has no effect on prior assertion callbacks.
— Usage example: vpi_control(vpiAssertionSysEnd, handle)
• vpiAssertionSysEnd discards all attempts in progress and disables any further assertions from
starting. All assertion callbacks currently installed shall be removed. Once this control is issued,
no further assertion-related actions shall be permitted.
— Usage example: vpi_control(vpiAssertionSysDisablePassAction, handle)
• vpiAssertionSysDisablePassAction disables execution of pass action for vacuous and
nonvacuous success of assertions. This has no effect on any existing attempts or if the assertion
pass action is already disabled. By default, all assertion pass actions are enabled.
— Usage example: vpi_control(vpiAssertionSysEnablePassAction, handle)
• vpiAssertionSysEnablePassAction enables execution of pass action for vacuous and
nonvacuous success of assertions. This has no effect on any existing attempts or if the assertion
pass action is already enabled.
— Usage example: vpi_control(vpiAssertionSysDisableFailAction, handle)
• vpiAssertionSysDisableFailAction disables execution of fail action for assertions. This has no
effect on any existing attempts or if the assertion fail action is already disabled. By default, all
fail actions are enabled.
— Usage example: vpi_control(vpiAssertionSysEnableFailAction, handle)
• vpiAssertionSysEnableFailAction enables execution of fail action for assertions. This has no
effect on any existing attempts or if the assertion fail action is already enabled.
— Usage example: vpi_control(vpiAssertionSysDisableVacuousAction, handle)
• vpiAssertionSysDisableVacuousAction disables execution of pass action on vacuous success
of assertions. This has no effect on any existing attempts or if the execution of pass action on
vacuous success is already disabled. By default, all vacuous actions are enabled.
— Usage example: vpi_control(vpiAssertionSysEnableNonvacuousAction, handle)
• vpiAssertionSysEnableNonvacuousAction enables execution of pass action on nonvacuous
success of assertions. This has no effect on any existing attempts or if the pass action for
nonvacuous success is already enabled.

<!-- Page 1155 -->

IEEE Std #### 39.5.2 Assertion control
To obtain assertion control information for assertion statements (e.g., assume, assert, cover), use
vpi_control() with one of the operators in this subclause. Only assertion statement handles are valid here,
not sequence or property instances.
For the following operators, the second argument shall be a valid assertion handle:
— Usage example: vpi_control(vpiAssertionReset, assertionHandle)
• vpiAssertionReset discards all current attempts in progress for this assertion and resets this
assertion to its initial state.
— Usage example: vpi_control(vpiAssertionLock, assertionHandle)
• vpiAssertionLock locks the status of the assertion from being changed. The status of the
assertion cannot be changed without unlocking it.
— Usage example: vpi_control(vpiAssertionUnlock, assertionHandle)
• vpiAssertionUnlock unlocks the assertion. Now the status of the assertion can be changed.
— Usage example: vpi_control(vpiAssertionDisable, assertionHandle)
• vpiAssertionDisable disables the starting of any new attempts for this assertion. This has no
effect on any existing attempts or if the assertion is already disabled. By default, all assertions are
enabled.
— Usage example: vpi_control(vpiAssertionEnable, assertionHandle)
• vpiAssertionEnable enables starting new attempts for this assertion. This has no effect on any
existing attempts or if the assertion is already enabled.
— Usage example: vpi_control(vpiAssertionDisablePassAction, assertionHandle)
• vpiAssertionDisablePassAction disables execution of pass action for vacuous and nonvacuous
success of this assertion. This has no effect on any existing attempts or if the assertion pass action
is already disabled. By default, all pass actions are enabled.
— Usage example: vpi_control(vpiAssertionEnablePassAction, assertionHandle)
• vpiAssertionEnablePassAction enables execution of pass action for vacuous and nonvacuous
success of this assertion. This has no effect on any existing attempts or if the assertion pass action
is already enabled.
— Usage example: vpi_control(vpiAssertionDisableFailAction, assertionHandle)
• vpiAssertionDisableFailAction disables execution of fail action for this assertion. This has no
effect on any existing attempts or if the assertion fail action is already disabled. By default, all
fail actions are enabled.
— Usage example: vpi_control(vpiAssertionEnableFailAction, assertionHandle)
• vpiAssertionEnableFailAction enables execution of fail action for this assertion. This has no
effect on any existing attempts or if the assertion fail action is already enabled.
— Usage example: vpi_control( vpiAssertionDisableVacuousAction,
assertionHandle)
• vpiAssertionDisableVacuousAction disables execution of pass action on vacuous success of
this assertion. This has no effect on any existing attempts or if the execution of pass action on
vacuous success is already disabled. By default, all vacuous actions are enabled.
— Usage example: vpi_control(vpiAssertionEnableNonvacuousAction,
assertionHandle)
• vpiAssertionEnableNonvacuousAction enables execution of pass action on nonvacuous
success of this assertion. This has no effect on any existing attempts or if the pass action is
already enabled for nonvacuous success of this assertion.

<!-- Page 1156 -->

IEEE Std For the following operators, the second argument shall be a valid assertion handle, and the third argument
shall be an attempt start time (as a pointer to a correctly initialized s_vpi_time structure):
— Usage example:
vpi_control(vpiAssertionKill, assertionHandle, attemptStartTime)
• vpiAssertionKill discards the given attempt, but leaves the assertion enabled and does not reset
any state used by this assertion (e.g., past() sampling).
— Usage example:
vpi_control(vpiAssertionDisableStep, assertionHandle, attemptStartTime)
• vpiAssertionDisableStep disables step callbacks for this assertion. This has no effect if stepping
is not enabled or it is already disabled.
For the following operator, the second argument shall be a valid assertion handle, the third argument shall be
an attempt start time (as a pointer to a correctly initialized s_vpi_time structure), and the fourth argument
shall be a step control constant:
— Usage example:
vpi_control(vpiAssertionEnableStep, assertionHandle, attemptStartTime,
vpiAssertionClockSteps)
• vpiAssertionEnableStep enables step callbacks to occur for this assertion attempt. By default,
stepping is disabled for all assertions. This call has no effect if stepping is already enabled for
this assertion and attempt, other than possibly changing the stepping mode for the attempt if the
attempt has not occurred yet. The stepping mode of any particular attempt cannot be modified
after the assertion attempt in question has started.
• The fine-grained step control constant vpiAssertionClockSteps indicates callbacks on a
per-assertion/clock-tick basis. The assertion clock is the event expression supplied as the
clocking expression to the assertion declaration. This step callback shall occur at every clocking
event, when stepping is enabled, as the assertion “advances” in evaluation.
#### 39.5.3 VPI functions on deferred assertions and procedural concurrent assertions
Deferred assertions (see 16.4) may be in a pending state where the assertion has executed but been placed in
a deferred assertion report queue. Similarly, procedural concurrent assertions (see 16.14.6) may have
pending instances in a procedural assertion queue waiting to mature. For any VPI function, if it discards
current evaluation attempts in progress, that also means it flushes any pending instances that have not yet
matured from these queues. If a VPI function does not interfere with current attempts, that also means it does
not affect or flush these queues.
For example, since vpiAssertionReset discards all current evaluation attempts in progress for the targeted
assertion, if applied to a deferred assertion, it flushes any pending reports for that assertion. However,
vpiAssertionDisable disables the starting of any new attempts without affecting existing attempts, so any
pending reports from a disabled deferred assertion that are already queued may still mature and be reported.

<!-- Page 1157 -->

IEEE Std 1800-2023