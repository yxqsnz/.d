#include <arpa/inet.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <time.h>

#define WAIT_FOR_ACK                                                           \
  memset(bufx, '\0', strlen(bufx));                                            \
  recvfrom(sfd, bufx, buffsize, 0, (struct sockaddr *)&client_addr,            \
           &client_struct_length);                                             \
  if (!strcmp(bufx, "ack")) {                                                  \
  } else {                                                                     \
    printf("expected `ack` found: %s\n", bufx);                                \
    return -1;                                                                 \
  }

unsigned long getsizeof(const char *name) {
  struct stat st;
  stat(name, &st);
  return st.st_size;
}

int main(int argc, char *argv[]) {
  int port, sfd;
  const char *filename, *host;
  struct sockaddr_in server_addr, client_addr;
  int buffsize = 256;
  char *bufx, *bufy, *buff;
  unsigned long total_read = 0, read = 0, avg = 0, bps = 0, last;

  if (argc != 5) {
    printf("Usage: %s <filename> <host> <addr> <buf size>\n", argv[0]);
    return 1;
  }

  filename = argv[1];
  host = argv[2];
  port = atoi(argv[3]);

  buffsize = atoi(argv[4]);

  bufx = malloc(buffsize);
  bufy = malloc(buffsize);
  buff = malloc(buffsize);

  printf("open: %s\n", filename);
  FILE *source = fopen(filename, "r");

  if (source == NULL) {
    perror(argv[1]);
    return 1;
  }

  unsigned long fsize = getsizeof(filename);

  printf("open: datagram socket .. ");
  fflush(stdout);
  sfd = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
  server_addr.sin_family = AF_INET;
  server_addr.sin_port = htons(port);
  server_addr.sin_addr.s_addr = inet_addr(host);

  if (bind(sfd, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
    perror("fail\nsocket");
  }

  puts("ok");

  socklen_t client_struct_length = sizeof(client_addr);

  recvfrom(sfd, bufx, buffsize, 0, (struct sockaddr *)&client_addr,
           &client_struct_length);

  printf("info: connection request by %s:%d\n", inet_ntoa(client_addr.sin_addr),
         ntohs(client_addr.sin_port));

  if (!strcmp(bufx, "connect")) {
    puts("info: starting to send data...");

    memset(bufy, '\0', strlen(bufy));
    sprintf(bufy, "%ld", fsize);

    if (sendto(sfd, bufy, strlen(bufy), 0, (struct sockaddr *)&client_addr,
               client_struct_length) < 0) {
      perror("error:");
      return 1;
    }

    last = time(NULL);

    while (1) {
      if (last < time(NULL)) {
        bps = avg;
        avg = 0;
        last = time(NULL);
      }

      printf("progress: %.2f MB (%ld%%: %ld KB/s)\r",
             (double)total_read / 1024 / 1024, (100 * total_read) / fsize,
             bps / 100);

      fflush(stdout);

      read = fread(buff, 1, 256, source);
      total_read += read;

      if (read == 0) {
        break;
      }

      if (sendto(sfd, buff, read, 0, (struct sockaddr *)&client_addr,
                 client_struct_length) < 0) {
        perror("error:");
        return 1;
      }

      WAIT_FOR_ACK
      avg += read;
    }

    puts("\nfinished.");
  }

  fclose(source);
  free(bufx);
  free(bufy);
  free(buff);
}