from pydoc import cli
import socket
import json

data = {
    "method": "account_add",
    "params": {
        "name": "bob"
    }
}

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client:
    client.connect(('127.0.0.1', 7878))
    client.send(bytes(json.dumps(data), encoding='utf-8'))
