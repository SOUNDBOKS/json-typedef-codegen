from dataclasses import dataclass
from typing import Any, Optional, Union, get_args, get_origin
def _from_json(cls, data):
    if data is None or cls in [bool, int, float, str] or cls is Any:
        return data
    if get_origin(cls) is Union:
        return _from_json(get_args(cls)[0], data)
    if get_origin(cls) is list:
        return [_from_json(get_args(cls)[0], d) for d in data]
    if get_origin(cls) is dict:
        return { k: _from_json(get_args(cls)[1], v) for k, v in data.items() }
    return cls.from_json(data)

def _to_json(data):
    if data is None or type(data) in [bool, int, float, str]:
        return data
    if type(data) is list:
        return [_to_json(d) for d in data]
    if type(data) is dict:
        return { k: _to_json(v) for k, v in data.items() }
    return data.to_json()
@dataclass
class Root0:
    foo: str
    @classmethod
    def from_json(cls, data) -> "Root0":
        return {
            "bar": RootBar,
            "quux": RootQuux,
        }[data["foo"]].from_json(data)
@dataclass
class RootBar(Root0):
    baz: "str"
    @classmethod
    def from_json(cls, data) -> "RootBar":
        return RootBar(
            "bar",
            _from_json(str, data["baz"]),
        )
    def to_json(self):
        return {
            "foo": "bar",
            "baz": _to_json(self.baz),
        }
@dataclass
class RootQuux(Root0):
    quuz: "str"
    @classmethod
    def from_json(cls, data) -> "RootQuux":
        return RootQuux(
            "quux",
            _from_json(str, data["quuz"]),
        )
    def to_json(self):
        return {
            "foo": "quux",
            "quuz": _to_json(self.quuz),
        }
@dataclass
class Root:
    value: "Optional[Root0]"
    @classmethod
    def from_json(cls, data) -> "Root":
        return Root(_from_json(Optional[Root0], data))
    def to_json(self):
        return _to_json(self.value)
