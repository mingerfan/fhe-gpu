"""Serialization helpers for complex slot vectors."""

from __future__ import annotations
from typing import Any


def list_to_slots(values: list[list[float]]) -> list[complex]:
    """Convert [[re, im], ...] JSON format to Python complex list."""
    return [complex(v[0], v[1]) for v in values]


def slots_to_list(slots: list[complex]) -> list[list[float]]:
    """Convert Python complex list to [[re, im], ...] JSON format."""
    return [[v.real, v.imag] for v in slots]
