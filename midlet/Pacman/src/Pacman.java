/*
 *  Pacman.java
 *
 *  Copyright (c) 2006 by Miklos Vajna <vmiklos@frugalware.org>
 *
 *  Some ideas by Brian Postma <b.postma@hetnet.nl> and Daniel Sipka
 *  <no1msd@gmail.com>
 * 
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, 
 *  USA.
 */

import javax.microedition.lcdui.*;
import javax.microedition.midlet.*;

public class Pacman extends MIDlet implements CommandListener
{
	private PacmanGame game;
	private Command helpCmd = new Command("Help", Command.SCREEN, 1);
	private Command exitCmd = new Command("Exit", Command.SCREEN, 2);
	private Command cancelCmd = new Command("Cancel", Command.SCREEN, 3);
	private Command okCmd = new Command("OK", Command.SCREEN, 1);

	public Pacman()
	{
		game = new PacmanGame();
		game.addCommand(helpCmd);
		game.addCommand(exitCmd);
		game.addCommand(cancelCmd);
		game.setCommandListener(this);
	}

	public void startApp()
	{
		Display.getDisplay(this).setCurrent(game);
		Thread myThread = new Thread(game);
		myThread.start();
	}

	public void pauseApp()
	{
	}

	public void destroyApp(boolean unconditional)
	{
		Display.getDisplay(this).setCurrent(null);
	}

	public void commandAction(Command c, Displayable s)
	{
		if (c == exitCmd)
		{
			destroyApp(false);
			notifyDestroyed();
		}
		else if (c == helpCmd)
		{
			Item[] levelItem =
			{
				new StringItem("", "Guide the yellow Pacman " +
						"around the maze and eat all the " +
						"little black dots whilst " +
						"avoiding those nasty red " +
						"ghosts! Use the key 5 to start " +
						"the game, 2, 4, 6 and 8 to move " +
						"Pacman up, left, right and " +
						"down, respectively.")
			};
			Form form = new Form("Help", levelItem);
			form.addCommand(okCmd);
			form.setCommandListener(this);
			Display.getDisplay(this).setCurrent(form);
		}
		else if ((c == cancelCmd) || (c == okCmd))
		{
			Display.getDisplay(this).setCurrent(game);
		}
	}
}
