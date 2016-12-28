#include <initializer_list>
#include <string>

#include <gtk/gtk.h>

namespace
{

void initAmount(GtkWidget* box)
{
    GtkWidget* amount = gtk_entry_new();
    gtk_box_pack_start(GTK_BOX(box), amount, TRUE, TRUE, 2);
}

void initUnit(GtkWidget* box, int active)
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
    gtk_box_pack_start(GTK_BOX(box), unitCombo, TRUE, TRUE, 2);
}

void initResult(GtkWidget* box)
{
    GtkWidget* result = gtk_label_new("");
    gtk_label_set_selectable(GTK_LABEL(result), TRUE);
    gtk_box_pack_start(GTK_BOX(box), result, TRUE, TRUE, 2);
}

void initConvert(GtkWidget* box)
{
    GtkWidget* convert = gtk_button_new_with_label("Convert");
    gtk_box_pack_start(GTK_BOX(box), convert, TRUE, TRUE, 2);
}

void initControls(GtkWidget* window)
{
    GtkWidget* box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 0);
    initAmount(box);
    initUnit(box, 0);
    initUnit(box, 1);
    initResult(box);
    initConvert(box);
    gtk_container_add(GTK_CONTAINER(window), box);
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
