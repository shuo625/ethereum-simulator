import socket
import json

data = {
    "method": "account_list",
    "params": {
        "name": "bob"
    }
}

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client:
    client.connect(('127.0.0.1', 8000))
    client.sendall(json.dumps(data).encode(encoding='utf-8'))
    data = client.recv(1024).decode(encoding='utf-8')
    print(data)
