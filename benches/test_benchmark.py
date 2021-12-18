"""
Find all python based solutions via glob, and set up a
pytest-benchmark-based benchmark for each.
"""

from pathlib import Path
import importlib.util
import pytest


def yield_py_files():
    """Yield all .py file paths in year subdirs."""
    # Note: searching relative to project root ".":
    for entry in sorted(Path(".").glob("202?/**/*.py")):
        yield entry


def load_module(name, filepath):
    """Load a python module directly from a file."""
    spec = importlib.util.spec_from_file_location(name, filepath)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def name_for_mod(filepath):
    """Return a suitable name for the module at filepath."""
    return "year" + filepath.parent.parent.name + filepath.parent.name


@pytest.mark.parametrize('entry', list(yield_py_files()), ids=name_for_mod)
def test_benchmarks(entry, benchmark):
    """Benchmarks for one python-based aoc solution. (Re-used via parametrize.)"""
    name = name_for_mod(entry)
    mod = load_module(name, entry)
    benchmark(mod.main, mod.INPUT)
