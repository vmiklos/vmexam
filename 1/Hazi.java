import java.util.*;
import java.io.*;

public class Hazi {
	boolean inhn = false;
	boolean ingn = false;
	String start = null;
	String end = null;
	HashMap<String,Integer> hn;
	HashMap<String,HashMap<String,Integer>> gn;
	HashMap<String,String> cameFrom;
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
	boolean aStar(String start, String end) {
		try {
			BufferedWriter sock = new BufferedWriter(
					new OutputStreamWriter(
						new FileOutputStream("output.txt"), "8859_2")
					);
			List<Node> openlist = new LinkedList<Node>();
			openlist.add(new Node(start, 0, hn.get(start)));
			List<Node> closedlist = new LinkedList<Node>();
			for (int count = 0; openlist.size() > 0; count++) {
				Collections.sort(openlist);
				sock.write("(:openlist " + count);
				for (Iterator i = openlist.listIterator(); i.hasNext();) {
					Node n = (Node) i.next();
					sock.write(" ("+n.f+" "+n.name+")");
				}
				sock.write(")");
				sock.newLine();
				sock.write("(:closedlist " + count);
				for (Iterator i = closedlist.listIterator(); i.hasNext();) {
					Node n = (Node) i.next();
					sock.write(" ("+n.f+" "+n.name+")");
				}
				sock.write(")");
				sock.newLine();
				
				Node x = openlist.get(0);
				openlist.remove(0);
				if (x.name.equals(end)) {
					sock.write("(:sol "+x.f);
					List<String> l = reconstructPath(cameFrom, end);
					for (Iterator i = l.listIterator(); i.hasNext();) {
						String s = (String) i.next();
						sock.write(" "+s);
					}
					sock.write(")");
					sock.newLine();
					return true;
				}
				closedlist.add(x);
				for (Iterator i = gn.get(x.name).keySet().iterator(); i.hasNext(); ) {
					String y = (String) i.next();
					if (nameInList(y, closedlist))
						continue;
					int tentative_g_score = x.g + gn.get(x.name).get(y);
					boolean tentative_is_better = false;
					if (!nameInList(y, openlist)) {
						openlist.add(new Node(y, tentative_g_score, hn.get(y)));
						tentative_is_better = true;
					} else if (tentative_g_score < nodeFromList(y, openlist).g) {
						tentative_is_better = true;
					}
					if (tentative_is_better == true)
						cameFrom.put(y, x.name);
				}
			}
		} catch(Exception e) {
			e.printStackTrace();
		}
		return false;
	}
	public Hazi(String filename) {
		try {
			BufferedReader sock = new BufferedReader(new FileReader(new File(filename)));
			String i;
			while((i = sock.readLine()) != null) {
				String line = i.trim();
				StringTokenizer tokens = new StringTokenizer(line);
				String prefix = tokens.nextToken();
				if (prefix.equals("(:start")) {
					String s = tokens.nextToken();
					start = s.substring(0, s.length()-1);
					System.out.println("debug, start: '"+start+"'");
				} else if (prefix.equals("(:end")) {
					String s = tokens.nextToken();
					end = s.substring(0, s.length()-1);
					System.out.println("debug, end: '"+end+"'");
				} else if (prefix.equals("(:hn")) {
					inhn = true;
				} else if (prefix.equals("(:gn")) {
					ingn = true;
				} else if (prefix.substring(0, 1).equals("(")) {
					StringTokenizer t = new StringTokenizer(line.substring(1, line.length()-1));
					String key = t.nextToken();
					String value = t.nextToken();
					if (inhn) {
						System.out.println("hn k/v: '"+key+"'/'"+value+"'");
						hn.put(key, Integer.parseInt(value));
					} else if (ingn) {
					}
				}
			}
		} catch(Exception e) {
			e.printStackTrace();
		}
	}
}
