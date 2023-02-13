#include <arpa/inet.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <time.h>
#include <unistd.h>

#define ACK                                                                    \
  strcpy(bufx, "ack");                                                         \
  if (sendto(sfd, bufx, strlen(bufx), 0, (struct sockaddr *)&server_addr,      \
             server_struct_length) < 0) {                                      \
    printf("Unable to send ack\n");                                            \
    return -1;                                                                 \
  }

int main(int argc, char *argv[]) {
  int port, sfd, buffsize;
  const char *filename, *host;
  struct sockaddr_in server_addr;
  char *bufx, *bufy;
  FILE *out;
  unsigned long total_read = 0, avg = 0, bps = 0, fsize = 0, read = 0, last;
  socklen_t server_struct_length = sizeof(server_addr);

  if (argc != 5) {
    printf("Usage: %s <filename> <host> <addr> <buf size>\n", argv[0]);
    return 1;
  }

  buffsize = atoi(argv[4]);
  bufx = malloc(buffsize);
  bufy = malloc(buffsize);

  filename = argv[1];
  host = argv[2];
  port = atoi(argv[3]);

  sfd = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
  server_addr.sin_family = AF_INET;
  server_addr.sin_port = htons(port);
  server_addr.sin_addr.s_addr = inet_addr(host);

  strcpy(bufx, "connect");
  if (sendto(sfd, bufx, strlen(bufx), 0, (struct sockaddr *)&server_addr,
             server_struct_length) < 0) {
    printf("Unable to send message\n");
    return -1;
  }

  if (recvfrom(sfd, bufy, buffsize, 0, (struct sockaddr *)&server_addr,
               &server_struct_length) < 0) {
    printf("error");
    return -1;
  }

  puts("info: connected!");
  out = fopen(filename, "w");

  fsize = atoi(bufy);
  avg = 0;
  bps = 0;
  last = time(NULL);
  printf("size: %.2f MB\n", (float)fsize / 1024 / 1024);

  if (out != NULL) {
    while (1) {
      if (last < time(NULL)) {
        bps = avg;
        avg = 0;
        last = time(NULL);
      }

      printf("progress: %.2f MB (%ld%%: %ld KB/s)\r",
             (double)total_read / 1024 / 1024, (100 * total_read) / fsize,
             bps / 100);

      if (total_read >= fsize) {
        break;
      }

      memset(bufy, '\0', strlen(bufy));
      read = recvfrom(sfd, bufy, buffsize, 0, (struct sockaddr *)&server_addr,
                      &server_struct_length);
      total_read += read;

      if (read < 0) {
        perror("Error while receiving server's msg");
        return -1;
      }

      if (fwrite(bufy, 1, read, out) < 0) {
        perror(filename);
        return -1;
      }

      avg += read;

      ACK
    }

    puts("\nfinished.");
    fclose(out);

  } else {
    perror(filename);
  }

  close(sfd);

  free(bufx);
  free(bufy);
}