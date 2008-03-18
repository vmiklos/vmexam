/*
 * Determines the kernel driver for an interface
 */

#include <sys/types.h>
#include <sys/socket.h>
#include <sys/ioctl.h>
#include <net/if.h>
#include <string.h>
#include <stdio.h>
#include <errno.h>
#include <linux/sockios.h>

/* hack, so we may include kernel's ethtool.h */
typedef unsigned long long __u64;
typedef __uint32_t __u32;         /* ditto */
typedef __uint16_t __u16;         /* ditto */
typedef __uint8_t __u8;           /* ditto */

#include <linux/ethtool.h>

int main()
{
	struct ifreq ifr;
	int fd, err;
	struct ethtool_drvinfo drvinfo;

	memset(&ifr, 0, sizeof(ifr));
	strcpy(ifr.ifr_name, "eth0");

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
	printf("%s\n", drvinfo.driver);
}
