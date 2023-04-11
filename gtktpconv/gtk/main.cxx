/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#include <initializer_list>
#include <map>
#include <sstream>
#include <string>

#include <gtk/gtk.h>
#ifdef _WIN32
#include <windows.h>
#endif

#include "lib.hxx"

/// Hold Gtk references to all data needed to perform a conversion.
class Conversion
{
    GtkEntry* _amount;
    GtkComboBox* _from;
    GtkComboBox* _to;
    GtkLabel* _result;

    void initAmount(GtkWidget* grid);
    static GtkComboBox* initUnit(GtkWidget* grid, int active, int top);
    void initResult(GtkWidget* grid);
    void initConvert(GtkWidget* grid);
    static void convert(GtkWidget* widget, gpointer userData);
    void convertImpl();
    static void initQuit(GtkWidget* grid);

  public:
    Conversion();
    void initControls(GtkWidget* window);
};

Conversion::Conversion()
    : _amount(nullptr), _from(nullptr), _to(nullptr), _result(nullptr)
{
}

void Conversion::initAmount(GtkWidget* grid)
{
    GtkWidget* amount = gtk_entry_new();
    _amount = GTK_ENTRY(amount);
    gtk_grid_attach(GTK_GRID(grid), amount, 0, 0, 1, 1);
}

GtkComboBox* Conversion::initUnit(GtkWidget* grid, int active, int top)
{
    GtkWidget* unitCombo = gtk_combo_box_text_new();
    for (const auto& unit : tpconv::getUnitNames())
        gtk_combo_box_text_append(GTK_COMBO_BOX_TEXT(unitCombo), nullptr,
                                  unit.c_str());
    gtk_combo_box_set_active(GTK_COMBO_BOX(unitCombo), active);
    gtk_grid_attach(GTK_GRID(grid), unitCombo, 1, top, 1, 1);
    return GTK_COMBO_BOX(unitCombo);
}

void Conversion::initResult(GtkWidget* grid)
{
    GtkWidget* result = gtk_label_new("");
    _result = GTK_LABEL(result);
    gtk_label_set_selectable(_result, TRUE);
    gtk_grid_attach(GTK_GRID(grid), result, 0, 1, 1, 1);
}

void Conversion::initConvert(GtkWidget* grid)
{
    GtkWidget* convertButton = gtk_button_new_with_label("Convert");
    gtk_grid_attach(GTK_GRID(grid), convertButton, 0, 2, 1, 1);
    g_signal_connect(convertButton, "clicked", G_CALLBACK(Conversion::convert),
                     this);
}

void Conversion::initControls(GtkWidget* window)
{
    GtkWidget* grid = gtk_grid_new();
    initAmount(grid);
    _from = Conversion::initUnit(grid, 0, 0);
    _to = Conversion::initUnit(grid, 1, 1);
    initResult(grid);
    initConvert(grid);
    initQuit(grid);
    gtk_container_add(GTK_CONTAINER(window), grid);
}

void Conversion::convert(GtkWidget* /*widget*/, gpointer userData)
{
    Conversion* conversion = static_cast<Conversion*>(userData);
    conversion->convertImpl();
}

void Conversion::convertImpl()
{
    std::string text;
    try
    {
        double amount = std::stod(gtk_entry_get_text(_amount));
        auto from = static_cast<tpconv::ConversionUnit>(
            gtk_combo_box_get_active(_from));
        auto to =
            static_cast<tpconv::ConversionUnit>(gtk_combo_box_get_active(_to));
        double ret = tpconv::convert(amount, from, to);
        text = std::to_string(ret);
    }
    catch (const std::invalid_argument& exception)
    {
        std::stringstream ss;
        ss << "invalid argument: " << exception.what();
        text = ss.str();
    }

    gtk_label_set_text(_result, text.c_str());
}

void Conversion::initQuit(GtkWidget* grid)
{
    GtkWidget* quit = gtk_button_new_with_label("Quit");
    g_signal_connect(quit, "clicked", G_CALLBACK(gtk_main_quit), nullptr);
    gtk_grid_attach(GTK_GRID(grid), quit, 1, 2, 1, 1);
}

int main(int argc, char** argv)
{
    gtk_init(&argc, &argv);
    GtkWidget* window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(window), "gtktpconv");
    g_signal_connect(window, "destroy", G_CALLBACK(gtk_main_quit), nullptr);

    Conversion conversion;
    conversion.initControls(window);

    gtk_widget_show_all(window);

    gtk_main();

    return 0;
}

#ifdef _WIN32
int WINAPI WinMain(HINSTANCE hInst, HINSTANCE hPrev, LPSTR szCmdLine, int sw)
{
    int argc = 0;
    char* argv = "";
    return main(argc, &argv);
}
#endif

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
