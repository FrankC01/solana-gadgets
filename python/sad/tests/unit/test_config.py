
import pytest
from src.config import Config


def test_config():
    for x, y in Config().__dict__.items():
        assert y is not None


def test_setter_fails():
    cfg = Config()
    with pytest.raises(AttributeError):
        cfg.config_file = "fubar"
        cfg.default_keypair = "fubar"
        cfg.websocket_url = "fubar"
        cfg.default_keypair = "fubar"
        cfg.commitment = "fubar"
