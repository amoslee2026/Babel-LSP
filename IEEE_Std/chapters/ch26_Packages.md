# Chapter 26: Packages

## 26. Packages
### 26.1 General
This clause describes the following:
— Package declarations
— Referencing data within packages
— Package search order rules
— Exporting imported names from packages
— The std built-in package
### 26.2 Package declarations
SystemVerilog packages provide an additional mechanism for sharing parameters, data, type, task, function,
sequence, property, and checker declarations among multiple SystemVerilog modules, interfaces, programs,
and checkers.
Packages are explicitly named scopes appearing at the outermost level of the source text (at the same level as
top-level modules and primitives). Types, nets, variables, tasks, functions, sequences, properties, and
checkers may be declared within a package. Such declarations may be referenced within modules,
interfaces, programs, checkers, and other packages by either import or fully resolved name.
Packages may contain processes inside checkers only. Therefore, net declarations with implicit continuous
assignments are not allowed.
package_declaration ::= // from A.1.2
{ attribute_instance } package [ lifetime ] package_identifier ;
[ timeunits_declaration ] { { attribute_instance } package_item }
endpackage [ : package_identifier ]
package_item ::= // from A.1.11
package_or_generate_item_declaration
| anonymous_program
| package_export_declaration
| timeunits_declaration3
package_or_generate_item_declaration ::=
net_declaration
| data_declaration
| task_declaration
| function_declaration
| checker_declaration
| dpi_import_export
| extern_constraint_declaration
| class_declaration
| interface_class_declaration
| class_constructor_declaration
| local_parameter_declaration ;
| parameter_declaration ;
| covergroup_declaration
| assertion_item_declaration
| ;

<!-- Page 809 -->

IEEE Std anonymous_program ::= program ; { anonymous_program_item } endprogram
anonymous_program_item ::=
task_declaration
| function_declaration
| class_declaration
| interface_class_declaration
| covergroup_declaration
| class_constructor_declaration
| ;
3) A timeunits_declaration shall be legal as a non_port_module_item, non_port_interface_item,
non_port_program_item, or package_item only if it repeats and matches a previous timeunits_declaration within
the same time scope.
Syntax26-1—Package declaration syntax (excerpt from AnnexA)
The package declaration creates a scope that contains declarations intended to be shared among one or
more compilation units, modules, interfaces, or programs. Items within packages are generally type
definitions, tasks, and functions. Items within packages shall not have hierarchical references to identifiers
except those created within the package or made visible by import of another package. A package shall not
refer to items defined in the compilation-unit scope. (See 3.12.1.) It is also possible to populate packages
with parameters, variables, and nets. This may be useful for global items that are not conveniently passed
down through the hierarchy. Variable declaration assignments within the package shall occur before any
initial or always procedures are started, in the same way as variables declared in a compilation unit or
module.
The following is an example of a package:
package ComplexPkg;
typedef struct {
shortreal i, r;
} Complex;
function Complex add(Complex a, b);
add.r = a.r + b.r;
add.i = a.i + b.i;
endfunction
function Complex mul(Complex a, b);
mul.r = (a.r * b.r) - (a.i * b.i);
mul.i = (a.r * b.i) + (a.i * b.r);
endfunction
endpackage : ComplexPkg
### 26.3 Referencing data in packages
The compilation of a package shall precede the compilation of scopes in which the package is imported.
One way to use declarations made in a package is to reference them using the package scope resolution
operator ::.
ComplexPkg::Complex cout = ComplexPkg::mul(a, b);
An alternate method for utilizing package declarations is via the import declaration (see Syntax26-2).

<!-- Page 810 -->

IEEE Std data_declaration ::= // from A.2.1.3
...
| package_import_declaration15
...
package_import_declaration ::=
import package_import_item { , package_import_item } ;
package_import_item ::=
package_identifier :: identifier
| package_identifier :: *
15) It shall be illegal to have an import statement directly within a class scope.
Syntax26-2—Package import syntax (excerpt from AnnexA)
The import declaration provides direct visibility of identifiers within packages. It allows identifiers
declared within packages to be visible within the current scope without a package name qualifier. Two
forms of the import declaration are provided: explicit import and wildcard import. Explicit import allows
control over precisely which symbols are imported:
import ComplexPkg::Complex;
import ComplexPkg::add;
An explicit import only imports the symbols specifically referenced by the import.
In the following example, the import of the enumeration type teeth_t does not import the enumeration
literals ORIGINAL and FALSE. In order to refer to the enumeration literal FALSE from package q, either add
import q::FALSE or use a full package reference as in teeth = q::FALSE;.
package p;
typedef enum { FALSE, TRUE } bool_t;
endpackage
package q;
typedef enum { ORIGINAL, FALSE } teeth_t;
endpackage
module top1 ;
import p::*;
import q::teeth_t;
teeth_t myteeth;
initial begin
myteeth = q:: FALSE; // OK:
myteeth = FALSE; // ERROR: Direct reference to FALSE refers to the
end // FALSE enumeration literal imported from p
endmodule
module top2 ;
import p::*;
import q::teeth_t, q::ORIGINAL, q::FALSE;
teeth_t myteeth;

<!-- Page 811 -->

IEEE Std initial begin
myteeth = FALSE; // OK: Direct reference to FALSE refers to the
end // FALSE enumeration literal imported from q
endmodule
An explicit import shall be illegal if the imported identifier is declared in the same scope or explicitly
imported from another package. Importing an identifier from the same package multiple times is allowed.
A wildcard import allows all identifiers declared within a package to be imported provided the identifier is
not otherwise defined in the importing scope: A wildcard import is of the following form:
import ComplexPkg::*;
An identifier is potentially locally visible at some point within a scope if there is a wildcard import of a
package before that point within the current scope and the package contains a declaration of that identifier.
An identifier is locally visible at some point within a scope if
a) The identifier denotes a nested scope within the current scope, or
b) The identifier is declared as an identifier prior to that point within the current scope, or
c) The identifier is visible from an explicit import prior to that point within the current scope.
A potentially locally visible identifier from a wildcard import may become locally visible if the resolution of
a reference to an identifier finds no other matching locally visible identifiers.
For a reference to an identifier other than function or task call, the locally visible identifiers defined at the
point of the reference in the current scope shall be searched. If the reference is a function or task call, all of
the locally visible identifiers to the end of the current scope shall be searched. If a match is found, the
reference shall be bound to that locally visible identifier.
If no locally visible identifiers match, then the potentially locally visible identifiers defined prior to the point
of the reference in the current scope shall be searched. If a match is found, that identifier from the package
shall be imported into the current scope, becoming a locally visible identifier within the current scope, and
the reference shall be bound to that identifier.
If the reference is not found within the current scope, the next outer lexical scope shall be searched; first
from among the locally visible identifiers in that scope and then from among the potentially locally visible
identifiers defined prior to the point of the reference. If a match is found among the potentially locally
visible identifiers, that identifier from the package shall be imported into the outer scope, becoming a locally
visible identifier within the outer scope.
If a wildcard imported symbol is made locally visible in a scope, any later locally visible declaration of the
same name in that scope shall be illegal.
The search algorithm shall be repeated for each outer lexical scope until an identifier is found that matches
the reference or there are no more outer lexical scopes, the compilation-unit scope being the final scope
searched. For a reference to an identifier other than function or task call, it shall be illegal if no identifier can
be found that matches the reference. If the reference is a function or task call, the search continues using
upwards hierarchical identifier resolution (see 23.8.1).
It shall be illegal if the wildcard import of more than one package within the same scope defines the same
potentially locally visible identifier and a search for a reference matches that identifier.

<!-- Page 812 -->

IEEE Std Example 1:
package p;
int x;
endpackage
module top;
import p::*; // line 1
if (1) begin : b
initial x = 1; // line 2
int x; // line 3
initial x = 1; // line 4
end
int x; // line 5
endmodule
The reference in line 2 causes the potentially locally visible x from wildcard import p::* ( p::x ) to
become locally visible in scope top, and line 2 initializes p::x. Line 4 initializes top.b.x. Line 5 is illegal
since it is a local declaration in scope top, which conflicts with the name x imported from p, which had
already become a locally visible declaration.
Example 2:
package p;
int x;
endpackage
package p2;
int x;
endpackage
module top;
import p::*; // line 1
if (1) begin : b
initial x = 1; // line 2
import p2::*; // line 3
end
endmodule
Line 2 causes the import of p::x in scope top because the wildcard import p::* is in the outer scope top
and precedes the occurrence of x. The declaration x from package p becomes locally visible in scope top.
Example 3:
package p;
function int f();
return 1;
endfunction
endpackage
module top;
int x;
if (1) begin : b
initial x = f(); // line 2
import p::*; // line 3
end

<!-- Page 813 -->

IEEE Std function int f();
return 1;
endfunction
endmodule
f() on line 2 binds to top.f and not to p::f since the import is after the function call reference.
Example 4:
package p;
function int f();
return 1;
endfunction
endpackage
package p2;
function int f();
return 1;
endfunction
endpackage
module top;
import p::*;
int x;
if (1) begin : b
initial x = f(); // line 1
end
import p2::*;
endmodule
Since f is not found in scope b, the rules require inspection of all wildcard imports in the parent scope.
There are two wildcard imports, but only the wildcard import p::* that is lexically preceding the occurrence
of f() is considered. In this case, f binds to p::f.
The effect of importing an identifier into a scope makes that identifier visible without requiring access using
the scope resolution operator. Importing does not copy the declaration of that identifier into the importing
scope. The imported identifier shall not be visible outside that importing scope by hierarchical reference into
that scope or by interface port reference into that scope.
It shall be illegal to have an import statement directly within a class scope.
### 26.4 Using packages in module headers
Package items can be referenced in module, interface or program parameter and port declarations by
importing the package as part of the header to the module, interface, or program declaration. The syntax is
shown in Syntax26-3.
module_nonansi_header ::= // from A.1.2
{ attribute_instance } module_keyword [ lifetime ] module_identifier
{ package_import_declaration } [ parameter_port_list ] list_of_ports ;
module_ansi_header ::=
{ attribute_instance } module_keyword [ lifetime ] module_identifier
{ package_import_declaration }1 [ parameter_port_list ] [ list_of_port_declarations ] ;

<!-- Page 814 -->

IEEE Std interface_nonansi_header ::=
{ attribute_instance } interface [ lifetime ] interface_identifier
{ package_import_declaration } [ parameter_port_list ] list_of_ports ;
interface_ansi_header ::=
{ attribute_instance } interface [ lifetime ] interface_identifier
{ package_import_declaration }1 [ parameter_port_list ] [ list_of_port_declarations ] ;
program_nonansi_header ::=
{ attribute_instance } program [ lifetime ] program_identifier
{ package_import_declaration } [ parameter_port_list ] list_of_ports ;
program_ansi_header ::=
{ attribute_instance } program [ lifetime ] program_identifier
{ package_import_declaration }1 [ parameter_port_list ] [ list_of_port_declarations ] ;
1) A package_import_declaration in a module_ansi_header, interface_ansi_header, or program_ansi_header shall
be followed by a parameter_port_list or list_of_port_declarations, or both.
Syntax26-3—Package import in header syntax (excerpt from AnnexA)
Package items that are imported as part of a module, interface, or program header are visible throughout the
module, interface, or program, including in parameter and port declarations.
For example:
package A;
typedef struct {
bit [ 7:0] opcode;
bit [23:0] addr;
} instruction_t;
endpackage: A
package B;
typedef enum bit {FALSE, TRUE} boolean_t;
endpackage: B
module M import A::instruction_t, B::*;
#(WIDTH = 32)
(input [WIDTH-1:0] data,
input instruction_t a,
output [WIDTH-1:0] result,
output boolean_t OK
);
...
endmodule: M
### 26.5 Search order rules
Table26-1 describes the search order rules for the declarations imported from a package. For the purposes
of the discussion that follows, consider the following package declarations:
package p;
typedef enum { FALSE, TRUE } BOOL;
const BOOL c = FALSE;
endpackage

<!-- Page 815 -->

IEEE Std package q;
const int c = 0;
endpackage
Table26-1—Scoping rules for package importation
In a scope In a scope not In a scope In a scope
containing a containing a containing an containing a
Example Description local local explicit import wildcard
declaration of declaration of of c (import import of c
c c q::c) (import q::*)
u = p::c; A qualified OK OK OK OK
y = p::TRUE; package identi-
fier is visible in Direct reference Direct reference Direct reference Direct reference
any scope to c refers to to c is illegal to c refers to the to c refers to the
(without the the locally because it is c imported from c imported from
need for an declared c. undefined. q. q.
import clause).
p::c refers to p::c refers to p::c refers to p::c refers to
the c in pack- the c in the c in package the c in package
age p. package p. p. p.
import p::*; All declara- OK OK OK OK / ERROR
tions inside
. . . package p Direct reference Direct reference Direct reference c is undefined in
become poten- to c refers to to c refers to to c refers to the the importing
y = FALSE; tially directly the locally the c imported c imported from scope. Thus, a
visible in the declared c. from package package q. direct reference
importing p. to c is illegal
scope: Direct reference and results in an
– c to other identi- error.
– BOOL fiers (e.g.,
– FALSE FALSE) refers The import
– TRUE to those implic- clause is other-
itly imported wise allowed.
from package
p.
import p::c; The imported ERROR OK ERROR OK / ERROR
identifier c
. . . becomes It is illegal to Direct reference It is illegal to The import of
directly visible import an iden- to c refers to import the same p::c makes
if( ! c ) ... in the import- tifier defined in the c imported identifier from any prior
ing scope. the importing from package different reference to c
scope. p. packages. illegal.
Otherwise,
direct reference
to c refers to the
c imported from
package p.
When using a wildcard import, a reference to an undefined identifier that is declared within the package
causes that identifier to be imported into the scope of the import statement. However, an error results if the
same identifier is later declared or explicitly imported in the same scope. This is shown in the following
example:
module m;
import q::*;

<!-- Page 816 -->

IEEE Std wire a = c; // This statement forces the import of q::c;
import p::c; // The conflict with q::c and p::c creates an error.
endmodule
### 26.6 Exporting imported names from packages
By default, declarations imported into a package are not visible by way of subsequent imports of that
package. Package export declarations allow a package to specify that imported declarations are to be made
visible in subsequent imports. A package export may precede a corresponding package import.
The syntax for package exports is shown in Syntax26-4.
package_export_declaration ::= // from A.2.1.3
export *::* ;
| export package_import_item { , package_import_item } ;
Syntax26-4—Package export syntax (excerpt from AnnexA)
An export of the form package_name::* exports all declarations that were actually imported from
package_name within the context of the exporting package. All names from package_name, whether
imported directly or through wildcard imports, are made available. Symbols that are candidates for import
but not actually imported are not made available. The special wildcard export form, export *::*; ,
exports all imported declarations from all packages from which imports occur.
An export of the form package_name::name makes the given declaration available. It shall be an error if
the given declaration is not a candidate for import or if the declaration is not actually imported in the
package. The declaration being exported shall be imported from the same package_name used in the
export. If the declaration is an unreferenced candidate for import, the export shall be considered to be a
reference and shall import the declaration into the package following the same rules as for a direct import of
the name.
An import of a declaration made visible through an export is equivalent to an import of the original
declaration. Thus direct or wildcard import of a declaration by way of multiple exported paths does not
cause conflicts.
It is valid to specify multiple exports that export the same actual declaration.
Examples:
package p1;
int x, y;
endpackage
package p2;
import p1::x;
export p1::*; // exports p1::x as the name "x";
// p1::x and p2::x are the same declaration
endpackage
package p3;
import p1::*;
import p2::*;
export p2::*;

<!-- Page 817 -->

IEEE Std int q = x;
// p1::x and q are made available from p3. Although p1::y
// is a candidate for import, it is not actually imported
// since it is not referenced. Since p1::y is not imported,
// it is not made available by the export.
endpackage
package p4;
import p1::*;
export p1::*;
int y = x; // y is available as a direct declaration;
// p1::x is made available by the export
endpackage
package p5;
import p4::*;
import p1::*;
export p1::x;
export p4::x; // p4::x refers to the same declaration
// as p1::x so this is legal.
endpackage
package p6;
import p1::*;
export p1::x;
int x; // Error. export p1::x is considered to
// be a reference to "x" so a subsequent
// declaration of x is illegal.
endpackage
package p7;
int y;
endpackage
package p8;
export *::*; // Exports both p7::y and p1::x.
import p7::y;
import p1::x;
endpackage
module top;
import p2::*;
import p4::*;
int y = x; // x is p1::x
endmodule
### 26.7 The std built-in package
SystemVerilog provides a built-in package that can contain system types (e.g., classes), variables, tasks, and
functions. Users cannot insert additional declarations into the built-in package.
The contents of the standard built-in package are defined in AnnexG.

<!-- Page 818 -->

IEEE Std The built-in package is implicitly wildcard imported into the compilation-unit scope of every compilation
unit (see 3.12.1). Thus, declarations in the built-in package are directly available in any other scope (like
system tasks and system functions) unless they are redefined by user code.
Declarations in the standard built-in package can also be directly referenced using the syntax shown in
Syntax26-5.
built_in_data_type ::= [ std :: ] data_type_identifier
built_in_function_call ::= [ std :: ] function_subroutine_call
Syntax26-5—Std package import syntax (not in AnnexA)
The package name std followed by the package scope resolution operator :: can be used to unambiguously
access names in the built-in package. For example:
std::sys_task(); // unambiguously call the system provided sys_task
Unlike system tasks and system functions, tasks and functions in the built-in package need not be prefixed
with a $ to avoid collisions with user-defined identifiers. This mechanism allows functional extensions to
the language in a backward compatible manner, without the addition of new keywords or polluting local
name spaces.

<!-- Page 819 -->

IEEE Std 1800-2023