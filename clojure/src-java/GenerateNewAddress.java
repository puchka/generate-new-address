public class GenerateNewAddress {
  public static native String getNewAddress(String pub_key);

  static {
    System.loadLibrary("generate_new_address");
  }

  public static void main(String[] args) {
    String output = GenerateNewAddress.getNewAddress(args[0]);
    System.out.println(output);
  }
}
