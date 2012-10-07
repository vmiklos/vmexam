from sgmllib import SGMLParser
import urllib
import re

def to_h_m(text):
	n = int(text)
	h = n / 60
	m = n % 60
	return "%d:%02d" % (h, m)

def get_event(url, track):
	class HTMLParser(SGMLParser):
		def reset(self):
			SGMLParser.reset(self)
			self.seenLabel = False
			self.seenTimeLabel = False
			self.inTimeSpan = False
			self.seenLengthLabel = False
			self.inLengthSpan = False
			self.inTitle = False
			self.seenPresenterLabel = False
			self.inPresenterSpan = False
			self.persons = []
			self.inDescription = False

		def start_label(self, attrs):
			self.seenLabel = True

		def start_span(self, attrs):
			if self.seenTimeLabel:
				self.inTimeSpan = True
				self.seenTimeLabel = False
			elif self.seenLengthLabel:
				self.inLengthSpan = True
				self.seenLengthLabel = False
			elif self.seenPresenterLabel:
				self.inPresenterSpan = True
				self.seenPresenterLabel = False

		def start_h1(self, attrs):
			for k, v in attrs:
				if k == "class" and v == "documentFirstHeading":
					self.inTitle = True

		def start_p(self, attrs):
			for k, v in attrs:
				if k == "class" and v == "documentDescription":
					self.inDescription = True
		def handle_data(self, text):
			if self.seenLabel:
				if text == "Time:":
					self.seenTimeLabel = True
				elif text == "Length of the Talk:":
					self.seenLengthLabel = True
				elif text == "Presenter:" or text == "Co-Presenter:":
					self.seenPresenterLabel = True
				self.seenLabel = False
			elif self.inTimeSpan:
				self.start = re.sub(r".*from (.*) to", r"\1", text).strip()
				self.inTimeSpan = False
			elif self.inLengthSpan:
				self.length = to_h_m(text)
				self.inLengthSpan = False
			elif self.inTitle:
				self.title = text
				self.inTitle = False
			elif self.inPresenterSpan:
				self.persons.append(text)
				self.inPresenterSpan = False
			elif self.inDescription:
				self.abstract = text
				self.inDescription = False
	
	sock = urllib.urlopen(url)
	page = sock.read()
	sock.close()

	parser = HTMLParser()
	parser.reset()
	parser.feed(page)
	parser.close()
	print '<event id="%s">' % url.split('/')[-1]
	print '<start>%s</start>' % parser.start
	print '<duration>%s</duration>' % parser.length
	print '<title>%s</title>' % parser.title
	print '<track>%s</track>' % track
	print '<persons>'
	for i in parser.persons:
		print '<person>%s</person>' % i
	print '</persons>'
	print "<abstract>%s</abstract>" % parser.abstract
	print '</event>'

def get_track(url, room):
	class HTMLParser(SGMLParser):
		def reset(self):
			SGMLParser.reset(self)
			self.seenH4 = False
			self.talks = []
			self.inSpan = False

		def start_h4(self, attrs):
			self.seenH4 = True

		def start_a(self, attrs):
			if self.seenH4:
				for k, v in attrs:
					if k == "href" and not (v.endswith("-break") or v == "lunch"):
						self.talks.append(v)
				self.seenH4 = False

		def start_span(self, attrs):
			for k, v in attrs:
				if k == "id" and v == "breadcrumbs-current":
					self.inSpan = True

		def handle_data(self, text):
			if self.inSpan:
				self.track = text
				self.inSpan = False
	
	sock = urllib.urlopen(url)
	page = sock.read()
	sock.close()

	parser = HTMLParser()
	parser.reset()
	parser.feed(page)
	parser.close()
	print '<room name="%s">' % room
	for i in parser.talks:
		get_event("%s/%s" % (url, i), parser.track)
	print '</room>'

def get_schedule(url):
	class HTMLParser(SGMLParser):
		def checkTrackRoom(self):
			# if there is a track and a room already, record it
			if self.track and self.room:
				self.tracks.append((self.track, self.room, self.fro))
				self.track = None
				self.room = None
				self.fro = None

		def reset(self):
			SGMLParser.reset(self)
			self.seenTbody = False
			self.seenH4 = False
			self.track = None
			self.room = None
			self.tracks = []
			self.spanLevel = 0
			self.inFrom = False
			self.fro = None

		def start_tbody(self, attrs):
			self.seenTbody = True

		def start_h4(self, attrs):
			if self.seenTbody:
				self.checkTrackRoom()
				self.seenH4 = True

		def start_span(self, attrs):
			if self.seenH4:
				self.spanLevel += 1
			elif self.room:
				self.inFrom = True

		def end_span(self):
			if self.seenH4:
				self.spanLevel -= 1
				if self.spanLevel == 0:
					self.seenH4 = False

		def start_a(self, attrs):
			if self.seenH4:
				for k, v in attrs:
					if k == "href":
						self.track = v

		def start_div(self, attrs):
			for k, v in attrs:
				if k == "class" and v == "sponsorships":
					self.checkTrackRoom()

		def handle_data(self, text):
			if self.spanLevel == 2:
				self.room = text
			elif self.inFrom:
				self.fro = text.split('.')[0]
				self.inFrom = False
	
	sock = urllib.urlopen(url)
	page = sock.read()
	sock.close()

	parser = HTMLParser()
	parser.reset()
	parser.feed(page)
	parser.close()
	days = sorted(set(map(lambda x: x[2], parser.tracks)))

	print '<?xml version="1.0"?>'
	print '<schedule>'
	print '<conference>'
	print '<title>LibreOffice Conference 2012</title>'
	print '<city>Berlin</city>'
	print '<start>2012-10-%s</start>' % days[0]
	print '<end>2012-10-%s</end>' % days[-1]
	print '<days>%s</days>' % len(days)
	print '<release>1.0</release>'
	print '<timeslot_duration>00:15</timeslot_duration>'
	print '</conference>'

	daycount = 1
	for i in days:
		print '<day date="2012-10-%s" index="%s">' % (i, daycount)
		daycount += 1
		for j in filter(lambda x: x[2] == i, parser.tracks):
			get_track("%s/%s" % (url, j[0]), j[1])
		print '</day>'

	print '</schedule>'

get_schedule("http://conference.libreoffice.org/program")
