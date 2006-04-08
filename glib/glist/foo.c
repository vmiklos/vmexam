#include <glib.h>
#include <stdio.h>

int main(void)
{
	GList *list = NULL;

	list = g_list_append(list, "foo");
	list = g_list_append(list, "bar");

	g_list_foreach(list, (GFunc) printf, "%s");
}
