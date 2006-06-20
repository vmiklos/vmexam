#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <linux/cdrom.h>

int main()
{
	int fd;

	if((fd = open("/dev/hdc", O_RDONLY|O_NONBLOCK))==-1)
		return(1);
	if((ioctl(fd, CDROMEJECT)) == -1)
		return(1);
	close(fd);
	return(0);
}

