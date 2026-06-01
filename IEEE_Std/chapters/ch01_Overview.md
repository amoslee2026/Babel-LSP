# Chapter 1: Overview

## 1. Overview
### 1.1 Scope
This standard provides the definition of the language syntax and semantics for the IEEE Std 1800™
SystemVerilog language, which is a unified hardware design, specification, and verification language. The
standard includes support for behavioral, register transfer level (RTL), and gate-level hardware descriptions;
testbench, coverage, assertion, object-oriented, and constrained random constructs; and also provides
application programming interfaces (APIs) to foreign programming languages.
### 1.2 Purpose
This standard develops the IEEE Std 1800 SystemVerilog language in order to meet the increasing usage of
the language in specification, design, and verification of hardware. This revision corrects errors and clarifies
aspects of the language definition in IEEE Std 1800-2017.6 This revision also provides enhanced features
that ease design, improve verification, and enhance cross-language interactions.
### 1.3 Content summary
This standard serves as a complete specification of the SystemVerilog language. This standard contains the
following:
— The formal syntax and semantics of all SystemVerilog constructs
— Simulation system tasks and system functions, such as text output display commands
— Compiler directives, such as text substitution macros and simulation time scaling
— The Programming Language Interface (PLI) mechanism
— The formal syntax and semantics of the SystemVerilog Verification Procedural Interface (VPI)
— An Application Programming Interface (API) for coverage access not included in VPI
6Information on references can be found in Clause2.

<!-- Page 43 -->

IEEE Std — Direct programming interface (DPI) for interoperation with the C programming language
— VPI, API, and DPI header files
— Concurrent assertion formal semantics
— The formal syntax and semantics of standard delay format (SDF) constructs
— Informative usage examples
NOTE—An earlier standard, IEEE Std 1800-2009, represented a merger of two previous standards:
IEEEStd1364™-2005 and IEEE Std 1800-2005. In these previous standards, Verilog® was the base language and
defined a completely self-contained standard. SystemVerilog defined a number of significant extensions to Verilog, but
IEEE Std 1800-2005 was not a self-contained standard; IEEE Std 1800-2005 referred to, and relied on, IEEE Std 1364-
## 2005. These two standards were designed to be used as one language. Merging the base Verilog language into the
SystemVerilog standard enabled users to have all information regarding syntax and semantics in a single document.7, 8
### 1.4 Special terms
Throughout this standard, the following terms apply:
— SystemVerilog 3.1a refers to the Accellera SystemVerilog 3.1a Language Reference Manual [B4],
aprecursor to IEEE Std 1800-2005.9
— Verilog refers to IEEE Std 1364-2005 for the Verilog hardware description language (HDL).
— Language Reference Manual (LRM) refers to the document describing a Verilog or SystemVerilog
standard.
— Tool refers to a software implementation that reads SystemVerilog source code, such as a logic
simulator.
NOTE—In IEEE Std 1800-2005, SystemVerilog referred to just the extensions to the IEEE Std 1364-2005 Verilog
language and did not include the Verilog base language.
### 1.5 Conventions used in this standard
This standard is organized into clauses, each of which focuses on a specific area of the language. There are
subclauses within each clause to discuss individual constructs and concepts. The discussion begins with an
introduction and an optional rationale for the construct or the concept, followed by syntax and semantic
descriptions, followed by examples and notes.
The terminology conventions used throughout this standard are as follows:
— The word shall is used to indicate mandatory requirements strictly to be followed in order to
conform to the standard and from which no deviation is permitted (shall equals is required to).10,11
— The word should is used to indicate that among several possibilities one is recommended as
particularly suitable, without mentioning or excluding others; or that a certain course of action is
preferred but not necessarily required; or that (in the negative form) a certain course of action is
deprecated but not prohibited (should equals is recommended that).
— The word may is used to indicate a course of action permissible within the limits of the standard
(may equals is permitted to).
— The word can is used for statements of possibility and capability, whether material, physical, or
causal (can equals is able to).
7Notes in text, tables, and figures are given for information only and do not contain requirements needed to implement the standard.
8Verilog is a registered trademark of Cadence Design Systems, Inc.
9The numbers in square brackets correspond to those of the bibliography in AnnexQ.
10 The use of the word must is deprecated and cannot be used when stating mandatory requirements; must is used only to describe
unavoidable situations.
11 The use of will is deprecated and cannot be used when stating mandatory requirements; will is only used in statements of fact.

<!-- Page 44 -->

IEEE Std ### 1.6 Syntactic description
The main text uses the following conventions:
— Italicized font for syntactic categories (see AnnexA) or when a term is being defined
— Constant-width font for examples, file names, and references to constants, especially 0, 1, x, and
z values
— Boldface constant-width font for SystemVerilog keywords, when referring to the actual
keyword
The formal syntax of SystemVerilog is described using Backus-Naur Form (BNF). The following
conventions are used:
— Lowercase words, some containing embedded underscores, denote syntactic categories. For
example:
module_declaration
— Boldface-red characters denote reserved keywords, operators, and punctuation marks as a
required part of the syntax. For example:
module => ;
— A vertical bar (|) that is not in boldface-red separates alternative items. For example:
unary_operator::= + | - | ! | ~ | & | ~& | | | ~| | ^ | ~^ | ^~
— Square brackets ([]) that are not in boldface-red enclose optional items. For example:
net_port_header ::= [ port_direction ] net_port_type
— Braces ({}) that are not in boldface-red enclose a repeated item. The item may appear zero or more
times; the repetitions occur from left to right as with an equivalent left-recursive rule. Thus, the
following two rules are equivalent:
list_of_param_assignments ::= param_assignment { , param_assignment }
list_of_param_assignments ::=
param_assignment
| list_of_param_assignments , param_assignment
### 1.7 Use of color in this standard
This standard uses a minimal amount of color to enhance readability. The coloring is not essential and does
not affect the accuracy of this standard when viewed in pure black and white. The places where color is used
are the following:
— Cross references that are hyperlinked to other portions of this standard are shown in underlined-blue
text (hyperlinking works when this standard is viewed interactively as a PDF file).
— Syntactic keywords and tokens in the formal language definitions are shown in boldface-red
text.
— Some figures use a minimal amount of color to enhance readability.
### 1.8 Contents of this standard
A synopsis of the clauses and annexes is presented as a quick reference. All clauses and several of the
annexes are normative parts of this standard. Some annexes are included for informative purposes only.

<!-- Page 45 -->

IEEE Std Part One: Design and Verification Constructs
Clause1 describes the contents of this standard and the conventions used in this standard.
Clause2 lists references to other standards that are required in order to implement this standard.
Clause3 introduces the major building blocks that make up a SystemVerilog design and verification
environment: modules, programs, interfaces, checkers, packages, and configurations. This clause also
discusses primitives, name spaces, the $unit compilation space, and the concept of simulation time.
Clause4 describes the SystemVerilog simulation scheduling semantics.
Clause5 describes the lexical tokens used in SystemVerilog source text and their conventions.
Clause6 describes SystemVerilog data objects and types, including nets and variables, their declaration
syntax and usage rules, and charge strength of the values on nets. This clause also discusses strings and
string methods, enumerated types, user-defined types, constants, data scope and lifetime, and type
compatibility.
Clause7 describes SystemVerilog compound data types: structures, unions, arrays, including packed and
unpacked arrays, dynamic arrays, associative arrays, and queues. This clause also describes various array
methods.
Clause8 describes the object-oriented programming capabilities in SystemVerilog. Topics include defining
classes, interface classes, dynamically constructing objects, inheritance and subclasses, data hiding and
encapsulation, polymorphism, and parameterized classes.
Clause9 describes the SystemVerilog procedural blocks: initial, always, always_comb, always_ff,
always_latch, and final. Sequential and parallel statement grouping, block names, statement labels, and
process control are also described.
Clause10 describes continuous assignments, blocking and nonblocking procedural assignments, and
procedural continuous assignments.
Clause11 describes the operators and operands that can be used in expressions.
Clause12 describes SystemVerilog procedural programming statements, such as decision statements and
looping constructs.
Clause13 describes tasks and functions, which are subroutines that can be called from more than one place
in a behavioral model.
Clause14 defines clocking blocks, input and output skews, cycle delays, and default clocking.
Clause15 describes interprocess communications using event types and event controls, and built-in
semaphore and mailbox classes.
Clause16 describes immediate and concurrent assertions, properties, sequences, sequence operations,
multiclock sequences, and clock resolution.
Clause17 describes checkers. Checkers allow the encapsulation of assertions and modeling code to create a
single verification entity.

<!-- Page 46 -->

IEEE Std Clause18 describes generating random numbers, constraining random number generation, dynamically
changing constraints, seeding random number generators (RNGs), and randomized case statement
execution.
Clause19 describes coverage groups, coverage points, cross coverage, coverage options, and coverage
methods.
Clause20 describes most of the built-in system tasks and system functions.
Clause21 describes additional system tasks and system functions that are specific to input/output (I/O)
operations.
Clause22 describes various compiler directives, including a directive for controlling reserved keyword
compatibility between versions of previous Verilog and SystemVerilog standards.
Part Two: Hierarchy Constructs
Clause23 describes how hierarchies are created in SystemVerilog using module instances and interface
instances, and port connection rules. This clause also discusses the $root top-level instances, nested
modules, extern modules, identifier search rules, how parameter values can be overridden, and binding
auxiliary code to scopes or instances.
Clause24 describes the testbench program construct, the elimination of testbench race conditions, and
program control tasks.
Clause25 describes interface syntax, interface ports, modports, interface subroutines, parameterized
interfaces, virtual interfaces, and accessing objects within interfaces.
Clause26 describes user-defined packages and the std built-in package.
Clause27 describes the generate construct and how generated constructs can be used to do conditional or
multiple instantiations of procedural code or hierarchy.
Clause28 describes the gate- and switch-level primitives and logic strength modeling.
Clause29 describes how a user-defined primitive (UDP) can be defined and how these primitives are
included in SystemVerilog models.
Clause30 describes how to specify timing relationships between input and output ports of a module.
Clause31 describes how timing checks are used in specify blocks to determine whether signals obey the
timing constraints.
Clause32 describes the syntax and semantics of SDF constructs.
Clause33 describes how to configure the contents of a design.
Clause34 describes encryption and decryption of source text regions.

<!-- Page 47 -->

IEEE Std Part Three: Application Programming Interfaces
Clause35 describes SystemVerilog’s direct programming interface (DPI), a direct interface to foreign
languages and the syntax for importing functions from a foreign language and exporting subroutines to a
foreign language.
Clause36 provides an overview of the programming language interface (PLI and VPI).
Clause37 presents the VPI data model diagrams, which document the VPI object relationships and access
methods.
Clause38 describes the VPI routines.
Clause39 describes the assertion API in SystemVerilog.
Clause40 describes the coverage API in SystemVerilog.
Part Four: Annexes
AnnexA (normative) defines the formal syntax of SystemVerilog, using BNF. Subclause A.10 includes
additional normative text to clarify specific details of BNF productions defined in A.1 through A.9.
AnnexB (normative) lists the SystemVerilog keywords.
AnnexC (informative) lists constructs that have been deprecated from SystemVerilog. The annex also
discusses the possible deprecation of the defparam statement and the procedural assign/deassign
statements.
AnnexD (informative) describes system tasks and system functions that are frequently used, but that are not
required in this standard.
AnnexE (informative) describes compiler directives that are frequently used, but that are not required in this
standard.
AnnexF (normative) describes a formal semantics for SystemVerilog concurrent assertions.
AnnexG (normative) describes the SystemVerilog standard package, containing type definitions for
mailbox, semaphore, randomize, process, and weak reference.
AnnexH (normative) defines the C language layer for the SystemVerilog DPI.
AnnexI (normative) defines the standard svdpi.h include file for use with SystemVerilog DPI
applications.
AnnexJ (normative) describes common guidelines for the inclusion of foreign language code into a
SystemVerilog application.
AnnexK (normative) provides a listing of the contents of the vpi_user.h file.
AnnexL (normative) provides a listing of the contents of the vpi_compatibility.h file, which extends
the vpi_user.h include file.
AnnexM (normative) provides a listing of the contents of the sv_vpi_user.h file, which extends the
vpi_user.h include file.

<!-- Page 48 -->

IEEE Std AnnexN (normative) provides the C source code for the SystemVerilog random distribution system
functions.
AnnexO (informative) describes various scenarios that can be used for intellectual property (IP) protection,
and it also shows how the relevant pragmas can be used to achieve the desired effect of securely protecting,
distributing, and decrypting the model.
AnnexP (informative) defines terms that are used in this standard.
AnnexQ (informative) lists reference documents that are related to this standard.
### 1.9 Deprecated clauses
AnnexC lists constructs that appeared in previous versions of either IEEE Std 1364 or IEEE Std 1800, but
that have been deprecated and do not appear in this standard. This annex also lists constructs that appear in
this standard, but that are under consideration for deprecation in a future version of this standard.
### 1.10 Examples
Small SystemVerilog code examples are shown throughout this standard. These examples are informative.
They are intended to illustrate the usage of SystemVerilog constructs in a simple context and do not define
the full syntax.
### 1.11 Prerequisites
Some clauses of this standard presuppose a working knowledge of the C programming language.

<!-- Page 49 -->

IEEE Std 1800-2023