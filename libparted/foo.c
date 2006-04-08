#include <parted/parted.h>

int main()
{
	PedDevice *dev = NULL;
	PedDisk   *disk = NULL;
	//char      *dev_name = NULL;
	//int	       nb_try = 0;

	ped_device_probe_all();

	if(ped_device_get_next(NULL)==NULL)
	{
		printf("no disk detected? :/\n");
		return(1);
	}

	for(dev=ped_device_get_next(NULL);dev!=NULL;dev=dev->next)
	{
		printf("%s, %s, %dGB\n", dev->path, dev->model, dev->length/1953125);
	}

	return(0);
}
