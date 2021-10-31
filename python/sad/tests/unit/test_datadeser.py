import base64
import io
from pathlib import Path
import pytest
from src.datadeser import Tree, load_deserializer
import yaml
from yaml.cyaml import CSafeLoader


def test_simple_tree_pass():
    in_dict = {
        "foo": [
            {
                "initialized": {
                    "type": "Bool",
                    "serialized": False
                }
            }
        ]
    }
    Tree(in_dict)


def test_simple_tree_fail():
    in_dict = {
        "foo":
            {
                "initialized": {
                    "type": "Bool",
                    "serialized": False
                }
            }
    }
    with pytest.raises(AttributeError):
        Tree(in_dict)


def test_tree_load_pass():
    valid_path = Path(
        "descriptors/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml")
    with open(valid_path) as f:
        Tree(yaml.load(f, Loader=CSafeLoader))


def test_load_file_pass():
    valid_path = Path(
        "descriptors/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml")
    load_deserializer(valid_path)


def test_load_file_fail():
    invalid_path = Path(
        "SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml")
    with pytest.raises(FileNotFoundError):
        load_deserializer(invalid_path)


def test_deserialize_simple():
    pacc = 'ASUAAAABAAAABAAAAEFLZXkVAAAATWludGVkIGtleSB2YWx1ZSBwYWlyAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=='
    decodedBytes = base64.urlsafe_b64decode(pacc)
    valid_path = Path(
        "descriptors/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml")
    dd = load_deserializer(valid_path)
    res = dd.deser(io.BytesIO(decodedBytes))
    assert res == [True, 37, {'AKey': 'Minted key value pair'}]
