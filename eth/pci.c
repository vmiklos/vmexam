#include <sys/types.h>
#include <sys/socket.h>
#include <sys/ioctl.h>
#include <net/if.h>
#include <string.h>
#include <stdio.h>
#include <errno.h>
#include <limits.h>
#include <fcntl.h>
#include <pci.h>
#include <linux/sockios.h>

/* hack, so we may include kernel's ethtool.h */
typedef unsigned long long __u64;
typedef __uint32_t __u32;         /* ditto */
typedef __uint16_t __u16;         /* ditto */
typedef __uint8_t __u8;           /* ditto */

#include <linux/ethtool.h>

int iface_desc(const char *iface, char *desc, int size)
{
	struct ifreq ifr;
	int fd, err, len, device, vendor;
	struct ethtool_drvinfo drvinfo;
	char buf[512], path[PATH_MAX];
	struct pci_access *pacc;

	memset(&ifr, 0, sizeof(ifr));
	strcpy(ifr.ifr_name, iface);

	fd = socket(AF_INET, SOCK_DGRAM, 0);
	if (fd < 0)
	{
		perror("Cannot get control socket");
		return 1;
	}
	drvinfo.cmd = ETHTOOL_GDRVINFO;
	ifr.ifr_data = (caddr_t)&drvinfo;
	err = ioctl(fd, SIOCETHTOOL, &ifr);
	if (err < 0)
	{
		perror("Cannot get driver information");
		printf("%d\n", errno);
		return 2;
	}
	close(fd);
	snprintf(path, PATH_MAX-1, "/sys/bus/pci/devices/%s/vendor", drvinfo.bus_info);
	fd = open(path, O_RDONLY);
	if (fd < 0)
	{
		perror("Cannot open the vendor file");
		return 3;
	}
	len = read(fd, buf, sizeof(buf));
	buf[len-1] = '\0';
	close(fd);
	sscanf(buf,"%X", &vendor);
	snprintf(path, PATH_MAX, "/sys/bus/pci/devices/%s/device", drvinfo.bus_info);
	fd = open(path, O_RDONLY);
	if (fd < 0)
	{
		perror("Cannot open the device file");
		return 3;
	}
	len = read(fd, buf, sizeof(buf));
	buf[len-1] = '\0';
	close(fd);
	sscanf(buf,"%X", &device);
	pacc = pci_alloc();
	pci_init(pacc);
	pci_lookup_name(pacc, desc, size,
			PCI_LOOKUP_VENDOR | PCI_LOOKUP_DEVICE,
			vendor, device);
	pci_cleanup(pacc);
	return(0);
}

int main(int argc, char **argv)
{
	char desc[128];

	if (argc <2)
	{
		printf("usage: %s <iface_name>\n", argv[0]);
		return 1;
	}

	iface_desc(argv[1], desc, sizeof(desc));
	printf("%s\n", desc);
}
