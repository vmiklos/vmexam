#include <gtk/gtk.h>

void initAmount(GtkWidget* box)
{
    GtkWidget* amount = gtk_entry_new();
    gtk_box_pack_start(GTK_BOX(box), amount, TRUE, TRUE, 2);
}

void initControls(GtkWidget* window)
{
    GtkWidget* box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 0);
    initAmount(box);
    gtk_container_add(GTK_CONTAINER(window), box);
}

int main(int argc, char** argv)
{
    gtk_init(&argc, &argv);
    GtkWidget* window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(window), "gtktpconv");
    g_signal_connect(window, "destroy", G_CALLBACK(gtk_main_quit), nullptr);

    initControls(window);

    gtk_widget_show_all(window);

    gtk_main();

    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
