make; sudo docker build . -t circus; sudo docker run -it -p 2222:22 -p 3333:3333 --cap-add SYS_PTRACE circus /bin/bash
