public class Hazi {
	class Node {
		String name;
		int f, g, h;
		public Node(String name, int g, int h) {
			this.name = name;
			this.f = g + h;
			this.g = g;
			this.h = h;
		}
	}
	public Hazi() {
	}
}
