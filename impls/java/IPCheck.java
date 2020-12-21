import java.net.InetAddress;
import javax.json.*;

public class IpCheck {
     public static void main(String []args) throws Exception {
        String input = args[0];
        InetAddress addr = InetAddress.getByName(input);

        JsonObject json = Json.createObjectBuilder()
            .add("to_ipv4", "<unsupported>")
            .add("to_ipv6", "<unsupported>")
            .add("is_unspecified", "<unsupported>")
            .add("is_loopback", addr.isLoopbackAddress())
            .add("is_reserved", "<unsupported>")
            .build();

        System.out.print(json.toString());
     }
}
