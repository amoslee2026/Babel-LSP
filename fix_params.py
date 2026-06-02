import re, sys

path = sys.argv[1] if len(sys.argv) > 1 else "/mnt/big10T/wrk/Babel-LSP/crates/mcp-server/src/server.rs"
with open(path) as f:
    content = f.read()

def find_matching_paren(s, start_pos):
    depth = 1
    i = start_pos
    in_str = False
    str_char = None
    while i < len(s) and depth > 0:
        c = s[i]
        if in_str:
            if c == "\\": i += 2; continue
            if c == str_char: in_str = False
            i += 1; continue
        if c in "\"\'":
            in_str = True; str_char = c
            i += 1; continue
        if c == "(": depth += 1
        elif c == ")": depth -= 1
        i += 1
    return i - 1 if depth == 0 else None

method_names = [
    "open_file", "read_file", "close_file", "get_symbols", "get_diagnostics",
    "get_completions", "get_definition", "find_references", "get_hover",
    "get_synthesizability", "format_source", "create_file", "update_file",
    "replace_content", "set_log_level", "get_project_memory", "list_open_files",
    "search_symbols", "search_pattern", "replace_lines", "get_line_range",
    "get_module_hierarchy", "get_memory_usage", "check_synthesizability",
    "get_file_outline", "get_instance_tree", "find_module_definition",
    "get_references", "rename_symbol",
]

param_types = [
    "OpenFileParams", "UriParam", "UpdateFileParams", "CreateFileParams",
    "ReplaceContentParams", "SearchPatternParams", "SetLogLevelParams",
    "GetDefinitionParams", "SearchSymbolsParams", "GetCompletionsParams",
    "ReplaceLinesParams", "GetLineRangeParams", "FormatSourceParams",
    "ModuleHierarchyParams", "RenameSymbolParams",
]

changes = []  # (start, end, replacement)

# Step 1: Find all calls where the first arg is a type literal { and add Parameters()
for method in method_names:
    pat = re.compile(r"\." + re.escape(method) + r"\(")
    for m in pat.finditer(content):
        call_start = m.end()

        # Skip if next char is not uppercase
        if not content[call_start].isupper():
            continue

        # Find the matching )
        closing = find_matching_paren(content, call_start)
        if closing is None:
            continue

        arg_body = content[call_start:closing]

        # Case 1: Already has Parameters( — check for extra )
        if arg_body.startswith("Parameters("):
            after = content[closing+1:closing+15].strip()
            # After the ), what follows?  .await? ; ? or another )?
            if after.startswith(")"):
                # Extra ) — remove it
                content = content[:closing+1] + content[closing+2:]
                continue

        # Case 2: No Parameters( wrapping — wrap it
        # arg_body should be something like "TypeName{...}"
        # We need to change: .method(TypeName{...}) to .method(Parameters(TypeName{...}))
        changes.append((call_start, closing, "Parameters(" + arg_body + ")"))
        print("Wrapping: .{}(TypeName{{...}}) -> .{}(Parameters(TypeName{{...}}))".format(method, method))

# Apply changes in reverse
changes.sort(key=lambda x: x[0], reverse=True)
for start, end, replacement in changes:
    content = content[:start] + replacement + content[end:]

# Step 2: Fix any remaining triple ))) patterns (from previous scripts)
# These look like: })) .await or })) ; or })) )
# Each } is actually }(struct) + )(wrong) + )(method).
# The correct pattern depends on whether Parameters( is used.

# Actually let's just handle what the errors tell us.
# Find })) before .await in test code
changes2 = []
for m in re.finditer(r"\}\)\)\)", content):
    pos = m.start()
    after = content[pos+3:pos+20].strip()
    if after.startswith(".await") or after.startswith(";") or after.startswith(")"):
        # Check if there's a matching Parameters( before this
        before = content[max(0,pos-500):pos]
        # Simple check: is there a Parameters(TypeName{ before this?
        # If yes: })) → })) is correct (struct + Parameters + method)
        # If no: })) → }) (remove one )
        has_params = re.search(r"Parameters\(\w+\{", before[::-1][:500])
        if has_params:
            pass  # already correct
        else:
            # No Parameters wrapping — remove the extra )
            content = content[:pos] + "})" + content[pos+3:]
            print("Fixed extra ) at pos", pos)

with open(path, "w") as f:
    f.write(content)
print("Done")
