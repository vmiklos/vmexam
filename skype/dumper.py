import sys
import time
import gobject

import dbus
import dbus.service
import dbus.mainloop.glib

class Callback_obj(dbus.service.Object):
	def __init__(self, bus, object_path):
		dbus.service.Object.__init__(self, bus, object_path, bus_name='com.Skype.API')

	@dbus.service.method(dbus_interface='com.Skype.API')
	def Notify(self, message):
		if "STATUS RECEIVED" in message:
			id = message.split(' ')[1]
			print "New message has ID #%s" % id
			#print out_connection.Invoke('GET CHATMESSAGE %s BODY' % id)

dbus.mainloop.glib.DBusGMainLoop(set_as_default=True)
remote_bus = dbus.SessionBus()
out_connection = remote_bus.get_object('com.Skype.API', '/com/Skype')
out_connection.Invoke('NAME myapp')
out_connection.Invoke('PROTOCOL 5')
callback_obj = Callback_obj(remote_bus, '/com/Skype/Client')
loop = gobject.MainLoop()
loop.run()
