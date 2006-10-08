// main.java

public class main {
  static {
    System.loadLibrary("alpm_java");
  }

  public static void main(String argv[]) {
    System.out.println(alpm.alpm_initialize("/"));
    System.out.println(alpm.alpm_initialize("/"));
  }
}
