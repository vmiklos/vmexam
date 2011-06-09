import javax.microedition.lcdui.*;
import javax.microedition.midlet.*;

public class Boobaa extends MIDlet implements CommandListener
{
	private BoobaaCounter counter;
	private Command exitCmd = new Command("Exit", Command.SCREEN, 0);

	public Boobaa()
	{
		counter = new BoobaaCounter();
		counter.addCommand(exitCmd);
		counter.setCommandListener(this);
	}

	public void startApp()
	{
		Display.getDisplay(this).setCurrent(counter);
	}

	public void pauseApp()
	{
	}

	public void destroyApp(boolean unconditional)
	{
	}

	public void commandAction(Command c, Displayable s)
	{
		notifyDestroyed();
	}
}
