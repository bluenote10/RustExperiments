#include <fcntl.h>
#include <iostream>
#include <linux/input.h>
#include <string>
#include <unistd.h>

int main() {
  std::cout << "Hello World\n";

  std::string path{"/dev/input/event14"}; // "/dev/input/js3"

  int fd = open(path.c_str(), O_RDWR | O_NONBLOCK);
  if (fd < 0) {
    close(fd);
    std::cout << "Failed to get file descriptor\n";
    return 1;
  } else {
    std::cout << "Got file descriptor: " << fd << "\n";
  }

  char device_name[256] = "";
  if (ioctl(fd, EVIOCGNAME(256), device_name) > 0) {
    std::cout << "Device name: " << device_name << "\n";
  } else {
    std::cout << "Failed to get device name.\n";
  }

  input_event event{};
  while (true) {
    bool success = read(fd, &event, sizeof(input_event)) == sizeof(input_event);
    if (success) {
      std::cout << "type: " << event.type << " code: " << event.code << "\n";
    }
  }

  return 0;
}