#include <glib.h>
#include <stdio.h>

int myfunc(char *i, char **udata)
{
	printf("%s/%s:%s ", udata[0], udata[1], i);
}

int main(void)
{
	GList *list = NULL;
	char *args[] = {"pre", "prf"};

	list = g_list_append(list, "foo");
	list = g_list_append(list, "bar");

	g_list_foreach(list, (GFunc) myfunc, args);
}
