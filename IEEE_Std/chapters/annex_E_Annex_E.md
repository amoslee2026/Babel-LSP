# Annex E: Annex E

## Annex E
(informative)
Optional compiler directives
E.1 General
The compiler directives described in this annex are for informative purposes only and are not part of this
standard.
This annex describes additional compiler directives as companions to the compiler directives described in
Clause22. The compiler directives described in this annex may not be available in all implementations of
SystemVerilog. The following compiler directives are described in this annex:
`default_decay_time [E.2] `delay_mode_path [E.5]
`default_trireg_strength [E.3] `delay_mode_unit [E.6]
`delay_mode_distributed [E.4] `delay_mode_zero [E.7]
E.2 `default_decay_time
The `default_decay_time compiler directive specifies the decay time for the trireg nets that do not have
any decay time specified in the declaration. This compiler directive applies to all of the trireg nets in all the
modules that follow it in the source description. An argument specifying the charge decay time shall be used
with this compiler directive.
Syntax:
`default_decay_time integer_constant | real_constant | infinite
Example 1: The following example shows how the default decay time for all trireg nets can be set to
100time units:
`default_decay_time 100
Example 2: The following example shows how to avoid charge decay on trireg nets:
`default_decay_time infinite
The keyword infinite specifies no charge decay for all the trireg nets that do not have decay time
specification.
E.3 `default_trireg_strength
The `default_trireg_strength compiler directive specifies the charge strength of trireg nets.
Syntax:
`default_trireg_strength integer_constant

<!-- Page 1234 -->

IEEE Std The integer constant shall be between 0 and 250. It indicates the relative strength of the capacitance on the
trireg net.
E.4 `delay_mode_distributed
The `delay_mode_distributed compiler directive specifies the distributed delay mode for all modules
that follow this directive in the source description.
Syntax:
`delay_mode_distributed
This compiler directive shall be used before the declaration of the module whose delay mode is being
controlled.
E.5 `delay_mode_path
The `delay_mode_path compiler directive specifies the path delay mode for all modules that follow this
directive in the source description.
Syntax:
`delay_mode_path
This compiler directive shall be used before the declaration of the module whose delay mode is being
controlled.
E.6 `delay_mode_unit
The `delay_mode_unit compiler directive specifies the unit delay mode for all modules that follow this
directive in the source description.
Syntax:
`delay_mode_unit
This compiler directive shall be used before the declaration of the module whose delay mode is being
controlled.
E.7 `delay_mode_zero
The `delay_mode_zero compiler directive specifies the zero delay mode for all modules that follow this
directive in the source description.
Syntax:
`delay_mode_zero
This compiler directive shall be used before the declaration of the module whose delay mode is being
controlled.

<!-- Page 1235 -->

IEEE Std 1800-2023