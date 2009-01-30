#!/usr/bin/env python

import sys, codecs

class Node:
	def __init__(self, name, g, h):
		self.name = name
		self.f = g + h
		self.g = g
		self.h = h

def sort_node(a, b):
	return cmp(a.f, b.f)

def name_in_list(y, l):
	for i in l:
		if y == i.name:
			return True
	return False

def node_from_list(y, l):
	for i in l:
		if y == i.name:
			return i

def reconstruct_path(came_from,current_node):
	if current_node in came_from.keys():
		p = reconstruct_path(came_from,came_from[current_node])
		return p + [current_node]
	else:
		return [current_node]

def a_star(start, end):
	sock = codecs.open("output.txt", "w", "ISO-8859-2")
	openlist = [Node(start, 0, hn[start])]
	closedlist = []
	count = 0
	while len(openlist):
		openlist.sort(cmp=sort_node)
		sock.write("(:openlist %s" % count)
		for i in openlist:
			sock.write(" (%s %s)" % (i.f, i.name))
		sock.write(")\n")
		sock.write("(:closedlist %s" % count)
		for i in closedlist:
			sock.write(" (%s %s)" % (i.f, i.name))
		sock.write(")\n")

		x = openlist.pop(0)
		if x.name == end:
			sock.write("(:sol %s " % x.f)
			sock.write(" ".join(reconstruct_path(came_from,end)))
			sock.write(")\n")
			return True
		closedlist.append(x)
		for y in gn[x.name].keys():
			if name_in_list(y, closedlist):
				continue
			tentative_g_score = x.g + gn[x.name][y]
			tentative_is_better = False
			if not name_in_list(y, openlist):
				openlist.append(Node(y, tentative_g_score, hn[y]))
				tentative_is_better = True
			elif tentative_g_score < node_from_list(y, openlist).g:
				tentative_is_better = True
			if tentative_is_better == True:
				came_from[y] = x.name
		count += 1
	return False

inhn = False
ingn = False
start = None
end = None
hn = {}
gn = {}
came_from = {}
sock = codecs.open(sys.argv[1], "r", "ISO-8859-2")
for i in sock.readlines():
	line = i.strip()
	if line.startswith("(:start"):
		start = line[8:-1]
	elif line.startswith("(:end"):
		end = line[6:-1]
	elif line.startswith("(:hn"):
		inhn = True
	elif line.startswith("(:gn"):
		ingn = True
	elif line.startswith("("):
		if inhn:
			items = line[1:-1].split(' ')
			hn[items[0]] = int(items[1])
		elif ingn:
			items = line[1:-1].split(' ')
			if items[0] not in gn.keys():
				gn[items[0]] = {}
			gn[items[0]][items[1]] = int(items[2])
	elif line.startswith(")"):
		if inhn:
			inhn = False
		elif ingn:
			ingn = False
sock.close()
a_star(start, end)
