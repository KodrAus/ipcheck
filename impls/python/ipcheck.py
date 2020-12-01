#!/usr/bin/python

import sys
import ipaddress
import json

input = sys.argv[1]
addr = ipaddress.ip_address(input)

output = json.dumps({
    'to_ipv4': str(addr.ipv4_mapped) if addr.version == 6 else "<unsupported>",
    'to_ipv6': "<unsupported>",
    'is_unspecified': addr.is_unspecified,
    'is_loopback': addr.is_loopback,
    'is_reserved': addr.is_reserved,
})

# normalize output
output = output.replace("\"None\"", "null")

print(output)