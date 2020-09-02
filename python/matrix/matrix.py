import typing as t


class Matrix:
    def __init__(self, matrix_string):
        self._as_rows: t.List[t.List[int]] = [
            [int(col) for col in row.split()]
            for row in matrix_string.splitlines()
        ]

    def row(self, index):
        zero_based_index = index - 1
        return self._as_rows[zero_based_index]

    def column(self, index):
        zero_based_index = index - 1
        return [row[zero_based_index] for row in self._as_rows]
