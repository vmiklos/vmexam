import os, sys, getpass, glob, datetime
import gdata.photos.service

#what filename accept
def isJPG(filename):
	return filename.lower()[-3:] == "jpg"

#environment
dir = os.path.basename(os.getcwd())
print "local album =", dir

#login
id = raw_input("Username:") + "@gmail.com"
ps = getpass.getpass()
print "Login in..."
gd_client = gdata.photos.service.PhotosService()
gd_client.ClientLogin(id, ps)

#which album
albumEntry = None
feed = gd_client.GetUserFeed(user = id)
for entry in feed.entry:
	if entry.title.text == dir:
		albumEntry = entry
		print "use album", albumEntry.title.text
		break
if albumEntry == None:
	albumEntry = gd_client.InsertAlbum(title=dir, summary='')
	print "create album", albumEntry.title.text
albumUrl = albumEntry.GetFeedLink().href

#which photos to upload
photoTitleListOnline = []
for photoEntry in gd_client.GetFeed(albumUrl).entry:
	photoTitleListOnline.append(photoEntry.title.text)

for filename in glob.glob('*'):
	if isJPG(filename) and not filename in photoTitleListOnline:
		print datetime.datetime.now().strftime("%H:%M:%S"), filename, "uploading"
		entry = gd_client.InsertPhotoSimple(albumUrl, filename, '', filename, content_type='image/jpeg')
		print datetime.datetime.now().strftime("%H:%M:%S"), "Done"
