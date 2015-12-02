#!/usr/bin/python

import glib, gtk

def test_clipboard():
    clipboard = gtk.Clipboard()
    targets = clipboard.wait_for_targets()
    print "Targets available:", ", ".join(map(str, targets))
    for target in targets:
        print "Trying '%s'..." % str(target)
        contents = clipboard.wait_for_contents(target)
        if contents:
            print contents.data

def main():
    mainloop = glib.MainLoop()
    def cb():
        test_clipboard()
        mainloop.quit()
    glib.idle_add(cb)
    mainloop.run()

if __name__ == "__main__":
    main()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
