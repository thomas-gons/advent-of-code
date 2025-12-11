#!/bin/bash
set -euo pipefail

remove_existing=false
day_number=""

# 1. Parse arguments
for arg in "$@"; do
    case "$arg" in
        --rm)
            remove_existing=true
            ;;
        *)
            day_number="$arg"
            ;;
    esac
done

if [ -z "$day_number" ]; then
    echo "Usage: $0 [--rm] <day_number>"
    exit 1
fi

day_number=$(printf "%02d" "$day_number") # Force two digits (e.g. 08)
_dir="day-$day_number"
workspace_toml="Cargo.toml"
template_file="scripts/day-template.rs"

# 2. Logic to remove existing day
if [ "$remove_existing" = true ]; then
  # 1. Vérification du travail existant avant suppression
  main_rs="$_dir/src/main.rs"
  needs_confirmation=false

  if [ -f "$main_rs" ]; then
    count=$(grep -c "aoc::not_implemented()" "$main_rs" || true) 
    
    if [ "$count" -lt 2 ]; then
      needs_confirmation=true
    fi
  fi
  
  if [ "$needs_confirmation" = true ]; then
    parts_implemented=$((2 - count))
    echo -e "\033[33;1m-> Warning : The code in $_dir appears to have been partially or fully implemented ($parts_implemented parts implemented).\033[0m"
    read -r -p "Are you sure you want to remove '$_dir'? [Y/n] " response
    response=${response,,} # tolower
    
    if [[ "$response" =~ ^(n|no)$ ]]; then
      echo -e "\033[34;1m Aborted.\033[0m"
      exit 0
    fi
  fi

  echo -e "\033[31;1m-> Removing existing $_dir...\033[0m"
  rm -rf "$_dir"
  # Remove from workspace if present
  sed -i "/\"$_dir\"/d" "$workspace_toml"
  exit 0
fi

if [ -d "$_dir" ]; then
  echo -e "\033[31;1m-> $_dir challenge already exists. Aborting.\033[0m"
  exit 1
fi

# 3. Create directories manually (Replacing cargo new)
# This prevents Cargo from auto-updating the workspace file messily
mkdir -p "$_dir/src"
mkdir -p "$_dir/data"
touch "$_dir/data/input.txt"

# 4. Generate main.rs from template
if [ -f "$template_file" ]; then
    template=$(sed "s|day-XX|day-$day_number|g" "$template_file")
    echo "$template" > "$_dir/src/main.rs"
else
    echo "fn main() { println!(\"Day $day_number\"); }" > "$_dir/src/main.rs"
    echo -e "\033[33mWarning: Template file not found, using default.\033[0m"
fi

# 5. Generate Cargo.toml for the package
cat <<EOF > "$_dir/Cargo.toml"
[package]
name = "day-$day_number"
version = "0.1.0"
edition = "2024"

[dependencies]
anyhow = "1.0"
aoc = { path = "../common" }
EOF

# 6. Register in Root Cargo.toml (Cleanly)
if ! grep -q "\"$_dir\"" "$workspace_toml"; then
  sed -i "/^\s*\]/i     \"$_dir\"," "$workspace_toml"
fi

echo -e "\033[32;1m-> Day $day_number initialized!\033[0m"