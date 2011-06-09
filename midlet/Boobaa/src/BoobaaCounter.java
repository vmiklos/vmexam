import javax.microedition.lcdui.*;

public class BoobaaCounter extends Canvas
{
	int num1 = 0;
	int num2 = 0;

	public BoobaaCounter()
	{
		repaint();
	}

	protected void keyPressed(int key)
	{
		if (key == Canvas.KEY_NUM1)
			num1++;
		else if (key == Canvas.KEY_NUM2)
			num2++;
		repaint();
	}

	public void paint(Graphics g)
	{
		g.setColor(255, 255, 255);
		g.fillRect(0, 0, getWidth(), getHeight());
		g.setColor(0, 0, 0);
		g.drawString("1: " + num1 + ", 2: " + num2, 0, 0, g.TOP|g.LEFT);
	}
}
