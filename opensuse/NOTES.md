# openSUSE notes

## osc

- Updating a package in OBS, examples:

```
osc build --alternative-project openSUSE:Leap:15.6 standard
osc build --alternative-project openSUSE:Factory
osc build --alternative-project openSUSE:Leap:42.3 --release 9999 standard (custom release)
osc build --alternative-project openSUSE:Leap:15.1:Update ports (ARM)
```

then:

```
osc vc
osc commit
```

- old meta XML snippets:

```
<repository name="openSUSE_Leap_15.3_ARM">
  <path project="openSUSE:Leap:15.3" repository="ports"/>
  <arch>aarch64</arch>
</repository>
<repository name="openSUSE_Factory_ARM">
  <path project="openSUSE:Factory:ARM" repository="standard"/>
  <arch>aarch64</arch>
</repository>
```

- Updating its git mirror:

cd $HOME/git/opensuse-packages
git checkout gtimelog
$HOME/git/bsgit/bsgit.py fetch home:vmiklos/gtimelog
git merge remotes/api.opensuse.org/home/vmiklos/gtimelog
git push origin

## zypper

- zypper tumbleweed bootstrap:

zypper --root /var/chroot/opensuse-tumbleweed addrepo http://download.opensuse.org/tumbleweed/repo/oss/ repo-oss
zypper --root /var/chroot/opensuse-tumbleweed install patterns-openSUSE-base

- zypper stable bootstrap:

zypper --root $PWD addrepo http://download.opensuse.org/distribution/leap/15.0/repo/oss/ repo-oss
zypper --root $PWD install patterns-openSUSE-base

- zyp-file: https://github.com/benthaman/zyp-file

- downgrade after broken update: zypper in --oldpackage --from repo-oss pkg1 [ pkg2 ... ]

- upgrade to some package from a custom repo:

```
zypper -p http://download.opensuse.org/repositories/KDE:/Extra/openSUSE_13.1/ in gtk2-engine-oxygen-1.4.4-11.1.x86_64 gtk2-theme-oxygen-1.4.4-11.1.x86_64
```

## cubox-i

- attach screen:

screen /dev/ttyUSB0 115200

- re-generate boot.scr (in /boot):

mkimage -C none -A arm -T script -d boot.script boot.scr

- raid1 root:

edit /etc/dracut.conf.d/01-dist.conf, comment out hostonly=

- special dracut problem because the kernel version ends with "-i", just run:

dracut --force

without any parameters, and it'll work.

## Upgrade notes

### openSUSE 13.1 -> 13.2

- [virt-manager is broken by default](https://bugzilla.suse.com/show_bug.cgi?id=901869)

TL;DR: `zypper in typelib-1_0-Gtk-3_0 typelib-1_0-SpiceClientGtk-3_0 typelib-1_0-GtkVnc-2_0`

- STL container pretty-printers do not work by default in gdb anymore (debuginfo uninstalled on upgrade?):

Can be fixed with: 'zypper in libstdc++6-debuginfo'

- [virt-resize is broken by default](https://bugzilla.novell.com/show_bug.cgi?id=908632)

TL;DR: `zypper -p http://download.opensuse.org/repositories/Virtualization/openSUSE_13.2/ in guestfs-data-1.26.9-163.1.x86_64`

### openSUSE 13.2 -> openSUSE Leap 42.1

- adjust repos:

  * 13.2 -> Leap-13.2 in names
  * 13.2 -> leap/42.1 in URLs

- [cpm fails to build](https://bugzilla.novell.com/show_bug.cgi?id=918553)

Fixed in <24 hours. :-)

- [cxoffice is broken](https://bugzilla.novell.com/show_bug.cgi?id=845916)

Fix: cd /opt/cxoffice/lib; mkdir t; mv libxcb* t

- KDE4 -> KDE5, retain a number of old settings:

  * update /etc/sysconfig/displaymanager: kdm -> sddm
  * cursor theme: Get New Theme, DMZ
  * desktop effects: change virtual desktop switching animation from slide to fade
  * panel height: 35px, clock date
  * digital clock settings -> tick 'show week numbers in Calendar'
  * k menu: alternatives -> launcher is the kde4 style thing
  * k menu favorites: systemsettings -> systemsettings5, etc
  * kbd layout chooser in systemsettings5
  * kbd layout fix: alt-1 and alt-7 works in bash once global keyboard shortcuts 'walk through windows of current application' (alt-~ and alt-`) are disabled
  * disable screen lock on screensaver
  * focus follows mouse: window management -> window behavior -> focus
  * kmix: individual process control seems to be no longer implemented, using 'pavucontrol' is a workaround

- sidebar is broken in mutt:

https://build.opensuse.org/request/show/350312
https://build.opensuse.org/request/show/353183

- STL container pretty-printers do not work by default in gdb anymore:

Can be fixed with: 'zypper in libstdc++6-devel-gcc5'

- (not really specific to this upgrade): SSD freezes the system from time to time:

libata.force=noncq kernel param seems to improve the situation. More info:

http://www.howtoeverything.net/linux/hardware/ubuntu-freeze-issue-after-ssd-upgrade

### openSUSE 42.1 -> openSUSE Leap 42.2

- adjust repos trivially (42.1 -> 42.2)
- mutt again prints "NBOX" for "INBOX"
  - minimal fix is to build server:mail/mutt r114 from OBS, that's new enough
    to be fixed, but old enough to be fully backwards-compatible; rpm is at
    <https://people.freedesktop.org/~vmiklos/2016/mutt-1.5.24-9999.x86_64.rpm>

### openSUSE 13.2 -> openSUSE Leap 42.2 for ARMV7

- images: http://download.opensuse.org/ports/armv7hl/distribution/leap/42.2/appliances/
  - openSUSE-Leap42.2-ARM-JeOS-cuboxi.armv7l-2016.11.25-Build1.8.raw.xz is what I used
- boot from raid1, still need to change:
  - /boot/boot.script, add RAID1 UUIDs + run mkimage
  - /etc/dracut.conf.d/01-dist.conf:
    - copy from /usr, comment out hostonly=
    - also 'add_drivers+="ci_hdrc_imx"' is needed for some reason to see the USB raid1 HDDs + run dracut

### openSUSE 42.2 -> openSUSE Leap 42.3

- mutt is replaced by neomutt, minor tweaks to config is necessary: https://github.com/vmiklos/dotfiles/commit/ac3dd6b0989527131c8bccd8602463aac3b05b33
  - gets rid of "unknown config key" error on startup + number of unread mails are shown again on the sidebar
- gcontacts is broken: need to install the python-oauth2client package
  - this used to be provided by the google-api-python-client package, which gets removed during upgrade
- colors in vim are strange: actually a konsole problem
  - settings -> edit current profile -> general -> environment -> edit -> TERM should be 'xterm', not 'xterm-256color'
- usual pain with firefox and html5 codecs: https://en.opensuse.org/SDB:Firefox_MP4/H.264_Video_Support
  - installing the mentioned packages from packman fixes the problem
- STL container pretty-printers do not work by default in gdb anymore:
  - can be fixed with: 'zypper in libstdc++6-devel-gcc7'

- KDE5 restart:

```
killall plasmashell #to stop it
kstart plasmashell #to restart it
```

### openSUSE Leap 42.2 -> openSUSE Leap 42.3 for ARMV7

- repo-update's URL is now http://download.opensuse.org/ports/update/42.3/oss (extra /oss at the end of the URL)
- migrate to new raid1:
  - old boot.script: rd.md.uuid=88a7927e:aa3d4f13:50015668:5fa5569a root=UUID=54a3f32d-941a-435f-968b-6fa43fa97951
  - new boot.script: rd.md.uuid=5f510f62:d1ed20dc:f7a02aac:46e749c5 root=UUID=abc2f9de-fc98-4a5e-9dd7-0176553d5977
  - need to find the values in /dev/disk/by-uuid/ (root) and /dev/disk/by-id/ (rd.md.uuid)
  - then update these in /boot/boot.script + run mkimage, reboot, then mdadm can stop the old raid1 and re-assign the now unused disk to the new one

### openSUSE Leap 42.3 -> openSUSE Leap 15.0 (x86-64)

- the usual steps from <https://en.opensuse.org/SDB:System_upgrade> (sed the config, zypper ref, zypper dup)
- konsole: font antialiasing seems to be rather aggressive now, settings -> edit current profile -> appearance -> smooth fonts -> disable is a workaround
- usual fix with html5 video codecs in firefox/chromium: https://en.opensuse.org/SDB:Firefox_MP4/H.264_Video_Support
  - if it still doesn't work, try to remove libavcodecN packages; the ones that are actually neeeded should be from packman
- muttprint is no longer in server:mail for 15.0 -> patch sent as https://build.opensuse.org/request/show/615320 -> merged
- `zypper -p https://download.opensuse.org/repositories/devel:/libraries:/c_c++/openSUSE_Leap_15.0/ in poco-devel` installs a cxx11 poco, fixing link errors
- `zypper in python2-gdata` is uninstalled by the upgrade, `python3-google-api-python-client` is the replacement package
- pdftk is no longer in the stock repos, but packman provides it
- `convert` to PDF is blacklisted by default, need to remove the blacklist from `/etc/ImageMagick-7_Q16HDRI6/policy.xml` manually
- also swithced to tmpfs for /tmp, fstab line is `tmpfs /tmp tmpfs defaults,noatime,mode=1777 0 0`

### openSUSE Leap 42.3 -> openSUSE Leap 15.0 (armv7l)

- repo-update url changed to <http://download.opensuse.org/ports/update/leap/15.0/oss>
- ifconfig and route is no more, see <https://dougvitale.wordpress.com/2011/12/21/deprecated-linux-networking-commands-and-their-replacements/> for their replacements
- rss2email broke: `.config/rss2email.cfg` has to be updated: `smtp-ssl-protocol` needs to be `TLSv1` (while `SSLv3` was fine previously)

### openSUSE Leap 15.0 -> openSUSE Leap 15.1 (x86-64)

- usual fix with html5 video codecs in firefox/chromium: https://en.opensuse.org/SDB:Firefox_MP4/H.264_Video_Support
- STL container pretty-printers do not work by default in gdb anymore:
  - can be fixed with: 'zypper in libstdc++6-devel-gcc8'

### openSUSE Leap 15.1 -> openSUSE Leap 15.2 (x86-64)

- the usual steps from <https://en.opensuse.org/SDB:System_upgrade> (sed the config, zypper ref, zypper dup)
- the usual fix with html5 video codecs in firefox/chromium: https://en.opensuse.org/SDB:Firefox_MP4/H.264_Video_Support
- my LANG=hu_HU.UTF-8 + LC_MESSAGES=C setup broke, had to restore the lost LANG part in KDE
  - konsole now catches alt-<number>, need to disable that in settings -> configure keyboard shortcuts, so it rearches e.g. irssi
  - ctrl-shift-left/right also broke (no longer moves the current tab left/right), can be fixed at the same place
- https://www.phoronix.com/scan.php?page=news_item&px=KDE-Plasma-5.18-Emojis windows + '.' now allows inserting turtles...

### openSUSE Leap 15.1 -> openSUSE Leap 15.2 (aarch64)

- the usual steps from <https://en.opensuse.org/SDB:System_upgrade> (zypper --releasever=15.2 ref, zypper --releasever=15.2 dup)
- samba: unix extensions are now disabled, it can be enabled with `server min protocol = LANMAN1` in `/etc/samba/smb.conf`, see <https://www.samba.org/samba/history/samba-4.11.0.html>
- nextcloud, restore clean URLs:
  - cd /srv/www/htdocs/nextcloud
  - chown wwwrun .htaccess
  - sudo -u wwwrun php ./occ maintenance:update:htaccess

### openSUSE Leap 15.2 -> openSUSE Leap 15.3 (x86_64)

- the usual steps from <https://en.opensuse.org/SDB:System_upgrade> (zypper --releasever=15.3 ref, zypper --releasever=15.3 dup)
- the usual fix with html5 video codecs in firefox/chromium: https://en.opensuse.org/SDB:Firefox_MP4/H.264_Video_Support
- now 2 additional update repos: https://doc.opensuse.org/release-notes/x86_64/openSUSE/Leap/15.3/#installation-new-update-repos
  - repo-backports-update from <http://download.opensuse.org/update/leap/$releasever/backports/>
  - repo-sle-update from <http://download.opensuse.org/update/leap/$releasever/sle/>
- if /etc/os-release starts to report SLES, install openSUSE-release back (wtf?)
- https://github.com/ycm-core/YouCompleteMe needs newer a vim version: zypper -p https://download.opensuse.org/repositories/editors/openSUSE_Leap_15.3/ in vim
- https://www.microsoft.com/en-ww/microsoft-365 needs newer mutt: zypper -p https://download.opensuse.org/repositories/server:/mail/openSUSE_Leap_15.3/ in mutt
- the intel X11 driver is buggy, e.g. typing in konsole sometimes misses a redraw, `modesetting_drv` is fine: `zypper remove xf86-video-intel`
- STL container pretty-printers do not work by default in gdb anymore:
  - can be fixed with: 'zypper in libstdc++6-devel-gcc11 libstdc++6-pp-gcc11'

### openSUSE Leap 15.2 -> openSUSE Leap 15.3 (aarch64)

- the usual steps from <https://en.opensuse.org/SDB:System_upgrade> (zypper --releasever=15.3 ref, zypper --releasever=15.3 dup)
- added the 2 new repos

### openSUSE Leap 15.3 -> openSUSE Leap 15.4 (x86-64)

- the usual steps from <https://en.opensuse.org/SDB:System_upgrade> (zypper --releasever=15.4 ref, zypper --releasever=15.4 dup)
  - `zypper -p https://download.opensuse.org/repositories/server:/mail/15.4/ in mutt`
  - `zypper -p https://download.opensuse.org/repositories/X11:/common:/Factory/openSUSE_Leap_15.3/ in libicu-devel`
  - `zypper -p https://download.opensuse.org/repositories/devel:/languages:/python/15.4/ in git-review`
  - `zypper -p https://download.opensuse.org/repositories/editors/openSUSE_Leap_15.4/ in vim`
    - python is now compiled with python3 support (only), need to replace "py" with "py3" in vim config
    - this also causes <https://github.com/actionshrimp/vim-xpath/issues/16>, will have to look for a replacement
- the usual fix with html5 video codecs in firefox/chromium: <https://en.opensuse.org/SDB:Firefox_MP4/H.264_Video_Support>
- konsole:
  - hide the new, not needed toolbar in konsole: <https://forum.kde.org/viewtopic.php?f=227&t=170988>
  - konsole now (again) catches alt-<number>, need to disable that in settings -> configure keyboard shortcuts, so it rearches e.g. irssi
  - and <https://forum.kde.org/viewtopic.php?f=227&t=167471> explains how to get rid of the new left blue line
- kde in general: <https://www.reddit.com/r/openSUSE/comments/pmdcno/display_problems_after_tumbleweed_update/> seems to be the solution for some rendering issues

### openSUSE Leap 15.3 -> openSUSE Leap 15.4 (aarch64)

- the usual steps from <https://en.opensuse.org/SDB:System_upgrade> (zypper --releasever=15.4 ref, zypper --releasever=15.4 dup)

### openSUSE Leap 15.4 -> openSUSE Leap 15.5 (x86-64)

- the usual steps from <https://en.opensuse.org/SDB:System_upgrade> (zypper --releasever=15.5 ref, zypper --releasever=15.5 dup)
  - `zypper -p https://download.opensuse.org/repositories/server:/mail/15.5/ in mutt`
  - `zypper -p https://download.opensuse.org/repositories/X11:/common:/Factory/15.5/ in libicu-devel` (skipped on t14, seems OK)
  - `zypper -p https://download.opensuse.org/repositories/devel:/languages:/python/15.5/ in git-review` (skipped on vostro, seems OK)
  - `zypper -p https://download.opensuse.org/repositories/editors/15.5/ in vim` (skipped on vostro, seems OK)
- the usual fix with html5 video codecs in firefox/chromium: <https://en.opensuse.org/SDB:Firefox_MP4/H.264_Video_Support>

### openSUSE Leap 15.4 -> openSUSE Leap 15.5 (aarch64)

- the usual steps from <https://en.opensuse.org/SDB:System_upgrade> (zypper --releasever=15.5 ref, zypper --releasever=15.5 dup)

### openSUSE Leap 15.5 -> openSUSE Leap 15.6 (x86-64)

- the usual steps from <https://en.opensuse.org/SDB:System_upgrade> (zypper --releasever=15.6 ref, zypper --releasever=15.6 dup)
- the usual fix with html5 video codecs in firefox/chromium: <https://en.opensuse.org/SDB:Firefox_MP4/H.264_Video_Support>
