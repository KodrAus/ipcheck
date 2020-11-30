using System;
using System.Net;
using System.Text.Json;

class IPCheck
{
    static void Main(string[] args)
    {
        var input = args[0];
        var addr = IPAddress.Parse(input);

        var data = new {
            to_ipv4 = addr.MapToIPv4().ToString(),
            to_ipv6 = addr.MapToIPv6().ToString(),
        };

        Console.WriteLine("{0}", JsonSerializer.Serialize(data));
    }
}