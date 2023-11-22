from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class NerRequest(_message.Message):
    __slots__ = ["text", "region"]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    REGION_FIELD_NUMBER: _ClassVar[int]
    text: str
    region: str
    def __init__(self, text: _Optional[str] = ..., region: _Optional[str] = ...) -> None: ...

class NerReply(_message.Message):
    __slots__ = ["ner_ret"]
    NER_RET_FIELD_NUMBER: _ClassVar[int]
    ner_ret: str
    def __init__(self, ner_ret: _Optional[str] = ...) -> None: ...
