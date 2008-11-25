import java.util.*;
import java.io.*;

/**
 * Ez a class valositja meg tenylegesen a hazit.
 */
public class Hazi {
	/**
	 * A prserben: bennevagyunk-e egy hn blokkban.
	 */
	boolean inhn = false;
	/**
	 * Parser: bennevagyunk-e egy gn blokkban.
	 */
	boolean ingn = false;
	/**
	 * A kiindulo node neve.
	 */
	String start = null;
	/**
	 * A cel neve.
	 */
	String end = null;
	/**
	 * Ebben fogjuk tarolni a heurisztika erteteket.
	 */
	HashMap<String,Integer> hn;
	/**
	 * Ebben pedig az egyes node-ok tavolsagat.
	 */
	HashMap<String,HashMap<String,Integer>> gn;
	/**
	 * Egy lista arrol, hogy honnan jottunk.
	 */
	HashMap<String,String> cameFrom;
	/**
	 * Egy varost valosit meg.
	 */
	class Node implements Comparable<Node> {
		/**
		 * Nev.
		 */
		String name;
		/**
		 * f(x) = g(x) + h(x)
		 *
		 * Ahol g(x) az idaig megtett ut, h(x) pedig a heurisztika.
		 */
		int f, g, h;
		/**
		 * Konstruktor.
		 */
		public Node(String name, int g, int h) {
			this.name = name;
			this.f = g + h;
			this.g = g;
			this.h = h;
		}
		/**
		 * Osszehasonlito, rendezeshez.
		 */
		public int compareTo(Node n) {
			Integer fi = new Integer(f);
			Integer nfi = new Integer(n.f);
			return fi.compareTo(nfi);
		}
	}
	/**
	 * Megnezi, hogy egy adott nevu varos bennevan-e a listaban.
	 */
	boolean nameInList(String y, List l) {
		for (Iterator i = l.listIterator(); i.hasNext();) {
			Node n = (Node) i.next();
			if (n.name.equals(y))
				return true;
		}
		return false;
	}
	/**
	 * Visszaad egy varost a neve alapjan egy listabol.
	 */
	Node nodeFromList(String y, List l) {
		for (Iterator i = l.listIterator(); i.hasNext();) {
			Node n = (Node) i.next();
			if (n.name.equals(y))
				return n;
		}
		return null;
	}
	/**
	 * A cameFrom tartalmazza, hogy az egyes pontok eseten honnan jottunk.
	 * Ez a fuggveny ebbol epit egy listat.
	 */
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
	/**
	 * Maga az A* algoritmus.
	 */
	boolean aStar(String start, String end) {
		try {
			// Megnyitjuk a kimeneti file-t
			BufferedWriter sock = new BufferedWriter(
					new OutputStreamWriter(
						new FileOutputStream("output.txt"), "8859_2")
					);
			// A kiertekelendo node-ok listaja, kezdetben csak a kiindulasi ponttal
			List<Node> openlist = new LinkedList<Node>();
			openlist.add(new Node(start, 0, hn.get(start)));
			// A mar kiertekelt node-ok listaja, kezdetben ures
			List<Node> closedlist = new LinkedList<Node>();
			// Addig megyunk amig van mit kifejteni
			for (int count = 0; openlist.size() > 0; count++) {
				// Kiirjuk a mostani allapotunkat
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
				
				// Legyen x az openlistbeli elemek kozul a
				// legkisebb f(x) ertekkel rendelkezo
				Node x = openlist.get(0);
				openlist.remove(0);
				if (x.name.equals(end)) {
					// Keszen vagyunk, kiirjuk a megoldast
					// es visszaterunk
					sock.write("(:sol "+x.f);
					List<String> l = reconstructPath(cameFrom, end);
					for (Iterator i = l.listIterator(); i.hasNext();) {
						String s = (String) i.next();
						sock.write(" "+s);
					}
					sock.write(")");
					sock.close();
					return true;
				}
				closedlist.add(x);
				// X minden egyes szomszedjara
				for (Iterator i = gn.get(x.name).keySet().iterator(); i.hasNext(); ) {
					String y = (String) i.next();
					if (nameInList(y, closedlist))
						continue;
					// g(x) es x-y tavolsaga
					int tentative_g_score = x.g + gn.get(x.name).get(y);
					boolean tentative_is_better = false;
					if (!nameInList(y, openlist)) {
						// Harom parameter: nev, g(x), h(x)
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
	/**
	 * A hazi class konstuktora.
	 */
	public Hazi(String filename) {
		// Beolvassuk a bemeneti file-t.
		try {
			hn = new HashMap<String,Integer>();
			gn = new HashMap<String,HashMap<String,Integer>>();
			cameFrom = new HashMap<String,String>();
			BufferedReader sock = new BufferedReader(new FileReader(new File(filename)));
			String i;
			while((i = sock.readLine()) != null) {
				String line = i.trim();
				StringTokenizer tokens = new StringTokenizer(line);
				String prefix = tokens.nextToken();
				if (prefix.equals("(:start")) {
					String s = tokens.nextToken();
					start = s.substring(0, s.length()-1);
				} else if (prefix.equals("(:end")) {
					String s = tokens.nextToken();
					end = s.substring(0, s.length()-1);
				} else if (prefix.equals("(:hn")) {
					inhn = true;
				} else if (prefix.equals("(:gn")) {
					ingn = true;
				} else if (prefix.substring(0, 1).equals("(")) {
					StringTokenizer t = new StringTokenizer(line.substring(1, line.length()-1));
					String key = t.nextToken();
					String value = t.nextToken();
					if (inhn) {
						hn.put(key, Integer.parseInt(value));
					} else if (ingn) {
						String n = t.nextToken();
						boolean in_keys = false;
						for (Iterator j = gn.keySet().iterator(); j.hasNext(); ) {
							String s = (String) j.next();
							if (s.equals(key)) {
								in_keys = true;
								break;
							}
						}
						if (!in_keys) {
							gn.put(key, new HashMap<String, Integer>());
						}
						gn.get(key).put(value, Integer.parseInt(n));
					}
				} else if (prefix.substring(0, 1).equals(")")) {
					if (inhn)
						inhn = false;
					else if (ingn)
						ingn = false;
				}
			}
		} catch(Exception e) {
			e.printStackTrace();
		}
		// Megkeresunk egy optimalis utat
		aStar(start, end);
	}
}
