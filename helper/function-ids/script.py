import re
from bs4 import BeautifulSoup

# Load the HTML file
with open("table.html", "r", encoding="utf-8") as file:
    html_content = file.read()

# Parse the HTML
soup = BeautifulSoup(html_content, "html.parser")

# Extract all function IDs
function_map = []
for row in soup.find_all("tr"):
    cells = row.find_all("td")
    if len(cells) > 1:
        function_text = cells[0].get_text(strip=True)
        match = re.search(r'function:([\w-]+)$', function_text)
        if match:
            function_id = function_text  # Full function ID (URI)
            function_name = match.group(1)  # Extracted short name
            function_map.append((function_id, function_name))

# Convert to Rust-style PascalCase enum variants
def to_rust_enum(name):
    return re.sub(r'(^|-)([a-z])', lambda m: m.group(2).upper(), name)

rust_enum_variants = [to_rust_enum(name) for _, name in function_map]

# Generate Rust enum definition
rust_enum = """use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XacmlFunction {
    """ + ",\n    ".join(rust_enum_variants) + """
}

impl FromStr for XacmlFunction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
""" + "\n".join([f'            "{fid}" => Ok(Self::{to_rust_enum(name)}),' for fid, name in function_map]) + """
            _ => Err(()),
        }
    }
}
"""

# Save to a Rust file
with open("xacml_enum.rs", "w", encoding="utf-8") as rust_file:
    rust_file.write(rust_enum)

print("Rust enum with FromStr implementation successfully generated!")
