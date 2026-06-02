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

changes = []

for method in method_names:
    pat = re.compile(r"\." + re.escape(method) + r"\(")
    for m in pat.finditer(content):
        call_start = m.end()

        # Skip if next 11 chars are "Parameters(" (already wrapped)
        if content[call_start:call_start+11] == "Parameters(":
            continue

        # Skip if next char is not uppercase (not a type literal)
        if not content[call_start].isupper():
            continue

        # Find the matching )
        closing = find_matching_paren(content, call_start)
        if closing is None:
            continue

        arg_body = content[call_start:closing]

        # Double-check: not already wrapped
        if arg_body.startswith("Parameters("):
            continue

        # Wrap with Parameters()
        changes.append((call_start, closing, "Parameters(" + arg_body + ")"))
        prefix = content[max(0,m.start()-20):m.start()]
        print("Wrapping: ...{}.{}({})".format(prefix[-30:], method, arg_body[:30]))

# Apply changes in reverse
changes.sort(key=lambda x: x[0], reverse=True)
for start, end, replacement in changes:
    content = content[:start] + replacement + content[end:]

# Step 2: Remove any double Parameters(Parameters(...))
while "Parameters(Parameters(" in content:
    content = content.replace("Parameters(Parameters(", "Parameters(")
    # Also need to fix the extra closing )
    # Find: Parameters(Type{...})) -> keep only one)
    # This is harder. Let me just remove double Parameters and fix closing later.

# Clean up double parens from double wrapping
# Pattern: Parameters(Parameters(T{...}))) -> Parameters(T{...}))
# After removing one Parameters(, we have one extra )
# Simplest: just remove "Parameters(Parameters(" and let cargo check find remaining issues

with open(path, "w") as f:
    f.write(content)
print("Applied {} wrappings".format(len(changes)))
