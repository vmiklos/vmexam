#include <sys/types.h>
#include <sys/socket.h>
#include <netdb.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <stdlib.h>
#include <errno.h>

#define BUF_LEN 512
#define PORT 1100

int main()
{
	int sock;
	struct sockaddr_in conn;
	struct hostent host;
	char buf[BUF_LEN+1];

	sock = socket(PF_INET, SOCK_STREAM, 0);
	host = *gethostbyname("localhost");
	conn.sin_family = AF_INET;
	conn.sin_port = htons(PORT);
	conn.sin_addr = *((struct in_addr *) host.h_addr);
	memset(&(conn.sin_zero), 0, 8);
	connect(sock, (struct sockaddr *)&conn, sizeof(struct sockaddr));
	snprintf(buf, BUF_LEN, "data from client\n");
	printf("sending to server: %s", buf);
	write(sock, buf, strlen(buf));
	read(sock, &buf, BUF_LEN);
	printf("got from server: %s", buf);
	close(sock);
	return 0;
}
