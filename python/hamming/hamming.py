"""Provides tools for finding hamming distance."""


def distance(strand_a, strand_b):
    """Find the hamming distance between two strands."""
    if len(strand_a) != len(strand_b):
        raise ValueError(
            "expected strands of equal length but found lengths "
            f"{len(strand_a)=}, {len(strand_b)=}"
        )
    return sum(1 for a, b in zip(strand_a, strand_b) if a != b)
