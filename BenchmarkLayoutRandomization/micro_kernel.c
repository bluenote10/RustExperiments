// This is the original "micro kernel" example given in the "Producing Wrong Data
// Without Doing Anything Obviously Wrong!" paper.

static int i = 0, j = 0, k = 0;

int main() {
  int g = 0, inc = 1;
  for (; g<65536; g++) {
    i += inc;
    j += inc;
    k += inc;
  }
  return 0;
}
