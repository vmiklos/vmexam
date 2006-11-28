import javax.microedition.lcdui.*;
import javax.microedition.midlet.*;

public class HelloMidlet extends MIDlet implements CommandListener
{
	private Form mMainForm;

	public HelloMidlet()
	{
		mMainForm = new Form("title");
		mMainForm.append(new StringItem(null, "content"));
		mMainForm.addCommand(new Command("button", Command.EXIT, 0));
		mMainForm.setCommandListener(this);
	}

	public void startApp()
	{
		Display.getDisplay(this).setCurrent(mMainForm);
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
