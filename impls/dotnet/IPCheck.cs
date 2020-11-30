using System;
using System.Net;
using System.Text.Json;

class Program
{
    static void Main(string[] args)
    {
        var addr = args[0];

        Console.WriteLine("{0}", JsonSerializer.Serialize<Result>(Check(addr)));
    }

    static Result Check(string addr)
    {
        var ipAddr = IPAddress.Parse(addr);

        var asV4 = ipAddr.MapToIPv4();
        var asV6 = ipAddr.MapToIPv6();

        return new Result
        {
            input = addr,
            rendered = ipAddr.ToString(),
            asV4 = asV4.ToString(),
            asV6 = asV6.ToString(),
        };
    }
}

record Result
{
    public string input { get; init; }
    public string rendered { get; init; }
    public string asV4 { get; init; }
    public string asV6 { get; init; }
}
