import socket
import json

# 0xc86e8742966300101238f1a5eb834530bbc7689a
add_bob = {
    "method": "account_add",
    "params": {
        "name": "bob"
    }
}

# 0x93746dcfda94c0566ac76dc7255ce526c94122a3
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

tx = {
    "from": "0xc86e8742966300101238f1a5eb834530bbc7689a",
    "to": "",
    "value": "0",
    "data": "608060405234801561001057600080fd5b5060b68061001f6000396000f3fe6080604052348015600f57600080fd5b506004361060285760003560e01c8063901717d114602d575b600080fd5b60336047565b604051603e91906067565b60405180910390f35b60006001905090565b6000819050919050565b6061816050565b82525050565b6000602082019050607a6000830184605a565b9291505056fea26469706673582212208237f441dea9dad1ed0b8161380ea48ff7cab71e3524abc1f52cf551d5e220e764736f6c634300080f0033"
}

tx_send = {
    "method": "tx_send",
    "params": tx
}

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client:
    client.connect(('127.0.0.1', 8000))
    client.sendall(json.dumps(account_list).encode(encoding='utf-8'))
    result = client.recv(1024).decode(encoding='utf-8')
    print(result)
