/*
 *  PacmanGame.java
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

import java.lang.Runnable;
import javax.microedition.lcdui.*;
import java.util.Random;

public class PacmanGame extends Canvas implements Runnable
{
	//int num = 0;
	int width;
	int height;
	Graphics graphics;
	Image image;
	Thread thread;
	Random random = null;

	// statuses
	boolean ingame = false;
	boolean scared = false;
	boolean dying = false;

	// rules
	final int maxghosts = 12;
	final int minscaredtime = 20;
	final int maxspeed = 6;
	final int validspeeds[] = { 1, 2, 3, 3, 4, 4 };

	// defaults
	int ghostnum = 6;
	final int xblocknum = 15;
	final int yblocknum = 13;
	int currentspeed = 4;

	// generated values
	int blocksize;
	final int xscrsize = xblocknum * blocksize;
	final int yscrsize = yblocknum * blocksize;


	// status variables
	int pacsleft, score, deathcounter;
	int scaredcount, scaredtime;

	// positions
	int[] dx, dy, ghostx, ghosty, ghostdx, ghostdy, ghostspeed;
	int pacmanx, pacmany, pacmandx, pacmandy, reqdx, reqdy;

	// the maze
	final short leveldata[] = { 
		19,26,26,22, 9,12,19,26,22, 9,12,19,26,26,22,
		37,11,14,17,26,26,20,15,17,26,26,20,11,14,37,
		17,26,26,20,11, 6,17,26,20, 3,14,17,26,26,20,
		21, 3, 6,25,22, 5,21, 7,21, 5,19,28, 3, 6,21,
		21, 9, 8,14,21,13,21, 5,21,13,21,11, 8,12,21,
		25,18,26,18,24,18,28, 5,25,18,24,18,26,18,28,
		 6,21, 7,21, 7,21,11, 8,14,21, 7,21, 7,21,03,
		19,24,26,24,26,16,26,18,26,16,26,24,26,24,22,
		21, 3, 2, 2, 6,21,15,21,15,21, 3, 2, 2,06,21,
		21, 9, 8, 8, 4,17,26, 8,26,20, 1, 8, 8,12,21,
		17,26,26,22,13,21,11, 2,14,21,13,19,26,26,20,
		37,11,14,17,26,24,22,13,19,24,26,20,11,14,37,
		25,26,26,28, 3, 6,25,26,28, 3, 6,25,26,26,28
	};
	short[] screendata;

	public PacmanGame()
	{
		short i;

		screendata = new short[xblocknum*yblocknum];
		ghostx = new int[maxghosts];
		ghostdx = new int[maxghosts];
		ghosty = new int[maxghosts];
		ghostdy = new int[maxghosts];
		ghostspeed = new int[maxghosts];
		dx=new int[4];
		dy=new int[4];
		width = getWidth();
		height = getHeight();
		image = Image.createImage(width, height);
		GameInit();
		if(width > height)
			blocksize = height/yblocknum;
		else
			blocksize = width/xblocknum;
	}

	// starts a demo or a game
	public void GameInit()
	{
		pacsleft = 3;
		score = 0;
		scaredtime = 120;
		LevelInit();
		ghostnum = 6;
		currentspeed = 4;
		DrawMaze();
	}

	// resets the state of the level (after a death, etc)
	public void LevelInit()
	{
		short i;
		int dx = 1;

		for (i = 0; i < ghostnum; i++)
		{
			ghostx[i] = 7 * blocksize;
			ghosty[i] = 6 * blocksize;
			ghostdx[i] = dx;
			ghostdy[i] = 0;
			dx = - dx;
			ghostspeed[i] = validspeeds[currentspeed - 3];
		}
		pacmanx = 7 * blocksize;
		pacmany = 9 * blocksize;
		pacmandx = 0;
		pacmandy = 0;
		reqdx = 0;
		reqdy = 0;
		dying = false;
		scared = false;
	}

	// draws the maze from scratch (after a death, after a completed level)
	public void DrawMaze()
	{
		int i;

		for (i = 0; i<xblocknum*yblocknum; i++)
		{
			screendata[i]=leveldata[i];
		}
	}


	protected void keyPressed(int key)
	{
		if(ingame)
		{
			if(key == Canvas.KEY_NUM4)
			{
				reqdx = -1;
				reqdy = 0;
			}
			else if(key == Canvas.KEY_NUM6)
			{
				reqdx = 1;
				reqdy = 0;
			}
			else if(key == Canvas.KEY_NUM2)
			{
				reqdx = 0;
				reqdy = -1;
			}
			else if(key == Canvas.KEY_NUM8)
			{
				reqdx = 0;
				reqdy = 1;
			}
		}
		else if(key == Canvas.KEY_NUM5)
		{
			ingame = true;
			GameInit();
		}
	}

	protected void keyReleased(int key)
	{
		if (key == Canvas.KEY_NUM4 || key == Canvas.KEY_NUM6 || key == Canvas.KEY_NUM2 ||  key == Canvas.KEY_NUM8)
		{
			reqdx = 0;
			reqdy = 0;
		}
	}

	public void paint(Graphics g)
	{
		int x, y;
		short i = 0;

		if (graphics == null && width > 0 && height > 0)
		{
			image = Image.createImage(width, height);
			graphics = image.getGraphics();
		}
		if(graphics == null || image == null)
		{
			return;
		}

		graphics.setColor(255, 255, 255);
		graphics.fillRect(0, 0, width, height);
		// draw the maze
		for(y = 0; y < blocksize*yblocknum; y += blocksize)
		{
			for(x = 0; x < blocksize*xblocknum; x += blocksize)
			{
				// borders
				if(!scared)
					graphics.setColor(0, 0, 0);
				else
					graphics.setColor(255, 0, 0);
				/*  2
				 * 1 4
				 *  8
				 */
				if((screendata[i]&1) != 0)
					graphics.drawLine(x,y,x,y+blocksize-1);
				if((screendata[i]&2) != 0)
					graphics.drawLine(x,y,x+blocksize-1,y);
				if((screendata[i]&4) != 0)
					graphics.drawLine(x+blocksize,y,x+blocksize,y+blocksize);
				if ((screendata[i]&8) != 0)
					graphics.drawLine(x,y+blocksize,x+blocksize-1,y+blocksize);
				if ((screendata[i]&16) != 0)
				{
					graphics.setColor(0, 0, 0);
					graphics.fillRect(x+blocksize/2,y+blocksize/2,1,1);
				}
				if ((screendata[i]&32) != 0)
				{
					graphics.setColor(0, 0, 255);
					graphics.fillRect(x+1,y+1,blocksize-1,blocksize-1);
				}
				i++;
			}
		}
		// TODO: draw some score
		if(ingame)
		{
			// play the game
			if (dying)
			{
				deathcounter--;
				if((deathcounter%8)<4)
				{
					graphics.setColor(255, 255, 255);
					graphics.fillRect(pacmanx+1, pacmany+1, blocksize-1, blocksize-1);
				}
				else if((deathcounter%8)>=4)
				{
					graphics.setColor(255, 255, 0);
					graphics.fillRect(pacmanx+1, pacmany+1, blocksize-1, blocksize-1);
				}
				if(deathcounter == 0)
				{
					pacsleft--;
					if(pacsleft == 0)
						ingame = false;
					LevelInit();
				}
			}
			else
			{
				UpdateWalls();
				// if we are not dying, we can move pacman
				int     pos;
				short   ch;

				if (reqdx==-pacmandx && reqdy==-pacmandy)
				{
					pacmandx=reqdx;
					pacmandy=reqdy;
				}
				if (pacmanx%blocksize==0 && pacmany%blocksize==0)
				{
					pos=pacmanx/blocksize+xblocknum*(int)(pacmany/blocksize);
					ch=screendata[pos];
					if ((ch&16)!=0)
					{
						screendata[pos]=(short)(ch&15);
						score++;
					}
					if ((ch&32)!=0)
					{
						scared=true;
						scaredcount=scaredtime;
						screendata[pos]=(short)(ch&15);
						score+=5;
					}

					if (reqdx!=0 || reqdy!=0)
					{
						if (!( (reqdx==-1 && reqdy==0 && (ch&1)!=0) ||
									(reqdx==1 && reqdy==0 && (ch&4)!=0) ||
									(reqdx==0 && reqdy==-1 && (ch&2)!=0) ||
									(reqdx==0 && reqdy==1 && (ch&8)!=0)))
						{
							pacmandx=reqdx;
							pacmandy=reqdy;
						}
					}

					// check if we should stop pacman
					if ( (pacmandx==-1 && pacmandy==0 && (ch&1)!=0) ||
							(pacmandx==1 && pacmandy==0 && (ch&4)!=0) ||
							(pacmandx==0 && pacmandy==-1 && (ch&2)!=0) ||
							(pacmandx==0 && pacmandy==1 && (ch&8)!=0))
					{
						pacmandx=0;
						pacmandy=0;
					}
				}
				pacmanx=pacmanx+currentspeed*pacmandx;
				pacmany=pacmany+currentspeed*pacmandy;
				graphics.setColor(255, 255, 0);
				graphics.fillRect(pacmanx+1, pacmany+1, blocksize-1, blocksize-1);
				CheckMaze();
			}
		}
		else
		{
			// demo
			UpdateWalls();
		}
		if(!dying)
		{
			// if we're not dying, we should move the ghosts (demo or game)
			int pos;
			int count;
			int j;

			for (i=0; i<ghostnum; i++)
			{
				if (ghostx[i]%blocksize==0 && ghosty[i]%blocksize==0)
				{
					pos=ghostx[i]/blocksize+xblocknum*(int)(ghosty[i]/blocksize);
					count=0;
					// no direction by default
					for(j=0;j<4;j++)
					{
						dx[j]=0;
						dy[j]=0;
					}
					if ((screendata[pos]&1)==0 && ghostdx[i]!=1)
					{
						dx[count]=-1;
						dy[count]=0;
						count++;
					}
					if ((screendata[pos]&2)==0 && ghostdy[i]!=1)
					{
						dx[count]=0;
						dy[count]=-1;
						count++;
					}
					if ((screendata[pos]&4)==0 && ghostdx[i]!=-1)
					{
						dx[count]=1;
						dy[count]=0;
						count++;
					}
					if ((screendata[pos]&8)==0 && ghostdy[i]!=-1)
					{
						dx[count]=0;
						dy[count]=1;
						count++;
					}
					if (count==0)
					{
						if ((screendata[pos]&15)==15)
						{
							ghostdx[i]=0;
							ghostdy[i]=0;
						}
						else
						{
							ghostdx[i]=-ghostdx[i];
							ghostdy[i]=-ghostdy[i];
						}
					}
					else
					{
						if(random == null)
						{
							random = new Random();
						}
						while(true)
						{
							// find a possible direction
							count = Math.abs(random.nextInt()%4);
							if(dx[count]==0 && dy[count]==0)
								continue;
							else
								break;
						}
						ghostdx[i]=dx[count]; // random: 0-3
						ghostdy[i]=dy[count];
					}
				}
				ghostx[i]=ghostx[i]+(ghostdx[i]*ghostspeed[i]);
				ghosty[i]=ghosty[i]+(ghostdy[i]*ghostspeed[i]);
				//graphics.drawString("random: "+ num , 0, 150, g.TOP|g.LEFT);
				graphics.setColor(255, 0, 0);
				graphics.fillRect(ghostx[i]+1, ghosty[i]+1, blocksize-1, blocksize-1);

				if (pacmanx>(ghostx[i]-(blocksize/2)) && pacmanx<(ghostx[i]+(blocksize/2)) &&
						pacmany>(ghosty[i]-(blocksize/2)) && pacmany<(ghosty[i]+(blocksize/2)) && ingame)
				{
					if (scared)
					{
						score+=10;
						ghostx[i]=7*blocksize;
						ghosty[i]=6*blocksize;
					}
					else
					{
						dying=true;
						deathcounter=64;
					}
				}
			}
		}
		g.drawImage(image, 0, 0, 0);
	}

	// lock / unlock the ghosts
	public void UpdateWalls()
	{
		scaredcount--;
		if (scaredcount<=0)
			scared=false;

		if (scared)
		{
			screendata[6*xblocknum+6]=11;
			screendata[6*xblocknum+8]=14;
		}
		else
		{
			screendata[6*xblocknum+6]=10;
			screendata[6*xblocknum+8]=10;
		}
	}

	// checks if this is the end of the game or not
	public void CheckMaze()
	{
		short i=0;
		boolean finished=true;

		while (i < xblocknum * yblocknum && finished)
		{
			if ((screendata[i]&48)!=0)
				finished = false;
			i++;
		}
		if (finished)
		{
			score += 50;
			// TODO: draw some score
			try
			{ 
				Thread.sleep(3000);
			}
			catch (InterruptedException e)
			{
			}
			if (ghostnum < maxghosts)
				ghostnum++; 
			if (currentspeed<maxspeed)
				currentspeed++;
			scaredtime=scaredtime-20;
			if (scaredtime<minscaredtime)
				scaredtime=minscaredtime;
			LevelInit();
			DrawMaze();
		}
	}

	public void run()
	{
		long  starttime;

		while(true)
		{
			starttime=System.currentTimeMillis();
			try
			{
				repaint();
				starttime += 40;
				Thread.sleep(Math.max(0, starttime-System.currentTimeMillis()));
			}
			catch(java.lang.InterruptedException ie)
			{
				break;
			}
		}
	}
}
