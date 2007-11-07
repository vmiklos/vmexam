import pickle
import socket
import threading, time

# Here's our thread:
class ConnectionThread ( threading.Thread ):

   def run ( self ):

      # Connect to the server:
      client = socket.socket ( socket.AF_INET, socket.SOCK_STREAM )
      client.connect ( ( 'localhost', 2727 ) )

      # Retrieve and unpickle the list object:
      print pickle.loads ( client.recv ( 1024 ) )

      # Send some messages:
      for x in range ( 10 ):
         client.send ( 'Hey. ' + str ( x ) + '\n' )

      time.sleep(1)
      # Close the connection
      client.close()

# Let's spawn a few threads:
for x in range ( 2 ):
   ConnectionThread().start()
