#include <mysql.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define HOST "localhost"
#define USERNAME "root"
#define PASSWD "foo"
#define DBNAME "blog"
#define SOCKET "/tmp/mysql.sock"

static MYSQL demo_db, *sock;

int main(int argc, char **argv)
{
	int insert_id;
	char *encdata, query[255];
	int datasize;
	MYSQL_RES *res;
	MYSQL_ROW row;

	if(argc<2)
	{
		printf("usage: %s id\n", argv[0]);
		return(0);
	}

	if(!(sock = mysql_real_connect(&demo_db, HOST, USERNAME, PASSWD, DBNAME, 0, SOCKET,0)))
	{
		printf("Connecting failed: %s\n", mysql_error(&demo_db));
		return(1);
	}

	sprintf(query, "SELECT id, content FROM posts WHERE id='%d'", atoi(argv[1]));
	if(mysql_query(sock, query))
	{
		printf("Query failed: %s\n", mysql_error(&demo_db));
		return(1);
	}

	res=mysql_store_result(&demo_db); /* Download result from server */
	if(!(row=mysql_fetch_row(res))) /* Get a row from the results */
	{
		printf("Empty set.\n");
		return(1);
	}
	printf("You selected \"%s\".\n", row[1]);
	mysql_free_result(res); /* Release memory used to store results. */
	mysql_close(&demo_db);

	return(0);
}
