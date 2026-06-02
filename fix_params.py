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
    "get_references",
]

fixes = 0

# Fix 1: extra ) after method(Parameters(Type{...}))
for method in method_names:
    pat = re.compile(r"\." + re.escape(method) + r"\(")
    for m in pat.finditer(content):
        start = m.end()
        close_pos = find_matching_paren(content, start)
        if close_pos is None:
            continue
        body = content[start:close_pos]
        if body.startswith("Parameters("):
            after = content[close_pos+1:close_pos+10].strip()
            if after.startswith(")"):
                content = content[:close_pos+1] + content[close_pos+2:]
                fixes += 1

# Fix 2: triple ))) patterns (simplified)
triple_positions = []
for m in re.finditer(r"\)\)\)", content):
    pos = m.start()
    after_chars = content[pos+3:pos+20]
    if ".await" in after_chars[:10] or ";" in after_chars[:5]:
        triple_positions.append(pos)

for pos in reversed(triple_positions):
    content = content[:pos] + "))" + content[pos+3:]
    fixes += 1

with open(path, "w") as f:
    f.write(content)
print("Total fixes:", fixes)
