#include <parted/parted.h>
#define MDSTATF "mdstat"

int listparts(PedDisk *disk, char *path)
{
	PedPartition *part = NULL;
	PedPartition *extpart = NULL;

	if(ped_disk_next_partition(disk, NULL)==NULL)
		// no partition detected
		return(1);
	for(part=ped_disk_next_partition(disk, NULL);
		part!=NULL;part=part->next)
	{
		if((part->num>0) && (part->type != PED_PARTITION_EXTENDED))
			printf("%s%d\n", path, part->num);
		if(part->type == PED_PARTITION_EXTENDED)
			for(extpart=part->part_list;
				extpart!=NULL;extpart=extpart->next)
				if(extpart->num>0)
					printf("%s%d\n", path, extpart->num);
	}
	return(0);
}

int main()
{
	PedDevice *dev = NULL;
	PedDisk *disk = NULL;

	ped_device_probe_all();

	if(ped_device_get_next(NULL)==NULL)
		// no disk detected already handled before, no need to inform
		// the user about this
		return(1);

	for(dev=ped_device_get_next(NULL);dev!=NULL;dev=dev->next)
	{
		if(dev->read_only)
			// we don't want to partition cds ;-)
			continue;
		disk = ped_disk_new(dev);
		listparts(disk, dev->path);
	}

	return(0);
}
