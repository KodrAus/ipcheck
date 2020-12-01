package main
import "net"
import "fmt"
import "log"
import "os"
import "strings"
import "encoding/json"

func main() {
    // note: only capitalized fields are public and are serialized
    type Data struct {
        ToIpv4 string
        ToIpv6 string
        IsUnspecified bool
        IsLoopback bool
        IsReserved string
    }

    addr := os.Args[1]
    ip := net.ParseIP(addr)

    json, err := json.Marshal(Data {
        "<unsupported>",
        "<unsupported>",
        ip.IsUnspecified(),
        ip.IsLoopback(),
        "<unsupported>",
    })

    if err != nil {
        log.Fatal(err)
        os.Exit(1)
    }

    output := string(json)

    // normalize field names
    output = strings.Replace(output, "ToIpv4", "to_ipv4", 1)
    output = strings.Replace(output, "ToIpv6", "to_ipv6", 1)
    output = strings.Replace(output, "IsUnspecified", "is_unspecified", 1)
    output = strings.Replace(output, "IsLoopback", "is_loopback", 1)
    output = strings.Replace(output, "IsReserved", "is_reserved", 1)

    // normalize null
    output = strings.Replace(output, "\"\\u003cnil\\u003e\"", "null", -1)
    output = strings.Replace(output, "\"\"", "null", -1)

    fmt.Println(output)
}
