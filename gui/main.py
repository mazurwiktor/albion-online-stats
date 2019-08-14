import time;
import sys;

import libmeter;

if __name__ == "__main__":
    libmeter.initialize()
    prev = None
    while True:
        time.sleep(1)
        if prev != libmeter.get_instance_session():

            sys.stdout.write("{:c}[2J".format(27))
            print(libmeter.get_instance_session())
