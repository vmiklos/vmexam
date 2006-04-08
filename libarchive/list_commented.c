#include <stdio.h>
#include <archive.h>
#include <archive_entry.h>


int main(int argc, char * argv[])
{
	register struct archive *archive;
	struct archive_entry *entry;

	if(argc != 2)
	{
		printf("usage: %s archive_file\n", argv[0]);
		return(1);
	}

	// initialize libarchive
	if ((archive = archive_read_new ()) == NULL)
		return(1);
	// use all compression type
	archive_read_support_compression_all (archive);
	// detect all format
	archive_read_support_format_all (archive);
	
	// open the archive
	if (archive_read_open_file (archive, argv[1], 10240) != ARCHIVE_OK)
	{
		perror("could not open package");
		return(1);
	}

	// read out the files
	while (archive_read_next_header (archive, &entry) == ARCHIVE_OK)
		printf("%s\n", archive_entry_pathname (entry));

	// close the archive
	archive_read_finish (archive);
}
