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
            asV4 = addr.MapToIPv4().ToString(),
            asV6 = addr.MapToIPv6().ToString(),
        };

        Console.WriteLine("{0}", JsonSerializer.Serialize(data));
    }
}