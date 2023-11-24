from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

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
    __slots__ = ["md"]
    MD_FIELD_NUMBER: _ClassVar[int]
    md: str
    def __init__(self, md: _Optional[str] = ...) -> None: ...

class ConvertMdReply(_message.Message):
    __slots__ = ["sym"]
    SYM_FIELD_NUMBER: _ClassVar[int]
    sym: str
    def __init__(self, sym: _Optional[str] = ...) -> None: ...
