#include <stdio.h>
#include <dialog.h>

int main(void)
{
	FILE *input = stdin;
	dialog_state.output = stderr;
	
	init_dialog(input, dialog_state.output);
	dialog_msgbox("title", "content", 0, 0, 0);
	end_dialog();
}
