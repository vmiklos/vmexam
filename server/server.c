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
	int sock, client, opt = 1;
	struct sockaddr_in conn;
	char buf[BUF_LEN+1];

	// 1: socket
	sock = socket(AF_INET, SOCK_STREAM, 0);
	setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));
	memset(&conn, 0, sizeof(conn));
	conn.sin_port = htons(PORT);
	conn.sin_addr.s_addr = htonl(INADDR_ANY);
	// 2: bind
	bind(sock,(struct sockaddr*) &conn, sizeof(conn));
	// 3: listen
	listen(sock, 1);
	// 4: accept
	client = accept(sock, NULL, NULL);
	read(client, &buf, BUF_LEN);
	printf("got from client: %s", buf);
	snprintf(buf, BUF_LEN, "data from server\n");
	printf("sending to client: %s", buf);
	write(client, buf, strlen(buf));
	// 5: close
	close(client);
	close(sock);
	return 0;
}
