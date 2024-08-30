#define _GNU_SOURCE
#include <sched.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mount.h>
#include <unistd.h>

int write_to_file(const char* path, const char* buf)
{
    int fd = open(path, O_WRONLY);
    if (fd < 0)
    {
        fprintf(stderr, "error, open() failed\n");
        return -1;
    }

    if (write(fd, buf, strlen(buf)) < 0)
    {
        fprintf(stderr, "error, write() failed\n");
        return -1;
    }

    close(fd);
    return 0;
}

int mount_rbind(const char* from, const char* to)
{
    return mount(from, to, NULL, MS_BIND | MS_REC, NULL);
}

int main()
{
    // Create a new user namespace.
    int unshare_flags = CLONE_NEWUSER;
    // Create a new mount namespace.
    unshare_flags |= CLONE_NEWNS;

    // It's important to save these before we call unshare().
    uid_t euid = geteuid();
    gid_t egid = getegid();

    // Create the namespaces.
    if (unshare(unshare_flags) == -1)
    {
        fprintf(stderr, "error, unshare() failed\n");
        return 1;
    }

    // Map the current effective user and group IDs to root in the user
    // namespace.
    char* buf = NULL;
    asprintf(&buf, "0 %u 1", euid);
    if (write_to_file("/proc/self/uid_map", buf) < 0)
    {
        fprintf(stderr, "failed to write /proc/self/uid_map\n");
        return 1;
    }
    free(buf);
    if (write_to_file("/proc/self/setgroups", "deny") < 0)
    {
        fprintf(stderr, "failed to write /proc/self/setgroups\n");
        return 1;
    }
    asprintf(&buf, "0 %u 1", egid);
    if (write_to_file("/proc/self/gid_map", buf) < 0)
    {
        fprintf(stderr, "failed to write /proc/self/gid_map\n");
        return 1;
    }
    free(buf);

    // Create bind mounts.
    if (mount_rbind("/dev", "dev") < 0)
    {
        fprintf(stderr, "failed to rbind mount /dev\n");
        return 1;
    }
    if (mount_rbind("/proc", "proc") < 0)
    {
        fprintf(stderr, "failed to rbind mount /proc\n");
        return 1;
    }
    if (mount_rbind("/sys", "sys") < 0)
    {
        fprintf(stderr, "failed to rbind mount /sys\n");
        return 1;
    }

    // Change the root dir.
    if (chroot(".") < 0)
    {
        fprintf(stderr, "chroot() failed\n");
        return 1;
    }
    if (chdir("/") < 0)
    {
        fprintf(stderr, "chdir() failed\n");
        return 1;
    }

    // Start bash.
    execl("/bin/bash", "-bash", (char*)NULL);
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
