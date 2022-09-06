import socket
import json

add_bob = {
    "method": "account_add",
    "params": {
        "name": "bob"
    }
}

add_jack = {
    "method": "account_add",
    "params": {
        "name": "jack"
    }
}

account_list = {
    "method": "account_list",
    "params": {}
}

deploy_metacoin = {
    "method": "contract_deploy",
    "params": {
        "from": "bob",
        "contract_file": "tests/MetaCoin.sol"
    }
}

call_metacoin_getbalance_bob = {
    "method": "contract_call",
    "params": {
        "from": "bob",
        "contract": "MetaCoin",
        "input": "f8b2cb4f000000000000000000000000276c4b805892938e2d7da4655462075f3d7809a7"
    }
}

call_metacoin_getbalance_jack = {
    "method": "contract_call",
    "params": {
        "from": "bob",
        "contract": "MetaCoin",
        "input": "f8b2cb4f000000000000000000000000ad0a63e93190c8ea17eff0fbfc39d55293eea117"
    }
}

call_metacoin_sendcoin = {
    "method": "contract_call",
    "params": {
        "from": "bob",
        "contract": "MetaCoin",
        "input": "68d4b9c9000000000000000000000000ad0a63e93190c8ea17eff0fbfc39d55293eea11700000000000000000000000000000000000000000000000000000000000000c8"
    }
}


def account_add():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client:
        client.connect(('127.0.0.1', 8000))

        client.sendall(json.dumps(add_bob).encode(encoding='utf-8'))
        result = client.recv(1024).decode(encoding='utf-8')
        print(json.loads(result))

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client:
        client.connect(('127.0.0.1', 8000))

        client.sendall(json.dumps(add_jack).encode(encoding='utf-8'))
        result = client.recv(1024).decode(encoding='utf-8')
        print(json.loads(result))


def call_contract():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client:
        client.connect(('127.0.0.1', 8000))

        client.sendall(json.dumps(deploy_metacoin).encode(encoding='utf-8'))
        result = client.recv(1024).decode(encoding='utf-8')
        print(json.loads(result))

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client:
        client.connect(('127.0.0.1', 8000))

        client.sendall(json.dumps(
            call_metacoin_getbalance_bob).encode(encoding='utf-8'))
        result = client.recv(1024).decode(encoding='utf-8')
        print(json.loads(result))

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client:
        client.connect(('127.0.0.1', 8000))

        client.sendall(json.dumps(
            call_metacoin_sendcoin).encode(encoding='utf-8'))
        result = client.recv(1024).decode(encoding='utf-8')
        print(json.loads(result))

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client:
        client.connect(('127.0.0.1', 8000))

        client.sendall(json.dumps(
            call_metacoin_getbalance_bob).encode(encoding='utf-8'))
        result = client.recv(1024).decode(encoding='utf-8')
        print(json.loads(result))

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client:
        client.connect(('127.0.0.1', 8000))

        client.sendall(json.dumps(
            call_metacoin_getbalance_jack).encode(encoding='utf-8'))
        result = client.recv(1024).decode(encoding='utf-8')
        print(json.loads(result))


if __name__ == '__main__':
    call_contract()
