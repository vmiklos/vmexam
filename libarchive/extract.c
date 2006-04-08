#include <stdio.h>
#include <limits.h>
#include <archive.h>
#include <archive_entry.h>
#include <errno.h>

#define ARCHIVE_EXTRACT_FLAGS   ARCHIVE_EXTRACT_OWNER | ARCHIVE_EXTRACT_PERM | ARCHIVE_EXTRACT_TIME | ARCHIVE_EXTRACT_ACL | ARCHIVE_EXTRACT_FFLAGS

int main(int argc, char * argv[])
{
	register struct archive *archive;
	struct archive_entry *entry;
	char expath[PATH_MAX];
	char cwd[PATH_MAX];

	if(argc != 2)
	{
		printf("usage: %s archive_file\n", argv[0]);
		return(1);
	}

	getcwd(cwd, PATH_MAX);
	if ((archive = archive_read_new ()) == NULL)
		return(1);
	archive_read_support_compression_all (archive);
	archive_read_support_format_all (archive);
	
	if (archive_read_open_file (archive, argv[1], 10240) != ARCHIVE_OK)
	{
		perror("could not open package");
		return(1);
	}
	while (archive_read_next_header (archive, &entry) == ARCHIVE_OK)
	{
		printf("%s\n", archive_entry_pathname (entry));
		snprintf(expath, PATH_MAX, "%s/%s", cwd,
			archive_entry_pathname(entry));
		archive_entry_set_pathname (entry, expath);
		if (archive_read_extract (archive, entry,
			ARCHIVE_EXTRACT_FLAGS) != ARCHIVE_OK)
		{
			printf ("%d\n",ARCHIVE_OK);
			fprintf(stderr, "could not extract %s: %s", expath,
				strerror(errno));
		}
	}
	archive_read_finish (archive);
}
