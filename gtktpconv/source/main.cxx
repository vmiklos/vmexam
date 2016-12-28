#include <initializer_list>
#include <string>

#include <gtk/gtk.h>

namespace
{

void initAmount(GtkWidget* grid)
{
    GtkWidget* amount = gtk_entry_new();
    gtk_grid_attach(GTK_GRID(grid), amount, 0, 0, 1, 1);
}

void initUnit(GtkWidget* grid, int active, int top)
{
    GtkWidget* unitCombo = gtk_combo_box_text_new();
    std::initializer_list<std::string> units = {"inch",  "point", "twip",

                                                "m",     "cm",    "mm",
                                                "mm100",

                                                "emu"};
    for (const auto& unit : units)
        gtk_combo_box_text_append(GTK_COMBO_BOX_TEXT(unitCombo), nullptr,
                                  unit.c_str());
    gtk_combo_box_set_active(GTK_COMBO_BOX(unitCombo), active);
    gtk_grid_attach(GTK_GRID(grid), unitCombo, 1, top, 1, 1);
}

void initResult(GtkWidget* grid)
{
    GtkWidget* result = gtk_label_new("");
    gtk_label_set_selectable(GTK_LABEL(result), TRUE);
    gtk_grid_attach(GTK_GRID(grid), result, 0, 1, 1, 1);
}

void initConvert(GtkWidget* grid)
{
    GtkWidget* convert = gtk_button_new_with_label("Convert");
    gtk_grid_attach(GTK_GRID(grid), convert, 0, 2, 1, 1);
}

void initQuit(GtkWidget* grid)
{
    GtkWidget* quit = gtk_button_new_with_label("Quit");
    g_signal_connect(quit, "clicked", G_CALLBACK(gtk_main_quit), nullptr);
    gtk_grid_attach(GTK_GRID(grid), quit, 1, 2, 1, 1);
}

void initControls(GtkWidget* window)
{
    GtkWidget* grid = gtk_grid_new();
    initAmount(grid);
    initUnit(grid, 0, 0);
    initUnit(grid, 1, 1);
    initResult(grid);
    initConvert(grid);
    initQuit(grid);
    gtk_container_add(GTK_CONTAINER(window), grid);
}

} // anonymous namespace

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
