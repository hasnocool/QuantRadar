#!/usr/bin/env python3
"""Lazy codemap.md generator.

Fills in placeholder codemap.md files (those with <!-- Fixer: -->)
and creates new ones for crates that lack them.

Scans:
  - crates/shared/*/codemap.md and crates/shared/*/src/codemap.md
  - crates/standalone/*/codemap.md and crates/standalone/*/src/codemap.md
  - configs/codemap.md (project root)

Generates sections: Responsibility, Source Map, Dependencies, Tests, Integration.
Reads lib.rs and Cargo.toml for metadata; falls back to crate-name heuristics.
"""
import os
import re
import sys
from pathlib import Path

BASE = Path("/home/hyperion/Projects/QuantRadar")
CRATES_SHARED = BASE / "crates" / "shared"
CRATES_STANDALONE = BASE / "crates" / "standalone"
CONFIGS = BASE / "configs"
# Additional directories with codemap.md files
PYTHON_QUANT = BASE / "python" / "quantaradar"
PYTHON_FS = BASE / "python" / "feature_store"


def read_file(path: Path) -> str:
    """Read a file, return empty string if missing."""
    try:
        return path.read_text()
    except FileNotFoundError:
        return ""


def extract_responsibility(content: str) -> str:
    """Extract the Responsibility section, or return empty if placeholder."""
    if "<!-- Fixer:" in content:
        return ""
    m = re.search(r"## Responsibility\s*\n(.+?)(?=\n## |\n\n## |\Z)", content, re.DOTALL)
    if m:
        txt = m.group(1).strip()
        if not txt or "<!--" in txt:
            return ""
        return txt
    return ""


def extract_cargo_deps(crate_path: Path) -> list:
    """Parse Cargo.toml for dependencies (simple, best-effort)."""
    cargo = crate_path / "Cargo.toml"
    if not cargo.exists():
        return []
    text = cargo.read_text()
    deps = []
    in_deps = False
    for line in text.splitlines():
        stripped = line.strip()
        if stripped == "[dependencies]":
            in_deps = True
            continue
        if stripped.startswith("["):
            in_deps = False
        if in_deps and stripped and not stripped.startswith("#"):
            dep = stripped.split()[0].split(",")[0]
            deps.append(dep)
    return deps


def find_codemap_paths() -> list:
    """Find all codemap.md paths and their crate/context info."""
    paths = []

    # Shared crates
    if CRATES_SHARED.exists():
        for entry in sorted(CRATES_SHARED.iterdir()):
            if not entry.is_dir():
                continue
            crate_name = entry.name
            crate_path = entry
            # Root codemap.md
            root_cm = crate_path / "codemap.md"
            # src/codemap.md
            src_cm = crate_path / "src" / "codemap.md"

            for cm_path in [root_cm, src_cm]:
                if cm_path.exists():
                    codemap_text = read_file(cm_path)
                    has_cargo = (crate_path / "Cargo.toml").exists()
                    paths.append({
                        "crate_name": crate_name,
                        "crate_path": crate_path,
                        "codemap_path": cm_path,
                        "codemap_text": codemap_text,
                        "root": str(cm_path.relative_to(BASE)).replace("codemap.md", ""),
                        "has_cargo": has_cargo,
                        "is_root": str(cm_path).count("/src/") == 0,
                    })

    # Standalone crates
    if CRATES_STANDALONE.exists():
        for entry in sorted(CRATES_STANDALONE.iterdir()):
            if not entry.is_dir():
                continue
            crate_name = entry.name
            crate_path = entry
            root_cm = crate_path / "codemap.md"
            src_cm = crate_path / "src" / "codemap.md"

            for cm_path in [root_cm, src_cm]:
                if cm_path.exists():
                    codemap_text = read_file(cm_path)
                    has_cargo = (crate_path / "Cargo.toml").exists()
                    paths.append({
                        "crate_name": crate_name,
                        "crate_path": crate_path,
                        "codemap_path": cm_path,
                        "codemap_text": codemap_text,
                        "root": str(cm_path.relative_to(BASE)).replace("codemap.md", ""),
                        "has_cargo": has_cargo,
                        "is_root": str(cm_path).count("/src/") == 0,
                    })

    # Root configs
    if CONFIGS.exists():
        cm = CONFIGS / "codemap.md"
        if cm.exists():
            paths.append({
                "crate_name": "configs",
                "crate_path": CONFIGS,
                "codemap_path": cm,
                "codemap_text": read_file(cm),
                "root": "configs/",
                "has_cargo": False,
                "is_root": True,
            })

    # Python quantaradar modules
    if PYTHON_QUANT.exists():
        for entry in sorted(PYTHON_QUANT.iterdir()):
            if not entry.is_dir():
                continue
            crate_name = entry.name
            crate_path = entry
            root_cm = crate_path / "codemap.md"
            src_cm = crate_path / "src" / "codemap.md" if (crate_path / "src").exists() else None

            for cm_path in [root_cm] + ([src_cm] if src_cm else []):
                if cm_path.exists():
                    codemap_text = read_file(cm_path)
                    # Python crates don't have Cargo.toml or lib.rs
                    paths.append({
                        "crate_name": crate_name,
                        "crate_path": crate_path,
                        "codemap_path": cm_path,
                        "codemap_text": codemap_text,
                        "root": str(cm_path.relative_to(BASE)).replace("codemap.md", ""),
                        "has_cargo": False,
                        "is_root": str(cm_path).count("/src/") == 0,
                    })

    # Python feature_store module
    if PYTHON_FS.exists():
        for entry in sorted(PYTHON_FS.iterdir()):
            if not entry.is_dir():
                continue
            crate_name = entry.name
            crate_path = entry
            root_cm = crate_path / "codemap.md"

            for cm_path in [root_cm]:
                if cm_path.exists():
                    codemap_text = read_file(cm_path)
                    paths.append({
                        "crate_name": crate_name,
                        "crate_path": crate_path,
                        "codemap_path": cm_path,
                        "codemap_text": codemap_text,
                        "root": str(cm_path.relative_to(BASE)).replace("codemap.md", ""),
                        "has_cargo": False,
                        "is_root": True,
                    })

    return paths


def infer_responsibility(crate_name: str, lib_content: str) -> str:
    """Lazily infer responsibility from crate name and lib.rs content."""
    name = crate_name.lower()
    if "data" in name or "ingest" in name:
        return "Data ingestion and validation pipeline."
    if "exchange" in name:
        return f"Ingestion adapter for {crate_name}."
    if "feature" in name:
        return "Feature computation and engineering."
    if "signal" in name or "ensemble" in name:
        return "Signal generation and ensemble modeling."
    if "ranking" in name:
        return "Cross-sectional ranking and scoring."
    if "regime" in name:
        return "Regime detection and market regime analysis."
    if "report" in name or "output" in name:
        return "Report generation and output formatting."
    if "storage" in name:
        return "Data storage and persistence."
    if "model" in name or "registry" in name:
        return "Model management and registry."
    if "dashboard" in name:
        return "Dashboard and visualization."
    if "cli" in name:
        return "Command-line interface and command dispatch."
    if "core" in name:
        return "Core system functionality."
    if "experiment" in name:
        return "Experiment framework and reproducibility."
    if "backtest" in name:
        return "Backtest execution and performance analysis."
    if "event" in name:
        return "Event handling and market-data distribution."
    if "order" in name or "book" in name:
        return "Order book and execution logic."
    if "microstructure" in name:
        return "Market microstructure analysis."
    if "news" in name:
        return "News ingestion and processing."
    if "screen" in name or "screener" in name:
        return "Screening and filtering pipeline."
    if "expected" in name:
        return "Expected return computation."
    if re.search(r"\bpca\b", name) or re.search(r"\brank\b", name):
        return "Dimensionality reduction and ranking."
    if "pca" in name:
        return "Principal component analysis and features."
    if "frequency" in name or "freq" in name:
        return "Frequency-domain analysis."
    return "Module functionality to be documented."


def key_items_from_lib(lib: str) -> list:
    """Extract a few key item names from lib.rs."""
    items = []
    for line in lib.splitlines():
        s = line.strip()
        if not s or s.startswith("#") or s.startswith("///") or s.startswith("//!"):
            continue
        s = s.strip('#"').strip()
        if s and not s.startswith("fn ") and not s.startswith("pub fn"):
            items.append(s)
        if len(items) >= 5:
            break
    return items


def generate_codemap(crate_name: str, crate_path: Path,
                     codemap_path: Path, codemap_text: str,
                     lib_content: str, deps: list) -> str:
    """Generate a codemap.md, preserving existing good sections."""
    has_tests = "#[cfg(test)]" in lib_content

    # If already has meaningful content (no Fixer placeholder, has Responsibility text),
    # preserve what's there and augment missing sections
    existing_resp = extract_responsibility(codemap_text)
    better_resp = existing_resp or infer_responsibility(crate_name, lib_content)

    # If the whole file is just a placeholder, generate from scratch
    is_placeholder = "<!-- Fixer:" in codemap_text
    minimal = codemap_text.strip().startswith("# ") and len(codemap_text.strip()) < 150

    lines = [f"#{crate_name}"]

    # --- Responsibility ---
    lines.append("## Responsibility")
    if is_placeholder or minimal:
        lines.append(better_resp)
    else:
        # Preserve existing responsibility, ensure it's not just a comment
        if existing_resp and "<!--" not in existing_resp:
            lines.append(existing_resp)
        else:
            lines.append(better_resp)

    # --- Source Map ---
    lines.append("## Source Map")
    if is_placeholder or minimal:
        lib_path = "src/lib.rs"
        if lib_content:
            items = key_items_from_lib(lib_content)
            if items:
                for item in items[:5]:
                    lines.append(f"- `{item}`")
            else:
                lines.append(f"- `{lib_path}`: main module")
        else:
            lines.append(f"- `{lib_path}`: not found")
    else:
        # Preserve existing source map if present
        sm_match = re.search(r"## Source Map\s*\n(.+?)(?=\n## |\n\n## |\Z)", codemap_text, re.DOTALL)
        if sm_match:
            lines.append(sm_match.group(1).strip())
        else:
            lib_path = "src/lib.rs"
            if lib_content:
                items = key_items_from_lib(lib_content)
                for item in items[:3]:
                    lines.append(f"- `{item}`")

    # --- Dependencies ---
    lines.append("## Dependencies")
    if is_placeholder or minimal:
        if deps:
            for dep in deps[:10]:
                lines.append(f"- `{dep}`")
        else:
            lines.append("_No dependencies parsed_")
    else:
        # Preserve existing deps
        dm_match = re.search(r"## Dependencies\s*\n(.+?)(?=\n## |\n\n## |\Z)", codemap_text, re.DOTALL)
        if dm_match:
            lines.append(dm_match.group(1).strip())
        elif deps:
            for dep in deps[:10]:
                lines.append(f"- `{dep}`")
        else:
            lines.append("_No dependencies parsed_")

    # --- Tests ---
    lines.append("## Tests")
    if is_placeholder or minimal:
        if has_tests:
            lines.append("- `cargo test -p <name>`")
        else:
            lines.append("_No test configuration found_")
    else:
        # Preserve existing tests section
        tm_match = re.search(r"## Tests\s*\n(.+?)(?=\n## |\n\n## |\Z)", codemap_text, re.DOTALL)
        if tm_match:
            lines.append(tm_match.group(1).strip())
        elif has_tests:
            lines.append("- `cargo test -p <name>`")
        else:
            lines.append("_No test configuration found_")

    # --- Integration ---
    lines.append("## Integration")
    lines.append(
        f"- Part of the `{crate_name}` crate in the QuantRadar workspace")

    return "\n\n".join(lines) + "\n"


def main():
    paths = find_codemap_paths()
    total = len(paths)
    improved = 0
    created = 0
    skipped = 0

    for i, crate in enumerate(paths, 1):
        name = crate["crate_name"]
        path = crate["crate_path"]
        cm_path = crate["codemap_path"]
        cm_text = crate["codemap_text"]
        has_cargo = crate["has_cargo"]
        is_root = crate["is_root"]

        # Get lib.rs content and deps if Rust crate
        lib_content = ""
        deps = []
        if has_cargo:
            lib_path = path / "src" / "lib.rs"
            if lib_path.exists():
                lib_content = lib_path.read_text()
            deps = extract_cargo_deps(path)

        # Determine if we should replace/improve or skip
        is_placeholder = "<!-- Fixer:" in cm_text
        minimal = cm_text.strip().startswith("# ") and len(cm_text.strip()) < 150
        has_meaningful = (
            not is_placeholder and
            not minimal and
            bool(re.search(r"## Responsibility\s*\n[^\n<!--]", cm_text, re.MULTILINE))
        )

        if has_meaningful:
            # Already has good content - skip but count it
            skipped += 1
            continue

        # Generate improved codemap
        codemap_md = generate_codemap(name, path, cm_path, cm_text,
                                      lib_content, deps)

        # Write it out
        cm_path.write_text(codemap_md)

        if is_placeholder:
            improved += 1  # replaced a placeholder
        elif not cm_path.exists() or minimal:
            created += 1  # created new where none existed or was very minimal
        else:
            improved += 1  # improved incomplete content

    print(f"Done: {total} codemap.md files processed")
    print(f"  Improved (placeholder -> content): {improved}")
    print(f"  Created (new codemap.md): {created}")
    print(f"  Skipped (already had content): {skipped}")


if __name__ == "__main__":
    main()