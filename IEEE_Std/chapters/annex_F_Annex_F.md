# Annex F: Annex F

## Annex F
(normative)
Formal semantics of concurrent assertions
F.1 General
This annex presents a formal semantics for SystemVerilog concurrent assertions. Immediate assertions and
coverage statements are not discussed here.
F.2 Overview
Throughout this annex, “assertion” is used to mean “concurrent assertion” and “iff” is used to mean “if and
only if.” The semantics is defined by a relation that determines when a finite or infinite word (i.e., trace)
satisfies an assertion. Intuitively, such a word represents a sequence of valuations of SystemVerilog
variables sampled at the finest relevant granularity of time (e.g., at the granularity of simulator cycles). The
process by which such words are produced is closely related to the SystemVerilog scheduling semantics and
is not defined here. In this annex, words are assumed to be sequences of elements, each element being either
a set of atomic propositions or one of two special symbols used as placeholders when extending finite
words. The atomic propositions are not further defined. The meaning of satisfaction of a SystemVerilog
Boolean expression by a set of atomic propositions is assumed to be understood.
The semantics in this annex describe each evaluation of a concurrent assertion, but there may be many
evaluations for each assertion implied within SystemVerilog code. This annex does not define the semantics
of queueing an instance of a concurrent assertion in procedural code (16.14.6). Once a pending procedural
assertion instance has matured, the semantics of the resulting property evaluation is defined by this annex. If
multiple evaluation attempts of a particular procedural concurrent assertion all mature, each of those
matured attempts is described separately by the equations in this annex. For a concurrent assertion statement
outside procedural code, which is continuously monitored, an instance of the equations in this annex exists
for each starting clock event of the assertion.
The semantics is based on an abstract syntax for SystemVerilog assertions. There are several advantages to
using the abstract syntax rather than the full SystemVerilog assertions BNF, as follows:
a) The abstract syntax facilitates separation of derived operators from basic operators. The satisfaction
relation is defined explicitly only for assertions built from basic operators.
b) The abstract syntax avoids reliance on operator precedence, associativity, and auxiliary rules for
resolving syntactic and semantic ambiguities.
c) The abstract syntax simplifies the assertion language by modifying or eliminating some features that
tend to encumber the definition of the formal semantics.
1) The abstract syntax modifies local variable declarations so that they are integrated with
sequence and property expressions. This change supports the rewriting algorithm (see F.4.1)
that replaces each instance of a named sequence or property with a flattened sequence or
property expression. The local variable declarations that appeared in the named sequence or
property declaration, including local variable formal arguments, become part of the flattened
expression. The abstract syntax also allows local variable declaration assignments. Local
variable declaration assignments are eliminated by a rewriting procedure after sequence and
property instances have been flattened (see F.4.3). The semantics of local variables does not
explicitly refer to their types.

<!-- Page 1236 -->

IEEE Std 2) The abstract syntax eliminates instantiation of sequences and properties. The semantics of an
assertion with an instance of a named sequence or nonrecursive property is the same as the
semantics of a related assertion obtained by replacing the sequence or nonrecursive property
instance with an explicitly written sequence or property expression. F.4.1 defines a rewriting
algorithm that replaces each instance of a named sequence or nonrecursive property with a
flattened sequence or property expression. The semantics of an assertion that has one or more
instances of recursive properties is defined in F.7. The definition is in terms of an infinite set of
associated assertions, each of which may have instances of sequences and nonrecursive
properties, but has no instances of recursive properties. The semantics of each associated
assertion is obtained, as before, by using the rewriting algorithm.
3) The abstract syntax does not allow implicit clocks. Clocking event controls have to be applied
explicitly in the abstract syntax.
In order to use this annex to determine the semantics of a SystemVerilog assertion, the assertion needs to
first be transformed into an assertion in the abstract syntax. For assertions that do not involve recursive
properties, this transformation involves eliminating sequence and nonrecursive property instances by using
the rewriting algorithm (see F.4.1), eliminating local variable declaration assignments (see F.4.3),
determining implicit or inferred clocking event controls, and eliminating redundant clocking event controls.
For example, the following SystemVerilog assertion:
property P(logic[3:0] a, property q);
(a[1:0] == 2'b10) ##1 (a[3:2] == 2'b01) |=> q;
endproperty
property Q(r, logic[1:2] d);
logic[1:2] v;
(1, v = d) ##1 r |=> d == v;
endproperty
always @(c) assert property ( P(A, Q(R, D)) );
is transformed into the assertion:
always @(c) assert property (
(
( item(type(logic[3:0])'(A))[1:0] == 2'b10 ) ##1
( item(type(logic[3:0])'(A))[3:2] == 2'b01 ) |=>
(
logic[1:2] v;
(1, v = item(type(logic[1:2])'(D))) ##1 item(type(R)'(R)) |=>
item(type(logic[1:2])'(D)) == v
)
)
);
in the abstract syntax, assuming R is not a variable_lvalue.
F.3 Abstract syntax
F.3.1 Clock control
In this annex, the clock controls are considered Boolean functions on the input alphabet, and in the @c
notation, c is assumed to be a Boolean. However, in SystemVerilog the notation @c is commonly used to
designate a value-change sensitive event control. To describe how value-change sensitive event controls are

<!-- Page 1237 -->

IEEE Std converted to Boolean, we introduce operator τdefining rewriting rules from an edge-sensitive clock control
to a level-sensitive clock control. b, b ,… denote a Boolean expression, and e, e ,… denote an event
1 1
expression.
In the following transformation it is assumed that all the clocking events occur at ticks of $global_clock.
— τ($global_clock) = 1
— τ(b) = $changing_gclk(b), for b = $global_clock, see 14.14.
— τ(posedge b) = $rising_gclk(b), see F.3.4.4.
— τ(negedge b) = $falling_gclk(b), see F.3.4.4.
— τ(edge b) = τ(posedge b) || τ(negedge b)
— τ(e) = $future_gclk(b), for a named event e (see 15.5), and for a dummy bit variable b
associated with the event e, such that b has value 1 in the time slots when the event e is triggered,
and value 0 in all other time slots.
— τ(e iff b) = τ(e) && b
— τ(e 1 or e 2 ) = τ(e 1 ) || τ(e 2 )
— τ(e 1 , e 2 ) = τ(e 1 ) || τ(e 2 )
For example, the SystemVerilog event control @(posedge clk) corresponds to
@($rising_gclk(clk)) in the formal semantics description.
F.3.2 Abstract grammars
In the following abstract grammars, b denotes a Boolean expression, t denotes a type, v denotes a local
variable name, and e denotes an expression.
The abstract grammar for unclocked sequences is as follows:
R ::= b // "Boolean expression" form
| ( t v [ = e ]; R ) // "local variable declaration" form
| ( 1, v = e ) // "local variable sampling" form
| ( R ) // "parenthesis" form
| ( R ##1 R ) // "concatenation" form
| ( R ##0 R ) // "fusion" form
| ( R or R ) // "or" form
| ( R intersect R ) // "intersect" form
| first_match ( R ) // "first match" form
| R [* 0 ] // "null repetition" form
| R [* 1:$ ] // "unbounded repetition" form
The abstract grammar for clocked sequences is as follows:
S ::= @(b) R // "clock" form
| ( t v [ = e ]; S ) // "local variable declaration" form
| ( S ) // "parenthesized" form
| ( S ##1 S ) // "concatenation" form
The abstract grammar for unclocked properties is as follows:
P ::= strong ( R ) // "strong sequence" form
| weak ( R ) // "weak sequence" form
| ( t v [ = e ]; P ) // "local variable declaration" form
| ( P ) // "parenthesis" form
| not P // "negation" form
| ( P or P ) // "or" form

<!-- Page 1238 -->

IEEE Std | ( P and P ) // "and" form
| ( R |-> P ) // "implication" form
| nexttime P // "nexttime" form
| ( P until P ) // "until" form
| accept_on ( b ) P // "abort" form
Each instance of R in this production shall be a nondegenerate unclocked sequence. In the “sequence” form,
R shall not be tightly satisfied by the empty word. See F.5.2 and F.5.5 for the definitions of nondegeneracy
and tight satisfaction.
The abstract grammar for clocked properties is as follows:
Q ::= @( b ) P // "clock" form
| strong ( S ) // "strong sequence" form
| weak ( S ) // "weak sequence" form
| ( t v [ = e ]; Q ) // "local variable declaration" form
| ( Q ) // "parenthesis" form
| not Q // "negation" form
| ( Q or Q ) // "or" form
| ( Q and Q ) // "and" form
| ( S |-> Q ) // "implication" form
| nexttime Q // "nexttime" form
| ( Q until Q ) // "until" form
| accept_on ( b ) Q // "abort" form
Each instance of S in this production shall be a nondegenerate clocked sequence. In the “sequence” form, S
shall not be tightly satisfied by the empty word. See F.5.2 and F.5.5 for the definitions of nondegeneracy and
tight satisfaction.
The abstract grammar for unclocked top-level properties is as follows:
T ::= P // plain form
| disable iff ( b ) P // "disable" form
| ( t v [ = e ]; T ) // "local variable declaration" form
| ( T ) // "parenthesis" form
The abstract grammar for clocked top-level properties is as follows:
U ::= Q // plain form
| disable iff ( b ) Q // "disable" form
| ( t v [ = e ]; U ) // "local variable declaration" form
| ( U ) // "parenthesis" form
The abstract grammar for assertions is as follows:
A ::= always assert property ( U ) // "always" form
| always @( b ) assert property ( T ) // "always with clock" form
| initial assert property ( U ) // "initial" form
| initial @( b ) assert property ( T ) // "initial with clock" form
F.3.3 Notations
Except where specified otherwise, the following notational conventions, including subscripted versions of
the notations, will be used throughout the remainder of this annex: b and c denote Boolean expressions; t
denotes a type; v denotes a local variable name; u denotes a free checker variable name; e denotes an
expression; uppercase R denotes an unclocked sequence; uppercase S denotes a clocked sequence; uppercase
P denotes an unclocked property; uppercase Q denotes a clocked property; uppercase T denotes an

<!-- Page 1239 -->

IEEE Std unclocked top-level property; uppercase U denotes a clocked top-level property; lowercase r and s denote
sequences, either clocked or unclocked; lowercase p and q denote properties, either clocked or unclocked
and either top-level or not; uppercase A denotes an assertion; i, j, k, m, and n denote non-negative integer
constants.
F.3.4 Derived forms
Internal parentheses are omitted in compositions of the (associative) operators ##1 and or.
F.3.4.1 Derived assertion statements
— restrict property assume property.
F.3.4.2 Derived sequence operators
F.3.4.2.1 Derived consecutive repetition operators
— Let m > 0. R [*m] ( R [*m–1] ##1 R ).
— R [*0:$] ( R [*0] or R [*1:$] ).
— R [*m:m] R[*m].
— Let m < n. R [*m:n] ( R [*m:n–1] or R [*n]).
— Let m > 1. R [*m:$] ( R [*m – 1] ##1 R [*1:$]).
— R [*] ( R [*0] or R [*1:$] ).
— R [+] ( R [*1:$] ).
F.3.4.2.2 Derived delay and concatenation operators
Let m < n.
— ( ##[m:n] R ) (1[*m:n] ##1 R ).
— ( ##[m:$] R ) (1[*m:$] ##1 R ).
— ( ##m R ) (1[*m] ##1 R ).
— ( ##[*] R ) (##[0:$] R ).
— ( ##[+] R ) (##[1:$] R ).
— Let m > 0. ( R 1 ##[m:n] R 2 ) ( R 1 ##1 1[*m – 1:n – 1] ##1 R 2 ).
— Let m > 0. ( R 1 ##[m:$] R 2 ) ( R 1 ##1 1[*m – 1:$] ##1 R 2 ).
— Let m > 1. ( R 1 ##m R 2 ) ( R 1 ##1 1[*m – 1] ##1 R 2 ).
— ( R 1 ##[0:0] R 2 ) ( R 1 ##0 R 2 ).
— Let n > 0. ( R 1 ##[0:n] R 2 ) (( R 1 ##0 R 2 ) or ( R 1 ##[1:n] R 2 )).
— ( R 1 ##[0:$] R 2 ) (( R 1 ##0 R 2 ) or ( R 1 ##[1:$] R 2 )).
F.3.4.2.3 Derived nonconsecutive repetition operators
Let m < n.
— b [->m:n] ( !b [*0:$] ##1 b )[*m:n].
— b [->m:$] ( !b [*0:$] ##1 b )[*m:$].
— b [->m] ( !b [*0:$] ##1 b)[*m].
— b [=m:n] ( b [->m:n] ##1 !b [*0:$] ).
— b [=m:$] ( b [->m:$] ##1 !b [*0:$] ).
— b [=m] ( b [->m] ##1 !b [*0:$] ).

<!-- Page 1240 -->

IEEE Std F.3.4.2.4 Other derived operators
— ( R 1 and R 2 )
((( R 1 ##1 1[*0:$]) intersect R 2 ) or ( R 1 intersect ( R 2 ##1 1[*0:$]))).
— ( R 1 within R 2 ) ((1[*0:$] ##1 R 1 ##1 1[*0:$]) intersect R 2 ).
— ( b throughout R ) (( b [*0:$]) intersect R ).
— ( R, v = e ) ( R ##0 ( 1, v = e )).
— ( R, v 1 = e 1 ,... ,v k = e k) (( R, v 1 = e 1) ##0 ( 1, v 2 = e 2 ,... , v k = e k )) for k > 1 .
F.3.4.3 Derived property operators
F.3.4.3.1 Derived sequential property
— R strong(R) when used in a cover property or expect statement. R weak(R) when
used in an assert property or assume property statement.
F.3.4.3.2 Derived Boolean operators
— p 1 implies p 2 (not p 1 or p 2).
— p 1 iff p 2 ((p 1 implies p 2) and (p 2 implies p 1)).
F.3.4.3.3 Derived nonoverlapping implication operator
— (R |=> P) ((R ##1 1) |-> P).
— (S |=> Q) ((S ##1 @(1) 1) |-> Q).
F.3.4.3.4 Derived conditional operators
— (if(b) P) (b |-> P).
— (if(b) P 1 else P 2) ((b |-> P 1) and (weak(b) or P 2)).
F.3.4.3.5 Derived case operators
Let specify(b) be a function that expands a Boolean expression b and treats it as signed or unsigned
according to the rules mentioned in 12.5 for performing expression comparison while evaluating case
statements.
— ( case ( b ) b 1 : P 1 endcase ) ( if (specify(b) === specify(b 1 ) ) P 1 ) .
— ( case ( b ) default: P d endcase ) ( P d ) .
— ( case ( b ) b 1 : P 1 default: P d endcase ) ( if (specify(b) === specify(b 1 ) ) P 1 else P d ) .
— ( case ( b ) b 1 : P 1 … b n : P n endcase ) ( if (specify(b) === specify(b 1 ) ) P 1
else case (specify(b)) b 2 : P 2 … b n : P n endcase ) .
— ( case ( b ) b 1 : P 1 … b n : P n default: P d endcase ) ( if (specify(b) === specify(b 1 ) ) P 1
else case (specify(b) ) b 2 : P 2 … b n : P n default: P d endcase ) .
F.3.4.3.6 Derived followed_by operators
— (r #-# p) (not(r |-> not p)).
— (r #=# p) (not(r |=> not p)).
F.3.4.3.7 Derived abort operators
— (reject_on ( b ) P ) (not accept_on ( b ) not P ) .
— ( sync_accept_on ( b ) P ) (accept_on ( b ) P ) when the clock context is 1.
— ( sync_reject_on ( b ) P ) (not (sync_accept_on ( b ) not P )) .

<!-- Page 1241 -->

IEEE Std F.3.4.3.8 Derived unbounded temporal operators
— (always p) (p until 0).
— (s_eventually p) (not (always(not p)).
— (p s_until q) ((p until q) and s_eventually q).
— (p until_with q) ((p until (p and q)).
— (p s_until_with q) ((p s_until (p and q)).
F.3.4.3.9 Derived bounded temporal operators
— (s_nexttime p) (not nexttime not p).
— (nexttime[0] p) (1 |-> p).
— Let m > 0. (nexttime[m] p) (nexttime(nexttime[m–1] p)).
— Let m > 0. (s_nexttime[m] p) (not nexttime[m] not p).
— Let m > 0. (eventually[m:m] p) (nexttime[m] p);
— Let m < n. (eventually[m:n] p) (eventually[m:n-1] p or nexttime[n] p).
— Let m > 0. (always[m:m] p) (nexttime[m] p).
— Let m < n. (always[m:n] p) (always[m:n–1] p and nexttime[n] p).
— Let m > 0. (always[m:$] p (nexttime[m] always p) .
— Let m < n. (s_eventually[m:n] p) (not always[m:n] not p).
— Let m > 0. (s_eventually[m:$] p) (s_nexttime[m] s_eventually p).
— Let m < n. (s_always[m:n] p) (not eventually[m:n] not p).
F.3.4.4 Derived sampled value functions
— $sampled(e) e.
— $rose(e,c) $past(b,1,1,c) !== 1 && b === 1, where b is the LSB of e.
— $fell(e,c) $past(b,1,1,c) !== 0 && b === 0, where b is the LSB of e.
— $stable(e,c) $past(e,1,1,c) === e.
— $changed(e,c) $past(e,1,1,c) !== e.
— $rose_gclk(e) $past_gclk(b) !== 1 && b === 1, where b is the LSB of e.
— $fell_gclk(e) $past_gclk(b) !== 0 && b === 0, where b is the LSB of e.
— $stable_gclk(e) $past_gclk(e) === e.
— $changed_gclk(e) $past_gclk(e) !== e.
— $rising_gclk(e) b !== 1 && $future_gclk(b) === 1, where b is the LSB of e.
— $falling_gclk(e) b !== 0 && $future_gclk(b) === 0, where b is the LSB of e.
— $steady_gclk(e) e === $future_gclk(e).
— $changing_gclk(e) e !== $future_gclk(e).
F.3.4.5 Other derived operators
— ( t 1 v 1 [ = e 1 ] ;... ; t k v k [ = e k ] ; X ) ( t 1 v 1 [ = e 1 ] ; ( t 2 v 2 [ = e 2 ] ;... ; t k v k [ = e k ] ; X ) )
for k > 1 and X any of P, Q, R, S, T, U .
F.3.4.6 Free checker variable assignment
— rand t u = e initial assume property (@1 u === e) .
— always_ff @c u <= e always_ff assume property (@1 $future_gclk(u) === (c ?
e : u)).

<!-- Page 1242 -->

IEEE Std If the assignment to u is in the scope of one or several conditional statements with a resulting
enabling condition b, then the equivalent assumption shall also be evaluated using the same enabling
condition b (see F.5.3.1).
F.4 Rewriting algorithms
For the rewriting algorithm, an auxiliary function item is defined as follows. The function item may be
applied to any SystemVerilog expression that may appear as an actual argument expression in an instance of
a named sequence, property, checker or let. If e is such an expression, then item(e) behaves like e in all
respects except that operations allowed on a reference to or instance of a named item declared with the same
type as e are also allowed on item(e). Also, any operation that is allowed on an instance of a named sequence
(respectively, property) is allowed on item applied to a sequence (respectively, property, including a top-
level property).
The function item is not a SystemVerilog function, and it is introduced only in the rewriting algorithm. The
rewriting algorithm uses item because operations that are legal on a reference to a formal argument within
the body of a declaration might no longer be legal when an actual argument expression is substituted for the
reference to the formal argument. For example, let a and b be variables of type logic[0:1], let v be a
variable of type logic[0:3], and let e be the cast expression type(logic[0:3])'({a,b}). If v is a
formal argument, then the part select expression v[1:2] is legal within the body of the declared item.
However, if e is an actual argument expression passed to v in an instance, then the part select operation
cannot be applied when e is substituted for v because (type(logic[0:3])'({a,b}))[1:2] is illegal.
Using the item function, the form item(type(logic[0:3])'({a,b}))[1:2] is legal. For expressions
with undefined type, item does not enable additional operations.
F.4.1 Rewriting sequence and property instances
This subclause describes an algorithm for rewriting a sequence or property that contains one or more
instances of named sequences or nonrecursive properties. The result of the algorithm is one flattened
sequence or property without instances. The semantics of a hierarchical sequence or property is defined to
be the semantics of the flattened sequence or property resulting from the rewriting algorithm. The rewriting
algorithm does not itself account for name resolution and assumes that names have been resolved prior to the
substitution of actual arguments for references to the corresponding formal arguments. If the flattened
sequence or property is not legal, then the source is not legal. A property rewritten in the algorithm may be
the top-level property of a concurrent assertion.
F.4.1.1 The rewriting algorithm
Given π a sequence or property, possibly a top-level property:
While there are property instances in π do:
begin
Select an arbitrary property instance p and replace it by flatten_property(p).
end
While there are sequence instances in π do:
begin
1) Select an arbitrary sequence instance r.
2) If either (a) r appears in an event expression in a clocking_event, or (b) r is the operand in a
sequence_method_call, then replace r by item(sequence'flatten_sequence(r)).
3) Otherwise, replace r by flatten_sequence(r).
end

<!-- Page 1243 -->

IEEE Std flatten_property(p)
begin
1) Create a copy p' of the declaration of p.
2) For each formal argument f of p', let a be the corresponding actual argument expression for the
f
instance p. I.e., a is the actual argument expression bound to f in p, or, if no argument is bound to f
f
in p, then a is the default actual argument declared for f in p'.
f
3) For each untyped formal argument f of p', do the following for each reference to f in p':
a) If a is either $ or a variable_lvalue, then replace the reference by a .
f f
b) Otherwise, replace the reference by item(type(a f)'(a f)).
4) For each typed formal argument f of p' that is not a local variable formal argument and whose type t
does not match (see 6.22.1) event, sequence, or property, do the following for each reference to
f in p':
a) If t is a casting_type (see 6.24), then replace the reference by item(t'(a )).
f
b) Otherwise, replace the reference by item(type(t)'(a f)).
According to 16.8.1, none of the references so replaced shall be the variable_lvalue in an
operator_assignment or inc_or_dec_expression in a sequence_match_item.
5) For each typed formal argument f of p' whose type t matches (see 6.22.1) event, sequence, or
property (and therefore is not a local variable formal argument), do the following for each
reference to f in p':
a) If the reference stands as the operand of a sequence_method_call, then replace the reference by
item(a).
f
b) Otherwise, replace the reference by (a f). The parentheses around a
f
may be omitted if the
reference is itself already enclosed in parentheses.
6) For each local variable formal argument f of p' whose type is t, add to the beginning of the body of p'
the local variable declaration “t f = a ;”. These local variable declarations may be arranged in any
f
order.
7) Return the expression obtained by copying the local variable declarations and body property_spec
from p' and enclosing the result in parentheses.
end
flatten_sequence(r)
begin
1) Create a copy r' of the declaration of r.
2) For each formal argument f of r', let a be the corresponding actual argument expression for the
f
instance r. I.e., a is the actual argument expression bound to f in r, or, if no argument is bound to f in
f
r, then a is the default actual argument declared for f in r'.
f
3) For each untyped formal argument f of r', do the following for each reference to f in r':
a) If a is either $ or a variable_lvalue, then replace the reference by a .
f f
b) Otherwise, replace the reference by item(type(a f)'(a f)).
4) For each typed formal argument f of r' that is not a local variable formal argument and whose type t
does not match (see 6.22.1) event or sequence, do the following for each reference to f in r':
a) If t is a casting_type (see 6.24), then replace the reference by item(t'(a f)).
Otherwise, replace the reference by item(type(t)'(a f)).
According to 16.8.1, none of the references so replaced shall be the variable_lvalue in an
operator_assignment or inc_or_dec_expression in a sequence_match_item.
5) For each typed formal argument f of r' whose type t matches (see 6.22.1) event or sequence (and
therefore is not a local variable formal argument), do the following for each reference to f in r':

<!-- Page 1244 -->

IEEE Std a) If the reference stands as the operand of a sequence_method_call, then replace the reference by
item(a).
f
b) Otherwise, replace the reference by (a f). The parentheses around a
f
may be omitted if the
reference is itself already enclosed in parentheses.
6)
a) For each input local variable formal argument f of r' whose type is t, add to the beginning of the
body of r' the local variable declaration “t f = a ;”.
f
b) For each inout local variable formal argument f of r' whose type is t, add to the beginning of the
body of r' the local variable declaration “t f = a ;” and include the assignment “a = f” in a list of
f f
match items attached to the end of the body sequence_expr of r'.
c) For each output local variable formal argument f of r' whose type is t, add to the beginning of the
body of r' the local variable declaration “t f ;” and include the assignment “a = f” in a list of
f
match items attached to the end of the body sequence_expr of r'.
The local variable declarations added to the beginning of the body of r' may be arranged in any
order.
7) Return the expression obtained by copying the local variable declarations and body sequence_expr
from r' and enclosing the result in parentheses.
end
According to 16.8.2, if f, f' are distinct local variable formal arguments of direction inout or input, then a f
= a'. Therefore, the overall result of the assignments to the actual arguments in 6(b) and 6(c) does not
f
depend on the order of these assignments.
F.4.2 Rewriting checkers
This subclause describes an algorithm for rewriting a checker that contains one or more instances of other
checkers. The result of the algorithm is one flattened checker without instances. The rewriting algorithm
does not itself account for name resolution and assumes that names have been resolved prior to the
substitution of actual arguments for references to the corresponding formal input arguments. The checker
formal arguments that have output direction shall be treated differently (see 17.2), and this algorithm does
not apply to them. If the flattened checker is not legal, then the source is not legal. A checker rewritten in the
algorithm may be a nested checker instance or a top-level checker instance.
F.4.2.1 The rewriting algorithm
Given π a checker, possibly a top-level checker:
While there are checker instances in π do:
begin
Select an arbitrary checker instance c and replace it by flatten_checker(c).
end
flatten_checker(c)
begin
1) Create a copy c' of the declaration of c.
2) For each formal input argument f of c', let a be the corresponding actual argument expression for the
f
instance c. I.e., a is the actual argument expression bound to f in c, or, if no argument is bound to f in
f
c, then a is the default actual argument declared for f in c'.
f
3) For each untyped formal input argument f of c', do the following for each reference to f in c':
a) If a is either $ or a variable_lvalue, then replace the reference by a .
f f

<!-- Page 1245 -->

IEEE Std b) Otherwise, replace the reference by item(type(a f)'(a f)).
4) For each typed formal input argument f of c' whose type t does not match (see 6.22.1) event,
sequence, or property, do the following for each reference to f in c':
a) If t is a casting_type (see 6.24), then replace the reference by item(t'(a )).
f
b) Otherwise, replace the reference by item(type(t)'(a f)).
None of the references so replaced shall be a variable_lvalue anywhere in the checker.
5) For each typed formal input argument f of c' whose type t matches (see 6.22.1) event, sequence,
or property, do the following for each reference to f in c':
a) If the reference stands as the operand of a sequence_method_call, then replace the reference by
item(a).
f
b) Otherwise, replace the reference by (a f). The parentheses around a
f
may be omitted if the
reference is itself already enclosed in parentheses.
6) Return the checker body.
end
F.4.3 Rewriting local variable declaration assignments
After replacing instances of named sequences and properties as described in F.4.1, local variable declaration
assignments are eliminated from the resulting sequences and properties. Corresponding local variable
assignments are added within the sequences and properties using the following procedure. Only after this
step is completed are the clock rewrite rules used.
At several points, the procedure for rewriting local variable declaration assignments queries whether a
sequence admits an empty match. The queries allow splitting of cases in order to avoid changing the empty
match behavior. Formally, a sequence admits an empty match if, and only if, it is tightly satisfied by the
empty word. The tight satisfaction relation is defined in F.5.2 and F.5.5, where it is assumed that the clock
rewrite rules have already been applied to eliminate clocking operators. The current procedure requires that
the clocking operators remain in the syntax. Therefore, an independent definition of admission of an empty
match is given below by the function admits_empty, which maps sequences to {0, 1}. It can be proved that
for a sequence r, admits_empty(r) = 1 if, and only if, the empty word tightly satisfies r', where r' is the
sequence that results from r by eliminating local variable declaration assignments and by applying the clock
rewrite rules.
— admits_empty(b) = 0.
— admits_empty((t v [= e ]; r)) = admits_empty(r).
— admits_empty((1, v = e)) = 0.
— admits_empty(( r )) = admits_empty(r).
— admits_empty((r 1 ##1 r 2)) = admits_empty(r 1 ) && admits_empty(r 2 ).
— admits_empty((r 1 ##0 r 2)) = 0.
— admits_empty((r 1 or r 2)) = admits_empty(r 1 ) || admits_empty(r 2 ).
— admits_empty((r 1 intersect r 2)) = admits_empty(r 1 ) && admits_empty(r 2 ).
— admits_empty(first_match(r)) = admits_empty(r).
— admits_empty(r[*0]) = 1.
— admits_empty(r[*1:$]) = admits_empty(r).
— admits_empty(@(c)r) = admits_empty(r).
Let r be a sequence, and let c be the unique semantic leading clock of r (semantic leading clocks are defined
κ κ
in 16.16.1). If c = inherited, then let (r) be the empty string. Otherwise, let (r) = @(c).

<!-- Page 1246 -->

IEEE Std The procedure first eliminates all local variable declaration assignments that are attached to sequences. In
general, ( t v = e; r ) is replaced by
κ
( t v; (r) ( ((1, v = e) ##0 (r)) or ((r) intersect 1[*0])) )
If admits_empty(r) = 0, then the replacement may be simplified to
κ
( t v; (r) ( ((1, v = e) ##0 (r)) )
If admits_empty(r) = 1, then the replacement may be simplified to
κ
( t v; (r) ( ((1, v = e) ##0 (r)) or 1[*0]) )
After this step, local variable declaration assignments remain only attached to properties. So that the
declaration assignments are executed after advancing to the alignment points with the appropriate semantic
leading clocks, the procedure next pushes these assignments down in the syntax using the function push
defined as follows. push takes a list of local variable declaration assignments as its first argument and a
property as its second argument. The property may be a top-level property. For clarity of notation,
concatenations of lists are enclosed in angle brackets (<, >), and the empty list is denoted by <>.
The procedure finishes by applying the function push with <> as first argument to each top-level property
and descending recursively.
Let E denote an ordered list of local variable assignments. Other notations are as in F.3.3.
— push(E, ( t v ; p )) = ( t v ; push(E, p) ).
— push(E, ( t v = e ; p )) = ( t v ; push(<E, v = e>, p) ).
— push(<>, r ) = r. If E is nonempty, then
κ
push(E, r) = (r) (1, E) ##0 (r)
In this case, r is a sequence used as a property. According to 16.12.22, admits_empty(r) = 0.
— push(<>, r |-> p) = r |-> push(<>, p). If E is nonempty, then
κ
push(E, r |-> p) = (r) (1, E) ##0 (r)|-> push(<>, p)
— push(<>, r |=> p) = r |=> push(<>, p). If E is nonempty and admits_empty(r) = 0, then
κ
push(E, r |=> p) = (r) (1, E) ##0 (r)|=> push(<>, p)
If E is nonempty and admits_empty(r) = 1, then
κ
push(E, r |=> p) = ( (r) (1, E) ##0 (r)|=> push(<>, p) ) and push(E, p) )
— push(<>, if(b) p [ else q ] ) = if(b) push(<>, p) [ else push(<>, q) ]. If E is nonempty, then
push(E, if(b) p [ else q ] ) = (1, E) |-> if(b) push(<>, p) [ else push(<>, q) ].
— push(E, disable iff (b) p) = disable iff (b) push(E, p).
— push(E, @(c) p) = @(c) push(E, p).
— push(E, ( p )) = ( push(E, p) ).
— push(E, not p) = not push(E, p).
— push(E, p or q) = push(E, p) or push(E, q).
— push(E, p and q) = push(E, p) and push(E, q).
F.5 Semantics
Let P be the set of atomic propositions.
The semantics of assertions and properties is defined via a relation of satisfaction by empty, finite, and
infinite words over the alphabet Σ = 2P U {T, ⊥}. Such a word is an empty, finite, or infinite sequence of
elements of Σ. The number of elements in the sequence is called the length of the word, and the length of
word w is denoted |w|, where |w| is either a non-negative integer or infinity.

<!-- Page 1247 -->

IEEE Std The sequence elements of a word are called its letters and are assumed to be indexed consecutively
beginning at zero. If |w| > 0, then the first letter of w is denoted w0; if |w| > 1, then the second letter of w is
denoted w1; and so forth. w i.. denotes the word obtained from w by deleting its first i letters. If i <
|w|, then w i.. = w iw i+1.... If i > |w|, then w i.. is empty.
If i < j, then w i, j denotes the finite word obtained from w by deleting its first i letters and also deleting all
letters after its ( j + 1)st. If i < j < |w|, then w i, j = w iw i+1...w j.
If w is a word over Σ, define w to be the word obtained from w by interchanging T with ⊥. More precisely,
wi= T if w i = ⊥ ; w i = ⊥ if w i = T; and w i = w i if w i is an element in 2P.
The semantics of clocked sequences and properties is defined in terms of the semantics of unclocked
sequences and properties. See F.5.1.
It is assumed that the satisfaction relation ζ b is defined for elements ζ in 2P and Boolean expressions b.
For any Boolean expression b, define
T b and ⊥ b
F.5.1 Rewrite rules for clocks
The semantics of clocked sequences and properties is defined in terms of the semantics of unclocked
sequences and properties. The following rewrite rules define the transformation of a clocked sequence or
property into an unclocked version that is equivalent for the purposes of defining the satisfaction relation. In
this transformation, it is required that the conditions in event controls not be dependent upon any local
variables.
F.5.1.1 Rewrite rules for sequences
The transformation T s (S, c) recursively defined as follows produces a sequence R from a sequence S and a
clock c:
— T s (b, c) = (!c[*0:$] ##1 c & b).
— T s ((1, v = e), c) = (T s (1, c) ##0 (1, v = e)).
— T s ((@(c 2) r), c 1 ) = (T s (r, c 2 )).
— T s ((r 1 ##1 r 2), c) = (T s (r 1 , c) ##1 T s (r 2 , c)).
— T s ((r 1 ##0 r 2), c) = (T s (r 1 , c) ##0 T s (r 2 , c)).
— T s ((r 1 or r 2), c) = (T s (r 1 , c) or T s (r 2 , c)).
— T s ((r 1 intersect r 2), c) = (T s (r 1 , c) intersect T s (r 2 , c)).
— T s ((first_match (r)), c) = (first_match (T s (r, c))).
— T s ((r[*0]), c) = (T s (r, c)[*0]).
— T s ((r[*1:$]), c) = (T s (r, c)[*1:$]).
F.5.1.2 Rewrite rules for properties
The transformation T p (p, c) recursively defined as follows produces a property P from a property p and a
clock c:
— T p (strong(r), c) = (strong(T s (r, c))).
— T p (weak(r), c) = (weak(T s (r, c))).
— T p ((@(c 2) p), c 1 ) = T p (p, c 2 ).
— T p ((disable iff(b) p), c) = (disable iff(b) T p (p, c)).
— T p ((accept_on(b) p), c) = (accept_on(b) T p (p, c)).

<!-- Page 1248 -->

IEEE Std — T p ((sync_accept_on(b) p), c) = (accept_on(b && c) T p (p, c)).
— T p ((not p), c) = (not T p (p, c)).
— T p ((r |-> p), c) = (T s (r, c) |-> T p (p, c)).
— T p ((p 1 or p 2), c) = (T p (p 1 , c) or T p (p 2 , c)).
— T p ((p 1 and p 2), c) = (T p (p 1 , c) and T p (p 2 , c)).
— T p ((nexttime p), c) = (!c until (c and nexttime (!c until (c and T p (p, c))))).
— T p ((p 1 until p 2), c) = ((not (c and not T p (p 1 , c))) until (c and T p (p 2 , c))).
F.5.2 Tight satisfaction without local variables
Tight satisfaction is denoted by . For unclocked sequences without local variables, tight satisfaction is
defined as follows: w, x, y, and z denote finite words over Σ.
— w b iff |w| = 1 and w0 b.
— w ( R ) iff w R.
— w ( R 1 ##1 R 2 ) iff there exist x, y so that w = xy and x R 1 and y R 2 .
— w ( R 1 ##0 R 2 ) iff there exist x, y, z so that w = xyz and |y| = 1, and xy R 1 and yz R 2 .
— w ( R 1 or R 2 ) iff either w R 1 or w R 2 .
— w ( R 1 intersect R 2 ) iff both w R 1 and w R 2 .
— w first_match ( R ) iff both
• w R and
• if there exist x, y so that w = xy and x R, then y is empty.
— w R [*0] iff |w| = 0.
— w R [*1:$] iff there exist words w 1 , w 2 ,..., w j ( j > 1) so that w = w 1 w 2 ...w j and for every i so that
1< i < j, w R.
i
If S is a clocked sequence, then w S iff w S', where S' is the unclocked sequence that results from S by
applying the rewrite rules.
An unclocked sequence R is nondegenerate iff there exists a nonempty finite word w over Σ so that w R.
A clocked sequence S is nondegenerate iff the unclocked sequence S' that results from S by applying the
rewrite rules is nondegenerate.
F.5.3 Satisfaction without local variables
F.5.3.1 Neutral satisfaction
w denotes a nonempty finite or infinite word over Σ. Assume that all properties, sequences, and unclocked
property fragments do not involve local variables.
Neutral satisfaction of assertion statements is as follows:
For the definition of neutral satisfaction of assertion statements, b denotes the Boolean expression
representing the enabling condition for the assertion statement. Intuitively, b is derived from the conditions
causing a queued evaluation attempt of a procedural assertion statement (see 16.14.6), while b is 1 for a
declarative assertion statement.
— w, b always @(c) assert property T iff for every 0 < i < |w| so that w i c and w i b,
either w i.. @(c) T or w i.. d @(c) T.

<!-- Page 1249 -->

IEEE Std — w, b always assert property U iff for every 0 < i < |w|, if w i b then either
w i.. U or w i.. d U.
— w, b initial @(c) assert property T iff for every 0 < i < |w| so that
w 0, i !c [*0:$] ##1 c and w i b, either w i.. @(c) T or w i.. d @(c) T.
— w, b initial assert property U iff (if w 0 b then either w U or w d U ).
— w, b always @(c) assume property T iff w, b always @(c) assert property T.
— w, b always assume property U iff w, b always assert property U.
— w, b initial @(c) assume property T iff w, b initial @(c) assert property T.
— w, b initial assume property U iff w, b initial assert property U.
— w, b always @(c) cover property T iff there exists 0 < i < |w| so that w i c, w i b, and
wi.. @(c) T.
— w, b always cover property U iff there exists 0 < i < |w| so that w i b and w i.. U.
— w, b initial @(c) cover property T iff there exists 0 < i < |w| so that w 0,i !c[*0:$]
##1 c, w i b, and w i.. @(c) T.
— w, b initial cover property U iff w 0 b and w U.
The neutral satisfaction of assertion statements previously defined describes the behavior of an assertion
statement on a single word. Given a set of words and a set of assumptions, the following definitions describe
assertion statement satisfaction on the set of words predicated on the set of assumptions:
— A word in the set of words is feasible if every assumption in the set of assumptions is satisfied on the
word.
— An assert property statement is satisfied on a set of words predicated on the set of assumptions
if it is satisfied on each feasible word.
— A cover property statement is satisfied on a set of words predicated on the set of assumptions if it
is satisfied on at least one feasible word.
An assertion statement holds globally on the set of words predicated on the set of assumptions if it is
satisfied on every feasible word.
Neutral satisfaction of top-level properties is defined as follows:
— For T = P, w T iff w P.
— For U = Q, w U iff w Q.
— For T = disable iff (b) P, w T iff either
• w P and no letter of w satisfies b, or
• Some letter of w satisfies b and w 0, i–1 ⊥ω P for i the least index such that
w i b, 0 < i < |w|.
— For U = disable iff (b) Q, w U iff either
• w Q and no letter of w satisfies b, or
• Some letter of w satisfies b and w 0, i–1 ⊥ω Q for i the least index such that
w i b, 0 < i < |w|.
— w ( T ) iff w T.
— w ( U ) iff w U.
Disabling of top-level properties is defined as follows:
— For T = P, w d T.
— For U = Q, w d U.

<!-- Page 1250 -->

IEEE Std — For T = disable iff (b)P, w d T iff some letter of w satisfies b and both w 0, i–1 T ω P and
w 0, i–1 ⊥ω P for i the least index such that w i b, 0 < i < |w|.
— For U = disable iff (b)Q, w d U iff some letter of w satisfies b and both w 0, i–1 T ω Q
and w 0, i–1 ⊥ω Q for i the least index such that w i b, 0 < i < |w|.
— w d ( T ) iff w d T.
— w d ( U ) iff w d U.
T is said to pass on w if w T. T is said to be disabled on w if w d T. T is said to fail on w if T neither
passes nor is disabled on w. It can be proved that T cannot both pass and be disabled on w.
Neutral satisfaction of properties is defined as follows:
— w ( P ) iff w P.
— w Q iff w T p (Q, 1).
— w not P iff w P.
— w strong (R) iff there exists 0 < j < |w| so that w 0, j R.
— w weak (R) iff for every 0 < j < |w|, w 0, j T ω strong (R).
— w ( R |-> P ) iff for every 0 < j < |w| so that w 0, j R, w j.. P.
— w ( P 1 or P 2 ) iff w P 1 or w P 2 .
— w ( P 1 and P 2 ) iff w P 1 and w P 2 .
— w ( nexttime P) iff either |w| = 0 or w 1.. P.
— w (P 1 until P 2) iff either there exists 0 < j < |w| so that w j.. P 2 and for every 0 < i < j,
wi.. P , or for every 0 < i < |w|, wi.. P .
1 1
— w ( accept_on (b) P) iff either:
• w P, or
• For some 0 < i < |w|, w i b and w 0, i–1 T ω P. Here, w 0, –1 denotes the empty word.
Remark: Because w is nonempty, it can be proved that w not b iff w !b.
F.5.3.2 Weak and strong satisfaction by finite words
This subclause defines weak and strong satisfaction, denoted – and + (respectively) of an assertion A by
a finite (possibly empty) word w over Σ. These relations are defined in terms of the relation of neutral
satisfaction by infinite words as follows:
— w – A iff w T ω A.
— w + A iff w⊥ω A.
A tool checking for satisfaction of A by the finite word w should return the following:
— “Holds strongly” if w + A.
— “Fails” if w – A.
— “Holds (but does not hold strongly)” if w A and w + A.
— “Pending” if w – A and w A.
F.5.3.3 Vacuity
non
This subclause defines the relation of non-vacuity, denoted , between a word w and a property P. An
non
evaluation of P on w is nonvacuous provided w P.

<!-- Page 1251 -->

IEEE Std — Base:
non
• w strong(R).
non
• w weak(R).
— Induction:
non non
• w (P) iff w P.
• w non R |-> P iff there exists i > 0 such that w 0..i R and w i.. non P.
non non non
• w P 1 and P 2 iff w P 1 or w P 2 .
non non non
• w P 1 or P 2 iff w P 1 or w P 2 .
non non non
• w P 1 iff P 2 iff w P 1 or w P 2 .
non non non
• w P 1 implies P 2 iff w P 1 , w P 1 , and w P 2 .
non non
• w not P iff w P.
• w non nextime P iff |w| > 0 and wi.. non P.
non
• w P 1 until P 2 iff there exists 0 < i < |w| such that the following holds:
— Either wi.. non P or wi.. non P and
1 2
— For all 0 < j < i, wj.. P 1 and not P 2 .
non
• w P 1 s_until P 2 iff there exists 0 < i < |w| such that the following holds:
— Either wi.. non P or wi.. non P and
1 2
— For all 0 < j < i, wj.. P 1 and not P 2 .
non
• w always P iff there exists 0 < i < |w| such that the following holds:
— wi.. non P and
— For all 0 < j < i, wj.. P.
non
• w always [m : n]P iff there exists m < i < n such that the following holds:
— wi.. non P and
— For all m < j < i, wj.. P.
non
• w s_always [m : n]P iff there exists m < i < n such that the following holds:
— wi.. non P and
— For all m < j < i, wj.. P.
non
• w s_eventually P iff there exists 0 < i < |w| such that the following holds:
— wi.. non P and
— For all 0 < j < i, wj.. not P.
non
• w eventually [m : n]P iff there exists m < i < n such that the following holds:
— wi.. non P and
— For all m < j < i, wj.. not P.
non
• w s_eventually [m : n]P iff there exists m < i < n such that the following holds:
— wi.. non P and
— For all m < j < i, wj.. not P.
non non
• w disable iff (b) P iff w P and one of the following holds:
1) For every 0 < i < |w|, w i b.
2) There exists a prefix x of w, such that for every 0 < i < |x|, x i b, and either x ⊥ω P or
ω
x T P.
non non
• w accept_on (b) P iff w P and one of the following holds:
1) For every 0 < i < |w|, w i b.
2) There exists a prefix x of w, such that for every 0 < i < |x|, x i b, and either x ⊥ω P or
ω
x T P.

<!-- Page 1252 -->

IEEE Std non non
• w reject_on (b) P iff w P and one of the following holds:
1) For every 0 < i < |w|, w i b.
2) There exists a prefix x of w, such that for every 0 < i < |x|, x i b, and either x ⊥ω P or
ω
x T P.
non
A word w satisfies property P nonvacuously iff w P and w P.
non non
The relation is not explicitly defined for all the derived operators. For these operators the relation is
implicitly defined by unrolling their derivation.
F.5.4 Local variable flow
This subclause defines inductively how local variable names flow through unclocked sequences. In the
following, “U” denotes set union, “ ” denotes set intersection, “–” denotes set difference, and “{}” denotes
the empty set.
The function “sample” takes a sequence as input and returns a set of local variable names as output.
Intuitively, this function returns the set of local variable names that are sampled (i.e., assigned) in the
sequence.
The function “block” takes a sequence as input and returns a set of local variable names as output.
Intuitively, this function returns the set of local variable names that are blocked from flowing out of the
sequence.
The function “flow” takes a set X of local variable names and a sequence as input and returns a set of local
variable names as output. Intuitively, this function returns the set of local variable names that flow out of the
sequence given the set X of local variable names that flow into the sequence.
The function “sample” is defined by the following:
— sample (b) = {}.
— sample (( t v; R )) = sample (R) – {v}.
— sample (( 1, v = e )) = {v}.
— sample (( R )) = sample (R).
— sample (( R 1 ##1 R 2 )) = sample (R 1 ) U sample (R 2 ).
— sample (( R 1 ##0 R 2 )) = sample (R 1 ) U sample (R 2 ).
— sample (( R 1 or R 2 )) = sample (R 1 ) U sample (R 2 ).
— sample (( R 1 intersect R 2 )) = sample (R 1 ) U sample (R 2 ).
— sample (first_match ( R )) = sample (R).
— sample (R [*0]) = {}.
— sample (R [*1:$]) = sample (R).
The function “block” is defined by the following:
— block (b) = {}.
— block (( t v; R )) = block (R) – {v}.
— block (( 1, v = e )) = {}.
— block (( R )) = block (R).
— block (( R 1 ##1 R 2 )) = (block (R 1 ) – flow ({}, R 2 )) U block (R 2 ).
— block (( R 1 ##0 R 2 )) = (block (R 1 ) – flow ({}, R 2 )) U block (R 2 ).
— block (( R 1 or R 2 )) = block (R 1 ) U block (R 2 ).

⊃

<!-- Page 1253 -->

IEEE Std — block (( R 1 intersect R 2 )) = block (R 1 ) U block (R 2 ) U ( sample (R 1 ) sample (R 2 )).
— block (first_match ( R )) = block (R).
— block (R [*0]) = {}.
— block (R [*1:$]) = block (R).
The function “flow” is defined by the following:
— flow (X, b) = X.
— flow (X , ( t v; R )) = (X {v}) U (flow (X – {v}, R) – {v} ).
— flow (X, ( 1, v = e )) = X U {v}.
— flow (X, ( R )) = flow (X, R).
— flow (X, ( R 1 ##1 R 2 )) = flow ( flow (X, R 1 ), R 2 ).
— flow (X, ( R 1 ##0 R 2 )) = flow ( flow (X, R 1 ), R 2 ).
— flow (X, ( R 1 or R 2 )) = flow (X, R 1 ) flow (X, R 2 ).
— flow (X, ( R 1 intersect R 2 )) = ( flow (X, R 1 ) U flow (X, R 2 )) – block (( R 1 intersect R 2 )).
— flow (X, first_match(R)) = flow (X, R).
— flow (X, R [*0]) = X.
— flow (X, R [*1:$]) = flow (X, R).
Remark: It can be proved that flow (X, R) = (X U flow ({}, R)) – block (R). It follows that flow ({}, R) and
block (R) are disjoint. It can also be proved that flow ({}, R) is a subset of sample (R).
F.5.5 Tight satisfaction with local variables
A local variable context is a function that assigns values to local variable names. If L is a local variable
context, then dom(L) denotes the set of local variable names that are in the domain of L. If D dom(L),
then L| means the local variable context obtained from L by restricting its domain to D. If v is a local
D
variable name, then L\v denotes L| and L[v] denotes L| .
dom(L)-{v} {v}
In the presence of local variables, tight satisfaction is a four-way relation defining when a finite word w over
the alphabet Σ together with an input local variable context L satisfies an unclocked sequence R and yields
an output local variable context L . This relation is denoted as follows:
w, L , L R.
0 1
and is defined below. It can be proved that the definition guarantees that w, L , L R implies
0 1
dom(L )=flow (dom(L ), R).
1 0
— w, L 0 , L 1 ( t v ; R ) iff there exists L such that w, L 0 \v, L R and L 1 = L 0 [v] U (L\v).
— w, L 0 , L 1 ( 1, v = e ) iff |w| = 1 and w0 1 and L 1 = {(v, e[L 0 , w0])} U L 0 \v), where e[L 0 , w0]
denotes the value obtained from e by evaluating first according to L and second according to w0. In
case w 0 {T, }, e[L ,T] and e[L , ] can be any constant values of the type of e.
0 0
— w, L , L b iff |w| = 1 and w0 b[L ] and L = L . Here b[L ] denotes the expression obtained from
0 1 0 1 0 0
b by substituting values from L .
— w, L 0 , L 1 ( R ) iff w, L 0 , L 1 R.
— w, L 0 , L 1 ( R 1 ##1 R 2 ) iff there exist x, y, L' so that w = xy and x, L 0 , L' R 1 and y, L', L 1 R 2 .
— w, L 0 , L 1 ( R 1 ##0 R 2 ) iff there exist x, y, z, L' so that w = xyz and |y| = 1, and xy, L 0 , L' R 1 and
yz, L', L R .
1 2
— w, L 0 , L 1 ( R 1 or R 2 ) iff there exists L' so that both of the following hold:

⊃
⊃
⊃
∈ ⊥ ⊥

<!-- Page 1254 -->

IEEE Std • Either w, L , L' R or w, L , L' R , and
0 1 0 2
• L 1 = L' | D , where D = flow (dom(L 0 ), ( R 1 or R 2 )).
— w, L 0 , L 1 ( R 1 intersect R 2 ) iff there exist L', L" so that w, L 0 , L' R 1 and w, L 0 , L" R 2 and
L = L' | U L" | , where
1 D’ D’’
D’ = flow (dom(L 0 ), R 1 ) – (block (( R 1 intersect R 2 )) U sample (R 2 ))
D’’ = flow (dom(L 0 ), R 2 ) – (block (( R 1 intersect R 2 )) U sample (R 1 ))
Remark: It can be proved that if w, L , L' R and w, L , L" R , then L' | U L" | is a function.
0 1 0 2 D’ D’’
— w, L 0 , L 1 first_match ( R ) iff both
• w, L , L R and
0 1
• If there exist x, y, L' so that w = xy and x, L , L' R, then y is empty.
— w, L 0 , L 1 R [*0] iff |w| = 0 and L 1 = L 0 .
— w, L 0 , L 1 R [*1:$] iff there exist L (0) = L 0 , w 1 , L (1) , w 2 , L (2) ,..., w j , L ( j) = L 1 ( j > 1) so that
w=w w ...w and for every i so that 1 < i < j, w, L , L R.
1 2 j i (i –1) (i)
If S is a clocked sequence, then w, L , L S iff w, L , L S', where S' is the unclocked sequence that
0 1 0 1
results from S by applying the rewrite rules.
An unclocked sequence R is nondegenerate iff there exist a nonempty finite word w over Σ and local
variable contexts L , L so that w, L , L R. A clocked sequence S is nondegenerate iff the unclocked
0 1 0 1
sequence S' that results from S by applying the rewrite rules is nondegenerate.
F.5.6 Satisfaction with local variables
F.5.6.1 Neutral satisfaction
w denotes a nonempty finite or infinite word over Σ. L and L denote local variable contexts.
0 1
The rules defining neutral satisfaction of an assertion are identical to those without local variables, but with
the understanding that the underlying properties can have local variables.
Neutral satisfaction of top-level properties is defined as follows:
— For T = P, w, L T iff w, L P.
0 0
— For U = Q, w, L U iff w, L Q.
0 0
— For T = disable iff (b) P, w, L 0 T iff either
• w, L P and no letter of w satisfies b, or
• Some letter of w satisfies b and w 0, i–1 ⊥ω , L P for i the least index such that
w i b, 0 < i < |w|.
— For U = disable iff (b) Q, w, L 0 U iff either
• w, L Q and no letter of w satisfies b, or
• Some letter of w satisfies b and w 0, i–1 ⊥ω , L Q for i the least index such that
w i b, 0 < i < |w|.
— w, L 0 ( t v ; T ) iff w, L 0 \v T.
— w, L 0 ( t v ; U ) iff w, L 0 \v U.
— w, L ( T ) iff w, L T.
0 0
— w, L ( U ) iff w, L U.
0 0

<!-- Page 1255 -->

IEEE Std Disabling of top-level properties is defined as follows:
— For T = P, w, L d T.
— For U = Q, w, L d U.
— For T = disable iff (b)P, w, L 0 d T iff some letter of w satisfies b and both w 0, i–1 T ω , L 0 P
and w 0, i–1 ⊥ω , L P for i the least index such that w i b, 0 < i < |w|.
— For U = disable iff (b)Q, w, L 0 d U iff some letter of w satisfies b and both w 0, i–1 T ω , L 0
Q and w 0, i–1 ⊥ω , L Q for i the least index such that w i b, 0 < i < |w|.
— w, L 0 d ( t v ; T ) iff w, L 0 \v d T.
— w, L 0 d ( t v ; U ) iff w, L 0 \v d U.
— w, L d ( T ) iff w, L d T.
0 0
— w, L d ( U ) iff w, L d U.
0 0
T is said to pass on w, L if w, L T. T is said to be disabled on w, L if w, L d T. T is said to fail on w,
0 0 0 0
L if T neither passes nor is disabled on w, L . It can be proved that T cannot both pass and be disabled on w,
0 0
L .
Neutral satisfaction of properties is defined as follows:
— w Q iff w, {} Q.
— w, L Q iff w, L T p (Q, 1).
0 0
— w, L 0 ( t v ; P ) iff w, L 0 \v d P.
— w, L 0 not P iff w, L 0 P.
— w, L 0 strong (R) iff there exist 0 < j < |w| and L 1 so that w 0, j, L 0 , L 1 R.
— w, L 0 weak (R) iff for every 0 < j < |w|, w 0, j T ω , L 0 strong (R).
— w, L 0 ( R |-> P ) iff for every 0 < j < |w| and L 1 so that w 0, j, L 0 , L 1 R, w j.., L 1 P.
— w, L 0 ( P ) iff w, L 0 P.
— w, L 0 ( P 1 or P 2 ) iff w, L 0 P 1 or w, L 0 P 2 .
— w, L 0 ( P 1 and P 2 ) iff w, L 0 P 1 and w, L 0 P 2 .
— w, L 0 ( nexttime P ) iff either |w| = 0 or w1.., L 0 P.
— w, L 0 ( P 1 until P 2 ) iff either there exists 0 < j < |w| so that w j.., L 0 P 2 and for every
0 < i < j, w i.., L P , or for every 0 < i < |w|, w i.., L P .
0 1 0 1
— w, L 0 ( accept_on (b) P) iff either:
• w, L P and no letter of w satisfies b, or
• For some 0 < i < |w|, w i b and w 0, i–1 T ω P. Here, w 0, –1 denotes the empty word.
F.5.6.2 Weak and strong satisfaction by finite words
The definition is identical to that without local variables, but with the understanding that the underlying
properties can have local variables.
F.5.6.3 Vacuity
The definition is identical to that without local variables (see F.5.3.3), but with the understanding that the
non non
underlying properties can have local variables and that w, L 0 ( t v ; P ) iff w, L 0 \v P.

<!-- Page 1256 -->

IEEE Std F.6 Extended expressions
This subclause describes the semantics of several constructs that are used like expressions, but whose
meaning at a point in a word may depend both on the letter at that point and on other letters in the word. By
abuse of notation, the meanings of these extended expressions are defined for letters denoted “w j” even
though they depend also on letters w i for i = j. The reason for this abuse is to make clear the way these
definitions should be used in combination with those in preceding subclauses.
F.6.1 Extended Booleans
w denotes a nonempty finite or infinite word over Σ, j denotes an integer so that 0 < j < |w|, and T(V) denotes
an instance of a clocked or unclocked sequence that is passed the local variables V as actual arguments.
— w j,L
,L
T(V).triggered iff there exist 0 < i < j and L so that both w i, j, {}, L T(V) and
L =L | U L , where D = dom(L ) – (dom(L) V).
1 0 D V 0
— w j,L
,L
@(c)(T(V).matched) iff there exists 0 < i < j so that w i,L
,L
T(V).triggered and
wi+1, j , {}, {} c [->1]).
F.6.2 Past
w denotes a nonempty finite or infinite word over Σ, and j denotes an integer so that 0 < j < |w|.
— Let n > 1. If there exist 0 < i < j so that
w i, j , {}, {} ((c && e 2) ##1 (c && e 2[=n-1] ##1 1),
then $past(e 1, n, e 2, c)[wj ] = e 1[wi ]. Otherwise, $past(e 1, n, e 2, c)[wj ] is the result of
evaluating the expression e using the initial values of the variables comprising the expression. The
initial value of a static variable is the value assigned in its declaration, or, in the absence of such an
assignment, it is the default (or uninitialized) value of the corresponding type (see 6.8, Table6-7).
The initial value of any other variable or signal is the default value of the corresponding type (see
6.8, Table6-7).
— If j < 0 then $past_gclk(e)[wj ] = e[wi–1]. $past_gclk(e)[w0 ]is the result of evaluating the
expression e using the initial values of the variables comprising the expression.
NOTE—$past(e) is equivalent to $past(e, 1, 1'b1,1'b1).
F.6.3 Future
w denotes a nonempty finite or infinite word over Σ, and j denotes an integer so that 0 < j < |w| – 1.
$future_gclk(e)[wj ] = e[wi+1 ]. If w is a finite word, $future_gclk(e)[w|w|–1] is undefined.
F.7 Recursive properties
This subclause defines the neutral semantics of properties, including top-level properties, with instances of
recursive properties in terms of the neutral semantics of properties with instances of nonrecursive properties.
The latter can be expanded to properties in the abstract syntax by applying the rewriting algorithm (see
F.4.1); therefore, their semantics is assumed to be understood.
Following are precise versions of the four restrictions given in 16.12.17 and the precise definition of
recursive property. The dependency digraph is the directed graph V, E , where V is the set of all named
properties and an order pair (p, q) is in E if, and only if, an instance of named property q appears in the
declaration of named property p. For example, for the set of properties

⊃
 

<!-- Page 1257 -->

IEEE Std property p1(v);
v |=> p2(p3());
endproperty
property p2(v);
a or (1'b1 |=> v);
endproperty
property p3;
p1(a && b);
endproperty
the dependency digraph is
{p1, p2, p3},{(p1,p2),(p1,p3),(p3,p1)}
A named property is recursive if it is in a nontrivial, strongly connected component of the dependency
digraph. An instance of named property q is recursive if it is in the declaration of a named property p so that
p and q are in the same nontrivial, strongly connected component of the dependency digraph. Here, p and q
need not be distinct properties. Define the weight of an instance of q in the declaration of p as the minimal
number of time steps that are guaranteed from the beginning of the declaration of p until the instance of q. In
the example above, the weights of p2(p3()) and of p3() in p1 are both one. Define the weight of an edge
(p, q) in the dependency digraph as the minimal weight among the weights of instances of q in the
declaration of p.
The following are the restrictions over recursive properties:
— RESTRICTION 1: The negation operator not cannot be applied to any property expression that
instantiates a property from which a recursive property can be reached in the dependency digraph.
— RESTRICTION 2: The operator disable iff cannot be used in the declaration of a recursive
property.
— RESTRICTION 3: In every cycle of the dependency digraph, the sum of the weights of the edges
shall be positive.
— RESTRICTION 4: For every recursive instance of property q in the declaration of property p, each
actual argument expression e of the instance satisfies at least one of the following conditions:
• e is itself a formal argument of p.
• No formal argument of p appears in e.
• e is bound to a local variable formal argument of q.
Let p be a named property. For k > 0, the k-fold approximation to p, denoted p[k], is a named property
without instances of recursive properties defined inductively as follows:
— The declaration of p[0] is obtained from the declaration of p by replacing the body property_spec by
the literal 1'b1.
— For k > 0, the declaration of p[k] is obtained from the declaration of p by replacing each instance of
a recursive property by the corresponding instance of its (k -1)-fold approximation and by
replacing each instance of a nonrecursive property by the corresponding instance of its k-fold
approximation.
Let π be a property, possibly the top-level property of a concurrent assertion. The k-fold approximation to π,
denoted π[k], is obtained from π by replacing each instance of a named property by the corresponding
instance of its k-fold approximation. The semantics of π is then defined as follows: for any word w over Σ
and local variable context L, w, L π iff for all k > 0, w, L π[k]. Since π[k] does not have instances of
recursive properties, its semantics is obtained using the rewriting algorithm (see F.4.1).

<!-- Page 1258 -->

IEEE Std 1800-2023