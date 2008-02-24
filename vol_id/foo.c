#include <libvolume_id.h>
#include <sys/ioctl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <stdio.h>

#define BLKGETSIZE64 _IOR(0x12,114,size_t)

int main(int argc, char **argv)
{
	int fd;
	struct volume_id *vid = NULL;
	uint64_t size;
	const char *label;

	fd = open(argv[1], O_RDONLY);
	if(fd<0)
		return 1;
	vid = volume_id_open_fd(fd);
	ioctl(fd, BLKGETSIZE64, &size);
	volume_id_probe_all(vid, 0, size);
	volume_id_get_label(vid, &label);
	printf("%s\n", label);
	volume_id_close(vid);
}
