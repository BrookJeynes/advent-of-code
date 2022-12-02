#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int part_one(FILE *fp);
int part_two(FILE *fp);

int main(void) {
  FILE *fp;

  fp = fopen("input.txt", "r");

  if (fp == NULL) {
    return -1;
  }

  int answer_one = part_one(fp);

  fclose(fp);

  fp = fopen("input.txt", "r");

  if (fp == NULL) {
    return -1;
  }

  int answer_two = part_two(fp);

  printf("Answer one: %d\nAnswer two: %d\n", answer_one, answer_two);

  fclose(fp);

  return 0;
}

int part_one(FILE *fp) {
  char * line = NULL;
  size_t len = 0;
  ssize_t read;

  int temp_count = 0;
  int largest = 0;

  while ((read = getline(&line, &len, fp)) != -1) {
    if(strcmp(line, "\n") == 0 || strcmp(line, "\r\n") == 0) {
      if (temp_count > largest) {
        largest = temp_count;
      }

      temp_count = 0;
    } else {
      temp_count += atoi(line);
    }
  }

  if (line) {
    free(line);
  }

  return largest;
}

int part_two(FILE *fp) {
  char * line = NULL;
  size_t len = 0;
  ssize_t read;

  int sums[] = {0, 0, 0};
  int n = 3; // size of sums array

  int temp_count = 0;
  int largest = 0;

  while ((read = getline(&line, &len, fp)) != -1) {
    if(strcmp(line, "\n") == 0 || strcmp(line, "\r\n") == 0) {
      if (temp_count > largest) {
        largest = temp_count;
      }

      if (temp_count > sums[0]) {
        sums[0] = temp_count;
      }

      // sort array
      for (int i = 0; i < n; ++i){
        for (int j = i + 1; j < n; ++j){
          if (sums[i] > sums[j]){
            int a = sums[i];
            sums[i] = sums[j];
            sums[j] = a;
          }
        }
      }

      temp_count = 0;
    } else {
      temp_count += atoi(line);
    }
  } 

  if (line) {
    free(line);
  }

  return sums[0] + sums[1] + sums[2];
}
