FSM = '7bb84a5e-5df6-495e-b311-0be3698e1bdb'
import random
import sys
random.seed(int(sys.argv[1]))
length = random.randint(5, 15)
for v in range(length):
    print(random.randint(0,1), end='')
print()