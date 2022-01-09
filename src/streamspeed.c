#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <pthread.h>
#include <time.h>

#define VERSION "1.0.0"

// Flag if the reading thread is still alive
int threadAlive = 1;
// Total bytes read from stdin
size_t bytesRead = 0;

// Thread to constantly read from stdin
void *doRead(void *vargp);

int main(int argc, char const *argv[]) {
    // Check if stdin is open
    if (!stdin) {
        fprintf(stderr, "stdin must be open\n");
        return 1;
    }

    if (argc > 1) {
        // Check for help flag
        if (strcmp("--help", argv[1]) == 0 || strcmp("-h", argv[1]) == 0) {
            printf("Usage: %s [period] [block size]\n", argv[0]);
            puts("Test the throughput speed of stdin.");
            puts("  [period]\t\tPeriod of time in milliseconds to print speed");
            puts("  [block size]\t\tNumber of bytes to read from stdin at once in the read loop.");
            return 0;
        }

        // Check for version flag
        if (strcmp("--version", argv[1]) == 0 || strcmp("-v", argv[1]) == 0) {
            printf("Version: %s\n", VERSION);
            return 0;
        }
    }

    // Time to wait in milliseconds
    int timeout = 500;
    // Convert to float for speed
    float timeout_f = timeout;
    // Amount of bytes to take from stdin at once
    int blockSize = 1024;

    // Set arguments
    switch (argc)
    {
    case 3:
        blockSize = atoi(argv[2]);
        if (blockSize == 0) {
            fprintf(stderr, "Invalid block size\n");
            return 2;
        }
    case 2:
        timeout = atoi(argv[1]);
        if (timeout == 0) {
            fprintf(stderr, "Invalid period\n");
            return 2;
        }
        break;
    default:
        break;
    }

    // Create read thread
    pthread_t tid;
    pthread_create(&tid, NULL, doRead, (void *) &blockSize);

    // Last time speed was printed
    clock_t lastClock = clock();
    // Bytes read previously
    size_t prevBytesCount = 0;

    // Infinite loop
    while (threadAlive) {
        // Wait until we have some data
        usleep(1000 * timeout);
        // Current time
        clock_t nowClock = clock();
        // Calculate difference in time since last print-out in seconds
        float timeDiff = (nowClock - lastClock) / (float) CLOCKS_PER_SEC;

        // New bytes read
        size_t bytesDiff = bytesRead - prevBytesCount;

        // Reset stats
        lastClock = nowClock;
        prevBytesCount = bytesRead;

        // The desired state
        float bytesPerSec = bytesDiff / timeDiff;

        // Convert to mebibytes and print
        printf("%.2f MiB/s\n", bytesPerSec / 1024 / 1024);
    }

    return 0;
}

void *doRead(void *vargp) {
    int blockSize = *((int *) vargp);
    // Memory buffer to read into
    char *buf = (char*) malloc(sizeof(char) * blockSize);

    // Keep reading while stream is open
    while (!feof(stdin)) {
        bytesRead += fread(buf, 1, blockSize, stdin);
    }

    free(buf);

    // Indicate that the thread has finished
    threadAlive = 0;
}

