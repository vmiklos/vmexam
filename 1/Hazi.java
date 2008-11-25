import java.util.*;

public class Hazi {
	class Node implements Comparable<Node> {
		String name;
		int f, g, h;
		public Node(String name, int g, int h) {
			this.name = name;
			this.f = g + h;
			this.g = g;
			this.h = h;
		}
		public int compareTo(Node n) {
			Integer fi = new Integer(f);
			Integer nfi = new Integer(n.f);
			return fi.compareTo(nfi);
		}
	}
	boolean nameInList(String y, List l) {
		for (Iterator i = l.listIterator(); i.hasNext();) {
			Node n = (Node) i.next();
			if (n.name.equals(y))
				return true;
		}
		return false;
	}
	Node nodeFromList(String y, List l) {
		for (Iterator i = l.listIterator(); i.hasNext();) {
			Node n = (Node) i.next();
			if (n.name.equals(y))
				return n;
		}
		return null;
	}
	List<String> reconstructPath(HashMap<String,String>cameFrom, String currentNode){
		boolean in_keys = false;
		for (Iterator i = cameFrom.keySet().iterator(); i.hasNext(); ) {
			String s = (String) i.next();
			if (s.equals(currentNode)) {
				in_keys = true;
				break;
			}
		}
		if (in_keys) {
			List<String> p = reconstructPath(cameFrom, cameFrom.get(currentNode));
			p.add(currentNode);
			return p;
		} else {
			List<String> p = new LinkedList<String>();
			p.add(currentNode);
			return p;
		}
	}
	public Hazi() {
	}
}
