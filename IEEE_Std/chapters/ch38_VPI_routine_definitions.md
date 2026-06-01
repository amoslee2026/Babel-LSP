# Chapter 38: VPI routine definitions

## 38. VPI routine definitions
### 38.1 General
This clause describes the VPI routines and explains their function, syntax, and usage. The routines are listed
in alphabetical order.
The following conventions are used in the definitions of the PLI routines described in this clause:
— Synopsis: A brief description of the PLI routine functionality, intended to be used as a quick
reference when searching for PLI routines to perform specific tasks.
— Syntax: The exact name of the PLI routine and the order of the arguments passed to the routine.
— Returns: The definition of the value returned when the PLI routine is called, along with a brief
description of what the value represents. The return definition contains the following fields:
• Type: The data type of the C value that is returned. The data type is either a standard ANSI C
type or a special type defined within the PLI.
• Description: A brief description of what the value represents.
— Arguments: The definition of the arguments passed with a call to the PLI routine. The argument
definition contains the following fields:
• Type: The data type of the C values that are passed as arguments. The data type is either a
standard ANSI C type or a special type defined within the PLI.
• Name: The name of the argument used in the syntax definition.
• Description: A brief description of what the value represents.
All arguments shall be considered mandatory unless specifically noted in the definition of the PLI
routine.
— Related routines: A list of PLI routines that are typically used with, or provide similar functionality
to, the PLI routine being defined. This list is provided as a convenience to facilitate finding
information in this standard. It is not intended to be all-inclusive, and it does not imply that the
related routines have to be used.
### 38.2 vpi_chk_error()
vpi_chk_error()
Synopsis: Retrieve information about VPI routine errors.
Syntax: vpi_chk_error(error_info_p)
Type Description
Returns: PLI_INT32 The error severity level if the previous VPI routine call resulted in an error; 0 (false)
if no error occurred.
Type Name Description
Arguments: p_vpi_error_info error_info_p Pointer to a structure containing error information.
Related
routines:
The VPI routine vpi_chk_error() shall return an integer constant representing an error severity level if the
previous call to a VPI routine resulted in an error. The error constants are shown in Table38-1. If the
previous call to a VPI routine did not result in an error, then vpi_chk_error() shall return 0 (false). The error

<!-- Page 1090 -->

IEEE Std status shall be reset by any VPI routine call except vpi_chk_error(). Calling vpi_chk_error() shall have no
effect on the error status.
Table38-1—Return error constants for vpi_chk_error()
Error constant Severity level
vpiNotice Lowest severity
vpiWarning
vpiError
vpiSystem
vpiInternal Highest severity
If an error occurred, the s_vpi_error_info structure shall contain information about the error. If the error
information is not needed, a NULL can be passed to the routine. The s_vpi_error_info structure used by
vpi_chk_error() is defined in vpi_user.h and is listed in Figure38-1.
typedef struct t_vpi_error_info
{
PLI_INT32 state; /* vpi[Compile,PLI,Run] */
PLI_INT32 level; /* vpi[Notice,Warning,Error,System,Internal] */
PLI_BYTE8 *message;
PLI_BYTE8 *product;
PLI_BYTE8 *code;
PLI_BYTE8 *file;
PLI_INT32 line;
} s_vpi_error_info, *p_vpi_error_info;
Figure38-1—s_vpi_error_info structure definition
### 38.3 vpi_compare_objects()
vpi_compare_objects()
Synopsis: Compare two handles to determine whether they reference the same object.
Syntax: vpi_compare_objects(obj1, obj2)
Type Description
Returns: PLI_INT32 1 (true) if the two handles refer to the same object; 0 (false) otherwise.
Type Name Description
Arguments: vpiHandle obj1 Handle to an object.
vpiHandle obj2 Handle to an object.
Related
routines:
The VPI routine vpi_compare_objects() shall return 1 (TRUE) if the two handles refer to the same
underlying simulation object at the time the function is called, provided that the simulation object exists.

<!-- Page 1091 -->

IEEE Std Otherwise, 0 (FALSE) shall be returned. Object equivalence cannot be determined with a C “==”
comparison.
The following examples illustrate the use of vpi_compare_objects().
Example 1:
struct packed {
int a;
reg [0:7] b;
} ps;
...
initial begin
ps[0] = ...;
ps.b[7] = ...;
end
The expression ps[0] is another way of referring to bit 7 of ps.b, so if a handle obj1 refers to ps[0] and
a handle obj2 refers to ps.b[7], then vpi_compare_objects(obj1, obj2) shall return TRUE.
Example 2:
integer i [0:3];
int j;
...
initial begin
j = 0;
i[j] = ...;
#(1)
j = 1;
i[j] = ...;
end
Let obj1 be a handle to an occurrence of the expression i[j], and let obj2 be a handle to the object i[0]
derived by iteration from the integer array i. Then
vpi_compare_objects(obj1, obj2)
shall return TRUE when j has the value 0 and FALSE when j has the value 1.
Example 3:
class MyClass;
int a;
endclass
...
MyClass c, d;
...
initial begin
c = null;
d = null;
#(1)
c = new;
c.a = 5;
#(1)
d = c;
d.a = 6;

<!-- Page 1092 -->

IEEE Std #(1)
c = new;
c.a = 7;
end
If obj1 represents the expression c.a, while obj2 represents d.a, then initially neither object exists, and
vpi_compare_objects(obj1, obj2) shall return FALSE. After one time step, c.a exists, but d.a does
not, and vpi_compare_objects(obj1, obj2) shall still return FALSE. After the second time step, c.a
and d.a point to the same int data member of the same class object, and vpi_compare_objects(obj1,
obj2) shall return TRUE. Finally, c gets a new class object assigned to it, but d does not, and
vpi_compare_objects(obj1, obj2) shall once again return FALSE.
### 38.4 vpi_control()
vpi_control()
Synopsis: Pass information from the application code to the simulator.
Syntax: vpi_control(operation, varargs)
Type Description
Returns: PLI_INT32 1 (true) if successful; 0 (false) on a failure.
Type Name Description
Arguments: PLI_INT32 operation Select type of operation.
varargs Variable number of operation-specific arguments.
Related
routines:
The VPI routine vpi_control() shall pass information from a user PLI application to a SystemVerilog
software tool, such as a simulator. The following control constants are defined as part of the VPI standard:
vpiStop Causes the $stop built-in SystemVerilog system task to be executed upon return
of the application routine. This operation shall be passed one additional integer
argument, which is the same as the diagnostic message level argument passed to
$stop (see 20.2).
vpiFinish Causes the $finish built-in SystemVerilog system task to be executed upon
return of the application routine. This operation shall be passed one additional
integer argument, which is the same as the diagnostic message level argument
passed to $finish (see 20.2).
vpiReset Causes the $reset built-in SystemVerilog system task to be executed upon
return of the application routine. This operation shall be passed three additional
integer arguments: stop_value, reset_value, and diagnostics_value,
which are the same values passed to the $reset system task (see D.8).
vpiSetInteractiveScope
Causes a tool’s interactive scope to be immediately changed to a new scope. This
operation shall be passed one additional argument, which is a vpiHandle object
within the vpiScope class.

<!-- Page 1093 -->

IEEE Std ### 38.5 vpi_flush()
vpi_flush()
Synopsis: Flushes the data from the simulator output channel and log file output buffers.
Syntax: vpi_flush()
Type Description
Returns: PLI_INT32 0 if successful; nonzero if unsuccessful.
Type Name Description
Arguments: None
Related Use vpi_printf() to write a finite number of arguments to the simulator output channel and log file.
routines: Use vpi_vprintf() to write a variable number of arguments to the simulator output channel and log file.
Use vpi_mcd_printf() to write one or more opened files.
The routine vpi_flush() shall flush the output buffers for the simulator’s output channel and current log file.
### 38.6 vpi_get()
vpi_get()
Synopsis: Get the value of an integer or Boolean property of an object.
Syntax: vpi_get(prop, obj)
Type Description
Returns: PLI_INT32 Value of an integer or Boolean property.
Type Name Description
Arguments: PLI_INT32 prop An integer constant representing the property of an
object for which to obtain a value.
vpiHandle obj Handle to an object.
Related Use vpi_get_str() to get string properties.
routines: Use vpi_get64() to get 64-bit integer properties.
The VPI routine vpi_get() shall return the value of integer and Boolean object properties. These properties
shall be of type PLI_INT32. Boolean properties shall have a value of 1 for TRUE and 0 for FALSE. For
integer object properties such as vpiSize, any integer shall be returned. For integer object properties that
return a defined value, see AnnexK and AnnexM for the value that shall be returned. For object property
vpiTimeUnit or vpiTimePrecision, if the object is NULL, then the simulation time unit shall be returned.
Unless otherwise specified, calling vpi_get() for a protected object shall be an error. Should an error occur,
vpi_get() shall return vpiUndefined.

<!-- Page 1094 -->

IEEE Std ### 38.7 vpi_get64()
vpi_get64()
Synopsis: Get the value of a 64-bit integer property of an object.
Syntax: vpi_get64(prop, obj)
Type Description
Returns: PLI_INT64 Value of a 64-bit integer property.
Type Name Description
Arguments: PLI_INT32 prop An integer constant representing the property of an
object for which to obtain a value.
vpiHandle obj Handle to an object.
Related Use vpi_get_str() to get string properties.
routines: Use vpi_get() to get integer or Boolean properties.
The VPI routine vpi_get64() shall return the value of 64-bit integer object properties. These properties shall
be of type PLI_INT64. For 64-bit integer object properties that return a defined value, see AnnexK and
AnnexM for the value that shall be returned. Unless otherwise specified, calling vpi_get64() for a protected
object shall be an error. Should an error occur, vpi_get64() shall return vpiUndefined.
### 38.8 vpi_get_cb_info()
vpi_get_cb_info()
Synopsis: Retrieve information about a simulation-related callback.
Syntax: vpi_get_cb_info(obj, cb_data_p)
Type Description
Returns: void
Type Name Description
Arguments: vpiHandle obj Handle to a simulation-related callback.
p_cb_data cb_data_p Pointer to a structure containing callback information.
Related Use vpi_get_systf_info() to retrieve information about a system task or system function callback.
routines:
The VPI routine vpi_get_cb_info() shall return information about a simulation-related callback in an
s_cb_data structure. The memory for this structure shall be allocated by the application.
The s_cb_data structure used by vpi_get_cb_info() is defined in vpi_user.h and is listed in Figure38-2.

<!-- Page 1095 -->

IEEE Std typedef struct t_cb_data
{
PLI_INT32 reason; /* callback reason */
PLI_INT32 (*cb_rtn)(struct t_cb_data *); /* call routine */
vpiHandle obj; /* trigger object */
p_vpi_time time; /* callback time */
p_vpi_value value; /* trigger object value */
PLI_INT32 index; /* index of the memory word or var select
that changed */
PLI_BYTE8 *user_data;
} s_cb_data, *p_cb_data;
Figure38-2—s_cb_data structure definition
### 38.9 vpi_get_data()
vpi_get_data()
Synopsis: Get data from an implementation’s save/restart location.
Syntax: vpi_get_data(id, dataLoc, numOfBytes)
Type Description
Returns: PLI_INT32 The number of bytes retrieved.
Type Name Description
Arguments: PLI_INT32 id A save/restart ID returned from
vpi_get(vpiSaveRestartID, NULL) .
PLI_BYTE8 * dataLoc Address of application-allocated storage.
PLI_INT32 numOfBytes Number of bytes to be retrieved from save/restart
location.
Related Use vpi_put_data() to write saved data.
routines:
The routine shall place numOfBytes of data into the memory location pointed to by dataLoc from a
simulation’s save/restart location. This memory location has to be properly allocated by the application. The
first call for a given id will retrieve the data starting at what was placed into the save/restart location with the
first call to vpi_put_data() for a given id. The return value shall be the number of bytes retrieved. On a
failure, the return value shall be 0. Each subsequent call shall start retrieving data where the last call left off.
It shall be a warning for an application to retrieve more data than were placed into the simulation save/restart
location for a given id. In this case, the dataLoc shall be filled with the data that are left for the given id, and
the remaining bytes shall be filled with “\0”. The return value shall be the actual number of bytes retrieved.
It shall be acceptable for an application to retrieve less data than were stored for a given id with
vpi_put_data(). This routine can only be called from an application routine that has been called for reason
cbStartOfRestart or cbEndOfRestart. The recommended way to get the “id” for vpi_get_data() is to pass
it as the value for user_data when registering for cbStartOfRestart or cbEndOfRestart from the
cbStartOfSave or cbEndOfSave application routine. An application can get the path to the simulation’s
save/restart location by calling vpi_get_str(vpiSaveRestartLocation, NULL) from an application routine
that has been called for reason cbStartOfRestart or cbEndOfRestart.
For an example of vpi_get_data(), see 38.31.

<!-- Page 1096 -->

IEEE Std ### 38.10 vpi_get_delays()
vpi_get_delays()
Synopsis: Retrieve the delays or pulse limits of an object.
Syntax: vpi_get_delays(obj, delay_p)
Type Description
Returns: void
Type Name Description
Arguments: vpiHandle obj Handle to an object.
p_vpi_delay delay_p Pointer to a structure containing delay information.
Related Use vpi_put_delays() to set the delays or timing limits of an object.
routines:
The VPI routine vpi_get_delays() shall retrieve the delays or pulse limits of an object and place them in an
s_vpi_delay structure that has been allocated by the application. The format of the delay information shall
be controlled by the time_type flag in the s_vpi_delay structure. This routine shall ignore the value of the
type flag in the s_vpi_time structure.
The s_vpi_delay and s_vpi_time structures used by both vpi_get_delays() and vpi_put_delays() are
defined in vpi_user.h and are listed in Figure38-3 and Figure38-4.
.
typedef struct t_vpi_delay
{
struct t_vpi_time *da; /* pointer to application-allocated
array of delay values */
PLI_INT32 no_of_delays; /* number of delays */
PLI_INT32 time_type; /* [vpiScaledRealTime, vpiSimTime,
vpiSuppressTime] */
PLI_INT32 mtm_flag; /* true for mtm values */
PLI_INT32 append_flag; /* true for append */
PLI_INT32 pulsere_flag; /* true for pulsere values */
} s_vpi_delay, *p_vpi_delay;
Figure38-3—s_vpi_delay structure definition
typedef struct t_vpi_time
{
PLI_INT32 type; /* [vpiScaledRealTime, vpiSimTime,
vpiSuppressTime] */
PLI_UINT32 high, low; /* for vpiSimTime */
double real; /* for vpiScaledRealTime */
} s_vpi_time, *p_vpi_time;
Figure38-4—s_vpi_time structure definition
The da field of the s_vpi_delay structure shall be an application-allocated array of s_vpi_time
structures. This array shall store delay values returned by vpi_get_delays(). The number of elements in this
array shall be determined by the following:

<!-- Page 1097 -->

IEEE Std — The number of delays to be retrieved
— The mtm_flag setting
— The pulsere_flag setting
The number of delays to be retrieved shall be set in the no_of_delays field of the s_vpi_delay structure.
Legal values for the number of delays shall be determined by the type of object, as follows:
— For primitive objects, the no_of_delays value shall be 2 or 3.
— For path delay objects, the no_of_delays value shall be 1, 2, 3, 6, or 12.
— For timing check objects, the no_of_delays value shall match the number of limits existing in the
timing check.
— For intermodule path objects, the no_of_delays value shall be 2 or 3.
The application-allocated s_vpi_delay array shall contain delays in the same order in which they occur in
the SystemVerilog description. The number of elements for each delay shall be determined by the flags
mtm_flag and pulsere_flag, as shown in Table38-2.
Table38-2—Size of the s_vpi_delay->da array
Number of
Order in which delay elements
Flag values s_vpi_time array elements
shall be filled
required for s_vpi_delay->da
mtm_flag = FALSE no_of_delays 1st delay: da[0] -> 1st delay
pulsere_flag = FALSE 2nd delay: da[1] -> 2nd delay
...
mtm_flag = TRUE 3 × no_of_delays 1st delay: da[0] -> min delay
pulsere_flag = FALSE da[1] -> typ delay
da[2] -> max delay
2nd delay: ...
mtm_flag = FALSE 3 × no_of_delays 1st delay: da[0] -> delay
pulsere_flag = TRUE da[1] -> reject limit
da[2] -> error limit
2nd delay element: ...
mtm_flag = TRUE 9 × no_of_delays 1st delay: da[0] -> min delay
pulsere_flag = TRUE da[1] -> typ delay
da[2] -> max delay
da[3] -> min reject
da[4] -> typ reject
da[5] -> max reject
da[6] -> min error
da[7] -> typ error
da[8] -> max error
2nd delay: ...
The delay structure has to be allocated before passing a pointer to vpi_get_delays(). In the following
example, a static structure, prim_da, is allocated for use by each call to the vpi_get_delays() function:
display_prim_delays(prim)
vpiHandle prim;
{
static s_vpi_time prim_da[3];
static s_vpi_delay delay_s = {NULL, 3, vpiScaledRealTime};
static p_vpi_delay delay_p = &delay_s;
delay_s.da = prim_da;
vpi_get_delays(prim, delay_p);
vpi_printf("Delays for primitive %s: %6.2f %6.2f %6.2f\n",

<!-- Page 1098 -->

IEEE Std vpi_get_str(vpiFullName, prim),
delay_p->da[0].real, delay_p->da[1].real, delay_p->da[2].real);
}
### 38.11 vpi_get_str()
vpi_get_str()
Synopsis: Get the value of a string property of an object.
Syntax: vpi_get_str(prop, obj)
Type Description
Returns: PLI_BYTE8 * Pointer to a character string containing the property value.
Type Name Description
Arguments: PLI_INT32 prop An integer constant representing the property of an object
for which to obtain a value.
vpiHandle obj Handle to an object.
Related Use vpi_get() to get integer and Boolean properties.
routines: Use vpi_get64() to get 64-bit integer properties.
The VPI routine vpi_get_str() shall return string property values. The string shall be placed in a temporary
buffer that shall be used by every call to this routine. If the string is to be used after a subsequent call, the
string should be copied to another location. A different string buffer shall be used for string values returned
through the s_vpi_value structure. Unless otherwise specified, calling vpi_get_str() for a protected object
shall be an error.
The following example illustrates the usage of vpi_get_str():
vpiHandle mod = vpi_handle_by_name("top.mod1",NULL);
vpi_printf ("Module top.mod1 is an instance of %s\n",
vpi_get_str(vpiDefName, mod));

<!-- Page 1099 -->

IEEE Std ### 38.12 vpi_get_systf_info()
vpi_get_systf_info()
Synopsis: Retrieve information about a user-defined system task or system function callback.
Syntax: vpi_get_systf_info(obj, systf_data_p)
Type Description
Returns: void
Type Name Description
Arguments: vpiHandle obj Handle to a system task or system function callback.
p_vpi_systf_data systf_data_p Pointer to a structure containing callback information.
Related Use vpi_get_cb_info() to retrieve information about a simulation-related callback.
routines:
The VPI routine vpi_get_systf_info() shall return information about a user-defined system task or system
function callback in an s_vpi_systf_data structure. The memory for this structure shall be allocated by
the application.
The s_vpi_systf_data structure used by vpi_get_systf_info() is defined in vpi_user.h and is listed in
Figure38-5.
typedef struct t_vpi_systf_data
{
PLI_INT32 type; /* vpiSysTask, vpiSysFunc */
PLI_INT32 sysfunctype; /* vpi[Int,Real,Time,Sized,SizedSigned]Func */
PLI_BYTE8 *tfname; /* first character has to be '$' */
PLI_INT32 (*calltf)(PLI_BYTE8 *);
PLI_INT32 (*compiletf)(PLI_BYTE8 *);
PLI_INT32 (*sizetf)(PLI_BYTE8 *); /* for sized function
callbacks only */
PLI_BYTE8 *user_data;
} s_vpi_systf_data, *p_vpi_systf_data;
Figure38-5—s_vpi_systf_data structure definition

<!-- Page 1100 -->

IEEE Std ### 38.13 vpi_get_time()
vpi_get_time()
Synopsis: Retrieve the current simulation time.
Syntax: vpi_get_time(obj, time_p)
Type Description
Returns: void
Type Name Description
Arguments: vpiHandle obj Handle to an object.
p_vpi_time time_p Pointer to a structure containing time information.
Related
routines:
The VPI routine vpi_get_time() shall retrieve the current simulation time, using the timescale of the object.
If obj is NULL, the simulation time is retrieved using the simulation time unit. If obj is a time queue object,
the scheduled time of the future event is retrieved using the simulation time unit. The time_p->type field
shall be set to indicate whether scaled real or simulation time is desired. The memory for the time_p
structure shall be allocated by the application.
The s_vpi_time structure used by vpi_get_time() is defined in vpi_user.h and is listed in Figure38-6
[this is the same time structure as used by vpi_put_value()].
typedef struct t_vpi_time
{
PLI_INT32 type; /* [vpiScaledRealTime, vpiSimTime,
vpiSuppressTime] */
PLI_UINT32 high, low; /* for vpiSimTime */
double real; /* for vpiScaledRealTime */
} s_vpi_time, *p_vpi_time;
Figure38-6—s_vpi_time structure definition

<!-- Page 1101 -->

IEEE Std ### 38.14 vpi_get_userdata()
vpi_get_userdata()
Synopsis: Get user-data value from an implementation’s system task or system function instance storage location.
Syntax: vpi_get_userdata(obj)
Type Description
Returns: void * User-data value associated with a system task instance or system function instance.
Type Name Description
Arguments: vpiHandle obj Handle to a system task instance or system function
instance.
Related Use vpi_put_userdata() to write data into the user-data storage area.
routines:
This routine shall return the value of the user data associated with a previous call to vpi_put_userdata() for
a user-defined system task or system function call handle. If no user data had been previously associated
with the object or if the routine fails, the return value shall be NULL.
After a restart or a reset, subsequent calls to vpi_get_userdata() shall return NULL. It is the application’s
responsibility to save the data during a save using vpi_put_data() and to then retrieve them using
vpi_get_data(). The user-data field can be set up again during or after callbacks of type cbEndOfRestart or
cbEndOfReset.
### 38.15 vpi_get_value()
vpi_get_value()
Synopsis: Retrieve the simulation value of an object.
Syntax: vpi_get_value(obj, value_p)
Type Description
Returns: void
Type Name Description
Arguments: vpiHandle obj Handle to an expression.
p_vpi_value value_p Pointer to a structure containing value information.
Related Use vpi_put_value() to set the value of an object.
routines:
The VPI routine vpi_get_value() shall retrieve the simulation value of VPI objects. The value shall be
placed in an s_vpi_value structure, which has been allocated by the application. The object shall be fully
evaluated as if simulated in the context in which it occurs in the SystemVerilog source, including all
expressions with side effects that occur as index expressions or as arguments to function calls embedded in
the object expression.

<!-- Page 1102 -->

IEEE Std For example, applying vpi_get_value() to the expression “i++” shall increment the value of i but shall
return the unincremented value. Similarly, retrieving the simulation value of “x[my_func(a)]” shall
evaluate my_func(a) in order to determine the value of the index expression.
The format of the value shall be set by the format field of the structure.
When the format field is vpiObjTypeVal, the routine shall fill in the value and change the format field
based on the object type, as follows:
— For an integer, vpiIntVal
— For a real, vpiRealVal
— For a scalar, either vpiScalar or vpiStrength
— For a time variable, vpiTimeVal with vpiSimTime
— For a vector, vpiVectorVal
The buffer this routine uses for string values shall be different from the buffer that vpi_get_str() shall use.
The string buffer used by vpi_get_value() is overwritten with each call. If the value is needed, it should be
saved by the application.
The s_vpi_value, s_vpi_vecval, and s_vpi_strengthval structures used by vpi_get_value() are
defined in vpi_user.h and are listed in Figure38-7, Figure38-8, and Figure38-9.
.
typedef struct t_vpi_value
{
PLI_INT32 format; /* vpi[[Bin,Oct,Dec,Hex]Str,Scalar,Int,Real,String,
Vector,Strength,Suppress,Time,ObjType]Val */
union
{
PLI_BYTE8 *str; /* string value */
PLI_INT32 scalar; /* vpi[0,1,X,Z] */
PLI_INT32 integer; /* integer value */
double real; /* real value */
struct t_vpi_time *time; /* time value */
struct t_vpi_vecval *vector; /* vector value */
struct t_vpi_strengthval *strength; /* strength value */
PLI_BYTE8 *misc; /* ...other */
} value;
} s_vpi_value, *p_vpi_value;
Figure38-7—s_vpi_value structure definition
typedef struct t_vpi_vecval
{
/* following fields are repeated enough times to contain vector */
PLI_UINT32 aval, bval; /* bit encoding: ab: 00=0, 10=1, 11=X, 01=Z */
} s_vpi_vecval, *p_vpi_vecval;
Figure38-8—s_vpi_vecval structure definition
typedef struct t_vpi_strengthval
{
PLI_INT32 logic; /* vpi[0,1,X,Z] */
PLI_INT32 s0, s1; /* refer to strength coding in Annex K */
} s_vpi_strengthval, *p_vpi_strengthval;
Figure38-9—s_vpi_strengthval structure definition

<!-- Page 1103 -->

IEEE Std For vectors, the p_vpi_vecval field shall point to an array of s_vpi_vecval structures. The size of this
array shall be determined by the size of the vector, where array_size = ((vector_size–1)/32 + 1). The LSB of
the vector shall be represented by the LSB of the 0-indexed element of s_vpi_vecval array. The 33rd bit
of the vector shall be represented by the LSB of the 1-indexed element of the array, and so on. The memory
for the union members str, time, vector, strength, and misc of the value union in the s_vpi_value structure
shall be provided by the routine vpi_get_value(). This memory shall only be valid until the next call to
vpi_get_value(). The application shall provide the memory for these members when calling
vpi_put_value(). When a value change callback occurs for a value type of vpiVectorVal, the system shall
create the associated memory (an array of s_vpi_vecval structures) and free the memory upon the return
of the callback.
Table38-3—Return value field of the s_vpi_value structure union
Format Union member Return description
vpiBinStrVal str String of binary character(s) [1, 0, x, z]
vpiOctStrVal str String of octal character(s) [0–7, x, X, z, Z]
x when all the bits are x
X when some of the bits are x
z when all the bits are z
Z when some of the bits are z
vpiDecStrVal str String of decimal character(s) [0–9]
vpiHexStrVal str String of hex character(s) [0–f, x, X, z, Z]
x when all the bits are x
X when some of the bits are x
z when all the bits are z
Z when some of the bits are z
vpiScalarVal scalar vpi1, vpi0, vpiX, vpiZ, vpiH, vpiL
vpiIntVal integer Integer value of the handle. Any bits x or z in the value
of the object are mapped to a 0
vpiRealVal real Value of the handle as a double
vpiStringVal str A string where each 8-bit group of the value of the
object is assumed to represent an ASCII character
vpiTimeVal time Integer value of the handle using two integers
vpiVectorVal vector aval/bval representation of the value of the object
vpiStrengthVal strength Value plus strength information
vpiObjTypeVal — Return a value in the closest format of the object
If the format field in the s_vpi_value structure is set to vpiStrengthVal, the value.strength pointer shall
point to an array of s_vpi_strengthval structures. This array shall have at least as many elements as
there are bits in the vector. If the object is a reg or variable, the strength will always be returned as strong.
If the logic value retrieved by vpi_get_value() needs to be preserved for later use, the application shall
allocate storage and copy the value. The following example can be used to copy a value that was retrieved
into an s_vpi_value structure into another structure allocated by the application:
/*
* Copy s_vpi_value structure - need to first allocate pointed-to fields.
* nvalp needs to be previously allocated.
* Need to first determine size for vector value.

<!-- Page 1104 -->

IEEE Std */
void copy_vpi_value(s_vpi_value *nvalp, s_vpi_value *ovalp,
PLI_INT32 blen, PLI_INT32 nd_alloc)
{
int i;
PLI_INT32 numvals;
nvalp->format = ovalp->format;
switch (nvalp->format) {
/* all string values */
case vpiBinStrVal: case vpiOctStrVal: case vpiDecStrVal:
case vpiHexStrVal: case vpiStringVal:
if (nd_alloc) nvalp->value.str = malloc(strlen(ovalp->value.str)+1);
strcpy(nvalp->value.str, ovalp->value.str);
break;
case vpiScalarVal:
nvalp->value.scalar = ovalp->value.scalar;
break;
case vpiIntVal:
nvalp->value.integer = ovalp->value.integer;
break;
case vpiRealVal:
nvalp->value.real = ovalp->value.real;
break;
case vpiVectorVal:
numvals = (blen + 31) >> 5;
if (nd_alloc)
{
nvalp->value.vector = (p_vpi_vecval)
malloc(numvals*sizeof(s_vpi_vecval));
}
/* t_vpi_vecval is really array of the 2 integer a/b sections */
/* memcpy or bcopy better here */
for (i = 0; i <numvals; i++)
nvalp->value.vector[i] = ovalp->value.vector[i];
break;
case vpiStrengthVal:
if (nd_alloc)
{
nvalp->value.strength = (p_vpi_strengthval)
malloc(sizeof(s_vpi_strengthval));
}
/* assumes C compiler supports struct assign */
*(nvalp->value.strength) = *(ovalp->value.strength);
break;
case vpiTimeVal:
nvalp->value.time = (p_vpi_time) malloc(sizeof(s_vpi_time));
/* assumes C compiler supports struct assign */
*(nvalp->value.time) = *(ovalp->value.time);
break;
/* not sure what to do here? */
case vpiObjTypeVal: case vpiSuppressVal:
vpi_printf(
"**ERR: cannot copy vpiObjTypeVal or vpiSuppressVal formats",
" - not for filled records.\n");
break;
}
}

<!-- Page 1105 -->

IEEE Std To get the ASCII values of UDP table entries (see Table29-1 in 29.3.6), the p_vpi_vecval field shall point to
an array of s_vpi_vecval structures. The size of this array shall be determined by the size of the table
entry (number of symbols per table entry), where array_size = ((table_entry_size–1)/4 + 1). Each symbol
shall require two bytes; the ordering of the symbols within s_vpi_vecval shall be the most significant byte
of abit first, then the least significant byte of abit, then the most significant byte of bbit, and then the least
significant byte of bbit. Each symbol can be either one or two characters; when it is a single character, the
second byte of the pair shall be an ASCII “\0”.
Real valued objects shall be converted to an integer using the rounding defined in 6.12.1 before being
returned in a format other than vpiRealVal and vpiStringVal. If the format specified is vpiStringVal, then
the value shall be returned as a string representation of a floating-point number. The format of this string
shall be in decimal notation with at most 16 digits of precision.
If a constant object’s vpiConstType is vpiStringConst, the value shall be retrieved using a format of either
vpiStringVal or vpiVectorVal.
The misc field in the s_vpi_value structure shall provide for alternative value types, which can be
implementation specific. If this field is utilized, one or more corresponding format types shall also be
provided.
In the following example, the binary value of each net that is contained in a particular module and whose
name begins with a particular string is displayed. [This function makes use of the strcmp() facility
normally declared in a string.h C library.]
void display_certain_net_values(mod, target)
vpiHandle mod;
PLI_BYTE8 *target;
{
static s_vpi_value value_s = {vpiBinStrVal};
static p_vpi_value value_p = &value_s;
vpiHandle net, itr;
itr = vpi_iterate(vpiNet, mod);
while (net = vpi_scan(itr))
{
PLI_BYTE8 *net_name = vpi_get_str(vpiName, net);
if (strcmp(target, net_name) == 0)
{
vpi_get_value(net, value_p);
vpi_printf("Value of net %s: %s\n",
vpi_get_str(vpiFullName, net),value_p->value.str);
}
}
}
The following example illustrates the use of vpi_get_value() to access UDP table entries. Two sample
outputs from this example are provided after the example.
/*
* hUDP has to be a handle to a UDP definition
*/
static void dumpUDPTableEntries(vpiHandle hUDP)
{
vpiHandle hEntry, hEntryIter;
s_vpi_value value;
PLI_INT32 numb;

<!-- Page 1106 -->

IEEE Std PLI_INT32 udpType;
PLI_INT32 item;
PLI_INT32 entryVal;
PLI_INT32 *abItem;
PLI_INT32 cnt, cnt2;
numb = vpi_get(vpiSize, hUDP);
udpType = vpi_get(vpiPrimType, hUDP);
if (udpType == vpiSeqPrim)
numb++; /* There is one more table entry for state */
numb++; /* There is a table entry for the output */
hEntryIter = vpi_iterate(vpiTableEntry, hUDP);
if (!hEntryIter)
return;
value.format = vpiVectorVal;
while(hEntry = vpi_scan(hEntryIter))
{
vpi_printf("\n");
/* Show the entry as a string */
value.format = vpiStringVal;
vpi_get_value(hEntry, &value);
vpi_printf("%s\n", value.value.str);
/* Decode the vector value format */
value.format = vpiVectorVal;
vpi_get_value(hEntry, &value);
abItem = (PLI_INT32 *)value.value.vector;
for(cnt=((numb-1)/2+1);cnt>0;cnt--)
{
entryVal = *abItem;
abItem++;
/* Rip out 4 characters */
for (cnt2=0;cnt2<4;cnt2++)
{
item = entryVal&0xff;
if (item)
vpi_printf("%c", item);
else
vpi_printf("_");
entryVal = entryVal>>8;
}
}
}
vpi_printf("\n");
}
For a UDP table of
1 0 :?:1;
0 (01) :?:-;
(10) 0 :0:1;
the output from the preceding example would be
10:1
_0_1___1
01:0
_1_0___0
00:1
_0_0___1

<!-- Page 1107 -->

IEEE Std For a UDP table entry of
1 0 :?:1;
0 (01) :?:-;
(10) 0 :0:1;
the output from the preceding example would be
10:?:1
_0_1_1_?
0(01):?:-
10_0_-_?
(10)0:0:1
_001_1_0
### 38.16 vpi_get_value_array()
vpi_get_value_array()
Synopsis: Retrieve simulation values for contiguous elements of a static unpacked array object.
Syntax: vpi_get_value_array(obj, arrayvalue_p, index_p, num)
Type Description
Returns: void
Type Name Description
Arguments: vpiHandle obj Handle to an unpacked array object.
p_vpi_arrayvalue arrayvalue_p Pointer to a structure containing array value information.
PLI_INT32 * index_p Pointer to an array of index values corresponding to the
start of the section of the object to be retrieved.
PLI_UINT32 num Number of array elements to be retrieved.
Related Use vpi_put_value_array() to set values of contiguous elements of a static unpacked array object
routines:
The VPI routine vpi_get_value_array() shall retrieve simulation values of contiguous elements in static
unpacked variable or net arrays (array objects for which the vpiArrayType property is vpiStaticArray).
Such arrays shall also have static lifetimes and not contain dynamic arrays or dynamic elements (e.g., string
vars). For purposes here, the term element corresponds to any indexable member of such an array with all
unpacked indices fully specified. The data type of each element so defined corresponds to the data type of
the array with all unpacked ranges removed. The elements of arrays are not allowed to be of an unpacked
type themselves (e.g., unpacked structs).
The values for the array section shall be placed in an s_vpi_arrayvalue structure defined in
vpi_user.h, as follows:
typedef struct t_vpi_arrayvalue
{
PLI_UINT32 format;
PLI_UINT32 flags;
union

<!-- Page 1108 -->

IEEE Std {
PLI_INT32 *integers;
PLI_INT16 *shortints;
PLI_INT64 *longints;
PLI_BYTE8 *rawvals;
struct t_vpi_vecval *vectors;
struct t_vpi_time *times;
double *reals;
float *shortreals;
} value;
} s_vpi_arrayvalue, *p_vpi_arrayvalue;
The s_vpi_arrayvalue structure shown above shall be allocated by the application. However, the
application has the flexibility of allocating the actual storage where the array element values are placed (see
the following). The layout of the values retrieved shall be set by the format field in the structure. In addition
to the format types vpiIntVal, vpiTimeVal, vpiVectorVal, and vpiRealVal available with the
vpi_get_value() function (Table38-3 in 38.15), the following format types are available:
vpiRawFourStateVal
Values for each element retrieved will be stored in aval/bval format (similar to 4-
state vectors) using the *rawvals field of the union above, interleaved
according to the following structure:
struct
{
PLI_BYTE8 avalbits[ngroups];
PLI_BYTE8 bvalbits[ngroups];
}
Each array element occupies ngroups*2 bytes stored consecutively as A/B byte
groups as shown above. For the first indexed array element, the avalbits
begins at rawvals[0], and the bvalbits at rawvals[ngroups],
respectively. The second array element’s avalbits begin at
rawvals[ngroups*2], and its bvalbits at rawvals[ngroups*3], etc.
ngroups is computed given the array element size in bits (= elemBits) as
follows:
int ngroups = (elemBits + 7) / 8;
The total storage required to hold “num” array elements shall be
ngroups * num * 2.
vpiRawTwoStateVal
Values for each element retrieved shall be stored similarly to
vpiRawFourStateVal above (also using the *rawvals struct member), except
that the bvalbits byte group shall be omitted. ngroups shall be computed
similarly also, but the total storage required shall instead be ngroups * num.
vpiShortIntVal Values retrieved will be stored as an array of “num” short(s), using the
*shortints field in the union in this case. This format is appropriate only for
arrays of vpiShortIntVar or vpiByteVar elements.
vpiLongIntVal Values retrieved will be stored as an array of “num” long(s), using the
*longints field in the union in this case. This format is appropriate for arrays
of vpiLongIntVar, vpiShortIntVar or vpiByteVar elements.
vpiShortRealVal Values retrieved will be stored as an array of “num” floats, using the
*shortrealvals field in the union in this case. This format is appropriate only
for arrays of vpiShortRealVar elements.

<!-- Page 1109 -->

IEEE Std The format types vpiIntVal, vpiTimeVal, vpiVectorVal, and vpiRealVal that are also available with
vpi_get_value() function correspond to similar union member names in s_vpi_arrayvalue (converted to
pointer values and ending in “s” to indicate they are arrays). For example, selecting the vpiIntVal format
shall cause an array of 32-bit integers to be returned (which should be accessed using the *integers field),
each representing an indexed element of the array object. The vpiVectorVal format shall cause an array of
consecutive A/B word groups formatted according to the t_vpi_vecval structure (Figure38-8 in 38.15) to
be retrieved. The *vectors field should be used to access them. Given the array element size in bits (==
elemBits), the number of words of storage required will be:
((elemBits + 31) / 32) * 2 * num
All other formats not mentioned here are unsupported and shall result in an error if requested. Also, formats
requested that are inconsistent with the data type of the array elements (except where explicitly allowed)
shall be considered an error.
The vpiRawFourStateVal and vpiVectorVal formats are appropriate for all 4-state array types (all net
arrays, or variable arrays of vpiLogicVar, vpiIntegerVar, vpiTimeVar, or 4-state packed vpiStructVar or
vpiUnionVar elements). The vpiRawTwoStateVal format is appropriate for all 2-state array types
(variable arrays of vpiBitVar, vpiByteVar, vpiShortInt, vpiInt, vpiLongInt, or 2-state packed
vpiStructVar or vpiUnionVar elements). The vpiRawFourStateVal or vpiVectorVal formats can also be
requested of a 2-state array type, and the vpiRawTwoStateVal format can be requested for a 4-state array
type. The bit values in each array element, whether fixed or variable width, correspond to significance order
in avalbits and bvalbits. That is, the LSB of rawvals[0] and rawvals[ngroups] indicates the A and B
value of the LSB (0th) bit of the first array element, respectively, and the LSB of rawvals[1] and
rawvals[ngroups+1] indicates the A and B value of bit 8 of the first array element (if it is of width 9 bits
or greater), and so on. Similar significance order conventions apply to A/B word groups in the
vpiVectorVal format, as described for vpi_get_value() (38.15).
The index_p argument is an array containing the indices of the starting element to be retrieved in the array
object. The indices are ordered in this array according to left-to-right order they would appear in an
expression in HDL text. The size of the index_p index array shall be equal to the number of unpacked
dimensions of obj, the array object.
The array element values are retrieved consecutively in order of the fastest varying index (rightmost
unpacked range of the array declaration), followed by more slowly varying indices accordingly until the
number of elements (num) has been retrieved. Index values within each range are ordered from leftmost
range value to rightmost. For example, elements of an array a[2:0][3:5] with index_p[0] = 1 and
index_p[1] = 4 would be retrieved in the order a[1][4], a[1][5], a[0][3], a[0][4], a[0][5],
respectively.
By default, array values shall be returned in memory allocated by VPI (in which case the storage should be
regarded as read-only). In this case, since the same VPI storage area may be overwritten with subsequent
calls to this function, the caller needs to save this data elsewhere in order to preserve it.
However, if the application sets the vpiUserAllocFlag in the value.flags field, the function will assume the
calling application has set the value field to point to a buffer of sufficient size allocated by the application
for placing the values. For all formats requested except for vpiRawFourStateVal, vpiVectorVal, and
vpiRawTwoStateVal, the buffer size can be simply computed as:
size = num * sizeof(<union ptr type>);
For example, a buffer sized to hold an array of small integers (of vpiByteVar or vpiShortIntVar elements)
using the vpiShortIntVal format type set would be sized as:
size = num * sizeof(PLI_INT16);

<!-- Page 1110 -->

IEEE Std Buffers allocated to hold the vpiRawFourStateVal and vpiRawTwoStateVal formats shall be sized
according to the instructions in their format description.
If vpi_get_value_array() returns NULL for the value pointer in either case, it shall indicate that a VPI error
has occurred in the retrieval process. It shall be the application’s responsibility to free memory it has
allocated, even if a VPI error has occurred (when the value field pointer is overwritten to NULL). The
application should always save the value of the pointer to memory it allocates so that it can be freed later.
Using the previous example of array a, the following code could be used to retrieve the five values shown
above starting at a[1][4] (with the application code allocating the storage for them):
/* Retrieve 5 element values from array "logic a[2:0][3:5]"
* starting at "a[1][4]", given "arrH", a vpiHandle for "a". */
int indexArr[2];
PLI_BYTE8 *valueBuffer; /* Retain local ptr to mem allocated */
s_vpi_arrayvalue arrayVal = { 0, 0, NULL };
vpiHandle elemH, elemI;
int elemWidth, ngroups;
int num = 5;
/* Get array element so we can get size to determine ngroups */
elemI = vpi_iterate(vpiReg, arrH);
elemH = vpi_scan(elemI);
elemWidth = vpi_get(vpiSize, elemH);
ngroups = (elemWidth + 7) / 8;
vpi_release_handle(elemI);
/* Allocate storage and retrieve the values. */
arrayVal.format = vpiRawFourStateVal;
arrayVal.flags |= vpiUserAllocFlag; /* We allocate the memory */
valueBuffer = (PLI_BYTE8 *)malloc(ngroups * 2 * num);
arrayVal.value.rawvals = valueBuffer;
indexArr[0] = 1;
indexArr[1] = 4;
vpi_get_value_array(arrH, &arrayVal, indexArr, num);
/* Check for result status */
if (arrayVal.value.rawvals == NULL) {
/* ... We have an error- check it. ... */
} else {
/* ... Values OK- process them. ... */
}
free(valueBuffer);

<!-- Page 1111 -->

IEEE Std ### 38.17 vpi_get_vlog_info()
vpi_get_vlog_info()
Synopsis: Retrieve information about SystemVerilog simulation execution.
Syntax: vpi_get_vlog_info(vlog_info_p)
Type Description
Returns: PLI_INT32 1 (true) on success; 0 (false) on failure.
Type Name Description
Arguments: p_vpi_vlog_info vlog_info_p Pointer to a structure containing simulation information.
Related
routines:
The VPI routine vpi_get_vlog_info() shall obtain the following information about SystemVerilog tool
execution:
— Number of invocation options (argc)
— Invocation option values (argv)
— Product and version strings
The information shall be contained in an s_vpi_vlog_info structure. The routine shall return 1 (true) on
success and 0 (false) on failure.
The s_vpi_vlog_info structure used by vpi_get_vlog_info() is defined in vpi_user.h and is listed in
Figure38-10.
typedef struct t_vpi_vlog_info
{
PLI_INT32 argc;
PLI_BYTE8 **argv;
PLI_BYTE8 *product;
PLI_BYTE8 *version;
} s_vpi_vlog_info, *p_vpi_vlog_info;
Figure38-10—s_vpi_vlog_info structure definition
The format of the argv array is that each pointer in the array shall point to a NULL-terminated character array
that contains the string located on the tool’s invocation command line. There shall be argc entries in the argv
array. The value in entry zero shall be the tool’s name.
The vendor tool may provide a command-line option to pass a file containing a set of options. In that case,
the argument strings returned by vpi_get_vlog_info() shall contain the vendor option string name followed
by a pointer to a NULL-terminated array of pointers to characters. This new array shall contain the parsed
contents of the file. The value in entry zero shall contain the name of the file. The remaining entries shall
contain pointers to NULL-terminated character arrays containing the different options in the file. The last
entry in this array shall be NULL. If one of the options is the vendor file option, then the next pointer shall
behave the same as previously described.

<!-- Page 1112 -->

IEEE Std ### 38.18 vpi_handle()
vpi_handle()
Synopsis: Obtain a handle to an object with a one-to-one relationship.
Syntax: vpi_handle(type, ref)
Type Description
Returns: vpiHandle Handle to an object.
Type Name Description
Arguments: PLI_INT32 type An integer constant representing the type of object for
which to obtain a handle.
vpiHandle ref Handle to a reference object.
Related Use vpi_iterate() and vpi_scan() to obtain handles to objects with a one-to-many relationship.
routines: Use vpi_handle_multi() to obtain a handle to an object with a many-to-one relationship.
The VPI routine vpi_handle() shall return the object of typetype associated with object ref. Unless
otherwise specified, calling vpi_handle() for a protected object shall be an error. The one-to-one
relationships that are traversed with this routine are indicated as single arrows in the data model diagrams.
The following example application displays each primitive that an input net drives:
void display_driven_primitives(net)
vpiHandle net;
{
vpiHandle load, prim, itr;
vpi_printf("Net %s drives terminals of the primitives: \n",
vpi_get_str(vpiFullName, net));
itr = vpi_iterate(vpiLoad, net);
if (!itr)
return;
while (load = vpi_scan(itr))
{
switch(vpi_get(vpiType, load))
{
case vpiGate:
case vpiSwitch:
case vpiUdp:
prim = vpi_handle(vpiPrimitive, load);
vpi_printf("\t%s\n", vpi_get_str(vpiFullName, prim));
}
}
}

<!-- Page 1113 -->

IEEE Std ### 38.19 vpi_handle_by_index()
vpi_handle_by_index()
Synopsis: Get a handle to an object using its index number within a parent object.
Syntax: vpi_handle_by_index(obj, index)
Type Description
Returns: vpiHandle Handle to an object.
Type Name Description
Arguments: vpiHandle obj Handle to an object.
PLI_INT32 index Index number of the object for which to obtain a handle.
Related
routines:
The VPI routine vpi_handle_by_index() shall return a handle to an object based on the index number of the
object within the reference object obj. The reference object shall be an object that has the access by index
property. Unless otherwise specified, calling vpi_handle_by_index() for a protected object shall be an
error. For example, to access a net bit, obj would be the associated net; to access an element of a reg array,
obj would be the array. If the selection represented by the index number does not lead to the construction of
a legal SystemVerilog index select expression, the routine shall return a null handle.
### 38.20 vpi_handle_by_multi_index()
vpi_handle_by_multi_index()
Synopsis: Obtain a handle to a subobject using an array of indices and a reference object.
Syntax: vpi_handle_by_multi_index(obj, num_index, index_array)
Type Description
Returns: vpiHandle Handle to an object.
Type Name Description
Arguments: vpiHandle obj Handle to an object.
PLI_INT32 num_index Number of indices in the index array.
PLI_INT32 * index_array Array of indices. Leftmost index first.
Related
routines:
The VPI routine vpi_handle_by_multi_index() shall provide access to an index-selected subobject of the
reference handle. The reference object shall be an object that has the access by index property. Unless
otherwise specified, calling vpi_handle_by_multi_index() for a protected object shall be an error. This
routine shall return a handle to a valid SystemVerilog object based on the list of indices provided by the
argument index_array and reference handle denoted by obj. The argument num_index shall contain the
number of indices in the provided array index_array.

<!-- Page 1114 -->

IEEE Std The order of the indices provided shall follow the array dimension declaration from the leftmost range to the
rightmost range of the reference handle; the array indices may be optionally followed by a bit-select index.
If the indices provided do not lead to the construction of a legal SystemVerilog index select expression, the
routine shall return a null handle.
### 38.21 vpi_handle_by_name()
vpi_handle_by_name()
Synopsis: Get a handle to an object with a specific name.
Syntax: vpi_handle_by_name(name, scope)
Type Description
Returns: vpiHandle Handle to an object.
Type Name Description
Arguments: PLI_BYTE8 * name A character string or pointer to a string containing the
name of an object.
vpiHandle scope Handle to a SystemVerilog scope.
Related
routines:
The VPI routine vpi_handle_by_name() shall return a handle to an object with a specific name. This
function can be applied to all objects with a fullname property. The name can be hierarchical or simple. If
scope is NULL, then name shall be searched for from the top level of hierarchy. If a scope object is provided,
then search within that scope only. Unless otherwise specified, calling vpi_handle_by_name() for a
protected scope object shall be an error. If the name is hierarchical and includes a protected scope, the call
shall be an error.

<!-- Page 1115 -->

IEEE Std ### 38.22 vpi_handle_multi()
vpi_handle_multi()
Synopsis: Obtain a handle for an object in a many-to-one relationship.
Syntax: vpi_handle_multi(type, ref1, ref2, ...)
Type Description
Returns: vpiHandle Handle to an object.
Type Name Description
Arguments: PLI_INT32 type An integer constant representing the type of object for
which to obtain a handle.
vpiHandle ref1, ref2, ... Handles to two or more reference objects.
Related Use vpi_iterate() and vpi_scan() to obtain handles to objects with a one-to-many relationship.
routines: Use vpi_handle() to obtain handles to objects with a one-to-one relationship.
The VPI routine vpi_handle_multi() can be used to return a handle to an object of type vpiInterModPath
associated with a list of output port and input port reference objects. The ports shall be of the same size and
can be at different levels of the hierarchy.
### 38.23 vpi_iterate()
vpi_iterate()
Synopsis: Obtain an iterator handle to objects with a one-to-many relationship.
Syntax: vpi_iterate(type, ref)
Type Description
Returns: vpiHandle Handle to an iterator for an object.
Type Name Description
Arguments: PLI_INT32 type An integer constant representing the type of object for
which to obtain iterator handles.
vpiHandle ref Handle to a reference object.
Related Use vpi_scan() to traverse the design hierarchy using the iterator handle returned from vpi_iterate().
routines: Use vpi_handle() to obtain handles to object with a one-to-one relationship.
Use vpi_handle_multi() to obtain a handle to an object with a many-to-one relationship.
The VPI routine vpi_iterate() shall be used to traverse one-to-many relationships, which are indicated as
double arrows in the data model diagrams. Unless otherwise specified, calling vpi_iterate() for a protected
object shall be an error. The vpi_iterate() routine shall return a handle to an iterator, whose type shall be
vpiIterator, which can used by vpi_scan() to traverse all objects of typetype associated with object ref. To
get the reference object from the iterator object, use vpi_handle(vpiUse, iterator_handle). If there are no
objects of typetype associated with the reference handle ref, then the vpi_iterate() routine shall return
NULL.
The following example application uses vpi_iterate() and vpi_scan() to display each net (including the size
for vectors) declared in the module. The example assumes it shall be passed a valid module handle.

<!-- Page 1116 -->

IEEE Std void display_nets(mod)
vpiHandle mod;
{
vpiHandle net;
vpiHandle itr;
vpi_printf("Nets declared in module %s\n",vpi_get_str(vpiFullName, mod));
itr = vpi_iterate(vpiNet, mod);
while (net = vpi_scan(itr))
{
vpi_printf("\t%s", vpi_get_str(vpiName, net));
if (vpi_get(vpiVector, net))
{
vpi_printf(" of size %d\n", vpi_get(vpiSize, net));
}
else vpi_printf("\n");
}
}
### 38.24 vpi_mcd_close()
vpi_mcd_close()
Synopsis: Close one or more files opened by vpi_mcd_open().
Syntax: vpi_mcd_close(mcd)
Type Description
Returns: PLI_UINT32 0 if successful; the mcd of unclosed channels if unsuccessful.
Type Name Description
Arguments: PLI_UINT32 mcd A multichannel descriptor representing the files to close.
Related Use vpi_mcd_open() to open a file.
routines: Use vpi_mcd_printf() to write to an opened file.
Use vpi_mcd_vprintf() to write a variable number of arguments to an opened file.
Use vpi_mcd_flush() to flush a file output buffer.
Use vpi_mcd_name() to get the name of a file represented by a channel descriptor.
The VPI routine vpi_mcd_close() shall close the file(s) specified by a multichannel descriptor mcd. Several
channels can be closed simultaneously because channels are represented by discrete bits in the integer mcd.
On success, this routine shall return a 0; on error, it shall return the mcd value of the unclosed channels. This
routine can also be used to close file descriptors that were opened using the system function $fopen. See
#### 21.3.1 for the functional description of $fopen.
The following descriptor is predefined and cannot be closed using vpi_mcd_close():
— descriptor 1 is for the output channel of the tool that invoked the PLI application and the current log
file

<!-- Page 1117 -->

IEEE Std ### 38.25 vpi_mcd_flush()
vpi_mcd_flush()
Synopsis: Flushes the data from the given mcd output buffers.
Syntax: vpi_mcd_flush(mcd)
Type Description
Returns: PLI_INT32 0 if successful; nonzero if unsuccessful.
Type Name Description
Arguments: PLI_UINT32 mcd A multichannel descriptor representing the files to which
to write.
Related Use vpi_mcd_printf() to write a finite number of arguments to an opened file.
routines: Use vpi_mcd_vprintf() to write a variable number of arguments to an opened file.
Use vpi_mcd_open() to open a file.
Use vpi_mcd_close() to close a file.
Use vpi_mcd_name() to get the name of a file represented by a channel descriptor.
The routine vpi_mcd_flush() shall flush the output buffers for the file(s) specified by the multichannel
descriptor mcd.
### 38.26 vpi_mcd_name()
vpi_mcd_name()
Synopsis: Get the name of a file represented by a channel descriptor.
Syntax: vpi_mcd_name(cd)
Type Description
Returns: PLI_BYTE8 * Pointer to a character string containing the name of a file.
Type Name Description
Arguments: PLI_UINT32 cd A channel descriptor representing a file.
Related Use vpi_mcd_open() to open a file.
routines: Use vpi_mcd_close() to close files.
Use vpi_mcd_printf() to write to an opened file.
Use vpi_mcd_flush() to flush a file output buffer.
Use vpi_mcd_vprintf() to write a variable number of arguments to an opened file.
The VPI routine vpi_mcd_name() shall return the name of a file represented by a single-channel descriptor
cd. On error, the routine shall return NULL. This routine shall overwrite the returned value on subsequent
calls. If the application needs to retain the string, it should copy it. This routine can be used to get the name
of any file opened using the system function $fopen or the VPI routine vpi_mcd_open(). The channel
descriptor cd could be an fd file descriptor returned from $fopen (indicated by the MSB being set) or an
mcd multichannel descriptor returned by either the system function $fopen or the VPI routine
vpi_mcd_open(). See 21.3.1 for the functional description of $fopen.

<!-- Page 1118 -->

IEEE Std ### 38.27 vpi_mcd_open()
vpi_mcd_open()
Synopsis: Open a file for writing.
Syntax: vpi_mcd_open(file)
Type Description
Returns: PLI_UINT32 A multichannel descriptor representing the file that was opened.
Type Name Description
Arguments: PLI_BYTE8 * file A character string or pointer to a string containing the file
name to be opened.
Related Use vpi_mcd_close() to close a file.
routines: Use vpi_mcd_printf() to write to an opened file.
Use vpi_mcd_vprintf() to write a variable number of arguments to an opened file.
Use vpi_mcd_flush() to flush a file output buffer.
Use vpi_mcd_name() to get the name of a file represented by a channel descriptor.
The VPI routine vpi_mcd_open() shall open a file for writing and shall return a corresponding multichannel
description number (mcd). The channel descriptor 1 (LSB) is reserved for representing the output channel of
the tool that invoked the PLI application and the log file (if one is currently open). The channel descriptor
32(MSB) is reserved to represent a file descriptor (fd) returned from the SystemVerilog $fopen system
function.
The mcd descriptor returned by vpi_mcd_open() routine is compatible with the mcd descriptors returned
from the $fopen system function. The mcd descriptors returned from vpi_mcd_open() and from $fopen
may be shared between the built-in system tasks that use mcd descriptors and the VPI routines that use mcd
descriptors. If the MSB of the return value from $fopen is set, then the value is an fd file descriptor, which
is not compatible with the mcd descriptor returned by vpi_mcd_open(). See 21.3.1 for the functional
description of $fopen.
The vpi_mcd_open() routine shall return a 0 on error. If the file has already been opened either by a
previous call to vpi_mcd_open() or using $fopen in the SystemVerilog source code, then vpi_mcd_open()
shall return the descriptor number.

<!-- Page 1119 -->

IEEE Std ### 38.28 vpi_mcd_printf()
vpi_mcd_printf()
Synopsis: Write to one or more files opened with vpi_mcd_open() or $fopen.
Syntax: vpi_mcd_printf(mcd, format, ...)
Type Description
Returns: PLI_INT32 The number of characters written.
Type Name Description
Arguments: PLI_UINT32 mcd A multichannel descriptor representing the files to which
to write.
PLI_BYTE8 * format A format string using the C fprintf() format.
Related Use vpi_mcd_vprintf() to write a variable number of arguments to an opened file.
routines: Use vpi_mcd_open() to open a file.
Use vpi_mcd_close() to close a file.
Use vpi_mcd_flush() to flush a file output buffer.
Use vpi_mcd_name() to get the name of a file represented by a channel descriptor.
The VPI routine vpi_mcd_printf() shall write to one or more channels (up to 31) determined by the mcd.
An mcd of 1 (bit 0 set) corresponds to the channel 1, an mcd of 2 (bit 1 set) corresponds to channel 2, an mcd
of 4 (bit 2 set) corresponds to channel 3, and so on. Channel 1 is reserved for the output channel of the tool
that invoked the PLI application and the current log file. The MSB of the descriptor is reserved by the tool to
indicate that the descriptor is actually a file descriptor instead of an mcd. vpi_mcd_printf() shall also write
to a file represented by an mcd that was returned from the SystemVerilog $fopen system function.
vpi_mcd_printf() shall not write to a file represented by an fd file descriptor returned from $fopen
(indicated by the MSB being set). See 21.3.1 for the functional description of $fopen.
Several channels can be written to simultaneously because channels are represented by discrete bits in the
integer mcd.
The text written shall be controlled by one or more format strings. The format strings shall use the same
format as the C fprintf() routine. The routine shall return the number of characters printed or return EOF if
an error occurred.

<!-- Page 1120 -->

IEEE Std ### 38.29 vpi_mcd_vprintf()
vpi_mcd_vprintf()
Synopsis: Write to one or more files opened with vpi_mcd_open() or $fopen using varargs that are already started.
Syntax: vpi_mcd_vprintf(mcd, format, ap)
Type Description
Returns: PLI_INT32 The number of characters written.
Type Name Description
Arguments: PLI_UINT32 mcd A multichannel descriptor representing the files to which
to write.
PLI_BYTE8 * format A format string using the C printf() format.
va_list ap An already started varargs list.
Related Use vpi_mcd_printf() to write a finite number of arguments to an opened file.
routines: Use vpi_mcd_open() to open a file.
Use vpi_mcd_close() to close a file.
Use vpi_mcd_flush() to flush a file output buffer.
Use vpi_mcd_name() to get the name of a file represented by a channel descriptor.
This routine performs the same function as vpi_mcd_printf(), except that varargs have already been started.
### 38.30 vpi_printf()
vpi_printf()
Synopsis: Write to the output channel of the tool that invoked the PLI application and the current tool log file.
Syntax: vpi_printf(format, ...)
Type Description
Returns: PLI_INT32 The number of characters written.
Type Name Description
Arguments: PLI_BYTE8 * format A format string using the C printf() format.
Related Use vpi_vprintf() to write a variable number of arguments.
routines: Use vpi_mcd_printf() to write to an opened file.
Use vpi_mcd_flush() to flush a file output buffer.
Use vpi_mcd_vprintf() to write a variable number of arguments to an opened file.
The VPI routine vpi_printf() shall write to both the output channel of the tool that invoked the PLI
application and the current tool log file. The format string shall use the same format as the Cprintf()
routine. The routine shall return the number of characters printed or return EOF if an error occurred.

<!-- Page 1121 -->

IEEE Std ### 38.31 vpi_put_data()
vpi_put_data()
Synopsis: Put data into an implementation’s save/restart location.
Syntax: vpi_put_data(id, dataLoc, numOfBytes)
Type Description
Returns: PLI_INT32 The number of bytes written.
Type Name Description
Arguments: PLI_INT32 id A save/restart ID returned from
vpi_get(vpiSaveRestartID, NULL).
PLI_BYTE8 * dataLoc Address of application-allocated storage.
PLI_INT32 numOfBytes Number of bytes to be added to save/restart location.
Related Use vpi_get_data() to retrieve saved data.
routines:
This routine shall place numOfBytes, which shall be greater than zero, of data located at dataLoc into an
implementation’s save/restart location. The return value shall be the number of bytes written. A zero shall be
returned if an error is detected. There shall be no restrictions on the following:
— How many times the routine can be called for a given id
— The order applications put data using the different ids
The data from multiple calls to vpi_put_data() with the same id shall be stored by the simulator in such a
way that the opposing routine vpi_get_data() can pull data out of the save/restart location using different
sizes of chunks. This routine can only be called from an application routine that has been called for the
reason cbStartOfSave or cbEndOfSave. An application can get the path to the implementation’s save/
restart location by calling vpi_get_str(vpiSaveRestartLocation, NULL) from an application callback
routine that has been called for reason cbStartOfSave or cbEndOfSave.
The following example illustrates using vpi_put_data() and vpi_get_data():
#include <stdlib.h>
#include <assert.h>
#include "vpi_user.h"
typedef struct myStruct *myStruct_p;
typedef struct myStruct {
PLI_INT32 d1;
PLI_INT32 d2;
myStruct_p next;
} myStruct_s;
static myStruct_p firstWrk = NULL;
PLI_INT32 consumer_restart(p_cb_data data)
{
struct myStruct *wrk;
PLI_INT32 status;
PLI_INT32 cnt, size;
PLI_INT32 id = (PLI_INT32)data->user_data;

<!-- Page 1122 -->

IEEE Std /* Get the number of structures */
status = vpi_get_data(id,(PLI_BYTE8 *)&cnt,sizeof(PLI_INT32));
assert(status > 0); /* Check returned status */
/* allocate memory for the structures */
size = cnt * sizeof(struct myStruct);
firstWrk = (myStruct_p)malloc(size);
/* retrieve the data structures */
if (cnt != vpi_get_data(id, (PLI_BYTE8 *)firstWrk,cnt))
return(1); /* error */
firstWrk = wrk;
/* Fix the next pointers in the linked list */
for (wrk = firstWrk; cnt > 0; cnt--)
{
wrk->next = wrk + 1;
wrk = wrk->next;
}
wrk->next = NULL;
return(0); /* SUCCESS */
}
PLI_INT32 consumer_save(p_cb_data data)
{
myStruct_p wrk;
s_cb_data cbData;
vpiHandle cbHdl;
PLI_INT32 id = 0;
PLI_INT32 cnt = 0;
/* Get the number of structures */
wrk = firstWrk;
while (wrk)
{
cnt++;
wrk = wrk->next;
}
/* now save the data */
wrk = firstWrk;
id = vpi_get(vpiSaveRestartID, NULL);
/* save the number of data structures */
vpi_put_data(id,(PLI_BYTE8 *)cnt,sizeof(PLI_INT32));
/* Save the different data structures. Note that a pointer
* is being saved. While this is allowed, an application
* needs to change it to something useful on a restart.
*/

<!-- Page 1123 -->

IEEE Std while (wrk)
{
vpi_put_data(id,(PLI_BYTE8 *)wrk,sizeof(myStruct_s));
wrk = wrk->next;
}
/* register a call for restart */
/* We need the "id" so that the saved data can be retrieved.
* Using the user_data field of the callback structure is the
* easiest way to pass this information to retrieval operation.
*/
cbData.user_data = (PLI_BYTE8 *)id;
cbData.reason = cbStartOfRestart;
/* See 38.9 vpi_get_data() for a description of how
* the callback routine can be used to retrieve the data.
*/
cbData.cb_rtn = consumer_restart;
cbData.value = NULL;
cbData.time = NULL;
cbHdl = vpi_register_cb(&cbData);
vpi_release_handle(cbHdl);
return(0);
}
### 38.32 vpi_put_delays()
vpi_put_delays()
Synopsis: Set the delays or timing limits of an object.
Syntax: vpi_put_delays(obj, delay_p)
Type Description
Returns: void
Type Name Description
Arguments: vpiHandle obj Handle to an object.
p_vpi_delay delay_p Pointer to a structure containing delay information.
Related Use vpi_get_delays() to retrieve delays or timing limits of an object.
routines:
The VPI routine vpi_put_delays() shall set the delays or timing limits of an object as indicated in the
delay_p structure. The same ordering of delays shall be used as described in the vpi_get_delays() function.
If only the delay changes and not the pulse limits, the pulse limits shall retain the values they had before the
delays where altered.
The s_vpi_delay and s_vpi_time structures used by both vpi_get_delays() and vpi_put_delays() are
defined in vpi_user.h and are listed in Figure38-11 and Figure38-12.

<!-- Page 1124 -->

IEEE Std typedef struct t_vpi_delay
{
struct t_vpi_time *da; /* pointer to application-allocated
array of delay values*/
PLI_INT32 no_of_delays; /* number of delays */
PLI_INT32 time_type; /* [vpiScaledRealTime,vpiSimTime,
vpiSuppressTime]*/
PLI_INT32 mtm_flag; /* true for mtm values */
PLI_INT32 append_flag; /* true for append */
PLI_INT32 pulsere_flag; /* true for pulsere values */
} s_vpi_delay, *p_vpi_delay;
Figure38-11—s_vpi_delay structure definition
typedef struct t_vpi_time
{
PLI_INT32 type; /* [vpiScaledRealTime, vpiSimTime, vpiSuppressTime] */
PLI_UINT32 high, low; /* for vpiSimTime */
double real; /* for vpiScaledRealTime */
} s_vpi_time, *p_vpi_time;
Figure38-12—s_vpi_time structure definition
The da field of the s_vpi_delay structure shall be an application-allocated array of s_vpi_time
structures. This array stores the delay values to be written by vpi_put_delays(). The number of elements in
this array is determined by the following:
— The number of delays to be written
— The mtm_flag setting
— The pulsere_flag setting
The number of delays to be set shall be set in the no_of_delays field of the s_vpi_delay structure. Legal
values for the number of delays shall be determined by the type of object, as follows:
— For primitive objects, the no_of_delays value shall be 2 or 3.
— For path delay objects, the no_of_delays value shall be 1, 2, 3, 6, or 12.
— For timing check objects, the no_of_delays value shall match the number of limits existing in the
timing check.
— For intermodule path objects, the no_of_delays value shall be 2 or 3.
The application-allocated s_vpi_delay array shall contain delays in the same order in which they occur in
the SystemVerilog source description. The number of elements for each delay shall be determined by the
flags mtm_flag and pulsere_flag, as shown in Table38-4.
Table38-4—Size of the s_vpi_delay->da array
Number of
Order in which delay elements
Flag values s_vpi_time array elements
shall be filled
required for s_vpi_delay->da
mtm_flag = FALSE no_of_delays 1st delay: da[0] -> 1st delay
pulsere_flag = FALSE 2nd delay: da[1] -> 2nd delay
...
mtm_flag = TRUE 3 × no_of_delays 1st delay: da[0] -> min delay
pulsere_flag = FALSE da[1] -> typ delay
da[2] -> max delay
2nd delay: ...

<!-- Page 1125 -->

IEEE Std Table38-4—Size of the s_vpi_delay->da array (continued)
Number of
Order in which delay elements
Flag values s_vpi_time array elements
shall be filled
required for s_vpi_delay->da
mtm_flag = FALSE 3 × no_of_delays 1st delay: da[0] -> delay
pulsere_flag = TRUE da[1] -> reject limit
da[2] -> error limit
2nd delay element: ...
mtm_flag = TRUE 9 × no_of_delays 1st delay: da[0] -> min delay
pulsere_flag = TRUE da[1] -> typ delay
da[2] -> max delay
da[3] -> min reject
da[4] -> typ reject
da[5] -> max reject
da[6] -> min error
da[7] -> typ error
da[8] -> max error
2nd delay: ...
The following example application accepts a module path handle, rise and fall delays, and replaces the
delays of the indicated path:
void set_path_rise_fall_delays(path, rise, fall)
vpiHandle path;
double rise, fall;
{
static s_vpi_time path_da[2];
static s_vpi_delay delay_s = {NULL, 2, vpiScaledRealTime};
static p_vpi_delay delay_p = &delay_s;
delay_s.da = path_da;
path_da[0].real = rise;
path_da[1].real = fall;
vpi_put_delays(path, delay_p);
}

<!-- Page 1126 -->

IEEE Std ### 38.33 vpi_put_userdata()
vpi_put_userdata()
Synopsis: Put user-data value into an implementation’s system task or system function instance storage location.
Syntax: vpi_put_userdata(obj, userdata)
Type Description
Returns: PLI_INT32 1 on success; 0 if an error occurs.
Type Name Description
Arguments: vpiHandle obj Handle to a system task instance or system function
instance.
void * userdata User-data value to be associated with the system task
instance or system function instance.
Related Use vpi_get_userdata() to retrieve the user-data value.
routines:
This routine will associate the value of the input userdata with the specified user-defined system task or
system function call handle. The stored value can later be retrieved with the routine vpi_get_userdata().
The routine will return a value of 1 on success or a 0 if it fails.
After a restart or a reset, subsequent calls to vpi_get_userdata() shall return NULL. It is the application’s
responsibility to save the data during a save using vpi_put_data() and to then retrieve it using
vpi_get_data(). The user-data field can be set up again during or after callbacks of type cbEndOfRestart or
cbEndOfReset.
### 38.34 vpi_put_value()
vpi_put_value()
Synopsis: Set a value on an object.
Syntax: vpi_put_value(obj, value_p, time_p, flags)
Type Description
Returns: vpiHandle Handle to the scheduled event caused by vpi_put_value().
Type Name Description
Arguments: vpiHandle obj Handle to an object.
p_vpi_value value_p Pointer to a structure with value information.
p_vpi_time time_p Pointer to a structure with delay information.
PLI_INT32 flags Integer constants that set the delay mode.
Related Use vpi_get_value() to retrieve the value of an expression.
routines:
The VPI routine vpi_put_value() shall set simulation logic values on an object. The value to be set shall be
stored in an s_vpi_value structure that has been allocated by the calling routine. Any storage referenced

<!-- Page 1127 -->

IEEE Std by the s_vpi_value structure shall also be allocated by the calling routine. The legal values that may be
specified for each value format are listed in Table38-3 in 38.15. The delay time before the value is set shall
be stored in an s_vpi_time structure that has been allocated by the calling routine. The routine can be
applied to nets, variables, variable selects, memory words, named events, system function calls, sequential
UDPs, and scheduled events, except for subelements of a net that belongs to a user-defined nettype. The
flags argument shall be used to direct the routine to use one of the following delay modes:
vpiInertialDelay All scheduled events on the object shall be removed before this event is
scheduled.
vpiTransportDelay All events on the object scheduled for times later than this event shall be
removed (modified transport delay).
vpiPureTransportDelay
No events on the object shall be removed (transport delay).
vpiNoDelay The object shall be set to the passed value with no delay. Argument time_p shall
be ignored and can be set to NULL.
vpiForceFlag The object shall be forced to the passed value with no delay (same as the
SystemVerilog procedural force). Argument time_p shall be ignored and can be
set to NULL.
vpiReleaseFlag The object shall be released from a forced value (same as the SystemVerilog
procedural release). Argument time_p shall be ignored and can be set to NULL.
The value_p shall be updated with the value of the object after its release. If the
value is a string, time, vector, strength, or miscellaneous value, the data pointed
to by the value_p argument shall be owned by the interface.
vpiCancelEvent A previously scheduled event shall be cancelled. The object passed to
vpi_put_value() shall be a handle to an object of type vpiSchedEvent.
If the flags argument also has the bit mask vpiReturnEvent, vpi_put_value() shall return a handle of type
vpiSchedEvent to the newly scheduled event, provided there is some form of a delay and an event is
scheduled. If the bit mask is not used, or if no delay is used, or if an event is not scheduled, the return value
shall be NULL.
A scheduled event can be cancelled by calling vpi_put_value() with obj set to the vpiSchedEvent handle
and flags set to vpiCancelEvent. The value_p and time_p arguments to vpi_put_value() are not needed for
cancelling an event and can be set to NULL. It shall not be an error to cancel an event that has already
occurred. The scheduled event can be tested by calling vpi_get() with the flag vpiScheduled. If an event is
cancelled, it shall simply be removed from the event queue. Any effects that were caused by scheduling the
event shall remain in effect (e.g., events that were cancelled due to inertial delay). Cancelling an event shall
also free the handle to that event.
Calling vpi_release_handle() on the handle shall free the handle, but shall not affect the event.
When vpi_put_value() is called for an object of type vpiNet or vpiNetBit, and with modes of
vpiInertialDelay, vpiTransportDelay, vpiPureTransportDelay, or vpiNoDelay, the value supplied
overrides the resolved value of the net. This value shall remain in effect until one of the drivers of the net
changes value. When this occurs, the net shall be reevaluated using the normal resolution algorithms.
It shall be illegal to specify the format of the value as vpiStringVal when putting a value to a real variable or
a system function call of type vpiRealFunc. It shall be illegal to specify the format of the value as
vpiStrengthVal when putting a value to a vector object.
When vpi_put_value() is used with vpiForceFlag, it shall perform a procedural force of a value onto the
same types of objects as supported by a procedural force. When used with vpiReleaseFlag, it shall release

<!-- Page 1128 -->

IEEE Std the forced value. This shall be the same functionality as the procedural force and release keywords in
SystemVerilog (see 10.6.2).
Sequential UDPs shall be set to the indicated value with no delay regardless of any delay on the primitive
instance. Putting values to UDP instances shall be done using the vpiNoDelay flag. Attempting to use the
other delay modes shall result in an error.
Calling vpi_put_value() on an object of type vpiNamedEvent shall cause the named event to toggle.
Objects of type vpiNamedEvent shall not require an actual value, and the value_p argument may be NULL.
The vpi_put_value() routine shall also return the value of a system function by passing a handle to the user-
defined system function as the object handle. This should only occur during execution of the calltf routine
for the system function. Attempts to use vpi_put_value() with a handle to the system function when the
calltf routine is not active shall be ignored. Should the calltf routine for a user-defined system function fail to
put a value during its execution, the default value of 0 will be applied. Putting return values to system
functions shall be done using the vpiNoDelay flag.
The vpi_put_value() routine shall only return a system function value in a calltf application when the call to
the system function is active. The action of vpi_put_value() to a system function shall be ignored when the
system function is not active. Putting values to system function shall be done using the vpiNoDelay flag.
The s_vpi_value and s_vpi_time structures used by vpi_put_value() are defined in vpi_user.h and
are listed in Figure38-13 and Figure38-14.
typedef struct t_vpi_value
{
PLI_INT32 format; /* vpi[[Bin,Oct,Dec,Hex]Str,Scalar,Int,Real,String,
Vector,Strength,Suppress,Time,ObjType]Val */
union
{
PLI_BYTE8 *str; /* string value */
PLI_INT32 scalar; /* vpi[0,1,X,Z] */
PLI_INT32 integer; /* integer value */
double real; /* real value */
struct t_vpi_time *time; /* time value */
struct t_vpi_vecval *vector; /* vector value */
struct t_vpi_strengthval *strength; /* strength value */
PLI_BYTE8 *misc; /* ...other */
} value;
} s_vpi_value, *p_vpi_value;
Figure38-13—s_vpi_value structure definition
typedef struct t_vpi_time
{
PLI_INT32 type; /* [vpiScaledRealTime, vpiSimTime, vpiSuppressTime] */
PLI_UINT32 high, low; /* for vpiSimTime */
double real; /* for vpiScaledRealTime */
} s_vpi_time, *p_vpi_time;
Figure38-14—s_vpi_time structure definition
The s_vpi_vecval and s_vpi_strengthval structures found in Figure38-13 are listed in Figure38-15
and Figure38-16.

<!-- Page 1129 -->

IEEE Std typedef struct t_vpi_vecval
{
/* following fields are repeated enough times to contain vector */
PLI_UINT32 aval, bval; /* bit encoding: ab: 00=0, 10=1, 11=X, 01=Z */
} s_vpi_vecval, *p_vpi_vecval;
Figure38-15—s_vpi_vecval structure definition
typedef struct t_vpi_strengthval
{
PLI_INT32 logic; /* vpi[0,1,X,Z] */
PLI_INT32 s0, s1; /* refer to strength coding in Annex K */
} s_vpi_strengthval, *p_vpi_strengthval;
Figure38-16—s_vpi_strengthval structure definition
For vpiScaledRealTime, the indicated time shall be in the timescale associated with the object.
### 38.35 vpi_put_value_array()
vpi_put_value_array()
Synopsis: Set values for contiguous elements of a static unpacked array object.
Syntax: vpi_put_value_array(obj, arrayvalue_p, index_p, num)
Type Description
Returns: void
Type Name Description
Arguments: vpiHandle obj Handle to an unpacked array object.
p_vpi_arrayvalue arrayvalue_p Pointer to a structure containing array value information.
PLI_INT32 * index_p Pointer to an array of index values corresponding to the
start of the section of the object to be updated.
PLI_UINT32 num Number of array elements to be updated.
Related Use vpi_get_value_array() to retrieve values of contiguous elements of a static unpacked array object.
routines:
The VPI routine vpi_put_value_array() shall modify simulation values of contiguous elements in static
unpacked variable or net arrays (array objects for which the vpiArrayType property is vpiStaticArray).
Such arrays shall also have static lifetimes and not contain dynamic arrays or dynamic elements (e.g., string
vars). For purposes here, the term element corresponds to any indexable member of such an array with all
unpacked indices fully specified. The data type of each element so defined corresponds to the data type of
the array with all unpacked ranges removed. The elements of arrays are not allowed to be of an unpacked
type themselves (e.g., unpacked structs).
The values to be set for the array shall be placed in an s_vpi_arrayvalue structure allocated by the
calling routine. Any storage referenced by the s_vpi_arrayvalue structure shall also be allocated by the
calling routine. The s_vpi_arrayvalue structure is defined in vpi_user.h, as follows:

<!-- Page 1130 -->

IEEE Std typedef struct t_vpi_arrayvalue
{
PLI_UINT32 format;
PLI_UINT32 flags;
union
{
PLI_INT32 *integers;
PLI_INT16 *shortints;
PLI_INT64 *longints;
PLI_BYTE8 *rawvals;
struct t_vpi_vecval *vectors;
struct t_vpi_time *times;
double *reals;
float *shortreals;
} value;
} s_vpi_arrayvalue, *p_vpi_arrayvalue;
The layout of the values to be set shall be specified by the calling routine by setting the format field in the
structure. In addition to the format types vpiIntVal, vpiVectorVal, vpiTimeVal, and vpiRealVal available
with vpi_get_value() function (Table38-3 in 38.15), the following format types can be used:
vpiRawFourStateVal
Values to be set for each element shall be specified in aval/bval format (similar to
4-state vectors) using the *rawvals field of the union above, interleaved
according to the following structure:
struct
{
PLI_BYTE8 avalbits[ngroups];
PLI_BYTE8 bvalbits[ngroups];
}
Each array element occupies ngroups*2 bytes stored consecutively as A/B byte
groups as shown above. For the first indexed array element, the avalbits
begins at rawvals[0], and the bvalbits at rawvals[ngroups],
respectively. The second array element’s avalbits begins at
rawvals[ngroups*2], and its bvalbits at rawvals[ngroups*3], etc.
ngroups is computed given the array element size in bits (= elemBits) as
follows:
int ngroups = (elemBits + 7) / 8;
The total storage required to hold “num” array elements shall be
ngroups * num * 2.
vpiRawTwoStateVal Values to be set shall be provided similarly to vpiRawFourStateVal above (also
using the *rawvals struct member), except that the bvalbits byte group shall
be omitted. ngroups shall be computed similarly also, but the total storage used
shall instead be ngroups * num.
vpiShortIntVal Values to be set will be provided as an array of “num” short(s), using the
*shortints field in the union in this case. This format is appropriate only for
arrays of vpiShortIntVar, vpiIntVar, or vpiLongIntVar elements.
vpiLongIntVal Values to be set will be stored as an array of “num” long(s), using the
*longints field in the union in this case. This format is appropriate for arrays
of vpiLongIntVar elements.
vpiShortRealVal Values to be set will be stored as an array of “num” floats, using the
*shortrealvals field in the union in this case. This format is appropriate only
for arrays of vpiShortRealVar elements.

<!-- Page 1131 -->

IEEE Std The format types vpiIntVal, vpiTimeVal, vpiVectorVal, and vpiRealVal that are also available with the
vpi_put_value() function correspond to similar union member names in s_vpi_arrayvalue (converted to
pointer values and ending in “s” to indicate they are arrays). For example, selecting the vpiIntVal format
shall cause an array of 32-bit integer values (set using the *integers field) to be loaded into the specified
section of the array object. The vpiVectorVal format shall assume that an array of consecutive A/B word
groups formatted according to the t_vpi_vecval structure (Figure38-8 in 38.15) is to be loaded. The
*vectors field should be used to provide these values. Given the array element size in bits (==
elemBits), the number of words of storage to provide data for num elements will be:
((elemBits + 31) / 32) * 2 * num
All other formats not mentioned here are unsupported and shall result in an error if specified. The
vpiRawFourStateVal format is appropriate for all 4-state array types (all net arrays, or variable arrays of
vpiLogicVar, vpiIntegerVar, vpiTimeVar, or 4-state packed vpiStructVar or vpiUnionVar elements).
The vpiRawTwoStateVal format is appropriate for all 2-state array types (variable arrays of vpiBitVar,
vpiByteVar, vpiShortInt, vpiInt, vpiLongInt, or 2-state packed vpiStructVar or vpiUnionVar elements).
If the vpiRawFourStateVal format is set for a 2-state array type, the bvalbits shall be ignored. If the
vpiRawTwoStateVal format is specified for a 4-state array type, the bvalbits shall be assumed to be 0.
The bit values in each array element, whether fixed or variable width, correspond to significance order in
avalbits and bvalbits. That is, the LSB of rawvals[0] and rawvals[ngroups] indicates the A and
B value of the LSB (0th) bit of the first array element, respectively, and the LSB of rawvals[1] and
rawvals[ngroups+1] indicates the A and B value of bit 8 of the first array element (if it is of width 9 bits
or greater), and so on.
The index_p argument is an array containing the indices of the starting element of the array object to be
retrieved. The indices are ordered in this array according to left-to-right order they would appear in an
expression in HDL text. The size of the index_p index array shall be equal to the number of unpacked
dimensions of obj, the array object.
The array element values will be set consecutively in order of the fastest varying index (rightmost unpacked
range of the array declaration), followed by more slowly varying indices accordingly until the number of
elements (num) has been loaded. Index values within each range are ordered from leftmost range value to
rightmost. For example, elements of an array a[2:0][3:5] with index_p[0] = 1 and
index_p[1] = 4 would be set in the order a[1][4], a[1][5], a[0][3], a[0][4], a[0][5],
respectively.
The flags field allows the following values to be set to control vpi_put_value_array() behavior:
vpiPropagateOff This flag inhibits notification of the fanouts of the array that one or more values
have changed. This reduces the performance impact of updating large numbers of
array elements. If this is used during active simulation, it may require that at least
one subsequent update event occurs for the array in order to achieve correct
simulation results.
vpiOneValue This flag set causes the function to apply only a single element value to the entire
array section specified. Data for only one element need be provided in the
s_vpi_arrayvalue structure.
The vpi_put_value_array() function does not allow the delay and event scheduling modes available in the
vpi_put_value() function (38.34). Its behavior is consistent with the vpiNoDelay mode specified there.
Flags other than vpiPropagateOff, vpiOneValue, or vpiNoDelay (the default) specified shall be an error.
When the vpi_put_value_array() function is called for an object of type vpiArrayNet, the values supplied
override the resolved values of the array net elements specified. These values shall remain in effect for each

<!-- Page 1132 -->

IEEE Std net element until one of the drivers of that element changes. When this occurs, the state of the net elements
shall be reevaluated according to the normal net resolution algorithms.
The following code shows an example of loading 5 elements of array a using the vpiRawFourStateVal
format. It takes 6 bytes of avalbits and 6 bytes of bvalbits to specify the 42-bit values for each element,
totaling 60 bytes for 5 elements. The index_p argument is set to start the loading at a[1][4].
/* Load 5 values into array "logic [41:0] a[2:0][3:5]":
* 1) 00000000000000000000000000000000000000000001
* 2) 00000000000000000000000000000000000100000001
* 3) 00000000000000000000000000010000000100000001
* 4) 00000000000000000001000000010000000100000001
* 5) 000000000001000000010000000100000001xxxxxxxx
* starting at "a[1][4]", given "arrH", a vpiHandle for "a". */
int indexArr[2];
PLI_BYTE8 *valueBuffer; /* Retain local ptr to mem allocated */
s_vpi_arrayvalue arrayVal = { 0, 0, NULL };
vpiHandle elemHdl, elemIter;
int elemWidth, ngroups, offset, bufsiz, elemInd;
int num = 5;
/* Get array element so we can get size to determine ngroups */
elemIter = vpi_iterate(vpiReg, arrH);
elemHdl = vpi_scan(elemIter);
elemWidth = vpi_get(vpiSize, elemHdl);
ngroups = (elemWidth + 7) / 8;
vpi_release_handle(elemIter);
arrayVal.format = vpiRawFourStateVal;
arrayVal.flags |= vpiPropagateOff; /* Disable value prop. */
/* Allocate storage and format the values. */
bufsiz = ngroups * 2 * num; /* Storage total for all values */
valueBuffer = (PLI_BYTE8 *) malloc(bufsiz);
arrayVal.value.rawvals = valueBuffer;
indexArr[0] = 1;
indexArr[1] = 4;
/* Set up the 5 values in valueBuffer */
offset = 0;
memset(valueBuffer, 0, bufsiz); /* Initialize value buffer */
for (elemInd = 1; elemInd <= num; elemInd++) {
for (int i = 0; i < elemInd; i++) {
valueBuffer[offset + i] = 1; /* Set LSB of Abits this byte */
}
offset += (ngroups * 2); /* Skip to beginning of next element */
}
/* Set final abits and bbits for final element 'x' values. */
offset -= (ngroups * 2); /* Back to beginning of last element */
valueBuffer[offset] = 0xff; /* Set avalbits to 1's */
valueBuffer[offset + ngroups] = 0xff; /* Set bvalbits to 1's */
/* Load values into "a" with propagation disabled. */
vpi_put_value_array(arrH, &arrayVal, indexArr, num);
free(valueBuffer);

<!-- Page 1133 -->

IEEE Std ### 38.36 vpi_register_cb()
vpi_register_cb()
Synopsis: Register simulation-related callbacks.
Syntax: vpi_register_cb(cb_data_p)
Type Description
Returns: vpiHandle Handle to the callback object.
Type Name Description
Arguments: p_cb_data cb_data_p Pointer to a structure with data about when callbacks
should occur and the data to be passed.
Related Use vpi_register_systf() to register callbacks for user-defined system tasks and system functions.
routines: Use vpi_remove_cb() to remove callbacks registered with vpi_register_cb().
The VPI routine vpi_register_cb() is used for registration of simulation-related callbacks to a user-provided
application for a variety of reasons during a simulation. The reasons for which a callback can occur are
divided into the following three categories:
— Simulation event
— Simulation time
— Simulation action or feature
How callbacks are registered for each of these categories is explained in this subclause.
The cb_data_p argument shall point to a s_cb_data structure, which is defined in vpi_user.h and given
in Figure38-17.
typedef struct t_cb_data
{
PLI_INT32 reason; /* callback reason */
PLI_INT32 (*cb_rtn)(struct t_cb_data *); /* call routine */
vpiHandle obj; /* trigger object */
p_vpi_time time; /* callback time */
p_vpi_value value; /* trigger object value */
PLI_INT32 index; /* index of the memory word or var select
that changed */
PLI_BYTE8 *user_data;
} s_cb_data, *p_cb_data;
Figure38-17—s_cb_data structure definition
For all callbacks, the reason field of the s_cb_data structure shall be set to a predefined constant, e.g.,
cbValueChange, cbAtStartOfSimTime, cbEndOfCompile. The reason constant shall determine when the
application shall be called back. See the vpi_user.h file listing in AnnexK and sv_vpi_user.h file in
AnnexM for a list of all callback reason constants.
The cb_rtn field of the s_cb_data structure shall be set to the application routine, which shall be invoked
when the simulator executes the callback. The uses of the remaining fields are detailed in 38.36.1 through
38.36.3.

<!-- Page 1134 -->

IEEE Std The callback routine shall be passed a pointer to an s_cb_data structure. This structure and all structures to
which it points belong to the simulator. If the application needs any of these data, it needs to copy the data
prior to returning from the callback routine.
#### 38.36.1 Simulation event callbacks
The vpi_register_cb() callback mechanism can be registered for callbacks to occur for simulation events,
such as value changes on certain objects, lifetime of dynamic data, and execution of a behavioral statement,
function call, or thread. When the cb_data_p->reason field is set to one of the following, the callback shall
occur as follows:
cbValueChange After value change on some variables, any expression, or terminal or after
execution of an event statement. Specifically excluded are class objects, dynamic
arrays, strings, queues, and associative arrays.
cbStmt Before execution of a behavioral statement.
cbForce/cbRelease After a force or release has occurred.
cbAssign/cbDeassign After a procedural assign or deassign statement has been executed.
cbDisable After a named block or task containing a system task or system function has been
disabled.
cbCreateObj After the class constructor call has completed and the internal state of a class
object has been initialized, or for shallow copy, after the copy operation has
completed.
cbReclaimObj Before the class object has been reclaimed by the automatic memory
management, when it has been marked as no longer being used. When control is
returned from this callback, any handles to this class object, its properties or their
subelements, and any associated callbacks should be considered invalid.
cbSizeChange After a dynamic array, associative array, queue, or string has been resized.
cbStartOfFrame Triggers when a frame is activated, i.e., when the associated task or function
begins execution. The frame’s automatic variables have been created and
initialized.
cbEndOfFrame Triggers when a frame’s associated task or function completes execution and
indicates that the frame is about to end. When control is returned from this
callback, any handles to this frame, its automatic variables, or their subelements
should be considered invalid.
cbStartOfThread Triggers whenever any thread is created.
cbEndOfThread Triggers when a particular thread gets deleted. All frames activated with this
thread will have already ended. Any outdated references made by the thread are
subject to deletion. When control is returned from this callback, any handles to
this thread, its out-of-scope references, or their subelements should be considered
invalid.
cbEnterThread Triggers whenever a particular thread resumes execution.
cbEndOfObject Triggers when a particular transient object is going to be deleted as a result of a
simulation event. Depending on the nature of the object, the semantics are
equivalent to cbReclaimObj, cbEndOfFrame, or cbEndOfThread, as
appropriate. In particular, when control is returned from this callback, any
handles to this object or its subelements should be considered invalid.
The following fields shall need to be initialized before passing the s_cb_data structure to
vpi_register_cb():
cb_data_p->obj This field shall be assigned a handle to an appropriate object, including class
typespec, frame, thread, variable including a class property, expression, terminal,

<!-- Page 1135 -->

IEEE Std or statement for which the callback shall occur. For cbCreateObj, this field shall
be assigned a handle to a class typespec object. For a cbReclaimObj, this field
shall be assigned either a handle to a class typespec or a class obj. With a class
typespec, any class object of that type shall generate a callback. For force and
release callbacks, if this is set to NULL, every force and release shall generate a
callback.
cb_data_p->time->type
This field shall be set to either vpiScaledRealTime or vpiSimTime, depending
on what time information the application requires during the callback. If
simulation time information is not needed during the callback, this field can be
set to vpiSuppressTime. For cbReclaimObj and cbEndOfObject, time
information is not passed to the callback routine; therefore, this field shall be
ignored.
cb_data_p->value->format
This field shall be set to one of the value formats indicated in Table38-5. If value
information is not needed during the callback, this field can be set to
vpiSuppressVal. For cbStmt callbacks, value information is not passed to the
callback routine; therefore, this field shall be ignored.
Table38-5—Value format field of cb_data_p->value->format
Format Registers a callback to return
vpiBinStrVal String of binary character(s) [1, 0, x, z]
vpiOctStrVal String of octal character(s) [0–7, x, X, z, Z]
vpiDecStrVal String of decimal character(s) [0–9]
vpiHexStrVal String of hex character(s) [0–f, x, X, z, Z]
vpiScalarVal vpi1, vpi0, vpiX, vpiZ, vpiH, vpiL
vpiIntVal Integer value of the handle
vpiRealVal Value of the handle as a double
vpiStringVal An ASCII string
vpiTimeVal Integer value of the handle using two integers
vpiVectorVal aval/bval representation of the value of the object
vpiStrengthVal Value plus strength information of a scalar object only
vpiObjTypeVal Return a value in the closest format of the object
When a simulation event callback occurs, the application shall be passed a single argument, which is a
pointer to an s_cb_data structure (this is not a pointer to the same structure that was passed to
vpi_register_cb()). The time and value information shall be set as directed by the time type and value
format fields in the call to vpi_register_cb(). The user_data field shall be equivalent to the user_data field
passed to vpi_register_cb(). The application can use the information in the passed structure and information
retrieved from other VPI routines to perform the desired callback processing.
cbValueChange callbacks can be placed onto event statements. When the event statement is executed, the
callback routine will be called. Because event statements do not have a value, when the callback routine is
called, the value field of the s_cb_data structure will be NULL.

<!-- Page 1136 -->

IEEE Std For a cbValueChange callback, if the obj has the vpiArrayMember property set to TRUE, the value in the
s_cb_data structure shall be the value of the array member that changed value. The index field shall
contain the index of the rightmost range of the array declaration. Use vpi_iterate(vpiIndex,obj) to find all
the indices.
The cbValueChange callback may be placed on a class var and will be called when its value changes, which
indicates that it is referring to a new dynamic object (including a newly constructed one) or no object. Its
value is opaque and cannot be obtained, and the value field of s_cb_data structure will be NULL. Its
vpiObjId property uniquely identifies what dynamic object, if any, a class var refers to.
If a cbValueChange callback is registered and the format is set to vpiStrengthVal, then the callback shall
occur whenever the object changes strength, including changes that do not result in a value change.
For a cbReclaimObj callback, there is no relationship to simulation time defined when automatic memory
management may occur. The time field of the s_cb_data structure will be NULL. The object field will
contain a valid handle to the class obj that is about to be reclaimed. The purpose of this callback is to allow
applications to clean up their data structures. All VPI properties of the class obj are accessible. Using this
handle as a reference for purposes of navigation or registering callbacks is undefined.
For cbForce, cbRelease, cbAssign, and cbDeassign callbacks, the object returned in the obj field shall be a
handle to the force, release, assign, or deassign statement. The value field shall contain the resultant value of
the left-hand expression. In the case of a release, the value field shall contain the value after the release has
occurred.
For a cbDisable callback, obj shall be a handle to a system task call, system function call, named begin,
named fork, task, or function.
It is illegal to attempt to place a callback for reason cbForce, cbRelease, or cbDisable on a variable
bit-select.
The following example shows an implementation of a simple monitor functionality for scalar nets, using a
simulation event callback:
setup_monitor(net)
vpiHandle net;
{
static s_vpi_time time_s = {vpiSimTime};
static s_vpi_value value_s = {vpiBinStrVal};
static s_cb_data cb_data_s =
{cbValueChange, my_monitor, NULL, &time_s, &value_s};
PLI_BYTE8 *net_name = vpi_get_str(vpiFullName, net);
cb_data_s.obj = net;
cb_data_s.user_data = malloc(strlen(net_name)+1);
strcpy(cb_data_s.user_data, net_name);
vpi_register_cb(&cb_data_s);
}
my_monitor(cb_data_p)
p_cb_data cb_data_p; {
vpi_printf("%d %d: %s = %s\n",
cb_data_p->time->high, cb_data_p->time->low,
cb_data_p->user_data,
cb_data_p->value->value.str);
}

<!-- Page 1137 -->

IEEE Std 38.36.1.1 Callbacks on individual statements
When cbStmt is used in the reason field of the s_cb_data structure, the other fields in the structure will be
defined as follows:
cb_data_p->cb_rtn The function to call before the given statement executes.
cb_data_p->obj A handle to the statement on which to place the callback (the allowable objects
are listed in Table38-6).
cb_data_p->time A pointer to an s_vpi_time structure, in which only the type is used, to
indicate the type of time that will be returned when the callback is made. This
type can be vpiScaledRealTime, vpiSimTime, or vpiSuppressTime if no time
information is needed by the callback routine.
cb_data_p->value Not used.
cb_data_p->index Not used.
cb_data_p->user_data Data to be passed to the callback function.
Just before the indicated statement executes, the indicated function will be called with a pointer to a new
s_cb_data structure, which will contain the following information:
cb_data_p->reason cbStmt.
cb_data_p->cb_rtn The same value as passed to vpi_register_cb().
cb_data_p->obj A handle to the statement which is about to execute.
cb_data_p->time A pointer to an s_vpi_time structure, which will contain the current
simulation time, of the type (vpiScaledRealTime or vpiSimTime) indicated in
the call to vpi_register_cb(). If the value in the call to vpi_register_cb() was
vpiSuppressTime, then the time pointer in the s_cb_data structure will be set
to NULL.
cb_data_p->value Always NULL.
cb_data_p->index Always set to 0.
cb_data_p->user_data The value passed in as user_data in the call to vpi_register_cb().
Multiple calls to vpi_register_cb() with the same data shall result in multiple callbacks.
Placing callbacks on statements that reside in protected portions of the code shall not be allowed and shall
cause vpi_register_cb() to return a NULL with an appropriate error message printed.
38.36.1.2 Behavior by statement type
Every possible object within the stmt class qualifies for having a cbStmt callback placed on it. Each
possible object is listed in Table38-6, for further clarification.
Table38-6—cbStmt callbacks
Object Description
vpiBegin One callback will occur prior to any of the statements within the block
vpiNamedBegin executing. The handle returned in the obj field will be the handle to the block
vpiFork object.
vpiNamedFork
vpiIf The callback will occur before the condition expression in the if statement is
vpiIfElse evaluated.
vpiWhile A callback will occur prior to the evaluation of the condition expression on
every iteration of the loop.

<!-- Page 1138 -->

IEEE Std Table38-6—cbStmt callbacks (continued)
Object Description
vpiRepeat A callback will occur when the repeat statement is first encountered and on
every subsequent iteration of the repeat loop.
vpiFor A callback will occur prior to any of the control expressions being evaluated.
Then on every iteration of the loop, a callback will occur prior to the evaluation
of the incremental statement.
vpiForever A callback will occur when the forever statement is first encountered and on
every subsequent iteration of the forever loop.
vpiWait The callback will occur before the statement executes.
vpiCase
vpiAssignment
vpiAssignStmt
vpiDeassign
vpiDisable
vpiForce
vpiRelease
vpiEventStmt
vpiDelayControl The callback will occur when the delay control is encountered, before the delay
occurs.
vpiEventControl The callback will occur when the event control is encountered, before the event
has occurred.
vpiTaskCall The callback will occur before the given task is executed.
vpiSysTaskCall
38.36.1.3 Registering callbacks on module-wide basis
vpi_register_cb() allows a handle to a module instance in the obj field of the s_cb_data structure. When
this is done, the effect will be to place a callback on every statement that can have a callback placed on it.
When using vpi_register_cb() on a module object, the call will return a handle to a single callback object
that can be passed to vpi_remove_cb() to remove the callback on every statement in the module instance.
Statements that reside in protected portions of the code shall not have callbacks placed on them.
#### 38.36.2 Simulation time callbacks
The vpi_register_cb() can register callbacks to occur for simulation time reasons, including callbacks at the
beginning or end of the execution of a particular time queue. The following time-related callback reasons are
defined:
cbAtStartOfSimTimeCallback shall occur before execution of events in a specified time queue. A
callback can be set for any time, even if no event is present.
cbNBASynch Callback shall occur immediately before the nonblocking assignment events are
processed.
cbReadWriteSynch Callback shall occur after execution of events for a specified time. This time
may be before or after nonblocking assignment events have been processed.
cbAtEndOfSimTime Callback shall occur after execution of nonblocking events, but before entering
the read-only phase of the time slice.
cbReadOnlySynch Callback shall occur the same as for cbReadWriteSynch, except that writing
values or scheduling events before the next scheduled event is not allowed.

<!-- Page 1139 -->

IEEE Std cbNextSimTime Callback shall occur before execution of events in the next event queue.
cbAfterDelay Callback shall occur after a specified amount of time, before execution of events
in a specified time queue. A callback can be set for any time, even if no event is
present.
For reason cbNextSimTime, the time field in the time structure is ignored. The following fields shall need
to be set before passing the s_cb_data structure to vpi_register_cb():
cb_data_p->time->type
This field shall be set to either vpiScaledRealTime or vpiSimTime, depending
on what time information the application requires during the callback.
vpiSuppressTime (or NULL for the cb_data_p->time field) will result in an
error.
cb_data_p->[time->low,time->high,time->real]
These fields shall contain the requested time of the callback or the delay before
the callback.
The following situations will generate an error, and no callback will be created:
— Attempting to place a cbAtStartOfSimTime callback with a delay of zero when simulation has
progressed into a time slice and the application is not currently within a cbAtStartOfSimTime
callback.
— Attempting to place a cbReadWriteSynch callback with a delay of zero at read-only synch time.
Placing a callback for cbAtStartOfSimTime and a delay of zero during a callback for reason
cbAtStartOfSimTime will result in another cbAtStartOfSimTime callback occurring during the same
time slice.
The value fields are ignored for all reasons with simulation time callbacks.
When the cb_data_p->time->type is set to vpiScaledRealTime, the cb_data_p->obj field shall be used as
the object for determining the time scaling.
When a simulation time callback occurs, the application callback routine shall be passed a single argument,
which is a pointer to an s_cb_data structure [this is not a pointer to the same structure that was passed to
vpi_register_cb()]. The time structure shall contain the current simulation time. The user_data field shall be
equivalent to the user_data field passed to vpi_register_cb().
The callback application can use the information in the passed structure and information retrieved from
other interface routines to perform the desired callback processing.
#### 38.36.3 Simulator action or feature callbacks
The vpi_register_cb() routine can register callbacks to occur for simulator action reasons or simulator
feature reasons. Simulator action reasons are callbacks such as the end of compilation or end of simulation.
Simulator feature reasons are tool-specific features, such as restarting from a saved simulation state or
entering an interactive mode. Actions are differentiated from features in that actions shall occur in all
VPI-compliant tools, whereas features might not exist in all VPI-compliant tools.
The following action-related callbacks shall be defined:
cbEndOfCompile End of simulation data structure compilation or build
cbStartOfSimulationStart of simulation (beginning of time zero simulation cycle)
cbEndOfSimulation End of simulation (simulation ended because no more events remain in the event
queue or a $finish system task executed)

<!-- Page 1140 -->

IEEE Std cbError Simulation run-time error occurred
cbPLIError Simulation run-time error occurred in a PLI function call
cbTchkViolation Timing check error occurred
cbSignal A signal occurred
Examples of possible feature-related callbacks are as follows:
cbStartOfSave Simulation save state command invoked
cbEndOfSave Simulation save state command completed
cbStartOfRestart Simulation restart from saved state command invoked
cbEndOfRestart Simulation restart command completed
cbStartOfReset Start of reset operation as defined by $reset system task
cbEndOfReset End of reset operation as defined by $reset system task.
cbEnterInteractive Simulation entering interactive debug mode (e.g., $stop system task executed)
cbExitInteractive Simulation exiting interactive mode
cbInteractiveScopeChange
Simulation command to change interactive scope executed
cbUnresolvedSystf Unknown user-defined system task or system function encountered
The only fields in the s_cb_data structure that shall need to be set up for simulation action or feature
callbacks are the reason, cb_rtn, and user_data (if desired) fields.
vpi_register_cb() can be used to set up a signal handler. To do this, set the reason field to cbSignal, and set
the index field to one of the legal signals specified by the operating system. When this signal occurs, the
simulator will trap the signal, proceed to a safe point (if possible), and then call the callback routine.
When a simulation action or feature callback occurs, the application routine shall be passed a pointer to an
s_cb_data structure. The reason field shall contain the reason for the callback. For cbTchkViolation
callbacks, the obj field shall be a handle to the timing check. For cbInteractiveScopeChange, obj shall be a
handle to the new scope. For cbUnresolvedSystf, user_data shall point to the name of the unresolved task
or system function. On a cbError callback, the routine vpi_chk_error() can be called to retrieve error
information.
The cbStartOfReset callback shall occur at the start of the $reset system task (see D.8), before the
simulation time has been reset to 0. The cbEndOfReset callback shall occur after all the activities of the
$reset system task have been completed, and in particular after $reset has reset the simulation time to
0and has restored the initial values of all variables and nets, but before the tool begins to execute the first
procedural statements in all initial and always procedures. Both callbacks shall occur whether the
$reset task has been invoked directly or whether it has been invoked indirectly through a call to
vpi_control(vpiReset, ...).
When an implementation restarts, the only VPI callbacks that shall exist are those for cbStartOfRestart and
cbEndOfRestart.
NOTE—When an application registers for these two callbacks, the user_data field should not be a pointer into memory.
The reason for this is that the executable used to restart an implementation may not be the exact same one used to save
the implementation state. A typical use of the user_data field for these two callbacks would be to store the identifier
returned from a call to vpi_put_data().
With the exception of cbStartOfRestart and cbEndOfRestart callbacks, when a restart occurs all
registered callbacks shall be removed.

<!-- Page 1141 -->

IEEE Std The following example shows a callback application that reports CPU usage at the end of a simulation. If the
application routine setup_report_cpu() is placed in the vlog_startup_routines list, it shall be
called just after the simulator is invoked.
static PLI_INT32 initial_cputime_g;
void report_cpu()
{
PLI_INT32 total = get_current_cputime() - initial_cputime_g;
vpi_printf("Simulation complete. CPU time used: %d\n", total);
}
void setup_report_cpu()
{
static s_cb_data cb_data_s = {cbEndOfSimulation, report_cpu};
initial_cputime_g = get_current_cputime();
vpi_register_cb(&cb_data_s);
}
### 38.37 vpi_register_systf()
vpi_register_systf()
Synopsis: Register user-defined system task or system function callbacks.
Syntax: vpi_register_systf(systf_data_p)
Type Description
Returns: vpiHandle Handle to the callback object.
Type Name Description
Arguments: p_vpi_systf_data systf_data_p Pointer to a structure with data about when callbacks
should occur and the data to be passed.
Related Use vpi_register_cb() to register callbacks for simulation events.
routines:
The VPI routine vpi_register_systf() shall register callbacks for user-defined system tasks or functions.
Callbacks can be registered to occur when a user-defined system task or system function is encountered
during compilation or execution of SystemVerilog source code.
The systf_data_p argument shall point to a s_vpi_systf_data structure, which is defined in vpi_user.h
and listed in Figure38-18.

<!-- Page 1142 -->

IEEE Std typedef struct t_vpi_systf_data
{
PLI_INT32 type; /* vpiSysTask, vpiSysFunc */
PLI_INT32 sysfunctype; /* vpi[Int,Real,Time,Sized,SizedSigned]Func */
PLI_BYTE8 *tfname; /* first character has to be '$' */
PLI_INT32 (*calltf)(PLI_BYTE8 *);
PLI_INT32 (*compiletf)(PLI_BYTE8 *);
PLI_INT32 (*sizetf)(PLI_BYTE8 *); /* for sized function
callbacks only */
PLI_BYTE8 *user_data;
} s_vpi_systf_data, *p_vpi_systf_data;
Figure38-18—s_vpi_systf_data structure definition
#### 38.37.1 System task and system function callbacks
User-defined SystemVerilog system tasks and system functions that use VPI routines can be registered with
vpi_register_systf(). The following system task and system function callbacks are defined:
The type field of the s_vpi_systf_data structure shall register the application to be a system task or a
system function. The type field value shall be an integer constant of vpiSysTask or vpiSysFunc.
The sysfunctype field of the s_vpi_systf_data structure shall define the type of value that a system
function shall return. The sysfunctype field shall be an integer constant of vpiIntFunc, vpiRealFunc,
vpiTimeFunc, vpiSizedFunc, or vpiSizedSignedFunc. This field shall only be used when the type field is
set to vpiSysFunc.
tfname is a character string containing the name of the system task or system function as it will be used in
SystemVerilog source code. The name shall begin with a dollar sign ($) and shall be followed by one or
more ASCII characters that are legal in SystemVerilog simple identifiers. These are the characters A
through Z, a through z, 0 through 9, underscore (_), and the dollar sign ($). The maximum name length shall
be the same as for SystemVerilog identifiers.
The compiletf, calltf, and sizetf fields of the s_vpi_systf_data structure shall be pointers to the user-
provided applications that are to be invoked by the system task or system function callback mechanism. One
or more of the compiletf, calltf, and sizetf fields can be set to NULL if they are not needed. Callbacks to the
applications pointed to by the compiletf and sizetf fields shall occur when the simulation data structure is
compiled or built (or for the first invocation if the system task or system function is invoked from an
interactive mode). Callbacks to the application pointed to by the calltf routine shall occur each time the
system task or system function is invoked during simulation execution.
The sizetf application shall only be called if the PLI application type is vpiSysFunc and the sysfunctype is
vpiSizedFunc or vpiSizedSignedFunc. If no sizetf is provided, a user-defined system function of type
vpiSizedFunc or vpiSizedSignedFunc shall return 32 bits.
The contents of the user_data field of the s_vpi_systf_data structure shall be the only argument passed
to the compiletf, sizetf, and calltf routines when they are called. This argument shall be of the type
“PLI_BYTE8 *”.
The following two examples illustrate allocating and filling in the s_vpi_systf_data structure and
calling the vpi_register_systf() function. These examples show two different C programming methods of
filling in the structure fields. A third method is shown in 38.37.3.

<!-- Page 1143 -->

IEEE Std /*
* VPI registration data for a $list_nets system task
*/
void listnets_register()
{
s_vpi_systf_data tf_data;
tf_data.type = vpiSysTask;
tf_data.tfname = "$list_nets";
tf_data.calltf = ListCall;
tf_data.compiletf = ListCheck;
vpi_register_systf(&tf_data);
}
/*
* VPI registration data for a $my_random system function
*/
void my_random_init()
{
s_vpi_systf_data func_data;
p_vpi_systf_data func_data_p = &func_data;
PLI_BYTE8 *my_workarea;
my_workarea = malloc(256);
func_data_p->type = vpiSysFunc;
func_data_p->sysfunctype= vpiSizedFunc;
func_data_p->tfname = "$my_random";
func_data_p->calltf = my_random;
func_data_p->compiletf = my_random_compiletf;
func_data_p->sizetf = my_random_sizetf;
func_data_p->user_data = my_workarea;
vpi_register_systf(func_data_p);
}
#### 38.37.2 Initializing VPI system task or system function callbacks
A means of initializing system task and system function callbacks and performing any other desired task just
after the simulator is invoked shall be provided by placing routines in a NULL-terminated static array,
vlog_startup_routines. A Cfunction using the array definition shall be provided as follows:
void (*vlog_startup_routines[]) ();
This C function shall be provided with a VPI-compliant tool. Entries in the array shall be added by the user.
The location of vlog_startup_routines and the procedure for linking vlog_startup_routines with
a tool shall be defined by the tool vendor.
NOTE—Callbacks can also be registered or removed at any time during an application routine, not just at start-up time.
This array of C functions shall be for registering system tasks and system functions. User-defined system
tasks and system functions that appear in a compiled description shall generally be registered by a routine in
this array.
The following example uses vlog_startup_routines to register the system task and system function
that were defined in the examples in 38.37.1.
A tool vendor shall supply a file that contains the vlog_startup_routines array. The names of the PLI
application register functions shall be added to this vendor-supplied file.

<!-- Page 1144 -->

IEEE Std extern void listnets_register();
extern void my_random_init();
void (*vlog_startup_routines[]) () =
{
listnets_register,
my_random_init,
}
#### 38.37.3 Registering multiple system tasks and system functions
Multiple system tasks and system functions can be registered at least two different ways, as follows:
— Allocate and define separate s_vpi_systf_data structures for each system task and system
function, and call vpi_register_systf() once for each structure. This is the method that was
used by the examples in 38.37.1 and 38.37.2.
— Allocate a static array of s_vpi_systf_data structures, and call vpi_register_systf() once for
each structure in the array. If the final element in the array is set to 0, then the calls to
vpi_register_systf() can be placed in a loop that terminates when it reaches the 0.
The following example uses a static structure to declare three system tasks and system functions and places
vpi_register_systf() in a loop to register them:
/*In a vendor tool file that contains vlog_startup_routines ...*/
extern void register_my_systfs();
extern void my_init();
void (*vlog_startup_routines[])() =
{
setup_report_cpu, /* user routine example in 38.36.3 */
register_my_systfs, /* user routine listed below */
0 /* shall be last entry in list */
}
/* In a user provided file... */
void register_my_systfs()
{
static s_vpi_systf_data systfTestList[] = {
{vpiSysTask, 0, "$my_task", my_task_calltf, my_task_comptf,0,0},
{vpiSysFunc, vpiIntFunc, "$my_int_func", my_int_func_calltf,
my_int_func_comptf, 0,0},
{vpiSysFunc, vpiSizedFunc, "$my_sized_func",
my_sized_func_calltf, my_sized_func_comptf,
my_sized_func_sizetf,0},
0};
p_vpi_systf_data systf_data_p = &(systfTestList[0]);
while (systf_data_p->type)
vpi_register_systf(systf_data_p++);
}

<!-- Page 1145 -->

IEEE Std ### 38.38 vpi_release_handle()
vpi_release_handle()
Synopsis: Release handle and its associated resources allocated by VPI routines.
Syntax: vpi_release_handle(obj)
Type Description
Returns: PLI_INT32 1 (true) on success; 0 (false) on failure.
Type Name Description
Arguments: vpiHandle obj Handle of an object.
Related
routines:
The VPI routine vpi_release_handle() shall free memory allocated for VPI handles. The SystemVerilog
tool may allocate memory when a handle to an object is obtained, although often all required memory has
been allocated when the underlying object was first created or elaborated. One may safely ignore calling
vpi_release_handle() when a handle is no longer needed, but it is always advisable to do so, provided the
handle is valid and will not automatically become invalid in the future. This avoids logical memory leaks.
vpi_release_handle() shall not be called on an invalid handle.
vpi_release_handle() may be used to free memory created for iterator objects. The iterator object shall
automatically be freed when vpi_scan() returns NULL because it has either completed an object traversal or
encountered an error condition. If neither of these conditions occurs (which can happen if the code breaks
out of an iteration loop before it has scanned every object), vpi_release_handle() should be called to free
any memory allocated for the iterator.
The routine shall return 1 (true) on success and 0 (false) on failure.
### 38.39 vpi_remove_cb()
vpi_remove_cb()
Synopsis: Remove a simulation-related callback registered with vpi_register_cb().
Syntax: vpi_remove_cb(cb_obj)
Type Description
Returns: PLI_INT32 1 (true) if successful; 0 (false) on a failure.
Type Name Description
Arguments: vpiHandle cb_obj Handle to the callback object.
Related Use vpi_register_cb() to register callbacks for simulation events.
routines:
The VPI routine vpi_remove_cb() shall remove callbacks that were registered with vpi_register_cb(). The
argument to this routine shall be a handle to the callback object. The routine shall return a 1 (true) if
successful and a 0 (false) on a failure. After vpi_remove_cb() is called with a handle to the callback, the
handle is no longer valid.

<!-- Page 1146 -->

IEEE Std ### 38.40 vpi_scan()
vpi_scan()
Synopsis: Scan the SystemVerilog hierarchy for objects with a one-to-many relationship.
Syntax: vpi_scan(itr)
Type Description
Returns: vpiHandle Handle to an object.
Type Name Description
Arguments: vpiHandle itr Handle to an iterator object returned from vpi_iterate().
Related Use vpi_iterate() to obtain an iterator handle.
routines: Use vpi_handle() to obtain handles to an object with a one-to-one relationship.
Use vpi_handle_multi() to obtain a handle to an object with a many-to-one relationship.
The VPI routine vpi_scan() shall traverse the instantiated SystemVerilog hierarchy and return handles to
objects as directed by the iterator itr. The iterator handle shall be obtained by calling vpi_iterate() for a
specific object type. Once vpi_scan() returns NULL, the iterator handle is no longer valid and cannot be
used again.
The following example application uses vpi_iterate() and vpi_scan() to display each net (including the size
for vectors) declared in the module. The example assumes it shall be passed a valid module handle.
void display_nets(mod)
vpiHandle mod;
{
vpiHandle net;
vpiHandle itr;
vpi_printf("Nets declared in module %s\n",vpi_get_str(vpiFullName, mod));
itr = vpi_iterate(vpiNet, mod);
while (net = vpi_scan(itr))
{
vpi_printf("\t%s", vpi_get_str(vpiName, net));
if (vpi_get(vpiVector, net))
{
vpi_printf(" of size %d\n", vpi_get(vpiSize, net));
}
else vpi_printf("\n");
}
}

<!-- Page 1147 -->

IEEE Std ### 38.41 vpi_vprintf()
vpi_vprintf()
Synopsis: Write to the output channel of the tool that invoked the PLI application and the current tool log file using
varargs that are already started.
Syntax: vpi_vprintf(format, ap)
Type Description
Returns: PLI_INT32 The number of characters written.
Type Name Description
Arguments: PLI_BYTE8 * format A format string using the C printf() format.
va_list ap An already started varargs list.
Related Use vpi_printf() to write a finite number of arguments.
routines: Use vpi_mcd_printf() to write to an opened file.
Use vpi_mcd_vprintf() to write a variable number of arguments to an opened file.
This routine performs the same function as vpi_printf(), except that varargs have already been started.

<!-- Page 1148 -->

IEEE Std 1800-2023