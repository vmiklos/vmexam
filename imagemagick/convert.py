import os, shutil, sys

who = sys.argv[1]

def system(cmd):
	print "-> " + cmd
	os.system(cmd)

bigs = []
smalls = []

os.chdir(who)

for root, dirs, files in os.walk("8as"):
	for i in files:
		bigs.append(os.path.join("8as", i))

for root, dirs, files in os.walk("4es"):
	for i in files:
		smalls.append(os.path.join("4es", i))

os.mkdir("scaled")

print "converting big images to std size"
for i in bigs:
	system("convert %s -scale 2200x1100! %s" % (i, os.path.join("scaled", os.path.basename(i))))

print "rotating big images :2"
for i in bigs[:2]:
	system("convert %s -rotate 90 %s" % (os.path.join("scaled", os.path.basename(i)), os.path.join("scaled", os.path.basename(i))))

print "splitting big images :2"
for i in bigs[:2]:
	system("convert %s -crop 1100x1100+0+0 %s.t.jpg" % (os.path.join("scaled", os.path.basename(i)), os.path.join("scaled", os.path.basename(i))))
	system("convert %s -crop 1100x1100+0+1100 %s.b.jpg" % (os.path.join("scaled", os.path.basename(i)), os.path.join("scaled", os.path.basename(i))))

print "splitting big images 2:"
for i in bigs[2:]:
	system("convert %s -crop 1100x1100+0+0 %s.t.jpg" % (os.path.join("scaled", os.path.basename(i)), os.path.join("scaled", os.path.basename(i))))
	system("convert %s -crop 1100x1100+1100+0 %s.b.jpg" % (os.path.join("scaled", os.path.basename(i)), os.path.join("scaled", os.path.basename(i))))

print "converting small images to std size"
for i in smalls:
	system("convert %s -scale 1100x1100! %s" % (i, os.path.join("scaled", os.path.basename(i))))

images = []
images.append(os.path.join("scaled", os.path.basename(smalls[0])))
images.append(os.path.join("scaled", os.path.basename(smalls[1])))
images.append(os.path.join("scaled", os.path.basename("%s.t.jpg" % bigs[0])))
images.append(os.path.join("scaled", os.path.basename(smalls[2])))
images.append(os.path.join("scaled", os.path.basename(smalls[3])))
images.append(os.path.join("scaled", os.path.basename("%s.b.jpg" % bigs[0])))
images.append(os.path.join("scaled", os.path.basename(smalls[4])))
images.append(os.path.join("scaled", os.path.basename(smalls[5])))
images.append(os.path.join("scaled", os.path.basename("%s.t.jpg" % bigs[1])))
images.append(os.path.join("scaled", os.path.basename("%s.t.jpg" % bigs[2])))
images.append(os.path.join("scaled", os.path.basename("%s.b.jpg" % bigs[2])))
images.append(os.path.join("scaled", os.path.basename("%s.b.jpg" % bigs[1])))
print "montaging"
system("montage %s -density 500 -tile 3x4 -geometry +0+0 ../%s.jpg" % (" ".join(images), who))
shutil.rmtree("scaled")
