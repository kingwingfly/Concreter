from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class ValueType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    NUMBER: _ClassVar[ValueType]
    EXPR: _ClassVar[ValueType]
NUMBER: ValueType
EXPR: ValueType

class HelloRequest(_message.Message):
    __slots__ = ["name"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    name: str
    def __init__(self, name: _Optional[str] = ...) -> None: ...

class HelloReply(_message.Message):
    __slots__ = ["message"]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    message: str
    def __init__(self, message: _Optional[str] = ...) -> None: ...

class ConvertMdRequest(_message.Message):
    __slots__ = ["md", "symbols", "symbol", "value", "type"]
    MD_FIELD_NUMBER: _ClassVar[int]
    SYMBOLS_FIELD_NUMBER: _ClassVar[int]
    SYMBOL_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    TYPE_FIELD_NUMBER: _ClassVar[int]
    md: str
    symbols: str
    symbol: str
    value: str
    type: ValueType
    def __init__(self, md: _Optional[str] = ..., symbols: _Optional[str] = ..., symbol: _Optional[str] = ..., value: _Optional[str] = ..., type: _Optional[_Union[ValueType, str]] = ...) -> None: ...

class ConvertMdReply(_message.Message):
    __slots__ = ["formula"]
    FORMULA_FIELD_NUMBER: _ClassVar[int]
    formula: str
    def __init__(self, formula: _Optional[str] = ...) -> None: ...
