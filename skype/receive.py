"""usage: send.py target 'hello world'"""

import dbus, sys

skypeout = dbus.SessionBus().get_object('com.Skype.API', '/com/Skype')
skypeout.Invoke('NAME myapp')
skypeout.Invoke('PROTOCOL 5')
print skypeout.Invoke('GET CHATMESSAGE %s BODY' % sys.argv[1])
