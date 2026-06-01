# Annex C: Annex C

## Annex C
(normative)
Deprecation
C.1 General
This annex identifies constructs that either
— have been deprecated from SystemVerilog and no longer appear in this standard, or
— are under consideration for deprecation and might be removed from future versions of this standard.
C.2 Constructs that have been deprecated
C.2.1 PLI TF and ACC routine libraries
IEEE Std 1364-2005 deprecated the Programming Language Interface (PLI) libraries containing the task/
function (TF) and access (ACC) routines that were contained in previous versions of that standard. These
routines were described in Clause 21 through Clause 25, Annex E, and Annex F of IEEE Std 1364-2001.
The text of these deprecated clauses and annexes do not appear in this version of the standard. The text can
be found in IEEE Std 1364-2001.
C.2.2 $sampled with a clocking event argument
IEEE Std 1800-2005 17.7.3 required that an explicit or inferred clocking event argument be provided for the
$sampled assertion system function. In this version of the standard, the semantics of $sampled have been
changed to a form that does not depend on a clocking event. Therefore the syntax for defining the clocking
event argument to $sampled is deprecated and does not appear in this version of the standard.
C.2.3 ended sequence method
IEEE Std 1800-2005 17.7.3 required using the sequence method ended in sequence expressions and the
sequence method triggered in other contexts. Since these two constructs have the same meaning but
mutually exclusive usage contexts, in this version of the standard, the triggered method is allowed to be
used in sequence expressions, and the usage of ended is deprecated and does not appear in this version of
the standard.
C.2.4 vpi_free_object()
The semantics of this VPI routine have been clarified to account for the nature of dynamic data in the
SystemVerilog information model and the concept of handle validity. It has been renamed
vpi_release_handle(), and thus vpi_free_object() has been deprecated.
C.2.5 Data read API
IEEE Std 1800-2009 deprecated the Data Read API that was contained in the previous version of the
standard. These routines were described in Clause 30 and Annex I of IEEE Std 1800-2005. The text of these

<!-- Page 1223 -->

IEEE Std deprecated clauses and annexes do not appear in this version of the standard. The text can be found in
IEEEStd 1800-2005.
C.2.6 Linked lists
IEEE Std 1800-2009 deprecated the built-in Linked List package that was contained in the previous version
of the standard. This package was described in Annex D of IEEE Std 1800-2005. The text of this deprecated
annex does not appear in this version of the standard. The text can be found in IEEE Std 1800-2005.
C.2.7 always statement in checkers
The always procedure in checkers was allowed by IEEE Std 1800-2009, but always_comb,
always_latch, and always_ff were forbidden. The limitations imposed on the always procedure in
checkers included the limitations imposed on always_ff procedures outside checkers. In this version of the
standard always_comb, always_latch, and always_ff have been added for checkers. As a result the
general always procedure in checkers would have imposed the same limitations as always_ff does.
Therefore the usage of always procedures in checkers is deprecated and does not appear in this version of
the standard.
C.2.8 Operator overloading
IEEE Std 1800-2017 deprecated the operator overloading construct that was contained in the previous
version of the standard. This construct was described in 11.11 and in the BNF syntax of IEEEStd1800-
## 2012. The text of this deprecated subclause and its syntax do not appear in this version of the standard. They
can be found in IEEE Std 1800-2012.
C.3 Accellera SystemVerilog 3.1a-compatible access to packed data
The Accellera SystemVerilog 3.1a [B4] semantics for svLogicPackedArrRef and svBitPackedArrRef
is deprecated. See H.14 for a full description of Accellera SystemVerilog 3.1a versus IEEE Std 1800
semantics.
C.4 Constructs identified for deprecation
NOTE—Certain SystemVerilog language features can be simulation inefficient, easily abused, and the source of design
problems. These features are being considered for removal from the SystemVerilog language if there is an alternate
method for these features.
The SystemVerilog language features that have been identified in this standard as ones that can be removed from
SystemVerilog are defparam and procedural assign/deassign.
C.4.1 Defparam statements
The defparam method of specifying the value of a parameter can be a source of design errors and can be an
impediment to tool implementation due to its usage of hierarchical paths. The defparam statement does not
provide a capability that cannot be done by another method that avoids these problems. Therefore, the
defparam statement is on a deprecation list. In other words, a future revision of IEEE Std 1800 might not
require support for this feature. This current standard still requires tools to support the defparam statement.
However, users are strongly encouraged to migrate their code to use one of the alternate methods of
parameter redefinition.

<!-- Page 1224 -->

IEEE Std Prior to the acceptance of IEEE Std 1364-2001 (Verilog-2001), it was common practice to change one or
more parameters of instantiated modules using a separate defparam statement. The defparam statements
can be a source of tool complexity and design problems.
A defparam statement can precede the instance to be modified, can follow the instance to be modified, can
be at the end of the file that contains the instance to be modified, can be in a separate file from the instance
to be modified, can modify parameters hierarchically that are in turn passed to other defparam statements
to modify, and can modify the same parameter from two different defparam statements (with undefined
results). Due to the many ways that a defparam can modify parameters, a SystemVerilog compiler cannot
resolve the final parameter values for an instance until after all of the design files are compiled.
Prior to IEEE Std 1364-2001, the only other method available to change the values of parameters on
instantiated modules was to use implicit inline parameter redefinition. This method uses
#(parameter_value) as part of the module instantiation. Implicit inline parameter redefinition syntax
requires that all parameters up to and including the parameter to be changed shall be placed in the correct
order and shall be assigned values.
IEEE Std 1364-2001 introduced explicit inline parameter redefinition, in the form #(.parameter_
name(value)), as part of the module instantiation. This method gives the capability to pass parameters by
name in the instantiation, which supplies all of the necessary parameter information to the model in the
instantiation itself.
The practice of using defparam statements is highly discouraged. Engineers are encouraged to take
advantage of the explicit inline parameter redefinition capability.
See 6.20 for more details on parameters.
C.4.2 Procedural assign and deassign statements
The procedural assign and deassign statements can be a source of design errors and can be an
impediment to tool implementation. The procedural assign and deassign statements do not provide a
capability that cannot be done by another method that avoids these problems. Therefore, the procedural
assign and deassign statements are on a deprecation list. In other words, a future revision of
IEEEStd1800 might not require support for these statements. This current standard still requires tools to
support the procedural assign and deassign statements. However, users are strongly encouraged to
migrate their code to use one of the alternate methods of procedural or continuous assignments.
SystemVerilog has two forms of the assign statement, as follows:
— Continuous assignments, placed outside any procedures
— Procedural continuous assignments, placed within a procedure
Continuous assignment statements are a separate process that are active throughout simulation. The
continuous assignment statement accurately represents combinational logic at an RTL level of modeling and
is frequently used.
Procedural continuous assignment statements become active when the assign statement is executed in the
procedure. The process can be deactivated using a deassign statement. The procedural assign and
deassign statements are seldom needed to model hardware behavior. In the unusual circumstances where
the behavior of procedural continuous assignments are required, the same behavior can be modeled using
the procedural force and release statements.

<!-- Page 1225 -->

IEEE Std Allowing the assign statement to be used both inside and outside a procedural block causes confusion and
is a source of errors in SystemVerilog models. The practice of using the assign and deassign statements
inside procedural blocks is highly discouraged.
C.4.3 VPI definitions
Certain object, relationship, and property definitions have been deprecated to implement corrections and
improvements to VPI. Some have been inherited from IEEE Std 1364 (see 36.12.1) and some have been
changed or removed to maintain consistency with related improvements.
1) vpiMemory (as an object type)
The vpiArrayVar (vpiRegArray) object type has been generalized to include vpiMemory and all
other arrays of variables. vpiMemory therefore no longer represents a VPI object type, except under
certain backwards compatibility modes (see 36.12.1). However, it is still in use as a relationship (see
### 37.20 diagram and detail 1).
2) vpiMemoryWord (as an object type)
This was exclusively used to represent elements of vpiMemory objects in IEEE Std 1364. Since
vpiArrayVar (vpiRegArray) has replaced the definition of vpiMemory, and variable object types
now represent their elements, this is represented by vpiLogicVar (vpiReg) types. Therefore, it no
longer represents a VPI object type, except under certain backwards compatibility modes (see
36.12.1). It is still in use as a relationship, however (see 37.20 diagram and detail 1).
3) vpiArray property
In IEEE Std 1364, variable types vpiIntegerVar, vpiTimeVar, and vpiRealVar could represent
single variable objects or arrays of those objects. The vpiArray property was required to distinguish
those cases (the property returned TRUE when they were arrays). Also, the property indicated when
vpiReg types represented elements of vpiRegArrays. These two uses became conflicting and
unnecessary when vpiRegArrays and arrays of integer, time, and real variables were
generalized as vpiArrayVar (vpiRegArray) objects. To distinguish when any variable is an
element of an array, the vpiArrayMember property is now used, thus replacing the original use of
vpiArray for reg types. The vpiArray property now has only limited use in IEEE Std 1364
backwards compatibility modes when vpiIntegerVar, vpiTimeVar, and vpiRealVar could
represent arrays (see 36.12.1).
4) vpiValid property
Significant revisions to VPI have rendered the original vpiValid property inconsistent with its
original purpose, which was to determine the extent to which a transient object represented by a VPI
handle was active or “alive” (see 37.2.4 and 37.3.7). Since the VPI object model no longer supports
maintaining handles to objects whose lifetimes have ended, such “validity” is implicit in their
existence, and their status needs to be determined by other means (see 38.36.1).
5) vpiInterfaceDecl one-to-many relationship
This relationship was used to return vpiRefObj objects representing virtual interface variables from
any scope. Its definition has been made equivalent to vpiVirtualInterfaceVar, which instead
returns vpiVirtualInterfaceVar object types. This was done to correctly reflect the true
variable-like characteristics of these objects (see 37.32 detail 11).

<!-- Page 1226 -->

IEEE Std 1800-2023