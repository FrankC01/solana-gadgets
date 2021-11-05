"""datadeser

Interprets a yaml data declaration file and produces
object ready to deserialization of Solana account data.

yaml dedclaration grammar describes each section of the
overall data structure initialzed on a Solana program.
"""
from borsh_construct import *
from enum import Enum, auto
from functools import partial
from io import BytesIO
from pathlib import Path
import yaml
from yaml.loader import SafeLoader


class Node():
    """Base tree node"""
    _BORSH_TYPES = {
        "I8": I8,
        "I16": I16,
        "I32": I32,
        "I64": I64,
        "I128": I128,
        "U8": U8,
        "U16": U16,
        "U32": U32,
        "U64": U64,
        "U128": U128,
        "F32": F32,
        "F64": F64,
        "Bool": Bool,
        "Vec": Vec,
        "CStruct": CStruct,
        "Tuple": TupleStruct,
        "Bytes": Bytes,
        "String": String,
        "Enum": Enum,
        "Option": Option,
        "HashMap": HashMap,
        "HashSet": HashSet,
    }

    def __init__(self, in_dict: dict) -> None:
        self._type = in_dict['type']
        self._serialized = in_dict['serialized']
        if self._type in self._BORSH_TYPES:
            self._borsh_type = self._BORSH_TYPES[self._type]
        else:
            self._borsh_type = None

    @property
    def in_type(self) -> str:
        return self._type

    @property
    def borsh_type(self):
        return self._borsh_type

    @property
    def serialized(self) -> bool:
        return self._serialized

    def describe(self) -> None:
        print(f"Type {self.in_type} serialized = {self.serialized}")

    def deser_line(self, length: int, in_stream: BytesIO, result: list) -> list:
        result.append(self._borsh_parse_fn(
            in_stream.read1(length+in_stream.tell())))
        return result

    def deser(self, in_stream: BytesIO, result: list) -> list:
        result.append(self._borsh_parse_stream_fn(in_stream))
        return result


class Leaf(Node):
    """Commonly scalar types"""

    def __init__(self, in_dict: dict) -> None:
        super().__init__(in_dict)
        if self.borsh_type:
            self._borsh_parse_fn = self._borsh_type.parse
            self._borsh_parse_stream_fn = self._borsh_type.parse_stream

    def describe(self) -> None:
        super().describe()


class NamedField(Leaf):
    """Fields with names"""

    def __init__(self, in_dict: dict) -> None:
        inner_dict = in_dict['descriptor']
        super().__init__(inner_dict)
        self._name = inner_dict['name']

    @property
    def name(self) -> str:
        return self._name


class NodeContainer(Node):
    """Node that contains nodes"""

    def __init__(self, container_name: str, in_dict: dict) -> None:
        super().__init__(in_dict)
        self._container_name = container_name
        self._children = []
        # Check for map
        if isinstance(in_dict[container_name], dict):
            self._children.append(parse(in_dict[container_name]))
        elif isinstance(in_dict[container_name], list):
            for list_item in in_dict[container_name]:
                self._children.append(parse(list_item))
        else:
            raise ValueError(
                f"Expected dict or list, found {type(in_dict[container_name])}")

    @property
    def children(self) -> list:
        return self._children

    def describe(self) -> None:
        super().describe()
        for c in self.children:
            c.describe()


class ArrayNode(NodeContainer):
    """Fixed array construct

    Has a size indicator as well as the inner variable type"""

    def __init__(self, container_name: str, in_dict: dict) -> None:
        super().__init__(container_name, in_dict)
        self._array_size = in_dict['elements']
        self._borsh_parse_fn = self.children[0].borsh_type[self._array_size].parse
        self._borsh_parse_stream_fn = self.children[0].borsh_type[self._array_size].parse_stream


class Vector(NodeContainer):
    """Vec construct"""

    def __init__(self, container_name: str, in_dict: dict) -> None:
        super().__init__(container_name, in_dict)
        self._borsh_parse_fn = self.borsh_type(
            self.children[0].borsh_type).parse
        self._borsh_parse_stream_fn = self.borsh_type(
            self.children[0].borsh_type).parse_stream


class Tuple(NodeContainer):
    """TupleStruct construct"""

    def __init__(self, container_name: str, in_dict: dict) -> None:
        super().__init__(container_name, in_dict)
        self._borsh_parse_fn = self._borsh_type(
            *[x.borsh_type for x in self.children],).parse
        self._borsh_parse_stream_fn = self._borsh_type(
            *[x.borsh_type for x in self.children],).parse_stream


class Opt(NodeContainer):
    """Option construct"""

    def __init__(self, container_name: str, in_dict: dict) -> None:
        super().__init__(container_name, in_dict)
        self._borsh_parse_fn = self.borsh_type(
            self.children[0].borsh_type).parse
        self._borsh_parse_stream_fn = self.borsh_type(
            self.children[0].borsh_type).parse_stream


class Structure(NodeContainer):
    """Struc construct"""

    def __init__(self, container_name: str, in_dict: dict) -> None:
        super().__init__(container_name, in_dict)
        slist = []
        for x in self.children:
            slist.append((x.name / x.borsh_type))
        self._borsh_parse_stream_fn = self.borsh_type(*slist).parse_stream
        self._borsh_parse_fn = self.borsh_type(*slist).parse_stream

    def deser_line(self, length: int, in_stream: BytesIO, result: list) -> list:
        result.append(self._borsh_parse_fn(
            in_stream.read1(length+in_stream.tell())))
        return result

    def deser(self, in_stream: BytesIO, result: list) -> list:
        result.append(self._borsh_parse_stream_fn(in_stream))
        return result


class Map(NodeContainer):
    """HashMap construct"""

    def __init__(self, container_name: str, in_dict: dict) -> None:
        super().__init__(container_name, in_dict)
        self._borsh_parse_fn = self._borsh_type(
            *[x.borsh_type for x in self.children],).parse
        self._borsh_parse_stream_fn = self._borsh_type(
            *[x.borsh_type for x in self.children],).parse_stream

    def describe(self) -> None:
        print(f"{HashMap(*[x.borsh_type for x in self.children],)}")

    def deser_line(self, length: int, in_stream: BytesIO, result: list) -> list:
        result.append(self._borsh_parse_fn(
            in_stream.read1(length+in_stream.tell())))
        return result

    def deser(self, in_stream: BytesIO, result: list) -> list:
        result.append(self._borsh_parse_stream_fn(in_stream))
        return result


class Set(NodeContainer):
    """HashSet construct"""

    def __init__(self, container_name: str, in_dict: dict) -> None:
        super().__init__(container_name, in_dict)
        self._borsh_parse_fn = self.borsh_type(
            self.children[0].borsh_type).parse
        self._borsh_parse_stream_fn = self.borsh_type(
            self.children[0].borsh_type).parse_stream


class LengthPrefixNode(NodeContainer):
    """Has a length associated to size of contained type"""

    def __init__(self, container_name: str, in_dict: dict) -> None:
        in_dict['type'] = in_dict['size_type']
        super().__init__(container_name, in_dict)
        self._borsh_parse_fn = self.borsh_type.parse
        self._borsh_parse_stream_fn = self.borsh_type.parse_stream

    def describe(self) -> None:
        print(f"consumes {self.in_type} to get length of:")
        for c in self.children:
            c.describe()

    def deser(self, in_stream: BytesIO, result: list) -> list:
        length = self._borsh_parse_stream_fn(in_stream)
        result.append(length)
        self.children[0].deser_line(length, in_stream, result)
        return result


class Tree(NodeContainer):
    def __init__(self, in_dict: dict):
        self._name = [*in_dict][0]
        self._type = 'tree'
        self._serialized = False
        self._children = []
        if isinstance(in_dict[self._name], list):
            for list_item in in_dict[self._name]:
                if isinstance(list_item, dict):
                    for _, y in list_item.items():
                        self._children.append(parse(y))
                else:
                    raise ValueError(f'Expected dict found {type(list_item)}')
        else:
            raise ValueError(
                f'Expected list found {type(in_dict[self._name])}')

    def deser(self, in_stream: BytesIO) -> list:
        result = []
        for c in self.children:
            c.deser(in_stream, result)
        return result


_BIG_MAP = {
    'length_prefix': partial(LengthPrefixNode, 'contains'),
    'array': partial(ArrayNode, 'contains'),
    'Vec': partial(Vector, 'contains'),
    'Option': partial(Opt, 'contains'),
    'HashSet': partial(Set, 'contains'),
    "NamedField": NamedField,
    'Tuple': partial(Tuple, 'fields'),
    'CStruct': partial(Structure, 'fields'),
    'HashMap': partial(Map, 'fields'),
}


def parse(in_dict: dict):
    inner_type = in_dict['type']
    if inner_type in _BIG_MAP:
        return _BIG_MAP[inner_type](in_dict)
    else:
        return Leaf(in_dict)


class Deserializer():
    """Deserializer

    Deserializes data stream based on data declaration
    model"""

    def __init__(self, my_dict: dict) -> None:
        self._declaration = my_dict
        self._tree = Tree(my_dict)

    @ property
    def declaration(self) -> dict:
        return self._declaration

    @ property
    def tree(self) -> Tree:
        """Deserializer tree"""
        return self._tree

    def describe(self) -> None:
        """Describe the tree"""
        self.tree.describe()

    def deser(self, in_stream: BytesIO) -> list:
        """Deserialize the inbound bytes"""
        return self.tree.deser(in_stream)


def load_deserializer(file_name: Path) -> Deserializer:
    """Load and parse the data deserialize declaration"""
    with open(file_name) as f:
        return Deserializer(yaml.load(f, Loader=SafeLoader))
