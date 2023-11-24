from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class NerRequest(_message.Message):
    __slots__ = ["text", "field"]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    FIELD_FIELD_NUMBER: _ClassVar[int]
    text: str
    field: str
    def __init__(self, text: _Optional[str] = ..., field: _Optional[str] = ...) -> None: ...

class NerReply(_message.Message):
    __slots__ = ["ner_ret"]
    NER_RET_FIELD_NUMBER: _ClassVar[int]
    ner_ret: str
    def __init__(self, ner_ret: _Optional[str] = ...) -> None: ...
