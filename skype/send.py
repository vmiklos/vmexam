"""usage: send.py target 'hello world'"""

import dbus, sys

skypeout = dbus.SessionBus().get_object('com.Skype.API', '/com/Skype')
skypeout.Invoke('NAME myapp')
skypeout.Invoke('PROTOCOL 5')
chat = skypeout.Invoke('CHAT CREATE %s' % sys.argv[1]).split(' ')[1]
skypeout.Invoke('CHATMESSAGE %s %s' % (chat, sys.argv[2]))
