#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <stdlib.h>
#include <libudev.h>

int check_devpath(const char *path, char *iface)
{
	int ret = 0;
	char *devpath = strdup(path);
	char *ptr = strrchr(devpath, '/');

	if (ptr && strcmp(iface, ++ptr) == 0)
		ret = 1;
	free(devpath);
	return ret;
}

int wait_interface(char *iface)
{
	struct udev *udev = NULL;
	struct udev_monitor *udev_monitor = NULL;
	struct udev_enumerate *udev_enumerate = NULL;
	struct udev_list_entry *item = NULL, *first = NULL;
	int rc = -1;

	udev = udev_new();
	if (!udev) {
		fprintf(stderr, "unable to create udev context");
		goto finish;
	}

	/* subscribe to udev events */
	udev_monitor = udev_monitor_new_from_netlink(udev, "udev");
	if (!udev_monitor) {
		fprintf(stderr, "unable to create netlink socket\n");
		goto finish;
	}
	udev_monitor_set_receive_buffer_size(udev_monitor, 128*1024*1024);
	if (udev_monitor_filter_add_match_subsystem_devtype(udev_monitor, "net", NULL) < 0) {
		fprintf(stderr, "unable to add matching subsystem to monitor\n");
		goto finish;
	}
	if (udev_monitor_enable_receiving(udev_monitor) < 0) {
		fprintf(stderr, "unable to subscribe to udev events\n");
		goto finish;
	}

	/* then enumerate over existing ones */
	udev_enumerate = udev_enumerate_new(udev);
	if (!udev_enumerate) {
		fprintf(stderr, "unable to create an an enumeration context\n");
		goto finish;
	}
	if (udev_enumerate_add_match_subsystem(udev_enumerate, "net") < 0) {
		fprintf(stderr, "unable to add mathing subsystem to enumerate\n");
		goto finish;
	}
	if (udev_enumerate_scan_devices(udev_enumerate) < 0) {
		fprintf(stderr, "unable to scan devices\n");
		goto finish;
	}
	first = udev_enumerate_get_list_entry(udev_enumerate);
	udev_list_entry_foreach(item, first) {
		if (check_devpath(udev_list_entry_get_name(item), iface)) {
			/* the interface is already up */
			rc = 0;
			goto finish;
		}
	}

	while (1) {
		struct udev_device *device;

		device = udev_monitor_receive_device(udev_monitor);
		if (device == NULL || strcmp("add", udev_device_get_action(device)) != 0)
			continue;
		int found = 0;
		if (check_devpath(udev_device_get_devpath(device), iface))
			/* the interface is just added */
			found = 1;
		udev_device_unref(device);
		if (found)
			break;
	}

	rc = 0;
finish:
	if (udev_enumerate)
		udev_enumerate_unref(udev_enumerate);
	if (udev_monitor)
		udev_monitor_unref(udev_monitor);
	if (udev)
		udev_unref(udev);
	return rc;
}

int main(int argc, char **argv)
{
	char *iface;
	if (argc < 2) {
		fprintf(stderr, "usage: %s <interface>\n", argv[0]);
		return -1;
	}
	iface = argv[1];
	wait_interface(iface);
	return 0;
}
