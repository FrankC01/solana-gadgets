import base64
from borsh_construct import *
import io
from pathlib import Path
import pytest
from src.datadeser import Tree, load_deserializer
import yaml
from yaml.cyaml import CSafeLoader


def test_simple_tree_pass() -> None:
    in_dict = {
        "foo": [
            {
                "initialized": {
                    "type": "Bool",
                }
            }
        ]
    }
    Tree(in_dict)


def test_simple_tree_fail() -> None:
    in_dict = {
        "foo":
            {
                "initialized": {
                    "type": "Bool",
                }
            }
    }
    with pytest.raises(ValueError):
        Tree(in_dict)


def test_tree_load_pass() -> None:
    valid_path = Path(
        "descriptors/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml")
    with open(valid_path) as f:
        Tree(yaml.load(f, Loader=CSafeLoader))


def test_load_file_pass() -> None:
    valid_path = Path(
        "descriptors/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml")
    load_deserializer(valid_path)


def test_load_file_fail() -> None:
    invalid_path = Path(
        "SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml")
    with pytest.raises(FileNotFoundError):
        load_deserializer(invalid_path)


def test_deserialize_simple_map_pass() -> None:
    pacc = 'ASUAAAABAAAABAAAAEFLZXkVAAAATWludGVkIGtleSB2YWx1ZSBwYWlyAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=='
    decodedBytes = base64.urlsafe_b64decode(pacc)
    valid_path = Path(
        "descriptors/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml")
    dd = load_deserializer(valid_path)
    res = dd.deser(io.BytesIO(decodedBytes))
    assert res == [True, 37, {'AKey': 'Minted key value pair'}]


def test_scalars_pass() -> None:
    scalar_types = ['Bool',
                    'U8', 'U16', 'U32', 'U64', 'U128',
                    'I8', 'I16', 'I32', 'I64', 'I128',
                    'F32', 'F64']
    results = [True,
               255, 65535, 4294967295, 18446744073709551615, 340282366920938463463374607431768211455,
               -128, -32768, -2147483648, -9223372036854775808, -
               170141183460469231731687303715884105728,
               0.05000000074505806, -0.05]
    for i in range(len(scalar_types)):
        borsh_type = scalar_types[i]
        borsh_class = Tree._BORSH_TYPES[borsh_type]
        tree = Tree({
            "foo": [
                {
                    "initialized": {
                        "type": borsh_type,
                    }
                }
            ]
        })
        assert tree is not None
        assert tree.deser(io.BytesIO(borsh_class.build(results[i]))) == [
            results[i]]


def test_fixed_arrays_pass() -> None:
    array_types = ['U8', 'I8', 'String']
    array_sized = [3, 3, 3]
    array_values = [[127, 65, 254], [-126, -65, 92], ['foo', 'bar', 'baz']]
    array_length = len(array_types)
    for i in range(len(array_types)):
        borsh_type = array_types[i]
        borsh_class = Tree._BORSH_TYPES[borsh_type]
        tree = Tree({
            "foo": [
                {
                    "arrays": {
                        "type": 'array',
                        "elements": array_length,
                        "contains": {
                            "type": borsh_type,
                        }
                    }
                }
            ]
        })
        assert tree is not None
        results = array_values[i]
        assert tree.deser(io.BytesIO(
            borsh_class[array_length].build(results)))[0] == results


def test_vector_pass() -> None:
    array_types = ['U8', 'I8', 'String']
    array_values = [[127, 65, 254], [-126, -65, 92], ['foo', 'bar', 'baz']]
    for i in range(len(array_types)):
        borsh_type = array_types[i]
        tree = Tree({
            "foo": [
                {
                    "arrays": {
                        "type": 'Vec',
                        "contains": {
                            "type": borsh_type,
                        }
                    }
                }
            ]
        })
        assert tree is not None
        results = array_values[i]
        assert tree.deser(io.BytesIO(
            Tree._BORSH_TYPES['Vec'](Tree._BORSH_TYPES[borsh_type]).build(results)))[0] == results


def test_tuple_pass() -> None:
    tuple_fields = ['U8', 'I8', 'String']
    tuple_values = [127,  -65, 'foo']
    tuple_types = [Tree._BORSH_TYPES[x] for x in tuple_fields]
    tree = Tree({
        "foo": [
            {
                "tuples": {
                    "type": 'Tuple',
                    "fields": [
                        {
                            "type": tuple_fields[0],
                        },
                        {
                            "type": tuple_fields[1],
                        },
                        {
                            "type": tuple_fields[2],
                        },
                    ]
                }
            }
        ]
    })
    assert tree is not None
    assert tree.deser(io.BytesIO(
        Tree._BORSH_TYPES['Tuple'](*tuple_types).build(tuple_values)))[0] == tuple_values


def test_cstruc_pass():
    struc_data = {'name': 'Alice', 'age': 50}
    tree = Tree({
        "foo": [
            {
                "strucs": {
                    "type": 'CStruct',
                    "fields": [
                        {
                            "type": "NamedField",
                            "descriptor": {
                                "name": "name",
                                "type": "String",
                            }
                        },
                        {
                            "type": "NamedField",
                            "descriptor": {
                                'name': "age",
                                "type": "U32",
                            }
                        },
                    ]
                }
            }
        ]
    })
    assert tree is not None
    struc = Tree._BORSH_TYPES['CStruct'](
        'name' / Tree._BORSH_TYPES['String'],
        'age' / Tree._BORSH_TYPES['U32'])
    assert tree.deser(io.BytesIO(
        struc.build(struc_data)))[0] == struc_data


def test_hashset_pass() -> None:
    array_types = ['U8', 'I8', 'String']
    array_values = [{127, 65, 65}, {-126, -65, 92}, {'foo', 'bar', 'bar'}]
    array_results = [{127, 65}, {-126, -65, 92}, {'foo', 'bar'}]
    for i in range(len(array_types)):
        borsh_type = array_types[i]
        tree = Tree({
            "foo": [
                {
                    "sets": {
                        "type": 'HashSet',
                        "contains": {
                            "type": borsh_type,
                        }
                    }
                }
            ]
        })
        assert tree is not None
        assert tree.deser(io.BytesIO(
            Tree._BORSH_TYPES['HashSet'](
                Tree._BORSH_TYPES[borsh_type]).build(array_values[i])))[0] == array_results[i]


def test_option_pass() -> None:
    array_types = ['U8', 'String']
    value_counts = 2
    array_values = [[None, 255], ['foo', None]]
    for i in range(len(array_types)):
        borsh_type = array_types[i]
        tree = Tree({
            "foo": [
                {
                    "arrays": {
                        "type": 'Option',
                        "contains": {
                            "type": borsh_type,
                        }
                    }
                }
            ]
        })
        assert tree is not None
        for j in range(value_counts):
            result = array_values[i][j]
            assert tree.deser(io.BytesIO(
                Tree._BORSH_TYPES['Option'](Tree._BORSH_TYPES[borsh_type]).build(result)))[0] == result
